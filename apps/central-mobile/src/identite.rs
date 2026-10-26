//! @id     central_mobile_identite
//! @do     regenerer_identite_appareil
//! @role   security
//! @layer  app
//! @human  central-mobile — régénération de la clé d'appareil : nouvelle clé stockée, liée par preuve de l'ancienne ou par un appareil de confiance, ancienne révoquée.

//! Régénération de l'identité de l'appareil (I6).
//!
//! L'identité de l'appareil est sa clé Ed25519 d'appairage (identité E2E,
//! `platform::e2e_identity_path`, fichier `0600` dans le stockage privé de
//! l'application) ; c'est aussi la clé d'appareil de ses passeports et celle
//! qu'il présente à MiyuCloud.
//!
//! Déroulé (rien n'est perdu tant que l'utilisateur n'a pas confirmé) :
//!
//! 1. [`preparer`] : génère la **nouvelle** clé et l'écrit à part
//!    (`…/e2e_identity.key.nouvelle`, `0600`, création exclusive, synchronisée).
//!    Une préparation interrompue est reprise (même nouvelle clé).
//! 2. Liaison de la nouvelle clé :
//!    - [`ModeLiaison::PreuveAncienne`] — l'ancienne clé est lisible : demande
//!      `MLA2` de **rotation** signée par l'ancienne (autorisation) et par la
//!      nouvelle (possession), envoyée à MiyuCloud dans une requête **signée
//!      par l'ancienne clé** (`POST /api/auth/miyu/appareils/rotation`) ;
//!      MiyuCloud révoque l'ancienne dans la même transaction ;
//!    - [`ModeLiaison::NouvelleLiaison`] — ancienne clé absente ou illisible :
//!      demande `MLA2` simple (possession seule), à coller depuis un **appareil
//!      déjà de confiance** (session MiyuCloud du navigateur, « Lier un nouvel
//!      appareil ») ; l'ancienne clé y est retirée.
//! 3. [`confirmer`] : la nouvelle clé remplace l'ancienne (renommage atomique)
//!    ; l'appairage au COG est à refaire (Central Desktop : nouvelle liaison
//!    approuvée par l'utilisateur, ancienne clé à révoquer).
//!    [`abandonner`] : la nouvelle clé est effacée, rien ne change.
//!
//! Format `MLA2` : voir `apps/miyucloud/src/infrastructure/services/miyu_liaison_appareil.rs`
//! (message de liaison d'appareil v2 de `miyauth`, origine = destination =
//! `cog_id` épinglé par MiyuCloud). Un vecteur de référence est partagé.

use std::path::{Path, PathBuf};

use ed25519_dalek::{Signer, SigningKey};
use miyauth::CogId;
use miyukini_cog_bridge::E2eIdentity;
use miyusceau::CleSignature;

/// En-tête du format de transport des demandes de liaison.
pub const MAGIC_LIAISON: [u8; 4] = *b"MLA2";
/// Chemin MiyuCloud de la rotation (requête signée par l'ancienne clé).
pub const CHEMIN_ROTATION: &str = "/api/auth/miyu/appareils/rotation";
/// Chemin MiyuCloud public donnant le `cog_id` à signer.
pub const CHEMIN_CONTEXTE: &str = "/api/auth/miyu/liaison-appareil";
/// Usage des requêtes signées par clé d'appareil vers MiyuCloud.
pub const USAGE_APPAREIL_MIYUCLOUD: &str = "miyucloud/appareil";
/// Validité demandée pour une demande de liaison (le serveur refuse > 5 min).
pub const VALIDITE_DEMANDE_SECS: u64 = 4 * 60;
/// Taille maximale d'une réponse MiyuCloud lue (C-13).
const MAX_REPONSE: usize = 8192;

/// Emplacements de l'identité courante et de la nouvelle (en attente).
#[derive(Debug, Clone)]
pub struct StockageIdentite {
    courante: PathBuf,
    nouvelle: PathBuf,
}

impl StockageIdentite {
    /// Emplacement de la plateforme (stockage privé de l'application).
    pub fn plateforme() -> Self {
        Self::a(&crate::platform::e2e_identity_path())
    }

    /// Identité courante à `courante`, nouvelle à côté (`.nouvelle`).
    pub fn a(courante: &Path) -> Self {
        let mut nom = courante.as_os_str().to_owned();
        nom.push(".nouvelle");
        Self {
            courante: courante.to_path_buf(),
            nouvelle: PathBuf::from(nom),
        }
    }

