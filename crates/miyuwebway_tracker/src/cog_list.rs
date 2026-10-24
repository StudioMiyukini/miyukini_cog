//! Tools MWS MiyuWebwayTracker — mws.cog_list.* (get, update, merge, filter).
//! Liste locale de COGs (en mémoire, bornée) ; sync réelle déléguée à Origin / miyuwebway_participant.
//!
//! - `get` / `update` : implémentés ; liste bornée à [`MAX_COG_ENTRIES`] entrées,
//!   chaque entrée validée (longueur, jeu de caractères) — C-13.
//! - `update` avec charge utile (statuts) : non stockée → refus explicite.
//! - `merge` / `filter` : la règle / le critère des Cores n'est pas encore
//!   interprétable ; les ignorer accepterait des COGs distants ou exposerait des
//!   COGs rejetés (fail-open) → refus explicite (C-12, C-14).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;
use std::sync::Mutex;

/// Nombre maximal d'entrées de la liste locale.
pub const MAX_COG_ENTRIES: usize = 4096;
/// Longueur maximale d'un identifiant COG (octets).
pub const MAX_COG_ENTRY_BYTES: usize = 128;

/// Liste bornée d'identifiants COG (sans doublon, ordre d'insertion).
#[derive(Debug)]
pub struct CogListStore {
    entries: Vec<String>,
    max: usize,
}

impl CogListStore {
    /// Liste vide bornée à `max` entrées.
    pub const fn new(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            max,
        }
    }

    /// Entrées courantes.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    /// Ajoute `entry` si absente ; refuse une entrée invalide ou une liste pleine.
    pub fn insert(&mut self, entry: &str) -> Result<(), MiyuwebwayTrackerError> {
        validate_entry(entry)?;
        if self.entries.iter().any(|e| e == entry) {
            return Ok(());
        }
        if self.entries.len() >= self.max {
            return Err(MiyuwebwayTrackerError::CapacityExceeded { max: self.max });
        }
        self.entries.push(entry.to_string());
        Ok(())
    }
}

/// Valide un identifiant COG : 1..=128 octets, `[A-Za-z0-9._:-]`.
pub fn validate_entry(entry: &str) -> Result<(), MiyuwebwayTrackerError> {
    if entry.is_empty() {
        return Err(MiyuwebwayTrackerError::InvalidEntry("empty"));
    }
    if entry.len() > MAX_COG_ENTRY_BYTES {
        return Err(MiyuwebwayTrackerError::InvalidEntry("too long"));
    }
    if !entry
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b':' | b'-'))
    {
        return Err(MiyuwebwayTrackerError::InvalidEntry("forbidden character"));
    }
    Ok(())
}

static COG_LIST: Mutex<CogListStore> = Mutex::new(CogListStore::new(MAX_COG_ENTRIES));

fn lock() -> Result<std::sync::MutexGuard<'static, CogListStore>, MiyuwebwayTrackerError> {
    COG_LIST
        .lock()
        .map_err(|_| MiyuwebwayTrackerError::TrackerUnavailable("cog_list lock".into()))
}

/// @id miyuwebway_tracker_mws_cog_list_get
/// @role accessor
/// @layer tool
/// @do mws_cog_list_get_under_governance
//@human / @human Lit la liste locale de COGs ; lecture.
pub fn get(ctx: &GovernedContext) -> Result<Vec<String>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(lock()?.entries().to_vec())
}

/// @id miyuwebway_tracker_mws_cog_list_update
/// @role mutator
/// @layer tool
/// @do mws_cog_list_update_under_governance
//@human / @human Met à jour une entrée dans la liste locale ; écriture liste locale.
/// Une charge utile non vide (statuts) n'est pas encore stockée : refus explicite
/// plutôt que de l'ignorer silencieusement.
pub fn update(
    ctx: &GovernedContext,
    entry: &str,
    payload: &str,
) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    if !payload.is_empty() {
        return Err(MiyuwebwayTrackerError::Unimplemented(
            "mws.cog_list.update (payload)",
        ));
    }
    lock()?.insert(entry)
}

/// @id miyuwebway_tracker_mws_cog_list_merge
/// @role mutator
/// @layer tool
/// @do mws_cog_list_merge_under_governance
//@human / @human Fusionne une liste reçue avec la liste locale ; règle fournie par Cores.
/// Refus explicite : règle de fusion des Cores non implémentée.
pub fn merge(
    ctx: &GovernedContext,
    _incoming: &str,
    _rule_ref: &str,
) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented("mws.cog_list.merge"))
}

/// @id miyuwebway_tracker_mws_cog_list_filter
/// @role accessor
/// @layer tool
/// @do mws_cog_list_filter_under_governance
//@human / @human Filtre la liste selon critère ; critère fourni par Border Guard, WorrySentinel.
/// Refus explicite : critère de filtrage des Cores non implémenté.
pub fn filter(
    ctx: &GovernedContext,
    _criterion_ref: &str,
) -> Result<Vec<String>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented("mws.cog_list.filter"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 2)
    }

    #[test]
    fn refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 2);
        assert_eq!(get(&empty), Err(MiyuwebwayTrackerError::NoMandate));
        assert_eq!(
            update(&empty, "a", ""),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
        assert_eq!(
            merge(&empty, "a", "r"),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
        assert_eq!(filter(&empty, "c"), Err(MiyuwebwayTrackerError::NoMandate));
    }

    #[test]
    fn update_puis_get() {
        update(&ctx(), "cog-test-update-get", "").unwrap();
        update(&ctx(), "cog-test-update-get", "").unwrap();
        let list = get(&ctx()).unwrap();
        assert_eq!(
            list.iter().filter(|e| *e == "cog-test-update-get").count(),
            1
        );
    }

    #[test]
    fn update_avec_payload_refuse() {
        assert_eq!(
            update(&ctx(), "cog-test-payload", "status=ok"),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.cog_list.update (payload)"
            ))
        );
        assert!(!get(&ctx())
            .unwrap()
            .contains(&"cog-test-payload".to_string()));
    }

    #[test]
    fn entree_invalide_refusee() {
        for bad in ["", "a b", "a,b", "a/b", "é"] {
            assert!(matches!(
                update(&ctx(), bad, ""),
                Err(MiyuwebwayTrackerError::InvalidEntry(_))
            ));
        }
        let long = "a".repeat(MAX_COG_ENTRY_BYTES + 1);
        assert_eq!(
            update(&ctx(), &long, ""),
            Err(MiyuwebwayTrackerError::InvalidEntry("too long"))
        );
    }

    #[test]
    fn liste_bornee() {
        let mut store = CogListStore::new(2);
        store.insert("a").unwrap();
        store.insert("b").unwrap();
        store.insert("a").unwrap();
        assert_eq!(
            store.insert("c"),
            Err(MiyuwebwayTrackerError::CapacityExceeded { max: 2 })
        );
        assert_eq!(store.entries(), ["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn merge_et_filter_refusent_explicitement() {
        assert_eq!(
            merge(&ctx(), "cog-x,cog-y", "union"),
            Err(MiyuwebwayTrackerError::Unimplemented("mws.cog_list.merge"))
        );
        assert!(!get(&ctx()).unwrap().contains(&"cog-x".to_string()));
        assert_eq!(
            filter(&ctx(), "exclude:rejected"),
            Err(MiyuwebwayTrackerError::Unimplemented("mws.cog_list.filter"))
        );
    }
}
