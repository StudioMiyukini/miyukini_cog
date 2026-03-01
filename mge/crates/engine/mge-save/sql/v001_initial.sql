-- v001_initial.sql
-- Schema initial de la couche persistance Sodomight.
-- Tables : accounts, characters, items, character_skills, waypoints, quest_flags, game_sessions.

-- Comptes
CREATE TABLE IF NOT EXISTS accounts (
    id          TEXT PRIMARY KEY,   -- UUID v4
    username    TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,    -- bcrypt
    email       TEXT NOT NULL UNIQUE,
    created_at  TEXT NOT NULL,      -- ISO 8601
    last_login  TEXT,
    is_banned   INTEGER NOT NULL DEFAULT 0,
    ban_reason  TEXT
);

-- Personnages
CREATE TABLE IF NOT EXISTS characters (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    class           TEXT NOT NULL,  -- "Soignante", "Seigneur Ombre", etc.
    level           INTEGER NOT NULL DEFAULT 1,
    experience      INTEGER NOT NULL DEFAULT 0,
    strength        INTEGER NOT NULL DEFAULT 10,
    dexterity       INTEGER NOT NULL DEFAULT 10,
    vitality        INTEGER NOT NULL DEFAULT 10,
    energy          INTEGER NOT NULL DEFAULT 10,
    unspent_stats   INTEGER NOT NULL DEFAULT 0,
    current_life    INTEGER NOT NULL DEFAULT 80,
    max_life        INTEGER NOT NULL DEFAULT 80,
    current_mana    INTEGER NOT NULL DEFAULT 20,
    max_mana        INTEGER NOT NULL DEFAULT 20,
    gold            INTEGER NOT NULL DEFAULT 0,
    zone_id         TEXT NOT NULL DEFAULT 'rogue_encampment',
    pos_x           REAL NOT NULL DEFAULT 0.0,
    pos_y           REAL NOT NULL DEFAULT 0.0,
    created_at      TEXT NOT NULL,
    last_played     TEXT,
    UNIQUE(account_id, name)
);
CREATE INDEX IF NOT EXISTS idx_characters_account ON characters(account_id);

-- Items (stockage JSON blob pour les affixes variables)
CREATE TABLE IF NOT EXISTS items (
    id              TEXT PRIMARY KEY,
    owner_id        TEXT NOT NULL,  -- character_id ou stash_id
    owner_type      TEXT NOT NULL,  -- "character_inventory", "character_equipped", "stash"
    slot            TEXT,           -- "head", "chest", "main_hand", NULL si inventaire
    grid_x          INTEGER,
    grid_y          INTEGER,
    item_data       TEXT NOT NULL,  -- JSON : base_item_id, quality, affixes[], socketed[], quantity
    created_at      TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_items_owner ON items(owner_id, owner_type);

-- Competences actives (points depenses)
CREATE TABLE IF NOT EXISTS character_skills (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    skill_id        TEXT NOT NULL,  -- ref vers TOML skills data
    points          INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (character_id, skill_id)
);

-- Waypoints debloques
CREATE TABLE IF NOT EXISTS waypoints (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    act             INTEGER NOT NULL,
    waypoint_id     TEXT NOT NULL,
    unlocked_at     TEXT NOT NULL,
    PRIMARY KEY (character_id, act, waypoint_id)
);

-- Flags de quetes
CREATE TABLE IF NOT EXISTS quest_flags (
    character_id    TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    quest_id        TEXT NOT NULL,
    state           TEXT NOT NULL,  -- "active" | "complete" | "failed"
    updated_at      TEXT NOT NULL,
    PRIMARY KEY (character_id, quest_id)
);

-- Sessions de jeu
CREATE TABLE IF NOT EXISTS game_sessions (
    id              TEXT PRIMARY KEY,
    host_account_id TEXT NOT NULL REFERENCES accounts(id),
    difficulty      TEXT NOT NULL,  -- "normal" | "nightmare" | "hell"
    act             INTEGER NOT NULL DEFAULT 1,
    started_at      TEXT NOT NULL,
    ended_at        TEXT,
    player_count    INTEGER NOT NULL DEFAULT 1
);
