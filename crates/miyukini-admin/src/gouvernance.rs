//! @id     miyukiniadmin_gouvernance
//! @do     gouverner_actions_admin
//! @role   securite
//! @layer  operator
//! @human  Porte StrongFather obligatoire de toute action modifiante de MiyukiniAdmin (K-11).
//!
//! # Gouvernance des actions d'administration
//!
//! - Toute requête modifiante est classée en [`ActionAdmin`] par [`classer_requete`] puis
//!   soumise à [`GouvernanceAdmin::executer_action_admin`] — point d'entrée UNIQUE du
//!   domaine — qui délègue à `bondingbrother::executer_si_autorise`
//! - Politiques : `<dossier>/admin.toml`, dossier = `COG_POLITIQUES_DIR` ou `config/strongfather`
//! - Fichier absent ou invalide ⇒ porte absente ⇒ toute action modifiante refusée
//!   (fail-closed) ; jamais « tout autoriser »
//! - Route modifiante inconnue ⇒ opération `inconnue` ⇒ refus (explicite et par défaut)
//! - Destruction / réinitialisation / effacement d'un COG ⇒ refus explicite (contrainte
//!   permanente : pas de destruction distante d'un COG)
//! - Les contrôles existants (session, jeton de setup, état VIERGE) restent en place

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bondingbrother::strongfather::{ActionType, CallContext, Intent, IntentData};
use bondingbrother::{executer_si_autorise, DecisionAutorisee, PorteStrongFather, Refus};

use crate::models::{AdminRole, AdminSession, EnvironmentState};

/// Variable d'environnement : dossier des politiques StrongFather.
pub const VAR_DOSSIER_POLITIQUES: &str = "COG_POLITIQUES_DIR";
/// Dossier par défaut des politiques (relatif au répertoire courant).
pub const DOSSIER_POLITIQUES_DEFAUT: &str = "config/strongfather";
/// Fichier de politiques du domaine Admin.
pub const FICHIER_ADMIN: &str = "admin.toml";
/// Sujet des intentions Admin.
pub const SUJET_ADMIN: &str = "admin";
/// Origine des intentions issues du serveur HTTP d'administration.
pub const ORIGINE_HTTP: &str = "admin.http";
/// Appelant quand aucun compte authentifié n'est connu.
pub const APPELANT_LOCAL: &str = "local";
/// Opération attribuée à toute route modifiante non répertoriée (toujours refusée).
pub const OPERATION_INCONNUE: &str = "inconnue";
/// Opérations de destruction d'un COG : refusées explicitement par la politique de base.
pub const OPERATIONS_DESTRUCTION: &[&str] = &[
    "environnement.detruire",
    "environnement.reinitialiser",
    "cog.effacer",
    "cog.detruire",
];

/// Dossier des politiques StrongFather (`COG_POLITIQUES_DIR` ou défaut).
pub fn dossier_politiques() -> PathBuf {
    std::env::var_os(VAR_DOSSIER_POLITIQUES)
        .filter(|v| !v.is_empty())
        .map_or_else(|| PathBuf::from(DOSSIER_POLITIQUES_DEFAUT), PathBuf::from)
}

/// @id miyukiniadmin_gouvernance_action
/// @role data
/// @layer operator
/// @do describe_admin_action
//@human / @human Action modifiante d'administration : opération canonique, type d'action, cible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionAdmin {
    /// Opération canonique (`data.operation`), ex. `module.supprimer`.
    pub operation: &'static str,
    /// Type d'action StrongFather.
    pub action: ActionType,
    /// Cible (table, ligne, module, sauvegarde…) ; vide si inconnue.
    pub cible: String,
    /// Méthode HTTP d'origine.
    pub methode: String,
    /// Chemin HTTP d'origine.
    pub chemin: String,
}

impl ActionAdmin {
    /// Nouvelle action (méthode / chemin renseignés par [`classer_requete`]).
    pub fn nouvelle(operation: &'static str, action: ActionType, cible: &str) -> Self {
        Self {
            operation,
            action,
            cible: cible.to_string(),
            methode: String::new(),
            chemin: String::new(),
        }
    }

    /// Vrai si la cible doit être lue dans le corps JSON (`champ`).
    pub fn champ_cible_corps(&self) -> Option<&'static str> {
        match self.operation {
            "sauvegarde.restaurer" => Some("backup_id"),
            "module.ajouter" => Some("id"),
            _ => None,
        }
    }
}

