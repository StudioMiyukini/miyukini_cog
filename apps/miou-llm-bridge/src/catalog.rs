//! Catalogue statique de modèles GGUF recommandés pour LM Studio.
//!
//! Chaque entrée définit un modèle avec ses exigences hardware et un score
//! de qualité. Le moteur de recommandation (`recommend.rs`) filtre et classe
//! ces entrées en fonction du hardware détecté et des modèles disponibles.

use serde::{Deserialize, Serialize};

/// Entrée du catalogue — un modèle GGUF recommandé.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
    /// Pattern (substring, case-insensitive) pour matcher les IDs LM Studio.
    /// Ex: "smollm2-1.7b" matche "lmstudio-community/SmolLM2-1.7B-Instruct-GGUF"
    pub match_pattern: &'static str,
    /// Nom affiché.
    pub display_name: &'static str,
    /// Nombre de paramètres (ex: "1.7B", "7B").
    pub params: &'static str,
    /// Quantization préférée.
    pub quantization: &'static str,
    /// RAM minimum requise (Mo).
    pub ram_required_mb: u64,
    /// VRAM préférée pour l'offload GPU (Mo). 0 = CPU-only OK.
    pub vram_preferred_mb: u64,
    /// Score qualité (0-100). Plus haut = meilleure qualité.
    pub quality_score: u8,
    /// Score français/multilingue (0-10).
    pub french_score: u8,
    /// Tier hardware minimum (1-5).
    pub min_tier: u8,
    /// Description courte.
    pub description: &'static str,
}

/// Catalogue statique — ordonné par quality_score croissant.
pub static CATALOG: &[CatalogEntry] = &[
    // ── Tier 1 : Ultra-Light (4-8 GB RAM) ─────────────────────────
    CatalogEntry {
        match_pattern: "smollm2-1.7b",
        display_name: "SmolLM2 1.7B Instruct",
        params: "1.7B",
        quantization: "Q4_K_M",
        ram_required_mb: 2_048,
        vram_preferred_mb: 0,
        quality_score: 25,
        french_score: 5,
        min_tier: 1,
        description: "Modèle ultra-léger, français basique, rapide sur CPU",
    },
    CatalogEntry {
        match_pattern: "qwen2.5-1.5b",
        display_name: "Qwen2.5 1.5B Instruct",
        params: "1.5B",
        quantization: "Q4_K_M",
        ram_required_mb: 2_048,
        vram_preferred_mb: 0,
        quality_score: 28,
        french_score: 7,
        min_tier: 1,
        description: "Qwen multilingue, bon français pour sa taille",
    },
    // ── Tier 2 : Light (8-16 GB RAM) ──────────────────────────────
    CatalogEntry {
        match_pattern: "ministral-3b",
        display_name: "Ministral 3B Instruct",
        params: "3B",
        quantization: "Q4_K_M",
        ram_required_mb: 3_072,
        vram_preferred_mb: 0,
        quality_score: 40,
        french_score: 8,
        min_tier: 2,
        description: "Mistral petit, excellent français",
    },
    CatalogEntry {
        match_pattern: "gemma-3-4b",
        display_name: "Gemma 3 4B Instruct",
        params: "4B",
        quantization: "Q4_K_M",
        ram_required_mb: 4_096,
        vram_preferred_mb: 2_048,
        quality_score: 45,
        french_score: 6,
        min_tier: 2,
        description: "Google Gemma 3, bon multilingue (140+ langues)",
    },
    // ── Tier 3 : Standard (16 GB RAM, 4-8 GB VRAM) ───────────────
    CatalogEntry {
        match_pattern: "mistral-7b",
        display_name: "Mistral 7B Instruct",
        params: "7B",
        quantization: "Q4_K_M",
        ram_required_mb: 6_144,
        vram_preferred_mb: 4_096,
        quality_score: 58,
        french_score: 9,
        min_tier: 3,
        description: "Mistral 7B, excellent français, rapide",
    },
    CatalogEntry {
        match_pattern: "qwen2.5-7b",
        display_name: "Qwen2.5 7B Instruct",
        params: "7B",
        quantization: "Q4_K_M",
        ram_required_mb: 6_144,
        vram_preferred_mb: 4_096,
        quality_score: 60,
        french_score: 8,
        min_tier: 3,
        description: "Qwen 7B, fort raisonnement et multilingue",
    },
    CatalogEntry {
        match_pattern: "llama-3",
        display_name: "Llama 3.2 8B Instruct",
        params: "8B",
        quantization: "Q4_K_M",
        ram_required_mb: 6_656,
        vram_preferred_mb: 5_120,
        quality_score: 62,
        french_score: 7,
        min_tier: 3,
        description: "Meta Llama 3, fort usage général",
    },
    // ── Tier 4 : Power (32+ GB RAM, 8-12 GB VRAM) ────────────────
    CatalogEntry {
        match_pattern: "mistral-nemo",
        display_name: "Mistral Nemo 12B",
        params: "12B",
        quantization: "Q4_K_M",
        ram_required_mb: 8_192,
        vram_preferred_mb: 8_192,
        quality_score: 72,
        french_score: 9,
        min_tier: 4,
        description: "Mistral Nemo, français remarquable",
    },
    CatalogEntry {
        match_pattern: "qwen3-14b",
        display_name: "Qwen3 14B",
        params: "14B",
        quantization: "Q4_K_M",
        ram_required_mb: 10_240,
        vram_preferred_mb: 8_192,
        quality_score: 78,
        french_score: 8,
        min_tier: 4,
        description: "Qwen3 14B, excellent raisonnement et multilingue",
    },
    // ── Tier 5 : Heavy (64+ GB RAM, 16+ GB VRAM) ─────────────────
    CatalogEntry {
        match_pattern: "qwen3-32b",
        display_name: "Qwen3 32B",
        params: "32B",
        quantization: "Q4_K_M",
        ram_required_mb: 20_480,
        vram_preferred_mb: 16_384,
        quality_score: 88,
        french_score: 9,
        min_tier: 5,
        description: "Qwen3 32B, qualité proche frontière",
    },
    CatalogEntry {
        match_pattern: "llama-3.1-70b",
        display_name: "Llama 3.1 70B",
        params: "70B",
        quantization: "Q4_K_M",
        ram_required_mb: 40_960,
        vram_preferred_mb: 32_768,
        quality_score: 92,
        french_score: 8,
        min_tier: 5,
        description: "Llama 70B, qualité classe frontière",
    },
];
