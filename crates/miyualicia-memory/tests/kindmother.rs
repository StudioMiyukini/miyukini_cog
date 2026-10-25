//! Mémoire d'Alicia par KindMother (K-13) : écriture et lecture réelles,
//! base nationale chiffrée et cloisonnée, import unique et idempotent de
//! l'ancienne base SQLite, KindMother indisponible ⇒ erreur explicite.

// @id service.alicia.memory.tests.kindmother
// @role tests
// @layer 7
// @do test_alicia_memory_through_kindmother
// @human Vérifie que KindMother est le seul écrivain de la mémoire d'Alicia, sans repli local.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::assert_is_empty
)]

use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::{Duration, Utc};
use kindmother_client::{KindMotherClient, OperatorCredential, SqlValue, TypedSelect};
use miyualicia_memory::{
    ConfigKindMother, Episode, EpisodeOutcome, Fact, MemoryError, MemoryStore, PendingStatus,
    Persona, Preference, BASE_KINDMOTHER,
};
use serde_json::json;

const TTL_EXEC: std::time::Duration = std::time::Duration::from_mins(15);
const TTL_PENDING: std::time::Duration = std::time::Duration::from_hours(24);

/// Serveur KindMother éphémère (SQLCipher par défaut) : adresse, justificatifs.
fn serveur(dir: &Path) -> (String, PathBuf) {
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(dir).expect("serveur KindMother");
    (addr.to_string(), creds)
}

fn cfg(addr: &str, creds: &Path) -> ConfigKindMother {
    ConfigKindMother::new(addr).avec_justificatifs(creds)
}

fn ouvrir(addr: &str, creds: &Path) -> MemoryStore {
    MemoryStore::connecter(&cfg(addr, creds)).expect("ouverture KindMother")
}

/// Tous les fichiers sous `dir` (récursif).
fn fichiers(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut pile = vec![dir.to_path_buf()];
    while let Some(d) = pile.pop() {
        for e in std::fs::read_dir(&d).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                pile.push(p);
            } else {
                out.push(p);
            }
        }
    }
    out
}

fn contient(dir: &Path, motif: &str) -> bool {
    fichiers(dir).iter().any(|f| {
        std::fs::read(f).is_ok_and(|o| o.windows(motif.len()).any(|w| w == motif.as_bytes()))
    })
}

#[test]
fn ecriture_et_lecture_persistantes_par_kindmother() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let s = ouvrir(&addr, &creds);

    s.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "Mode vocal".into(),
        true,
    ))
    .unwrap();
    let fait = Fact::new(
        "voice".into(),
        "cat:mochi".into(),
        "Le chat s'appelle Mochi-Secret".into(),
        "user".into(),
    );
    s.remember(&fait).unwrap();
    s.prefer(&Preference::new(
        "voice".into(),
        "notify".into(),
        json!("desktop"),
    ))
    .unwrap();
    let ep = Episode::new(
        "voice".into(),
        "jaykoa".into(),
        "schedule_event".into(),
        "planifie la réunion de demain".into(),
        json!({"titre": "réunion"}),
        EpisodeOutcome::Ok,
        None,
    );
    s.record_episode(&ep).unwrap();
    let skill = s
        .propose_learned_skill(
            "voice",
            "réunion demain",
            "jaykoa.schedule_event",
            json!({}),
        )
        .unwrap();
    s.accept_learned_skill(skill.id).unwrap();
    let pc = s
        .enqueue_pending_confirmation(
            "jaykoa.schedule_event",
            json!({"a": 1}),
            "voice",
            "x",
            "Planifier ?",
        )
        .unwrap();
    s.claim_pending_confirmation(pc.id).unwrap();
    drop(s);

    // Nouvelle connexion : tout est relu depuis KindMother.
    let s = ouvrir(&addr, &creds);
    assert_eq!(s.default_persona().unwrap().unwrap().id, "voice");
    assert_eq!(s.fact(fait.id).unwrap().unwrap().content, fait.content);
    assert_eq!(s.fact_history(fait.id).unwrap().len(), 1);
    assert_eq!(s.recall("mochi", 5).unwrap().len(), 1);
    assert_eq!(
        s.preference("voice", "notify").unwrap().unwrap().value,
        json!("desktop")
    );
    assert_eq!(s.recent_episodes(10).unwrap(), vec![ep.clone()]);
    assert_eq!(s.search_episodes("réunion", 5).unwrap().len(), 1);
    assert!(s
        .match_learned_skill("voice", "Réunion demain !")
        .unwrap()
        .is_some());
    assert_eq!(
        s.pending_confirmation(pc.id).unwrap().unwrap().status,
        PendingStatus::Executing
    );

    // Base dédiée, chiffrée (SQLCipher) : aucun contenu en clair sur disque.
    let fichier = dir.path().join(format!("{BASE_KINDMOTHER}.db"));
    assert!(fichier.exists(), "base KindMother dédiée : {fichier:?}");
    assert!(
        !kindmother_service::is_plaintext_sqlite(&fichier),
        "base chiffrée"
    );
    assert!(
        !contient(dir.path(), "Mochi-Secret"),
        "aucun contenu en clair"
    );
}

