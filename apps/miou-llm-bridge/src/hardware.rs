//! @id     miou_llm_bridge_hardware
//! @do     definir_hardware_info
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « hardware ».

//! Détection hardware du host — CPU, RAM, GPU/VRAM.
//!
//! Utilise `sysinfo` pour CPU/RAM et `wmic` (Windows) pour le GPU.
//! Le résultat est caché une fois au démarrage dans `ProxyState`.

use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

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
    pub fn label(self) -> &'static str {
        match self {
            Self::UltraLight => "Ultra-Light (Tier 1)",
            Self::Light => "Light (Tier 2)",
            Self::Standard => "Standard (Tier 3)",
            Self::Power => "Power (Tier 4)",
            Self::Heavy => "Heavy (Tier 5)",
        }
    }

    pub fn number(self) -> u8 {
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
    // CPU + mémoire seulement : `new_all`/`refresh_all` parcourraient aussi tous les
    // processus (/proc/N/stat, /proc/N/task/), inutiles ici et refusés par AppArmor.
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );

    let cpu_name = sys
        .cpus()
        .first()
        .map_or_else(|| "Unknown CPU".into(), |c| c.brand().to_string());
    let cpu_cores = sys.physical_core_count().unwrap_or(1) as u32;
    let cpu_threads = sys.cpus().len() as u32;
    let ram_total_mb = sys.total_memory() / (1024 * 1024);
    let ram_available_mb = sys.available_memory() / (1024 * 1024);

    let gpu = detect_gpu();

    let tier = compute_tier(ram_total_mb, gpu.as_ref());

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
/// Préfère le GPU discret (NVIDIA > AMD > Intel) sur les laptops hybrides.
#[cfg(target_os = "windows")]
fn detect_gpu() -> Option<GpuInfo> {
    // 1. Essayer nvidia-smi en priorité — plus fiable pour les dGPU NVIDIA sur laptop
    if let Some(gpu) = detect_gpu_nvidia_smi() {
        return Some(gpu);
    }

    // 2. Fallback wmic — itère tous les adaptateurs et préfère NVIDIA > AMD > Intel
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

    let mut best: Option<GpuInfo> = None;

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

        if name.is_empty()
            || name.contains("Microsoft")
            || name.contains("Basic")
            || name.contains("Virtual")
            || name.contains("Remote")
        {
            continue;
        }

        let vram_bytes: u64 = adapter_ram.parse().unwrap_or(0);
        let vram_mb = vram_bytes / (1024 * 1024);

        let name_lower = name.to_lowercase();
        let compat_lower = compat.to_lowercase();
        let vendor = if name_lower.contains("nvidia") || compat_lower.contains("nvidia") {
            "nvidia"
        } else if name_lower.contains("amd")
            || name_lower.contains("radeon")
            || compat_lower.contains("amd")
        {
            "amd"
        } else if name_lower.contains("intel") || compat_lower.contains("intel") {
            "intel"
        } else {
            "unknown"
        };

        let gpu = GpuInfo {
            name: name.to_string(),
            vram_mb,
            vendor: vendor.to_string(),
        };

        match vendor {
            // NVIDIA discret = priorité maximale → retour immédiat
            "nvidia" => return Some(gpu),
            // AMD discret > Intel intégré
            "amd" => {
                if best.as_ref().map(|g| g.vendor.as_str()) != Some("amd") {
                    best = Some(gpu);
                }
            }
            _ => {
                if best.is_none() {
                    best = Some(gpu);
                }
            }
        }
    }

    best
}

/// Tente de détecter le GPU NVIDIA via nvidia-smi sur Windows.
#[cfg(target_os = "windows")]
fn detect_gpu_nvidia_smi() -> Option<GpuInfo> {
    let output = std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=name,memory.total",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?.trim();
    let mut parts = line.splitn(2, ',');
    let name = parts.next()?.trim().to_string();
    let vram_mb: u64 = parts.next()?.trim().parse().unwrap_or(0);

    if name.is_empty() {
        return None;
    }

    Some(GpuInfo {
        name,
        vram_mb,
        vendor: "nvidia".into(),
    })
}

/// Détecte le GPU sur Linux : pilote NVIDIA (`/proc/driver/nvidia`) ou `/sys/class/drm` (AMD/Intel), sans processus externe.
#[cfg(target_os = "linux")]
fn detect_gpu() -> Option<GpuInfo> {
    // 1. NVIDIA : fichier du pilote (aucun outil externe)
    if let Some(gpu) = detect_gpu_nvidia() {
        return Some(gpu);
    }
    // 2. Essayer AMD via /sys/class/drm
    if let Some(gpu) = detect_gpu_amd_linux() {
        return Some(gpu);
    }
    None
}

#[cfg(target_os = "linux")]
fn detect_gpu_nvidia() -> Option<GpuInfo> {
    // Pilote NVIDIA : /proc/driver/nvidia/gpus/<bus>/information (« Model: … »).
    // Aucun processus externe : l'outil SMI de NVIDIA est interdit d'exécution
    // par le profil AppArmor. La VRAM n'y figure pas (0 = inconnue) : la
    // mémoire réellement utilisable est relevée par maia-llm (llama.cpp).
    let gpus = std::fs::read_dir("/proc/driver/nvidia/gpus").ok()?;
    for entree in gpus.flatten() {
        let Ok(info) = std::fs::read_to_string(entree.path().join("information")) else {
            continue;
        };
        if let Some(name) = modele_nvidia(&info) {
            return Some(GpuInfo {
                name,
                vram_mb: 0,
                vendor: "nvidia".into(),
            });
        }
    }
    None
}

