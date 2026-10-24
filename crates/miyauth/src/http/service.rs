//! @id     miyauth_http_service
//! @do     signer_verifier_requetes_services
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — clé de service (graine 0600) et garde des requêtes signées entre services locaux.

//! Services locaux : clé de service et garde des requêtes signées (sortie du lot 2).
//!
//! Brique commune aux services appelés par d'autres composants du COG (maia,
//! maia-llm, hub TEAM, MiyuSTT / MiyuTTS…) pour remplacer les jetons Bearer
//! partagés par des **requêtes signées machine à machine** ([`super`] §2) :
//!
//! - chaque appelant détient **sa propre** clé Ed25519, dont la graine (32 octets
//!   bruts) est créée une seule fois dans un fichier `0600` ([`charger_ou_creer_cle`]) ;
//! - le service appelé **épingle** les clés publiques de ses appelants dans sa
//!   configuration ([`analyser_cles_publiques`]) ;
//! - [`GardeSignee`] applique l'ordre fail-closed : clé épinglée, fenêtre
//!   d'horodatage et nonce non rejoué **avant** la lecture du corps
//!   ([`GardeSignee::controler_entetes`]), puis signature sur le corps lu
//!   ([`GardeSignee::verifier`]). **Aucune clé épinglée ⇒ tout est refusé.**
//!
//! `chemin` signé = chemin **et** requête (`/v1/x?y=1`), tels qu'envoyés.

use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read, Write};
use std::path::Path;

use miyusceau::CleSignature;
use zeroize::Zeroize;

use super::{
    b64_decoder, b64_encoder, signer_requete, verifier_requete_signee, EntetesSignees, RefusHttp, RegistreNonces,
};

/// Taille d'une graine Ed25519 (octets).
pub const TAILLE_GRAINE: usize = 32;

/// Secondes Unix courantes (0 si l'horloge est antérieure à 1970).
pub fn maintenant_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Clé de service (graine dans un fichier 0600)
// ═══════════════════════════════════════════════════════════════════════════════

/// Charge la clé de service depuis `chemin`, ou la crée (graine aléatoire, fichier
/// `0600`, dossier parent `0700`) si le fichier n'existe pas.
///
/// Refus (fail-closed) : lien symbolique, fichier non régulier, droits ouverts au
/// groupe ou aux autres (Unix), taille différente de 32 octets.
pub fn charger_ou_creer_cle(chemin: &Path) -> io::Result<CleSignature> {
    match std::fs::symlink_metadata(chemin) {
        Ok(_) => charger_cle(chemin),
        Err(e) if e.kind() == io::ErrorKind::NotFound => match creer_cle(chemin) {
            // Course : un autre processus vient de la créer.
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => charger_cle(chemin),
            r => r,
        },
        Err(e) => Err(e),
    }
}

/// Charge une clé de service existante (mêmes contrôles que [`charger_ou_creer_cle`]).
pub fn charger_cle(chemin: &Path) -> io::Result<CleSignature> {
    let avant = std::fs::symlink_metadata(chemin)?;
    if !avant.file_type().is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} : fichier de clé non régulier (lien symbolique refusé)", chemin.display()),
        ));
    }
    let mut fichier = std::fs::File::open(chemin)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        let ouvert = fichier.metadata()?;
        // Le fichier ouvert doit être celui contrôlé (pas de substitution entre-temps).
        if ouvert.dev() != avant.dev() || ouvert.ino() != avant.ino() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "fichier de clé substitué pendant l'ouverture"));
        }
        if ouvert.permissions().mode() & 0o077 != 0 {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("{} : droits trop ouverts (0600 attendu)", chemin.display()),
            ));
        }
    }
    let mut tampon = Vec::with_capacity(TAILLE_GRAINE + 1);
    let lu = (&mut fichier).take(TAILLE_GRAINE as u64 + 1).read_to_end(&mut tampon);
    let resultat = match lu {
        Ok(TAILLE_GRAINE) => {
            let mut graine = [0u8; TAILLE_GRAINE];
            graine.copy_from_slice(&tampon);
            let cle = CleSignature::depuis_graine(&graine);
            graine.zeroize();
            Ok(cle)
        }
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{} : graine de {TAILLE_GRAINE} octets attendue", chemin.display()),
        )),
        Err(e) => Err(e),
    };
    tampon.zeroize();
    resultat
}

