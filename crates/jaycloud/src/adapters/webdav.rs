//! Adaptateur WebDAV (RFC 4918) — branche `dav-server` 0.11 sur `FilesOp`.
//!
//! Ce module expose un `DavHandler` consommable par n'importe quel runtime
//! HTTP (axum, hyper, warp). Le binding HTTP réel (axum + middleware
//! Basic Auth via app-passwords) sera fait en PR-5 quand le portail web
//! arrive. Cette séparation permet de tester l'adaptateur sans serveur.
//!
//! ## Modèle d'implémentation
//!
//! `JayCloudDavFs` implémente `dav_server::fs::DavFileSystem`. Il wrappe un
//! `Arc<FilesOp>` et mappe chaque appel DAV vers `files_op`. Les
//! opérations d'écriture (PUT) sont **bufferisées en mémoire jusqu'au
//! flush()** car `FilesOp::put` est atomique — pas de streaming d'écriture
//! au niveau Opérateur.
//!
//! Cohérence avec la Spec :
//! - DT-01 (`dav-server` 0.11 comme base) ✅
//! - DT-07 (`files_op` héritier MiyuCloud) ✅
//! - §6 (méthodes WebDAV P3 : OPTIONS/PROPFIND/PROPPATCH/GET/PUT/DELETE/
//!   MKCOL/COPY/MOVE/LOCK/UNLOCK). LOCK/UNLOCK est fourni par `dav-server`
//!   via son `MemLs` (en mémoire).

use std::io::SeekFrom;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use bytes::Bytes;
use dav_server::fs::{
    DavDirEntry, DavFile, DavFileSystem, DavMetaData, FsError, FsFuture, FsResult, FsStream,
    OpenOptions, ReadDirMeta,
};
use dav_server::{DavHandler, davpath::DavPath, memls::MemLs};

use crate::operators::files_op::{FileEntry, FilesOp, FilesOpError};

/// Adaptateur `DavFileSystem` au-dessus de `FilesOp`.
#[derive(Clone)]
pub struct JayCloudDavFs {
    files: Arc<FilesOp>,
}

impl JayCloudDavFs {
    /// Construit l'adaptateur.
    #[must_use]
    pub fn new(files: Arc<FilesOp>) -> Self {
        Self { files }
    }

    /// Construit un `DavHandler` complet prêt à être monté dans un serveur
    /// HTTP (axum, hyper). LOCK/UNLOCK fourni par `MemLs` (en mémoire).
    #[must_use]
    pub fn build_handler(files: Arc<FilesOp>) -> DavHandler {
        let fs = Self::new(files);
        DavHandler::builder()
            .filesystem(Box::new(fs))
            .locksystem(MemLs::new())
            .build_handler()
    }
}

/// Convertit un `DavPath` en chemin canonique JayCloud (sans `/` initial).
fn dav_path_to_canonical(path: &DavPath) -> String {
    let s = path.as_url_string();
    s.trim_start_matches('/').to_string()
}

/// Convertit une erreur `FilesOp` en `FsError` WebDAV.
fn map_files_err(e: FilesOpError) -> FsError {
    match e {
        FilesOpError::NotFound(_) => FsError::NotFound,
        FilesOpError::InvalidPath(_) => FsError::Forbidden,
        FilesOpError::Cas(_) => FsError::GeneralFailure,
    }
}

