//! Store de la mémoire d'Alicia, écrit et lu par **KindMother** (K-13).
//!
//! Tables (base KindMother `maia-_alicia`, voir [`crate::schema`]) :
//! - `facts` — vue courante des faits ; `fact_versions` — historique
//!   append-only (EX-52) ; `fact_tombstones` — oublis demandés (EX-56)
//! - `preferences(scope, key)` ; `episodes` ; `personas` ; `learned_skills` ;
//!   `pending_confirmations` ; `meta` (étiquette `nationale`)
//!
//! Toutes les écritures sont des **intentions typées** en lots atomiques ;
//! les lectures sont typées (égalités, tri, limite). La recherche plein texte
//! (ancien FTS5) est faite par le store sur les lignes lues
//! ([`crate::recherche`]).
//!
//! L'API reste **synchrone** (pont vers le client asynchrone de KindMother).
//! [`MemoryStore::open_in_memory`] sert les essais : mêmes intentions, tables
//! en mémoire vive, rien sur disque.
//!
//! Concurrence : les transitions de confirmation sont des mises à jour
//! conditionnelles (`clé = {id, statut}`) appliquées atomiquement par
//! KindMother — une seule réclamation gagne. Les écritures composées (lecture
//! puis écriture : version d'un fait, oubli, upsert qui préserve `created_at`)
//! sont sérialisées dans le processus ; l'opérateur `maia-_alicia` n'appartient
//! qu'au runtime d'Alicia.

// @id service.alicia.memory.store
// @role memory_store
// @layer 7
// @do persist_alicia_memory
// @human Store de la mémoire d'Alicia (faits, préférences, épisodes, personas) par KindMother.

use std::collections::HashSet;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use chrono::{DateTime, Utc};
use kindmother_client::{TypedSelect, TypedWriteIntent};
use serde_json::Value;
use uuid::Uuid;

use crate::entrepot::{Entrepot, Ligne, MAX_LOT};
use crate::errors::MemoryError;
use crate::kindmother::{base_heritee_a_importer, ConfigKindMother, EntrepotKindMother};
use crate::recherche::Requete;
use crate::schema::{
    EPISODES, FACTS, FACT_TOMBSTONES, FACT_VERSIONS, LEARNED_SKILLS, PENDING, PERSONAS, PREFERENCES,
};
use crate::types::{
    Episode, EpisodeOutcome, Fact, FactTombstone, FactVersion, LearnedSkill, LearnedSkillStatus,
    PendingConfirmation, PendingExpiryReport, PendingStatus, Persona, Preference,
    PENDING_REASON_EXPIRED,
};
use crate::vif::EntrepotVif;

/// Store mémoire — intentions typées vers KindMother (ou mémoire vive).
///
/// `Send + Sync` : partageable en `Arc<MemoryStore>` entre handlers axum,
/// tâches tokio et le runtime Alicia.
pub struct MemoryStore {
    entrepot: Entrepot,
    /// Sérialise les écritures composées (lecture puis lot) du processus.
    ecriture: Mutex<()>,
}

impl MemoryStore {
    fn avec(entrepot: Entrepot) -> Self {
        Self {
            entrepot,
            ecriture: Mutex::new(()),
        }
    }

