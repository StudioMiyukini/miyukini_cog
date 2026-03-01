//! Bases de contexte — mémoire structurée pour les agents.
//!
//! Permet de stocker et récupérer des données de contexte pour enrichir
//! les prompts des agents : préférences utilisateur, historique, codebase,
//! documentation, discussions passées.
//!
//! Chaque base de contexte est un ensemble de documents textuels indexés
//! par catégorie, optimisés pour être injectés dans les prompts LLM.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ── Types ────────────────────────────────────────────────────────────

/// Catégorie de contexte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextCategory {
    /// Préférences et profil utilisateur.
    UserPreferences,
    /// Contexte personnel (famille, santé, habitudes).
    Personal,
    /// Base de code (fichiers importants, architecture).
    Codebase,
    /// Documentation technique.
    Documentation,
    /// Historique des discussions (résumés compressés).
    ConversationHistory,
    /// Données métier spécifiques (projet, client, etc.).
    Business,
    /// Formation et pédagogie (fiches élève, programme).
    Education,
    /// Autre.
    Custom,
}

/// Un document de contexte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDocument {
    /// Identifiant unique du document.
    pub id: String,
    /// Titre court.
    pub title: String,
    /// Contenu textuel (optimisé pour injection prompt).
    pub content: String,
    /// Catégorie.
    pub category: ContextCategory,
    /// Tags pour la recherche rapide.
    pub tags: Vec<String>,
    /// Nombre de tokens estimé (1 token ≈ 4 chars).
    pub estimated_tokens: u32,
    /// Date de dernière mise à jour (ISO 8601).
    pub updated_at: String,
    /// Priorité (plus haut = injecté en premier). 0-100.
    pub priority: u8,
}

/// Une base de contexte (collection de documents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBase {
    /// Identifiant unique.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// Description.
    pub description: String,
    /// Documents.
    pub documents: Vec<ContextDocument>,
    /// Budget token maximum pour cette base (0 = illimité).
    pub token_budget: u32,
    /// Date de création.
    pub created_at: String,
}

impl ContextBase {
    /// Nombre total de tokens estimé.
    pub fn total_tokens(&self) -> u32 {
        self.documents.iter().map(|d| d.estimated_tokens).sum()
    }

    /// Sélectionne les documents les plus pertinents dans le budget token.
    pub fn select_within_budget(&self, max_tokens: u32) -> Vec<&ContextDocument> {
        let mut docs: Vec<&ContextDocument> = self.documents.iter().collect();
        // Tri par priorité décroissante
        docs.sort_by(|a, b| b.priority.cmp(&a.priority));

        let mut selected = Vec::new();
        let mut remaining = max_tokens;

        for doc in docs {
            if doc.estimated_tokens <= remaining {
                remaining -= doc.estimated_tokens;
                selected.push(doc);
            }
        }

        selected
    }

    /// Recherche les documents par tag.
    pub fn search_by_tag(&self, tag: &str) -> Vec<&ContextDocument> {
        let tag_lower = tag.to_lowercase();
        self.documents
            .iter()
            .filter(|d| d.tags.iter().any(|t| t.to_lowercase().contains(&tag_lower)))
            .collect()
    }