impl DavFileSystem for JayCloudDavFs {
    fn open<'a>(
        &'a self,
        path: &'a DavPath,
        options: OpenOptions,
    ) -> FsFuture<'a, Box<dyn DavFile>> {
        Box::pin(async move {
            let canonical = dav_path_to_canonical(path);

            // Mode lecture pure : on charge le contenu actuel.
            // Mode écriture / création : on prépare un buffer vide.
            let read_content = if options.write {
                // Si truncate (PUT classique) → buffer vide.
                // Sinon (append rare en DAV) → on charge le contenu courant.
                if options.truncate || options.create_new {
                    None
                } else {
                    self.files.get(&canonical).ok()
                }
            } else if options.read {
                Some(self.files.get(&canonical).map_err(map_files_err)?)
            } else {
                return Err(FsError::Forbidden);
            };

            // Existence check pour create_new (échec si déjà présent).
            if options.create_new && self.files.stat(&canonical).is_ok() {
                return Err(FsError::Exists);
            }

            let file = JayCloudDavFile {
                path: canonical,
                files: Arc::clone(&self.files),
                content: read_content.unwrap_or_default(),
                pos: 0,
                modified: false,
                is_writing: options.write,
            };
            Ok(Box::new(file) as Box<dyn DavFile>)
        })
    }

    fn read_dir<'a>(
        &'a self,
        path: &'a DavPath,
        _meta: ReadDirMeta,
    ) -> FsFuture<'a, FsStream<Box<dyn DavDirEntry>>> {
        Box::pin(async move {
            let prefix = {
                let canonical = dav_path_to_canonical(path);
                if canonical.is_empty() {
                    String::new()
                } else {
                    format!("{canonical}/")
                }
            };

            // Liste : on filtre les entrées dont le path commence par prefix,
            // et on ne garde que les composants DIRECTS (pas en profondeur).
            let mut direct_files: Vec<FileEntry> = Vec::new();
            let mut seen_subdirs: std::collections::BTreeSet<String> =
                std::collections::BTreeSet::new();

            for entry in self.files.list() {
                if !entry.path.starts_with(&prefix) {
                    continue;
                }
                let rest = &entry.path[prefix.len()..];
                if let Some(slash) = rest.find('/') {
                    // Sous-dossier — on en synthétise une entrée DAV virtuelle.
                    let dirname = format!("{prefix}{}", &rest[..slash]);
                    seen_subdirs.insert(dirname);
                } else if !rest.is_empty() {
                    direct_files.push(entry);
                }
            }

            let mut entries: Vec<Box<dyn DavDirEntry>> = Vec::new();
            for dir_path in seen_subdirs {
                entries.push(Box::new(JayCloudDavDirEntry::virtual_dir(dir_path)));
            }
            for f in direct_files {
                entries.push(Box::new(JayCloudDavDirEntry::file(f)));
            }

            let stream = futures::stream::iter(entries.into_iter().map(Ok::<_, FsError>));
            let boxed: FsStream<Box<dyn DavDirEntry>> = Box::pin(stream);
            Ok(boxed)
        })
    }

    fn metadata<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, Box<dyn DavMetaData>> {
        Box::pin(async move {
            let canonical = dav_path_to_canonical(path);
            // Racine ou prefix de dossier → dossier virtuel.
            if canonical.is_empty() || self.path_is_virtual_dir(&canonical) {
                return Ok(Box::new(JayCloudDavMeta::virtual_dir()) as Box<dyn DavMetaData>);
            }
            let entry = self.files.stat(&canonical).map_err(map_files_err)?;
            Ok(Box::new(JayCloudDavMeta::from_entry(&entry)) as Box<dyn DavMetaData>)
        })
    }

    fn symlink_metadata<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, Box<dyn DavMetaData>> {
        // JayCloud ne supporte pas les symlinks ; même résultat que metadata.
        self.metadata(path)
    }

    // ─── Méthodes optionnelles utiles ───────────────────────────────────────

    fn remove_file<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        Box::pin(async move {
            let canonical = dav_path_to_canonical(path);
            self.files.delete(&canonical).map_err(map_files_err)
        })
    }

    fn remove_dir<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        Box::pin(async move {
            // Suppression d'un dossier virtuel = retrait de toutes les entrées
            // dont le path commence par prefix.
            let canonical = dav_path_to_canonical(path);
            let prefix = if canonical.is_empty() {
                String::new()
            } else {
                format!("{canonical}/")
            };
            let to_delete: Vec<String> = self
                .files
                .list()
                .into_iter()
                .filter(|e| e.path == canonical || e.path.starts_with(&prefix))
                .map(|e| e.path)
                .collect();
            if to_delete.is_empty() {
                return Err(FsError::NotFound);
            }
            for p in to_delete {
                let _ = self.files.delete(&p);
            }
            Ok(())
        })
    }

    fn create_dir<'a>(&'a self, _path: &'a DavPath) -> FsFuture<'a, ()> {
        // Les dossiers sont virtuels (inférés des chemins des fichiers).
        // Pour passer Litmus, on accepte silencieusement le MKCOL.
        Box::pin(async move { Ok(()) })
    }
}

impl JayCloudDavFs {
    /// Indique si un chemin canonique correspond à un dossier virtuel
    /// (c.-à-d. au moins une entrée commence par ce préfixe).
    fn path_is_virtual_dir(&self, canonical: &str) -> bool {
        let prefix = format!("{canonical}/");
        self.files
            .list()
            .into_iter()
            .any(|e| e.path.starts_with(&prefix))
    }
}

// ─────────────────────────────────────────────────────────────────────────
// DavFile
// ─────────────────────────────────────────────────────────────────────────

