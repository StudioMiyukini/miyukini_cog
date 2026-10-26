//! @id     maia_llm_banc
//! @do     mesurer_performances_modele
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — banc d'essai court : chargement vérifié, délai du premier jeton, jetons/s, mémoire (RSS, VRAM).

//! Banc d'essai (`maia-llm banc …`), hors service, au premier plan.
//!
//! ```text
//! maia-llm banc --modele /srv/app/models/canal/X.gguf --ancre <64 hex>
//!               [--gpu-layers -1|0|N] [--contexte 4096] [--jetons 128]
//!               [--repetitions 3] [--threads 0]
//! ```
//!
//! Le modèle passe le **même contrôle de provenance** qu'en service (manifeste
//! signé par l'ancre donnée, empreinte SHA-256). Sortie : une ligne JSON.

use std::time::Instant;

use serde::Serialize;

use crate::config::MaiaLlmConfig;
use crate::engine::{Generation, LlmEngine};
use crate::materiel;

/// Prompt français fixe (≈ 90 jetons de prompt avec le gabarit de chat).
const PROMPT_BANC: &str = "Explique en quelques phrases, en français simple, pourquoi il est \
    utile de garder ses données personnelles sur son propre ordinateur plutôt que dans un \
    service en ligne, et donne deux conseils pratiques.";

/// Résultat du banc d'essai.
#[derive(Debug, Serialize)]
pub struct ResultatBanc {
    pub modele: String,
    pub taille_mo: u64,
    pub gpu_layers: u32,
    pub gpu_utilisable: bool,
    pub peripheriques: Vec<materiel::Peripherique>,
    pub chargement_ms: u128,
    pub jetons_prompt: u32,
    /// Délai jusqu'au premier jeton, par répétition (ms).
    pub premier_jeton_ms: Vec<u128>,
    /// Jetons générés par seconde, par répétition.
    pub jetons_par_seconde: Vec<f64>,
    pub jetons_generes: Vec<u32>,
    /// Mémoire résidente du processus après génération (Mio).
    pub rss_mo: u64,
    /// Pic de mémoire résidente (Mio).
    pub rss_pic_mo: u64,
    /// VRAM consommée (Mio, différence de mémoire libre GPU).
    pub vram_utilisee_mo: Option<u64>,
    pub recommandation: materiel::Recommandation,
    pub extrait: String,
}

fn valeur(args: &[String], nom: &str) -> Option<String> {
    args.iter()
        .position(|a| a == nom)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn entier<T: std::str::FromStr>(args: &[String], nom: &str, defaut: T) -> Result<T, String> {
    valeur(args, nom).map_or(Ok(defaut), |v| {
        v.parse().map_err(|_| format!("{nom} : nombre attendu"))
    })
}

/// `(VmRSS, VmHWM)` en Mio depuis `/proc/self/status`.
fn rss_mo() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let champ = |nom: &str| {
        s.lines()
            .find(|l| l.starts_with(nom))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|kb| kb.parse::<u64>().ok())
            .map_or(0, |kb| kb / 1024)
    };
    (champ("VmRSS:"), champ("VmHWM:"))
}

fn vram_libre_mo() -> Option<u64> {
    let gpus: Vec<_> = materiel::peripheriques()
        .into_iter()
        .filter(|p| p.gpu)
        .collect();
    if gpus.is_empty() {
        None
    } else {
        Some(gpus.iter().map(|p| p.memoire_libre_mo).sum())
    }
}

/// Sous-commande `banc` : `None` si `args` ne la désigne pas.
pub fn executer(args: &[String], derogation: bool) -> Option<Result<String, String>> {
    if args.get(1).map(String::as_str) != Some("banc") {
        return None;
    }
    Some(banc(args, derogation))
}

