-- Schéma JayXpose pour KindMother
-- @id: jayxpose_schema_sql
-- @do: define_jayxpose_database_schema
-- @layer: infra

CREATE TABLE IF NOT EXISTS exposants (
    id TEXT PRIMARY KEY,
    company_name TEXT,
    legal_form TEXT,
    slogan TEXT,
    description_short TEXT,
    description_long TEXT,
    stand_name TEXT,
    contact_email TEXT,
    contact_phone TEXT,
    adresse_siege TEXT,
    adresse_correspondance TEXT,
    contact_facturation_nom TEXT,
    contact_facturation_email TEXT,
    contact_facturation_phone TEXT,
    contact_logistique_nom TEXT,
    contact_logistique_email TEXT,
    contact_logistique_phone TEXT,
    logo_url TEXT,
    banner_url TEXT,
    site_web TEXT,
    siret TEXT,
    siren TEXT,
    code_ape TEXT,
    num_immatriculation TEXT,
    secteur TEXT,
    tags TEXT,
    social_facebook TEXT,
    social_instagram TEXT,
    social_linkedin TEXT,
    social_tiktok TEXT,
    social_youtube TEXT,
    social_pinterest TEXT,
    social_x TEXT,
    visible_annuaire INTEGER,
    vitrine_slug TEXT,
    vitrine_status TEXT,
    vitrine_colors TEXT,
    seo_title TEXT,
    seo_description TEXT,
    seo_keywords TEXT,
    created_at TEXT,
    updated_at TEXT,
    password_hash TEXT
);

CREATE TABLE IF NOT EXISTS produits_catalogue (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    name TEXT,
    description TEXT,
    price REAL,
    currency TEXT,
    category_id TEXT,
    availability TEXT,
    is_featured INTEGER,
    sort_order INTEGER,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS categories_produits (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    name TEXT,
    description TEXT,
    sort_order INTEGER,
    created_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS produits_visuels (
    id TEXT PRIMARY KEY,
    produit_id TEXT,
    url TEXT,
    alt_text TEXT,
    is_primary INTEGER,
    sort_order INTEGER,
    created_at TEXT,
    FOREIGN KEY (produit_id) REFERENCES produits_catalogue(id)
);

CREATE TABLE IF NOT EXISTS documents_professionnels (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    doc_type TEXT,
    label TEXT,
    file_url TEXT,
    file_name TEXT,
    file_size INTEGER,
    mime_type TEXT,
    status TEXT,
    expires_at TEXT,
    version INTEGER,
    notes TEXT,
    rejection_reason TEXT,
    uploaded_at TEXT,
    validated_at TEXT,
    validated_by TEXT,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS documents_versions (
    id TEXT PRIMARY KEY,
    document_id TEXT,
    version INTEGER,
    file_url TEXT,
    file_name TEXT,
    file_size INTEGER,
    uploaded_at TEXT,
    FOREIGN KEY (document_id) REFERENCES documents_professionnels(id)
);

CREATE TABLE IF NOT EXISTS documents_partages (
    id TEXT PRIMARY KEY,
    document_id TEXT,
    exposant_id TEXT,
    target_user_id TEXT,
    target_context_type TEXT,
    target_context_id TEXT,
    status TEXT,
    message TEXT,
    requested_at TEXT,
    responded_at TEXT,
    revoked_at TEXT,
    expires_at TEXT,
    FOREIGN KEY (document_id) REFERENCES documents_professionnels(id),
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS vitrine_pages (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    page_type TEXT,
    content TEXT,
    is_visible INTEGER,
    sort_order INTEGER,
    updated_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS confidentialite_profil (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    field_name TEXT,
    visibility TEXT,
    updated_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE TABLE IF NOT EXISTS vitrine_blocs (
    id TEXT PRIMARY KEY,
    page_id TEXT NOT NULL,
    block_key TEXT NOT NULL,
    block_type TEXT NOT NULL,
    props_json TEXT NOT NULL,
    position INTEGER NOT NULL,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (page_id) REFERENCES vitrine_pages(id)
);

CREATE TABLE IF NOT EXISTS vitrine_templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    site_type TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    content_json TEXT NOT NULL,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT,
    updated_at TEXT
);

CREATE TABLE IF NOT EXISTS sync_logs (
    id TEXT PRIMARY KEY,
    exposant_id TEXT,
    sync_source TEXT NOT NULL,
    sync_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload_json TEXT,
    error_message TEXT,
    created_at TEXT
);

CREATE TABLE IF NOT EXISTS pos_stock_links (
    id TEXT PRIMARY KEY,
    produit_id TEXT NOT NULL UNIQUE,
    pos_sku TEXT NOT NULL,
    stock_qty INTEGER NOT NULL DEFAULT 0,
    last_sync_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (produit_id) REFERENCES produits_catalogue(id)
);

CREATE INDEX IF NOT EXISTS idx_vitrine_blocs_page ON vitrine_blocs(page_id, position);

CREATE INDEX IF NOT EXISTS idx_sync_logs_exposant ON sync_logs(exposant_id, created_at);

CREATE INDEX IF NOT EXISTS idx_pos_stock_sku ON pos_stock_links(pos_sku);

CREATE TABLE IF NOT EXISTS cms_articles (
    id TEXT PRIMARY KEY,
    exposant_id TEXT NOT NULL,
    title TEXT,
    slug TEXT,
    excerpt TEXT,
    content TEXT,
    article_type TEXT DEFAULT 'article',
    status TEXT DEFAULT 'brouillon',
    category_id TEXT,
    tags_json TEXT,
    cover_image_url TEXT,
    cover_image_alt TEXT,
    seo_title TEXT,
    seo_description TEXT,
    seo_keywords TEXT,
    is_featured INTEGER DEFAULT 0,
    allow_comments INTEGER DEFAULT 1,
    published_at TEXT,
    author_name TEXT,
    view_count INTEGER DEFAULT 0,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id),
    FOREIGN KEY (category_id) REFERENCES cms_categories(id)
);

CREATE TABLE IF NOT EXISTS cms_categories (
    id TEXT PRIMARY KEY,
    exposant_id TEXT NOT NULL,
    name TEXT NOT NULL,
    slug TEXT,
    description TEXT,
    color TEXT,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT,
    FOREIGN KEY (exposant_id) REFERENCES exposants(id)
);

CREATE INDEX IF NOT EXISTS idx_cms_articles_exposant ON cms_articles(exposant_id, status);

CREATE INDEX IF NOT EXISTS idx_cms_articles_slug ON cms_articles(exposant_id, slug);

CREATE INDEX IF NOT EXISTS idx_cms_categories_exposant ON cms_categories(exposant_id)
