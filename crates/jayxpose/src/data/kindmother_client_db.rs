//! Base de données JayXpose via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! @id: jayxpose_kindmother_client_db
//! @do: persist_jayxpose_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{
    CategorieProduit, CmsArticle, CmsCategory, ConfidentialiteProfil, DocumentPartage,
    DocumentProfessionnel, DocumentVersion, ExposantProfile, PosStockLink, ProduitCatalogue,
    ProduitVisuel, SyncLog, VitrineBlock, VitrinePage, VitrineTemplate,
};
use kindmother_client::{KindMotherClient, ClientError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;
use thiserror::Error;

// Note: Ce fichier n'est compilé que si le feature "kindmother-only" est activé

/// Erreur de la base JayXpose (KindMother Client).
#[derive(Error, Debug)]
pub enum DbError {
    /// Erreur du client KindMother.
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),
    
    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),
    
    /// Client non initialisé.
    #[error("Client not initialized")]
    NotInitialized,
}

/// Adresse par défaut du service KindMother.
const DEFAULT_KINDMOTHER_ADDR: &str = "http://[::1]:50051";

/// Client global partagé (singleton).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Base de données JayXpose via KindMother Client.
///
/// @id: jayxpose_db_client_struct
/// @do: hold_kindmother_client_connection
/// @layer: infra
#[derive(Clone)]
pub struct JayXposeDb {
    client: Arc<KindMotherClient>,
}

impl JayXposeDb {
    /// Initialise la connexion globale au service KindMother.
    ///
    /// Cette fonction doit être appelée une fois au démarrage de l'application.
    pub async fn init_global(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "jayxpose", "jayxpose").await?;
        
        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;
        
        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT
            .get()
            .ok_or(DbError::NotInitialized)?
            .clone();
        