#[test]
fn base_nationale_hors_de_l_espace_des_utilisateurs_maia() {
    // Le préfixe `maia-` classe la base nationale (règle de KindMother) ;
    // `_alicia` n'est pas un identifiant d'utilisateur MAIA : le service refuse
    // de démarrer si on le déclare.
    assert!(BASE_KINDMOTHER.starts_with("maia-"));
    let refus = tempfile::tempdir().unwrap();
    std::fs::write(refus.path().join("maia-utilisateurs"), "_alicia\n").unwrap();
    assert!(kindmother_service::spawn_ephemeral_blocking(refus.path()).is_err());

    // Un utilisateur MAIA « alicia » a sa propre base, sans accès à celle d'Alicia.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("maia-utilisateurs"), "alicia\n").unwrap();
    let (addr, creds) = serveur(dir.path());
    let s = ouvrir(&addr, &creds);
    s.remember(&Fact::new(
        "voice".into(),
        "user".into(),
        "secret du foyer".into(),
        "user".into(),
    ))
    .unwrap();
    let cred = OperatorCredential::load_from_dir("maia-alicia", &creds).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let intrus = KindMotherClient::connect(&addr, &cred, BASE_KINDMOTHER)
            .await
            .unwrap();
        assert!(
            intrus.select(TypedSelect::from("facts")).await.is_err(),
            "cloisonnement"
        );
        // Sensibilité lue chez KindMother par l'opérateur d'Alicia : nationale.
        let alicia = OperatorCredential::load_from_dir(BASE_KINDMOTHER, &creds).unwrap();
        let c = KindMotherClient::connect(&addr, &alicia, BASE_KINDMOTHER)
            .await
            .unwrap();
        let etat = c
            .lire_etiquette(
                kindmother_client::RefDonnee::new(BASE_KINDMOTHER, "meta", "valeur")
                    .cle("cle", "etiquette"),
            )
            .await
            .unwrap();
        assert_eq!(etat.sensibilite, kindmother_client::Sensibilite::Nationale);
        assert!(!etat.publiable);
    });
}

#[test]
fn kindmother_indisponible_erreur_explicite_sans_repli_local() {
    // Port libre : rien n'écoute.
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let dir = tempfile::tempdir().unwrap();
    let (_, creds) = serveur(dir.path());
    let ancienne = dir.path().join("jamais-creee.db");
    let r = MemoryStore::connecter(
        &cfg(&format!("127.0.0.1:{port}"), &creds).avec_base_heritee(&ancienne),
    );
    assert!(
        matches!(r, Err(MemoryError::KindMotherUnavailable(_))),
        "erreur explicite attendue, obtenu {:?}",
        r.err()
    );
    assert!(!ancienne.exists(), "aucun fichier local créé");

    // Justificatif absent : refus explicite.
    let vide = tempfile::tempdir().unwrap();
    assert!(matches!(
        MemoryStore::connecter(&cfg(&format!("127.0.0.1:{port}"), vide.path())),
        Err(MemoryError::KindMother(_))
    ));
}

