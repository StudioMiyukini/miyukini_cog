//! @id     origin_web_forum_auth
//! @do     authentifier_forum_sans_secret_partage
//! @role   securite
//! @layer  app
//! @human  origin — authentification forum : passeport + visa + preuve de possession, sync signée par COG.

//! Authentification forum d'Origin (inventaire des secrets partagés #11 à #13, sortie du lot 2).
//!
//! Plus aucun secret partagé : ni clé HMAC de session, ni secret de synchronisation.
//!
//! # Connexion d'un utilisateur de COG (#11, #12)
//!
//! Origin agit comme **COG hôte** (doc gelée lot 2, §5 à §7) avec sa propre clé de
//! scellement des visas (`[forum] cle_emission_path`, `cog_id` = empreinte de la clé).
//!
//! 1. `POST /api/auth/forum/defi` → `{defi_id, defi, audience: "forum", expire_a, cog_id}`
//!    (défi de 32 o, usage unique, 60 s ; `cog_id` = destination à mettre dans le passeport).
//! 2. Le COG d'origine de l'utilisateur émet un **passeport** pour la destination Origin
//!    (durée ≤ 15 min, voir « Révocation ») ; l'appareil signe la preuve de possession.
//! 3. `POST /api/auth/forum/login` avec
//!    `Authorization: MiyuPasseport <b64url(passeport)>` et
//!    `X-Miyu-PoP: <defi_id>.<b64url(signature)>` → `{visa, expire_a, role, identite}`.
//!    Le visa (audience `forum`, ≤ 15 min) **est** la session.
//! 4. `POST /api/auth/session/verify` avec
//!    `Authorization: MiyuVisa <b64url(visa)>.<b64url(passeport)>` et une **nouvelle**
//!    preuve de possession (nouveau défi) → `{valid: true, identite, role, expire_a}`.
//!    Un visa sans preuve de possession fraîche est refusé (pas de jeton au porteur).
//!
//! Liaison de la preuve : Origin est derrière un mandataire inverse qui termine TLS, l'exporteur
//! TLS n'est pas disponible ; la preuve est liée à la requête (méthode, chemin, SHA-256 du
//! corps) par [`miyauth::http::liaison_requete`].
//!
//! Passeports acceptés : seulement ceux des COG épinglés dans `[forum] cogs_reconnus`
//! (`cog_id` + clé d'émission, niveau « reconnu »). Choix fail-closed : la chaîne
//! racine → clé d'émission (certificat) n'est pas encore spécifiée (A-003).
//!
//! Révocation : aucune liste de révocation des COG reconnus n'est encore distribuée à
//! Origin. Choix fail-closed : Origin n'accepte que des passeports de **15 minutes au
//! plus** ; un appareil révoqué par son COG d'origine ne peut plus obtenir de passeport et
//! perd l'accès en moins de 15 min (même borne que la fraîcheur des listes de révocation).
//! Les COG non reconnus sont refusés (liste absente).
//!
//! # Synchronisation des profils (#13)
//!
//! `POST /api/auth/forum/sync` : requête signée ([`miyauth::http`], usage
//! `origin/forum-sync`) par la **clé d'identité** d'un COG épinglée dans
//! `[forum] sync_cles` (`cog_id` = empreinte de la clé), horodatage ± 5 min, nonce
//! anti-rejeu. Un COG ne peut écrire que ses propres profils.
//!
//! # Mot de passe (personnes sans COG)
//!
//! `POST /api/auth/forum/validate` (email + mot de passe Argon2id) n'existe que si
//! `[forum] password_login = true` (**désactivé par défaut**). Il ne délivre plus de
//! jeton de session.

use argon2::password_hash::{PasswordHash, PasswordVerifier};
use miyauth::http::{
    b64_encoder, verifier_requete_passeport, verifier_requete_signee, verifier_requete_visa, EntetesSignees,
    MagasinDefis, MagasinDefisMemoire, RefusHttp, RegistreNonces, RequeteHttp, ENTETE_HORODATAGE, ENTETE_NONCE,
    ENTETE_POP, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE,
};
use miyauth::titres::DUREE_MAX_VISA_SECS;
use miyauth::{
    emettre_visa, AnnuaireEmetteurs, CibleRevocation, CogId, DemandeVisa, ErreurRevocation, IdentityRole,
    PolitiqueIdentite, Portees, SourceRevocation,
};
use miyusceau::{CleSignature, NiveauConfiance};
use miyuwebway_participant::identite::cog_id_depuis_cle;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Mutex;
use tracing::{debug, info, warn};

use crate::config::ForumConfig;

// ---------------------------------------------------------------------------
// Constantes
// ---------------------------------------------------------------------------

/// Nombre max de tentatives échouées avant verrouillage du compte (mot de passe).
const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Durée de verrouillage en secondes (15 min).
const LOCKOUT_DURATION_SECS: u64 = 900;
/// Audience des visas forum.
pub const AUDIENCE_FORUM: &str = "forum";
/// Usage (séparation de domaine) des requêtes de sync signées.
pub const USAGE_SYNC: &str = "origin/forum-sync";
/// Durée maximale d'un passeport accepté par Origin (voir « Révocation »).
pub const DUREE_MAX_PASSEPORT_FORUM_SECS: u64 = 15 * 60;
/// Préfixe des portées recopiées du passeport dans le visa forum.
const PREFIXE_PORTEE_FORUM: &str = "forum:";
/// Taille maximale du corps des routes `/api/auth/*` (contrôlée avant lecture).
pub const TAILLE_MAX_CORPS_AUTH: usize = 4096;

const ROUTE_DEFI: &str = "/api/auth/forum/defi";
const ROUTE_LOGIN: &str = "/api/auth/forum/login";
const ROUTE_SYNC: &str = "/api/auth/forum/sync";
const ROUTE_SESSION: &str = "/api/auth/session/verify";

// ---------------------------------------------------------------------------
// Types publics
// ---------------------------------------------------------------------------

/// Profil forum synchronisé (connexion par mot de passe).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumProfile {
    /// ID profil Central (UUID).
    pub id: String,
    /// Courriel (identifiant de connexion).
    pub email: String,
    /// Pseudonyme affiché.
    pub pseudonyme: Option<String>,
}

/// État de verrouillage d'un compte.
#[derive(Debug, Clone, Default)]
struct AccountLockState {
    failed_attempts: u32,
    locked_until: Option<u64>,
}