/// Nom du GPU dans un fichier `information` du pilote NVIDIA.
#[cfg(target_os = "linux")]
fn modele_nvidia(information: &str) -> Option<String> {
    information
        .lines()
        .find_map(|l| l.strip_prefix("Model:"))
        .map(|m| m.trim().to_string())
        .filter(|m| !m.is_empty())
}

#[cfg(target_os = "linux")]
fn detect_gpu_amd_linux() -> Option<GpuInfo> {
    // Cherche /sys/class/drm/card*/device/vendor pour AMD (0x1002) ou Intel (0x8086)
    let drm_path = std::path::Path::new("/sys/class/drm");
    if !drm_path.exists() {
        return None;
    }

    for entry in std::fs::read_dir(drm_path).ok()?.flatten() {
        let card = entry.path();
        let vendor_path = card.join("device/vendor");
        if !vendor_path.exists() {
            continue;
        }
        let vendor_str = std::fs::read_to_string(&vendor_path).ok()?;
        let vendor_id = vendor_str.trim().to_lowercase();

        let (vendor, name) = if vendor_id == "0x1002" {
            ("amd", "AMD GPU")
        } else if vendor_id == "0x8086" {
            continue; // Ignorer Intel intégré
        } else {
            continue;
        };

        // Essayer de lire la VRAM depuis /sys/class/drm/card*/device/mem_info_vram_total
        let vram_path = card.join("device/mem_info_vram_total");
        let vram_mb = std::fs::read_to_string(&vram_path)
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok())
            .map_or(0, |bytes| bytes / (1024 * 1024));

        return Some(GpuInfo {
            name: name.to_string(),
            vram_mb,
            vendor: vendor.to_string(),
        });
    }
    None
}

/// Fallback pour macOS et autres plateformes.
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn detect_gpu() -> Option<GpuInfo> {
    None
}

/// Calcule le tier hardware à partir de la RAM totale et du GPU.
/// Note : la RAM déclarée par le système peut être légèrement sous le nominal
/// (ex: 15 GB déclarés pour 16 GB installés). Seuils légèrement assouplis.
fn compute_tier(ram_total_mb: u64, gpu: Option<&GpuInfo>) -> HardwareTier {
    let ram_gb = ram_total_mb / 1024;
    let vram_mb = gpu.map_or(0, |g| g.vram_mb);
    let vram_gb = vram_mb / 1024;

    // GPU discret détecté : le tier est piloté principalement par la VRAM
    // VRAM inconnue (0, ex. NVIDIA lu sans outil externe) : tier selon la RAM.
    if let Some(g) = gpu.filter(|g| g.vram_mb > 0) {
        if g.vendor == "nvidia" || g.vendor == "amd" {
            return match vram_gb {
                v if v >= 16 => HardwareTier::Heavy,
                v if v >= 8 => HardwareTier::Power,
                v if v >= 4 => HardwareTier::Standard,
                _ => HardwareTier::Light,
            };
        }
    }

    // Pas de dGPU — tier basé sur la RAM
    match ram_gb {
        r if r >= 64 => HardwareTier::Heavy,
        r if r >= 32 => HardwareTier::Power,
        r if r >= 14 => HardwareTier::Standard,
        r if r >= 8 => HardwareTier::Light,
        _ => HardwareTier::UltraLight,
    }
}

#[cfg(test)]
mod tests_detection {
    use super::detect_hardware;

    /// Le rafraîchissement restreint (CPU + mémoire) renseigne toujours ces champs.
    #[test]
    fn detection_restreinte_renseigne_cpu_et_memoire() {
        let hw = detect_hardware();
        assert!(hw.cpu_threads >= 1);
        assert!(hw.cpu_cores >= 1);
        assert!(hw.ram_total_mb > 0);
        assert!(hw.ram_available_mb <= hw.ram_total_mb);
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests_nvidia {
    use super::*;

    #[test]
    fn modele_lu_dans_le_pilote_sans_outil_externe() {
        let info = "Model: \t\t NVIDIA GeForce RTX 3060 Ti\nIRQ:   137\n";
        assert_eq!(modele_nvidia(info).as_deref(), Some("NVIDIA GeForce RTX 3060 Ti"));
        assert_eq!(modele_nvidia("IRQ: 1\n"), None);
        let src = include_str!("hardware.rs");
        let linux = src.split("fn detect_gpu() -> Option<GpuInfo> {").nth(2).unwrap_or("");
        assert!(!linux.contains(&["nvidia", "-smi"].concat()));
    }

    #[test]
    fn vram_inconnue_tier_selon_la_ram() {
        let g = GpuInfo { name: "x".into(), vram_mb: 0, vendor: "nvidia".into() };
        assert_eq!(compute_tier(32 * 1024, Some(&g)), compute_tier(32 * 1024, None));
    }
}