    /// Vrai si une régénération est en attente de confirmation.
    pub fn en_attente(&self) -> bool {
        std::fs::symlink_metadata(&self.nouvelle).is_ok()
    }
}

/// Comment la nouvelle clé sera liée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeLiaison {
    /// L'ancienne clé signe la rotation (autorisation) : liaison directe.
    PreuveAncienne,
    /// Ancienne clé indisponible : liaison depuis un appareil de confiance.
    NouvelleLiaison,
}

/// Régénération préparée : ancienne clé (si lisible) et nouvelle clé.
pub struct Regeneration {
    ancienne: Option<E2eIdentity>,
    nouvelle: E2eIdentity,
}

impl std::fmt::Debug for Regeneration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Regeneration")
            .field("mode", &self.mode())
            .field("nouvelle", &hex::encode(self.nouvelle.public_key_bytes()))
            .finish_non_exhaustive()
    }
}

/// Requête HTTP prête à envoyer (corps et en-têtes signés).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequeteSignee {
    /// Chemin (sans hôte).
    pub chemin: String,
    /// Corps JSON.
    pub corps: Vec<u8>,
    /// En-têtes `X-Miyu-*` de la signature.
    pub entetes: Vec<(&'static str, String)>,
}

fn erreur(contexte: &str, e: &std::io::Error) -> String {
    format!("{contexte} : {}", e.kind())
}

/// Écrit la graine de `identite` dans `chemin` (création exclusive, `0600`,
/// synchronisée avant de rendre la main).
fn ecrire_exclusif(chemin: &Path, identite: &E2eIdentity) -> Result<(), String> {
    use std::io::Write;
    if let Some(parent) = chemin.parent() {
        std::fs::create_dir_all(parent).map_err(|e| erreur("identité", &e))?;
    }
    let mut contenu = zeroize::Zeroizing::new(hex::encode(identite.secret_bytes().as_ref()));
    contenu.push('\n');
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut f = options
        .open(chemin)
        .map_err(|e| erreur("nouvelle identité", &e))?;
    f.write_all(contenu.as_bytes())
        .and_then(|()| f.sync_all())
        .map_err(|e| erreur("nouvelle identité", &e))
}

fn synchroniser_dossier(chemin: &Path) {
    if let Some(parent) = chemin.parent() {
        if let Ok(d) = std::fs::File::open(parent) {
            let _ = d.sync_all();
        }
    }
}

/// Prépare la régénération : nouvelle clé générée et stockée à part (ou reprise
/// si une préparation est déjà en attente) ; ancienne clé lue si possible.
pub fn preparer(stockage: &StockageIdentite) -> Result<Regeneration, String> {
    let nouvelle = if stockage.en_attente() {
        crate::platform::load_e2e_identity_at(&stockage.nouvelle)
            .map_err(|e| format!("régénération en attente illisible ({e}) : annulez-la"))?
    } else {
        let n = E2eIdentity::generate();
        ecrire_exclusif(&stockage.nouvelle, &n)?;
        synchroniser_dossier(&stockage.nouvelle);
        n
    };
    // Ancienne illisible (absente, corrompue, droits trop larges) : la liaison
    // se fera depuis un appareil de confiance, jamais avec une clé douteuse.
    let ancienne = crate::platform::load_e2e_identity_at(&stockage.courante).ok();
    Ok(Regeneration { ancienne, nouvelle })
}

/// La nouvelle clé devient l'identité de l'appareil (renommage atomique).
pub fn confirmer(stockage: &StockageIdentite) -> Result<(), String> {
    // Vérifie qu'elle est lisible avant de remplacer quoi que ce soit.
    crate::platform::load_e2e_identity_at(&stockage.nouvelle)?;
    std::fs::rename(&stockage.nouvelle, &stockage.courante)
        .map_err(|e| erreur("remplacement de l'identité", &e))?;
    synchroniser_dossier(&stockage.courante);
    Ok(())
}

