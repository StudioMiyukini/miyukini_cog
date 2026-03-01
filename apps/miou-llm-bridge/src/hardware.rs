//! Détection hardware du host — CPU, RAM, GPU/VRAM.
//!
//! Utilise `sysinfo` pour CPU/RAM et `wmic` (Windows) pour le GPU.
//! Le résultat est caché une fois au démarrage dans `ProxyState`.

use serde::{Deserialize, Serialize};
use sysinfo::System;

/// Informations hardware détectées sur la machine hôte.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    /// RAM totale en Mo.
    pub ram_total_mb: u64,
    /// RAM disponible en Mo (snapshot au moment de la détection).
    pub ram_available_mb: u64,
    /// GPU discret détecté (None si aucun ou intégré uniquement).
    pub gpu: Option<GpuInfo>,
    /// Tier hardware calculé (1-5).
    pub tier: HardwareTier,
}

/// Informations GPU.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    /// VRAM dédiée en Mo (0 si inconnue).
    pub vram_mb: u64,
    /// Vendeur : "nvidia", "amd", "intel", "unknown".
    pub vendor: String,
}

/// Tier hardware pour la sélection de modèle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardwareTier {
    /// 4-8 GB RAM, pas de GPU
    UltraLight,
    /// 8-16 GB RAM, pas/faible GPU
    Light,
    /// 16 GB RAM, 4-8 GB VRAM
    Standard,
    /// 32+ GB RAM, 8-12 GB VRAM
    Power,
    /// 64+ GB RAM, 16+ GB VRAM
    Heavy,
}

impl HardwareTier {
    pub fn label(&self) -> &'static str {
        match self {
            Self::UltraLight => "Ultra-Light (Tier 1)",
            Self::Light => "Light (Tier 2)",
            Self::Standard => "Standard (Tier 3)",
            Self::Power => "Power (Tier 4)",
            Self::Heavy => "Heavy (Tier 5)",
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::UltraLight => 1,
            Self::Light => 2,
            Self::Standard => 3,
            Self::Power => 4,
            Self::Heavy => 5,
        }
    }
}

/// Détecte les specs hardware de la machine courante.
pub fn detect_hardware() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown CPU".into());
    let cpu_cores = sys.physical_core_count().unwrap_or(1) as u32;
    let cpu_threads = sys.cpus().len() as u32;
    let ram_total_mb = sys.total_memory() / (1024 * 1024);
    let ram_available_mb = sys.available_memory() / (1024 * 1024);

    let gpu = detect_gpu();

    let tier = compute_tier(ram_total_mb, &gpu);

    HardwareInfo {
        cpu_name,
        cpu_cores,
        cpu_threads,
        ram_total_mb,
        ram_available_mb,
        gpu,
        tier,
    }
}

/// Détecte le GPU via `wmic` sur Windows.
#[cfg(target_os = "windows")]
fn detect_gpu() -> Option<GpuInfo> {
    let output = std::process::Command::new("wmic")
        .args([
            "path",
            "win32_VideoController",
            "get",
            "Name,AdapterRAM,AdapterCompatibility",
            "/format:csv",
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Format CSV : Node,AdapterCompatibility,AdapterRAM,Name
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("Node") {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            continue;
        }

        let compat = parts[1].trim();
        let adapter_ram = parts[2].trim();
        let name = parts[3].trim();

        // Ignorer les adaptateurs virtuels / basiques
        if name.is_empty()
            || name.contains("Microsoft")
            || name.contains("Basic")
            || name.contains("Virtual")
        {
            continue;
        }

        let vram_bytes: u64 = adapter_ram.parse().unwrap_or(0);
        let vram_mb = vram_bytes / (1024 * 1024);

        let compat_lower = compat.to_lowercase();
        let vendor = if compat_lower.contains("nvidia") {
            "nvidia"
        } else if compat_lower.contains("amd") || compat_lower.contains("ati") {
            "amd"
        } else if compat_lower.contains("intel") {
            "intel"
        } else {
            "unknown"
        };

        return Some(GpuInfo {
            name: name.to_string(),
            vram_mb,
            vendor: vendor.to_string(),
        });
    }

    None
}

/// Fallback pour les plateformes non-Windows.
#[cfg(not(target_os = "windows"))]
fn detect_gpu() -> Option<GpuInfo> {
    None
}

/// Calcule le tier hardware à partir de la RAM totale et du GPU.
fn compute_tier(ram_total_mb: u64, gpu: &Option<GpuInfo>) -> HardwareTier {
    let ram_gb = ram_total_mb / 1024;
    let vram_gb = gpu.as_ref().map(|g| g.vram_mb / 1024).unwrap_or(0);

    match (ram_gb, vram_gb) {
        (r, v) if r >= 64 && v >= 16 => HardwareTier::Heavy,
        (r, v) if r >= 32 && v >= 8 => HardwareTier::Power,
        (r, v) if r >= 16 && v >= 4 => HardwareTier::Standard,
        (r, _) if r >= 8 => HardwareTier::Light,
        _ => HardwareTier::UltraLight,
    }
}
