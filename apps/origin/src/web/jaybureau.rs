//! Module Jay Bureau côté serveur — stockage des documents + salles de collab.
//!
//! Routes REST :
//! - GET  /api/jaybureau/docs                 — liste des documents de l'utilisateur
//! - POST /api/jaybureau/docs                 — créer un nouveau document
//! - GET  /api/jaybureau/docs/:id             — récupérer un document + son contenu
//! - PUT  /api/jaybureau/docs/:id/title       — renommer
//! - DELETE /api/jaybureau/docs/:id           — supprimer
//! - POST /api/jaybureau/docs/:id/share       — partager (ajouter ACL entry)
//!
//! WebSocket :
//! - /ws/jaybureau/collab/:doc_id             — salle de collaboration temps réel

use jay_collab::room::RoomRegistry;
use jaybureau_core::{DocKind, DocumentBase, UserId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// État partagé du module Jay Bureau.
pub struct JayBureauState {
    /// Index des documents par id.
    pub docs: RwLock<HashMap<String, StoredDocument>>,
    /// Salles de collaboration actives.
    pub rooms: Arc<RoomRegistry>,
    /// Répertoire de stockage des snapshots de documents.
    pub storage_dir: std::path::PathBuf,
}

/// Document persisté côté serveur.
#[derive(Debug, Clone)]
pub struct StoredDocument {
    pub meta: DocumentBase,
    /// Snapshot binaire (Yrs encoded state).
    pub snapshot: Vec<u8>,
}

impl JayBureauState {
    pub fn new(storage_dir: impl Into<std::path::PathBuf>) -> Self {
        let dir = storage_dir.into();
        std::fs::create_dir_all(&dir).ok();
        Self {
            docs: RwLock::new(HashMap::new()),
            rooms: Arc::new(RoomRegistry::new()),
            storage_dir: dir,
        }
    }

    /// Charge les documents depuis le stockage disque.
    pub fn load_all(&self) -> std::io::Result<usize> {
        let mut count = 0;
        if !self.storage_dir.exists() {
            return Ok(0);
        }
        for entry in std::fs::read_dir(&self.storage_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            if let Ok(json) = std::fs::read_to_string(&path) {
                if let Ok(meta) = serde_json::from_str::<DocumentBase>(&json) {
                    let snapshot_path = path.with_extension("bin");
                    let snapshot = std::fs::read(&snapshot_path).unwrap_or_default();
                    let stored = StoredDocument {
                        meta: meta.clone(),
                        snapshot,
                    };
                    if let Ok(mut docs) = self.docs.write() {
                        docs.insert(meta.id.clone(), stored);
                        count += 1;
                    }
                }
            }
        }
        tracing::info!("Jay Bureau: {count} documents chargés depuis le disque");
        Ok(count)
    }

    /// Crée un nouveau document.
    pub fn create_doc(&self, kind: DocKind, title: String, owner_id: UserId) -> StoredDocument {
        let doc = DocumentBase::new(kind, title, owner_id);
        let stored = StoredDocument {
            meta: doc,
            snapshot: Vec::new(),
        };
        self.save_to_disk(&stored).ok();
        if let Ok(mut docs) = self.docs.write() {
            docs.insert(stored.meta.id.clone(), stored.clone());
        }
        stored
    }

    /// Retourne un document par id.
    pub fn get_doc(&self, id: &str) -> Option<StoredDocument> {
        self.docs.read().ok().and_then(|docs| docs.get(id).cloned())
    }

    /// Liste les documents accessibles à un utilisateur.
    pub fn list_for_user(&self, user_id: &str) -> Vec<DocumentBase> {
        self.docs
            .read()
            .map(|docs| {
                docs.values()
                    .filter(|d| d.meta.acl.role_for(user_id).is_some())
                    .map(|d| d.meta.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Supprime un document (si l'utilisateur est Owner).
    pub fn delete_doc(&self, id: &str, user_id: &str) -> bool {
        let can_delete = self
            .docs
            .read()
            .ok()
            .and_then(|docs| docs.get(id).cloned())
            .map(|d| {
                d.meta
                    .acl
                    .role_for(user_id)
                    .map(|r| r.can_delete())
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        if !can_delete {
            return false;
        }

        if let Ok(mut docs) = self.docs.write() {
            docs.remove(id);
        }
        let json_path = self.storage_dir.join(format!("{id}.json"));
        let bin_path = self.storage_dir.join(format!("{id}.bin"));
        std::fs::remove_file(&json_path).ok();
        std::fs::remove_file(&bin_path).ok();
        true
    }

    /// Sauvegarde un document sur disque.
    pub fn save_to_disk(&self, doc: &StoredDocument) -> std::io::Result<()> {
        let json_path = self.storage_dir.join(format!("{}.json", doc.meta.id));
        let bin_path = self.storage_dir.join(format!("{}.bin", doc.meta.id));
        let json = serde_json::to_string_pretty(&doc.meta)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        std::fs::write(json_path, json)?;
        if !doc.snapshot.is_empty() {
            std::fs::write(bin_path, &doc.snapshot)?;
        }
        Ok(())
    }

    /// Stats globales.
    pub async fn stats(&self) -> JayBureauStats {
        let doc_count = self.docs.read().map(|d| d.len()).unwrap_or(0);
        let room_count = self.rooms.count().await;
        JayBureauStats {
            doc_count,
            active_rooms: room_count,
        }
    }
}

/// Statistiques globales pour monitoring.
#[derive(Debug, serde::Serialize)]
pub struct JayBureauStats {
    pub doc_count: usize,
    pub active_rooms: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use jaybureau_core::DocKind;

    #[test]
    fn create_and_retrieve_doc() {
        let tmp = std::env::temp_dir().join(format!("jaybureau-test-{}", uuid::Uuid::new_v4()));
        let state = JayBureauState::new(&tmp);
        let doc = state.create_doc(DocKind::Doc, "Mon document".into(), "alice".into());
        assert_eq!(doc.meta.title, "Mon document");

        let got = state.get_doc(&doc.meta.id).unwrap();
        assert_eq!(got.meta.id, doc.meta.id);
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn list_for_user_filters_by_acl() {
        let tmp = std::env::temp_dir().join(format!("jaybureau-test-{}", uuid::Uuid::new_v4()));
        let state = JayBureauState::new(&tmp);
        let _d1 = state.create_doc(DocKind::Doc, "Alice's doc".into(), "alice".into());
        let _d2 = state.create_doc(DocKind::Sheet, "Bob's sheet".into(), "bob".into());

        let alice_docs = state.list_for_user("alice");
        assert_eq!(alice_docs.len(), 1);
        assert_eq!(alice_docs[0].title, "Alice's doc");
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn delete_requires_owner() {
        let tmp = std::env::temp_dir().join(format!("jaybureau-test-{}", uuid::Uuid::new_v4()));
        let state = JayBureauState::new(&tmp);
        let doc = state.create_doc(DocKind::Doc, "Test".into(), "alice".into());

        assert!(!state.delete_doc(&doc.meta.id, "bob"));
        assert!(state.get_doc(&doc.meta.id).is_some());

        assert!(state.delete_doc(&doc.meta.id, "alice"));
        assert!(state.get_doc(&doc.meta.id).is_none());
        std::fs::remove_dir_all(&tmp).ok();
    }
}