/// Abandonne la régénération : la nouvelle clé est effacée.
pub fn abandonner(stockage: &StockageIdentite) -> Result<(), String> {
    match std::fs::remove_file(&stockage.nouvelle) {
        Ok(()) => {
            synchroniser_dossier(&stockage.nouvelle);
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(erreur("abandon", &e)),
    }
}

fn signer(identite: &E2eIdentity, message: &[u8]) -> [u8; 64] {
    SigningKey::from_bytes(&identite.secret_bytes())
        .sign(message)
        .to_bytes()
}

/// Encode une demande `MLA2` : nouvelle clé (possession) et, si fournie,
/// ancienne clé (autorisation), pour `cog` jusqu'à `pas_apres`.
fn encoder_demande(
    cog: &CogId,
    nouvelle: &E2eIdentity,
    ancienne: Option<&E2eIdentity>,
    pas_apres: u64,
) -> String {
    let cle = nouvelle.public_key_bytes();
    let message = miyauth::message_liaison(cog, cog, &cle, pas_apres);
    let mut o = Vec::with_capacity(204);
    o.extend_from_slice(&MAGIC_LIAISON);
    o.extend_from_slice(&cle);
    o.extend_from_slice(&pas_apres.to_be_bytes());
    o.extend_from_slice(&signer(nouvelle, &message));
    if let Some(a) = ancienne {
        o.extend_from_slice(&a.public_key_bytes());
        o.extend_from_slice(&signer(a, &message));
    }
    miyauth::http::b64_encoder(&o)
}

impl Regeneration {
    /// Mode de liaison possible.
    pub fn mode(&self) -> ModeLiaison {
        if self.ancienne.is_some() {
            ModeLiaison::PreuveAncienne
        } else {
            ModeLiaison::NouvelleLiaison
        }
    }

    /// Clé publique de la nouvelle identité.
    pub fn cle_nouvelle(&self) -> [u8; 32] {
        self.nouvelle.public_key_bytes()
    }

    /// Demande simple (possession seule), à coller depuis un appareil de
    /// confiance (MiyuCloud : « Lier un nouvel appareil »).
    pub fn demande_nouvelle_liaison(&self, cog: &CogId, maintenant: u64) -> String {
        encoder_demande(
            cog,
            &self.nouvelle,
            None,
            maintenant + VALIDITE_DEMANDE_SECS,
        )
    }

    /// Demande de rotation signée par l'ancienne clé ; `None` sans ancienne.
    pub fn demande_rotation(&self, cog: &CogId, maintenant: u64) -> Option<String> {
        let ancienne = self.ancienne.as_ref()?;
        Some(encoder_demande(
            cog,
            &self.nouvelle,
            Some(ancienne),
            maintenant + VALIDITE_DEMANDE_SECS,
        ))
    }

    /// Requête de rotation MiyuCloud, **signée par l'ancienne clé** (en-têtes
    /// `X-Miyu-Signataire`, `-Horodatage`, `-Nonce`, `-Signature`).
    pub fn requete_rotation(&self, cog: &CogId, maintenant: u64) -> Result<RequeteSignee, String> {
        let ancienne = self.ancienne.as_ref().ok_or(
            "ancienne clé indisponible : liez la nouvelle depuis un appareil de confiance",
        )?;
        let demande = self
            .demande_rotation(cog, maintenant)
            .ok_or("ancienne clé indisponible")?;
        let corps = serde_json::to_vec(&serde_json::json!({ "demande": demande }))
            .map_err(|_| "sérialisation de la demande".to_string())?;
        let cle = CleSignature::depuis_graine(&ancienne.secret_bytes());
        let signees = miyauth::http::signer_requete(
            &cle,
            USAGE_APPAREIL_MIYUCLOUD,
            "POST",
            CHEMIN_ROTATION,
            &corps,
            maintenant,
        );
        Ok(RequeteSignee {
            chemin: CHEMIN_ROTATION.to_string(),
            corps,
            entetes: signees.en_entetes().to_vec(),
        })
    }
}

/// Base MiyuCloud valide : `https://hôte[:port]` sans chemin (HTTP refusé).
pub fn base_miyucloud(url: &str) -> Result<String, String> {
    let u = url.trim().trim_end_matches('/');
    let Some(reste) = u.strip_prefix("https://") else {
        return Err("Adresse MiyuCloud : https:// requis".into());
    };
    if reste.is_empty() || reste.contains(['/', '?', '#', '@', ' ']) {
        return Err("Adresse MiyuCloud invalide (https://hôte[:port])".into());
    }
    Ok(u.to_string())
}

async fn lire_borne(r: reqwest::Response) -> Result<(u16, Vec<u8>), String> {
    let statut = r.status().as_u16();
    if r.content_length().is_some_and(|n| n > MAX_REPONSE as u64) {
        return Err("réponse MiyuCloud trop grande".into());
    }
    let o = r
        .bytes()
        .await
        .map_err(|_| "réponse MiyuCloud illisible".to_string())?;
    if o.len() > MAX_REPONSE {
        return Err("réponse MiyuCloud trop grande".into());
    }
    Ok((statut, o.to_vec()))
}

fn client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| "client HTTP indisponible".to_string())
}

