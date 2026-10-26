//! @id     miyuportail_jeton
//! @do     emettre_verifier_jeton_reponse
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — jeton de réponse à usage unique, signé par l'entrée, vérifié et consommé par la sortie (A-022 §3) sans état partagé entre les deux processus.

//! # Jeton de réponse (A-007, A-022 §3)
//!
//! Le moteur d'un service H2 ne publie qu'en **réponse** à une déclaration
//! admise. Quand le Portail est scindé en deux processus sans état commun,
//! la preuve « cette réponse répond à une déclaration admise de la session
//! en cours » ne peut plus être un objet en mémoire : c'est un **jeton signé**.
//!
//! - Les Cores tirent, à l'ouverture d'une session, une paire Ed25519
//!   **éphémère** : la graine ([`CleJetons`]) va à l'**entrée** seule, la clé
//!   publique ([`SessionSortie::ancre_jetons`](crate::sortie::SessionSortie))
//!   à la **sortie** seule, avec un identifiant de session aléatoire ;
//! - l'entrée signe un jeton pour chaque déclaration admise ;
//! - la sortie vérifie la signature, le service et la session, puis le
//!   **consomme** : un second usage est refusé
//!   ([`crate::ErreurPortail::JetonDejaUtilise`]).
//!
//! Une entrée compromise peut signer des jetons, mais ne peut rien
//! **publier** : seule la file des réponses (écrite par le moteur) mène à la
//! sortie, et l'entrée n'y a aucun droit.
//!
//! Format (big-endian) :
//!
//! ```text
//! "MJR1" | ls u8 | service | session (32) | empreinte (32) | émis le u64 | signature (64)
//! signature Ed25519 de : "miyukini/portail/jeton/v1\0" | tout ce qui précède la signature
//! ```

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand_core::{OsRng, RngCore};

use crate::erreurs::{ErreurPortail, RefusDecodage};
use crate::registre::Empreinte;
use crate::traite::{Curseur, IdService};

/// Magic d'un jeton.
pub const MAGIC_JETON: &[u8; 4] = b"MJR1";
/// Domaine de signature des jetons (séparé de tout autre usage de clé).
pub const DOMAINE_JETON: &[u8] = b"miyukini/portail/jeton/v1\0";
/// Taille maximale d'un jeton encodé.
pub const TAILLE_MAX_JETON: usize = 4 + 1 + crate::traite::MAX_ID_SERVICE + 32 + 32 + 8 + 64;

/// Identifiant aléatoire d'une session (commun aux deux moitiés **par
/// configuration** des Cores, jamais par un canal entre elles).
pub type IdSession = [u8; 32];

/// Tire un identifiant de session.
pub fn nouvel_id_session() -> IdSession {
    let mut id = [0u8; 32];
    OsRng.fill_bytes(&mut id);
    id
}

/// Clé de signature des jetons d'une session (détenue par l'**entrée** seule).
pub struct CleJetons(SigningKey);

impl std::fmt::Debug for CleJetons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CleJetons").field("ancre", &crate::hex(&self.ancre())).finish_non_exhaustive()
    }
}

impl CleJetons {
    /// Paire éphémère tirée par les Cores à l'ouverture d'une session.
    pub fn generer() -> Self {
        Self(SigningKey::generate(&mut OsRng))
    }

    /// Clé depuis sa graine (descripteur de voie de l'entrée).
    pub fn depuis_graine(graine: &[u8; 32]) -> Self {
        Self(SigningKey::from_bytes(graine))
    }