    fn verrou(&self) -> Result<MutexGuard<'_, ()>, MemoryError> {
        // Fail-closed (C-12) : un verrou empoisonné refuse l'opération.
        self.ecriture.lock().map_err(|_| MemoryError::LockPoisoned)
    }

    /// Ouvre la mémoire servie par KindMother (adresse `KINDMOTHER_ADDR`,
    /// défaut `127.0.0.1:50051`, justificatif `maia-_alicia` à l'emplacement
    /// standard).
    ///
    /// `path` désigne l'**ancienne base SQLite** de la mémoire : si elle
    /// existe (non vide) et n'a pas encore été importée, elle est importée une
    /// fois (feature `import-heritage`) ; sans cette feature, l'ouverture est
    /// refusée tant qu'elle n'est pas importée. Le fichier n'est jamais créé,
    /// modifié ni supprimé. Voir [`MemoryStore::connecter`].
    pub fn open(path: &Path) -> Result<Self, MemoryError> {
        Self::connecter(&ConfigKindMother::depuis_env().avec_base_heritee(path))
    }

    /// Ouvre la mémoire servie par KindMother selon `cfg`.
    ///
    /// Fail-closed : KindMother injoignable ⇒ [`MemoryError::KindMotherUnavailable`] ;
    /// justificatif absent, authentification refusée, base non `nationale` ou
    /// non chiffrée ⇒ refus. Aucun repli sur un fichier local.
    pub fn connecter(cfg: &ConfigKindMother) -> Result<Self, MemoryError> {
        let km = EntrepotKindMother::connecter(cfg)?;
        let store = Self::avec(Entrepot::KindMother(Box::new(km)));
        if let Some(chemin) = &cfg.base_heritee {
            store.reprendre_base_heritee(chemin)?;
        }
        store.completer_historique()?;
        Ok(store)
    }

    /// Ouvre un store en mémoire vive (tests, développement) : rien sur disque.
    pub fn open_in_memory() -> Result<Self, MemoryError> {
        Ok(Self::avec(Entrepot::Vif(EntrepotVif::new())))
    }

    /// Ancienne base non importée : import (feature `import-heritage`) ou refus.
    fn reprendre_base_heritee(&self, chemin: &Path) -> Result<(), MemoryError> {
        if !base_heritee_a_importer(chemin) {
            return Ok(());
        }
        #[cfg(feature = "import-heritage")]
        {
            let rapport = self.importer_base_heritee(chemin)?;
            tracing::info!(
                base_heritee = %chemin.display(),
                lignes = rapport.total_attendues(),
                orphelines = rapport.total_orphelines(),
                "mémoire d'Alicia : ancienne base SQLite importée dans KindMother (fichier conservé)"
            );
            Ok(())
        }
        #[cfg(not(feature = "import-heritage"))]
        {
            Err(MemoryError::LegacyDatabase(format!(
                "{} présente et non importée dans KindMother (K-13) : ouverture refusée pour ne \
                 pas présenter une mémoire vide. Importer avec `alicia-memoire-import {}` \
                 (feature import-heritage), ou créer {} pour confirmer l'abandon volontaire \
                 (le fichier hérité n'est jamais touché).",
                chemin.display(),
                chemin.display(),
                kindmother_client::heritage::chemin_marqueur_import(chemin).display()
            )))
        }
    }

    /// Importe **une fois** l'ancienne base SQLite `chemin` dans KindMother
    /// (identifiants conservés, vérification ligne à ligne, marqueur
    /// `<chemin>.importe-kindmother`). Idempotent : relancé après succès, ne
    /// fait rien ; après un échec, reprend sans doublon. Le fichier hérité
    /// n'est jamais modifié ni supprimé.
    #[cfg(feature = "import-heritage")]
    pub fn importer_base_heritee(
        &self,
        chemin: &Path,
    ) -> Result<kindmother_heritage::RapportImport, MemoryError> {
        let rapport =
            {
                let _g = self.verrou()?;
                match &self.entrepot {
                    Entrepot::KindMother(k) => k.importer(chemin)?,
                    Entrepot::Vif(_) => return Err(MemoryError::LegacyDatabase(
                        "mémoire vive : import d'une ancienne base impossible (KindMother requis)"
                            .to_string(),
                    )),
                }
            };
        self.completer_historique()?;
        Ok(rapport)
    }

    /// EX-52 : chaque fait sans historique (base antérieure au versionnage)
    /// reçoit une version 1 égale à son contenu courant. Idempotent.
    fn completer_historique(&self) -> Result<(), MemoryError> {
        let _g = self.verrou()?;
        let faits = self.entrepot.toutes(FACTS, &[("id", false)], |q| q)?;
        if faits.is_empty() {
            return Ok(());
        }
        let avec_historique: HashSet<String> = self
            .entrepot
            .toutes(
                FACT_VERSIONS,
                &[("fact_id", false), ("version", false)],
                |q| q.columns(["fact_id", "version"]),
            )?
            .iter()
            .filter_map(|l| l.get("fact_id").and_then(Value::as_str).map(str::to_string))
            .collect();
        let mut intents = Vec::new();
        for l in &faits {
            let f = fact_de(l)?;
            if !avec_historique.contains(&f.id.to_string()) {
                intents.push(intention_version(&f, 1, f.updated_at));
            }
        }
        let n = intents.len();
        for lot in intents.chunks(MAX_LOT) {
            self.entrepot.soumettre(lot.to_vec())?;
        }
        if n > 0 {
            tracing::info!(
                faits = n,
                "mémoire : version initiale créée pour les faits existants"
            );
        }
        Ok(())
    }

    // ------------------------------------------------------------------
    // FACTS
    // ------------------------------------------------------------------

    /// Enregistre un fait, ou en crée une **nouvelle version** s'il existe
    /// déjà (EX-52). Identifiant stable par `id`.
    ///
    /// - La version précédente reste dans l'historique (`fact_history`) ;
    ///   aucune écriture destructive.
    /// - Réécrire un fait identique (persona, sujet, contenu, source,
    ///   confiance) ne crée pas de version (idempotent).
    /// - Un fait oublié (pierre tombale) ne peut pas être réécrit sous le même
    ///   identifiant : [`MemoryError::FactForgotten`].
    ///
    /// Version et vue courante sont écrites dans un seul lot atomique.
    pub fn remember(&self, fact: &Fact) -> Result<(), MemoryError> {
        let id = fact.id.to_string();
        let _g = self.verrou()?;
        if !self
            .entrepot
            .select(
                TypedSelect::from(FACT_TOMBSTONES)
                    .filter("fact_id", id.as_str())
                    .limit(1),
            )?
            .is_empty()
        {
            return Err(MemoryError::FactForgotten(fact.id));
        }
        let actuel = self
            .entrepot
            .select(TypedSelect::from(FACTS).filter("id", id.as_str()))?
            .first()
            .map(EtatFait::de)
            .transpose()?;
        if actuel.as_ref().is_some_and(|a| a.identique(fact)) {
            // Rien de nouveau : pas de version supplémentaire.
            return Ok(());
        }
        let derniere = self
            .entrepot
            .select(
                TypedSelect::from(FACT_VERSIONS)
                    .columns(["version"])
                    .filter("fact_id", id.as_str())
                    .order_by("version", true)
                    .limit(1),
            )?
            .first()
            .map(|l| entier(l, "version"))
            .transpose()?
            .unwrap_or(0);
        let version = u32::try_from(derniere.saturating_add(1)).map_err(|_| {
            MemoryError::Corrupt(format!("fait {id} : version {derniere} hors bornes"))
        })?;

        let mut intents = vec![intention_version(fact, version, fact.updated_at)];
        intents.push(if actuel.is_some() {
            TypedWriteIntent::update(FACTS)
                .key("id", id.as_str())
                .value("persona", fact.persona.as_str())
                .value("subject", fact.subject.as_str())
                .value("content", fact.content.as_str())
                .value("source", fact.source.as_str())
                .value("confidence", f64::from(fact.confidence))
                .value("updated_at", fact.updated_at.to_rfc3339())
                .purpose("alicia_memoire_fait_maj")
        } else {
            TypedWriteIntent::insert(FACTS)
                .value("id", id.as_str())
                .value("persona", fact.persona.as_str())
                .value("subject", fact.subject.as_str())
                .value("content", fact.content.as_str())
                .value("source", fact.source.as_str())
                .value("confidence", f64::from(fact.confidence))
                .value("created_at", fact.created_at.to_rfc3339())
                .value("updated_at", fact.updated_at.to_rfc3339())
                .purpose("alicia_memoire_fait")
        });
        self.entrepot.soumettre(intents)?;
        Ok(())
    }

    /// **Oubli demandé** d'un fait (EX-52, EX-56) : seule écriture destructive
    /// autorisée sur la mémoire.
    ///
    /// Efface réellement la vue courante et **toutes** les versions, puis
    /// enregistre une pierre tombale sans contenu (qui, quand, pourquoi,
    /// nombre de versions effacées), dans un seul lot atomique.
    /// `forgotten_by` et `reason` sont obligatoires (non vides, bornés).
    ///
    /// Renvoie le nombre de faits retirés de la vue courante (0 ou 1). Un fait
    /// inconnu ou déjà oublié renvoie 0 sans nouvelle pierre tombale.
    pub fn forget(&self, id: Uuid, forgotten_by: &str, reason: &str) -> Result<usize, MemoryError> {
        valider_champ_oubli("auteur", forgotten_by, OUBLI_AUTEUR_MAX)?;
        valider_champ_oubli("motif", reason, OUBLI_MOTIF_MAX)?;
        let id_s = id.to_string();
        let _g = self.verrou()?;
        let actuel = !self
            .entrepot
            .select(
                TypedSelect::from(FACTS)
                    .columns(["id"])
                    .filter("id", id_s.as_str()),
            )?
            .is_empty();
        let versions = self
            .entrepot
            .toutes(FACT_VERSIONS, &[("version", false)], |q| {
                q.columns(["version"]).filter("fact_id", id_s.as_str())
            })?
            .len();
        if !actuel && versions == 0 {
            return Ok(0);
        }
        let deja_tombe = !self
            .entrepot
            .select(
                TypedSelect::from(FACT_TOMBSTONES)
                    .filter("fact_id", id_s.as_str())
                    .limit(1),
            )?
            .is_empty();

        let mut intents = Vec::new();
        if actuel {
            intents.push(
                TypedWriteIntent::delete(FACTS)
                    .key("id", id_s.as_str())
                    .purpose("alicia_memoire_oubli_demande"),
            );
        }
        intents.push(
            TypedWriteIntent::delete(FACT_VERSIONS)
                .key("fact_id", id_s.as_str())
                .purpose("alicia_memoire_oubli_demande"),
        );
        if !deja_tombe {
            intents.push(
                TypedWriteIntent::insert(FACT_TOMBSTONES)
                    .value("fact_id", id_s.as_str())
                    .value("forgotten_at", Utc::now().to_rfc3339())
                    .value("forgotten_by", forgotten_by.trim())
                    .value("reason", reason.trim())
                    .value(
                        "versions_erased",
                        i64::try_from(versions).unwrap_or(i64::MAX),
                    )
                    .purpose("alicia_memoire_pierre_tombale"),
            );
        }
        let lignes = self.entrepot.soumettre(intents)?;
        let retires = if actuel {
            usize::try_from(lignes.first().copied().unwrap_or(0)).unwrap_or(usize::MAX)
        } else {
            0
        };
        tracing::info!(fait = %id, versions, "mémoire : fait oublié à la demande (pierre tombale)");
        Ok(retires)
    }

    /// Version courante d'un fait (None si inconnu ou oublié).
    pub fn fact(&self, id: Uuid) -> Result<Option<Fact>, MemoryError> {
        self.entrepot
            .select(TypedSelect::from(FACTS).filter("id", id.to_string()))?
            .first()
            .map(fact_de)
            .transpose()
    }

    /// Historique complet d'un fait, de la version 1 à la version courante
    /// (ordre croissant). Vide si inconnu ou oublié.
    pub fn fact_history(&self, id: Uuid) -> Result<Vec<FactVersion>, MemoryError> {
        self.entrepot
            .toutes(FACT_VERSIONS, &[("version", false)], |q| {
                q.filter("fact_id", id.to_string())
            })?
            .iter()
            .map(version_de)
            .collect()
    }

    /// Pierre tombale d'un fait oublié (None si jamais oublié).
    pub fn fact_tombstone(&self, id: Uuid) -> Result<Option<FactTombstone>, MemoryError> {
        self.entrepot
            .select(TypedSelect::from(FACT_TOMBSTONES).filter("fact_id", id.to_string()))?
            .first()
            .map(tombe_de)
            .transpose()
    }

    /// Liste les faits d'un persona (triés par `updated_at` décroissant).
    pub fn facts_for_persona(&self, persona: &str) -> Result<Vec<Fact>, MemoryError> {
        self.entrepot
            .toutes(FACTS, &[("updated_at", true), ("id", false)], |q| {
                q.filter("persona", persona)
            })?
            .iter()
            .map(fact_de)
            .collect()
    }

    /// Recherche plein texte sur les faits — retourne au plus `limit` résultats,
    /// les plus pertinents d'abord (voir [`crate::recherche`]).
    pub fn recall(&self, query: &str, limit: usize) -> Result<Vec<Fact>, MemoryError> {
        let requete = Requete::analyser(query);
        if requete.est_vide() || limit == 0 {
            return Ok(Vec::new());
        }
        let mut trouves: Vec<(f64, Fact)> = Vec::new();
        for l in self.entrepot.toutes(FACTS, &[("id", false)], |q| q)? {
            let f = fact_de(&l)?;
            if let Some(s) = requete.score(&f.content) {
                trouves.push((s, f));
            }
        }
        trouves.sort_by(|a, b| {
            b.0.total_cmp(&a.0)
                .then_with(|| b.1.updated_at.cmp(&a.1.updated_at))
        });
        Ok(trouves.into_iter().take(limit).map(|(_, f)| f).collect())
    }

    // ------------------------------------------------------------------
    // PREFERENCES
    // ------------------------------------------------------------------

    /// Définit (ou met à jour) une préférence.
    pub fn prefer(&self, pref: &Preference) -> Result<(), MemoryError> {
        let value_json = serde_json::to_string(&pref.value)?;
        self.entrepot
            .soumettre(vec![TypedWriteIntent::upsert(PREFERENCES)
                .key("scope", pref.scope.as_str())
                .key("key", pref.key.as_str())
                .value("value_json", value_json)
                .value("updated_at", pref.updated_at.to_rfc3339())
                .purpose("alicia_memoire_preference")])?;
        Ok(())
    }

    /// Récupère une préférence (None si absente).
    pub fn preference(&self, scope: &str, key: &str) -> Result<Option<Preference>, MemoryError> {
        self.entrepot
            .select(
                TypedSelect::from(PREFERENCES)
                    .filter("scope", scope)
                    .filter("key", key),
            )?
            .first()
            .map(preference_de)
            .transpose()
    }

    /// Liste toutes les préférences d'un scope (persona ou `"global"`).
    pub fn preferences_for_scope(&self, scope: &str) -> Result<Vec<Preference>, MemoryError> {
        self.entrepot
            .toutes(PREFERENCES, &[("key", false)], |q| q.filter("scope", scope))?
            .iter()
            .map(preference_de)
            .collect()
    }

    /// Supprime une préférence. Renvoie le nombre de lignes supprimées.
    pub fn unprefer(&self, scope: &str, key: &str) -> Result<usize, MemoryError> {
        let n = self
            .entrepot
            .soumettre(vec![TypedWriteIntent::delete(PREFERENCES)
                .key("scope", scope)
                .key("key", key)
                .purpose("alicia_memoire_preference_retrait")])?;
        Ok(usize::try_from(n.first().copied().unwrap_or(0)).unwrap_or(usize::MAX))
    }

    // ------------------------------------------------------------------
    // EPISODES
    // ------------------------------------------------------------------

    /// Enregistre un épisode (insertion uniquement — pas d'upsert).
    pub fn record_episode(&self, ep: &Episode) -> Result<(), MemoryError> {
        let params_json = serde_json::to_string(&ep.params)?;
        self.entrepot
            .soumettre(vec![TypedWriteIntent::insert(EPISODES)
                .value("id", ep.id.to_string())
                .value("persona", ep.persona.as_str())
                .value("service_id", ep.service_id.as_str())
                .value("intent", ep.intent.as_str())
                .value("user_phrase", ep.user_phrase.as_str())
                .value("params_json", params_json)
                .value("outcome", outcome_as_str(ep.outcome))
                .value("error", ep.error.clone())
                .value("created_at", ep.created_at.to_rfc3339())
                .purpose("alicia_memoire_episode")])?;
        Ok(())
    }

    /// Récupère les `limit` derniers épisodes (tous personas confondus).
    pub fn recent_episodes(&self, limit: usize) -> Result<Vec<Episode>, MemoryError> {
        self.entrepot
            .select(
                TypedSelect::from(EPISODES)
                    .order_by("created_at", true)
                    .limit(limite(limit)),
            )?
            .iter()
            .map(episode_de)
            .collect()
    }

    /// Épisodes récents restreints à une liste de personas (filtre côté
    /// KindMother, avant la limite). Liste vide → aucun épisode. Au plus 64
    /// personas.
    pub fn recent_episodes_for_personas(
        &self,
        personas: &[String],
        limit: usize,
    ) -> Result<Vec<Episode>, MemoryError> {
        let mut vus = HashSet::new();
        let personas: Vec<&String> = personas
            .iter()
            .take(64)
            .filter(|p| vus.insert(p.as_str()))
            .collect();
        if personas.is_empty() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for p in personas {
            for l in self.entrepot.select(
                TypedSelect::from(EPISODES)
                    .filter("persona", p.as_str())
                    .order_by("created_at", true)
                    .limit(limite(limit)),
            )? {
                out.push(episode_de(&l)?);
            }
        }
        out.sort_by_key(|e: &Episode| std::cmp::Reverse(e.created_at));
        out.truncate(limit);
        Ok(out)
    }

    /// Liste les épisodes d'un service donné, plus récents d'abord.
    pub fn episodes_for_service(
        &self,
        service_id: &str,
        limit: usize,
    ) -> Result<Vec<Episode>, MemoryError> {
        self.entrepot
            .select(
                TypedSelect::from(EPISODES)
                    .filter("service_id", service_id)
                    .order_by("created_at", true)
                    .limit(limite(limit)),
            )?
            .iter()
            .map(episode_de)
            .collect()
    }

    /// Recherche plein texte sur les phrases utilisateur des épisodes.
    pub fn search_episodes(&self, query: &str, limit: usize) -> Result<Vec<Episode>, MemoryError> {
        let requete = Requete::analyser(query);
        if requete.est_vide() || limit == 0 {
            return Ok(Vec::new());
        }
        let mut trouves: Vec<(f64, Episode)> = Vec::new();
        for l in self.entrepot.toutes(EPISODES, &[("id", false)], |q| q)? {
            let e = episode_de(&l)?;
            if let Some(s) = requete.score(&e.user_phrase) {
                trouves.push((s, e));
            }
        }
        trouves.sort_by(|a, b| {
            b.0.total_cmp(&a.0)
                .then_with(|| b.1.created_at.cmp(&a.1.created_at))
        });
        Ok(trouves.into_iter().take(limit).map(|(_, e)| e).collect())
    }

    // ------------------------------------------------------------------
    // PERSONAS
    // ------------------------------------------------------------------

    /// Crée ou met à jour un persona (la date de création d'origine est conservée).
    pub fn upsert_persona(&self, persona: &Persona) -> Result<(), MemoryError> {
        if persona.id.trim().is_empty() {
            return Err(MemoryError::InvalidId(persona.id.clone()));
        }
        let _g = self.verrou()?;
        let existe = !self
            .entrepot
            .select(
                TypedSelect::from(PERSONAS)
                    .columns(["id"])
                    .filter("id", persona.id.as_str()),
            )?
            .is_empty();
        let intent = if existe {
            TypedWriteIntent::update(PERSONAS).key("id", persona.id.as_str())
        } else {
            TypedWriteIntent::insert(PERSONAS)
                .value("id", persona.id.as_str())
                .value("created_at", persona.created_at.to_rfc3339())
        };
        self.entrepot.soumettre(vec![intent
            .value("display_name", persona.display_name.as_str())
            .value("description", persona.description.as_str())
            .value("is_default", i64::from(persona.is_default))
            .purpose("alicia_memoire_persona")])?;
        Ok(())
    }

    /// Récupère un persona par son `id`.
    pub fn persona(&self, id: &str) -> Result<Option<Persona>, MemoryError> {
        self.entrepot
            .select(TypedSelect::from(PERSONAS).filter("id", id))?
            .first()
            .map(persona_de)
            .transpose()
    }

    /// Liste tous les personas (triés par `id`).
    pub fn personas(&self) -> Result<Vec<Persona>, MemoryError> {
        self.entrepot
            .toutes(PERSONAS, &[("id", false)], |q| q)?
            .iter()
            .map(persona_de)
            .collect()
    }

    /// Persona marqué `is_default = 1` (s'il y en a un ; le plus ancien sinon).
    pub fn default_persona(&self) -> Result<Option<Persona>, MemoryError> {
        self.entrepot
            .select(
                TypedSelect::from(PERSONAS)
                    .filter("is_default", 1_i64)
                    .order_by("created_at", false)
                    .order_by("id", false)
                    .limit(1),
            )?
            .first()
            .map(persona_de)
            .transpose()
    }

    // ------------------------------------------------------------------
    // PERSONA ACTIVE (runtime)
    // ------------------------------------------------------------------

    /// Identifiant de la persona active.
    ///
    /// Résolution :
    /// 1. `Preference` scope=`global`, key=`active_persona` (string)
    /// 2. sinon le `default_persona()`
    /// 3. sinon `None`
    pub fn active_persona_id(&self) -> Result<Option<String>, MemoryError> {
        if let Some(pref) = self.preference("global", "active_persona")? {
            if let Value::String(s) = &pref.value {
                if !s.trim().is_empty() {
                    return Ok(Some(s.clone()));
                }
            }
        }
        Ok(self.default_persona()?.map(|p| p.id))
    }

    /// Pose la persona active. Vérifie qu'elle existe — sinon `UnknownPersona`.
    pub fn set_active_persona(&self, persona_id: &str) -> Result<(), MemoryError> {
        if persona_id.trim().is_empty() {
            return Err(MemoryError::InvalidId(persona_id.into()));
        }
        if self.persona(persona_id)?.is_none() {
            return Err(MemoryError::UnknownPersona(persona_id.into()));
        }
        self.prefer(&Preference::new(
            "global".into(),
            "active_persona".into(),
            Value::String(persona_id.into()),
        ))
    }

    // ------------------------------------------------------------------
    // PRÉFÉRENCE DE CONFIRMATION (par capability)
    // ------------------------------------------------------------------

    /// Lit la préférence de confirmation pour une capability.
    ///
    /// Convention de stockage :
    /// - clé : `confirm.<qualified_id>` (ex: `confirm.jaykoa.schedule_event`)
    /// - valeur JSON : `true` (toujours demander) / `false` (ne jamais demander)
    ///
    /// Résolution avec fallback :
    /// 1. scope = `persona`
    /// 2. scope = `global`
    /// 3. `None` (= aucune préférence posée → caller applique son défaut)
    pub fn confirmation_preference(
        &self,
        persona: &str,
        qualified_id: &str,
    ) -> Result<Option<bool>, MemoryError> {
        let key = format!("confirm.{qualified_id}");
        for scope in [persona, "global"] {
            if let Some(pref) = self.preference(scope, &key)? {
                if let Value::Bool(b) = pref.value {
                    return Ok(Some(b));
                }
                tracing::warn!(
                    persona = %persona,
                    qualified_id = %qualified_id,
                    scope = %scope,
                    "préférence confirmation non booléenne, ignorée"
                );
            }
        }
        Ok(None)
    }

    /// Pose la préférence de confirmation pour une capability sur un persona
    /// donné (ou `"global"`).
    pub fn set_confirmation_preference(
        &self,
        scope: &str,
        qualified_id: &str,
        ask: bool,
    ) -> Result<(), MemoryError> {
        if scope.trim().is_empty() || qualified_id.trim().is_empty() {
            return Err(MemoryError::InvalidId(format!(
                "scope='{scope}' qualified_id='{qualified_id}'"
            )));
        }
        self.prefer(&Preference::new(
            scope.into(),
            format!("confirm.{qualified_id}"),
            Value::Bool(ask),
        ))
    }

    // ------------------------------------------------------------------
    // SKILLS APPRISES (P5)
    // ------------------------------------------------------------------

    /// Insère ou met à jour une `LearnedSkill` (clé = `id` ; la date de
    /// création d'origine est conservée).
    pub fn upsert_learned_skill(&self, skill: &LearnedSkill) -> Result<(), MemoryError> {
        let params_json = serde_json::to_string(&skill.params_template)?;
        let id = skill.id.to_string();
        let _g = self.verrou()?;
        let existe = !self
            .entrepot
            .select(
                TypedSelect::from(LEARNED_SKILLS)
                    .columns(["id"])
                    .filter("id", id.as_str()),
            )?
            .is_empty();
        let intent = if existe {
            TypedWriteIntent::update(LEARNED_SKILLS).key("id", id.as_str())
        } else {
            TypedWriteIntent::insert(LEARNED_SKILLS)
                .value("id", id.as_str())
                .value("created_at", skill.created_at.to_rfc3339())
        };
        self.entrepot.soumettre(vec![intent
            .value("persona", skill.persona.as_str())
            .value("trigger_phrase", skill.trigger_phrase.as_str())
            .value("qualified_id", skill.qualified_id.as_str())
            .value("params_template", params_json)
            .value("status", learned_status_as_str(skill.status))
            .value("accepted_at", skill.accepted_at.map(|d| d.to_rfc3339()))
            .purpose("alicia_memoire_skill")])?;
        Ok(())
    }

    /// Propose une nouvelle skill apprise (raccourci pour les détecteurs).
    pub fn propose_learned_skill(
        &self,
        persona: &str,
        trigger_phrase: &str,
        qualified_id: &str,
        params_template: Value,
    ) -> Result<LearnedSkill, MemoryError> {
        if persona.trim().is_empty()
            || trigger_phrase.trim().is_empty()
            || qualified_id.trim().is_empty()
        {
            return Err(MemoryError::InvalidId(format!(
                "persona='{persona}' qualified_id='{qualified_id}' trigger='{trigger_phrase}'"
            )));
        }
        let skill = LearnedSkill::proposed(
            persona.to_string(),
            trigger_phrase.to_string(),
            qualified_id.to_string(),
            params_template,
        );
        self.upsert_learned_skill(&skill)?;
        Ok(skill)
    }

    /// Marque une skill comme acceptée (statut `Accepted`, `accepted_at` posé).
    pub fn accept_learned_skill(&self, id: Uuid) -> Result<(), MemoryError> {
        let n = self
            .entrepot
            .soumettre(vec![TypedWriteIntent::update(LEARNED_SKILLS)
                .key("id", id.to_string())
                .value(
                    "status",
                    learned_status_as_str(LearnedSkillStatus::Accepted),
                )
                .value("accepted_at", Utc::now().to_rfc3339())
                .purpose("alicia_memoire_skill_acceptee")])?;
        if n.first().copied().unwrap_or(0) == 0 {
            return Err(MemoryError::LearnedSkillNotFound(id));
        }
        Ok(())
    }

    /// Marque une skill comme refusée (statut `Rejected`).
    pub fn reject_learned_skill(&self, id: Uuid) -> Result<(), MemoryError> {
        let n = self
            .entrepot
            .soumettre(vec![TypedWriteIntent::update(LEARNED_SKILLS)
                .key("id", id.to_string())
                .value(
                    "status",
                    learned_status_as_str(LearnedSkillStatus::Rejected),
                )
                .purpose("alicia_memoire_skill_refusee")])?;
        if n.first().copied().unwrap_or(0) == 0 {
            return Err(MemoryError::LearnedSkillNotFound(id));
        }
        Ok(())
    }

    /// Récupère une skill apprise par son id.
    pub fn learned_skill(&self, id: Uuid) -> Result<Option<LearnedSkill>, MemoryError> {
        self.entrepot
            .select(TypedSelect::from(LEARNED_SKILLS).filter("id", id.to_string()))?
            .first()
            .map(skill_de)
            .transpose()
    }

    fn skills_par_statut(
        &self,
        persona: &str,
        statut: LearnedSkillStatus,
    ) -> Result<Vec<LearnedSkill>, MemoryError> {
        self.entrepot
            .toutes(
                LEARNED_SKILLS,
                &[("created_at", true), ("id", false)],
                |q| {
                    q.filter("persona", persona)
                        .filter("status", learned_status_as_str(statut))
                },
            )?
            .iter()
            .map(skill_de)
            .collect()
    }

    /// Liste les skills apprises **acceptées** pour une persona donnée
    /// (utilisable au runtime pour matcher des phrases).
    pub fn active_learned_skills(&self, persona: &str) -> Result<Vec<LearnedSkill>, MemoryError> {
        self.skills_par_statut(persona, LearnedSkillStatus::Accepted)
    }

    /// Liste les skills apprises **proposées** pour une persona — c'est ce
    /// qu'on pousse dans le dialogue de promotion.
    pub fn pending_learned_skills(&self, persona: &str) -> Result<Vec<LearnedSkill>, MemoryError> {
        self.skills_par_statut(persona, LearnedSkillStatus::Proposed)
    }

    /// `true` si une skill apprise existe déjà (proposée OU rejetée OU
    /// acceptée) pour ce `(persona, qualified_id)`. Sert au détecteur de
    /// patterns pour ne pas re-proposer en boucle.
    pub fn learned_skill_exists(
        &self,
        persona: &str,
        qualified_id: &str,
    ) -> Result<bool, MemoryError> {
        Ok(!self
            .entrepot
            .select(
                TypedSelect::from(LEARNED_SKILLS)
                    .columns(["id"])
                    .filter("persona", persona)
                    .filter("qualified_id", qualified_id)
                    .limit(1),
            )?
            .is_empty())
    }

    /// Cherche, parmi les `LearnedSkill` **acceptées** pour la persona, une
    /// dont le `trigger_phrase` matche la phrase utilisateur.
    ///
    /// Matching P5 (volontairement simple) :
    /// 1. Normalisation : lowercase + suppression de la ponctuation +
    ///    compactage des whitespaces
    /// 2. Match si phrase normalisée == trigger normalisé **OU** phrase
    ///    contient trigger (utile si l'utilisateur ajoute des mots autour)
    /// 3. En cas d'ambiguïté (plusieurs matches) : la skill **la plus
    ///    récemment créée** gagne — `active_learned_skills` renvoie déjà
    ///    triées DESC par `created_at`.
    ///
    /// Renvoie `None` si la phrase est vide après normalisation, ou si rien
    /// ne matche.
    pub fn match_learned_skill(
        &self,
        persona: &str,
        phrase: &str,
    ) -> Result<Option<LearnedSkill>, MemoryError> {
        let phrase_norm = normalize_phrase(phrase);
        if phrase_norm.is_empty() {
            return Ok(None);
        }
        for skill in self.active_learned_skills(persona)? {
            let trigger_norm = normalize_phrase(&skill.trigger_phrase);
            if trigger_norm.is_empty() {
                continue;
            }
            if phrase_norm == trigger_norm || phrase_norm.contains(&trigger_norm) {
                return Ok(Some(skill));
            }
        }
        Ok(None)
    }
}

