//! Entrepôt KindMother de la mémoire d'Alicia.
//!
//! KindMother est le **seul écrivain** (K-13) : schéma par migrations
//! versionnées non destructives, écritures par intentions typées (aucun texte
//! SQL), lectures typées, SQLCipher, audit chaîné, une clé par base.
//!
//! - **Base et opérateur** : `maia-_alicia` (propriétaire de sa seule base).
//!   Le préfixe `maia-` classe la base **`nationale`** par la règle de
//!   KindMother (mémoire de MAIA / Alicia, jamais publiable, A-009, Mémoire
//!   MAIA §10–§11) ; le segment `_alicia` n'est pas un identifiant
//!   d'utilisateur MAIA valide (`[a-z0-9][a-z0-9_-]*`) : aucune mémoire
//!   `maia-<utilisateur>` ne peut se confondre avec celle d'Alicia.
//! - **Ouverture** (fail-closed) : migrations, étiquette locale `nationale`
//!   posée dans `meta` (jamais remplacée), sensibilité `nationale` relue chez
//!   KindMother, base chiffrée ; tout écart ⇒ refus.
//! - **Indisponibilité** : erreur explicite [`MemoryError::KindMotherUnavailable`],
//!   jamais de repli sur un fichier local. Une connexion perdue est rouverte à
//!   l'appel suivant ; une lecture est rejouée une fois, une écriture
//!   interrompue ne l'est jamais (issue inconnue signalée).

// @id service.alicia.memory.kindmother
// @role memory_kindmother_backend
// @layer 7
// @do store_alicia_memory_through_kindmother
// @human Mémoire d'Alicia écrite et lue par KindMother : base maia-_alicia, nationale, chiffrée.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, RefDonnee, Sensibilite, TypedSelect,
    TypedWriteIntent,
};
use serde_json::Value;
use uuid::Uuid;

use crate::entrepot::Ligne;
use crate::errors::MemoryError;
use crate::schema::{migrations, CLE_ETIQUETTE, ETIQUETTE, META};

/// Base KindMother de la mémoire d'Alicia (classée `nationale` par le préfixe `maia-`).
pub const BASE_KINDMOTHER: &str = "maia-_alicia";
/// Opérateur KindMother de la mémoire d'Alicia (propriétaire de [`BASE_KINDMOTHER`]).
pub const OPERATEUR_KINDMOTHER: &str = BASE_KINDMOTHER;
/// Variable d'environnement de l'adresse du service KindMother.
pub const ENV_ADRESSE_KINDMOTHER: &str = "KINDMOTHER_ADDR";
/// Adresse par défaut du service KindMother.
pub const ADRESSE_KINDMOTHER_DEFAUT: &str = "127.0.0.1:50051";

/// Configuration de la mémoire servie par KindMother.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigKindMother {
    /// Adresse du service (`hôte:port`).
    pub adresse: String,
    /// Dossier des justificatifs (`maia-_alicia.key`, `server.pub`) ; `None` :
    /// emplacement standard de KindMother (`OperatorCredential::load`).
    pub justificatifs: Option<PathBuf>,
    /// Ancienne base SQLite de la mémoire (avant KindMother) : importée une
    /// fois si présente (feature `import-heritage`) puis effacée de façon
    /// sûre, sinon ouverture refusée tant qu'elle n'est pas importée. Tant
    /// qu'une copie existe, l'oubli est refusé ([`crate::copie_heritee`]).
    pub base_heritee: Option<PathBuf>,
}

impl ConfigKindMother {
    /// Adresse explicite, justificatifs à l'emplacement standard, sans ancienne base.
    pub fn new(adresse: impl Into<String>) -> Self {
        Self {
            adresse: adresse.into(),
            justificatifs: None,
            base_heritee: None,
        }
    }

    /// Adresse lue dans `KINDMOTHER_ADDR` (défaut `127.0.0.1:50051`).
    pub fn depuis_env() -> Self {
        Self::new(
            std::env::var(ENV_ADRESSE_KINDMOTHER)
                .ok()
                .filter(|a| !a.trim().is_empty())
                .unwrap_or_else(|| ADRESSE_KINDMOTHER_DEFAUT.to_string()),
        )
    }

    /// Dossier des justificatifs.
    pub fn avec_justificatifs(mut self, dossier: impl Into<PathBuf>) -> Self {
        self.justificatifs = Some(dossier.into());
        self
    }