fn creer_cle(chemin: &Path) -> io::Result<CleSignature> {
    if let Some(parent) = chemin.parent().filter(|p| !p.as_os_str().is_empty()) {
        let mut dossier = std::fs::DirBuilder::new();
        dossier.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            dossier.mode(0o700);
        }
        dossier.create(parent)?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut fichier = options.open(chemin)?;
    let cle = CleSignature::generer();
    let mut graine = cle.graine();
    let ecrit = fichier.write_all(&graine).and_then(|()| fichier.sync_all());
    graine.zeroize();
    ecrit?;
    Ok(cle)
}

// ═══════════════════════════════════════════════════════════════════════════════
// Clés publiques épinglées
// ═══════════════════════════════════════════════════════════════════════════════

/// Clé publique en base64 URL sans remplissage (forme à épingler côté service).
pub fn encoder_cle_publique(cle: &[u8; 32]) -> String {
    b64_encoder(cle)
}

/// Analyse une clé publique épinglée : base64 URL sans remplissage (43 caractères)
/// ou hexadécimal (64 caractères). `None` si mal formée.
pub fn analyser_cle_publique(texte: &str) -> Option<[u8; 32]> {
    let t = texte.trim();
    if t.len() == 64 && t.bytes().all(|b| b.is_ascii_hexdigit()) {
        let mut cle = [0u8; 32];
        for (i, paire) in t.as_bytes().chunks(2).enumerate() {
            let haut = char::from(paire[0]).to_digit(16)?;
            let bas = char::from(paire[1]).to_digit(16)?;
            cle[i] = u8::try_from(haut * 16 + bas).ok()?;
        }
        return Some(cle);
    }
    b64_decoder(t).and_then(|v| v.try_into().ok())
}