    /// Recherche les documents par contenu (substring).
    pub fn search_by_content(&self, query: &str) -> Vec<&ContextDocument> {
        let query_lower = query.to_lowercase();
        self.documents
            .iter()
            .filter(|d| {
                d.title.to_lowercase().contains(&query_lower)
                    || d.content.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

/// Résumé d'une base (pour les endpoints GET).
#[derive(Debug, Clone, Serialize)]
pub struct ContextBaseSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub document_count: usize,
    pub total_tokens: u32,
    pub token_budget: u32,
    pub created_at: String,
}

impl From<&ContextBase> for ContextBaseSummary {
    fn from(base: &ContextBase) -> Self {
        Self {
            id: base.id.clone(),
            name: base.name.clone(),
            description: base.description.clone(),
            document_count: base.documents.len(),
            total_tokens: base.total_tokens(),
            token_budget: base.token_budget,
            created_at: base.created_at.clone(),
        }
    }
}

// ── Requêtes CRUD ────────────────────────────────────────────────────

/// Requête de création d'une base de contexte.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateContextRequest {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub documents: Vec<ContextDocument>,
    #[serde(default)]
    pub token_budget: u32,
}

/// Requête de mise à jour.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContextRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub token_budget: Option<u32>,
    /// Ajouter des documents.
    #[serde(default)]
    pub add_documents: Vec<ContextDocument>,
    /// Supprimer des documents par ID.
    #[serde(default)]
    pub remove_document_ids: Vec<String>,
}

// ── Registre ─────────────────────────────────────────────────────────

/// Registre des bases de contexte.
#[derive(Debug, Clone)]
pub struct ContextRegistry {
    bases: Arc<RwLock<HashMap<String, ContextBase>>>,
}

impl ContextRegistry {
    pub fn new() -> Self {
        Self {
            bases: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Liste toutes les bases (résumés).
    pub fn list_bases(&self) -> Vec<ContextBaseSummary> {
        let bases = self.bases.read().unwrap();
        bases.values().map(ContextBaseSummary::from).collect()
    }

    /// Récupère une base complète.
    pub fn get_base(&self, id: &str) -> Option<ContextBase> {
        let bases = self.bases.read().unwrap();
        bases.get(id).cloned()
    }

    /// Crée une base.
    pub fn create_base(&self, req: CreateContextRequest) -> ContextBase {
        let id = if req.id.is_empty() {
            format!("ctx_{}", &uuid_simple()[..8])
        } else {
            req.id
        };

        let base = ContextBase {
            id: id.clone(),
            name: req.name,
            description: req.description,
            documents: req.documents,
            token_budget: req.token_budget,
            created_at: now_iso(),
        };

        let mut bases = self.bases.write().unwrap();
        bases.insert(id, base.clone());
        base
    }

    /// Met à jour une base.
    pub fn update_base(&self, id: &str, req: UpdateContextRequest) -> Option<ContextBase> {
        let mut bases = self.bases.write().unwrap();
        let base = bases.get_mut(id)?;

        if let Some(name) = req.name {
            base.name = name;
        }
        if let Some(desc) = req.description {
            base.description = desc;
        }
        if let Some(budget) = req.token_budget {
            base.token_budget = budget;
        }

        // Supprimer les documents demandés
        base.documents
            .retain(|d| !req.remove_document_ids.contains(&d.id));

        // Ajouter les nouveaux documents
        for doc in req.add_documents {
            base.documents.push(doc);
        }

        Some(base.clone())
    }

    /// Supprime une base.
    pub fn delete_base(&self, id: &str) -> bool {
        let mut bases = self.bases.write().unwrap();
        bases.remove(id).is_some()
    }

    /// Compile le contexte pour un agent : sélectionne les meilleurs documents
    /// dans le budget token depuis les bases spécifiées.
    pub fn compile_context(&self, base_ids: &[String], max_tokens: u32) -> String {
        let bases = self.bases.read().unwrap();
        let mut all_docs: Vec<(&ContextDocument, &str)> = Vec::new();

        for base_id in base_ids {
            if let Some(base) = bases.get(base_id) {
                for doc in &base.documents {
                    all_docs.push((doc, &base.name));
                }
            }
        }

        // Tri par priorité décroissante
        all_docs.sort_by(|a, b| b.0.priority.cmp(&a.0.priority));

        let mut result = String::new();
        let mut remaining = max_tokens;

        for (doc, base_name) in all_docs {
            if doc.estimated_tokens > remaining {
                continue;
            }
            remaining -= doc.estimated_tokens;
            result.push_str(&format!("## [{base_name}] {}\n{}\n\n", doc.title, doc.content));
        }

        result
    }
}

/// UUID simple (sans tirets, 16 chars hex).
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{nanos:032x}")
}

/// Date ISO 8601 actuelle.
fn now_iso() -> String {
    // Format simplifié sans dépendance chrono
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

/// Estime le nombre de tokens d'un texte (~4 chars/token).
pub fn estimate_tokens(text: &str) -> u32 {
    (text.len() as u32 + 3) / 4
}
