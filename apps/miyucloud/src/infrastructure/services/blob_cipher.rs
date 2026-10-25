//! @id     miyucloud_infrastructure_services_blob_cipher
//! @do     chiffrer_blobs_au_repos
//! @role   securite
//! @layer  app
//! @human  miyucloud — chiffrement au repos des blobs (AES-256-GCM par segments, format versionné « MIYUCENC » v1), lecture transparente des blobs clairs hérités.

//! Chiffrement au repos des blobs (ISO 27001 A.8.24, RGPD art. 32).
//!
//! Activé par `MIYUCLOUD_ENCRYPTION_ENABLED=true` avec une clé
//! `MIYUCLOUD_ENCRYPTION_KEY` (base64 standard, **exactement 32 octets**) ;
//! toute autre clé fait échouer le démarrage (fermé par défaut).
//!
//! # Format v1 (`MIYUCENC` 0x01)
//!
//! ```text
//! en-tête (16 octets) : "MIYUCENC" (8) | version = 0x01 (1) | préfixe de nonce aléatoire (7)
//! segments            : AES-256-GCM(segment clair ≤ 64 Kio) ‖ étiquette (16), l'un après l'autre
//! ```
//!
//! - Le clair est découpé en segments de 64 Kio ; le dernier (éventuellement
//!   vide) est marqué. Il y a toujours au moins un segment.
//! - Nonce du segment `i` (construction STREAM) : préfixe (7) ‖ `i` en
//!   gros-boutiste (4) ‖ drapeau « dernier » (1). Toute permutation,
//!   troncature ou extension est détectée par l'authentification.
//! - Données associées : l'en-tête complet (lie version et préfixe).
//! - Taille claire déduite de la taille chiffrée (voir [`taille_claire`]) :
//!   lecture par plage sans déchiffrer ce qui précède.
//!
//! # Blobs hérités
//!
//! Un blob qui ne commence pas par l'en-tête est un blob **clair** écrit
//! avant l'activation : il est relu tel quel (lecture transparente). Le
//! hachage (BLAKE3) reste celui du clair, la déduplication est inchangée ;
//! un contenu déjà présent en clair n'est pas réécrit chiffré.
//!
//! Un blob chiffré lu sans clé, ou avec une autre clé, est **refusé** (jamais
//! servi brut).

use std::io::{Read, Write};
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, OsRng, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use bytes::Bytes;
use futures::Stream;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::common::config::EncryptionConfig;

/// Marqueur de début d'un blob chiffré.
pub const MAGIC: &[u8; 8] = b"MIYUCENC";
/// Version du format décrite ci-dessus.
pub const VERSION_V1: u8 = 0x01;
/// Taille de l'en-tête (marqueur + version + préfixe de nonce).
pub const TAILLE_ENTETE: usize = 16;
/// Taille d'un segment clair.
pub const TAILLE_SEGMENT: usize = 64 * 1024;
/// Taille de l'étiquette GCM.
pub const TAILLE_ETIQUETTE: usize = 16;
/// Taille d'un segment chiffré complet.
const TAILLE_SEGMENT_CHIFFRE: u64 = (TAILLE_SEGMENT + TAILLE_ETIQUETTE) as u64;
/// Taille exigée de la clé (AES-256).
pub const TAILLE_CLE: usize = 32;

/// Flux d'octets clairs.
pub type FluxClair = Pin<Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send>>;

fn erreur_io(msg: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, msg.into())
}

/// Vrai si `debut` commence par l'en-tête d'un blob chiffré (toutes versions).
pub fn est_chiffre(debut: &[u8]) -> bool {
    debut.len() >= MAGIC.len() && &debut[..MAGIC.len()] == MAGIC
}

/// Géométrie d'un blob chiffré, déduite de sa taille sur disque.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geometrie {
    /// Nombre de segments (≥ 1).
    pub segments: u64,
    /// Taille chiffrée du dernier segment (étiquette comprise).
    pub dernier_chiffre: u64,
    /// Taille claire totale.
    pub taille_claire: u64,
}

