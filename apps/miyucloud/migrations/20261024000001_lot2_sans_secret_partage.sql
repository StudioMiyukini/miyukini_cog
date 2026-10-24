-- Lot 2 (sortie) : plus aucune authentification par secret partagé.
--
-- 1. Liaison « identité Miyukini » → compte MiyuCloud. Un visa (audience
--    `miyucloud`) émis par le COG d'origine épinglé prouve un sujet par paire
--    (`sujet`, 32 o) dans l'espace du COG `cog_id`. Le compte n'est accessible
--    par visa qu'après une liaison explicite faite par l'utilisateur connecté.
CREATE TABLE IF NOT EXISTS auth.miyu_identities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    cog_id TEXT NOT NULL,
    sujet BYTEA NOT NULL CHECK (octet_length(sujet) = 32),
    label VARCHAR(255) NOT NULL DEFAULT 'Identité Miyukini',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP WITH TIME ZONE,
    UNIQUE (cog_id, sujet)
);
CREATE INDEX IF NOT EXISTS idx_miyu_identities_user_id ON auth.miyu_identities(user_id);

-- 2. Clés d'appareil (fin du flux RFC 8628) : l'appareil signe chaque requête
--    avec sa clé Ed25519 ; le serveur n'en garde que la clé publique.
CREATE TABLE IF NOT EXISTS auth.device_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    public_key BYTEA NOT NULL UNIQUE CHECK (octet_length(public_key) = 32),
    client_name VARCHAR(255) NOT NULL DEFAULT 'Unknown Client',
    scopes VARCHAR(512) NOT NULL DEFAULT 'webdav,caldav,carddav',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at TIMESTAMP WITH TIME ZONE,
    revoked BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX IF NOT EXISTS idx_device_keys_user_id ON auth.device_keys(user_id);

-- 3. Le flux d'appareil ne transporte plus de jeton : clé publique de
--    l'appareil à la place ; les jetons en clair restants sont effacés.
ALTER TABLE auth.device_codes ADD COLUMN IF NOT EXISTS device_public_key BYTEA
    CHECK (device_public_key IS NULL OR octet_length(device_public_key) = 32);
ALTER TABLE auth.device_codes DROP COLUMN IF EXISTS access_token;
ALTER TABLE auth.device_codes DROP COLUMN IF EXISTS refresh_token;

