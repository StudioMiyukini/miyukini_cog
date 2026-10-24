//! @id     miyumarket_signature
//! @do     signer_verifier_paquet
//! @role   securite
//! @layer  core
//! @human  miyumarket — signature obligatoire des paquets Market (Sceau v1, type 0x03).

//! Paquets signés du Market (CA-01, EX-09, EX-26, C-23, C-34).
//!
//! # Format « paquet signé » (`.msp` v2)
//!
//! ```text
//! "MSPS" (4) | version 0x01 (1) | longueur du sceau u32 BE (4) | sceau (n) | archive ZIP (reste)
//! ```
//!
//! Le sceau est un titre Sceau v1 de type `ManifestePaquet` (0x03) signé par
//! la clé d'un **éditeur de confiance** épinglée dans Central. Sa charge TLV :
//!
//! | Tag | Champ | Obligatoire |
//! |-----|-------|-------------|
//! | 1 | identifiant du service | oui |
//! | 2 | version | oui |
//! | 3 | SHA-256 de l'archive ZIP | oui |
//! | 4 | `binary_name` du manifeste | si présent dans le manifeste |
//!
//! L'enveloppe est opaque pour Origin (stockage et checksum inchangés) : aucune
//! modification du serveur n'est requise pour distribuer un paquet signé.
//!
//! # Ordre de vérification (fail-closed)
//!
//! enveloppe stricte → Sceau (format, type, signataire épinglé, `verify_strict`,
//! TLV canonique) → SHA-256 de l'archive = valeur signée → id / version = demande.
//! Rien n'est décompressé ni écrit avant la fin de ces contrôles.

use miyusceau::{
    verifier, AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, LecteurTlv, Titre,
    TypeSceau, Verifie,
};
use sha2::{Digest, Sha256};

/// Entête des paquets signés.
pub const MAGIC_PAQUET_SIGNE: &[u8; 4] = b"MSPS";
/// Version du format d'enveloppe.
pub const VERSION_ENVELOPPE: u8 = 0x01;
/// Taille de l'entête d'enveloppe (magic + version + longueur).
const TAILLE_ENTETE: usize = 4 + 1 + 4;
/// Taille maximale d'un sceau dans l'enveloppe (entête Sceau + charge max + signature).
pub const TAILLE_MAX_SCEAU: usize = 42 + miyusceau::TAILLE_MAX_CHARGE + 64;
/// Longueur maximale d'un champ texte (id, version, binaire).
const TAILLE_MAX_TEXTE: usize = 255;

const TAG_SERVICE_ID: u8 = 1;
const TAG_VERSION: u8 = 2;
const TAG_SHA256_ARCHIVE: u8 = 3;
const TAG_NOM_BINAIRE: u8 = 4;

/// Titre signé « manifeste de paquet » (Sceau type 0x03).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestePaquet {
    /// Identifiant du service (doit égaler l'id du manifeste embarqué).
    pub service_id: String,
    /// Version du service (doit égaler la version du manifeste embarqué).
    pub version: String,
    /// SHA-256 de l'archive ZIP (hors enveloppe).
    pub sha256_archive: [u8; 32],
    /// `binary_name` déclaré par le manifeste embarqué (None si absent).
    pub nom_binaire: Option<String>,
}

