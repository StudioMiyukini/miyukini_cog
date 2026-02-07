//! Base de données fille SQLite JayXpose sous autorité KindMother.
//!
//! Instance Daughter : persistance locale via SQLite, identité KindMother pour gouvernance.
//!
//! @id: jayxpose_kindmother_db
//! @do: persist_jayxpose_data_sqlite_kindmother
//! @layer: infra

use crate::data::types::{
    CategorieProduit, ConfidentialiteProfil, DocumentPartage, DocumentProfessionnel,
    DocumentVersion, ExposantProfile, ProduitCatalogue, ProduitVisuel, VitrinePage,
};
use kindmother::{InstanceIdentity, InstanceType};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Erreur de la base JayXpose (SQLite, sérialisation).
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayXpose DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError(e.to_string())
    }
}

/// Base de données fille SQLite de JayXpose (KindMother Daughter).
///
/// @id: jayxpose_db_struct
/// @do: hold_sqlite_connection_and_kindmother_identity
/// @layer: infra
pub struct JayXposeDb {
    conn: Mutex<Connection>,
    /// Identité KindMother : instance Fille (base locale).
    pub instance: InstanceIdentity,
}

impl JayXposeDb {
    /// Ouvre ou crée la base SQLite à `path` et initialise le schéma.
    /// Identité KindMother : Daughter (base fille).
    ///
    /// @id: jayxpose_db_open
    /// @do: open_or_create_sqlite_database
    /// @layer: infra
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Crée le schéma des tables JayXpose (8 tables).
    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(
            r#"
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
            "#,
        )?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exposants
    // -----------------------------------------------------------------------