/// Classe une requête HTTP. `None` : requête non modifiante (GET, HEAD, OPTIONS) ou
/// authentification de l'utilisateur lui-même (login / logout), hors gouvernance.
/// Toute autre requête modifiante non répertoriée ⇒ opération `inconnue` (refusée).
pub fn classer_requete(methode: &str, chemin: &str) -> Option<ActionAdmin> {
    let methode = methode.to_ascii_uppercase();
    if matches!(methode.as_str(), "GET" | "HEAD" | "OPTIONS") {
        return None;
    }
    let segments: Vec<&str> = chemin
        .split('?')
        .next()
        .unwrap_or("")
        .split('/')
        .filter(|s| !s.is_empty())
        .collect();
    let (operation, action, cible) = match (methode.as_str(), segments.as_slice()) {
        ("POST", ["api", "auth", "login" | "logout"]) => return None,
        ("POST", ["api", "setup", "create-admin"]) => {
            ("setup.creer_premier_admin", ActionType::Creation, String::new())
        }
        ("POST", ["api", "tables", table, "rows"]) => {
            ("table.ligne.creer", ActionType::Creation, (*table).to_string())
        }
        ("PUT", ["api", "tables", table, "rows", id]) => {
            ("table.ligne.modifier", ActionType::Modification, format!("{table}/{id}"))
        }
        ("DELETE", ["api", "tables", table, "rows", id]) => {
            ("table.ligne.supprimer", ActionType::Deletion, format!("{table}/{id}"))
        }
        ("POST", ["api", "migrations", "apply"]) => {
            ("migrations.appliquer", ActionType::Modification, String::new())
        }
        ("POST", ["api", "backups"]) => ("sauvegarde.creer", ActionType::Creation, String::new()),
        ("POST", ["api", "backups", "restore"]) => {
            ("sauvegarde.restaurer", ActionType::Modification, String::new())
        }
        ("POST", ["api", "modules"]) => ("module.ajouter", ActionType::Creation, String::new()),
        ("POST", ["api", "modules", id, "tests", "run"]) => {
            ("module.tests.executer", ActionType::Evaluation, (*id).to_string())
        }
        ("POST", ["api", "modules", id, "lock"]) => {
            ("module.verrouiller", ActionType::Modification, (*id).to_string())
        }
        ("POST", ["api", "modules", id, "unlock"]) => {
            ("module.deverrouiller", ActionType::Modification, (*id).to_string())
        }
        ("DELETE", ["api", "modules", id]) => {
            ("module.supprimer", ActionType::Deletion, (*id).to_string())
        }
        _ if segments.iter().any(|s| est_segment_destruction(s)) => {
            ("environnement.detruire", ActionType::Deletion, chemin.to_string())
        }
        _ => (OPERATION_INCONNUE, ActionType::Modification, chemin.to_string()),
    };
    let mut a = ActionAdmin::nouvelle(operation, action, &cible);
    a.methode = methode;
    a.chemin = chemin.to_string();
    Some(a)
}

/// Segment de chemin évoquant une destruction / un effacement d'environnement.
fn est_segment_destruction(segment: &str) -> bool {
    let s = segment.to_ascii_lowercase();
    ["destroy", "destruction", "detruire", "wipe", "effacer", "reinit", "reset", "factory"]
        .iter()
        .any(|m| s.contains(m))
}

/// @id miyukiniadmin_gouvernance_principal
/// @role data
/// @layer operator
/// @do describe_admin_principal
//@human / @human Principal d'une action : compte de session, rôle, mode d'authentification, état.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalAdmin {
    /// Identifiant du compte (session valide) ; `None` ⇒ `local`.
    pub compte: Option<String>,
    /// Rôle du compte (instantané de session).
    pub role: Option<AdminRole>,
    /// Mode d'authentification : `session`, `jeton_setup` ou `aucune`.
    pub authentification: &'static str,
    /// État de l'environnement au démarrage.
    pub etat: EnvironmentState,
}

impl PrincipalAdmin {
    /// Principal issu d'une session validée.
    pub fn depuis_session(session: &AdminSession, etat: EnvironmentState) -> Self {
        Self {
            compte: Some(session.account_id.clone()),
            role: Some(session.role),
            authentification: "session",
            etat,
        }
    }

    /// Porteur présumé du jeton de setup (vérifié ensuite par le serveur).
    pub fn setup(etat: EnvironmentState) -> Self {
        Self {
            compte: None,
            role: None,
            authentification: "jeton_setup",
            etat,
        }
    }