/// `cog_id` que MiyuCloud attend dans les demandes de liaison.
pub async fn contexte_miyucloud(base: &str) -> Result<CogId, String> {
    let base = base_miyucloud(base)?;
    let r = client()?
        .get(format!("{base}{CHEMIN_CONTEXTE}"))
        .send()
        .await
        .map_err(|_| "MiyuCloud injoignable".to_string())?;
    let (statut, corps) = lire_borne(r).await?;
    if statut != 200 {
        return Err(format!(
            "MiyuCloud refuse la liaison d'appareil (HTTP {statut})"
        ));
    }
    let v: serde_json::Value =
        serde_json::from_slice(&corps).map_err(|_| "réponse MiyuCloud invalide".to_string())?;
    v.get("cog_id")
        .and_then(serde_json::Value::as_str)
        .and_then(CogId::new)
        .ok_or_else(|| "cog_id MiyuCloud invalide".to_string())
}

/// Envoie la rotation (signée par l'ancienne clé) ; succès seulement sur 201.
pub async fn envoyer_rotation(base: &str, requete: &RequeteSignee) -> Result<(), String> {
    let base = base_miyucloud(base)?;
    let mut req = client()?
        .post(format!("{base}{}", requete.chemin))
        .header("content-type", "application/json")
        .body(requete.corps.clone());
    for (nom, valeur) in &requete.entetes {
        req = req.header(*nom, valeur);
    }
    let r = req
        .send()
        .await
        .map_err(|_| "MiyuCloud injoignable".to_string())?;
    let (statut, _) = lire_borne(r).await?;
    match statut {
        201 => Ok(()),
        401 => Err("Ancienne clé inconnue ou révoquée par MiyuCloud : liez la nouvelle depuis un appareil de confiance.".into()),
        409 => Err("Nouvelle clé déjà liée ou demande déjà utilisée.".into()),
        s => Err(format!("Rotation refusée par MiyuCloud (HTTP {s})")),
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;

    fn dossier() -> PathBuf {
        std::env::temp_dir().join(format!("central-mobile-identite-{}", uuid::Uuid::new_v4()))
    }

    fn cog() -> CogId {
        let cle = SigningKey::from_bytes(&[3; 32]).verifying_key().to_bytes();
        CogId::new(&miyusceau::cog_id_depuis_cle(&cle)).unwrap()
    }

    /// Vecteur de référence partagé avec MiyuCloud (vérifié des deux côtés).
    const VECTEUR: &str =
        include_str!("../../miyucloud/src/infrastructure/services/miyu_liaison_appareil.vecteur");

    #[test]
    fn encoding_matches_miyucloud_test_vector() {
        let nouvelle = E2eIdentity::from_secret_bytes(&[1; 32]);
        let ancienne = E2eIdentity::from_secret_bytes(&[2; 32]);
        let texte = encoder_demande(&cog(), &nouvelle, Some(&ancienne), 1_800_000_000);
        assert_eq!(texte, VECTEUR.trim());
        // Demande simple = préfixe de 108 octets de la rotation.
        let simple = encoder_demande(&cog(), &nouvelle, None, 1_800_000_000);
        let o = miyauth::http::b64_decoder(&simple).unwrap();
        assert_eq!(o.len(), 108);
        assert_eq!(&miyauth::http::b64_decoder(&texte).unwrap()[..108], &o[..]);
    }

    #[test]
    fn regeneration_with_old_key_signs_rotation_then_confirms() {
        let d = dossier();
        let s = StockageIdentite::a(&d.join("e2e_identity.key"));
        let ancienne = crate::platform::load_or_create_e2e_identity_at(&d.join("e2e_identity.key"))
            .unwrap()
            .public_key_bytes();
        let r = preparer(&s).unwrap();
        assert_eq!(r.mode(), ModeLiaison::PreuveAncienne);
        assert_ne!(r.cle_nouvelle(), ancienne);
        assert!(s.en_attente());
        // Préparation reprise : même nouvelle clé.
        assert_eq!(preparer(&s).unwrap().cle_nouvelle(), r.cle_nouvelle());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&s.nouvelle).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }

        // Requête de rotation : signée par l'ancienne clé, vérifiable comme MiyuCloud le fait.
        let t = 1_700_000_000;
        let req = r.requete_rotation(&cog(), t).unwrap();
        assert_eq!(req.chemin, CHEMIN_ROTATION);
        let valeur = |nom: &str| {
            req.entetes
                .iter()
                .find(|(n, _)| *n == nom)
                .map(|(_, v)| v.as_str())
        };
        let entetes = miyauth::http::EntetesSignees::analyser(
            valeur(miyauth::http::ENTETE_SIGNATAIRE),
            valeur(miyauth::http::ENTETE_HORODATAGE),
            valeur(miyauth::http::ENTETE_NONCE),
            valeur(miyauth::http::ENTETE_SIGNATURE),
        )
        .unwrap();
        assert_eq!(entetes.signataire, ancienne);
        let nonces = miyauth::http::RegistreNonces::default();
        miyauth::http::verifier_requete_signee(
            &entetes,
            USAGE_APPAREIL_MIYUCLOUD,
            "POST",
            CHEMIN_ROTATION,
            &req.corps,
            &nonces,
            t,
        )
        .unwrap();
        // Corps : demande de rotation (204 octets) autorisée par l'ancienne clé.
        let v: serde_json::Value = serde_json::from_slice(&req.corps).unwrap();
        let o = miyauth::http::b64_decoder(v["demande"].as_str().unwrap()).unwrap();
        assert_eq!(o.len(), 204);
        assert_eq!(&o[..4], b"MLA2");
        assert_eq!(&o[4..36], &r.cle_nouvelle());
        assert_eq!(&o[108..140], &ancienne);

        // Confirmation : la nouvelle clé devient l'identité, plus rien en attente.
        confirmer(&s).unwrap();
        assert!(!s.en_attente());
        let courante = crate::platform::load_e2e_identity_at(&d.join("e2e_identity.key")).unwrap();
        assert_eq!(courante.public_key_bytes(), r.cle_nouvelle());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn regeneration_without_old_key_requires_trusted_device() {
        let d = dossier();
        let s = StockageIdentite::a(&d.join("e2e_identity.key"));
        // Ancienne corrompue : jamais utilisée.
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("e2e_identity.key"), "corrompu").unwrap();
        let r = preparer(&s).unwrap();
        assert_eq!(r.mode(), ModeLiaison::NouvelleLiaison);
        assert!(r.demande_rotation(&cog(), 1).is_none());
        assert!(r.requete_rotation(&cog(), 1).is_err());
        let texte = r.demande_nouvelle_liaison(&cog(), 1_000);
        let o = miyauth::http::b64_decoder(&texte).unwrap();
        assert_eq!(o.len(), 108);
        assert_eq!(&o[4..36], &r.cle_nouvelle());
        assert_eq!(
            u64::from_be_bytes(o[36..44].try_into().unwrap()),
            1_000 + VALIDITE_DEMANDE_SECS
        );
        // Abandon : rien n'a changé, la nouvelle clé est effacée.
        abandonner(&s).unwrap();
        assert!(!s.en_attente());
        assert_eq!(
            std::fs::read_to_string(d.join("e2e_identity.key")).unwrap(),
            "corrompu"
        );
        abandonner(&s).unwrap();
        // Confirmer sans régénération en attente : refus, identité intacte.
        assert!(confirmer(&s).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn miyucloud_base_url_is_https_only() {
        assert_eq!(
            base_miyucloud(" https://cloud.example:8443/ ").unwrap(),
            "https://cloud.example:8443"
        );
        for mauvaise in [
            "http://cloud",
            "https://",
            "https://a/b",
            "https://u@h",
            "cloud.example",
            "ftp://x",
        ] {
            assert!(base_miyucloud(mauvaise).is_err(), "{mauvaise}");
        }
    }
}