fn banc(args: &[String], derogation: bool) -> Result<String, String> {
    const USAGE: &str = "usage : maia-llm banc --modele <X.gguf> --ancre <64 hex> \
        [--gpu-layers N] [--contexte N] [--jetons N] [--repetitions N] [--threads N]";
    let modele = valeur(args, "--modele").ok_or(USAGE)?;
    let ancres: Vec<String> = args
        .iter()
        .enumerate()
        .filter(|(_, a)| a.as_str() == "--ancre")
        .filter_map(|(i, _)| args.get(i + 1).cloned())
        .collect();
    let config = MaiaLlmConfig {
        gpu_layers: entier(args, "--gpu-layers", -1)?,
        context_size: entier(args, "--contexte", 4096)?,
        threads: entier(args, "--threads", 0)?,
        model_trust_anchors: ancres.clone(),
        auto_load: false,
        ..MaiaLlmConfig::default()
    };
    let jetons: u32 = entier(args, "--jetons", 128)?;
    let repetitions: usize = entier(args, "--repetitions", 3)?;

    let ancres = maia_modeles::ancres_depuis_hex(&ancres).map_err(|e| e.to_string())?;
    let politique = maia_modeles::PolitiqueModeles::new(ancres, derogation);
    let moteur = LlmEngine::new(config, politique).map_err(|e| e.to_string())?;

    let vram_avant = vram_libre_mo();
    let debut = Instant::now();
    let info = moteur
        .load_model(std::path::PathBuf::from(&modele))
        .map_err(|e| e.to_string())?;
    let chargement_ms = debut.elapsed().as_millis();
    let vram_apres = vram_libre_mo();

    let messages = vec![
        serde_json::json!({"role": "system", "content": "Tu es un assistant IA concis."}),
        serde_json::json!({"role": "user", "content": PROMPT_BANC}),
    ];
    let generation = Generation::libre(0.0, jetons);
    let mut premier_jeton_ms = Vec::new();
    let mut jetons_par_seconde = Vec::new();
    let mut jetons_generes = Vec::new();
    let mut jetons_prompt = 0;
    let mut extrait = String::new();
    for i in 0..repetitions.max(1) {
        let mut texte = String::new();
        let m = moteur
            .generer(&messages, &generation, &mut |morceau| {
                texte.push_str(morceau);
                true
            })
            .map_err(|e| e.to_string())?;
        jetons_prompt = m.jetons_prompt;
        premier_jeton_ms.push(m.premier_jeton.as_millis());
        jetons_par_seconde.push((m.jetons_par_seconde() * 10.0).round() / 10.0);
        jetons_generes.push(m.jetons_generes);
        if i == 0 {
            extrait = texte.chars().take(240).collect();
        }
    }
    let (rss, pic) = rss_mo();
    let resultat = ResultatBanc {
        modele: info.name,
        taille_mo: info.size_bytes / (1024 * 1024),
        gpu_layers: info.gpu_layers,
        gpu_utilisable: moteur.gpu_utilisable(),
        peripheriques: materiel::peripheriques(),
        chargement_ms,
        jetons_prompt,
        premier_jeton_ms,
        jetons_par_seconde,
        jetons_generes,
        rss_mo: rss,
        rss_pic_mo: pic,
        vram_utilisee_mo: vram_avant.zip(vram_apres).map(|(a, b)| a.saturating_sub(b)),
        recommandation: materiel::recommander_ici(),
        extrait,
    };
    serde_json::to_string(&resultat).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn autre_sous_commande_ignoree() {
        assert!(executer(&["maia-llm".into(), "serve".into()], false).is_none());
    }

    #[test]
    fn usage_sans_modele() {
        let r = executer(&["maia-llm".into(), "banc".into()], false);
        assert!(matches!(r, Some(Err(e)) if e.contains("usage")));
    }

    #[test]
    fn modele_non_signe_refuse() {
        let d = std::env::temp_dir().join(format!("maia-banc-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        let p = d.join("faux.gguf");
        std::fs::write(&p, b"GGUF faux").unwrap();
        let args: Vec<String> = ["maia-llm", "banc", "--modele", p.to_str().unwrap()]
            .iter()
            .map(ToString::to_string)
            .collect();
        let r = executer(&args, false).unwrap();
        assert!(matches!(r, Err(e) if e.contains("refusé") || e.contains("manifeste")));
        let _ = std::fs::remove_dir_all(&d);
    }
}