/// Géométrie d'un blob chiffré de `taille_fichier` octets (en-tête compris).
pub fn geometrie(taille_fichier: u64) -> Result<Geometrie, std::io::Error> {
    let corps = taille_fichier
        .checked_sub(TAILLE_ENTETE as u64)
        .filter(|c| *c >= TAILLE_ETIQUETTE as u64)
        .ok_or_else(|| erreur_io("blob chiffré tronqué"))?;
    let segments = corps.div_ceil(TAILLE_SEGMENT_CHIFFRE);
    let dernier_chiffre = corps - (segments - 1) * TAILLE_SEGMENT_CHIFFRE;
    if dernier_chiffre < TAILLE_ETIQUETTE as u64 {
        return Err(erreur_io("blob chiffré : segment final invalide"));
    }
    Ok(Geometrie {
        segments,
        dernier_chiffre,
        taille_claire: corps - segments * TAILLE_ETIQUETTE as u64,
    })
}

/// Taille claire d'un blob chiffré de `taille_fichier` octets.
pub fn taille_claire(taille_fichier: u64) -> Result<u64, std::io::Error> {
    geometrie(taille_fichier).map(|g| g.taille_claire)
}

/// Taille chiffrée (en-tête compris) d'un clair de `taille` octets.
pub fn taille_chiffree(taille: u64) -> u64 {
    let segments = taille.div_ceil(TAILLE_SEGMENT as u64).max(1);
    TAILLE_ENTETE as u64 + taille + segments * TAILLE_ETIQUETTE as u64
}

/// Chiffreur de blobs (clé maîtresse AES-256).
pub struct BlobCipher {
    aead: Aes256Gcm,
}

impl std::fmt::Debug for BlobCipher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Jamais la clé.
        f.write_str("BlobCipher(aes-256-gcm, MIYUCENC v1)")
    }
}

impl BlobCipher {
    /// Chiffreur à partir d'une clé brute de exactement 32 octets.
    pub fn from_key(cle: &[u8]) -> Result<Self, String> {
        if cle.len() != TAILLE_CLE {
            return Err(format!(
                "MIYUCLOUD_ENCRYPTION_KEY doit faire exactement {TAILLE_CLE} octets (256 bits) une fois décodée, reçu {} octets. Générer avec : openssl rand -base64 32",
                cle.len()
            ));
        }
        let aead = Aes256Gcm::new_from_slice(cle)
            .map_err(|_| "MIYUCLOUD_ENCRYPTION_KEY : clé AES-256 invalide".to_string())?;
        Ok(Self { aead })
    }