    /// Aucun principal authentifié.
    pub fn anonyme(etat: EnvironmentState) -> Self {
        Self {
            compte: None,
            role: None,
            authentification: "aucune",
            etat,
        }
    }
}

/// Nom canonique d'un rôle (`data.role`).
pub const fn nom_role(role: Option<AdminRole>) -> &'static str {
    match role {
        Some(AdminRole::Admin) => "admin",
        Some(AdminRole::Recovery) => "recovery",
        Some(AdminRole::Audit) => "audit",
        None => "aucun",
    }
}

/// Nom canonique d'un état d'environnement (`meta.etat_environnement`).
pub const fn nom_etat(etat: EnvironmentState) -> &'static str {
    match etat {
        EnvironmentState::Vierge => "vierge",
        EnvironmentState::Initialise => "initialise",
        EnvironmentState::Compromis => "compromis",
    }
}

/// Intention StrongFather d'une action Admin (forme stable, voir `config/strongfather/admin.toml`).
pub fn intention(action: &ActionAdmin, principal: &PrincipalAdmin) -> Intent {
    let mut data = HashMap::new();
    data.insert("operation".to_string(), action.operation.to_string());
    data.insert("cible".to_string(), action.cible.clone());
    data.insert("role".to_string(), nom_role(principal.role).to_string());
    data.insert("methode".to_string(), action.methode.clone());
    data.insert("chemin".to_string(), action.chemin.clone());
    let mut meta = HashMap::new();
    meta.insert(
        "authentification".to_string(),
        principal.authentification.to_string(),
    );
    meta.insert(
        "etat_environnement".to_string(),
        nom_etat(principal.etat).to_string(),
    );
    Intent::new(
        uuid::Uuid::new_v4().to_string(),
        action.action,
        SUJET_ADMIN.to_string(),
        CallContext {
            caller_identity: principal
                .compte
                .clone()
                .filter(|c| !c.trim().is_empty())
                .unwrap_or_else(|| APPELANT_LOCAL.to_string()),
            origin: ORIGINE_HTTP.to_string(),
            instance: "cog".to_string(),
        },
        IntentData { content: data },
    )
    .with_metadata(meta)
}

/// @id miyukiniadmin_gouvernance_refus
/// @role error
/// @layer operator
/// @do represent_admin_refusal
//@human / @human Refus d'une action Admin : porte absente ou décision DENY de StrongFather.
#[derive(Debug)]
pub enum RefusAdmin {
    /// Politiques non chargées (fichier absent ou invalide).
    PorteAbsente(String),
    /// Décision de refus de StrongFather.
    Refuse(Refus),
}

impl std::fmt::Display for RefusAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PorteAbsente(m) => write!(f, "action refusée : {m}"),
            Self::Refuse(r) => write!(f, "action refusée par StrongFather : {r}"),
        }
    }
}

impl std::error::Error for RefusAdmin {}

/// @id miyukiniadmin_gouvernance_porte
/// @role securite
/// @layer operator
/// @do hold_admin_gate
//@human / @human Porte StrongFather du domaine Admin ; absente ⇒ toute action modifiante refusée.
#[derive(Clone)]
pub struct GouvernanceAdmin {
    porte: Result<PorteStrongFather, String>,
}

impl std::fmt::Debug for GouvernanceAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GouvernanceAdmin")
            .field("porte_chargee", &self.porte.is_ok())
            .finish()
    }
}

impl GouvernanceAdmin {
    /// Charge `admin.toml` depuis le dossier des politiques (journalise l'échec).
    pub fn charger() -> Self {
        Self::depuis_fichier(&dossier_politiques().join(FICHIER_ADMIN))
    }

    /// Charge un fichier explicite ; illisible ou invalide ⇒ porte absente.
    pub fn depuis_fichier(chemin: &Path) -> Self {
        match PorteStrongFather::depuis_fichier(chemin) {
            Ok(porte) => {
                eprintln!(
                    "MiyukiniAdmin: politiques StrongFather chargées depuis {}",
                    chemin.display()
                );
                Self { porte: Ok(porte) }
            }
            Err(e) => {
                let motif = format!(
                    "politiques Admin indisponibles ({}) : {e}",
                    chemin.display()
                );
                eprintln!(
                    "MiyukiniAdmin: ERREUR StrongFather — {motif} — toute action modifiante est REFUSÉE (fail-closed)"
                );
                Self { porte: Err(motif) }
            }
        }
    }