/// Paramètres de sécurité du forum (issus de `[forum]`).
#[derive(Debug, Default)]
pub struct ParametresForum {
    /// Clé de scellement des visas d'Origin (None = connexion par passeport désactivée).
    pub cle_hote: Option<CleSignature>,
    /// COG reconnus : `(cog_id, clé d'émission)`.
    pub cogs_reconnus: Vec<(CogId, [u8; 32])>,
    /// Clés d'identité des COG autorisés à synchroniser.
    pub sync_cles: Vec<[u8; 32]>,
    /// Connexion par mot de passe autorisée.
    pub password_login: bool,
    /// Propriétaire désigné des profils hérités (`cog_id` vide) ; None = lecture seule.
    pub cog_heritage: Option<String>,
}

fn cle_hex(texte: &str, champ: &str) -> Result<[u8; 32], String> {
    hex::decode(texte.trim())
        .ok()
        .and_then(|v| v.try_into().ok())
        .ok_or_else(|| format!("{champ} : clé publique hexadécimale de 32 octets attendue"))
}

impl ParametresForum {
    /// Construit les paramètres depuis la configuration (charge ou crée la clé d'Origin).
    /// Toute entrée invalide est une erreur (fail-closed : pas d'entrée ignorée).
    pub fn depuis_config(config: &ForumConfig) -> Result<Self, String> {
        let cle_hote = match config.cle_emission_path.as_deref() {
            Some(chemin) => Some(CleSignature::depuis_graine(&charger_ou_creer_graine(Path::new(chemin))?)),
            None => None,
        };
        let mut cogs_reconnus = Vec::new();
        for c in &config.cogs_reconnus {
            let cog = CogId::new(c.cog_id.trim()).ok_or_else(|| format!("forum.cogs_reconnus : cog_id invalide « {} »", c.cog_id))?;
            cogs_reconnus.push((cog, cle_hex(&c.cle_emission, "forum.cogs_reconnus.cle_emission")?));
        }
        let sync_cles =
            config.sync_cles.iter().map(|c| cle_hex(c, "forum.sync_cles")).collect::<Result<Vec<_>, _>>()?;
        let cog_heritage = match config.cog_heritage.as_deref().map(str::trim) {
            Some(c) => Some(
                CogId::new(c)
                    .ok_or_else(|| format!("forum.cog_heritage : cog_id invalide « {c} »"))?
                    .as_str()
                    .to_string(),
            ),
            None => None,
        };
        Ok(Self { cle_hote, cogs_reconnus, sync_cles, password_login: config.password_login, cog_heritage })
    }
}

/// Charge la graine de 32 octets, ou la crée (`0600`, création exclusive).
fn charger_ou_creer_graine(chemin: &Path) -> Result<[u8; 32], String> {
    if chemin.exists() {
        let octets = std::fs::read(chemin).map_err(|e| format!("clé forum : {e}"))?;
        return octets.as_slice().try_into().map_err(|_| "clé forum : graine de 32 octets attendue".to_string());
    }
    if let Some(parent) = chemin.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| format!("clé forum : {e}"))?;
    }
    let graine = CleSignature::generer().graine();
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut fichier = options.open(chemin).map_err(|e| format!("clé forum : {e}"))?;
    std::io::Write::write_all(&mut fichier, &graine).map_err(|e| format!("clé forum : {e}"))?;
    info!("Clé de scellement des visas forum créée : {}", chemin.display());
    Ok(graine)
}

// ---------------------------------------------------------------------------
// COG hôte (visas forum)
// ---------------------------------------------------------------------------

/// Statut de révocation vu d'Origin (voir l'en-tête du module).
#[derive(Debug)]
struct RevocationsOrigin {
    cog_local: CogId,
    reconnus: HashSet<CogId>,
    locales: Mutex<HashSet<CibleRevocation>>,
}

impl SourceRevocation for RevocationsOrigin {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, _maintenant: u64) -> Result<bool, ErreurRevocation> {
        if emetteur == &self.cog_local {
            return Ok(self.locales.lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains(cible));
        }
        if self.reconnus.contains(emetteur) {
            // Pas de liste distribuée : compensé par la durée maximale des passeports (15 min).
            return Ok(false);
        }
        Err(ErreurRevocation::ListeAbsente)
    }
}

/// Origin en tant que COG hôte du forum.
struct HoteForum {
    cog: CogId,
    cle: CleSignature,
    annuaire: AnnuaireEmetteurs,
    revocations: RevocationsOrigin,
    defis: MagasinDefisMemoire,
}

impl HoteForum {
    fn new(cle: CleSignature, reconnus: &[(CogId, [u8; 32])]) -> Result<Self, String> {
        let cog = CogId::new(&cog_id_depuis_cle(&cle.cle_publique())).ok_or("cog_id d'Origin invalide")?;
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire
            .ajouter(cog.clone(), &cle.cle_publique(), NiveauConfiance::Souverain)
            .map_err(|e| format!("clé forum : {e}"))?;
        let mut ids = HashSet::new();
        for (c, k) in reconnus {
            if c == &cog {
                return Err("forum.cogs_reconnus : le cog_id d'Origin ne peut pas être « reconnu »".into());
            }
            annuaire
                .ajouter(c.clone(), k, NiveauConfiance::Reconnu)
                .map_err(|e| format!("forum.cogs_reconnus ({c}) : {e}"))?;
            ids.insert(c.clone());
        }
        let revocations = RevocationsOrigin { cog_local: cog.clone(), reconnus: ids, locales: Mutex::new(HashSet::new()) };
        Ok(Self { cog, cle, annuaire, revocations, defis: MagasinDefisMemoire::default() })
    }

    fn politique(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite { cog_local: self.cog.clone(), annuaire: &self.annuaire, revocations: &self.revocations }
    }
}

fn texte_role(role: IdentityRole) -> &'static str {
    match role {
        IdentityRole::Citizen => "citizen",
        IdentityRole::Visitor => "visitor",
        IdentityRole::External => "external",
    }
}

fn identite_opaque(cog: &CogId, sujet: &[u8; 32]) -> String {
    format!("pw:{cog}:{}", hex::encode(sujet))
}

// ---------------------------------------------------------------------------
// Store
// ---------------------------------------------------------------------------