/// Normalise une phrase pour le matching : lowercase, supprime la
/// ponctuation (remplacée par espace), compacte les whitespaces.
fn normalize_phrase(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Limite d'une lecture (bornée à `u32::MAX`).
fn limite(limit: usize) -> u32 {
    u32::try_from(limit).unwrap_or(u32::MAX)
}

// ----------------------------------------------------------------------
// PENDING CONFIRMATIONS (queue async)
// ----------------------------------------------------------------------

impl MemoryStore {
    /// Pose une nouvelle confirmation en attente dans la queue. Retourne la
    /// `PendingConfirmation` créée (statut `Pending`).
    pub fn enqueue_pending_confirmation(
        &self,
        qualified_id: &str,
        params: Value,
        persona: &str,
        user_phrase: &str,
        prompt: &str,
    ) -> Result<PendingConfirmation, MemoryError> {
        if qualified_id.trim().is_empty() || persona.trim().is_empty() {
            return Err(MemoryError::InvalidId(format!(
                "qualified_id='{qualified_id}' persona='{persona}'"
            )));
        }
        let pc = PendingConfirmation::new(
            qualified_id.to_string(),
            params,
            persona.to_string(),
            user_phrase.to_string(),
            prompt.to_string(),
        );
        let params_json = serde_json::to_string(&pc.params)?;
        self.entrepot
            .soumettre(vec![TypedWriteIntent::insert(PENDING)
                .value("id", pc.id.to_string())
                .value("qualified_id", pc.qualified_id.as_str())
                .value("params_json", params_json)
                .value("persona", pc.persona.as_str())
                .value("user_phrase", pc.user_phrase.as_str())
                .value("prompt", pc.prompt.as_str())
                .value("status", pending_status_as_str(pc.status))
                .value("created_at", pc.created_at.to_rfc3339())
                .value("resolved_at", pc.resolved_at.map(|d| d.to_rfc3339()))
                .purpose("alicia_memoire_confirmation")])?;
        Ok(pc)
    }

    fn pending_filtrees(
        &self,
        filtre: impl Fn(TypedSelect) -> TypedSelect,
    ) -> Result<Vec<PendingConfirmation>, MemoryError> {
        self.entrepot
            .toutes(PENDING, &[("created_at", true), ("id", false)], filtre)?
            .iter()
            .map(pending_de)
            .collect()
    }

    /// Liste toutes les confirmations actuellement `Pending` (toutes personas
    /// confondues, plus récentes d'abord).
    pub fn list_pending_confirmations(&self) -> Result<Vec<PendingConfirmation>, MemoryError> {
        self.pending_filtrees(|q| q.filter("status", pending_status_as_str(PendingStatus::Pending)))
    }

    /// Liste les confirmations `Pending` filtrées par persona.
    pub fn pending_confirmations_for_persona(
        &self,
        persona: &str,
    ) -> Result<Vec<PendingConfirmation>, MemoryError> {
        self.pending_filtrees(|q| {
            q.filter("persona", persona)
                .filter("status", pending_status_as_str(PendingStatus::Pending))
        })
    }

    /// Récupère une confirmation par id, quel que soit son statut.
    pub fn pending_confirmation(
        &self,
        id: Uuid,
    ) -> Result<Option<PendingConfirmation>, MemoryError> {
        self.entrepot
            .select(TypedSelect::from(PENDING).filter("id", id.to_string()))?
            .first()
            .map(pending_de)
            .transpose()
    }

    /// Marque une confirmation `Confirmed` (statut + resolved_at) directement
    /// depuis `Pending`, **sans exécution**. Refuse si la confirmation
    /// n'existe pas ou n'est plus `Pending`.
    ///
    /// Le chemin d'exécution réel doit passer par
    /// [`MemoryStore::claim_pending_confirmation`] puis
    /// [`MemoryStore::finalize_pending_confirmation`] (R-017).
    pub fn mark_pending_confirmed(&self, id: Uuid) -> Result<(), MemoryError> {
        self.transition_pending(id, PendingStatus::Pending, PendingStatus::Confirmed)
    }

    /// Marque une confirmation `Cancelled`. Transition atomique
    /// `Pending → Cancelled` : refuse si la confirmation n'existe pas ou n'est
    /// plus `Pending` (y compris si une exécution l'a déjà réclamée).
    pub fn mark_pending_cancelled(&self, id: Uuid) -> Result<(), MemoryError> {
        self.transition_pending(id, PendingStatus::Pending, PendingStatus::Cancelled)
    }

    /// Réclame **atomiquement** une confirmation en vue de son exécution
    /// (R-017) : mise à jour conditionnelle `clé = {id, statut = pending}`
    /// appliquée par KindMother. Seul l'appelant qui fait passer exactement
    /// une ligne obtient l'enregistrement ; tout appel concurrent reçoit
    /// `PendingConfirmationAlreadyResolved` (ou `NotFound`).
    ///
    /// Retourne l'enregistrement stocké (statut `Executing`) : c'est lui, et
    /// lui seul, qui doit être exécuté.
    pub fn claim_pending_confirmation(&self, id: Uuid) -> Result<PendingConfirmation, MemoryError> {
        let n = self
            .entrepot
            .soumettre(vec![TypedWriteIntent::update(PENDING)
                .key("id", id.to_string())
                .key("status", pending_status_as_str(PendingStatus::Pending))
                .value("status", pending_status_as_str(PendingStatus::Executing))
                .value("claimed_at", Utc::now().timestamp_millis())
                .purpose("alicia_memoire_confirmation_reclamee")])?;
        let current = self.pending_confirmation(id)?;
        match (n.first().copied().unwrap_or(0), current) {
            (1, Some(pc)) => Ok(pc),
            (_, None) => Err(MemoryError::PendingConfirmationNotFound(id)),
            (_, Some(pc)) => Err(MemoryError::PendingConfirmationAlreadyResolved {
                id,
                status: pending_status_as_str(pc.status).into(),
            }),
        }
    }

    /// Finalise une confirmation réclamée par
    /// [`MemoryStore::claim_pending_confirmation`] : `Executing → Confirmed`
    /// si `executed`, sinon `Executing → Failed`. Refuse toute autre
    /// transition.
    pub fn finalize_pending_confirmation(
        &self,
        id: Uuid,
        executed: bool,
    ) -> Result<(), MemoryError> {
        let target = if executed {
            PendingStatus::Confirmed
        } else {
            PendingStatus::Failed
        };
        self.transition_pending(id, PendingStatus::Executing, target)
    }

    /// Balaye les confirmations périmées (horloge courante). Voir
    /// [`MemoryStore::expire_stale_pending_confirmations_at`].
    pub fn expire_stale_pending_confirmations(
        &self,
        executing_ttl: std::time::Duration,
        pending_ttl: std::time::Duration,
    ) -> Result<PendingExpiryReport, MemoryError> {
        self.expire_stale_pending_confirmations_at(Utc::now(), executing_ttl, pending_ttl)
    }

    /// Balaye les confirmations périmées par rapport à l'instant `now` :
    ///
    /// - `Executing` réclamée depuis au moins `executing_ttl` (arrêt brutal
    ///   pendant l'exécution) → `Failed`, motif `"expired"`. Jamais
    ///   ré-exécutée : l'issue réelle de l'exécution interrompue est
    ///   inconnue, une reprise automatique risquerait un double effet ;
    /// - `Pending` créée depuis au moins `pending_ttl` → `Cancelled`, motif
    ///   `"expired"` : une confirmation ancienne ne peut plus être confirmée.
    ///
    /// Chaque transition est une mise à jour conditionnelle
    /// (`clé = {id, statut}`) dans un lot atomique : le statut est réévalué
    /// par KindMother, aucune course possible avec une réclamation ou une
    /// finalisation concurrente (une ligne changée entre-temps n'est pas
    /// touchée et n'apparaît pas dans le bilan).
    pub fn expire_stale_pending_confirmations_at(
        &self,
        now: DateTime<Utc>,
        executing_ttl: std::time::Duration,
        pending_ttl: std::time::Duration,
    ) -> Result<PendingExpiryReport, MemoryError> {
        let now_ms = now.timestamp_millis();
        let ttl_ms = |d: std::time::Duration| i64::try_from(d.as_millis()).unwrap_or(i64::MAX);
        let seuil_exec = now_ms.saturating_sub(ttl_ms(executing_ttl));
        let seuil_pending = now_ms.saturating_sub(ttl_ms(pending_ttl));
        let resolu = now.to_rfc3339();

        let mut candidates: Vec<(Uuid, PendingStatus, PendingStatus)> = Vec::new();
        for l in self.entrepot.toutes(PENDING, &[("id", false)], |q| {
            q.columns(["id", "claimed_at"])
                .filter("status", pending_status_as_str(PendingStatus::Executing))
        })? {
            if entier(&l, "claimed_at")? <= seuil_exec {
                candidates.push((
                    uuid_col(&l, "id")?,
                    PendingStatus::Executing,
                    PendingStatus::Failed,
                ));
            }
        }
        for l in self.entrepot.toutes(PENDING, &[("id", false)], |q| {
            q.columns(["id", "created_at"])
                .filter("status", pending_status_as_str(PendingStatus::Pending))
        })? {
            if date(&l, "created_at")?.timestamp_millis() <= seuil_pending {
                candidates.push((
                    uuid_col(&l, "id")?,
                    PendingStatus::Pending,
                    PendingStatus::Cancelled,
                ));
            }
        }

        let mut rapport = PendingExpiryReport::default();
        for lot in candidates.chunks(MAX_LOT) {
            let intents = lot
                .iter()
                .map(|(id, from, to)| {
                    TypedWriteIntent::update(PENDING)
                        .key("id", id.to_string())
                        .key("status", pending_status_as_str(*from))
                        .value("status", pending_status_as_str(*to))
                        .value("resolved_at", resolu.as_str())
                        .value("reason", PENDING_REASON_EXPIRED)
                        .purpose("alicia_memoire_confirmation_expiree")
                })
                .collect();
            let lignes = self.entrepot.soumettre(intents)?;
            for ((id, _, to), n) in lot.iter().zip(lignes) {
                if n == 1 {
                    match to {
                        PendingStatus::Failed => rapport.failed.push(*id),
                        _ => rapport.cancelled.push(*id),
                    }
                }
            }
        }
        Ok(rapport)
    }

    /// Transition atomique `from → to` (une seule mise à jour conditionnelle).
    /// Erreur typée si aucune ligne ne change.
    fn transition_pending(
        &self,
        id: Uuid,
        from: PendingStatus,
        to: PendingStatus,
    ) -> Result<(), MemoryError> {
        let n = self
            .entrepot
            .soumettre(vec![TypedWriteIntent::update(PENDING)
                .key("id", id.to_string())
                .key("status", pending_status_as_str(from))
                .value("status", pending_status_as_str(to))
                .value("resolved_at", Utc::now().to_rfc3339())
                .purpose("alicia_memoire_confirmation_resolue")])?;
        if n.first().copied().unwrap_or(0) == 1 {
            return Ok(());
        }
        match self.pending_confirmation(id)? {
            None => Err(MemoryError::PendingConfirmationNotFound(id)),
            Some(pc) => Err(MemoryError::PendingConfirmationAlreadyResolved {
                id,
                status: pending_status_as_str(pc.status).into(),
            }),
        }
    }
}

// ----------------------------------------------------------------------
// Versionnage des faits (EX-52)
// ----------------------------------------------------------------------

/// Longueur maximale de l'auteur d'un oubli (C-13).
const OUBLI_AUTEUR_MAX: usize = 128;
/// Longueur maximale du motif d'un oubli (C-13).
const OUBLI_MOTIF_MAX: usize = 512;

/// État courant d'un fait, lu avant une nouvelle écriture.
struct EtatFait {
    persona: String,
    subject: String,
    content: String,
    source: String,
    confidence: f64,
}

impl EtatFait {
    fn de(l: &Ligne) -> Result<Self, MemoryError> {
        Ok(Self {
            persona: texte(l, "persona")?,
            subject: texte(l, "subject")?,
            content: texte(l, "content")?,
            source: texte(l, "source")?,
            confidence: reel(l, "confidence")?,
        })
    }

    /// Vrai si `fait` n'apporte aucun changement à l'état courant.
    fn identique(&self, fait: &Fact) -> bool {
        self.persona == fait.persona
            && self.subject == fait.subject
            && self.content == fait.content
            && self.source == fait.source
            && (self.confidence - f64::from(fait.confidence)).abs() < f64::EPSILON
    }
}

/// Valide un champ obligatoire d'une demande d'oubli (non vide, borné).
fn valider_champ_oubli(nom: &str, valeur: &str, max: usize) -> Result<(), MemoryError> {
    let v = valeur.trim();
    if v.is_empty() {
        return Err(MemoryError::InvalidForgetRequest(format!("{nom} manquant")));
    }
    if v.chars().count() > max {
        return Err(MemoryError::InvalidForgetRequest(format!(
            "{nom} trop long (max {max} caractères)"
        )));
    }
    Ok(())
}

/// Intention d'ajout d'une version de fait (append-only).
fn intention_version(f: &Fact, version: u32, enregistre: DateTime<Utc>) -> TypedWriteIntent {
    TypedWriteIntent::insert(FACT_VERSIONS)
        .value("fact_id", f.id.to_string())
        .value("version", i64::from(version))
        .value("persona", f.persona.as_str())
        .value("subject", f.subject.as_str())
        .value("content", f.content.as_str())
        .value("source", f.source.as_str())
        .value("confidence", f64::from(f.confidence))
        .value("recorded_at", enregistre.to_rfc3339())
        .purpose("alicia_memoire_version_fait")
}

// ----------------------------------------------------------------------
// Statuts
// ----------------------------------------------------------------------

fn outcome_as_str(o: EpisodeOutcome) -> &'static str {
    match o {
        EpisodeOutcome::Ok => "ok",
        EpisodeOutcome::Cancelled => "cancelled",
        EpisodeOutcome::Failed => "failed",
    }
}