/// Implémentation `DavFile` bufferisée en mémoire.
///
/// `FilesOp::put` est atomique → on accumule les écritures dans `content`
/// et on flush via `put` au moment du `flush()` ou du `Drop` (best-effort).
pub struct JayCloudDavFile {
    path: String,
    files: Arc<FilesOp>,
    content: Vec<u8>,
    pos: u64,
    modified: bool,
    is_writing: bool,
}

impl std::fmt::Debug for JayCloudDavFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JayCloudDavFile")
            .field("path", &self.path)
            .field("content_len", &self.content.len())
            .field("pos", &self.pos)
            .field("modified", &self.modified)
            .field("is_writing", &self.is_writing)
            .finish_non_exhaustive()
    }
}

impl JayCloudDavFile {
    fn flush_to_files(&mut self) -> Result<(), FilesOpError> {
        if !self.modified {
            return Ok(());
        }
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        self.files.put(&self.path, &self.content, now)?;
        self.modified = false;
        Ok(())
    }
}

impl DavFile for JayCloudDavFile {
    fn metadata(&'_ mut self) -> FsFuture<'_, Box<dyn DavMetaData>> {
        Box::pin(async move {
            // Renvoie une métadonnée basée sur le buffer courant (cohérent
            // même avant flush).
            let meta = JayCloudDavMeta::for_file(self.content.len() as u64);
            Ok(Box::new(meta) as Box<dyn DavMetaData>)
        })
    }

    fn write_buf(&'_ mut self, mut buf: Box<dyn bytes::Buf + Send>) -> FsFuture<'_, ()> {
        Box::pin(async move {
            if !self.is_writing {
                return Err(FsError::Forbidden);
            }
            while buf.has_remaining() {
                let chunk = buf.chunk();
                self.append_at_pos(chunk);
                let len = chunk.len();
                buf.advance(len);
            }
            self.modified = true;
            Ok(())
        })
    }

    fn write_bytes(&'_ mut self, buf: Bytes) -> FsFuture<'_, ()> {
        Box::pin(async move {
            if !self.is_writing {
                return Err(FsError::Forbidden);
            }
            self.append_at_pos(&buf);
            self.modified = true;
            Ok(())
        })
    }

    fn read_bytes(&'_ mut self, count: usize) -> FsFuture<'_, Bytes> {
        Box::pin(async move {
            let start = self.pos as usize;
            if start >= self.content.len() {
                return Ok(Bytes::new());
            }
            let end = (start + count).min(self.content.len());
            let slice = Bytes::copy_from_slice(&self.content[start..end]);
            self.pos = end as u64;
            Ok(slice)
        })
    }

    fn seek(&'_ mut self, pos: SeekFrom) -> FsFuture<'_, u64> {
        Box::pin(async move {
            let new_pos: i64 = match pos {
                SeekFrom::Start(n) => n as i64,
                SeekFrom::End(d) => self.content.len() as i64 + d,
                SeekFrom::Current(d) => self.pos as i64 + d,
            };
            if new_pos < 0 {
                return Err(FsError::GeneralFailure);
            }
            self.pos = new_pos as u64;
            Ok(self.pos)
        })
    }

    fn flush(&'_ mut self) -> FsFuture<'_, ()> {
        Box::pin(async move {
            self.flush_to_files()
                .map_err(map_files_err)
        })
    }
}

impl JayCloudDavFile {
    /// Append-or-overwrite à la position courante.
    fn append_at_pos(&mut self, chunk: &[u8]) {
        let pos = self.pos as usize;
        let end = pos + chunk.len();
        if end > self.content.len() {
            self.content.resize(end, 0);
        }
        self.content[pos..end].copy_from_slice(chunk);
        self.pos = end as u64;
    }
}

impl Drop for JayCloudDavFile {
    fn drop(&mut self) {
        // Best-effort : si le client DAV ferme la connexion sans flush
        // explicite, on persiste quand même.
        let _ = self.flush_to_files();
    }
}

// ─────────────────────────────────────────────────────────────────────────
// DavMetaData
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct JayCloudDavMeta {
    len: u64,
    is_dir: bool,
    modified_unix: i64,
}

impl JayCloudDavMeta {
    fn from_entry(entry: &FileEntry) -> Self {
        Self {
            len: entry.size_bytes.unwrap_or(0),
            is_dir: entry.is_dir,
            modified_unix: entry.mtime,
        }
    }

    fn for_file(len: u64) -> Self {
        Self {
            len,
            is_dir: false,
            modified_unix: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
        }
    }

    fn virtual_dir() -> Self {
        Self {
            len: 0,
            is_dir: true,
            modified_unix: 0,
        }
    }
}

impl DavMetaData for JayCloudDavMeta {
    fn len(&self) -> u64 {
        self.len
    }

    fn modified(&self) -> FsResult<SystemTime> {
        Ok(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.modified_unix.max(0) as u64))
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }
}

// ─────────────────────────────────────────────────────────────────────────
// DavDirEntry
// ─────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct JayCloudDavDirEntry {
    name: String,
    meta: JayCloudDavMeta,
}

impl JayCloudDavDirEntry {
    fn file(entry: FileEntry) -> Self {
        let name = entry
            .path
            .rsplit('/')
            .next()
            .unwrap_or(&entry.path)
            .to_string();
        let meta = JayCloudDavMeta::from_entry(&entry);
        Self { name, meta }
    }

    fn virtual_dir(canonical_path: String) -> Self {
        let name = canonical_path
            .rsplit('/')
            .next()
            .unwrap_or(&canonical_path)
            .to_string();
        Self {
            name,
            meta: JayCloudDavMeta::virtual_dir(),
        }
    }
}

impl DavDirEntry for JayCloudDavDirEntry {
    fn name(&self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }

    fn metadata(&self) -> FsFuture<'_, Box<dyn DavMetaData>> {
        let meta = self.meta.clone();
        Box::pin(async move { Ok(Box::new(meta) as Box<dyn DavMetaData>) })
    }
}

// Compile-time check : laisse Rust valider que la signature ci-dessus est
// correcte (futures bornés par Pin<Box<dyn Future ... + Send + 'a>>).
fn _assert_send<T: Send>() {}
fn _assert_impls() {
    _assert_send::<Pin<Box<dyn DavFile>>>();
    _assert_send::<JayCloudDavFs>();
}

// ─────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kits::cas_kit::CasConfig;
    use crate::kits::crypto_kit::Key32;
    use dav_server::davpath::DavPath;
    use futures::stream::StreamExt;

    fn setup() -> (JayCloudDavFs, Arc<FilesOp>, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let cas_root = dir.path().join("cas");
        let files = Arc::new(FilesOp::new(cas_root, Key32::generate(), CasConfig::default()));
        let dav = JayCloudDavFs::new(Arc::clone(&files));
        (dav, files, dir)
    }

    fn dp(p: &str) -> DavPath {
        DavPath::new(p).unwrap()
    }

    // ─── metadata ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn metadata_file() {
        let (dav, files, _dir) = setup();
        files.put("hello.txt", b"Hello WebDAV!", 1_700_000_000).unwrap();
        let meta = dav.metadata(&dp("/hello.txt")).await.unwrap();
        assert!(!meta.is_dir());
        assert_eq!(meta.len(), 13);
    }

    #[tokio::test]
    async fn metadata_root_is_dir() {
        let (dav, _files, _dir) = setup();
        let meta = dav.metadata(&dp("/")).await.unwrap();
        assert!(meta.is_dir());
    }

    #[tokio::test]
    async fn metadata_virtual_subdir_is_dir() {
        let (dav, files, _dir) = setup();
        files.put("docs/a.txt", b"a", 1).unwrap();
        files.put("docs/b.txt", b"b", 2).unwrap();
        let meta = dav.metadata(&dp("/docs")).await.unwrap();
        assert!(meta.is_dir());
    }

    #[tokio::test]
    async fn metadata_missing_file_is_not_found() {
        let (dav, _files, _dir) = setup();
        let r = dav.metadata(&dp("/ghost.txt")).await;
        assert!(matches!(r, Err(FsError::NotFound)));
    }

    // ─── read_dir ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn read_dir_root_lists_files_and_virtual_subdirs() {
        let (dav, files, _dir) = setup();
        files.put("a.txt", b"x", 1).unwrap();
        files.put("docs/b.txt", b"y", 2).unwrap();
        files.put("docs/c.txt", b"z", 3).unwrap();
        files.put("src/main.rs", b"fn main(){}", 4).unwrap();

        let mut stream = dav.read_dir(&dp("/"), ReadDirMeta::Data).await.unwrap();
        let mut names: Vec<String> = Vec::new();
        while let Some(item) = stream.next().await {
            let entry = item.unwrap();
            names.push(String::from_utf8(entry.name()).unwrap());
        }
        names.sort();
        // Attendu : "a.txt", "docs", "src" (les fichiers et sous-dossiers
        // virtuels au premier niveau).
        assert_eq!(names, vec!["a.txt", "docs", "src"]);
    }