    /// Ancienne base SQLite à importer (une fois).
    pub fn avec_base_heritee(mut self, chemin: impl Into<PathBuf>) -> Self {
        self.base_heritee = Some(chemin.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Pont synchrone
// ---------------------------------------------------------------------------

/// Runtime Tokio du pont synchrone (l'API du store est synchrone).
fn runtime() -> Result<&'static tokio::runtime::Runtime, MemoryError> {
    static RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();
    RUNTIME
        .get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .thread_name("alicia-memoire-km")
                .enable_all()
                .build()
                .map_err(|e| e.to_string())
        })
        .as_ref()
        .map_err(|e| MemoryError::KindMother(format!("runtime tokio : {e}")))
}

/// Exécute un futur jusqu'à son terme depuis du code synchrone.
///
/// Hors runtime : `block_on` sur le runtime du pont. Dans un runtime (quel
/// qu'en soit le type) : exécution sur un fil dédié (pas de `block_in_place`,
/// donc pas de panique dans un runtime `current_thread`). Le futur est créé
/// sur ce fil : il n'a pas besoin d'être `Send`.
fn run_blocking<F, Fut>(f: F) -> Result<Fut::Output, MemoryError>
where
    F: FnOnce() -> Fut + Send,
    Fut: Future,
    Fut::Output: Send,
{
    let rt = runtime()?;
    if tokio::runtime::Handle::try_current().is_err() {
        return Ok(rt.block_on(f()));
    }
    std::thread::scope(|scope| {
        scope
            .spawn(|| rt.block_on(f()))
            .join()
            .map_err(|_| MemoryError::KindMother("fil du pont KindMother interrompu".to_string()))
    })
}

/// Erreur de transport : connexion perdue ou impossible.
fn est_transport(e: &ClientError) -> bool {
    matches!(
        e,
        ClientError::Connection(_)
            | ClientError::Transport(_)
            | ClientError::Timeout(_)
            | ClientError::NotConnected
    )
}