fn pending_status_as_str(s: PendingStatus) -> &'static str {
    match s {
        PendingStatus::Pending => "pending",
        PendingStatus::Executing => "executing",
        PendingStatus::Confirmed => "confirmed",
        PendingStatus::Failed => "failed",
        PendingStatus::Cancelled => "cancelled",
    }
}

fn learned_status_as_str(s: LearnedSkillStatus) -> &'static str {
    match s {
        LearnedSkillStatus::Proposed => "proposed",
        LearnedSkillStatus::Accepted => "accepted",
        LearnedSkillStatus::Rejected => "rejected",
    }
}

// ----------------------------------------------------------------------
// Ligne → struct
// ----------------------------------------------------------------------

fn illisible(c: &str) -> MemoryError {
    MemoryError::Corrupt(format!("colonne {c} absente ou de type inattendu"))
}

fn texte(l: &Ligne, c: &str) -> Result<String, MemoryError> {
    l.get(c)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| illisible(c))
}

fn texte_opt(l: &Ligne, c: &str) -> Result<Option<String>, MemoryError> {
    match l.get(c) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(illisible(c)),
    }
}

fn entier(l: &Ligne, c: &str) -> Result<i64, MemoryError> {
    l.get(c).and_then(Value::as_i64).ok_or_else(|| illisible(c))
}

