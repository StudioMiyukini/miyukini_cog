//! Tests EX-52 : faits versionnés, oubli demandé avec pierre tombale,
//! migration additive depuis l'ancien schéma.

// @id service.alicia.memory.tests.fact_versions
// @role tests
// @layer 7
// @do test_fact_versioning
// @human Vérifie qu'aucune écriture de fait n'est destructive hors oubli demandé.

#![allow(clippy::assert_is_empty)]

use chrono::{Duration, Utc};
use miyualicia_memory::{Fact, MemoryError, MemoryStore};

fn store() -> MemoryStore {
    MemoryStore::open_in_memory().expect("init store")
}

fn fait(contenu: &str) -> Fact {
    Fact::new("voice".into(), "cat:mochi".into(), contenu.into(), "user".into())
}

#[test]
fn mise_a_jour_conserve_la_version_precedente() {
    let s = store();
    let mut f = fait("le chat s'appelle Mochi");
    s.remember(&f).unwrap();

    f.content = "le chat s'appelle Mochi-San".into();
    f.updated_at = Utc::now() + Duration::seconds(1);
    s.remember(&f).unwrap();

    let courant = s.fact(f.id).unwrap().expect("fait courant");
    assert_eq!(courant.content, "le chat s'appelle Mochi-San");

    let h = s.fact_history(f.id).unwrap();
    assert_eq!(h.len(), 2);
    assert_eq!(h[0].content, "le chat s'appelle Mochi");
    assert_eq!(h[1].content, "le chat s'appelle Mochi-San");

    // L'index plein texte suit la version courante, sans doublon.
    let hits = s.recall("Mochi", 5).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].content, "le chat s'appelle Mochi-San");
}

#[test]
fn historique_ordonne_et_idempotent() {
    let s = store();
    let mut f = fait("v1");
    s.remember(&f).unwrap();
    // Réécriture identique : pas de nouvelle version.
    s.remember(&f).unwrap();
    for c in ["v2", "v3", "v4"] {
        f.content = c.into();
        s.remember(&f).unwrap();
    }
    f.confidence = 0.5;
    s.remember(&f).unwrap();

    let h = s.fact_history(f.id).unwrap();
    let versions: Vec<u32> = h.iter().map(|v| v.version).collect();
    assert_eq!(versions, vec![1, 2, 3, 4, 5]);
    let contenus: Vec<&str> = h.iter().map(|v| v.content.as_str()).collect();
    assert_eq!(contenus, vec!["v1", "v2", "v3", "v4", "v4"]);
    assert!((h[4].confidence - 0.5).abs() < f32::EPSILON);
}

#[test]
fn oubli_laisse_une_pierre_tombale_sans_contenu() {
    let s = store();
    let mut f = fait("secret Mochi");
    s.remember(&f).unwrap();
    f.content = "secret Mochi bis".into();
    s.remember(&f).unwrap();

    let n = s.forget(f.id, "user", "demande explicite").unwrap();
    assert_eq!(n, 1);

    assert!(s.fact(f.id).unwrap().is_none());
    assert!(s.fact_history(f.id).unwrap().is_empty());
    assert!(s.recall("Mochi", 5).unwrap().is_empty());
    assert!(s.facts_for_persona("voice").unwrap().is_empty());

    let t = s.fact_tombstone(f.id).unwrap().expect("pierre tombale");
    assert_eq!(t.fact_id, f.id);
    assert_eq!(t.forgotten_by, "user");
    assert_eq!(t.reason, "demande explicite");
    assert_eq!(t.versions_erased, 2);

    // Un fait oublié ne revient pas sous le même identifiant.
    let err = s.remember(&f).unwrap_err();
    assert!(matches!(err, MemoryError::FactForgotten(id) if id == f.id));

    // Second oubli : rien à faire, pierre tombale inchangée.
    assert_eq!(s.forget(f.id, "user", "encore").unwrap(), 0);
    assert_eq!(s.fact_tombstone(f.id).unwrap().expect("tombe").reason, "demande explicite");
}

#[test]
fn oubli_exige_auteur_et_motif() {
    let s = store();
    let f = fait("x");
    s.remember(&f).unwrap();
    assert!(matches!(
        s.forget(f.id, " ", "motif"),
        Err(MemoryError::InvalidForgetRequest(_))
    ));
    assert!(matches!(
        s.forget(f.id, "user", ""),
        Err(MemoryError::InvalidForgetRequest(_))
    ));
    assert!(matches!(
        s.forget(f.id, "user", &"m".repeat(513)),
        Err(MemoryError::InvalidForgetRequest(_))
    ));
    // Refus = aucune écriture.
    assert!(s.fact(f.id).unwrap().is_some());
    assert!(s.fact_tombstone(f.id).unwrap().is_none());
}