        Ok(Self { client })
    }

    /// Crée une nouvelle instance avec une connexion personnalisée.
    pub async fn connect(addr: &str) -> Result<Self, DbError> {
        let client = KindMotherClient::connect(addr, "jayxpose", "jayxpose").await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Initialise le schéma de la base de données.
    ///
    /// Le schéma est créé côté KindMother Service, cette méthode
    /// envoie les requêtes DDL pour créer les tables si elles n'existent pas.
    pub async fn init_schema(&self) -> Result<(), DbError> {
        // Le schéma est créé via des requêtes DDL envoyées à KindMother
        let schema_sql = include_str!("schema.sql");
        
        // Exécuter le schéma en une transaction
        let statements: Vec<&str> = schema_sql
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        
        for sql in statements {
            self.client
                .execute(sql, Vec::<String>::new(), "init_schema")
                .await
                .map_err(DbError::from)?;
        }
        
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Exposants
    // -----------------------------------------------------------------------

    /// Insère ou remplace un profil exposant.
    pub async fn exposant_upsert(&self, exp: &ExposantProfile) -> Result<(), DbError> {
        let id = exp
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created = exp.created_at.clone().unwrap_or_else(|| now.clone());

        self.client
            .execute(
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
                vec![
                    id,
                    exp.company_name.clone().unwrap_or_default(),
                    exp.legal_form.clone().unwrap_or_default(),
                    exp.slogan.clone().unwrap_or_default(),
                    exp.description_short.clone().unwrap_or_default(),
                    exp.description_long.clone().unwrap_or_default(),
                    exp.stand_name.clone().unwrap_or_default(),
                    exp.contact_email.clone().unwrap_or_default(),
                    exp.contact_phone.clone().unwrap_or_default(),
                    exp.adresse_siege.clone().unwrap_or_default(),
                    exp.adresse_correspondance.clone().unwrap_or_default(),
                    exp.contact_facturation_nom.clone().unwrap_or_default(),
                    exp.contact_facturation_email.clone().unwrap_or_default(),
                    exp.contact_facturation_phone.clone().unwrap_or_default(),
                    exp.contact_logistique_nom.clone().unwrap_or_default(),
                    exp.contact_logistique_email.clone().unwrap_or_default(),
                    exp.contact_logistique_phone.clone().unwrap_or_default(),
                    exp.logo_url.clone().unwrap_or_default(),
                    exp.banner_url.clone().unwrap_or_default(),
                    exp.site_web.clone().unwrap_or_default(),
                    exp.siret.clone().unwrap_or_default(),
                    exp.siren.clone().unwrap_or_default(),
                    exp.code_ape.clone().unwrap_or_default(),
                    exp.num_immatriculation.clone().unwrap_or_default(),
                    exp.secteur.clone().unwrap_or_default(),
                    exp.tags.clone().unwrap_or_default(),
                    exp.social_facebook.clone().unwrap_or_default(),
                    exp.social_instagram.clone().unwrap_or_default(),
                    exp.social_linkedin.clone().unwrap_or_default(),
                    exp.social_tiktok.clone().unwrap_or_default(),
                    exp.social_youtube.clone().unwrap_or_default(),
                    exp.social_pinterest.clone().unwrap_or_default(),
                    exp.social_x.clone().unwrap_or_default(),
                    exp.visible_annuaire.map(|b| if b { "1" } else { "0" }).unwrap_or("0").to_string(),
                    exp.vitrine_slug.clone().unwrap_or_default(),
                    exp.vitrine_status.clone().unwrap_or_default(),
                    exp.vitrine_colors.clone().unwrap_or_default(),
                    exp.seo_title.clone().unwrap_or_default(),
                    exp.seo_description.clone().unwrap_or_default(),
                    exp.seo_keywords.clone().unwrap_or_default(),
                    created,
                    now,
                ],
                "exposant_upsert",
            )
            .await?;
        
        Ok(())
    }

    /// Récupère un profil exposant par identifiant.
    pub async fn exposant_by_id(&self, id: &str) -> Result<Option<ExposantProfile>, DbError> {
        let rows = self.client
            .query(
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
                vec![id],
            )
            .await?;
        
        rows.into_iter()
            .next()
            .map(|row| Self::row_to_exposant(&row))
            .transpose()
    }

    /// Liste les exposants visibles dans l'annuaire.
    pub async fn exposants_list_annuaire(&self) -> Result<Vec<ExposantProfile>, DbError> {
        let rows = self.client
            .query(
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
                Vec::<String>::new(),
            )
            .await?;
        
        rows.iter()
            .map(|row| Self::row_to_exposant(row))
            .collect()
    }

    /// Retourne le premier slug de vitrine publiée (pour la carte Home COG / lien Découvrir).
    pub async fn first_published_vitrine_slug(&self) -> Result<Option<String>, DbError> {
        let rows = self.client
            .query(
                "SELECT vitrine_slug FROM exposants WHERE vitrine_status = 'publiee'
                 AND vitrine_slug IS NOT NULL AND trim(vitrine_slug) != '' ORDER BY company_name LIMIT 1",
                vec![],
            )
            .await?;
        let first = rows.into_iter().next();
        Ok(first.and_then(|r| Self::get_opt_string(&r, "vitrine_slug")))
    }

    /// Récupère un exposant par slug vitrine, uniquement si la vitrine est publiée.
    /// Utilisé par le service WEB (Origin) pour servir les pages vitrine publiques.
    pub async fn exposant_by_vitrine_slug(&self, slug: &str) -> Result<Option<ExposantProfile>, DbError> {
        let rows = self.client
            .query(
                "SELECT id, company_name, legal_form, slogan, description_short, description_long,
                        stand_name, contact_email, contact_phone, adresse_siege, adresse_correspondance,
                        contact_facturation_nom, contact_facturation_email, contact_facturation_phone,
                        contact_logistique_nom, contact_logistique_email, contact_logistique_phone,
                        logo_url, banner_url, site_web, siret, siren, code_ape, num_immatriculation,
                        secteur, tags, social_facebook, social_instagram, social_linkedin, social_tiktok,
                        social_youtube, social_pinterest, social_x, visible_annuaire, vitrine_slug,
                        vitrine_status, vitrine_colors, seo_title, seo_description, seo_keywords,
                        created_at, updated_at
                 FROM exposants WHERE vitrine_slug = ?1 AND vitrine_status = 'publiee'",
                vec![slug],
            )
            .await?;
        rows.into_iter()
            .next()
            .map(|row| Self::row_to_exposant(&row))
            .transpose()
    }

    /// Cherche un exposant par email et password hash.
    pub async fn exposant_by_email_password(
        &self,
        email: &str,
        password_hash: &str,
    ) -> Result<Option<ExposantProfile>, DbError> {
        let rows = self.client
            .query(
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
                vec![email, password_hash],
            )
            .await?;
        
        rows.into_iter()
            .next()
            .map(|row| Self::row_to_exposant(&row))
            .transpose()
    }

    /// Crée un exposant minimal avec email, password_hash et company_name.
    pub async fn exposant_create(
        &self,
        email: &str,
        password_hash: &str,
        company_name: &str,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        self.client
            .execute(
                "INSERT INTO exposants (id, company_name, contact_email, password_hash, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                vec![&id, company_name, email, password_hash, &now, &now],
                "exposant_create",
            )
            .await?;
        
        Ok(id)
    }

    /// Convertit une ligne en ExposantProfile.
    fn row_to_exposant(row: &HashMap<String, serde_json::Value>) -> Result<ExposantProfile, DbError> {
        Ok(ExposantProfile {
            id: Self::get_opt_string(row, "id"),
            company_name: Self::get_opt_string(row, "company_name"),
            legal_form: Self::get_opt_string(row, "legal_form"),
            slogan: Self::get_opt_string(row, "slogan"),
            description_short: Self::get_opt_string(row, "description_short"),
            description_long: Self::get_opt_string(row, "description_long"),
            stand_name: Self::get_opt_string(row, "stand_name"),
            contact_email: Self::get_opt_string(row, "contact_email"),
            contact_phone: Self::get_opt_string(row, "contact_phone"),
            adresse_siege: Self::get_opt_string(row, "adresse_siege"),
            adresse_correspondance: Self::get_opt_string(row, "adresse_correspondance"),
            contact_facturation_nom: Self::get_opt_string(row, "contact_facturation_nom"),
            contact_facturation_email: Self::get_opt_string(row, "contact_facturation_email"),
            contact_facturation_phone: Self::get_opt_string(row, "contact_facturation_phone"),
            contact_logistique_nom: Self::get_opt_string(row, "contact_logistique_nom"),
            contact_logistique_email: Self::get_opt_string(row, "contact_logistique_email"),
            contact_logistique_phone: Self::get_opt_string(row, "contact_logistique_phone"),
            logo_url: Self::get_opt_string(row, "logo_url"),
            banner_url: Self::get_opt_string(row, "banner_url"),
            site_web: Self::get_opt_string(row, "site_web"),
            siret: Self::get_opt_string(row, "siret"),
            siren: Self::get_opt_string(row, "siren"),
            code_ape: Self::get_opt_string(row, "code_ape"),
            num_immatriculation: Self::get_opt_string(row, "num_immatriculation"),
            secteur: Self::get_opt_string(row, "secteur"),
            tags: Self::get_opt_string(row, "tags"),
            social_facebook: Self::get_opt_string(row, "social_facebook"),
            social_instagram: Self::get_opt_string(row, "social_instagram"),
            social_linkedin: Self::get_opt_string(row, "social_linkedin"),
            social_tiktok: Self::get_opt_string(row, "social_tiktok"),
            social_youtube: Self::get_opt_string(row, "social_youtube"),
            social_pinterest: Self::get_opt_string(row, "social_pinterest"),
            social_x: Self::get_opt_string(row, "social_x"),
            visible_annuaire: Self::get_opt_bool(row, "visible_annuaire"),
            vitrine_slug: Self::get_opt_string(row, "vitrine_slug"),
            vitrine_status: Self::get_opt_string(row, "vitrine_status"),
            vitrine_colors: Self::get_opt_string(row, "vitrine_colors"),
            seo_title: Self::get_opt_string(row, "seo_title"),
            seo_description: Self::get_opt_string(row, "seo_description"),
            seo_keywords: Self::get_opt_string(row, "seo_keywords"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Produits catalogue
    // -----------------------------------------------------------------------

    /// Insère un produit catalogue.
    pub async fn produit_insert(&self, p: &ProduitCatalogue) -> Result<(), DbError> {
        let id = p.id.clone().unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();
        
        self.client
            .execute(
                "INSERT INTO produits_catalogue (id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                vec![
                    id,
                    p.exposant_id.clone().unwrap_or_default(),
                    p.name.clone().unwrap_or_default(),
                    p.description.clone().unwrap_or_default(),
                    p.price.map(|v| v.to_string()).unwrap_or_default(),
                    p.currency.clone().unwrap_or_default(),
                    p.category_id.clone().unwrap_or_default(),
                    p.availability.clone().unwrap_or_default(),
                    p.is_featured.map(|b| if b { "1" } else { "0" }).unwrap_or("0").to_string(),
                    p.sort_order.map(|v| v.to_string()).unwrap_or("0".to_string()),
                    now.clone(),
                    now,
                ],
                "produit_insert",
            )
            .await?;
        
        Ok(())
    }

    /// Met à jour un produit catalogue.
    pub async fn produit_update(&self, p: &ProduitCatalogue) -> Result<(), DbError> {
        let id = p.id.as_ref().ok_or_else(|| DbError::Internal("produit_update: id requis".to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        
        self.client
            .execute(
                "UPDATE produits_catalogue SET exposant_id=?1, name=?2, description=?3, price=?4, currency=?5, category_id=?6, availability=?7, is_featured=?8, sort_order=?9, updated_at=?10 WHERE id=?11",
                vec![
                    p.exposant_id.clone().unwrap_or_default(),
                    p.name.clone().unwrap_or_default(),
                    p.description.clone().unwrap_or_default(),
                    p.price.map(|v| v.to_string()).unwrap_or_default(),
                    p.currency.clone().unwrap_or_default(),
                    p.category_id.clone().unwrap_or_default(),
                    p.availability.clone().unwrap_or_default(),
                    p.is_featured.map(|b| if b { "1" } else { "0" }).unwrap_or("0").to_string(),
                    p.sort_order.map(|v| v.to_string()).unwrap_or("0".to_string()),
                    now,
                    id.clone(),
                ],
                "produit_update",
            )
            .await?;
        
        Ok(())
    }

    /// Supprime un produit catalogue.
    pub async fn produit_delete(&self, id: &str) -> Result<(), DbError> {
        self.client
            .execute(
                "DELETE FROM produits_catalogue WHERE id = ?1",
                vec![id],
                "produit_delete",
            )
            .await?;
        
        Ok(())
    }

    /// Liste les produits d'un exposant.
    pub async fn produits_by_exposant(&self, exposant_id: &str) -> Result<Vec<ProduitCatalogue>, DbError> {
        let rows = self.client
            .query(
                "SELECT id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at
                 FROM produits_catalogue WHERE exposant_id = ?1 ORDER BY sort_order, name",
                vec![exposant_id],
            )
            .await?;
        
        rows.iter()
            .map(|row| Self::row_to_produit(row))
            .collect()
    }

    /// Récupère un produit par identifiant.
    pub async fn produit_by_id(&self, id: &str) -> Result<Option<ProduitCatalogue>, DbError> {
        let rows = self.client
            .query(
                "SELECT id, exposant_id, name, description, price, currency, category_id, availability, is_featured, sort_order, created_at, updated_at
                 FROM produits_catalogue WHERE id = ?1",
                vec![id],
            )
            .await?;
        
        rows.into_iter()
            .next()
            .map(|row| Self::row_to_produit(&row))
            .transpose()
    }

    fn row_to_produit(row: &HashMap<String, serde_json::Value>) -> Result<ProduitCatalogue, DbError> {
        Ok(ProduitCatalogue {
            id: Self::get_opt_string(row, "id"),
            exposant_id: Self::get_opt_string(row, "exposant_id"),
            name: Self::get_opt_string(row, "name"),
            description: Self::get_opt_string(row, "description"),
            price: Self::get_opt_f64(row, "price"),
            currency: Self::get_opt_string(row, "currency"),
            category_id: Self::get_opt_string(row, "category_id"),
            availability: Self::get_opt_string(row, "availability"),
            is_featured: Self::get_opt_bool(row, "is_featured"),
            sort_order: Self::get_opt_i32(row, "sort_order"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Helpers pour extraction de valeurs
    // -----------------------------------------------------------------------

    fn get_opt_string(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    }

    fn get_opt_bool(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<bool> {
        row.get(key)
            .and_then(|v| {
                if let Some(i) = v.as_i64() {
                    Some(i != 0)
                } else if let Some(b) = v.as_bool() {
                    Some(b)
                } else {
                    None
                }
            })
    }

    fn get_opt_i32(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<i32> {
        row.get(key)
            .and_then(|v| v.as_i64())
            .map(|i| i as i32)
    }

    fn get_opt_i64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<i64> {
        row.get(key).and_then(|v| v.as_i64())
    }

    fn get_opt_f64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<f64> {
        row.get(key).and_then(|v| v.as_f64())
    }

    // -----------------------------------------------------------------------
    // Autres méthodes (à implémenter sur le même modèle)
    // -----------------------------------------------------------------------

    /// Vérification de la santé de la connexion.
    pub async fn health_check(&self) -> Result<bool, DbError> {
        self.client.health_check().await.map_err(DbError::from)
    }
}