fn reel(l: &Ligne, c: &str) -> Result<f64, MemoryError> {
    l.get(c).and_then(Value::as_f64).ok_or_else(|| illisible(c))
}

#[allow(clippy::cast_possible_truncation)]
fn confiance(l: &Ligne, c: &str) -> Result<f32, MemoryError> {
    Ok(reel(l, c)? as f32)
}

fn u32_col(l: &Ligne, c: &str) -> Result<u32, MemoryError> {
    u32::try_from(entier(l, c)?)
        .map_err(|_| MemoryError::Corrupt(format!("colonne {c} hors bornes")))
}

fn parse_date(s: &str, c: &str) -> Result<DateTime<Utc>, MemoryError> {
    DateTime::parse_from_rfc3339(s)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|e| {
            MemoryError::Corrupt(format!("colonne {c} : date RFC3339 invalide '{s}' : {e}"))
        })
}

fn date(l: &Ligne, c: &str) -> Result<DateTime<Utc>, MemoryError> {
    parse_date(&texte(l, c)?, c)
}

fn date_opt(l: &Ligne, c: &str) -> Result<Option<DateTime<Utc>>, MemoryError> {
    texte_opt(l, c)?.map(|s| parse_date(&s, c)).transpose()
}

fn uuid_col(l: &Ligne, c: &str) -> Result<Uuid, MemoryError> {
    let s = texte(l, c)?;
    Uuid::parse_str(&s)
        .map_err(|e| MemoryError::Corrupt(format!("colonne {c} : uuid invalide '{s}' : {e}")))
}