#[test]
fn kindmother_coupe_apres_ouverture_erreur_explicite() {
    let dir = tempfile::tempdir().unwrap();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let (addr, creds) = rt
        .block_on(kindmother_service::spawn_ephemeral(dir.path()))
        .unwrap();
    let s = ouvrir(&addr.to_string(), &creds);
    let f = Fact::new(
        "voice".into(),
        "user".into(),
        "avant la coupure".into(),
        "user".into(),
    );
    s.remember(&f).unwrap();

    // Arrêt de KindMother.
    rt.shutdown_timeout(std::time::Duration::from_secs(5));

    let lecture = s.fact(f.id);
    assert!(
        matches!(lecture, Err(MemoryError::KindMotherUnavailable(_))),
        "{lecture:?}"
    );
    let ecriture = s.remember(&Fact::new(
        "voice".into(),
        "user".into(),
        "après".into(),
        "user".into(),
    ));
    assert!(
        matches!(ecriture, Err(MemoryError::KindMotherUnavailable(_))),
        "{ecriture:?}"
    );
    assert!(matches!(
        s.recent_episodes(5),
        Err(MemoryError::KindMotherUnavailable(_))
    ));
}

#[test]
fn reclamations_concurrentes_une_seule_gagne_dans_kindmother() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let s = Arc::new(ouvrir(&addr, &creds));
    let pc = s
        .enqueue_pending_confirmation("demo.shout", json!(null), "voice", "x", "p")
        .unwrap();
    let fils: Vec<_> = (0..6)
        .map(|_| {
            let s = Arc::clone(&s);
            std::thread::spawn(move || s.claim_pending_confirmation(pc.id).is_ok())
        })
        .collect();
    let gagnants = fils
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|ok| *ok)
        .count();
    assert_eq!(gagnants, 1);
}

#[test]
fn oubli_reel_dans_kindmother() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let marqueur = "MARQUEUR-SECRET-A-OUBLIER-7f3a";
    let s = ouvrir(&addr, &creds);
    let mut f = Fact::new(
        "voice".into(),
        "user".into(),
        format!("{marqueur} version 1"),
        "user".into(),
    );
    s.remember(&f).unwrap();
    f.content = format!("{marqueur} version 2");
    f.updated_at = Utc::now() + Duration::seconds(1);
    s.remember(&f).unwrap();
    assert_eq!(s.forget(f.id, "test", "droit à l'oubli").unwrap(), 1);
    drop(s);

    let s = ouvrir(&addr, &creds);
    assert!(s.fact(f.id).unwrap().is_none());
    assert!(s.fact_history(f.id).unwrap().is_empty());
    let t = s.fact_tombstone(f.id).unwrap().unwrap();
    assert_eq!(t.versions_erased, 2);
    assert!(matches!(s.remember(&f), Err(MemoryError::FactForgotten(_))));
    assert!(
        !contient(dir.path(), marqueur),
        "contenu oublié présent sur disque"
    );
}

// ----------------------------------------------------------------------
// Ancienne base SQLite
// ----------------------------------------------------------------------