    /// Chiffreur à partir d'une clé base64 (standard, avec remplissage).
    pub fn from_key_base64(cle_b64: &str) -> Result<Self, String> {
        let cle = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, cle_b64.trim())
            .map_err(|_| {
                "MIYUCLOUD_ENCRYPTION_KEY n'est pas du base64 valide. Générer avec : openssl rand -base64 32"
                    .to_string()
            })?;
        Self::from_key(&cle)
    }

    /// Chiffreur de la configuration : `None` si le chiffrement est désactivé,
    /// erreur (démarrage refusé) si activé avec une clé absente ou invalide,
    /// ou un algorithme inconnu.
    pub fn from_config(config: &EncryptionConfig) -> Result<Option<Arc<Self>>, String> {
        if !config.enabled {
            return Ok(None);
        }
        if config.key_base64.trim().is_empty() {
            return Err(
                "MIYUCLOUD_ENCRYPTION_ENABLED=true mais MIYUCLOUD_ENCRYPTION_KEY est vide. Générer avec : openssl rand -base64 32"
                    .to_string(),
            );
        }
        if !config.algorithm.eq_ignore_ascii_case("aes-256-gcm") {
            return Err(format!(
                "MIYUCLOUD_ENCRYPTION_ALGORITHM « {} » non pris en charge (seul aes-256-gcm l'est)",
                config.algorithm
            ));
        }
        Self::from_key_base64(&config.key_base64).map(|c| Some(Arc::new(c)))
    }

    fn nonce(
        entete: &[u8; TAILLE_ENTETE],
        index: u64,
        dernier: bool,
    ) -> Result<[u8; 12], std::io::Error> {
        let index =
            u32::try_from(index).map_err(|_| erreur_io("blob chiffré : trop de segments"))?;
        let mut n = [0u8; 12];
        n[..7].copy_from_slice(&entete[9..16]);
        n[7..11].copy_from_slice(&index.to_be_bytes());
        n[11] = u8::from(dernier);
        Ok(n)
    }

    fn nouvel_entete() -> [u8; TAILLE_ENTETE] {
        let mut entete = [0u8; TAILLE_ENTETE];
        entete[..8].copy_from_slice(MAGIC);
        entete[8] = VERSION_V1;
        OsRng.fill_bytes(&mut entete[9..16]);
        entete
    }

    fn verifier_entete(entete: &[u8; TAILLE_ENTETE]) -> Result<(), std::io::Error> {
        if !est_chiffre(entete) {
            return Err(erreur_io("blob chiffré : marqueur absent"));
        }
        if entete[8] != VERSION_V1 {
            return Err(erreur_io(format!(
                "blob chiffré : version de format {} inconnue",
                entete[8]
            )));
        }
        Ok(())
    }

    fn chiffrer_segment(
        &self,
        entete: &[u8; TAILLE_ENTETE],
        index: u64,
        dernier: bool,
        clair: &[u8],
    ) -> Result<Vec<u8>, std::io::Error> {
        let nonce = Self::nonce(entete, index, dernier)?;
        self.aead
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: clair,
                    aad: entete,
                },
            )
            .map_err(|_| erreur_io("chiffrement du segment impossible"))
    }

    fn dechiffrer_segment(
        &self,
        entete: &[u8; TAILLE_ENTETE],
        index: u64,
        dernier: bool,
        chiffre: &[u8],
    ) -> Result<Vec<u8>, std::io::Error> {
        let nonce = Self::nonce(entete, index, dernier)?;
        self.aead
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: chiffre,
                    aad: entete,
                },
            )
            .map_err(|_| {
                erreur_io("blob chiffré : authentification refusée (clé différente ou blob altéré)")
            })
    }

    /// Chiffre `clair` en mémoire (format v1 complet).
    pub fn chiffrer_octets(&self, clair: &[u8]) -> Result<Vec<u8>, std::io::Error> {
        let mut sortie = Vec::with_capacity(
            usize::try_from(taille_chiffree(clair.len() as u64)).unwrap_or(clair.len()),
        );
        self.chiffrer_vers(&mut std::io::Cursor::new(clair), &mut sortie)?;
        Ok(sortie)
    }

    /// Chiffre le flux `source` vers `dest` (synchrone, mémoire bornée à
    /// deux segments).
    pub fn chiffrer_vers<R: Read, W: Write>(
        &self,
        source: &mut R,
        dest: &mut W,
    ) -> std::io::Result<()> {
        let entete = Self::nouvel_entete();
        dest.write_all(&entete)?;
        let mut courant = lire_segment(source)?;
        let mut index = 0u64;
        loop {
            let suivant = if courant.len() == TAILLE_SEGMENT {
                lire_segment(source)?
            } else {
                Vec::new()
            };
            let dernier = courant.len() < TAILLE_SEGMENT || suivant.is_empty();
            dest.write_all(&self.chiffrer_segment(&entete, index, dernier, &courant)?)?;
            if dernier {
                return Ok(());
            }
            courant = suivant;
            index += 1;
        }
    }

    /// Chiffre le fichier `source` dans `dest` (créé, synchronisé sur disque).
    pub fn chiffrer_fichier(&self, source: &Path, dest: &Path) -> std::io::Result<()> {
        let mut entree = std::io::BufReader::new(std::fs::File::open(source)?);
        let fichier = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dest)?;
        let mut sortie = std::io::BufWriter::new(fichier);
        self.chiffrer_vers(&mut entree, &mut sortie)?;
        let fichier = sortie.into_inner().map_err(|e| e.into_error())?;
        fichier.sync_all()
    }

    /// Déchiffre un blob chiffré complet (`source`, en-tête compris, de
    /// `taille_fichier` octets) vers `dest`, segment par segment.
    pub fn dechiffrer_vers<R: Read, W: Write>(
        &self,
        source: &mut R,
        taille_fichier: u64,
        dest: &mut W,
    ) -> std::io::Result<u64> {
        let geo = geometrie(taille_fichier)?;
        let mut entete = [0u8; TAILLE_ENTETE];
        source.read_exact(&mut entete)?;
        Self::verifier_entete(&entete)?;
        let mut tampon = vec![0u8; TAILLE_SEGMENT_CHIFFRE as usize];
        let mut total = 0u64;
        for index in 0..geo.segments {
            let dernier = index + 1 == geo.segments;
            let longueur = if dernier {
                geo.dernier_chiffre
            } else {
                TAILLE_SEGMENT_CHIFFRE
            } as usize;
            source.read_exact(&mut tampon[..longueur])?;
            let clair = self.dechiffrer_segment(&entete, index, dernier, &tampon[..longueur])?;
            total += clair.len() as u64;
            dest.write_all(&clair)?;
        }
        Ok(total)
    }

    /// Déchiffre `chiffre` en mémoire.
    pub fn dechiffrer_octets(&self, chiffre: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut sortie = Vec::new();
        self.dechiffrer_vers(
            &mut std::io::Cursor::new(chiffre),
            chiffre.len() as u64,
            &mut sortie,
        )?;
        Ok(sortie)
    }
}

