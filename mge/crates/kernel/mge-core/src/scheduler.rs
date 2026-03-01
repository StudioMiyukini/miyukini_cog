//! Ordonnanceur de systemes -- ordre d'execution par priorite.
//!
//! <!-- @id: MGE-Core-Scheduler @do: system-scheduling @role: back-end @layer: 1 @human: miyuk -->
//!
//! Le [`SystemScheduler`] gere l'enregistrement et l'ordonnancement des systemes
//! de jeu. Chaque systeme a une [`SystemPriority`] qui determine son ordre
//! d'execution : plus la valeur est basse, plus le systeme s'execute tot.

/// Priorite d'execution d'un systeme (plus bas = plus tot).
///
/// Constantes pre-definies pour les phases classiques d'une boucle de jeu :
/// `INPUT (-200)` -> `PHYSICS (-100)` -> `GAME (0)` -> `RENDER (100)` -> `UI (200)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemPriority(pub i32);

impl SystemPriority {
    /// Phase input : lecture des entrees utilisateur.
    pub const INPUT: Self = Self(-200);
    /// Phase physique : collision, deplacement, gravite.
    pub const PHYSICS: Self = Self(-100);
    /// Phase logique de jeu : IA, combat, quetes.
    pub const GAME: Self = Self(0);
    /// Phase rendu : preparation des draw calls.
    pub const RENDER: Self = Self(100);
    /// Phase UI : HUD, menus, tooltips.
    pub const UI: Self = Self(200);
}

/// Identifiant unique d'un systeme.
///
/// Utilise une chaine statique pour eviter les allocations.
/// Exemple : `SystemId("movement")`, `SystemId("combat_pipeline")`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemId(pub &'static str);

impl std::fmt::Display for SystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Entree dans le scheduler : un systeme avec sa priorite et son etat actif/inactif.
pub struct SystemEntry {
    /// Identifiant unique du systeme.
    pub id: SystemId,
    /// Priorite d'execution (tri croissant).
    pub priority: SystemPriority,
    /// Si `false`, le systeme est ignore lors de l'execution.
    pub enabled: bool,
}

/// Ordonnanceur de systemes.
///
/// Gere l'enregistrement, le tri par priorite, et l'activation/desactivation
/// des systemes. Le scheduler ne possede pas les fonctions systeme elles-memes
/// (qui vivent dans l'ECS) -- il fournit uniquement l'ordre d'execution.
pub struct SystemScheduler {
    systems: Vec<SystemEntry>,
}

impl SystemScheduler {
    /// Cree un scheduler vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    /// Enregistre un systeme avec sa priorite. Re-trie automatiquement.
    pub fn register(&mut self, id: SystemId, priority: SystemPriority) {
        self.systems.push(SystemEntry {
            id,
            priority,
            enabled: true,
        });
        self.systems.sort_by_key(|s| s.priority);
    }

    /// Iterateur sur les `SystemId` des systemes actifs, dans l'ordre de priorite.
    pub fn enabled_ids(&self) -> impl Iterator<Item = &SystemId> {
        self.systems
            .iter()
            .filter(|s| s.enabled)
            .map(|s| &s.id)
    }

    /// Active ou desactive un systeme par son identifiant.
    pub fn set_enabled(&mut self, id: &SystemId, enabled: bool) {
        if let Some(s) = self.systems.iter_mut().find(|s| &s.id == id) {
            s.enabled = enabled;
        }
    }

    /// Retourne `true` si un systeme avec cet identifiant est enregistre.
    #[must_use]
    pub fn contains(&self, id: &SystemId) -> bool {
        self.systems.iter().any(|s| &s.id == id)
    }

    /// Nombre total de systemes enregistres (actifs et inactifs).
    #[must_use]
    pub fn len(&self) -> usize {
        self.systems.len()
    }

    /// Retourne `true` si aucun systeme n'est enregistre.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }
}

impl Default for SystemScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for SystemScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemScheduler")
            .field("system_count", &self.systems.len())
            .finish()
    }
}