fn json_col(l: &Ligne, c: &str) -> Result<Value, MemoryError> {
    Ok(serde_json::from_str(&texte(l, c)?)?)
}

fn fact_de(l: &Ligne) -> Result<Fact, MemoryError> {
    Ok(Fact {
        id: uuid_col(l, "id")?,
        persona: texte(l, "persona")?,
        subject: texte(l, "subject")?,
        content: texte(l, "content")?,
        source: texte(l, "source")?,
        confidence: confiance(l, "confidence")?,
        created_at: date(l, "created_at")?,
        updated_at: date(l, "updated_at")?,
    })
}

fn version_de(l: &Ligne) -> Result<FactVersion, MemoryError> {
    Ok(FactVersion {
        fact_id: uuid_col(l, "fact_id")?,
        version: u32_col(l, "version")?,
        persona: texte(l, "persona")?,
        subject: texte(l, "subject")?,
        content: texte(l, "content")?,
        source: texte(l, "source")?,
        confidence: confiance(l, "confidence")?,
        recorded_at: date(l, "recorded_at")?,
    })
}

fn tombe_de(l: &Ligne) -> Result<FactTombstone, MemoryError> {
    Ok(FactTombstone {
        fact_id: uuid_col(l, "fact_id")?,
        forgotten_at: date(l, "forgotten_at")?,
        forgotten_by: texte(l, "forgotten_by")?,
        reason: texte(l, "reason")?,
        versions_erased: u32_col(l, "versions_erased")?,
    })
}