    /// Porte déjà construite (tests, injection).
    pub fn depuis_porte(porte: PorteStrongFather) -> Self {
        Self { porte: Ok(porte) }
    }

    /// Vrai si une porte valide est chargée.
    pub fn est_chargee(&self) -> bool {
        self.porte.is_ok()
    }

    /// Point d'entrée UNIQUE du domaine Admin : `f` n'est appelée que si StrongFather
    /// autorise l'action pour ce principal ; sinon le refus est renvoyé sans appeler `f`.
    pub fn executer_action_admin<T>(
        &self,
        action: &ActionAdmin,
        principal: &PrincipalAdmin,
        f: impl FnOnce(&DecisionAutorisee) -> T,
    ) -> Result<T, RefusAdmin> {
        let porte = self
            .porte
            .as_ref()
            .map_err(|m| RefusAdmin::PorteAbsente(m.clone()))?;
        let intent = intention(action, principal);
        executer_si_autorise(porte, &intent, |jeton| {
            eprintln!(
                "MiyukiniAdmin: StrongFather autorise '{}' ({}) par '{}' — règle {}, empreinte {}",
                action.operation,
                action.cible,
                intent.call_context.caller_identity,
                jeton.regle(),
                jeton.empreinte()
            );
            f(jeton)
        })
        .map_err(|refus| {
            eprintln!(
                "MiyukiniAdmin: StrongFather refuse '{}' ({}) par '{}' : {refus}",
                action.operation, action.cible, intent.call_context.caller_identity
            );
            RefusAdmin::Refuse(refus)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn base() -> GouvernanceAdmin {
        let chemin = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../config/strongfather")
            .join(FICHIER_ADMIN);
        let g = GouvernanceAdmin::depuis_fichier(&chemin);
        assert!(g.est_chargee(), "admin.toml doit se charger");
        g
    }

    fn session(role: AdminRole) -> AdminSession {
        AdminSession {
            session_id: "s-1".into(),
            account_id: "compte-1".into(),
            role,
            expires_at: Utc::now(),
            created_at: Utc::now(),
            ip: None,
            user_agent: None,
        }
    }

    fn admin() -> PrincipalAdmin {
        PrincipalAdmin::depuis_session(&session(AdminRole::Admin), EnvironmentState::Initialise)
    }

    fn autorise(porte: &GouvernanceAdmin, methode: &str, chemin: &str, qui: &PrincipalAdmin) -> bool {
        let action = classer_requete(methode, chemin).expect("requête modifiante");
        porte.executer_action_admin(&action, qui, |_| ()).is_ok()
    }

    const ROUTES_MODIFIANTES: &[(&str, &str)] = &[
        ("POST", "/api/tables/users/rows"),
        ("PUT", "/api/tables/users/rows/1"),
        ("DELETE", "/api/tables/users/rows/1"),
        ("POST", "/api/migrations/apply"),
        ("POST", "/api/backups"),
        ("POST", "/api/backups/restore"),
        ("POST", "/api/modules"),
        ("POST", "/api/modules/m1/tests/run"),
        ("POST", "/api/modules/m1/lock"),
        ("POST", "/api/modules/m1/unlock"),
        ("DELETE", "/api/modules/m1"),
    ];

    #[test]
    fn classement_des_routes() {
        assert_eq!(classer_requete("GET", "/api/tables"), None);
        assert_eq!(classer_requete("POST", "/api/auth/login"), None);
        assert_eq!(classer_requete("POST", "/api/auth/logout"), None);
        let a = classer_requete("DELETE", "/api/modules/m1").unwrap();
        assert_eq!((a.operation, a.action, a.cible.as_str()), ("module.supprimer", ActionType::Deletion, "m1"));
        let a = classer_requete("put", "/api/tables/t/rows/3?x=1").unwrap();
        assert_eq!((a.operation, a.cible.as_str()), ("table.ligne.modifier", "t/3"));
        assert_eq!(classer_requete("POST", "/api/nouvelle-route").unwrap().operation, OPERATION_INCONNUE);
        assert_eq!(classer_requete("POST", "/api/environment/wipe").unwrap().operation, "environnement.detruire");
        for (m, c) in ROUTES_MODIFIANTES {
            assert_ne!(classer_requete(m, c).unwrap().operation, OPERATION_INCONNUE, "{m} {c}");
        }
    }

    #[test]
    fn politique_de_base_autorise_les_admins_authentifies() {
        let g = base();
        for p in [admin(), PrincipalAdmin::depuis_session(&session(AdminRole::Recovery), EnvironmentState::Initialise)] {
            for (m, c) in ROUTES_MODIFIANTES {
                assert!(autorise(&g, m, c, &p), "{m} {c}");
            }
        }
        assert!(autorise(&g, "POST", "/api/setup/create-admin", &PrincipalAdmin::setup(EnvironmentState::Vierge)));
    }

    #[test]
    fn politique_de_base_refuse_le_reste() {
        let g = base();
        let audit = PrincipalAdmin::depuis_session(&session(AdminRole::Audit), EnvironmentState::Initialise);
        let anonyme = PrincipalAdmin::anonyme(EnvironmentState::Initialise);
        for (m, c) in ROUTES_MODIFIANTES {
            assert!(!autorise(&g, m, c, &audit), "audit {m} {c}");
            assert!(!autorise(&g, m, c, &anonyme), "anonyme {m} {c}");
        }
        // Setup hors état VIERGE ou par une session : refus.
        assert!(!autorise(&g, "POST", "/api/setup/create-admin", &PrincipalAdmin::setup(EnvironmentState::Initialise)));
        assert!(!autorise(&g, "POST", "/api/setup/create-admin", &admin()));
        // Route modifiante inconnue : refus même pour un admin.
        assert!(!autorise(&g, "POST", "/api/config/ecrire", &admin()));
    }

    /// Contrainte permanente : aucune destruction de COG par l'administration, même admin.
    #[test]
    fn destruction_de_cog_toujours_refusee() {
        let g = base();
        for op in OPERATIONS_DESTRUCTION {
            let a = ActionAdmin::nouvelle(op, ActionType::Deletion, "cog");
            let r = g.executer_action_admin(&a, &admin(), |_| ());
            match r {
                Err(RefusAdmin::Refuse(refus)) => assert_eq!(refus.regle(), "admin-refus-destruction-cog"),
                autre => panic!("{op} : refus explicite attendu, obtenu {autre:?}"),
            }
        }
        assert!(!autorise(&g, "POST", "/api/cog/destroy", &admin()));
        assert!(!autorise(&g, "DELETE", "/api/environment/reset", &admin()));
    }

    #[test]
    fn action_refusee_non_executee() {
        let g = base();
        let mut effets = 0;
        let a = classer_requete("DELETE", "/api/modules/m1").unwrap();
        let audit = PrincipalAdmin::depuis_session(&session(AdminRole::Audit), EnvironmentState::Initialise);
        assert!(g.executer_action_admin(&a, &audit, |_| effets += 1).is_err());
        assert_eq!(effets, 0);
        assert!(g.executer_action_admin(&a, &admin(), |_| effets += 1).is_ok());
        assert_eq!(effets, 1);
    }

    #[test]
    fn fichier_absent_ou_invalide_refuse_tout() {
        let g = GouvernanceAdmin::depuis_fichier(Path::new("/inexistant/admin.toml"));
        assert!(!g.est_chargee());
        let mut effets = 0;
        let a = classer_requete("POST", "/api/backups").unwrap();
        let r = g.executer_action_admin(&a, &admin(), |_| effets += 1);
        assert!(matches!(r, Err(RefusAdmin::PorteAbsente(_))));
        assert_eq!(effets, 0);

        let dir = std::env::temp_dir().join(format!("miyu-admin-sf-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join(FICHIER_ADMIN);
        std::fs::write(&f, "version = 1\n").unwrap();
        assert!(!GouvernanceAdmin::depuis_fichier(&f).est_chargee());
        std::fs::write(&f, "version = 1\n[[policy]]\nid = \"x\"\ntype = \"permission\"\neffect = \"authorize\"\nrule = \"caller ==\"\n").unwrap();
        assert!(!GouvernanceAdmin::depuis_fichier(&f).est_chargee());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn intention_porte_le_principal() {
        let a = classer_requete("POST", "/api/modules/m1/lock").unwrap();
        let i = intention(&a, &admin());
        assert_eq!(i.subject, SUJET_ADMIN);
        assert_eq!(i.call_context.caller_identity, "compte-1");
        assert_eq!(i.call_context.origin, ORIGINE_HTTP);
        assert_eq!(i.data.content["role"], "admin");
        assert_eq!(i.metadata["authentification"], "session");
        let i = intention(&a, &PrincipalAdmin::anonyme(EnvironmentState::Initialise));
        assert_eq!(i.call_context.caller_identity, APPELANT_LOCAL);
    }
}