    /// Graine (écrite par les Cores dans le descripteur de l'entrée seulement).
    pub fn graine(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    /// Clé publique (remise à la sortie).
    pub fn ancre(&self) -> [u8; 32] {
        self.0.verifying_key().to_bytes()
    }

    /// Signe le jeton d'une déclaration admise.
    pub fn emettre(
        &self,
        service: &IdService,
        session: &IdSession,
        empreinte: &Empreinte,
        emis_le: u64,
    ) -> JetonReponse {
        let mut octets = corps(service, session, empreinte, emis_le);
        let mut message = Vec::with_capacity(DOMAINE_JETON.len() + octets.len());
        message.extend_from_slice(DOMAINE_JETON);
        message.extend_from_slice(&octets);
        octets.extend_from_slice(&self.0.sign(&message).to_bytes());
        JetonReponse { service: service.clone(), session: *session, empreinte: *empreinte, emis_le, octets }
    }
}

fn corps(service: &IdService, session: &IdSession, empreinte: &Empreinte, emis_le: u64) -> Vec<u8> {
    let s = service.as_str().as_bytes();
    let mut o = Vec::with_capacity(TAILLE_MAX_JETON);
    o.extend_from_slice(MAGIC_JETON);
    o.push(s.len() as u8);
    o.extend_from_slice(s);
    o.extend_from_slice(session);
    o.extend_from_slice(empreinte);
    o.extend_from_slice(&emis_le.to_be_bytes());
    o
}

/// Jeton de réponse : décodé, **non vérifié** (la sortie seule le vérifie et
/// le consomme). Ni `Clone` ni `Copy` : en processus unique, il se consomme
/// par valeur ; entre processus, l'usage unique est tenu par la sortie.
#[derive(Debug, PartialEq, Eq)]
pub struct JetonReponse {
    service: IdService,
    session: IdSession,
    empreinte: Empreinte,
    emis_le: u64,
    octets: Vec<u8>,
}

impl JetonReponse {
    /// Service de la voie.
    pub fn service(&self) -> &IdService {
        &self.service
    }

    /// Session d'émission.
    pub fn session(&self) -> &IdSession {
        &self.session
    }

    /// Empreinte (SHA-256) de la trame à laquelle on répond.
    pub fn empreinte(&self) -> &Empreinte {
        &self.empreinte
    }

    /// Date d'émission (secondes Unix, déclarative).
    pub fn emis_le(&self) -> u64 {
        self.emis_le
    }

    /// Encodage (enveloppe de la file vers le moteur, puis de la réponse).
    pub fn octets(&self) -> &[u8] {
        &self.octets
    }

    /// Décodage strict, **sans** vérification de signature.
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        if octets.len() > TAILLE_MAX_JETON {
            return Err(RefusDecodage::Taille);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_JETON {
            return Err(RefusDecodage::Magic);
        }
        let ls = usize::from(c.u8()?);
        let so = c.prendre(ls)?;
        let service =
            std::str::from_utf8(so).ok().and_then(|s| IdService::new(s).ok()).ok_or(RefusDecodage::Service)?;
        let session = c.b32()?;
        let empreinte = c.b32()?;
        let emis_le = c.u64()?;
        c.prendre(64)?;
        c.fin()?;
        Ok(Self { service, session, empreinte, emis_le, octets: octets.to_vec() })
    }

    /// Vérifie la signature contre l'ancre de la session (côté sortie).
    pub(crate) fn verifier(&self, ancre: &[u8; 32]) -> Result<(), ErreurPortail> {
        let vk = VerifyingKey::from_bytes(ancre).map_err(|_| ErreurPortail::JetonInvalide)?;
        if vk.is_weak() || self.octets.len() < 64 {
            return Err(ErreurPortail::JetonInvalide);
        }
        let (corps, sig) = self.octets.split_at(self.octets.len() - 64);
        let sig: [u8; 64] = sig.try_into().map_err(|_| ErreurPortail::JetonInvalide)?;
        let mut message = Vec::with_capacity(DOMAINE_JETON.len() + corps.len());
        message.extend_from_slice(DOMAINE_JETON);
        message.extend_from_slice(corps);
        vk.verify_strict(&message, &Signature::from_bytes(&sig)).map_err(|_| ErreurPortail::JetonInvalide)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour_et_signature() {
        let cle = CleJetons::generer();
        let s = IdService::new("docs").unwrap();
        let session = nouvel_id_session();
        let j = cle.emettre(&s, &session, &[7; 32], 1_790_000_000);
        let d = JetonReponse::decoder(j.octets()).unwrap();
        assert_eq!(d, j);
        d.verifier(&cle.ancre()).unwrap();
        assert_eq!(d.verifier(&CleJetons::generer().ancre()), Err(ErreurPortail::JetonInvalide));
    }

    #[test]
    fn jeton_altere_refuse() {
        let cle = CleJetons::generer();
        let s = IdService::new("docs").unwrap();
        let j = cle.emettre(&s, &[1; 32], &[7; 32], 5);
        for i in 0..j.octets().len() {
            let mut o = j.octets().to_vec();
            o[i] ^= 0x01;
            if let Ok(d) = JetonReponse::decoder(&o) {
                assert_eq!(d.verifier(&cle.ancre()), Err(ErreurPortail::JetonInvalide), "octet {i}");
            }
        }
        let mut long = j.octets().to_vec();
        long.push(0);
        assert!(JetonReponse::decoder(&long).is_err());
    }
}