fn preference_de(l: &Ligne) -> Result<Preference, MemoryError> {
    Ok(Preference {
        scope: texte(l, "scope")?,
        key: texte(l, "key")?,
        value: json_col(l, "value_json")?,
        updated_at: date(l, "updated_at")?,
    })
}

fn episode_de(l: &Ligne) -> Result<Episode, MemoryError> {
    let outcome = match texte(l, "outcome")?.as_str() {
        "ok" => EpisodeOutcome::Ok,
        "cancelled" => EpisodeOutcome::Cancelled,
        "failed" => EpisodeOutcome::Failed,
        other => return Err(MemoryError::Corrupt(format!("outcome inconnu : {other}"))),
    };
    Ok(Episode {
        id: uuid_col(l, "id")?,
        persona: texte(l, "persona")?,
        service_id: texte(l, "service_id")?,
        intent: texte(l, "intent")?,
        user_phrase: texte(l, "user_phrase")?,
        params: json_col(l, "params_json")?,
        outcome,
        error: texte_opt(l, "error")?,
        created_at: date(l, "created_at")?,
    })
}

fn persona_de(l: &Ligne) -> Result<Persona, MemoryError> {
    Ok(Persona {
        id: texte(l, "id")?,
        display_name: texte(l, "display_name")?,
        description: texte(l, "description")?,
        is_default: entier(l, "is_default")? != 0,
        created_at: date(l, "created_at")?,
    })
}