/// Schéma exact de l'ancienne base (rusqlite + FTS5), versionnage EX-52 compris.
const SCHEMA_HERITE: &str = r"
PRAGMA secure_delete = ON;
CREATE TABLE facts (id TEXT PRIMARY KEY, persona TEXT NOT NULL, subject TEXT NOT NULL,
    content TEXT NOT NULL, source TEXT NOT NULL, confidence REAL NOT NULL,
    created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
CREATE INDEX idx_facts_persona ON facts(persona);
CREATE VIRTUAL TABLE facts_fts USING fts5(content, content='facts', content_rowid='rowid');
CREATE TABLE preferences (scope TEXT NOT NULL, key TEXT NOT NULL, value_json TEXT NOT NULL,
    updated_at TEXT NOT NULL, PRIMARY KEY (scope, key));
CREATE TABLE episodes (id TEXT PRIMARY KEY, persona TEXT NOT NULL, service_id TEXT NOT NULL,
    intent TEXT NOT NULL, user_phrase TEXT NOT NULL, params_json TEXT NOT NULL,
    outcome TEXT NOT NULL, error TEXT, created_at TEXT NOT NULL);
CREATE VIRTUAL TABLE episodes_fts USING fts5(user_phrase, content='episodes', content_rowid='rowid');
CREATE TABLE personas (id TEXT PRIMARY KEY, display_name TEXT NOT NULL, description TEXT NOT NULL,
    is_default INTEGER NOT NULL, created_at TEXT NOT NULL);
CREATE TABLE learned_skills (id TEXT PRIMARY KEY, persona TEXT NOT NULL, trigger_phrase TEXT NOT NULL,
    qualified_id TEXT NOT NULL, params_template TEXT NOT NULL, status TEXT NOT NULL,
    created_at TEXT NOT NULL, accepted_at TEXT);
CREATE TABLE pending_confirmations (id TEXT PRIMARY KEY, qualified_id TEXT NOT NULL,
    params_json TEXT NOT NULL, persona TEXT NOT NULL, user_phrase TEXT NOT NULL, prompt TEXT NOT NULL,
    status TEXT NOT NULL, created_at TEXT NOT NULL, resolved_at TEXT,
    claimed_at INTEGER NOT NULL DEFAULT 0, reason TEXT);
CREATE TABLE fact_versions (fact_id TEXT NOT NULL, version INTEGER NOT NULL CHECK (version >= 1),
    persona TEXT NOT NULL, subject TEXT NOT NULL, content TEXT NOT NULL, source TEXT NOT NULL,
    confidence REAL NOT NULL, recorded_at TEXT NOT NULL, PRIMARY KEY (fact_id, version));
CREATE TABLE fact_tombstones (fact_id TEXT PRIMARY KEY, forgotten_at TEXT NOT NULL,
    forgotten_by TEXT NOT NULL, reason TEXT NOT NULL, versions_erased INTEGER NOT NULL);
";

fn t(s: &str) -> SqlValue {
    SqlValue::Text(s.to_string())
}

fn sha(chemin: &Path) -> String {
    kindmother_heritage::empreinte_sha256(chemin).unwrap()
}

#[test]
fn import_unique_idempotent_et_sans_perte() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let ancienne = dir.path().join("alicia-memoire.db");
    let maintenant = Utc::now().to_rfc3339();
    let plus_tard = (Utc::now() + Duration::seconds(5)).to_rfc3339();
    let (f1, f2, oublie) = (
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
    );
    let (ep, sk, pc) = (
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
    );
    let fait = |id: uuid::Uuid, contenu: &str| -> (&'static str, Vec<(&'static str, SqlValue)>) {
        (
            "facts",
            vec![
                ("id", t(&id.to_string())),
                ("persona", t("voice")),
                ("subject", t("user")),
                ("content", t(contenu)),
                ("source", t("user")),
                ("confidence", SqlValue::Real(0.75)),
                ("created_at", t(&maintenant)),
                ("updated_at", t(&plus_tard)),
            ],
        )
    };
    let version =
        |id: uuid::Uuid, v: i64, contenu: &str| -> (&'static str, Vec<(&'static str, SqlValue)>) {
            (
                "fact_versions",
                vec![
                    ("fact_id", t(&id.to_string())),
                    ("version", SqlValue::Integer(v)),
                    ("persona", t("voice")),
                    ("subject", t("user")),
                    ("content", t(contenu)),
                    ("source", t("user")),
                    ("confidence", SqlValue::Real(0.75)),
                    ("recorded_at", t(&maintenant)),
                ],
            )
        };
    let lignes = vec![
        (
            "personas",
            vec![
                ("id", t("family")),
                ("display_name", t("Famille")),
                ("description", t("Le foyer")),
                ("is_default", SqlValue::Integer(1)),
                ("created_at", t(&maintenant)),
            ],
        ),
        fait(f1, "Jean prend le train"),
        version(f1, 1, "Jean prend le métro"),
        version(f1, 2, "Jean prend le train"),
        // Fait antérieur au versionnage : sans historique.
        fait(f2, "Le chat s'appelle Mochi"),
        (
            "fact_tombstones",
            vec![
                ("fact_id", t(&oublie.to_string())),
                ("forgotten_at", t(&maintenant)),
                ("forgotten_by", t("user")),
                ("reason", t("oubli")),
                ("versions_erased", SqlValue::Integer(3)),
            ],
        ),
        (
            "preferences",
            vec![
                ("scope", t("global")),
                ("key", t("active_persona")),
                ("value_json", t("\"family\"")),
                ("updated_at", t(&maintenant)),
            ],
        ),
        (
            "episodes",
            vec![
                ("id", t(&ep.to_string())),
                ("persona", t("family")),
                ("service_id", t("jaykoa")),
                ("intent", t("schedule_event")),
                ("user_phrase", t("ajoute le dîner de dimanche")),
                ("params_json", t("{\"jour\":\"dimanche\"}")),
                ("outcome", t("failed")),
                ("error", t("hors ligne")),
                ("created_at", t(&maintenant)),
            ],
        ),
        (
            "learned_skills",
            vec![
                ("id", t(&sk.to_string())),
                ("persona", t("family")),
                ("trigger_phrase", t("dîner dimanche")),
                ("qualified_id", t("jaykoa.schedule_event")),
                ("params_template", t("{}")),
                ("status", t("accepted")),
                ("created_at", t(&maintenant)),
                ("accepted_at", t(&maintenant)),
            ],
        ),
        (
            "pending_confirmations",
            vec![
                ("id", t(&pc.to_string())),
                ("qualified_id", t("jaykoa.schedule_event")),
                ("params_json", t("{}")),
                ("persona", t("family")),
                ("user_phrase", t("x")),
                ("prompt", t("Planifier ?")),
                ("status", t("pending")),
                ("created_at", t(&maintenant)),
                ("claimed_at", SqlValue::Integer(0)),
            ],
        ),
    ];
    kindmother_heritage::fabrique::creer_base_heritee(&ancienne, SCHEMA_HERITE, &lignes).unwrap();
    let empreinte = sha(&ancienne);

    // Ouverture avec l'ancienne base : import unique.
    let s = MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne)).unwrap();
    let verifier = |s: &MemoryStore| {
        assert_eq!(s.active_persona_id().unwrap().as_deref(), Some("family"));
        let h1 = s.fact_history(f1).unwrap();
        assert_eq!(
            h1.iter().map(|v| v.content.as_str()).collect::<Vec<_>>(),
            ["Jean prend le métro", "Jean prend le train"]
        );
        let courant = s.fact(f1).unwrap().unwrap();
        assert!((courant.confidence - 0.75).abs() < f32::EPSILON);
        // Version initiale créée pour le fait sans historique.
        let h2 = s.fact_history(f2).unwrap();
        assert_eq!(h2.len(), 1);
        assert_eq!(h2[0].version, 1);
        assert_eq!(s.recall("mochi", 5).unwrap().len(), 1);
        assert_eq!(
            s.fact_tombstone(oublie).unwrap().unwrap().versions_erased,
            3
        );
        let eps = s.recent_episodes(10).unwrap();
        assert_eq!(eps.len(), 1);
        assert_eq!(eps[0].error.as_deref(), Some("hors ligne"));
        assert_eq!(eps[0].params, json!({"jour": "dimanche"}));
        assert_eq!(s.search_episodes("diner", 5).unwrap().len(), 1);
        assert_eq!(s.active_learned_skills("family").unwrap().len(), 1);
        assert_eq!(s.list_pending_confirmations().unwrap().len(), 1);
    };
    verifier(&s);
    let marqueur = kindmother_heritage::chemin_marqueur_import(&ancienne);
    assert!(marqueur.exists(), "marqueur d'import écrit");
    assert!(ancienne.exists(), "ancien fichier jamais supprimé");
    assert_eq!(sha(&ancienne), empreinte, "ancien fichier intact");

    // Relances : aucun doublon, aucune perte.
    drop(s);
    let s = MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne)).unwrap();
    verifier(&s);
    let rapport = s.importer_base_heritee(&ancienne).unwrap();
    assert!(rapport.deja_importe);
    verifier(&s);
    assert_eq!(sha(&ancienne), empreinte);

    // Reprise forcée (marqueur retiré) : upserts sur les mêmes clés, sans doublon.
    std::fs::remove_file(&marqueur).unwrap();
    let rapport = s.importer_base_heritee(&ancienne).unwrap();
    assert!(!rapport.deja_importe);
    assert!(rapport.tables.iter().all(|t| t.ecrites == 0), "{rapport}");
    verifier(&s);
    assert_eq!(sha(&ancienne), empreinte);
}

