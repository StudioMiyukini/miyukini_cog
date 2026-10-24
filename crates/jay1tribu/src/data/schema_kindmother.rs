//! @id     jay1tribu_schema_kindmother
//! @do     declarer_migrations_kindmother
//! @role   donnee
//! @layer  infra
//! @human  jay1tribu — schéma KindMother versionné, partagé par la persistance KindMother
//!         et par l'import de la base héritée (feature `import-heritage`).

//! Migrations versionnées de la base KindMother du service.
//!
//! Module séparé de la couche de persistance pour être compilé aussi avec la feature
//! `import-heritage`, y compris quand `legacy-sqlite` est activée par unification des
//! features d'un espace de travail (le chemin KindMother n'est alors pas compilé).

use kindmother::intent::SchemaMigration;

/// Schéma v1 : identique au schéma HÉRITÉ (`kindmother_db.rs`, référence), clés
/// étrangères et index compris. `IF NOT EXISTS` : une base déjà créée par l'ancien
/// client (DDL brut, sans version) est adoptée sans perte.
const SCHEMA_V1: &str = r"
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

/// Migrations versionnées de la base `jay1tribu`.
///
/// Évolutions futures : ajouter une nouvelle version en fin de liste (v2, v3…), ne
/// jamais modifier une version publiée ni supprimer de table de données (politique
/// non destructive appliquée par kindmother-service).
pub(crate) fn migrations() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "jay1tribu : schéma initial (amis, tribus, salons, messages, livraisons, invitations, pièces jointes)",
        &[SCHEMA_V1],
    )]
}