/// Stockage SQLite des profils synchronisés + état de sécurité du forum.
pub struct ForumAuthStore {
    conn: Mutex<rusqlite::Connection>,
    /// État de verrouillage par central_id (en mémoire, reset au redémarrage).
    lock_state: Mutex<HashMap<String, AccountLockState>>,
    /// COG hôte (None = connexion par passeport désactivée).
    hote: Option<HoteForum>,
    /// Clés d'identité autorisées à synchroniser → `cog_id`.
    sync_cles: HashMap<[u8; 32], String>,
    /// Nonces des requêtes de sync déjà acceptées.
    sync_nonces: RegistreNonces,
    /// Connexion par mot de passe autorisée.
    password_login: bool,
    /// Propriétaire désigné des profils hérités (`cog_id` vide) ; None = lecture seule.
    cog_heritage: Option<String>,
}

impl ForumAuthStore {
    /// Ouvre ou crée la base au chemin donné.
    pub fn open(path: impl AsRef<Path>, parametres: ParametresForum) -> Result<Self, String> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let conn = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
        let hote = match parametres.cle_hote {
            Some(cle) => Some(HoteForum::new(cle, &parametres.cogs_reconnus)?),
            None => None,
        };
        let sync_cles = parametres.sync_cles.iter().map(|k| (*k, cog_id_depuis_cle(k))).collect::<HashMap<_, _>>();
        let store = Self {
            conn: Mutex::new(conn),
            lock_state: Mutex::new(HashMap::new()),
            hote,
            sync_cles,
            sync_nonces: RegistreNonces::default(),
            password_login: parametres.password_login,
            cog_heritage: parametres.cog_heritage,
        };
        if let Some(h) = &store.hote {
            info!("Forum : connexion par passeport activée (Origin cog_id {})", h.cog);
        } else {
            warn!("Forum : [forum] cle_emission_path absent, connexion par passeport désactivée");
        }
        if store.sync_cles.is_empty() {
            warn!("Forum : [forum] sync_cles vide, /api/auth/forum/sync désactivé");
        }
        if store.password_login {
            warn!("Forum : connexion par mot de passe ACTIVÉE ([forum] password_login = true)");
        }
        store.init_schema()?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS forum_profiles (
                central_id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                pseudonyme TEXT,
                updated_at INTEGER NOT NULL,
                last_login_at INTEGER,
                failed_attempts INTEGER NOT NULL DEFAULT 0
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_forum_profiles_email ON forum_profiles(email);",
        )
        .map_err(|e| e.to_string())?;
        // Migration : colonnes ajoutées après coup.
        for col in &[
            ("last_login_at", "INTEGER"),
            ("failed_attempts", "INTEGER NOT NULL DEFAULT 0"),
            // COG propriétaire du profil (empreinte de la clé qui l'a synchronisé).
            ("cog_id", "TEXT NOT NULL DEFAULT ''"),
        ] {
            let sql = format!("ALTER TABLE forum_profiles ADD COLUMN {} {}", col.0, col.1);
            if let Err(e) = conn.execute(&sql, []) {
                if !e.to_string().contains("duplicate column name") {
                    return Err(e.to_string());
                }
            }
        }
        Ok(())
    }

    /// `cog_id` d'Origin en tant que COG hôte (None si désactivé).
    pub fn cog_hote(&self) -> Option<&CogId> {
        self.hote.as_ref().map(|h| &h.cog)
    }

    /// Indique si le canal de sync est activé.
    #[must_use]
    pub fn sync_enabled(&self) -> bool {
        !self.sync_cles.is_empty()
    }

    /// Bannit localement une cible (série de passeport/visa, clé d'appareil).
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn bannir(&self, cible: CibleRevocation) -> bool {
        self.hote.as_ref().is_some_and(|h| {
            h.revocations.locales.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(cible)
        })
    }

    // ----- Mot de passe (désactivé par défaut) -----

    /// Un hash est acceptable uniquement s'il est un hash PHC `$argon2id$` valide.
    #[must_use]
    pub fn is_valid_argon2id_hash(hash: &str) -> bool {
        hash.starts_with("$argon2id$") && PasswordHash::new(hash).is_ok()
    }

    fn verify_password(password: &str, stored_hash: &str) -> bool {
        if !stored_hash.starts_with("$argon2id$") {
            warn!("Hash non argon2id refusé à la vérification (format legacy ?) : re-sync requis");
            return false;
        }
        match PasswordHash::new(stored_hash) {
            Ok(parsed) => argon2::Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok(),
            Err(_) => false,
        }
    }

    fn is_account_locked(&self, central_id: &str) -> bool {
        let lock = self.lock_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        lock.get(central_id).and_then(|s| s.locked_until).is_some_and(|until| epoch_secs() < until)
    }

    fn record_failed_attempt(&self, central_id: &str) {
        let mut lock = self.lock_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let state = lock.entry(central_id.to_string()).or_default();
        state.failed_attempts += 1;
        if state.failed_attempts >= MAX_FAILED_ATTEMPTS {
            state.locked_until = Some(epoch_secs() + LOCKOUT_DURATION_SECS);
            warn!("Account locked for central_id={} ({} failed attempts)", central_id, state.failed_attempts);
        }
    }

    fn clear_failed_attempts(&self, central_id: &str) {
        self.lock_state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).remove(central_id);
    }

    /// Valide email + mot de passe (Argon2id) ; verrouillage de compte. Ne délivre
    /// aucun jeton. Refuse tout si `password_login` est désactivé.
    pub fn validate(&self, email: &str, password: &str) -> Result<Option<ForumProfile>, String> {
        if !self.password_login {
            return Ok(None);
        }
        let email = email.trim().to_lowercase();
        if email.is_empty() || password.is_empty() {
            return Ok(None);
        }
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        let row = conn.query_row(
            "SELECT central_id, email, pseudonyme, password_hash FROM forum_profiles WHERE email = ?1",
            rusqlite::params![email],
            |row| {
                Ok((
                    ForumProfile {
                        id: row.get::<_, String>(0)?,
                        email: row.get::<_, String>(1)?,
                        pseudonyme: row.get::<_, Option<String>>(2)?,
                    },
                    row.get::<_, String>(3)?,
                ))
            },
        );
        match row {
            Ok((profile, stored_hash)) => {
                if self.is_account_locked(&profile.id) {
                    warn!("Login attempt on locked account: {}", profile.id);
                    return Ok(None);
                }
                if Self::verify_password(password, &stored_hash) {
                    let now = epoch_secs().cast_signed();
                    let _ = conn.execute(
                        "UPDATE forum_profiles SET last_login_at = ?1, failed_attempts = 0 WHERE central_id = ?2",
                        rusqlite::params![now, profile.id],
                    );
                    self.clear_failed_attempts(&profile.id);
                    Ok(Some(profile))
                } else {
                    self.record_failed_attempt(&profile.id);
                    let _ = conn.execute(
                        "UPDATE forum_profiles SET failed_attempts = failed_attempts + 1 WHERE central_id = ?1",
                        rusqlite::params![profile.id],
                    );
                    Ok(None)
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Met à jour ou insère un profil pour le COG `cog_id` (après requête signée vérifiée).
    ///
    /// Un COG ne peut pas écraser le profil (ni le courriel) d'un autre COG. Les profils
    /// hérités de l'ancien canal HMAC (`cog_id` vide) n'appartiennent qu'au COG désigné
    /// par `[forum] cog_heritage` ; sans désignation, ils restent en lecture seule
    /// (jamais d'adoption par le premier COG venu).
    fn sync_profile(
        &self,
        cog_id: &str,
        central_id: &str,
        email: &str,
        password_hash: &str,
        pseudonyme: Option<&str>,
    ) -> Result<(), String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || central_id.is_empty() {
            return Err("central_id et email requis".to_string());
        }
        if !Self::is_valid_argon2id_hash(password_hash) {
            return Err("password_hash doit être au format $argon2id$".to_string());
        }
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        let proprietaire = |sql: &str, valeur: &str| -> Result<Option<(String, String)>, String> {
            match conn.query_row(sql, rusqlite::params![valeur], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))) {
                Ok(v) => Ok(Some(v)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.to_string()),
            }
        };
        // Propriétaire effectif : le COG inscrit, ou pour un profil hérité le COG désigné.
        let heritage = self.cog_heritage.as_deref();
        let detenu_par = |cog: &str| -> bool {
            if cog.is_empty() { heritage == Some(cog_id) } else { cog == cog_id }
        };
        if let Some((_, cog)) = proprietaire("SELECT central_id, cog_id FROM forum_profiles WHERE central_id = ?1", central_id)? {
            if !detenu_par(&cog) {
                return Err("profil détenu par un autre COG (ou hérité non attribué)".to_string());
            }
        }
        if let Some((autre_id, cog)) = proprietaire("SELECT central_id, cog_id FROM forum_profiles WHERE email = ?1", &email)? {
            if autre_id != central_id || !detenu_par(&cog) {
                return Err("courriel déjà utilisé par un autre profil".to_string());
            }
        }
        let now = epoch_secs().cast_signed();
        conn.execute(
            "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at, cog_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(central_id) DO UPDATE SET
               email = excluded.email,
               password_hash = excluded.password_hash,
               pseudonyme = excluded.pseudonyme,
               updated_at = excluded.updated_at,
               cog_id = excluded.cog_id",
            rusqlite::params![central_id, email, password_hash, pseudonyme, now, cog_id],
        )
        .map_err(|e| e.to_string())?;
        debug!("Forum profile synced: central_id={} cog_id={}", central_id, cog_id);
        Ok(())
    }

    /// Contrôle des en-têtes de sync **avant** lecture du corps : clé épinglée,
    /// horodatage, nonce non déjà vu. Rend le `cog_id` du signataire.
    fn controler_entetes_sync(&self, headers: &[String], maintenant: u64) -> Result<(EntetesSignees, String), RefusHttp> {
        let entetes = EntetesSignees::analyser(
            header_value(headers, ENTETE_SIGNATAIRE).as_deref(),
            header_value(headers, ENTETE_HORODATAGE).as_deref(),
            header_value(headers, ENTETE_NONCE).as_deref(),
            header_value(headers, ENTETE_SIGNATURE).as_deref(),
        )?;
        let cog = self.sync_cles.get(&entetes.signataire).ok_or(RefusHttp::SignataireNonAutorise)?.clone();
        entetes.controler_horodatage(maintenant, self.sync_nonces.fenetre())?;
        if self.sync_nonces.deja_vu(&entetes.kid(), &entetes.nonce) {
            return Err(RefusHttp::Rejeu);
        }
        Ok((entetes, cog))
    }
}