fn err_client(e: &ClientError) -> MemoryError {
    if est_transport(e) {
        MemoryError::KindMotherUnavailable(e.to_string())
    } else {
        MemoryError::KindMother(e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Entrepôt
// ---------------------------------------------------------------------------

/// Entrepôt KindMother (opérateur et base `maia-_alicia`).
///
/// Le justificatif reste en mémoire pour la reconnexion (effacé à la
/// destruction, `ZeroizeOnDrop`).
pub(crate) struct EntrepotKindMother {
    adresse: String,
    justificatif: OperatorCredential,
    client: Mutex<Option<Arc<KindMotherClient>>>,
}

impl EntrepotKindMother {
    /// Connexion authentifiée puis préparation (migrations, étiquette, chiffrement).
    pub(crate) fn connecter(cfg: &ConfigKindMother) -> Result<Self, MemoryError> {
        let justificatif = match &cfg.justificatifs {
            Some(dir) => OperatorCredential::load_from_dir(OPERATEUR_KINDMOTHER, dir),
            None => OperatorCredential::load(OPERATEUR_KINDMOTHER),
        }
        .map_err(|e| {
            MemoryError::KindMother(format!(
                "justificatif de l'opérateur {OPERATEUR_KINDMOTHER} : {e}"
            ))
        })?;
        let entrepot = Self {
            adresse: cfg.adresse.clone(),
            justificatif,
            client: Mutex::new(None),
        };
        entrepot.client()?;
        entrepot.preparer()?;
        Ok(entrepot)
    }

    /// Client connecté (rouvert si la connexion précédente a été perdue).
    fn client(&self) -> Result<Arc<KindMotherClient>, MemoryError> {
        let mut g = self.client.lock().map_err(|_| MemoryError::LockPoisoned)?;
        if let Some(c) = g.as_ref() {
            return Ok(Arc::clone(c));
        }
        let (adresse, justificatif) = (&self.adresse, &self.justificatif);
        let c = run_blocking(|| KindMotherClient::connect(adresse, justificatif, BASE_KINDMOTHER))?
            .map_err(|e| err_client(&e))?;
        let c = Arc::new(c);
        *g = Some(Arc::clone(&c));
        Ok(c)
    }

    /// Abandonne la connexion `c` après une erreur de transport (rouverte au prochain appel).
    fn abandonner(&self, c: &Arc<KindMotherClient>, cause: &ClientError) {
        if let Ok(mut g) = self.client.lock() {
            if g.as_ref().is_some_and(|x| Arc::ptr_eq(x, c)) {
                tracing::warn!(base = BASE_KINDMOTHER, %cause, "connexion KindMother perdue : elle sera rouverte");
                *g = None;
            }
        }
    }

    /// Opération idempotente : sur erreur de transport, reconnexion puis **un** rejeu.
    fn idempotente<T, F, Fut>(&self, op: F) -> Result<T, MemoryError>
    where
        F: Fn(Arc<KindMotherClient>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, ClientError>>,
        T: Send,
    {
        let c = self.client()?;
        let c2 = Arc::clone(&c);
        match run_blocking(|| op(c2))? {
            Err(e) if est_transport(&e) => {
                self.abandonner(&c, &e);
                let c = self.client()?;
                let c2 = Arc::clone(&c);
                match run_blocking(|| op(c2))? {
                    Err(e) if est_transport(&e) => {
                        self.abandonner(&c, &e);
                        Err(err_client(&e))
                    }
                    r => r.map_err(|e| err_client(&e)),
                }
            }
            r => r.map_err(|e| err_client(&e)),
        }
    }

    /// Lecture typée.
    pub(crate) fn select(&self, q: TypedSelect) -> Result<Vec<Ligne>, MemoryError> {
        self.idempotente(|c| {
            let q = q.clone();
            async move { c.select(q).await }
        })
    }

    /// Lot atomique d'intentions typées. Jamais rejoué à l'aveugle : une
    /// coupure pendant l'envoi donne une erreur explicite (issue inconnue).
    pub(crate) fn soumettre(
        &self,
        mut intents: Vec<TypedWriteIntent>,
    ) -> Result<Vec<u64>, MemoryError> {
        for i in &mut intents {
            if i.intent_id.is_empty() {
                i.intent_id = Uuid::new_v4().to_string();
            }
        }
        let c = self.client()?;
        let c2 = Arc::clone(&c);
        match run_blocking(|| async move { c2.submit_batch(intents).await })? {
            Ok(issues) => Ok(issues.iter().map(|o| o.rows_affected).collect()),
            Err(e) if est_transport(&e) => {
                self.abandonner(&c, &e);
                Err(MemoryError::KindMotherUnavailable(format!(
                    "écriture interrompue ({e}) : issue inconnue, non rejouée"
                )))
            }
            Err(e) => Err(err_client(&e)),
        }
    }

    /// Migrations, étiquette locale `nationale`, sensibilité relue, chiffrement.
    fn preparer(&self) -> Result<(), MemoryError> {
        self.idempotente(|c| async move { c.apply_migrations(migrations()).await })?;

        let meta = self.select(TypedSelect::from(META).filter("cle", CLE_ETIQUETTE))?;
        match meta
            .first()
            .map(|l| l.get("valeur").and_then(Value::as_str))
        {
            None => {
                self.soumettre(vec![TypedWriteIntent::insert(META)
                    .value("cle", CLE_ETIQUETTE)
                    .value("valeur", ETIQUETTE)
                    .purpose("alicia_memoire_etiquette")])?;
            }
            Some(Some(v)) if v == ETIQUETTE => {}
            Some(autre) => {
                return Err(MemoryError::NotNational(format!(
                    "étiquette locale {:?} (attendue {ETIQUETTE})",
                    autre.unwrap_or_default()
                )));
            }
        }

        let donnee = RefDonnee::new(BASE_KINDMOTHER, META, "valeur").cle("cle", CLE_ETIQUETTE);
        let etat = self.idempotente(|c| {
            let d = donnee.clone();
            async move { c.lire_etiquette(d).await }
        })?;
        if etat.sensibilite != Sensibilite::Nationale || etat.publiable {
            return Err(MemoryError::NotNational(format!(
                "KindMother classe {BASE_KINDMOTHER} en {:?}",
                etat.sensibilite
            )));
        }

        let info = self.idempotente(|c| async move { c.database_info().await })?;
        if !info.encrypted {
            return Err(MemoryError::NotEncrypted);
        }
        Ok(())
    }

    /// Import unique de l'ancienne base SQLite (voir [`crate::heritage`]).
    #[cfg(feature = "import-heritage")]
    pub(crate) fn importer(
        &self,
        chemin: &Path,
    ) -> Result<kindmother_heritage::RapportImport, MemoryError> {
        let c = self.client()?;
        let plan = crate::heritage::plan_import();
        run_blocking(|| async { kindmother_heritage::importer(chemin, &c, &plan).await })?.map_err(
            |e| match e {
                kindmother_heritage::ErreurImport::KindMother(ce) if est_transport(&ce) => {
                    self.abandonner(&c, &ce);
                    MemoryError::KindMotherUnavailable(format!("import interrompu : {ce}"))
                }
                autre => MemoryError::LegacyDatabase(autre.to_string()),
            },
        )
    }
}

/// Vrai si `chemin` est une ancienne base non vide sans marqueur d'import.
pub(crate) fn base_heritee_a_importer(chemin: &Path) -> bool {
    let non_vide = std::fs::metadata(chemin).is_ok_and(|m| m.is_file() && m.len() > 0);
    non_vide && !kindmother_client::heritage::chemin_marqueur_import(chemin).exists()
}