/// Lit jusqu'à un segment clair complet (moins seulement en fin de flux).
fn lire_segment<R: Read>(source: &mut R) -> std::io::Result<Vec<u8>> {
    let mut tampon = vec![0u8; TAILLE_SEGMENT];
    let mut lu = 0;
    while lu < TAILLE_SEGMENT {
        match source.read(&mut tampon[lu..]) {
            Ok(0) => break,
            Ok(n) => lu += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    tampon.truncate(lu);
    Ok(tampon)
}

/// Lit l'en-tête éventuel d'un fichier ouvert : `Some(en-tête)` s'il est
/// chiffré ; sinon le curseur est remis au début.
async fn detecter(fichier: &mut tokio::fs::File) -> std::io::Result<Option<[u8; TAILLE_ENTETE]>> {
    let mut entete = [0u8; TAILLE_ENTETE];
    let mut lu = 0;
    while lu < TAILLE_ENTETE {
        let n = fichier.read(&mut entete[lu..]).await?;
        if n == 0 {
            break;
        }
        lu += n;
    }
    if est_chiffre(&entete[..lu]) {
        if lu < TAILLE_ENTETE {
            return Err(erreur_io("blob chiffré tronqué"));
        }
        BlobCipher::verifier_entete(&entete)?;
        return Ok(Some(entete));
    }
    fichier.seek(std::io::SeekFrom::Start(0)).await?;
    Ok(None)
}

fn refus_sans_cle() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "blob chiffré au repos mais aucune clé configurée (MIYUCLOUD_ENCRYPTION_ENABLED/KEY) : lecture refusée",
    )
}

/// Taille claire du blob `chemin` (chiffré ou clair hérité).
pub async fn taille_claire_fichier(chemin: &Path) -> std::io::Result<u64> {
    let mut fichier = tokio::fs::File::open(chemin).await?;
    let taille = fichier.metadata().await?.len();
    match detecter(&mut fichier).await? {
        Some(_) => taille_claire(taille),
        None => Ok(taille),
    }
}

/// Flux clair de la plage `[debut, fin)` (`fin` exclue, `None` = jusqu'au
/// bout) du blob `chemin`, chiffré ou clair hérité. Mémoire bornée à un
/// segment ; seuls les segments couvrant la plage sont lus et déchiffrés.
pub async fn ouvrir_flux_clair(
    chemin: &Path,
    chiffreur: Option<Arc<BlobCipher>>,
    debut: u64,
    fin: Option<u64>,
    taille_tranche: usize,
) -> std::io::Result<FluxClair> {
    let mut fichier = tokio::fs::File::open(chemin).await?;
    let taille_fichier = fichier.metadata().await?.len();
    let Some(entete) = detecter(&mut fichier).await? else {
        // Blob clair hérité : lecture directe.
        fichier.seek(std::io::SeekFrom::Start(debut)).await?;
        return Ok(match fin {
            Some(f) => Box::pin(tokio_util::io::ReaderStream::with_capacity(
                fichier.take(f.saturating_sub(debut)),
                taille_tranche,
            )),
            None => Box::pin(tokio_util::io::ReaderStream::with_capacity(
                fichier,
                taille_tranche,
            )),
        });
    };
    let chiffreur = chiffreur.ok_or_else(refus_sans_cle)?;
    let geo = geometrie(taille_fichier)?;
    let fin = fin.map_or(geo.taille_claire, |f| f.min(geo.taille_claire));
    if debut >= fin {
        return Ok(Box::pin(futures::stream::empty()));
    }
    let premier = debut / TAILLE_SEGMENT as u64;
    fichier
        .seek(std::io::SeekFrom::Start(
            TAILLE_ENTETE as u64 + premier * TAILLE_SEGMENT_CHIFFRE,
        ))
        .await?;
    let etat = EtatFlux {
        fichier,
        chiffreur,
        entete,
        geo,
        index: premier,
        saut: (debut % TAILLE_SEGMENT as u64) as usize,
        restant: fin - debut,
    };
    Ok(Box::pin(futures::stream::try_unfold(etat, segment_suivant)))
}

