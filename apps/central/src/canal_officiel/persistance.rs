//! @id     central_canal_officiel_persistance
//! @do     persister_etat_canal_officiel
//! @role   securite
//! @layer  app
//! @human  central — module « persistance » du canal officiel (secret local 0600, état du registre officiel, horloge de confiance).

//! Persistance du canal officiel dans le dossier de données de Central
//! (`<données>/canal-officiel/`, dossier `0700`, fichiers `0600`, écriture
//! atomique fichier temporaire + renommage).
//!
//! | Fichier | Contenu |
//! |---|---|
//! | `secret-local` | secret local du COG (32 o aléatoires), créé **une fois** ; dérive la clé HMAC de la quarantaine (A-006) |
//! | `miroir.mrgj` | miroir intégral du registre officiel (`StockageFichier`, format `MRGJ`) |
//! | `etat.mcce` | état du relevé, horloge de confiance comprise (format ci-dessous) |
//! | `quarantaine.mcqt` | annonces en attente scellées (format `MCQT`, A-006) |
//!
//! # `etat.mcce` (non spécifié par la doc gelée, à amender)
//!
//! ```text
//! "MCCE" | version 0x01 | 3 × ( longueur (4, BE, ≤ 1 Mio) | octets )
//!   1. EtatRegistre (`MSRG`)   2. fenêtre MTP du registre (`MSMT`)
//!   3. horloge de confiance (`HorlogeConfiance::encoder`)
//! ```
//!
//! Un seul fichier, remplacé atomiquement : jamais d'état partiel. Son
//! contenu reste revérifié au chargement (signatures de la tête, registre
//! attendu : `Releveur::reprendre`).

use std::path::Path;

/// Secret local du COG.
pub const FICHIER_SECRET: &str = "secret-local";
/// Miroir du registre officiel.
pub const FICHIER_MIROIR: &str = "miroir.mrgj";
/// État du relevé.
pub const FICHIER_ETAT: &str = "etat.mcce";
/// État de quarantaine scellé.
pub const FICHIER_QUARANTAINE: &str = "quarantaine.mcqt";

const MAGIC: &[u8; 4] = b"MCCE";
const VERSION: u8 = 0x01;
const TAILLE_MAX_PARTIE: usize = 1 << 20;

/// État du relevé à persister.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtatCanal {
    /// `EtatRegistre` encodé.
    pub etat: Vec<u8>,
    /// Fenêtre MTP du registre officiel.
    pub temps: Vec<u8>,
    /// Horloge de confiance.
    pub horloge: Vec<u8>,
}

impl EtatCanal {
    /// Encodage (voir module).
    pub fn encoder(&self) -> Vec<u8> {
        let mut s = MAGIC.to_vec();
        s.push(VERSION);
        for partie in [&self.etat, &self.temps, &self.horloge] {
            s.extend_from_slice(&(partie.len() as u32).to_be_bytes());
            s.extend_from_slice(partie);
        }
        s
    }

    /// Décodage strict (taille exacte, bornes) ; `None` = illisible.
    pub fn decoder(octets: &[u8]) -> Option<Self> {
        if octets.len() < 5 || &octets[0..4] != MAGIC || octets[4] != VERSION {
            return None;
        }
        let mut pos = 5usize;
        let mut parties = Vec::with_capacity(3);
        for _ in 0..3 {
            let n = u32::from_be_bytes(octets.get(pos..pos + 4)?.try_into().ok()?) as usize;
            if n == 0 || n > TAILLE_MAX_PARTIE {
                return None;
            }
            parties.push(octets.get(pos + 4..pos + 4 + n)?.to_vec());
            pos += 4 + n;
        }
        if pos != octets.len() {
            return None;
        }
        let horloge = parties.pop()?;
        let temps = parties.pop()?;
        let etat = parties.pop()?;
        Some(Self { etat, temps, horloge })
    }
}

/// Lit un fichier facultatif (absent = `None`).
pub fn lire(chemin: &Path) -> Result<Option<Vec<u8>>, String> {
    match std::fs::read(chemin) {
        Ok(o) => Ok(Some(o)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("{} illisible : {e}", chemin.display())),
    }
}

/// Écrit atomiquement un fichier `0600`.
pub fn ecrire(chemin: &Path, octets: &[u8]) -> Result<(), String> {
    miyukini_central::fichiers_secrets::ecrire_secret(chemin, octets)
        .map_err(|e| format!("écriture de {} : {e}", chemin.display()))
}

/// Secret local du COG : créé une fois (`0600`, création exclusive), relu
/// ensuite. Fichier lien symbolique, d'une autre taille ou lisible par
/// d'autres ⇒ refus (fail-closed : le canal reste désactivé).
pub fn secret_local(dossier: &Path) -> Result<[u8; 32], String> {
    use std::io::Write;
    let chemin = dossier.join(FICHIER_SECRET);
    match std::fs::symlink_metadata(&chemin) {
        Ok(meta) => {
            if !meta.file_type().is_file() {
                return Err(format!("{} : pas un fichier régulier", chemin.display()));
            }
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if meta.permissions().mode() & 0o077 != 0 {
                    return Err(format!("{} : droits trop ouverts (0600 exigé)", chemin.display()));
                }
            }
            let octets = std::fs::read(&chemin).map_err(|e| format!("{} illisible : {e}", chemin.display()))?;
            octets.try_into().map_err(|_| format!("{} : 32 octets attendus", chemin.display()))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let mut secret = [0u8; 32];
            rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut secret);
            let mut options = std::fs::OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let ecrit = options.open(&chemin).and_then(|mut f| {
                f.write_all(&secret)?;
                f.sync_all()
            });
            if let Err(e) = ecrit {
                let _ = std::fs::remove_file(&chemin);
                return Err(format!("création de {} : {e}", chemin.display()));
            }
            tracing::info!("canal officiel : secret local créé ({})", chemin.display());
            Ok(secret)
        }
        Err(e) => Err(format!("{} illisible : {e}", chemin.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn etat_aller_retour_strict() {
        let e = EtatCanal { etat: vec![1, 2], temps: vec![3], horloge: vec![4, 5, 6] };
        let o = e.encoder();
        assert_eq!(EtatCanal::decoder(&o), Some(e));
        assert!(EtatCanal::decoder(&o[..o.len() - 1]).is_none());
        assert!(EtatCanal::decoder(&[o.as_slice(), &[0]].concat()).is_none());
        let mut v = o.clone();
        v[4] = 2;
        assert!(EtatCanal::decoder(&v).is_none());
    }

    #[test]
    fn secret_cree_une_fois_en_0600() {
        let d = std::env::temp_dir().join(format!("central-canal-secret-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        let a = secret_local(&d).unwrap();
        assert_eq!(secret_local(&d).unwrap(), a);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let f = d.join(FICHIER_SECRET);
            assert_eq!(std::fs::metadata(&f).unwrap().permissions().mode() & 0o777, 0o600);
            std::fs::set_permissions(&f, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(secret_local(&d).is_err(), "droits trop ouverts : refus");
            std::fs::set_permissions(&f, std::fs::Permissions::from_mode(0o600)).unwrap();
            std::fs::write(&f, b"court").unwrap();
            assert!(secret_local(&d).is_err());
        }
        let _ = std::fs::remove_dir_all(&d);
    }
}