#[test]
fn import_depuis_le_schema_initial_sans_versions_ni_colonnes_d_expiration() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let ancienne = dir.path().join("ancienne.db");
    let maintenant = Utc::now().to_rfc3339();
    let (fait, id_pending, id_exec) = (
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
        uuid::Uuid::new_v4(),
    );
    let pending = |id: uuid::Uuid, statut: &str| {
        (
            "pending_confirmations",
            vec![
                ("id", t(&id.to_string())),
                ("qualified_id", t("a.b")),
                ("params_json", t("{}")),
                ("persona", t("voice")),
                ("user_phrase", t("x")),
                ("prompt", t("p")),
                ("status", t(statut)),
                ("created_at", t(&maintenant)),
                ("resolved_at", SqlValue::Null),
            ],
        )
    };
    kindmother_heritage::fabrique::creer_base_heritee(
        &ancienne,
        "CREATE TABLE facts (
            id TEXT PRIMARY KEY, persona TEXT NOT NULL, subject TEXT NOT NULL,
            content TEXT NOT NULL, source TEXT NOT NULL, confidence REAL NOT NULL,
            created_at TEXT NOT NULL, updated_at TEXT NOT NULL);
         CREATE VIRTUAL TABLE facts_fts
            USING fts5(content, content='facts', content_rowid='rowid');
         CREATE TABLE pending_confirmations (
            id TEXT PRIMARY KEY, qualified_id TEXT NOT NULL, params_json TEXT NOT NULL,
            persona TEXT NOT NULL, user_phrase TEXT NOT NULL, prompt TEXT NOT NULL,
            status TEXT NOT NULL, created_at TEXT NOT NULL, resolved_at TEXT);",
        &[
            (
                "facts",
                vec![
                    ("id", t(&fait.to_string())),
                    ("persona", t("voice")),
                    ("subject", t("user")),
                    ("content", t("Jean aime le métro")),
                    ("source", t("user")),
                    ("confidence", SqlValue::Real(1.0)),
                    ("created_at", t(&maintenant)),
                    ("updated_at", t(&maintenant)),
                ],
            ),
            pending(id_pending, "pending"),
            pending(id_exec, "executing"),
        ],
    )
    .unwrap();

    let s = MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne)).unwrap();
    // EX-52 : version 1 créée depuis le contenu courant.
    let h = s.fact_history(fait).unwrap();
    assert_eq!(h.len(), 1);
    assert_eq!(h[0].content, "Jean aime le métro");
    assert_eq!(s.recall("métro", 5).unwrap().len(), 1);
    let mut f = s.fact(fait).unwrap().unwrap();
    f.content = "Jean aime le train".into();
    s.remember(&f).unwrap();
    assert_eq!(s.fact_history(fait).unwrap().len(), 2);

    // Colonnes absentes : claimed_at = 0 (date inconnue) ⇒ l'exécution héritée expire.
    let p = s.pending_confirmation(id_pending).unwrap().unwrap();
    assert_eq!(p.status, PendingStatus::Pending);
    assert!(p.reason.is_none());
    let rapport = s
        .expire_stale_pending_confirmations(TTL_EXEC, TTL_PENDING)
        .unwrap();
    assert_eq!(rapport.failed, vec![id_exec]);
    assert!(rapport.cancelled.is_empty());
    s.claim_pending_confirmation(id_pending).unwrap();
    drop(s);

    // Réouverture : idempotente.
    let s = MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne)).unwrap();
    assert_eq!(s.fact_history(fait).unwrap().len(), 2);
    assert_eq!(s.recall("train", 5).unwrap().len(), 1);
    assert!(s.recall("métro", 5).unwrap().is_empty());
    assert_eq!(
        s.pending_confirmation(id_pending).unwrap().unwrap().status,
        PendingStatus::Executing
    );
}

#[test]
fn ancienne_base_illisible_refusee_sans_marqueur_et_intacte() {
    let dir = tempfile::tempdir().unwrap();
    let (addr, creds) = serveur(dir.path());
    let ancienne = dir.path().join("abimee.db");
    std::fs::write(&ancienne, b"ceci n'est pas une base SQLite").unwrap();
    let r = MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne));
    assert!(
        matches!(r, Err(MemoryError::LegacyDatabase(_))),
        "{:?}",
        r.err()
    );
    assert!(!kindmother_heritage::chemin_marqueur_import(&ancienne).exists());
    assert_eq!(
        std::fs::read(&ancienne).unwrap(),
        b"ceci n'est pas une base SQLite"
    );

    // Abandon volontaire (marqueur posé par l'administrateur) : ouverture acceptée.
    std::fs::write(kindmother_heritage::chemin_marqueur_import(&ancienne), b"").unwrap();
    assert!(MemoryStore::connecter(&cfg(&addr, &creds).avec_base_heritee(&ancienne)).is_ok());
}
