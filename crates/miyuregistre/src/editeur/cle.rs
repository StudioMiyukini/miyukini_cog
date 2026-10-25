//! @id     miyuregistre_editeur_cle
//! @do     lire_cle_journal_fichier_strict
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — lecture stricte du fichier de la clé de journal (O_NOFOLLOW, fichier ordinaire, 0600 ou plus strict, propriétaire = uid effectif, lecture bornée, tampon effacé).

//! Clé de journal lue depuis un fichier **strict** (jamais en argument ni en
//! variable d'environnement), mêmes règles que les fichiers secrets de la clé
//! racine (A-013) :
//!
//! - dernier composant ouvert en `O_NOFOLLOW` (lien symbolique ⇒ refus) ;
//! - contrôles sur le **descripteur ouvert** : fichier ordinaire, aucun droit
//!   pour le groupe ni les autres (`0600` ou plus strict), propriétaire = uid
//!   effectif ;
//! - au plus [`TAILLE_MAX_FICHIER_CLE`] octets lus, tampon effacé.
//!
//! Contenu : la **graine Ed25519** de 32 octets, brute (32 octets exactement)
//! ou en hexadécimal (64 caractères, blancs de fin tolérés).

use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use miyusceau::CleSignature;
use rustix::fs::{Mode, OFlags};
use zeroize::{Zeroize, Zeroizing};

/// Taille maximale lue dans le fichier de clé.
pub const TAILLE_MAX_FICHIER_CLE: u64 = 256;

/// Fichier de clé refusé.
#[derive(Debug, thiserror::Error)]
pub enum ErreurCle {
    /// Fichier refusé (lien, type, droits, propriétaire, contenu).
    #[error("fichier de clé de journal refusé : {0}")]
    Refuse(&'static str),
    /// Erreur d'entrée/sortie.
    #[error("fichier de clé de journal : {0}")]
    EntreeSortie(#[from] std::io::Error),
}

fn chiffre_hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// Graine contenue dans `octets` (32 octets bruts, ou 64 caractères hexadécimaux).
fn graine(octets: &[u8]) -> Option<Zeroizing<[u8; 32]>> {
    let mut g = Zeroizing::new([0u8; 32]);
    if octets.len() == 32 {
        g.copy_from_slice(octets);
        return Some(g);
    }
    let texte = octets.trim_ascii_end();
    if texte.len() != 64 {
        return None;
    }
    let (paires, _) = texte.as_chunks::<2>();
    for (i, paire) in paires.iter().enumerate() {
        g[i] = (chiffre_hex(paire[0])? << 4) | chiffre_hex(paire[1])?;
    }
    Some(g)
}

/// Lit la clé de journal dans `chemin` (voir le module).
pub fn lire_cle_journal(chemin: &Path) -> Result<CleSignature, ErreurCle> {
    let fd = rustix::fs::open(
        chemin,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK,
        Mode::empty(),
    )
    .map_err(|e| if e == rustix::io::Errno::LOOP { ErreurCle::Refuse("lien symbolique") } else { std::io::Error::from(e).into() })?;
    let fichier = File::from(fd);
    let meta = fichier.metadata()?;
    if !meta.file_type().is_file() {
        return Err(ErreurCle::Refuse("pas un fichier ordinaire"));
    }
    if meta.mode() & 0o077 != 0 {
        return Err(ErreurCle::Refuse("droits trop larges (attendu 0600, aucun accès groupe/autres)"));
    }
    if meta.uid() != rustix::process::geteuid().as_raw() {
        return Err(ErreurCle::Refuse("propriétaire différent de l'utilisateur effectif"));
    }
    if meta.len() > TAILLE_MAX_FICHIER_CLE {
        return Err(ErreurCle::Refuse("fichier trop grand"));
    }
    // Capacité fixée d'avance : aucune réallocation (pas de copie non effacée).
    let mut octets = Zeroizing::new(Vec::with_capacity(TAILLE_MAX_FICHIER_CLE as usize + 1));
    fichier.take(TAILLE_MAX_FICHIER_CLE + 1).read_to_end(&mut octets)?;
    if octets.len() as u64 > TAILLE_MAX_FICHIER_CLE {
        return Err(ErreurCle::Refuse("fichier trop grand"));
    }
    let mut g = graine(&octets).ok_or(ErreurCle::Refuse("contenu attendu : graine de 32 octets (brute ou 64 hex)"))?;
    let cle = CleSignature::depuis_graine(&g);
    g.zeroize();
    Ok(cle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn ecrire(dossier: &Path, nom: &str, contenu: &[u8], mode: u32) -> std::path::PathBuf {
        let p = dossier.join(nom);
        std::fs::write(&p, contenu).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(mode)).unwrap();
        p
    }

    #[test]
    fn fichier_strict() {
        let d = tempfile::tempdir().unwrap();
        let cle = CleSignature::generer();
        let hex = cle.graine().iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        });
        let p = ecrire(d.path(), "hex", format!("{hex}\n").as_bytes(), 0o600);
        assert_eq!(lire_cle_journal(&p).unwrap().cle_publique(), cle.cle_publique());
        let p = ecrire(d.path(), "brute", &cle.graine(), 0o400);
        assert_eq!(lire_cle_journal(&p).unwrap().cle_publique(), cle.cle_publique());
        let p = ecrire(d.path(), "large", &cle.graine(), 0o640);
        assert!(matches!(lire_cle_journal(&p), Err(ErreurCle::Refuse(_))));
        let lien = d.path().join("lien");
        std::os::unix::fs::symlink(d.path().join("brute"), &lien).unwrap();
        assert!(matches!(lire_cle_journal(&lien), Err(ErreurCle::Refuse("lien symbolique"))));
        let p = ecrire(d.path(), "court", b"abcd", 0o600);
        assert!(matches!(lire_cle_journal(&p), Err(ErreurCle::Refuse(_))));
        let p = ecrire(d.path(), "gros", &[b'a'; 300], 0o600);
        assert!(matches!(lire_cle_journal(&p), Err(ErreurCle::Refuse("fichier trop grand"))));
        assert!(matches!(lire_cle_journal(d.path()), Err(ErreurCle::Refuse(_))));
    }
}