// ---------------------------------------------------------------------------
// Requêtes API
// ---------------------------------------------------------------------------

/// Corps JSON pour POST /api/auth/forum/validate (mot de passe, désactivé par défaut).
#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    /// Courriel.
    pub email: String,
    /// Mot de passe.
    pub password: String,
}

/// Corps JSON pour POST /api/auth/forum/sync (requête signée par un COG).
#[derive(Debug, Deserialize)]
pub struct SyncRequest {
    /// ID du profil dans le COG émetteur.
    pub central_id: String,
    /// Courriel.
    pub email: String,
    /// Hash du mot de passe (Argon2id `$argon2id$` obligatoire).
    pub password_hash: String,
    /// Pseudonyme.
    pub pseudonyme: Option<String>,
}

fn chemin_sans_requete(path: &str) -> &str {
    path.split_once('?').map_or(path, |(p, _)| p).trim_end_matches('/')
}

/// Contrôles effectués **avant** la lecture du corps (en-têtes seuls). `Some` = réponse
/// de refus à renvoyer immédiatement.
pub fn precontrole(path: &str, headers: &[String], store: &ForumAuthStore) -> Option<super::server::RouteResponse> {
    if chemin_sans_requete(path) == ROUTE_SYNC {
        if !store.sync_enabled() {
            return Some(json_response(503, r#"{"ok":false,"error":"sync_disabled"}"#));
        }
        if let Err(e) = store.controler_entetes_sync(headers, epoch_secs()) {
            warn!("Sync forum refusée avant lecture du corps : {}", e);
            return Some(refus_http(&e));
        }
    }
    None
}

fn refus_http(e: &RefusHttp) -> super::server::RouteResponse {
    if e.statut() == 503 {
        json_response(503, r#"{"ok":false,"error":"unavailable"}"#)
    } else {
        json_response(401, r#"{"ok":false,"valid":false,"error":"unauthorized"}"#)
    }
}

fn json_value(status: u16, v: &serde_json::Value) -> super::server::RouteResponse {
    json_response(status, &serde_json::to_string(v).unwrap_or_default())
}

/// Traite une requête API auth (POST, corps déjà lu et borné).
pub fn handle_api(path: &str, body: &[u8], headers: &[String], store: &ForumAuthStore) -> super::server::RouteResponse {
    let path_clean = chemin_sans_requete(path);
    if body.len() > TAILLE_MAX_CORPS_AUTH {
        return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
    }
    let maintenant = epoch_secs();
    match path_clean {
        ROUTE_DEFI => route_defi(store, maintenant),
        ROUTE_LOGIN => route_login(store, path_clean, body, headers, maintenant),
        ROUTE_SESSION => route_session(store, path_clean, body, headers, maintenant),
        ROUTE_SYNC => route_sync(store, path_clean, body, headers, maintenant),
        "/api/auth/forum/validate" | "/api/auth/validate" => route_validate(store, body),
        // `/api/auth/security/info` : retiré du routage public (404).
        _ => json_response(404, r#"{"ok":false,"error":"not_found"}"#),
    }
}

fn hote_ou_503(store: &ForumAuthStore) -> Result<&HoteForum, super::server::RouteResponse> {
    store.hote.as_ref().ok_or_else(|| json_response(503, r#"{"ok":false,"error":"passport_login_disabled"}"#))
}

fn route_defi(store: &ForumAuthStore, maintenant: u64) -> super::server::RouteResponse {
    let hote = match hote_ou_503(store) {
        Ok(h) => h,
        Err(r) => return r,
    };
    match hote.defis.emettre(AUDIENCE_FORUM, maintenant) {
        Ok(d) => json_value(
            200,
            &serde_json::json!({
                "ok": true,
                "defi_id": d.id_b64(),
                "defi": d.defi_b64(),
                "audience": d.audience,
                "expire_a": d.expire_a,
                "cog_id": hote.cog.as_str(),
            }),
        ),
        Err(e) => refus_http(&e),
    }
}

fn requete<'a>(chemin: &'a str, body: &'a [u8], autorisation: Option<&'a str>, pop: Option<&'a str>) -> RequeteHttp<'a> {
    RequeteHttp { methode: "POST", chemin, corps: body, autorisation, pop, liaison: None }
}

fn route_login(store: &ForumAuthStore, chemin: &str, body: &[u8], headers: &[String], maintenant: u64) -> super::server::RouteResponse {
    let hote = match hote_ou_503(store) {
        Ok(h) => h,
        Err(r) => return r,
    };
    let autorisation = header_value(headers, "authorization");
    let pop = header_value(headers, ENTETE_POP);
    let req = requete(chemin, body, autorisation.as_deref(), pop.as_deref());
    let possession = match verifier_requete_passeport(&req, AUDIENCE_FORUM, &hote.politique(), &hote.defis, maintenant) {
        Ok(p) => p,
        Err(e) => {
            warn!("Connexion forum refusée : {}", e);
            return refus_http(&e);
        }
    };
    let p = possession.passeport().passeport();
    if p.pas_apres.saturating_sub(p.pas_avant) > DUREE_MAX_PASSEPORT_FORUM_SECS {
        warn!("Connexion forum refusée : passeport de plus de 15 min (révocation non vérifiable)");
        return json_response(401, r#"{"ok":false,"error":"passport_too_long"}"#);
    }
    let portees = p.portees.as_slice().iter().filter(|x| x.starts_with(PREFIXE_PORTEE_FORUM)).cloned();
    let Some(portees) = Portees::new(portees) else {
        return json_response(401, r#"{"ok":false,"error":"unauthorized"}"#);
    };
    let pas_avant = maintenant.max(p.pas_avant);
    let pas_apres = maintenant.saturating_add(DUREE_MAX_VISA_SECS).min(p.pas_apres);
    if pas_apres <= pas_avant {
        return json_response(401, r#"{"ok":false,"error":"unauthorized"}"#);
    }
    let demande = DemandeVisa { audience: AUDIENCE_FORUM.to_string(), portees, pas_avant, pas_apres };
    match emettre_visa(&hote.cog, &hote.cle, &possession, &demande) {
        Ok(visa) => {
            let role = if p.emetteur_cog == hote.cog { IdentityRole::Citizen } else { IdentityRole::Visitor };
            json_value(
                200,
                &serde_json::json!({
                    "ok": true,
                    "visa": b64_encoder(&visa),
                    "expire_a": pas_apres,
                    "role": texte_role(role),
                    "identite": identite_opaque(&p.emetteur_cog, &p.sujet),
                }),
            )
        }
        Err(e) => {
            warn!("Émission de visa forum refusée : {}", e);
            json_response(401, r#"{"ok":false,"error":"unauthorized"}"#)
        }
    }
}

fn route_session(store: &ForumAuthStore, chemin: &str, body: &[u8], headers: &[String], maintenant: u64) -> super::server::RouteResponse {
    let hote = match hote_ou_503(store) {
        Ok(h) => h,
        Err(r) => return r,
    };
    let autorisation = header_value(headers, "authorization");
    let pop = header_value(headers, ENTETE_POP);
    let req = requete(chemin, body, autorisation.as_deref(), pop.as_deref());
    match verifier_requete_visa(&req, AUDIENCE_FORUM, &hote.politique(), &hote.defis, maintenant) {
        Ok(acces) => json_value(
            200,
            &serde_json::json!({
                "ok": true,
                "valid": true,
                "identite": identite_opaque(acces.cog_origine(), acces.sujet()),
                "role": texte_role(acces.role()),
                "portees": acces.acces().visa().portees.as_slice(),
                "expire_a": acces.acces().visa().pas_apres,
            }),
        ),
        Err(e) => {
            debug!("Session forum refusée : {}", e);
            refus_http(&e)
        }
    }
}

fn route_sync(store: &ForumAuthStore, chemin: &str, body: &[u8], headers: &[String], maintenant: u64) -> super::server::RouteResponse {
    if !store.sync_enabled() {
        return json_response(503, r#"{"ok":false,"error":"sync_disabled"}"#);
    }
    let verifie = store.controler_entetes_sync(headers, maintenant).and_then(|(entetes, cog)| {
        verifier_requete_signee(&entetes, USAGE_SYNC, "POST", chemin, body, &store.sync_nonces, maintenant).map(|()| cog)
    });
    let cog = match verifie {
        Ok(cog) => cog,
        Err(e) => {
            warn!("Sync forum refusée : {}", e);
            return refus_http(&e);
        }
    };
    let Ok(req) = serde_json::from_slice::<SyncRequest>(body) else {
        return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
    };
    match store.sync_profile(&cog, &req.central_id, &req.email, &req.password_hash, req.pseudonyme.as_deref()) {
        Ok(()) => json_response(200, r#"{"ok":true}"#),
        Err(e) => {
            warn!("Sync forum refusée ({}) : {}", cog, e);
            json_response(400, r#"{"ok":false,"error":"sync_rejected"}"#)
        }
    }
}

fn route_validate(store: &ForumAuthStore, body: &[u8]) -> super::server::RouteResponse {
    if !store.password_login {
        return json_response(403, r#"{"ok":false,"error":"password_login_disabled"}"#);
    }
    let Ok(req) = serde_json::from_slice::<ValidateRequest>(body) else {
        return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
    };
    match store.validate(&req.email, &req.password) {
        Ok(Some(profile)) => json_value(
            200,
            &serde_json::json!({
                "ok": true,
                "profile": { "id": profile.id, "email": profile.email, "pseudonyme": profile.pseudonyme },
            }),
        ),
        Ok(None) => json_response(401, r#"{"ok":false,"error":"invalid_credentials"}"#),
        Err(e) => {
            warn!("Auth validate error: {}", e);
            json_response(500, r#"{"ok":false,"error":"internal"}"#)
        }
    }
}

// ---------------------------------------------------------------------------
// Utilitaires
// ---------------------------------------------------------------------------

fn json_response(status: u16, body: &str) -> super::server::RouteResponse {
    let status_str = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        403 => "403 Forbidden",
        404 => "404 Not Found",
        429 => "429 Too Many Requests",
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    super::server::RouteResponse::Normal {
        status: status_str.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: body.to_string(),
    }
}

/// Valeur d'un en-tête HTTP (nom insensible à la casse).
pub(crate) fn header_value(headers: &[String], name: &str) -> Option<String> {
    headers.iter().find_map(|line| {
        line.split_once(':')
            .filter(|(k, _)| k.trim().eq_ignore_ascii_case(name))
            .map(|(_, v)| v.trim().to_string())
    })
}

fn epoch_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::super::server::RouteResponse;
    use super::*;
    use argon2::password_hash::{PasswordHasher, SaltString};
    use miyauth::http::{
        b64_decoder, formater_autorisation_passeport, formater_autorisation_visa, formater_pop,
        liaison_requete, signer_pop, signer_requete,
    };
    use miyauth::{emettre_passeport, DemandePasseport, SecretTitulaire};

    /// COG d'origine d'un utilisateur (reconnu par Origin) et son appareil.
    struct Monde {
        store: ForumAuthStore,
        cog_h: CogId,
        cle_h: CleSignature,
        appareil: CleSignature,
        secret: SecretTitulaire,
        cle_sync: CleSignature,
    }

    fn monde_avec(password_login: bool) -> Monde {
        let cle_h = CleSignature::generer();
        let cog_h = CogId::new(&cog_id_depuis_cle(&CleSignature::generer().cle_publique())).unwrap();
        let cle_sync = CleSignature::generer();
        let parametres = ParametresForum {
            cle_hote: Some(CleSignature::generer()),
            cogs_reconnus: vec![(cog_h.clone(), cle_h.cle_publique())],
            sync_cles: vec![cle_sync.cle_publique()],
            password_login,
            cog_heritage: None,
        };
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        let store = ForumAuthStore::open(path, parametres).expect("store");
        Monde { store, cog_h, cle_h, appareil: CleSignature::generer(), secret: SecretTitulaire::generer(), cle_sync }
    }

    fn monde() -> Monde {
        monde_avec(false)
    }

    fn status_of(r: &RouteResponse) -> String {
        match r {
            RouteResponse::Normal { status, .. } => status.clone(),
            _ => String::new(),
        }
    }

    fn json_of(r: &RouteResponse) -> serde_json::Value {
        match r {
            RouteResponse::Normal { body, .. } => serde_json::from_str(body).unwrap_or_default(),
            _ => serde_json::Value::Null,
        }
    }

    fn entete(nom: &str, valeur: &str) -> String {
        format!("{nom}: {valeur}\r\n")
    }

    impl Monde {
        fn cog_origin(&self) -> CogId {
            self.store.cog_hote().unwrap().clone()
        }

        fn passeport(&self, duree: u64, cle: &CleSignature) -> Vec<u8> {
            let t = epoch_secs();
            let demande = DemandePasseport {
                secret: &self.secret,
                destination: self.cog_origin(),
                cle_appareil: self.appareil.cle_publique(),
                portees: Portees::new(["forum:ecrire", "jaykoa:read"]).unwrap(),
                pas_avant: t - 10,
                pas_apres: t - 10 + duree,
            };
            emettre_passeport(&self.cog_h, cle, &demande).unwrap()
        }

        /// Défi émis par Origin : `(id b64, défi)`.
        fn defi(&self) -> (String, [u8; 32]) {
            let r = handle_api(ROUTE_DEFI, b"", &[], &self.store);
            assert!(status_of(&r).starts_with("200"));
            let j = json_of(&r);
            assert_eq!(j["cog_id"], self.cog_origin().as_str());
            let defi: [u8; 32] = b64_decoder(j["defi"].as_str().unwrap()).unwrap().try_into().unwrap();
            (j["defi_id"].as_str().unwrap().to_string(), defi)
        }

        fn pop(&self, chemin: &str, body: &[u8]) -> String {
            let (id, defi) = self.defi();
            let id: [u8; 16] = b64_decoder(&id).unwrap().try_into().unwrap();
            formater_pop(&id, &signer_pop(&self.appareil, &defi, AUDIENCE_FORUM, &liaison_requete("POST", chemin, body)))
        }

        fn login(&self, passeport: &[u8]) -> RouteResponse {
            let h = vec![
                entete("Authorization", &formater_autorisation_passeport(passeport)),
                entete("X-Miyu-PoP", &self.pop(ROUTE_LOGIN, b"")),
            ];
            handle_api(ROUTE_LOGIN, b"", &h, &self.store)
        }

        fn session(&self, visa: &[u8], passeport: &[u8], pop: &str) -> RouteResponse {
            let h = vec![entete("Authorization", &formater_autorisation_visa(visa, passeport)), entete("X-Miyu-PoP", pop)];
            handle_api(ROUTE_SESSION, b"", &h, &self.store)
        }
    }

    // ----- #11 / #12 : passeport + visa + PoP -----

    #[test]
    fn connexion_passeport_puis_session_visa() {
        let m = monde();
        let p = m.passeport(600, &m.cle_h);
        let r = m.login(&p);
        assert!(status_of(&r).starts_with("200"), "{}", status_of(&r));
        let j = json_of(&r);
        assert_eq!(j["role"], "visitor");
        assert!(j["identite"].as_str().unwrap().starts_with(&format!("pw:{}:", m.cog_h)));
        let visa = b64_decoder(j["visa"].as_str().unwrap()).unwrap();

        let pop = m.pop(ROUTE_SESSION, b"");
        let r = m.session(&visa, &p, &pop);
        assert!(status_of(&r).starts_with("200"), "{}", status_of(&r));
        let s = json_of(&r);
        assert_eq!(s["valid"], true);
        assert_eq!(s["identite"], j["identite"]);
        assert_eq!(s["portees"], serde_json::json!(["forum:ecrire"]));

        // Rejeu de la même preuve (défi consommé) : refus.
        assert!(status_of(&m.session(&visa, &p, &pop)).starts_with("401"));
    }

    #[test]
    fn session_sans_pop_ou_pop_d_un_autre_chemin_refusee() {
        let m = monde();
        let p = m.passeport(600, &m.cle_h);
        let visa = b64_decoder(json_of(&m.login(&p))["visa"].as_str().unwrap()).unwrap();
        let h = vec![entete("Authorization", &formater_autorisation_visa(&visa, &p))];
        assert!(status_of(&handle_api(ROUTE_SESSION, b"", &h, &m.store)).starts_with("401"));
        // Preuve liée à la route de connexion, présentée à la vérification de session.
        let pop = m.pop(ROUTE_LOGIN, b"");
        assert!(status_of(&m.session(&visa, &p, &pop)).starts_with("401"));
        // Défi inventé.
        let faux = formater_pop(&[7; 16], &[0; 64]);
        assert!(status_of(&m.session(&visa, &p, &faux)).starts_with("401"));
    }

    #[test]
    fn passeport_forge_inconnu_ou_trop_long_refuse() {
        let m = monde();
        // Signé par une clé non épinglée.
        assert!(status_of(&m.login(&m.passeport(600, &CleSignature::generer()))).starts_with("401"));
        // Plus de 15 min : révocation non vérifiable → refus.
        assert!(status_of(&m.login(&m.passeport(3600, &m.cle_h))).starts_with("401"));
        // Octet modifié.
        let mut p = m.passeport(600, &m.cle_h);
        let n = p.len();
        p[n - 1] ^= 1;
        assert!(status_of(&m.login(&p)).starts_with("401"));
        // Sans PoP.
        let h = vec![entete("Authorization", &formater_autorisation_passeport(&m.passeport(600, &m.cle_h)))];
        assert!(status_of(&handle_api(ROUTE_LOGIN, b"", &h, &m.store)).starts_with("401"));
        // PoP signée par une autre clé que celle de l'appareil.
        let (id, defi) = m.defi();
        let sig = signer_pop(&CleSignature::generer(), &defi, AUDIENCE_FORUM, &liaison_requete("POST", ROUTE_LOGIN, b""));
        let id: [u8; 16] = b64_decoder(&id).unwrap().try_into().unwrap();
        let h = vec![
            entete("Authorization", &formater_autorisation_passeport(&m.passeport(600, &m.cle_h))),
            entete("X-Miyu-PoP", &formater_pop(&id, &sig)),
        ];
        assert!(status_of(&handle_api(ROUTE_LOGIN, b"", &h, &m.store)).starts_with("401"));
    }

    #[test]
    fn appareil_banni_refuse() {
        let m = monde();
        assert!(m.store.bannir(CibleRevocation::appareil(&m.appareil.cle_publique())));
        assert!(status_of(&m.login(&m.passeport(600, &m.cle_h))).starts_with("401"));
    }

    #[test]
    fn ancien_jeton_hmac_refuse() {
        let m = monde();
        let body = br#"{"central_id":"c1","session_token":"00","expires_at":9999999999}"#;
        let r = handle_api(ROUTE_SESSION, body, &[], &m.store);
        assert!(status_of(&r).starts_with("401"));
        let h = vec![entete("Authorization", "Bearer 0011")];
        assert!(status_of(&handle_api(ROUTE_SESSION, body, &h, &m.store)).starts_with("401"));
    }

    #[test]
    fn connexion_par_passeport_desactivee_sans_cle() {
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        let store = ForumAuthStore::open(path, ParametresForum::default()).unwrap();
        for route in [ROUTE_DEFI, ROUTE_LOGIN, ROUTE_SESSION, ROUTE_SYNC] {
            assert!(status_of(&handle_api(route, b"", &[], &store)).starts_with("503"), "{route}");
        }
    }

    // ----- #13 : sync signée par la clé du COG -----

    fn hash(pw: &str) -> String {
        argon2::Argon2::default()
            .hash_password(pw.as_bytes(), &SaltString::generate(&mut rand::thread_rng()))
            .unwrap()
            .to_string()
    }

    fn sync_body(central_id: &str, email: &str, hash: &str) -> String {
        serde_json::json!({"central_id": central_id, "email": email, "password_hash": hash}).to_string()
    }

    fn signed(cle: &CleSignature, body: &str, ts: u64) -> Vec<String> {
        signer_requete(cle, USAGE_SYNC, "POST", ROUTE_SYNC, body.as_bytes(), ts)
            .en_entetes()
            .iter()
            .map(|(k, v)| entete(k, v))
            .collect()
    }

    #[test]
    fn sync_signee_acceptee_puis_rejeu_refuse() {
        let m = monde_avec(true);
        let body = sync_body("c1", "a@b.c", &hash("pw"));
        let h = signed(&m.cle_sync, &body, epoch_secs());
        assert!(precontrole(ROUTE_SYNC, &h, &m.store).is_none());
        let r = handle_api(ROUTE_SYNC, body.as_bytes(), &h, &m.store);
        assert!(status_of(&r).starts_with("200"), "{}", status_of(&r));
        assert!(m.store.validate("a@b.c", "pw").unwrap().is_some());
        // Rejeu exact : refusé dès les en-têtes et à la vérification.
        assert!(precontrole(ROUTE_SYNC, &h, &m.store).is_some());
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &h, &m.store)).starts_with("401"));
    }

    #[test]
    fn sync_non_signee_forgee_perimee_ou_inconnue_refusee() {
        let m = monde_avec(true);
        let body = sync_body("c1", "a@b.c", &hash("pw"));
        // Non signée.
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &[], &m.store)).starts_with("401"));
        assert!(precontrole(ROUTE_SYNC, &[], &m.store).is_some());
        // Corps altéré.
        let h = signed(&m.cle_sync, &body, epoch_secs());
        let altere = sync_body("c1", "evil@b.c", &hash("pw"));
        assert!(status_of(&handle_api(ROUTE_SYNC, altere.as_bytes(), &h, &m.store)).starts_with("401"));
        // Périmée.
        let h = signed(&m.cle_sync, &body, epoch_secs() - 3600);
        assert!(precontrole(ROUTE_SYNC, &h, &m.store).is_some());
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &h, &m.store)).starts_with("401"));
        // cog_id inconnu (clé non épinglée).
        let h = signed(&CleSignature::generer(), &body, epoch_secs());
        assert!(precontrole(ROUTE_SYNC, &h, &m.store).is_some());
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &h, &m.store)).starts_with("401"));
        // Ancien schéma HMAC à secret partagé : refusé.
        let h = vec![entete("X-Miyu-Timestamp", &epoch_secs().to_string()), entete("X-Miyu-Nonce", "nonce-abcdef01"), entete("X-Miyu-Signature", "00")];
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &h, &m.store)).starts_with("401"));
        assert!(m.store.validate("a@b.c", "pw").unwrap().is_none());
    }

    #[test]
    fn sync_sans_cle_configuree_renvoie_503() {
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        let store = ForumAuthStore::open(path, ParametresForum::default()).unwrap();
        assert!(status_of(&precontrole(ROUTE_SYNC, &[], &store).unwrap()).starts_with("503"));
    }

    #[test]
    fn sync_un_cog_n_ecrase_pas_un_autre() {
        let mut m = monde_avec(true);
        let autre = CleSignature::generer();
        // Second COG autorisé.
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        m.store = ForumAuthStore::open(
            path,
            ParametresForum { sync_cles: vec![m.cle_sync.cle_publique(), autre.cle_publique()], password_login: true, ..Default::default() },
        )
        .unwrap();
        let body = sync_body("c1", "a@b.c", &hash("pw"));
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store)).starts_with("200"));
        let pirate = sync_body("c1", "a@b.c", &hash("pirate"));
        assert!(status_of(&handle_api(ROUTE_SYNC, pirate.as_bytes(), &signed(&autre, &pirate, epoch_secs()), &m.store)).starts_with("400"));
        let pirate = sync_body("c2", "a@b.c", &hash("pirate"));
        assert!(status_of(&handle_api(ROUTE_SYNC, pirate.as_bytes(), &signed(&autre, &pirate, epoch_secs()), &m.store)).starts_with("400"));
        assert!(m.store.validate("a@b.c", "pw").unwrap().is_some());
    }

    #[test]
    fn profil_herite_non_adopte_sans_designation() {
        let m = monde_avec(true);
        {
            let conn = m.store.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at, cog_id)
                 VALUES ('h1', 'h@b.c', ?1, NULL, 0, '')",
                rusqlite::params![hash("ancien")],
            )
            .unwrap();
        }
        // Sans `cog_heritage`, aucun COG ne peut réécrire le profil hérité ni prendre son courriel.
        let body = sync_body("h1", "h@b.c", &hash("pirate"));
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store)).starts_with("400"));
        let body = sync_body("h2", "h@b.c", &hash("pirate"));
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store)).starts_with("400"));

        // COG désigné : peut reprendre le profil hérité.
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        let designe = cog_id_depuis_cle(&m.cle_sync.cle_publique());
        let store = ForumAuthStore::open(
            path,
            ParametresForum { sync_cles: vec![m.cle_sync.cle_publique()], cog_heritage: Some(designe), ..Default::default() },
        )
        .unwrap();
        {
            let conn = store.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at, cog_id)
                 VALUES ('h1', 'h@b.c', ?1, NULL, 0, '')",
                rusqlite::params![hash("ancien")],
            )
            .unwrap();
        }
        let body = sync_body("h1", "h@b.c", &hash("nouveau"));
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &store)).starts_with("200"));
    }

    #[test]
    fn sync_hash_non_argon2_refuse() {
        let m = monde_avec(true);
        let legacy = hex::encode([1u8; 32]);
        let body = sync_body("c1", "a@b.c", &legacy);
        let r = handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store);
        assert!(status_of(&r).starts_with("400"));
        assert!(!ForumAuthStore::is_valid_argon2id_hash("$argon2i$v=19$m=1,t=1,p=1$YQ$YQ"));
        assert!(!ForumAuthStore::verify_password("pw", &legacy));
    }

    // ----- Mot de passe : désactivé par défaut -----

    #[test]
    fn mot_de_passe_desactive_par_defaut() {
        assert!(!ForumConfig::default().password_login);
        let m = monde();
        let body = sync_body("c1", "a@b.c", &hash("pw"));
        assert!(status_of(&handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store)).starts_with("200"));
        let r = handle_api("/api/auth/forum/validate", br#"{"email":"a@b.c","password":"pw"}"#, &[], &m.store);
        assert!(status_of(&r).starts_with("403"));
        assert!(m.store.validate("a@b.c", "pw").unwrap().is_none());
    }

    #[test]
    fn mot_de_passe_active_explicitement_sans_jeton() {
        let m = monde_avec(true);
        let body = sync_body("c1", "a@b.c", &hash("pw"));
        handle_api(ROUTE_SYNC, body.as_bytes(), &signed(&m.cle_sync, &body, epoch_secs()), &m.store);
        let r = handle_api("/api/auth/forum/validate", br#"{"email":"a@b.c","password":"pw"}"#, &[], &m.store);
        assert!(status_of(&r).starts_with("200"));
        assert!(json_of(&r).get("session_token").is_none());
        let r = handle_api("/api/auth/forum/validate", br#"{"email":"a@b.c","password":"mauvais"}"#, &[], &m.store);
        assert!(status_of(&r).starts_with("401"));
    }

    #[test]
    fn security_info_retire_du_routage() {
        let m = monde();
        let r = handle_api("/api/auth/security/info", br#"{"central_id":"c1"}"#, &[], &m.store);
        assert!(status_of(&r).starts_with("404"));
    }

    #[test]
    fn graine_creee_en_0600_puis_relue() {
        let dir = std::env::temp_dir().join(format!("forum_cle_{}", uuid::Uuid::new_v4()));
        let chemin = dir.join("forum.cle");
        let a = charger_ou_creer_graine(&chemin).unwrap();
        assert_eq!(charger_ou_creer_graine(&chemin).unwrap(), a);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&chemin).unwrap().permissions().mode() & 0o777, 0o600);
        }
        std::fs::write(&chemin, b"court").unwrap();
        assert!(charger_ou_creer_graine(&chemin).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }
}