    /// Insère ou remplace un profil exposant (INSERT OR REPLACE).
    ///
    /// @id: exposant_upsert
    /// @do: upsert_exposant_profile_to_db
    /// @layer: infra
    pub fn exposant_upsert(&self, exp: &ExposantProfile) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = exp
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created = exp.created_at.clone().unwrap_or_else(|| now.clone());
        conn.execute(
            "INSERT OR REPLACE INTO exposants (
                id, company_name, legal_form, slogan, description_short, description_long,
                stand_name, contact_email, contact_phone, adresse_siege, adresse_correspondance,
                contact_facturation_nom, contact_facturation_email, contact_facturation_phone,
                contact_logistique_nom, contact_logistique_email, contact_logistique_phone,
                logo_url, banner_url, site_web, siret, siren, code_ape, num_immatriculation,
                secteur, tags, social_facebook, social_instagram, social_linkedin, social_tiktok,
                social_youtube, social_pinterest, social_x, visible_annuaire, vitrine_slug,
                vitrine_status, vitrine_colors, seo_title, seo_description, seo_keywords,
                created_at, updated_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17,
                ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32,
                ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42
            )",
            params![
                id,
                exp.company_name,
                exp.legal_form,
                exp.slogan,
                exp.description_short,
                exp.description_long,
                exp.stand_name,
                exp.contact_email,
                exp.contact_phone,
                exp.adresse_siege,
                exp.adresse_correspondance,
                exp.contact_facturation_nom,
                exp.contact_facturation_email,
                exp.contact_facturation_phone,
                exp.contact_logistique_nom,
                exp.contact_logistique_email,
                exp.contact_logistique_phone,
                exp.logo_url,
                exp.banner_url,
                exp.site_web,
                exp.siret,
                exp.siren,
                exp.code_ape,
                exp.num_immatriculation,
                exp.secteur,
                exp.tags,
                exp.social_facebook,
                exp.social_instagram,
                exp.social_linkedin,
                exp.social_tiktok,
                exp.social_youtube,
                exp.social_pinterest,
                exp.social_x,
                exp.visible_annuaire.map(|b| if b { 1_i32 } else { 0_i32 }),
                exp.vitrine_slug,
                exp.vitrine_status,
                exp.vitrine_colors,
                exp.seo_title,
                exp.seo_description,
                exp.seo_keywords,
                created,
                now,
            ],
        )?;
        Ok(())
    }

    /// Récupère un profil exposant par identifiant.
    pub fn exposant_by_id(&self, id: &str) -> Result<Option<ExposantProfile>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, company_name, legal_form, slogan, description_short, description_long,
                    stand_name, contact_email, contact_phone, adresse_siege, adresse_correspondance,
                    contact_facturation_nom, contact_facturation_email, contact_facturation_phone,
                    contact_logistique_nom, contact_logistique_email, contact_logistique_phone,
                    logo_url, banner_url, site_web, siret, siren, code_ape, num_immatriculation,
                    secteur, tags, social_facebook, social_instagram, social_linkedin, social_tiktok,
                    social_youtube, social_pinterest, social_x, visible_annuaire, vitrine_slug,
                    vitrine_status, vitrine_colors, seo_title, seo_description, seo_keywords,
                    created_at, updated_at
             FROM exposants WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], Self::row_to_exposant);
        match row {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Liste les exposants visibles dans l'annuaire (WHERE `visible_annuaire` = 1).
    pub fn exposants_list_annuaire(&self) -> Result<Vec<ExposantProfile>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, company_name, legal_form, slogan, description_short, description_long,
                    stand_name, contact_email, contact_phone, adresse_siege, adresse_correspondance,
                    contact_facturation_nom, contact_facturation_email, contact_facturation_phone,
                    contact_logistique_nom, contact_logistique_email, contact_logistique_phone,
                    logo_url, banner_url, site_web, siret, siren, code_ape, num_immatriculation,
                    secteur, tags, social_facebook, social_instagram, social_linkedin, social_tiktok,
                    social_youtube, social_pinterest, social_x, visible_annuaire, vitrine_slug,
                    vitrine_status, vitrine_colors, seo_title, seo_description, seo_keywords,
                    created_at, updated_at
             FROM exposants WHERE visible_annuaire = 1 ORDER BY company_name",
        )?;
        let rows = stmt.query_map([], Self::row_to_exposant)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Cherche un exposant par contact_email et password_hash (pour auth).
    ///
    /// @id: exposant_by_email_password
    /// @do: find_exposant_by_email_and_password_hash
    /// @layer: infra
    pub fn exposant_by_email_password(
        &self,
        email: &str,
        password_hash: &str,
    ) -> Result<Option<ExposantProfile>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, company_name, legal_form, slogan, description_short, description_long,
                    stand_name, contact_email, contact_phone, adresse_siege, adresse_correspondance,
                    contact_facturation_nom, contact_facturation_email, contact_facturation_phone,
                    contact_logistique_nom, contact_logistique_email, contact_logistique_phone,
                    logo_url, banner_url, site_web, siret, siren, code_ape, num_immatriculation,
                    secteur, tags, social_facebook, social_instagram, social_linkedin, social_tiktok,
                    social_youtube, social_pinterest, social_x, visible_annuaire, vitrine_slug,
                    vitrine_status, vitrine_colors, seo_title, seo_description, seo_keywords,
                    created_at, updated_at
             FROM exposants WHERE contact_email = ?1 AND password_hash = ?2",
        )?;
        let row = stmt.query_row(params![email, password_hash], |row| {
            Self::row_to_exposant(row)
        });
        match row {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Cree un exposant minimal avec email, password_hash et company_name.
    /// Retourne l'identifiant UUID cree.
    ///
    /// @id: exposant_create
    /// @do: create_exposant_with_credentials
    /// @layer: infra
    pub fn exposant_create(
        &self,
        email: &str,
        password_hash: &str,
        company_name: &str,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO exposants (id, company_name, contact_email, password_hash, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, company_name, email, password_hash, now, now],
        )?;
        Ok(id)
    }

    /// Convertit une ligne SQLite en `ExposantProfile`.
    fn row_to_exposant(row: &rusqlite::Row<'_>) -> rusqlite::Result<ExposantProfile> {
        Ok(ExposantProfile {
            id: row.get(0)?,
            company_name: row.get(1)?,
            legal_form: row.get(2)?,
            slogan: row.get(3)?,
            description_short: row.get(4)?,
            description_long: row.get(5)?,
            stand_name: row.get(6)?,
            contact_email: row.get(7)?,
            contact_phone: row.get(8)?,
            adresse_siege: row.get(9)?,
            adresse_correspondance: row.get(10)?,
            contact_facturation_nom: row.get(11)?,
            contact_facturation_email: row.get(12)?,
            contact_facturation_phone: row.get(13)?,
            contact_logistique_nom: row.get(14)?,
            contact_logistique_email: row.get(15)?,
            contact_logistique_phone: row.get(16)?,
            logo_url: row.get(17)?,
            banner_url: row.get(18)?,
            site_web: row.get(19)?,
            siret: row.get(20)?,
            siren: row.get(21)?,
            code_ape: row.get(22)?,
            num_immatriculation: row.get(23)?,
            secteur: row.get(24)?,
            tags: row.get(25)?,
            social_facebook: row.get(26)?,
            social_instagram: row.get(27)?,
            social_linkedin: row.get(28)?,
            social_tiktok: row.get(29)?,
            social_youtube: row.get(30)?,
            social_pinterest: row.get(31)?,
            social_x: row.get(32)?,
            visible_annuaire: row.get::<_, Option<i32>>(33)?.map(|x| x != 0),
            vitrine_slug: row.get(34)?,
            vitrine_status: row.get(35)?,
            vitrine_colors: row.get(36)?,
            seo_title: row.get(37)?,
            seo_description: row.get(38)?,
            seo_keywords: row.get(39)?,
            created_at: row.get(40)?,
            updated_at: row.get(41)?,
        })
    }

    // -----------------------------------------------------------------------
    // Produits catalogue
    // -----------------------------------------------------------------------

    /// Insère un produit catalogue.
    pub fn produit_insert(&self, p: &ProduitCatalogue) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = p
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO produits_catalogue (id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                id,
                p.exposant_id,
                p.name,
                p.description,
                p.price,
                p.currency,
                p.category_id,
                p.availability,
                p.is_featured.map(|b| if b { 1_i32 } else { 0 }),
                p.sort_order,
                now,
                now,
            ],
        )?;
        Ok(())
    }

    /// Met à jour un produit catalogue existant.
    pub fn produit_update(&self, p: &ProduitCatalogue) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        let id = p.id.as_deref().ok_or_else(|| DbError("produit_update: id requis".to_string()))?;
        conn.execute(
            "UPDATE produits_catalogue SET exposant_id=?1, name=?2, description=?3, price=?4, currency=?5, category_id=?6, availability=?7, is_featured=?8, sort_order=?9, updated_at=?10 WHERE id=?11",
            params![
                p.exposant_id,
                p.name,
                p.description,
                p.price,
                p.currency,
                p.category_id,
                p.availability,
                p.is_featured.map(|b| if b { 1_i32 } else { 0 }),
                p.sort_order,
                now,
                id,
            ],
        )?;
        Ok(())
    }

    /// Supprime un produit catalogue par identifiant.
    pub fn produit_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM produits_catalogue WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Liste les produits d'un exposant.
    pub fn produits_by_exposant(&self, exposant_id: &str) -> Result<Vec<ProduitCatalogue>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at
             FROM produits_catalogue WHERE exposant_id = ?1 ORDER BY sort_order, name",
        )?;
        let rows = stmt.query_map(params![exposant_id], |row| {
            Ok(ProduitCatalogue {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                price: row.get(4)?,
                currency: row.get(5)?,
                category_id: row.get(6)?,
                availability: row.get(7)?,
                is_featured: row.get::<_, Option<i32>>(8)?.map(|x| x != 0),
                sort_order: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Récupère un produit catalogue par identifiant.
    pub fn produit_by_id(&self, id: &str) -> Result<Option<ProduitCatalogue>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at
             FROM produits_catalogue WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(ProduitCatalogue {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                price: row.get(4)?,
                currency: row.get(5)?,
                category_id: row.get(6)?,
                availability: row.get(7)?,
                is_featured: row.get::<_, Option<i32>>(8)?.map(|x| x != 0),
                sort_order: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        });
        match row {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // -----------------------------------------------------------------------
    // Catégories produits
    // -----------------------------------------------------------------------

    /// Insère une catégorie de produit.
    pub fn categorie_insert(&self, c: &CategorieProduit) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = c
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO categories_produits (id, exposant_id, name, description, sort_order, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, c.exposant_id, c.name, c.description, c.sort_order, now],
        )?;
        Ok(())
    }

    /// Met à jour une catégorie de produit existante.
    pub fn categorie_update(&self, c: &CategorieProduit) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = c.id.as_deref().ok_or_else(|| DbError("categorie_update: id requis".to_string()))?;
        conn.execute(
            "UPDATE categories_produits SET exposant_id=?1, name=?2, description=?3, sort_order=?4 WHERE id=?5",
            params![c.exposant_id, c.name, c.description, c.sort_order, id],
        )?;
        Ok(())
    }

    /// Supprime une catégorie de produit par identifiant.
    pub fn categorie_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM categories_produits WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Liste les catégories d'un exposant.
    pub fn categories_by_exposant(&self, exposant_id: &str) -> Result<Vec<CategorieProduit>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, name, description, sort_order, created_at
             FROM categories_produits WHERE exposant_id = ?1 ORDER BY sort_order, name",
        )?;
        let rows = stmt.query_map(params![exposant_id], |row| {
            Ok(CategorieProduit {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                sort_order: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // -----------------------------------------------------------------------
    // Visuels produits
    // -----------------------------------------------------------------------

    /// Insère un visuel de produit.
    pub fn visuel_insert(&self, v: &ProduitVisuel) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = v
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO produits_visuels (id, produit_id, url, alt_text, is_primary, sort_order, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                v.produit_id,
                v.url,
                v.alt_text,
                v.is_primary.map(|b| if b { 1_i32 } else { 0 }),
                v.sort_order,
                now,
            ],
        )?;
        Ok(())
    }

    /// Supprime un visuel de produit par identifiant.
    pub fn visuel_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM produits_visuels WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Liste les visuels d'un produit.
    pub fn visuels_by_produit(&self, produit_id: &str) -> Result<Vec<ProduitVisuel>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, produit_id, url, alt_text, is_primary, sort_order, created_at
             FROM produits_visuels WHERE produit_id = ?1 ORDER BY sort_order",
        )?;
        let rows = stmt.query_map(params![produit_id], |row| {
            Ok(ProduitVisuel {
                id: row.get(0)?,
                produit_id: row.get(1)?,
                url: row.get(2)?,
                alt_text: row.get(3)?,
                is_primary: row.get::<_, Option<i32>>(4)?.map(|x| x != 0),
                sort_order: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // -----------------------------------------------------------------------
    // Documents professionnels
    // -----------------------------------------------------------------------

    /// Insère un document professionnel.
    pub fn document_insert(&self, d: &DocumentProfessionnel) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = d
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO documents_professionnels (
                id, exposant_id, doc_type, label, file_url, file_name, file_size, mime_type,
                status, expires_at, version, notes, rejection_reason, uploaded_at,
                validated_at, validated_by, created_at, updated_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                id,
                d.exposant_id,
                d.doc_type,
                d.label,
                d.file_url,
                d.file_name,
                d.file_size,
                d.mime_type,
                d.status,
                d.expires_at,
                d.version,
                d.notes,
                d.rejection_reason,
                d.uploaded_at,
                d.validated_at,
                d.validated_by,
                now,
                now,
            ],
        )?;
        Ok(())
    }

    /// Met à jour un document professionnel existant.
    pub fn document_update(&self, d: &DocumentProfessionnel) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        let id = d.id.as_deref().ok_or_else(|| DbError("document_update: id requis".to_string()))?;
        conn.execute(
            "UPDATE documents_professionnels SET
                exposant_id=?1, doc_type=?2, label=?3, file_url=?4, file_name=?5, file_size=?6,
                mime_type=?7, status=?8, expires_at=?9, version=?10, notes=?11, rejection_reason=?12,
                uploaded_at=?13, validated_at=?14, validated_by=?15, updated_at=?16
             WHERE id=?17",
            params![
                d.exposant_id,
                d.doc_type,
                d.label,
                d.file_url,
                d.file_name,
                d.file_size,
                d.mime_type,
                d.status,
                d.expires_at,
                d.version,
                d.notes,
                d.rejection_reason,
                d.uploaded_at,
                d.validated_at,
                d.validated_by,
                now,
                id,
            ],
        )?;
        Ok(())
    }

    /// Supprime un document professionnel par identifiant.
    pub fn document_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM documents_professionnels WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Liste les documents professionnels d'un exposant.
    pub fn documents_by_exposant(&self, exposant_id: &str) -> Result<Vec<DocumentProfessionnel>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, doc_type, label, file_url, file_name, file_size, mime_type,
                    status, expires_at, version, notes, rejection_reason, uploaded_at,
                    validated_at, validated_by, created_at, updated_at
             FROM documents_professionnels WHERE exposant_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![exposant_id], Self::row_to_document)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Récupère un document professionnel par identifiant.
    pub fn document_by_id(&self, id: &str) -> Result<Option<DocumentProfessionnel>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, doc_type, label, file_url, file_name, file_size, mime_type,
                    status, expires_at, version, notes, rejection_reason, uploaded_at,
                    validated_at, validated_by, created_at, updated_at
             FROM documents_professionnels WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], Self::row_to_document);
        match row {
            Ok(d) => Ok(Some(d)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    /// Convertit une ligne SQLite en `DocumentProfessionnel`.
    fn row_to_document(row: &rusqlite::Row<'_>) -> rusqlite::Result<DocumentProfessionnel> {
        Ok(DocumentProfessionnel {
            id: row.get(0)?,
            exposant_id: row.get(1)?,
            doc_type: row.get(2)?,
            label: row.get(3)?,
            file_url: row.get(4)?,
            file_name: row.get(5)?,
            file_size: row.get(6)?,
            mime_type: row.get(7)?,
            status: row.get(8)?,
            expires_at: row.get(9)?,
            version: row.get(10)?,
            notes: row.get(11)?,
            rejection_reason: row.get(12)?,
            uploaded_at: row.get(13)?,
            validated_at: row.get(14)?,
            validated_by: row.get(15)?,
            created_at: row.get(16)?,
            updated_at: row.get(17)?,
        })
    }

    // -----------------------------------------------------------------------
    // Documents versions
    // -----------------------------------------------------------------------

    /// Insère une version de document.
    pub fn document_version_insert(&self, v: &DocumentVersion) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = v
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO documents_versions (id, document_id, version, file_url, file_name, file_size, uploaded_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                v.document_id,
                v.version,
                v.file_url,
                v.file_name,
                v.file_size,
                v.uploaded_at.clone().unwrap_or(now),
            ],
        )?;
        Ok(())
    }

    /// Liste les versions d'un document.
    pub fn document_versions_by_document(&self, document_id: &str) -> Result<Vec<DocumentVersion>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, document_id, version, file_url, file_name, file_size, uploaded_at
             FROM documents_versions WHERE document_id = ?1 ORDER BY version DESC",
        )?;
        let rows = stmt.query_map(params![document_id], |row| {
            Ok(DocumentVersion {
                id: row.get(0)?,
                document_id: row.get(1)?,
                version: row.get(2)?,
                file_url: row.get(3)?,
                file_name: row.get(4)?,
                file_size: row.get(5)?,
                uploaded_at: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // -----------------------------------------------------------------------
    // Documents partages
    // -----------------------------------------------------------------------

    /// Insère un partage de document.
    pub fn partage_insert(&self, p: &DocumentPartage) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = p
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO documents_partages (
                id, document_id, exposant_id, target_user_id, target_context_type,
                target_context_id, status, message, requested_at, responded_at,
                revoked_at, expires_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                id,
                p.document_id,
                p.exposant_id,
                p.target_user_id,
                p.target_context_type,
                p.target_context_id,
                p.status,
                p.message,
                p.requested_at.clone().unwrap_or(now),
                p.responded_at,
                p.revoked_at,
                p.expires_at,
            ],
        )?;
        Ok(())
    }

    /// Met à jour le statut d'un partage de document.
    pub fn partage_update_status(&self, id: &str, status: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE documents_partages SET status = ?1, responded_at = ?2 WHERE id = ?3",
            params![status, now, id],
        )?;
        Ok(())
    }

    /// Liste les partages de documents d'un exposant.
    pub fn partages_by_exposant(&self, exposant_id: &str) -> Result<Vec<DocumentPartage>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, document_id, exposant_id, target_user_id, target_context_type,
                    target_context_id, status, message, requested_at, responded_at,
                    revoked_at, expires_at
             FROM documents_partages WHERE exposant_id = ?1 ORDER BY requested_at DESC",
        )?;
        let rows = stmt.query_map(params![exposant_id], |row| {
            Ok(DocumentPartage {
                id: row.get(0)?,
                document_id: row.get(1)?,
                exposant_id: row.get(2)?,
                target_user_id: row.get(3)?,
                target_context_type: row.get(4)?,
                target_context_id: row.get(5)?,
                status: row.get(6)?,
                message: row.get(7)?,
                requested_at: row.get(8)?,
                responded_at: row.get(9)?,
                revoked_at: row.get(10)?,
                expires_at: row.get(11)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // -----------------------------------------------------------------------
    // Vitrine pages
    // -----------------------------------------------------------------------

    /// Insère ou remplace une page vitrine (INSERT OR REPLACE).
    pub fn vitrine_page_upsert(&self, v: &VitrinePage) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = v
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT OR REPLACE INTO vitrine_pages (id, exposant_id, page_type, content, is_visible, sort_order, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id,
                v.exposant_id,
                v.page_type,
                v.content,
                v.is_visible.map(|b| if b { 1_i32 } else { 0 }),
                v.sort_order,
                now,
            ],
        )?;
        Ok(())
    }

    /// Liste les pages vitrine d'un exposant.
    pub fn vitrine_pages_by_exposant(&self, exposant_id: &str) -> Result<Vec<VitrinePage>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, page_type, content, is_visible, sort_order, updated_at
             FROM vitrine_pages WHERE exposant_id = ?1 ORDER BY sort_order",
        )?;
        let rows = stmt.query_map(params![exposant_id], |row| {
            Ok(VitrinePage {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                page_type: row.get(2)?,
                content: row.get(3)?,
                is_visible: row.get::<_, Option<i32>>(4)?.map(|x| x != 0),
                sort_order: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // -----------------------------------------------------------------------
    // Confidentialité profil
    // -----------------------------------------------------------------------

    /// Insère ou remplace une règle de confidentialité (INSERT OR REPLACE).
    pub fn confidentialite_upsert(&self, c: &ConfidentialiteProfil) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let id = c
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT OR REPLACE INTO confidentialite_profil (id, exposant_id, field_name, visibility, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, c.exposant_id, c.field_name, c.visibility, now],
        )?;
        Ok(())
    }

    /// Liste les règles de confidentialité d'un exposant.
    pub fn confidentialite_by_exposant(&self, exposant_id: &str) -> Result<Vec<ConfidentialiteProfil>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, field_name, visibility, updated_at
             FROM confidentialite_profil WHERE exposant_id = ?1 ORDER BY field_name",
        )?;
        let rows = stmt.query_map(params![exposant_id], |row| {
            Ok(ConfidentialiteProfil {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                field_name: row.get(2)?,
                visibility: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }
}