fn texte_valide(tag: u8, v: &str) -> Result<(), ErreurSceau> {
    if v.is_empty() || v.len() > TAILLE_MAX_TEXTE || v.contains('\0') {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    Ok(())
}

impl Titre for ManifestePaquet {
    const TYPE: TypeSceau = TypeSceau::ManifestePaquet;

    fn encoder(&self, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        texte_valide(TAG_SERVICE_ID, &self.service_id)?;
        texte_valide(TAG_VERSION, &self.version)?;
        ecrivain
            .texte(TAG_SERVICE_ID, &self.service_id)?
            .texte(TAG_VERSION, &self.version)?
            .b32(TAG_SHA256_ARCHIVE, &self.sha256_archive)?;
        if let Some(nom) = &self.nom_binaire {
            texte_valide(TAG_NOM_BINAIRE, nom)?;
            ecrivain.texte(TAG_NOM_BINAIRE, nom)?;
        }
        Ok(())
    }

    fn decoder(lecteur: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let service_id = lecteur.exiger_texte(TAG_SERVICE_ID)?.to_string();
        texte_valide(TAG_SERVICE_ID, &service_id)?;
        let version = lecteur.exiger_texte(TAG_VERSION)?.to_string();
        texte_valide(TAG_VERSION, &version)?;
        let sha256_archive = lecteur.exiger_b32(TAG_SHA256_ARCHIVE)?;
        let nom_binaire = match lecteur.optionnel(TAG_NOM_BINAIRE)? {
            Some(octets) => {
                let nom = std::str::from_utf8(octets)
                    .map_err(|_| ErreurSceau::ValeurInvalide(TAG_NOM_BINAIRE))?
                    .to_string();
                texte_valide(TAG_NOM_BINAIRE, &nom)?;
                Some(nom)
            }
            None => None,
        };
        Ok(Self { service_id, version, sha256_archive, nom_binaire })
    }
}

/// Motif de refus d'un paquet. Chaque variante = refus d'installation.
#[derive(Debug, thiserror::Error)]
pub enum ErreurSignaturePaquet {
    /// Le paquet n'a pas d'enveloppe signée (archive ZIP nue ou autre format).
    #[error("paquet non signé : signature d'éditeur obligatoire")]
    NonSigne,
    /// Enveloppe tronquée, version inconnue ou longueur incohérente.
    #[error("enveloppe de paquet signé invalide : {0}")]
    EnveloppeInvalide(&'static str),
    /// Le sceau est refusé (format, signataire non épinglé, signature, TLV).
    #[error("sceau du paquet refusé : {0}")]
    Sceau(#[from] ErreurSceau),
    /// L'archive ne correspond pas à l'empreinte signée.
    #[error("archive modifiée : SHA-256 différent de la valeur signée")]
    EmpreinteDifferente,
    /// Identifiant signé différent de celui demandé.
    #[error("identifiant signé '{signe}' différent du service demandé '{attendu}'")]
    IdentifiantDifferent {
        /// Valeur signée.
        signe: String,
        /// Valeur attendue.
        attendu: String,
    },
    /// Version signée différente de celle demandée.
    #[error("version signée '{signe}' différente de la version demandée '{attendu}'")]
    VersionDifferente {
        /// Valeur signée.
        signe: String,
        /// Valeur attendue.
        attendu: String,
    },
    /// Aucun éditeur de confiance configuré.
    #[error("aucun éditeur de confiance épinglé : installation impossible")]
    AucuneAncre,
}

/// Paquet dont l'enveloppe, la signature, l'éditeur, l'empreinte, l'id et la
/// version ont été vérifiés. Seul [`verifier_paquet`] peut en construire un.
#[derive(Debug)]
pub struct PaquetVerifie {
    manifeste: Verifie<ManifestePaquet>,
    archive: Vec<u8>,
    sceau: Vec<u8>,
}

impl PaquetVerifie {
    /// Titre signé vérifié.
    pub fn manifeste(&self) -> &ManifestePaquet {
        self.manifeste.titre()
    }

    /// `kid` de l'éditeur signataire.
    pub fn editeur(&self) -> &miyusceau::Kid {
        self.manifeste.signataire()
    }

    /// Archive ZIP (octets couverts par l'empreinte signée).
    pub fn archive(&self) -> &[u8] {
        &self.archive
    }

    /// Octets du sceau (à conserver avec l'installation pour re-vérification).
    pub fn sceau(&self) -> &[u8] {
        &self.sceau
    }
}

/// SHA-256 brut.
fn sha256(octets: &[u8]) -> [u8; 32] {
    Sha256::digest(octets).into()
}

/// Vrai si les octets commencent par l'entête de paquet signé.
pub fn est_paquet_signe(octets: &[u8]) -> bool {
    octets.starts_with(MAGIC_PAQUET_SIGNE)
}

/// Sépare l'enveloppe en `(sceau, archive)` sans rien vérifier d'autre que la structure.
pub fn separer_enveloppe(octets: &[u8]) -> Result<(&[u8], &[u8]), ErreurSignaturePaquet> {
    if !est_paquet_signe(octets) {
        return Err(ErreurSignaturePaquet::NonSigne);
    }
    if octets.len() < TAILLE_ENTETE {
        return Err(ErreurSignaturePaquet::EnveloppeInvalide("entête tronquée"));
    }
    if octets[4] != VERSION_ENVELOPPE {
        return Err(ErreurSignaturePaquet::EnveloppeInvalide("version d'enveloppe inconnue"));
    }
    let mut longueur_octets = [0u8; 4];
    longueur_octets.copy_from_slice(&octets[5..9]);
    let longueur = u32::from_be_bytes(longueur_octets) as usize;
    if longueur == 0 || longueur > TAILLE_MAX_SCEAU {
        return Err(ErreurSignaturePaquet::EnveloppeInvalide("longueur de sceau hors bornes"));
    }
    let fin = TAILLE_ENTETE
        .checked_add(longueur)
        .ok_or(ErreurSignaturePaquet::EnveloppeInvalide("longueur de sceau hors bornes"))?;
    let sceau = octets
        .get(TAILLE_ENTETE..fin)
        .ok_or(ErreurSignaturePaquet::EnveloppeInvalide("sceau tronqué"))?;
    let archive = &octets[fin..];
    if archive.is_empty() {
        return Err(ErreurSignaturePaquet::EnveloppeInvalide("archive absente"));
    }
    Ok((sceau, archive))
}

/// Signe une archive ZIP et produit le paquet signé (outillage éditeur / dev).
pub fn signer_paquet(
    archive: &[u8],
    service_id: &str,
    version: &str,
    nom_binaire: Option<&str>,
    cle: &CleSignature,
) -> Result<Vec<u8>, ErreurSignaturePaquet> {
    let titre = ManifestePaquet {
        service_id: service_id.to_string(),
        version: version.to_string(),
        sha256_archive: sha256(archive),
        nom_binaire: nom_binaire.map(str::to_string),
    };
    let sceau = miyusceau::sceller(&titre, cle)?;
    let longueur = u32::try_from(sceau.len())
        .map_err(|_| ErreurSignaturePaquet::EnveloppeInvalide("sceau trop grand"))?;
    let mut sortie = Vec::with_capacity(TAILLE_ENTETE + sceau.len() + archive.len());
    sortie.extend_from_slice(MAGIC_PAQUET_SIGNE);
    sortie.push(VERSION_ENVELOPPE);
    sortie.extend_from_slice(&longueur.to_be_bytes());
    sortie.extend_from_slice(&sceau);
    sortie.extend_from_slice(archive);
    Ok(sortie)
}

/// Vérifie un paquet signé avant toute décompression ou écriture (fail-closed).
///
/// `version_attendue` : `None` uniquement si l'appelant n'a pas de version de
/// référence (la version signée reste alors la seule acceptée par l'installeur).
pub fn verifier_paquet(
    octets: &[u8],
    id_attendu: &str,
    version_attendue: Option<&str>,
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
) -> Result<PaquetVerifie, ErreurSignaturePaquet> {
    if ancres.is_empty() {
        return Err(ErreurSignaturePaquet::AucuneAncre);
    }
    let (sceau, archive) = separer_enveloppe(octets)?;
    let manifeste = verifier::<ManifestePaquet>(sceau, ancres, maintenant_secs)?;
    // Comparaison d'empreintes publiques : pas de secret, comparaison directe.
    if manifeste.titre().sha256_archive != sha256(archive) {
        return Err(ErreurSignaturePaquet::EmpreinteDifferente);
    }
    if manifeste.titre().service_id != id_attendu {
        return Err(ErreurSignaturePaquet::IdentifiantDifferent {
            signe: manifeste.titre().service_id.clone(),
            attendu: id_attendu.to_string(),
        });
    }
    if let Some(v) = version_attendue {
        if manifeste.titre().version != v {
            return Err(ErreurSignaturePaquet::VersionDifferente {
                signe: manifeste.titre().version.clone(),
                attendu: v.to_string(),
            });
        }
    }
    Ok(PaquetVerifie { manifeste, archive: archive.to_vec(), sceau: sceau.to_vec() })
}

/// Re-vérifie le sceau conservé d'un service installé (avant de lui remettre
/// des secrets) : signature, éditeur épinglé et identifiant.
pub fn verifier_sceau_installe(
    sceau: &[u8],
    id_attendu: &str,
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
) -> Result<Verifie<ManifestePaquet>, ErreurSignaturePaquet> {
    if ancres.is_empty() {
        return Err(ErreurSignaturePaquet::AucuneAncre);
    }
    let manifeste = verifier::<ManifestePaquet>(sceau, ancres, maintenant_secs)?;
    if manifeste.titre().service_id != id_attendu {
        return Err(ErreurSignaturePaquet::IdentifiantDifferent {
            signe: manifeste.titre().service_id.clone(),
            attendu: id_attendu.to_string(),
        });
    }
    Ok(manifeste)
}

/// Encode des octets en hexadécimal minuscule.
pub fn hex_encoder(octets: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(octets.len() * 2);
    for b in octets {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Décode de l'hexadécimal (refuse longueur impaire et caractères non hex).
pub fn hex_decoder(texte: &str) -> Option<Vec<u8>> {
    let t = texte.trim();
    if !t.len().is_multiple_of(2) {
        return None;
    }
    let octets = t.as_bytes();
    let mut sortie = Vec::with_capacity(t.len() / 2);
    for paire in octets.chunks(2) {
        let haut = (paire[0] as char).to_digit(16)?;
        let bas = (paire[1] as char).to_digit(16)?;
        sortie.push(((haut << 4) | bas) as u8);
    }
    Some(sortie)
}

/// Décode une clé publique Ed25519 hexadécimale (64 caractères).
pub fn cle_publique_depuis_hex(texte: &str) -> Option<[u8; 32]> {
    hex_decoder(texte)?.try_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyusceau::NiveauConfiance;

    fn ancres_pour(cle: &CleSignature) -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        a
    }

    const ARCHIVE: &[u8] = b"PK\x03\x04 archive de test";

    #[test]
    fn paquet_signe_valide_accepte() {
        let cle = CleSignature::generer();
        let paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", Some("svc.exe"), &cle).unwrap();
        let v = verifier_paquet(&paquet, "svc", Some("1.0.0"), &ancres_pour(&cle), 0).unwrap();
        assert_eq!(v.archive(), ARCHIVE);
        assert_eq!(v.manifeste().nom_binaire.as_deref(), Some("svc.exe"));
        assert_eq!(v.editeur(), &cle.kid());
        // Le sceau conservé se re-vérifie seul.
        assert!(verifier_sceau_installe(v.sceau(), "svc", &ancres_pour(&cle), 0).is_ok());
        assert!(verifier_sceau_installe(v.sceau(), "autre", &ancres_pour(&cle), 0).is_err());
    }

    #[test]
    fn archive_nue_refusee() {
        let cle = CleSignature::generer();
        assert!(matches!(
            verifier_paquet(ARCHIVE, "svc", None, &ancres_pour(&cle), 0),
            Err(ErreurSignaturePaquet::NonSigne)
        ));
    }

    #[test]
    fn archive_alteree_refusee() {
        let cle = CleSignature::generer();
        let mut paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", None, &cle).unwrap();
        let dernier = paquet.len() - 1;
        paquet[dernier] ^= 0x01;
        assert!(matches!(
            verifier_paquet(&paquet, "svc", Some("1.0.0"), &ancres_pour(&cle), 0),
            Err(ErreurSignaturePaquet::EmpreinteDifferente)
        ));
    }

    #[test]
    fn sceau_altere_refuse() {
        let cle = CleSignature::generer();
        let mut paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", None, &cle).unwrap();
        // Octet de la charge TLV (après entête d'enveloppe + entête Sceau).
        paquet[TAILLE_ENTETE + 42 + 6] ^= 0x01;
        assert!(matches!(
            verifier_paquet(&paquet, "svc", Some("1.0.0"), &ancres_pour(&cle), 0),
            Err(ErreurSignaturePaquet::Sceau(_))
        ));
    }

    #[test]
    fn mauvais_editeur_refuse() {
        let cle = CleSignature::generer();
        let autre = CleSignature::generer();
        let paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", None, &autre).unwrap();
        assert!(matches!(
            verifier_paquet(&paquet, "svc", Some("1.0.0"), &ancres_pour(&cle), 0),
            Err(ErreurSignaturePaquet::Sceau(ErreurSceau::SignataireInconnu))
        ));
    }

    #[test]
    fn aucune_ancre_refuse() {
        let cle = CleSignature::generer();
        let paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", None, &cle).unwrap();
        assert!(matches!(
            verifier_paquet(&paquet, "svc", None, &AncresDeConfiance::new(), 0),
            Err(ErreurSignaturePaquet::AucuneAncre)
        ));
    }

    #[test]
    fn id_ou_version_differents_refuses() {
        let cle = CleSignature::generer();
        let paquet = signer_paquet(ARCHIVE, "svc", "1.0.0", None, &cle).unwrap();
        let a = ancres_pour(&cle);
        assert!(matches!(
            verifier_paquet(&paquet, "miyucloud", None, &a, 0),
            Err(ErreurSignaturePaquet::IdentifiantDifferent { .. })
        ));
        assert!(matches!(
            verifier_paquet(&paquet, "svc", Some("2.0.0"), &a, 0),
            Err(ErreurSignaturePaquet::VersionDifferente { .. })
        ));
    }

    #[test]
    fn enveloppe_malformee_refusee() {
        let cle = CleSignature::generer();
        let a = ancres_pour(&cle);
        for octets in [
            b"MSPS".to_vec(),
            b"MSPS\x02\x00\x00\x00\x01x".to_vec(),
            b"MSPS\x01\xff\xff\xff\xffx".to_vec(),
            b"MSPS\x01\x00\x00\x00\x00x".to_vec(),
        ] {
            assert!(matches!(
                verifier_paquet(&octets, "svc", None, &a, 0),
                Err(ErreurSignaturePaquet::EnveloppeInvalide(_))
            ));
        }
    }

    #[test]
    fn hex_aller_retour() {
        let cle = CleSignature::generer();
        let h = hex_encoder(&cle.cle_publique());
        assert_eq!(cle_publique_depuis_hex(&h), Some(cle.cle_publique()));
        assert!(cle_publique_depuis_hex("zz").is_none());
        assert!(cle_publique_depuis_hex("abc").is_none());
    }
}