fn skill_de(l: &Ligne) -> Result<LearnedSkill, MemoryError> {
    let status = match texte(l, "status")?.as_str() {
        "proposed" => LearnedSkillStatus::Proposed,
        "accepted" => LearnedSkillStatus::Accepted,
        "rejected" => LearnedSkillStatus::Rejected,
        other => {
            return Err(MemoryError::Corrupt(format!(
                "statut LearnedSkill inconnu : {other}"
            )))
        }
    };
    Ok(LearnedSkill {
        id: uuid_col(l, "id")?,
        persona: texte(l, "persona")?,
        trigger_phrase: texte(l, "trigger_phrase")?,
        qualified_id: texte(l, "qualified_id")?,
        params_template: json_col(l, "params_template")?,
        status,
        created_at: date(l, "created_at")?,
        accepted_at: date_opt(l, "accepted_at")?,
    })
}

fn pending_de(l: &Ligne) -> Result<PendingConfirmation, MemoryError> {
    let status = match texte(l, "status")?.as_str() {
        "pending" => PendingStatus::Pending,
        "executing" => PendingStatus::Executing,
        "confirmed" => PendingStatus::Confirmed,
        "failed" => PendingStatus::Failed,
        "cancelled" => PendingStatus::Cancelled,
        other => {
            return Err(MemoryError::Corrupt(format!(
                "statut PendingConfirmation inconnu : {other}"
            )))
        }
    };
    Ok(PendingConfirmation {
        id: uuid_col(l, "id")?,
        qualified_id: texte(l, "qualified_id")?,
        params: json_col(l, "params_json")?,
        persona: texte(l, "persona")?,
        user_phrase: texte(l, "user_phrase")?,
        prompt: texte(l, "prompt")?,
        status,
        created_at: date(l, "created_at")?,
        resolved_at: date_opt(l, "resolved_at")?,
        reason: texte_opt(l, "reason")?,
    })
}
