//! Écran de chargement au démarrage de Miyukini Central.
//!
//! Affiche un titre, une barre de progression à remplissage irrégulier (à-coups),
//! et des phrases qui alternent toutes les 1 à 2 secondes.

use rand::Rng;
use std::time::Instant;

/// Phrases affichées pendant le chargement (alternées toutes les 1–2 s).
pub const LOADING_PHRASES: &[&str] = &[
    "/user/videos/secret/Gay/BBC est trop gros pour être chargé dans sa totalité.",
    "Veuillez appuyer sur F4 et Alt en même temps.",
    "Alexa ! OK Google ! %@# de domotique de merde !",
    "42 étant la réponse à votre question...",
    "C'est toujours la faute à ta mère — Freud",
    "Vous entrez dans un écosystème inutile.",
    "Pour appeler le service client composez le ...",
    "PNG c'est surfait, rien ne vaut le SVG — Disiz Vecto",
    "Votre chaise renifle vos parties plus que votre partenaire.",
    "Le jour où vous serez écouté sera... Profitez bien du moment.",
    "Profitez bien du moment où vous serez un jour écouté — Moi",
    "MastaPimp le sorcier gobelin, élu de Nawak.",
];

/// État de l'écran de chargement.
pub struct LoadingState {
    /// Durée totale du chargement (secondes), entre 10 et 12.
    pub total_duration_sec: f32,
    /// Segments (durée en s, progression par seconde). Remplissage en à-coups.
    pub segments: Vec<(f32, f32)>,
    /// Instant de début (secondes depuis démarrage app), fixé au premier frame.
    pub start_time_sec: Option<f64>,
    /// Création au démarrage (fallback si ctx.input().time n'avance pas).
    pub created_at: Instant,
    /// Prochaine rotation de phrase (temps absolu en secondes).
    pub next_phrase_at_sec: f64,
    /// Index de la phrase actuelle.
    pub current_phrase_index: usize,
}

impl LoadingState {
    /// Construit un nouvel état avec durée 10–12 s et segments aléatoires (pattern non prévisible).
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let total_duration_sec = rng.gen_range(10.0_f32..=12.0);
        let n = rng.gen_range(8..=14);
        let mut durations: Vec<f32> = (0..n)
            .map(|_| rng.gen_range(0.4..=1.8))
            .collect();
        let sum: f32 = durations.iter().sum();
        for d in &mut durations {
            *d *= total_duration_sec / sum;
        }
        let mut raw_speeds: Vec<f32> = durations
            .iter()
            .map(|_| {
                if rng.gen_bool(0.5) {
                    rng.gen_range(0.02..=0.08)
                } else {
                    rng.gen_range(0.15..=0.35)
                }
            })
            .collect();
        let weighted: f32 = durations.iter().zip(&raw_speeds).map(|(d, s)| d * s).sum();
        if weighted > 0.0 {
            let scale = 1.0 / weighted;
            for s in &mut raw_speeds {
                *s *= scale;
            }
        }
        let segments: Vec<(f32, f32)> = durations.into_iter().zip(raw_speeds).collect();
        let current_phrase_index = rng.gen_range(0..LOADING_PHRASES.len());
        Self {
            total_duration_sec,
            segments,
            start_time_sec: None,
            created_at: Instant::now(),
            next_phrase_at_sec: 0.0,
            current_phrase_index,
        }
    }

    /// Progression à afficher (0.0 ..= 1.0) pour un temps écoulé donné.
    pub fn progress_at(&self, elapsed_sec: f32) -> f32 {
        let mut t = elapsed_sec;
        let mut p = 0.0_f32;
        for (dur, speed) in &self.segments {
            if t <= 0.0 {
                break;
            }
            let step = t.min(*dur);
            p += step * speed;
            t -= step;
        }
        p.min(1.0)
    }

    /// Met à jour l’état (démarrage, rotation des phrases). Retourne la phrase courante.
    pub fn update(&mut self, time_sec: f64, rng: &mut impl Rng) -> &'static str {
        if self.start_time_sec.is_none() {
            self.start_time_sec = Some(time_sec);
            self.next_phrase_at_sec = time_sec + rng.gen_range(1.5..=3.0);
        }
        if time_sec >= self.next_phrase_at_sec {
            self.current_phrase_index = rng.gen_range(0..LOADING_PHRASES.len());
            self.next_phrase_at_sec = time_sec + rng.gen_range(1.5..=3.0);
        }

        LOADING_PHRASES[self.current_phrase_index]
    }

    /// Temps écoulé depuis le début (secondes), depuis egui uniquement.
    pub fn elapsed_sec(&self, time_sec: f64) -> f32 {
        self.start_time_sec
            .map(|s| (time_sec - s) as f32)
            .unwrap_or(0.0)
    }

    /// Temps écoulé effectif (egui ou Instant, le plus avancé) pour affichage et is_done.
    pub fn effective_elapsed_sec(&self, time_sec: f64) -> f32 {
        let egui = self.elapsed_sec(time_sec);
        let instant = self.created_at.elapsed().as_secs_f32();
        egui.max(instant)
    }

    /// True si le chargement est terminé (durée atteinte).
    pub fn is_done(&self, time_sec: f64) -> bool {
        self.effective_elapsed_sec(time_sec) >= self.total_duration_sec
    }
}
