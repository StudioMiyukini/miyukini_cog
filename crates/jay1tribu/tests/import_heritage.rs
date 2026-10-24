//! @id     jay1tribu_tests_import_heritage
//! @do     verifier_import_base_heritee_jay1tribu
//! @role   test
//! @layer  infra
//! @human  jay1tribu — import de la base SQLite héritée dans un KindMother éphémère :
//!         lignes identiques, orphelines écartées, marqueur, no-op à la relance,
//!         fichier intact, `open` accepté après import.

#![cfg(all(
    feature = "import-heritage",
    feature = "kindmother-only",
    not(feature = "legacy-sqlite")
))]

use std::path::PathBuf;

use jay1tribu::data::import_heritage::{importer_base_heritee, plan};
use jay1tribu::data::Jay1TribuDb;
use kindmother_client::heritage::{chemin_marqueur_import, refuser_si_base_heritee};
use kindmother_client::{KindMotherClient, OperatorCredential, SqlValue};
use kindmother_heritage::fabrique::{creer_base_heritee, verifier_dans_kindmother};

/// DDL du chemin hérité (`kindmother_db.rs`, `init_schema`), copié à l'identique.
const DDL_HERITE: &str = r"
CREATE TABLE IF NOT EXISTS friends (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    friend_cog_id TEXT NOT NULL,
    friend_pseudo TEXT,
    created_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tribes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    creator_cog_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS tribe_roles (
    id TEXT PRIMARY KEY,
    tribe_id TEXT NOT NULL,
    name TEXT NOT NULL,
    permissions_json TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (tribe_id) REFERENCES tribes(id)
);
CREATE TABLE IF NOT EXISTS tribe_members (
    id TEXT PRIMARY KEY,
    tribe_id TEXT NOT NULL,
    cog_id TEXT NOT NULL,
    role_id TEXT NOT NULL,
    joined_at TEXT NOT NULL,
    FOREIGN KEY (tribe_id) REFERENCES tribes(id),
    FOREIGN KEY (role_id) REFERENCES tribe_roles(id)
);
CREATE TABLE IF NOT EXISTS salons (
    id TEXT PRIMARY KEY,
    tribe_id TEXT,
    name TEXT NOT NULL,
    salon_type TEXT NOT NULL DEFAULT 'direct',
    created_at TEXT NOT NULL,
    FOREIGN KEY (tribe_id) REFERENCES tribes(id)
);
CREATE TABLE IF NOT EXISTS salon_members (
    salon_id TEXT NOT NULL,
    cog_id TEXT NOT NULL,
    joined_at TEXT NOT NULL,
    PRIMARY KEY (salon_id, cog_id),
    FOREIGN KEY (salon_id) REFERENCES salons(id)
);
CREATE TABLE IF NOT EXISTS messages (
    id TEXT PRIMARY KEY,
    salon_id TEXT NOT NULL,
    sender_cog_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (salon_id) REFERENCES salons(id)
);
CREATE TABLE IF NOT EXISTS pending_deliveries (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    recipient_cog_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id)
);
CREATE TABLE IF NOT EXISTS tribe_invitations (
    id TEXT PRIMARY KEY,
    tribe_id TEXT NOT NULL,
    inviter_cog_id TEXT NOT NULL,
    invitee_cog_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL,
    FOREIGN KEY (tribe_id) REFERENCES tribes(id)
);
CREATE TABLE IF NOT EXISTS message_attachments (
    id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    local_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (message_id) REFERENCES messages(id)
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_friends_profile_cog ON friends(profile_id, friend_cog_id);
CREATE INDEX IF NOT EXISTS idx_pending_deliveries_recipient ON pending_deliveries(recipient_cog_id);
CREATE INDEX IF NOT EXISTS idx_tribe_invitations_invitee ON tribe_invitations(invitee_cog_id);
CREATE INDEX IF NOT EXISTS idx_friends_profile ON friends(profile_id);
CREATE INDEX IF NOT EXISTS idx_tribe_members_tribe ON tribe_members(tribe_id);
CREATE INDEX IF NOT EXISTS idx_salons_tribe ON salons(tribe_id);
CREATE INDEX IF NOT EXISTS idx_messages_salon ON messages(salon_id);
";

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn t(s: &str) -> SqlValue {
    SqlValue::Text(s.to_string())
}

#[allow(dead_code)]
fn i(v: i64) -> SqlValue {
    SqlValue::Integer(v)
}

const N: SqlValue = SqlValue::Null;

/// Serveur KindMother éphémère, justificatif de l'opérateur, base héritée créée.
fn preparer(
    operateur: &str,
    fichier: &str,
    lignes: &[(&str, Vec<(&str, SqlValue)>)],
) -> (TempDir, String, PathBuf, OperatorCredential, PathBuf) {
    let tmp = TempDir(
        std::env::temp_dir().join(format!("{operateur}-heritage-{}", uuid::Uuid::new_v4())),
    );
    std::fs::create_dir_all(&tmp.0).expect("tempdir");
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(tmp.0.join("km")).expect("serveur éphémère");
    let cred = OperatorCredential::load_from_dir(operateur, &creds).expect("justificatif");
    let base = tmp.0.join(fichier);
    creer_base_heritee(&base, DDL_HERITE, lignes).expect("base héritée");
    (tmp, addr.to_string(), creds, cred, base)
}

/// Lignes héritées : deux orphelines (livraison et invitation dont le parent a été
/// supprimé par l'ancien chemin), un salon `tribe_id = ''`.
fn lignes() -> Vec<(&'static str, Vec<(&'static str, SqlValue)>)> {
    let d = || t("2025-05-01T10:00:00+00:00");
    vec![
        (
            "friends",
            vec![
                ("id", t("f1")),
                ("profile_id", t("p1")),
                ("friend_cog_id", t("cogA")),
                ("friend_pseudo", N),
                ("created_at", d()),
            ],
        ),
        (
            "friends",
            vec![
                ("id", t("f2")),
                ("profile_id", t("p1")),
                ("friend_cog_id", t("cogB")),
                ("friend_pseudo", t("")),
                ("created_at", d()),
            ],
        ),
        (
            "tribes",
            vec![
                ("id", t("t1")),
                ("name", t("Famille")),
                ("description", N),
                ("creator_cog_id", t("cogA")),
                ("created_at", d()),
                ("updated_at", d()),
            ],
        ),
        (
            "tribe_roles",
            vec![
                ("id", t("r1")),
                ("tribe_id", t("t1")),
                ("name", t("chef")),
                ("permissions_json", t("[\"*\"]")),
                ("created_at", d()),
            ],
        ),
        (
            "tribe_members",
            vec![
                ("id", t("tm1")),
                ("tribe_id", t("t1")),
                ("cog_id", t("cogA")),
                ("role_id", t("r1")),
                ("joined_at", d()),
            ],
        ),
        (
            "salons",
            vec![
                ("id", t("s1")),
                ("tribe_id", t("t1")),
                ("name", t("Général")),
                ("salon_type", t("tribe")),
                ("created_at", d()),
            ],
        ),
        (
            "salons",
            vec![
                ("id", t("s2")),
                ("tribe_id", t("")),
                ("name", t("Direct")),
                ("salon_type", t("direct")),
                ("created_at", d()),
            ],
        ),
        (
            "salon_members",
            vec![
                ("salon_id", t("s1")),
                ("cog_id", t("cogA")),
                ("joined_at", d()),
            ],
        ),
        (
            "salon_members",
            vec![
                ("salon_id", t("s2")),
                ("cog_id", t("cogB")),
                ("joined_at", d()),
            ],
        ),
        (
            "messages",
            vec![
                ("id", t("msg1")),
                ("salon_id", t("s1")),
                ("sender_cog_id", t("cogA")),
                ("content", t("Bonjour")),
                ("created_at", d()),
            ],
        ),
        (
            "pending_deliveries",
            vec![
                ("id", t("pd1")),
                ("message_id", t("msg1")),
                ("recipient_cog_id", t("cogB")),
                ("created_at", d()),
            ],
        ),
        (
            "pending_deliveries",
            vec![
                ("id", t("pd-orpheline")),
                ("message_id", t("msg-supprime")),
                ("recipient_cog_id", t("cogB")),
                ("created_at", d()),
            ],
        ),
        (
            "tribe_invitations",
            vec![
                ("id", t("inv1")),
                ("tribe_id", t("t1")),
                ("inviter_cog_id", t("cogA")),
                ("invitee_cog_id", t("cogC")),
                ("status", t("pending")),
                ("created_at", d()),
            ],
        ),
        (
            "tribe_invitations",
            vec![
                ("id", t("inv-orpheline")),
                ("tribe_id", t("t-supprimee")),
                ("inviter_cog_id", t("cogA")),
                ("invitee_cog_id", t("cogC")),
                ("status", t("pending")),
                ("created_at", d()),
            ],
        ),
        (
            "message_attachments",
            vec![
                ("id", t("att1")),
                ("message_id", t("msg1")),
                ("kind", t("image")),
                ("local_path", t("/tmp/a.png")),
                ("created_at", d()),
            ],
        ),
    ]
}