struct EtatFlux {
    fichier: tokio::fs::File,
    chiffreur: Arc<BlobCipher>,
    entete: [u8; TAILLE_ENTETE],
    geo: Geometrie,
    index: u64,
    saut: usize,
    restant: u64,
}

async fn segment_suivant(mut etat: EtatFlux) -> std::io::Result<Option<(Bytes, EtatFlux)>> {
    if etat.restant == 0 || etat.index >= etat.geo.segments {
        return Ok(None);
    }
    let dernier = etat.index + 1 == etat.geo.segments;
    let longueur = if dernier {
        etat.geo.dernier_chiffre
    } else {
        TAILLE_SEGMENT_CHIFFRE
    } as usize;
    let mut chiffre = vec![0u8; longueur];
    etat.fichier.read_exact(&mut chiffre).await?;
    let clair = etat
        .chiffreur
        .dechiffrer_segment(&etat.entete, etat.index, dernier, &chiffre)?;
    let debut = etat.saut.min(clair.len());
    let utile = (clair.len() - debut).min(usize::try_from(etat.restant).unwrap_or(usize::MAX));
    let morceau = Bytes::copy_from_slice(&clair[debut..debut + utile]);
    etat.restant -= utile as u64;
    etat.saut = 0;
    etat.index += 1;
    Ok(Some((morceau, etat)))
}

/// Lit entièrement le blob `chemin` en clair (synchrone ; vignettes).
pub fn lire_clair_sync(chemin: &Path, chiffreur: Option<&BlobCipher>) -> std::io::Result<Vec<u8>> {
    let donnees = std::fs::read(chemin)?;
    if !est_chiffre(&donnees) {
        return Ok(donnees);
    }
    chiffreur
        .ok_or_else(refus_sans_cle)?
        .dechiffrer_octets(&donnees)
}

/// Copie le blob `chemin` en clair dans `dest` (synchrone, mémoire bornée).
pub fn copier_clair_sync(
    chemin: &Path,
    chiffreur: Option<&BlobCipher>,
    dest: &Path,
) -> std::io::Result<u64> {
    let mut entree = std::fs::File::open(chemin)?;
    let taille = entree.metadata()?.len();
    let mut debut = [0u8; MAGIC.len()];
    let lu = lire_au_plus(&mut entree, &mut debut)?;
    let mut sortie = std::io::BufWriter::new(std::fs::File::create(dest)?);
    let total = if est_chiffre(&debut[..lu]) {
        let chiffreur = chiffreur.ok_or_else(refus_sans_cle)?;
        let mut source = std::io::BufReader::new(Read::chain(std::io::Cursor::new(debut), entree));
        chiffreur.dechiffrer_vers(&mut source, taille, &mut sortie)?
    } else {
        sortie.write_all(&debut[..lu])?;
        lu as u64 + std::io::copy(&mut entree, &mut sortie)?
    };
    sortie.flush()?;
    Ok(total)
}

/// BLAKE3 (hex) et taille du **clair** du blob `chemin` (synchrone ;
/// vérification d'intégrité).
pub fn hacher_clair_sync(
    chemin: &Path,
    chiffreur: Option<&BlobCipher>,
) -> std::io::Result<(String, u64)> {
    let mut entree = std::fs::File::open(chemin)?;
    let taille = entree.metadata()?.len();
    let mut debut = [0u8; MAGIC.len()];
    let lu = lire_au_plus(&mut entree, &mut debut)?;
    let mut hacheur = blake3::Hasher::new();
    let total = if est_chiffre(&debut[..lu]) {
        let chiffreur = chiffreur.ok_or_else(refus_sans_cle)?;
        let mut source = std::io::BufReader::new(Read::chain(std::io::Cursor::new(debut), entree));
        chiffreur.dechiffrer_vers(&mut source, taille, &mut hacheur)?
    } else {
        hacheur.update(&debut[..lu]);
        lu as u64 + std::io::copy(&mut entree, &mut hacheur)?
    };
    Ok((hacheur.finalize().to_hex().to_string(), total))
}

