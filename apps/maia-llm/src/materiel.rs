//! @id     maia_llm_materiel
//! @do     detecter_materiel_et_profil
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — détection du matériel par llama.cpp (jamais d'outil SMI externe) et profil recommandé P1/P2/P3.

//! Matériel vu par le moteur et profil recommandé
//! ([MODELES-IA-PETITES-CONFIGS](../../../docs/architecture/MODELES-IA-PETITES-CONFIGS.md) §2).
//!
//! - Les GPU sont listés par **llama.cpp lui-même** (périphériques ggml du
//!   binaire) : aucune commande externe, l'outil SMI de NVIDIA n'est jamais exécuté
//!   (interdit par le profil AppArmor). Un binaire compilé sans `cuda` ne voit
//!   aucun GPU : il sert en CPU.
//! - La RAM vient de `/proc/meminfo`.

use serde::Serialize;

use llama_cpp_2::{list_llama_ggml_backend_devices, LlamaBackendDeviceType};

/// Périphérique de calcul vu par llama.cpp.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Peripherique {
    /// Nom ggml (ex. `CUDA0`, `CPU`).
    pub nom: String,
    /// Description (ex. `NVIDIA GeForce RTX 3060 Ti`).
    pub description: String,
    /// Moteur ggml (`CUDA`, `CPU`…).
    pub moteur: String,
    /// Mémoire totale (Mio).
    pub memoire_totale_mo: u64,
    /// Mémoire libre (Mio).
    pub memoire_libre_mo: u64,
    /// GPU (dédié ou intégré).
    pub gpu: bool,
}

/// Profils matériels de la doc (P1 CPU seul, P2 GPU 8 Go, P3 GPU 12–16 Go).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub enum Profil {
    /// CPU seul (VM sans GPU, petite machine).
    P1,
    /// GPU ≥ 6 Go et < 12 Go (serveur actuel : RTX 3060 Ti 8 Go).
    P2,
    /// GPU ≥ 12 Go.
    P3,
}

/// Recommandation de configuration du moteur pour la machine.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Recommandation {
    /// Profil retenu.
    pub profil: Profil,
    /// `gpu_layers` conseillé (`-1` = tout sur GPU, `0` = CPU).
    pub gpu_layers: i32,
    /// Contexte conseillé (jetons).
    pub context_size: u32,
    /// Fils CPU conseillés (`0` = automatique).
    pub threads: u32,
    /// Explication lisible.
    pub motif: String,
}

/// Périphériques ggml du binaire (CPU compris).
pub fn peripheriques() -> Vec<Peripherique> {
    list_llama_ggml_backend_devices()
        .into_iter()
        .map(|d| Peripherique {
            gpu: matches!(
                d.device_type,
                LlamaBackendDeviceType::Gpu | LlamaBackendDeviceType::IntegratedGpu
            ),
            nom: d.name,
            description: d.description,
            moteur: d.backend,
            memoire_totale_mo: u64::try_from(d.memory_total).unwrap_or(0) / (1024 * 1024),
            memoire_libre_mo: u64::try_from(d.memory_free).unwrap_or(0) / (1024 * 1024),
        })
        .collect()
}

/// Vrai si llama.cpp voit au moins un GPU.
pub fn gpu_present() -> bool {
    peripheriques().iter().any(|p| p.gpu)
}

/// Vrai si un GPU a assez de mémoire **libre** pour un modèle de `taille`
/// octets (× 1,3 + 768 Mio pour le cache KV et les tampons).
pub fn vram_libre_suffisante(taille: u64) -> bool {
    let besoin_mo = besoin_vram_mo(taille);
    peripheriques()
        .iter()
        .any(|p| p.gpu && p.memoire_libre_mo >= besoin_mo)
}

fn besoin_vram_mo(taille: u64) -> u64 {
    (taille / (1024 * 1024)).saturating_mul(13) / 10 + 768
}