#[test]
fn import_base_heritee_jay1tribu() {
    let lignes = lignes();
    let (_tmp, addr, creds, cred, base) = preparer("jay1tribu", "jay1tribu.db", &lignes);
    let octets = std::fs::read(&base).expect("lecture");

    // Avant import : `open` refuse.
    assert!(Jay1TribuDb::open(&base).is_err(), "garde de bascule");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        let client = KindMotherClient::connect(&addr, &cred, "jay1tribu")
            .await
            .expect("client");
        let rapport = importer_base_heritee(&base, &client).await.expect("import");
        println!("{rapport}");
        assert_eq!(rapport.total_orphelines(), 2);
        let salons = rapport
            .tables
            .iter()
            .find(|t| t.table == "salons")
            .expect("salons");
        assert_eq!(salons.normalisees, 1, "tribe_id '' normalisé en NULL");

        // Lignes identiques (orphelines exclues, salon direct attendu avec tribe_id NULL).
        let attendues: Vec<_> = lignes
            .iter()
            .filter(|(_, l)| {
                !l.iter()
                    .any(|(_, v)| matches!(v, SqlValue::Text(s) if s.ends_with("-orpheline")))
            })
            .map(|(table, l)| {
                let l = l
                    .iter()
                    .map(|(c, v)| {
                        if *table == "salons" && *c == "tribe_id" && *v == t("") {
                            (*c, N)
                        } else {
                            (*c, v.clone())
                        }
                    })
                    .collect::<Vec<_>>();
                (*table, l)
            })
            .collect();
        let n = verifier_dans_kindmother(&client, &plan(), &attendues, &[])
            .await
            .expect("lignes identiques");
        assert_eq!(n, lignes.len() - 2);

        // Relance : no-op.
        let encore = importer_base_heritee(&base, &client)
            .await
            .expect("relance");
        assert!(encore.deja_importe);
    });

    assert!(chemin_marqueur_import(&base).is_file());
    assert_eq!(
        std::fs::read(&base).expect("lecture"),
        octets,
        "fichier hérité intact"
    );
    assert!(refuser_si_base_heritee(&base).is_ok());

    // Après import : `open` (client global, environnement) accepte et lit les données.
    std::env::set_var("KINDMOTHER_ADDR", &addr);
    std::env::set_var(kindmother_client::ENV_CREDENTIALS_DIR, &creds);
    let db = Jay1TribuDb::open(&base).expect("open après import");
    let amis = db.friend_list("p1").expect("amis");
    assert_eq!(amis.len(), 2);
    assert!(
        amis.iter().any(|a| a.friend_pseudo.is_none()),
        "NULL conservé"
    );
    assert!(
        amis.iter().any(|a| a.friend_pseudo.as_deref() == Some("")),
        "chaîne vide conservée"
    );
}