#[test]
fn oubli_d_un_fait_inconnu_ne_cree_rien() {
    let s = store();
    let id = uuid::Uuid::new_v4();
    assert_eq!(s.forget(id, "user", "test").unwrap(), 0);
    assert!(s.fact_tombstone(id).unwrap().is_none());
}

#[test]
fn migration_depuis_l_ancien_schema() {
    let dir = tempfile::tempdir().unwrap();
    let chemin = dir.path().join("memoire.db");
    let id = uuid::Uuid::new_v4();
    let maintenant = Utc::now().to_rfc3339();
    {
        // Schéma antérieur : facts + FTS, sans versions ni pierres tombales,
        // avec le doublon FTS que produisait l'ancien `remember`.
        let c = rusqlite::Connection::open(&chemin).unwrap();
        c.execute_batch(
            "CREATE TABLE facts (
                id TEXT PRIMARY KEY, persona TEXT NOT NULL, subject TEXT NOT NULL,
                content TEXT NOT NULL, source TEXT NOT NULL, confidence REAL NOT NULL,
                created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
             CREATE VIRTUAL TABLE facts_fts
                USING fts5(content, content='facts', content_rowid='rowid');",
        )
        .unwrap();
        c.execute(
            "INSERT INTO facts VALUES (?1, 'voice', 'user', 'Jean aime le métro', 'user', 1.0, ?2, ?2)",
            rusqlite::params![id.to_string(), maintenant],
        )
        .unwrap();
        c.execute_batch(
            "INSERT INTO facts_fts(rowid, content) SELECT rowid, content FROM facts;
             INSERT INTO facts_fts(rowid, content) SELECT rowid, content FROM facts;",
        )
        .unwrap();
    }

    let s = MemoryStore::open(&chemin).unwrap();
    let h = s.fact_history(id).unwrap();
    assert_eq!(h.len(), 1);
    assert_eq!(h[0].version, 1);
    assert_eq!(h[0].content, "Jean aime le métro");
    assert_eq!(s.recall("métro", 5).unwrap().len(), 1);

    // Mise à jour après migration : v2, v1 conservée.
    let mut f = s.fact(id).unwrap().expect("fait migré");
    f.content = "Jean aime le train".into();
    s.remember(&f).unwrap();
    let h = s.fact_history(id).unwrap();
    assert_eq!(h.len(), 2);
    assert_eq!(h[0].content, "Jean aime le métro");
    assert_eq!(h[1].content, "Jean aime le train");
    drop(s);

    // Réouverture : migration idempotente (pas de version dupliquée).
    let s = MemoryStore::open(&chemin).unwrap();
    assert_eq!(s.fact_history(id).unwrap().len(), 2);
    assert_eq!(s.recall("train", 5).unwrap().len(), 1);
    assert!(s.recall("métro", 5).unwrap().is_empty());
}

#[test]
fn oubli_efface_le_contenu_du_fichier() {
    let dir = std::env::temp_dir().join(format!("alicia-oubli-{}", uuid_court()));
    std::fs::create_dir_all(&dir).unwrap();
    let chemin = dir.join("memoire.db");
    let marqueur = "MARQUEUR-SECRET-A-OUBLIER-7f3a";
    {
        let s = MemoryStore::open(&chemin).unwrap();
        let mut f = fait(&format!("{marqueur} version 1"));
        s.remember(&f).unwrap();
        f.content = format!("{marqueur} version 2");
        f.updated_at = Utc::now() + Duration::seconds(1);
        s.remember(&f).unwrap();
        s.forget(f.id, "test", "droit à l'oubli").unwrap();
    }
    for entree in std::fs::read_dir(&dir).unwrap() {
        let octets = std::fs::read(entree.unwrap().path()).unwrap();
        assert!(
            !octets.windows(marqueur.len()).any(|w| w == marqueur.as_bytes()),
            "contenu oublié encore présent sur disque"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

fn uuid_court() -> String {
    format!("{}-{}", std::process::id(), Utc::now().timestamp_nanos_opt().unwrap_or(0))
}