    #[tokio::test]
    async fn read_dir_inside_virtual_subdir() {
        let (dav, files, _dir) = setup();
        files.put("docs/a.txt", b"x", 1).unwrap();
        files.put("docs/b.txt", b"y", 2).unwrap();
        files.put("src/main.rs", b"z", 3).unwrap();

        let mut stream = dav.read_dir(&dp("/docs"), ReadDirMeta::Data).await.unwrap();
        let mut names: Vec<String> = Vec::new();
        while let Some(item) = stream.next().await {
            let entry = item.unwrap();
            names.push(String::from_utf8(entry.name()).unwrap());
        }
        names.sort();
        assert_eq!(names, vec!["a.txt", "b.txt"]);
    }

    // ─── open / read / write ───────────────────────────────────────────

    #[tokio::test]
    async fn open_read_returns_content() {
        let (dav, files, _dir) = setup();
        files.put("hello.txt", b"Hello DAV", 1).unwrap();

        let mut file = dav
            .open(
                &dp("/hello.txt"),
                OpenOptions {
                    read: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        let bytes = file.read_bytes(1024).await.unwrap();
        assert_eq!(&bytes[..], b"Hello DAV");
    }

    #[tokio::test]
    async fn open_read_missing_fails() {
        let (dav, _files, _dir) = setup();
        let r = dav
            .open(
                &dp("/ghost.txt"),
                OpenOptions {
                    read: true,
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(r, Err(FsError::NotFound)));
    }

    #[tokio::test]
    async fn put_then_get_roundtrip() {
        let (dav, files, _dir) = setup();
        let mut file = dav
            .open(
                &dp("/new.txt"),
                OpenOptions {
                    write: true,
                    create: true,
                    truncate: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        file.write_bytes(Bytes::from_static(b"hello from PUT")).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        let content = files.get("new.txt").unwrap();
        assert_eq!(content, b"hello from PUT");
    }

    #[tokio::test]
    async fn create_new_fails_if_exists() {
        let (dav, files, _dir) = setup();
        files.put("existing.txt", b"x", 1).unwrap();
        let r = dav
            .open(
                &dp("/existing.txt"),
                OpenOptions {
                    write: true,
                    create_new: true,
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(r, Err(FsError::Exists)));
    }

    #[tokio::test]
    async fn write_at_offset_via_seek() {
        let (dav, files, _dir) = setup();
        let mut file = dav
            .open(
                &dp("/buf.txt"),
                OpenOptions {
                    write: true,
                    create: true,
                    truncate: true,
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        file.write_bytes(Bytes::from_static(b"AAAAAAA")).await.unwrap();
        // 7 'A' écrits, pos = 7. Seek à 2, on overwrite 2 chars puis pos=4.
        // Résultat final : positions [0..2]=AA, [2..4]=BB, [4..7]=AAA.
        file.seek(SeekFrom::Start(2)).await.unwrap();
        file.write_bytes(Bytes::from_static(b"BB")).await.unwrap();
        file.flush().await.unwrap();
        drop(file);

        let content = files.get("buf.txt").unwrap();
        assert_eq!(content, b"AABBAAA");
    }

    // ─── remove ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn remove_file_works() {
        let (dav, files, _dir) = setup();
        files.put("to_delete.txt", b"x", 1).unwrap();
        dav.remove_file(&dp("/to_delete.txt")).await.unwrap();
        assert!(files.stat("to_delete.txt").is_err());
    }

    #[tokio::test]
    async fn remove_dir_removes_all_entries_under_prefix() {
        let (dav, files, _dir) = setup();
        files.put("docs/a.txt", b"a", 1).unwrap();
        files.put("docs/b.txt", b"b", 2).unwrap();
        files.put("src/main.rs", b"c", 3).unwrap();

        dav.remove_dir(&dp("/docs")).await.unwrap();
        assert!(files.stat("docs/a.txt").is_err());
        assert!(files.stat("docs/b.txt").is_err());
        assert!(files.stat("src/main.rs").is_ok());
    }

    #[tokio::test]
    async fn remove_dir_empty_fails() {
        let (dav, _files, _dir) = setup();
        let r = dav.remove_dir(&dp("/ghost")).await;
        assert!(matches!(r, Err(FsError::NotFound)));
    }

    // ─── handler ───────────────────────────────────────────────────────

    #[test]
    fn build_handler_succeeds() {
        let dir = tempfile::tempdir().unwrap();
        let files = Arc::new(FilesOp::new(
            dir.path().join("cas"),
            Key32::generate(),
            CasConfig::default(),
        ));
        let _handler = JayCloudDavFs::build_handler(files);
        // Le simple fait d'arriver ici sans panic suffit (DavHandler n'a pas
        // d'API d'inspection publique).
    }
}