/// Analyse une liste de clés épinglées. Une seule entrée mal formée ⇒ erreur
/// (le service doit alors refuser de démarrer).
pub fn analyser_cles_publiques<S: AsRef<str>>(textes: &[S]) -> Result<Vec<[u8; 32]>, String> {
    textes
        .iter()
        .enumerate()
        .map(|(i, t)| analyser_cle_publique(t.as_ref()).ok_or_else(|| format!("clé publique n°{} mal formée", i + 1)))
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Côté appelant
// ═══════════════════════════════════════════════════════════════════════════════

/// En-têtes `X-Miyu-*` d'une requête signée maintenant par `cle`, pour `usage`.
/// `chemin` = chemin + requête exactement tels qu'envoyés.
pub fn entetes_signees(
    cle: &CleSignature,
    usage: &str,
    methode: &str,
    chemin: &str,
    corps: &[u8],
) -> [(&'static str, String); 4] {
    signer_requete(cle, usage, methode, chemin, corps, maintenant_unix()).en_entetes()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Côté service : garde
// ═══════════════════════════════════════════════════════════════════════════════

/// Garde d'un service : clés d'appelants épinglées + registre anti-rejeu.
///
/// Fail-closed : sans clé épinglée, toute requête est refusée.
pub struct GardeSignee {
    epingles: HashSet<[u8; 32]>,
    nonces: RegistreNonces,
}

impl fmt::Debug for GardeSignee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GardeSignee")
            .field("cles_epinglees", &self.epingles.len())
            .field("fenetre", &self.nonces.fenetre())
            .finish_non_exhaustive()
    }
}

impl Default for GardeSignee {
    /// Garde **sans** clé épinglée : refuse tout.
    fn default() -> Self {
        Self::new([])
    }
}

impl GardeSignee {
    /// Garde avec registre de nonces par défaut (± 300 s).
    pub fn new(epingles: impl IntoIterator<Item = [u8; 32]>) -> Self {
        Self::avec_registre(epingles, RegistreNonces::default())
    }

    /// Garde avec un registre de nonces donné (tests, capacité propre).
    pub fn avec_registre(epingles: impl IntoIterator<Item = [u8; 32]>, nonces: RegistreNonces) -> Self {
        Self { epingles: epingles.into_iter().collect(), nonces }
    }

    /// Vrai si au moins une clé est épinglée.
    pub fn est_configuree(&self) -> bool {
        !self.epingles.is_empty()
    }

    /// Vrai si `cle` est épinglée.
    pub fn autorise(&self, cle: &[u8; 32]) -> bool {
        self.epingles.contains(cle)
    }

    /// Contrôles **avant** lecture du corps : en-têtes bien formés, signataire
    /// épinglé, horodatage dans la fenêtre, nonce pas encore vu.
    pub fn controler_entetes(
        &self,
        signataire: Option<&str>,
        horodatage: Option<&str>,
        nonce: Option<&str>,
        signature: Option<&str>,
        maintenant: u64,
    ) -> Result<EntetesSignees, RefusHttp> {
        if !self.est_configuree() {
            return Err(RefusHttp::SignataireNonAutorise);
        }
        let entetes = EntetesSignees::analyser(signataire, horodatage, nonce, signature)?;
        if !self.autorise(&entetes.signataire) {
            return Err(RefusHttp::SignataireNonAutorise);
        }
        entetes.controler_horodatage(maintenant, self.nonces.fenetre())?;
        if self.nonces.deja_vu(&entetes.kid(), &entetes.nonce) {
            return Err(RefusHttp::Rejeu);
        }
        Ok(entetes)
    }

    /// Vérification complète sur le corps lu : signataire épinglé, signature sur
    /// `usage`/méthode/chemin/horodatage/nonce/SHA-256(corps), nonce enregistré.
    pub fn verifier(
        &self,
        entetes: &EntetesSignees,
        usage: &str,
        methode: &str,
        chemin: &str,
        corps: &[u8],
        maintenant: u64,
    ) -> Result<(), RefusHttp> {
        if !self.autorise(&entetes.signataire) {
            return Err(RefusHttp::SignataireNonAutorise);
        }
        verifier_requete_signee(entetes, usage, methode, chemin, corps, &self.nonces, maintenant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dossier_temp(nom: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("miyauth-service-{nom}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn cle_creee_une_fois_puis_relue() {
        let d = dossier_temp("creation");
        let chemin = d.join("keys/svc.seed");
        let a = charger_ou_creer_cle(&chemin).unwrap();
        let b = charger_ou_creer_cle(&chemin).unwrap();
        assert_eq!(a.cle_publique(), b.cle_publique());
        assert_eq!(std::fs::read(&chemin).unwrap().len(), TAILLE_GRAINE);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&chemin).unwrap().permissions().mode() & 0o777, 0o600);
            assert_eq!(std::fs::metadata(chemin.parent().unwrap()).unwrap().permissions().mode() & 0o777, 0o700);
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn cle_mal_formee_ou_ouverte_refusee() {
        let d = dossier_temp("refus");
        std::fs::create_dir_all(&d).unwrap();
        let courte = d.join("courte.seed");
        std::fs::write(&courte, [1u8; 31]).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&courte, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        assert!(charger_ou_creer_cle(&courte).is_err());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let ouverte = d.join("ouverte.seed");
            std::fs::write(&ouverte, [1u8; 32]).unwrap();
            std::fs::set_permissions(&ouverte, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert_eq!(charger_cle(&ouverte).unwrap_err().kind(), io::ErrorKind::PermissionDenied);
            let lien = d.join("lien.seed");
            std::os::unix::fs::symlink(&ouverte, &lien).unwrap();
            assert!(charger_ou_creer_cle(&lien).is_err());
        }
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn cles_publiques_b64_et_hex() {
        let cle = CleSignature::generer().cle_publique();
        assert_eq!(analyser_cle_publique(&encoder_cle_publique(&cle)), Some(cle));
        let hex = cle.iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write;
            let _ = write!(s, "{b:02x}");
            s
        });
        assert_eq!(analyser_cle_publique(&hex), Some(cle));
        assert_eq!(analyser_cle_publique(&hex.to_uppercase()), Some(cle));
        assert!(analyser_cle_publique("abc").is_none());
        assert!(analyser_cles_publiques(&[encoder_cle_publique(&cle), "x".into()]).is_err());
        assert_eq!(analyser_cles_publiques(&[hex]).unwrap(), vec![cle]);
    }

    fn garde_pour(cle: &CleSignature) -> GardeSignee {
        GardeSignee::new([cle.cle_publique()])
    }

    fn controler(g: &GardeSignee, e: &[(&'static str, String); 4], t: u64) -> Result<EntetesSignees, RefusHttp> {
        g.controler_entetes(Some(&e[0].1), Some(&e[1].1), Some(&e[2].1), Some(&e[3].1), t)
    }

    #[test]
    fn garde_accepte_puis_refuse_rejeu() {
        let cle = CleSignature::generer();
        let g = garde_pour(&cle);
        let e = signer_requete(&cle, "svc/x", "POST", "/a?b=1", b"{}", 1000).en_entetes();
        let ent = controler(&g, &e, 1010).unwrap();
        assert!(g.verifier(&ent, "svc/x", "POST", "/a?b=1", b"{}", 1010).is_ok());
        assert_eq!(controler(&g, &e, 1010).unwrap_err(), RefusHttp::Rejeu);
        assert_eq!(g.verifier(&ent, "svc/x", "POST", "/a?b=1", b"{}", 1010).unwrap_err(), RefusHttp::Rejeu);
    }

    #[test]
    fn garde_refus_fail_closed() {
        let cle = CleSignature::generer();
        let g = garde_pour(&cle);
        // Sans en-têtes.
        assert!(g.controler_entetes(None, None, None, None, 1000).is_err());
        // Clé inconnue.
        let autre = CleSignature::generer();
        let e = signer_requete(&autre, "svc/x", "GET", "/a", b"", 1000).en_entetes();
        assert_eq!(controler(&g, &e, 1000).unwrap_err(), RefusHttp::SignataireNonAutorise);
        // Horodatage périmé.
        let e = signer_requete(&cle, "svc/x", "GET", "/a", b"", 1000).en_entetes();
        assert_eq!(controler(&g, &e, 1000 + 301).unwrap_err(), RefusHttp::HorodatageHorsFenetre);
        // Corps altéré, usage ou chemin différent.
        let ent = controler(&g, &e, 1000).unwrap();
        assert_eq!(g.verifier(&ent, "svc/x", "GET", "/a", b"x", 1000).unwrap_err(), RefusHttp::SignatureInvalide);
        assert_eq!(g.verifier(&ent, "svc/y", "GET", "/a", b"", 1000).unwrap_err(), RefusHttp::SignatureInvalide);
        assert_eq!(g.verifier(&ent, "svc/x", "GET", "/b", b"", 1000).unwrap_err(), RefusHttp::SignatureInvalide);
        // Aucune clé épinglée : tout est refusé.
        let vide = GardeSignee::new([]);
        assert!(!vide.est_configuree());
        assert_eq!(controler(&vide, &e, 1000).unwrap_err(), RefusHttp::SignataireNonAutorise);
        assert_eq!(vide.verifier(&ent, "svc/x", "GET", "/a", b"", 1000).unwrap_err(), RefusHttp::SignataireNonAutorise);
    }

    #[test]
    fn entetes_signees_maintenant() {
        let cle = CleSignature::generer();
        let g = garde_pour(&cle);
        let e = entetes_signees(&cle, "svc/x", "DELETE", "/n/1", b"");
        let ent = controler(&g, &e, maintenant_unix()).unwrap();
        assert!(g.verifier(&ent, "svc/x", "DELETE", "/n/1", b"", maintenant_unix()).is_ok());
    }
}