fn lire_au_plus<R: Read>(source: &mut R, tampon: &mut [u8]) -> std::io::Result<usize> {
    let mut lu = 0;
    while lu < tampon.len() {
        match source.read(&mut tampon[lu..]) {
            Ok(0) => break,
            Ok(n) => lu += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(lu)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;

    fn chiffreur(octet: u8) -> Arc<BlobCipher> {
        Arc::new(BlobCipher::from_key(&[octet; 32]).expect("clé de test"))
    }

    fn motif(n: usize) -> Vec<u8> {
        (0..n).map(|i| (i % 251) as u8).collect()
    }

    async fn lire(
        chemin: &Path,
        c: Option<Arc<BlobCipher>>,
        debut: u64,
        fin: Option<u64>,
    ) -> std::io::Result<Vec<u8>> {
        let flux = ouvrir_flux_clair(chemin, c, debut, fin, 4096).await?;
        let morceaux: Vec<Bytes> = flux.try_collect().await?;
        Ok(morceaux.concat())
    }

    #[test]
    fn cle_de_longueur_exacte_exigee() {
        assert!(BlobCipher::from_key(&[1u8; 16]).is_err());
        assert!(BlobCipher::from_key(&[1u8; 33]).is_err());
        assert!(BlobCipher::from_key(&[1u8; 32]).is_ok());
        let b64 = |n: usize| {
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, vec![7u8; n])
        };
        assert!(BlobCipher::from_key_base64(&b64(31)).is_err());
        assert!(BlobCipher::from_key_base64("pas du base64 !").is_err());
        assert!(BlobCipher::from_key_base64(&b64(32)).is_ok());
    }

    #[test]
    fn configuration_fermee_par_defaut() {
        let desactive = EncryptionConfig::default();
        assert!(matches!(BlobCipher::from_config(&desactive), Ok(None)));
        let mut cfg = EncryptionConfig {
            enabled: true,
            ..EncryptionConfig::default()
        };
        assert!(BlobCipher::from_config(&cfg).is_err(), "clé vide");
        cfg.key_base64 =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, [9u8; 16]);
        assert!(BlobCipher::from_config(&cfg).is_err(), "clé de 16 octets");
        cfg.key_base64 =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, [9u8; 32]);
        assert!(matches!(BlobCipher::from_config(&cfg), Ok(Some(_))));
        cfg.algorithm = "chacha20".into();
        assert!(BlobCipher::from_config(&cfg).is_err(), "algorithme inconnu");
    }

    #[test]
    fn aller_retour_toutes_tailles_et_geometrie() {
        let c = chiffreur(0x42);
        for n in [
            0,
            1,
            TAILLE_SEGMENT - 1,
            TAILLE_SEGMENT,
            TAILLE_SEGMENT + 1,
            3 * TAILLE_SEGMENT,
            3 * TAILLE_SEGMENT + 17,
        ] {
            let clair = motif(n);
            let chiffre = c.chiffrer_octets(&clair).expect("chiffrement");
            assert!(est_chiffre(&chiffre));
            assert_eq!(
                chiffre.len() as u64,
                taille_chiffree(n as u64),
                "taille n={n}"
            );
            assert_eq!(taille_claire(chiffre.len() as u64).expect("géo"), n as u64);
            if n > 64 {
                assert!(
                    !chiffre.windows(64).any(|w| w == &clair[..64]),
                    "le clair n'apparaît pas"
                );
            }
            assert_eq!(c.dechiffrer_octets(&chiffre).expect("déchiffrement"), clair);
        }
    }

    #[test]
    fn mauvaise_cle_et_alteration_refusees() {
        let clair = motif(2 * TAILLE_SEGMENT + 5);
        let chiffre = chiffreur(1).chiffrer_octets(&clair).expect("chiffrement");
        assert!(
            chiffreur(2).dechiffrer_octets(&chiffre).is_err(),
            "autre clé"
        );
        let mut altere = chiffre.clone();
        altere[TAILLE_ENTETE + 3] ^= 1;
        assert!(
            chiffreur(1).dechiffrer_octets(&altere).is_err(),
            "bit modifié"
        );
        // Troncature à une frontière de segment : le segment devenu dernier
        // n'était pas marqué « dernier ».
        let tronque = &chiffre[..TAILLE_ENTETE + 2 * TAILLE_SEGMENT_CHIFFRE as usize];
        assert!(
            chiffreur(1).dechiffrer_octets(tronque).is_err(),
            "troncature"
        );
        let mut version = chiffre.clone();
        version[8] = 2;
        assert!(
            chiffreur(1).dechiffrer_octets(&version).is_err(),
            "version inconnue"
        );
    }

    #[tokio::test]
    async fn flux_par_plage_sur_blob_chiffre() {
        let dir = tempfile::tempdir().expect("tempdir");
        let c = chiffreur(3);
        let clair = motif(3 * TAILLE_SEGMENT + 1000);
        let chemin = dir.path().join("b.blob");
        std::fs::write(&chemin, c.chiffrer_octets(&clair).expect("chiffrement")).expect("écriture");

        assert_eq!(
            lire(&chemin, Some(c.clone()), 0, None).await.expect("tout"),
            clair
        );
        let (d, f) = (TAILLE_SEGMENT as u64 - 10, 2 * TAILLE_SEGMENT as u64 + 33);
        assert_eq!(
            lire(&chemin, Some(c.clone()), d, Some(f))
                .await
                .expect("plage"),
            clair[d as usize..f as usize]
        );
        let d = clair.len() as u64 - 5;
        assert_eq!(
            lire(&chemin, Some(c.clone()), d, Some(u64::MAX))
                .await
                .expect("fin"),
            clair[d as usize..]
        );
        assert!(
            lire(&chemin, Some(c.clone()), clair.len() as u64, None)
                .await
                .expect("vide")
                .is_empty()
        );
        assert_eq!(
            taille_claire_fichier(&chemin).await.expect("taille"),
            clair.len() as u64
        );

        // Sans clé ou avec une autre clé : refus, jamais d'octets bruts.
        assert!(
            ouvrir_flux_clair(&chemin, None, 0, None, 4096)
                .await
                .is_err()
        );
        assert!(lire(&chemin, Some(chiffreur(4)), 0, None).await.is_err());
    }

    #[tokio::test]
    async fn blob_clair_herite_relu_tel_quel() {
        let dir = tempfile::tempdir().expect("tempdir");
        let clair = motif(100_000);
        let chemin = dir.path().join("h.blob");
        std::fs::write(&chemin, &clair).expect("écriture");
        let c = chiffreur(5);
        assert_eq!(
            lire(&chemin, Some(c.clone()), 0, None).await.expect("tout"),
            clair
        );
        assert_eq!(
            lire(&chemin, None, 10, Some(20)).await.expect("plage"),
            clair[10..20]
        );
        assert_eq!(
            taille_claire_fichier(&chemin).await.expect("taille"),
            100_000
        );
        assert_eq!(lire_clair_sync(&chemin, Some(&c)).expect("sync"), clair);
        let (h, n) = hacher_clair_sync(&chemin, None).expect("hachage");
        assert_eq!((h, n), (blake3::hash(&clair).to_hex().to_string(), 100_000));
        // Fichier plus court que le marqueur.
        let court = dir.path().join("c.blob");
        std::fs::write(&court, b"MIY").expect("écriture");
        assert_eq!(lire(&court, None, 0, None).await.expect("court"), b"MIY");
    }

    #[test]
    fn fichiers_synchrones_chiffrer_copier_hacher() {
        let dir = tempfile::tempdir().expect("tempdir");
        let c = chiffreur(6);
        let clair = motif(TAILLE_SEGMENT * 2 + 3);
        let source = dir.path().join("s");
        let blob = dir.path().join("b");
        let copie = dir.path().join("c");
        std::fs::write(&source, &clair).expect("écriture");
        c.chiffrer_fichier(&source, &blob).expect("chiffrement");
        assert!(
            c.chiffrer_fichier(&source, &blob).is_err(),
            "jamais d'écrasement"
        );
        assert_eq!(
            copier_clair_sync(&blob, Some(&c), &copie).expect("copie"),
            clair.len() as u64
        );
        assert_eq!(std::fs::read(&copie).expect("lecture"), clair);
        assert!(copier_clair_sync(&blob, None, &copie).is_err(), "sans clé");
        let (h, n) = hacher_clair_sync(&blob, Some(&c)).expect("hachage");
        assert_eq!(h, blake3::hash(&clair).to_hex().to_string());
        assert_eq!(n, clair.len() as u64);
        assert_eq!(lire_clair_sync(&blob, Some(&c)).expect("lecture"), clair);
        assert!(
            lire_clair_sync(&blob, Some(&chiffreur(7))).is_err(),
            "autre clé"
        );
    }
}