/// RAM totale et disponible (Mio) lues dans `/proc/meminfo` ; `(0, 0)` ailleurs.
pub fn ram_mo() -> (u64, u64) {
    std::fs::read_to_string("/proc/meminfo").map_or((0, 0), |s| ram_depuis_meminfo(&s))
}

fn ram_depuis_meminfo(meminfo: &str) -> (u64, u64) {
    let champ = |nom: &str| {
        meminfo
            .lines()
            .find(|l| l.starts_with(nom))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|kb| kb.parse::<u64>().ok())
            .map_or(0, |kb| kb / 1024)
    };
    (champ("MemTotal:"), champ("MemAvailable:"))
}

/// Profil recommandé pour `vram_mo` (plus grand GPU, 0 = aucun) et `ram_mo`.
pub fn recommander(vram_mo: u64, ram_mo: u64) -> Recommandation {
    if vram_mo >= 12 * 1024 {
        Recommandation {
            profil: Profil::P3,
            gpu_layers: -1,
            context_size: 8192,
            threads: 0,
            motif: format!("GPU {vram_mo} Mio ≥ 12 Gio : tout le modèle en VRAM, contexte 8K"),
        }
    } else if vram_mo >= 6 * 1024 {
        Recommandation {
            profil: Profil::P2,
            gpu_layers: -1,
            context_size: 8192,
            threads: 0,
            motif: format!("GPU {vram_mo} Mio : mini modèle entièrement en VRAM, contexte 8K"),
        }
    } else {
        // CPU seul (ou GPU trop petit) : contexte réduit sous 8 Gio de RAM.
        let context_size = if ram_mo > 0 && ram_mo < 8 * 1024 {
            2048
        } else {
            4096
        };
        Recommandation {
            profil: Profil::P1,
            gpu_layers: 0,
            context_size,
            threads: 0,
            motif: format!(
                "aucun GPU utilisable (VRAM {vram_mo} Mio) : CPU seul, RAM {ram_mo} Mio, contexte {context_size}"
            ),
        }
    }
}

/// Recommandation pour la machine courante (GPU vus par ce binaire).
pub fn recommander_ici() -> Recommandation {
    let vram = peripheriques()
        .iter()
        .filter(|p| p.gpu)
        .map(|p| p.memoire_totale_mo)
        .max()
        .unwrap_or(0);
    recommander(vram, ram_mo().0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profils_selon_vram_et_ram() {
        assert_eq!(recommander(8 * 1024, 30 * 1024).profil, Profil::P2);
        assert_eq!(recommander(8 * 1024, 30 * 1024).gpu_layers, -1);
        assert_eq!(recommander(16 * 1024, 64 * 1024).profil, Profil::P3);
        let cpu = recommander(0, 6 * 1024);
        assert_eq!(cpu.profil, Profil::P1);
        assert_eq!(cpu.gpu_layers, 0);
        assert_eq!(cpu.context_size, 2048);
        assert_eq!(recommander(4 * 1024, 16 * 1024).profil, Profil::P1);
        assert_eq!(recommander(0, 16 * 1024).context_size, 4096);
    }

    #[test]
    fn besoin_vram_avec_marge() {
        // Mini modèle de 1,04 Gio : ≈ 2 Gio exigés libres (GPU occupé à 7,2/8 Gio ⇒ CPU).
        assert_eq!(besoin_vram_mo(1_117_320_736), 1384 + 768);
    }

    #[test]
    fn meminfo_lu() {
        let m = "MemTotal:       31457280 kB\nMemFree: 1 kB\nMemAvailable:   11534336 kB\n";
        assert_eq!(ram_depuis_meminfo(m), (30720, 11264));
        assert_eq!(ram_depuis_meminfo(""), (0, 0));
    }

    #[test]
    fn jamais_nvidia_smi() {
        // La détection passe par llama.cpp : aucun processus externe.
        let src = include_str!("materiel.rs");
        let commande = ["std::process::", "Command"].concat();
        let smi = ["nvidia", "-smi"].concat();
        assert!(!src.contains(&commande) && !src.contains(&smi));
    }
}
