# IMPL-09 --- Scripting : Rhai Quest Engine

Guide d'implementation du moteur de quetes Sodomight via Rhai.
Stack : rhai 1.x (pure Rust, pas de dependances externes).

---

## 1. Crate `sd-scripting`

### Cargo.toml

```toml
[package]
name = "sd-scripting"
version = "0.1.0"
edition = "2021"

[dependencies]
rhai = { version = "1", features = ["sync"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
thiserror = "2"
```

### Structure

```
sd-scripting/src/
├── lib.rs          -- pub use, ScriptError
├── engine.rs       -- ScriptEngine (rhai::Engine + API exposed)
├── context.rs      -- ScriptContext (etat disponible pour les scripts)
├── triggers.rs     -- TriggerSystem (zone_enter, npc_talk, kill_count)
├── quest.rs        -- QuestScript, etat de quete
└── api/
    ├── mod.rs
    ├── player.rs   -- fonctions player_*
    ├── world.rs    -- fonctions world_*
    └── ui.rs       -- fonctions ui_*
```

---

## 2. ScriptError

```rust
// src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScriptError {
    #[error("Rhai parse error: {0}")]
    Parse(String),
    #[error("Rhai runtime error: {0}")]
    Runtime(String),
    #[error("Script not found: {0}")]
    NotFound(String),
    #[error("Invalid script context: {0}")]
    Context(String),
}

pub type ScriptResult<T> = Result<T, ScriptError>;

pub use engine::ScriptEngine;
pub use context::ScriptContext;
pub use triggers::TriggerSystem;
pub use quest::{QuestScript, QuestState};

mod engine;
mod context;
mod triggers;
mod quest;
pub mod api;
```

---

## 3. ScriptContext --- etat injecte dans les scripts

```rust
// src/context.rs
use std::collections::HashMap;

/// Tout ce qu'un script peut lire/modifier sur l'etat du jeu.
/// Passe par clonage au moteur Rhai (types simples, Clone + Send + Sync).
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// ID du personnage courant
    pub character_id: String,
    /// Niveau du personnage
    pub level: i32,
    /// Stats cles
    pub strength: i32,
    pub dexterity: i32,
    /// Inventaire simplifie : item_id -> quantite
    pub inventory: HashMap<String, u32>,
    /// Flags de quete actifs : quest_id -> state
    pub quest_flags: HashMap<String, String>,
    /// Zone actuelle
    pub zone_id: String,
    /// NPC courant (si dialogue)
    pub npc_id: Option<String>,
    /// Nombre de kills par type de monstre (pour les quetes kill_count)
    pub kill_counts: HashMap<String, u32>,
    /// Waypoints debloques : act -> [waypoint_id]
    pub waypoints: HashMap<i32, Vec<String>>,
    /// Messages a afficher (queue de retour vers le jeu)
    pub pending_messages: Vec<String>,
    /// Warps a effectuer (zone_id cible)
    pub pending_warp: Option<String>,
    /// Items a donner au joueur
    pub pending_item_grants: Vec<(String, u32)>, // (item_id, qty)
    /// XP a accorder
    pub pending_xp: i64,
}

impl ScriptContext {
    pub fn new(character_id: &str, zone_id: &str) -> Self {
        Self {
            character_id: character_id.to_string(),
            level: 1,
            strength: 10,
            dexterity: 10,
            inventory: HashMap::new(),
            quest_flags: HashMap::new(),
            zone_id: zone_id.to_string(),
            npc_id: None,
            kill_counts: HashMap::new(),
            waypoints: HashMap::new(),
            pending_messages: Vec::new(),
            pending_warp: None,
            pending_item_grants: Vec::new(),
            pending_xp: 0,
        }
    }
}
```

---

## 4. API exposee aux scripts Rhai

### api/player.rs

```rust
// src/api/player.rs
use rhai::Engine;

/// Enregistre les fonctions player_* dans le moteur Rhai
pub fn register(engine: &mut Engine) {
    // player_level() -> int
    engine.register_fn("player_level", |ctx: &mut rhai::Dynamic| -> i64 {
        // Le contexte est passe comme variable globale "ctx"
        // Note : dans l'implementation reelle, on accede via scope
        0_i64 // placeholder
    });
}
```

### api/world.rs

```rust
// src/api/world.rs -- fonctions world_*
```

### api/mod.rs

```rust
pub mod player;
pub mod world;
pub mod ui;
```

---

## 5. ScriptEngine --- moteur Rhai configure

```rust
// src/engine.rs
use rhai::{Engine, Scope, AST, Dynamic};
use std::collections::HashMap;
use crate::{ScriptResult, ScriptError, ScriptContext};

pub struct ScriptEngine {
    engine: Engine,
    /// Cache des scripts compiles : script_id -> AST
    compiled: HashMap<String, AST>,
}

impl ScriptEngine {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Limites de securite
        engine.set_max_operations(50_000); // anti-boucle infinie
        engine.set_max_call_levels(32);
        engine.set_max_string_size(4096);
        engine.set_max_array_size(1024);

        // Desactiver les features dangereuses
        engine.disable_symbol("eval");

        // Enregistrer les types personnalises
        engine.register_type_with_name::<ScriptContext>("ScriptContext");

        // Fonctions player_*
        engine.register_fn("player_has_item", |ctx: &mut ScriptContext, item_id: &str| -> bool {
            ctx.inventory.get(item_id).copied().unwrap_or(0) > 0
        });
        engine.register_fn("player_item_count", |ctx: &mut ScriptContext, item_id: &str| -> i64 {
            ctx.inventory.get(item_id).copied().unwrap_or(0) as i64
        });
        engine.register_fn("player_level", |ctx: &mut ScriptContext| -> i64 {
            ctx.level as i64
        });
        engine.register_fn("player_zone", |ctx: &mut ScriptContext| -> String {
            ctx.zone_id.clone()
        });
        engine.register_fn("player_kill_count", |ctx: &mut ScriptContext, monster: &str| -> i64 {
            ctx.kill_counts.get(monster).copied().unwrap_or(0) as i64
        });

        // Fonctions quest_*
        engine.register_fn("quest_get", |ctx: &mut ScriptContext, quest_id: &str| -> String {
            ctx.quest_flags.get(quest_id).cloned().unwrap_or_else(|| "none".to_string())
        });
        engine.register_fn("quest_set", |ctx: &mut ScriptContext, quest_id: &str, state: &str| {
            ctx.quest_flags.insert(quest_id.to_string(), state.to_string());
        });
        engine.register_fn("quest_is_complete", |ctx: &mut ScriptContext, quest_id: &str| -> bool {
            ctx.quest_flags.get(quest_id).map(|s| s == "complete").unwrap_or(false)
        });

        // Fonctions world_*
        engine.register_fn("world_warp", |ctx: &mut ScriptContext, zone_id: &str| {
            ctx.pending_warp = Some(zone_id.to_string());
        });
        engine.register_fn("world_unlock_waypoint", |ctx: &mut ScriptContext, act: i64, wp_id: &str| {
            ctx.waypoints.entry(act as i32).or_default().push(wp_id.to_string());
        });

        // Fonctions ui_*
        engine.register_fn("ui_message", |ctx: &mut ScriptContext, msg: &str| {
            ctx.pending_messages.push(msg.to_string());
        });
        engine.register_fn("ui_dialog", |ctx: &mut ScriptContext, text: &str| {
            ctx.pending_messages.push(format!("[DIALOG] {}", text));
        });

        // Fonctions reward_*
        engine.register_fn("reward_item", |ctx: &mut ScriptContext, item_id: &str, qty: i64| {
            ctx.pending_item_grants.push((item_id.to_string(), qty as u32));
        });
        engine.register_fn("reward_xp", |ctx: &mut ScriptContext, xp: i64| {
            ctx.pending_xp += xp;
        });
        engine.register_fn("reward_skill_point", |ctx: &mut ScriptContext| {
            ctx.pending_messages.push("[REWARD_SKILL_POINT]".to_string());
        });

        // Fonctions utilitaires
        engine.register_fn("log_info", |msg: &str| {
            log::info!("[Rhai] {}", msg);
        });
        engine.register_fn("log_warn", |msg: &str| {
            log::warn!("[Rhai] {}", msg);
        });

        Self { engine, compiled: HashMap::new() }
    }

    /// Compile et cache un script Rhai
    pub fn compile(&mut self, id: &str, source: &str) -> ScriptResult<()> {
        let ast = self.engine.compile(source)
            .map_err(|e| ScriptError::Parse(e.to_string()))?;
        self.compiled.insert(id.to_string(), ast);
        Ok(())
    }

    /// Execute un script avec le contexte donne.
    /// Le contexte est modifie en place par le script.
    pub fn run(&self, id: &str, ctx: &mut ScriptContext) -> ScriptResult<()> {
        let ast = self.compiled.get(id)
            .ok_or_else(|| ScriptError::NotFound(id.to_string()))?;

        let mut scope = Scope::new();
        scope.push("ctx", ctx.clone());

        self.engine.run_ast_with_scope(&mut scope, ast)
            .map_err(|e| ScriptError::Runtime(e.to_string()))?;

        // Recuperer le contexte modifie
        if let Some(new_ctx) = scope.get_value::<ScriptContext>("ctx") {
            *ctx = new_ctx;
        }

        Ok(())
    }

    /// Appelle une fonction specifique du script (ex: "on_complete")
    pub fn call_fn(&self, id: &str, fn_name: &str, ctx: &mut ScriptContext) -> ScriptResult<()> {
        let ast = self.compiled.get(id)
            .ok_or_else(|| ScriptError::NotFound(id.to_string()))?;

        let mut scope = Scope::new();
        scope.push("ctx", ctx.clone());

        let _: Dynamic = self.engine.call_fn(&mut scope, ast, fn_name, (ctx.clone(),))
            .map_err(|e| ScriptError::Runtime(e.to_string()))?;

        if let Some(new_ctx) = scope.get_value::<ScriptContext>("ctx") {
            *ctx = new_ctx;
        }

        Ok(())
    }
}
```

---

## 6. Triggers --- declencheurs d'evenements

```rust
// src/triggers.rs
use std::collections::HashMap;
use crate::{ScriptEngine, ScriptContext, ScriptResult};

/// Types de declencheurs
#[derive(Debug, Clone)]
pub enum TriggerType {
    /// Joueur entre dans une zone
    ZoneEnter { zone_id: String },
    /// Joueur parle a un NPC
    NpcTalk { npc_id: String },
    /// Monstre tue (type)
    MonsterKilled { monster_type: String },
    /// Item ramasse
    ItemPickedUp { item_id: String },
    /// Quest completee
    QuestComplete { quest_id: String },
    /// Niveau atteint
    LevelUp { level: i32 },
}

/// Un trigger mappe a un script
pub struct Trigger {
    pub trigger_type: TriggerType,
    pub script_id: String,
}

pub struct TriggerSystem {
    triggers: Vec<Trigger>,
}

impl TriggerSystem {
    pub fn new() -> Self {
        Self { triggers: Vec::new() }
    }

    pub fn register(&mut self, trigger_type: TriggerType, script_id: &str) {
        self.triggers.push(Trigger {
            trigger_type,
            script_id: script_id.to_string(),
        });
    }

    /// Declenche tous les scripts correspondant a un evenement zone_enter
    pub fn fire_zone_enter(
        &self,
        zone_id: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        for t in &self.triggers {
            if let TriggerType::ZoneEnter { zone_id: z } = &t.trigger_type {
                if z == zone_id {
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }

    /// Declenche les scripts NPC talk
    pub fn fire_npc_talk(
        &self,
        npc_id: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        ctx.npc_id = Some(npc_id.to_string());
        for t in &self.triggers {
            if let TriggerType::NpcTalk { npc_id: n } = &t.trigger_type {
                if n == npc_id {
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }

    /// Declenche les scripts kill_count
    pub fn fire_monster_killed(
        &self,
        monster_type: &str,
        engine: &ScriptEngine,
        ctx: &mut ScriptContext,
    ) -> ScriptResult<()> {
        // Incrementer le kill count dans le contexte
        *ctx.kill_counts.entry(monster_type.to_string()).or_insert(0) += 1;

        for t in &self.triggers {
            if let TriggerType::MonsterKilled { monster_type: m } = &t.trigger_type {
                if m == monster_type || m == "*" { // "*" = tous les monstres
                    engine.run(&t.script_id, ctx)?;
                }
            }
        }
        Ok(())
    }
}
```

---

## 7. QuestScript --- structure d'une quete scriptee

```rust
// src/quest.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuestState {
    Inactive,
    Active,
    Complete,
    Failed,
}

/// Metadonnees d'une quete (chargees depuis TOML)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestScript {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub description: String,
    /// Chemin vers le script Rhai
    pub script_path: String,
    /// Quetes prerequises (doivent etre completes)
    pub prerequisites: Vec<String>,
    /// Rewards decrits en metadata (le script peut en donner plus)
    pub reward_xp: i64,
    pub reward_items: Vec<(String, u32)>,
    pub reward_skill_point: bool,
}
```

---

## 8. Scripts Rhai d'exemple

### Quest : Den of Evil (Acte 1)

```rhai
// assets/scripts/quests/den_of_evil.rhai

// Appele quand le joueur entre dans la Blood Moor pour la premiere fois
fn on_zone_enter() {
    let state = quest_get(ctx, "den_of_evil");
    if state == "none" {
        quest_set(ctx, "den_of_evil", "active");
        ui_dialog(ctx, "Akara: Les demons envahissent la grotte de l'Antre du Mal. Nettoyez-la !");
    }
}

// Appele a chaque monstre tue
fn on_monster_killed() {
    let state = quest_get(ctx, "den_of_evil");
    if state != "active" { return; }

    // Verifier si tous les monstres de la grotte sont morts
    // (Le jeu injecte un flag "den_cleared" quand la condition est remplie)
    let den_cleared = quest_get(ctx, "den_cleared") == "true";
    if den_cleared {
        quest_set(ctx, "den_of_evil", "complete");
        reward_skill_point(ctx);
        reward_xp(ctx, 1500);
        ui_dialog(ctx, "Akara: Bien joue, aventurier. Prenez ce point de competence en recompense.");
        world_unlock_waypoint(ctx, 1, "blood_moor");
    }
}
```

### Quest : Recherche d'Deckard Cain (Acte 1)

```rhai
// assets/scripts/quests/search_for_cain.rhai

fn on_npc_talk() {
    let quest_state = quest_get(ctx, "search_for_cain");

    if quest_state == "none" {
        quest_set(ctx, "search_for_cain", "active");
        ui_dialog(ctx, "Kashya: Trouvez Deckard Cain a Tristram. Le portail de Tavern se trouve en Dark Wood.");
    } else if quest_state == "tristram_entered" {
        // Le joueur a libere Cain
        quest_set(ctx, "search_for_cain", "complete");
        reward_xp(ctx, 2500);
        reward_item(ctx, "identify_scroll", 5);
        ui_dialog(ctx, "Deckard Cain: Merci de m'avoir sauve. Je peux identifier vos objets.");
    }
}

fn on_zone_enter() {
    // Zone ID = "tristram"
    let state = quest_get(ctx, "search_for_cain");
    if state == "active" {
        quest_set(ctx, "search_for_cain", "tristram_entered");
        ui_dialog(ctx, "Vous etes a Tristram. Liberez Deckard Cain de son cercle runique !");
    }
}
```

### Trigger NPC Akara (marchand/guerisseuse Acte 1)

```rhai
// assets/scripts/npcs/akara.rhai

fn on_talk() {
    let den_done = quest_is_complete(ctx, "den_of_evil");
    let sisters_done = quest_is_complete(ctx, "sisters_burial_grounds");

    if !den_done {
        ui_dialog(ctx, "Akara: L'Antre du Mal menace notre camp. Eliminez les monstres qui y rodent.");
    } else if !sisters_done {
        ui_dialog(ctx, "Akara: Blood Raven, notre ancienne capitaine, deshonore le Cimetiere des Soeurs.");
    } else {
        ui_dialog(ctx, "Akara: Que Diablo soit detruit ! Restez en bonne sante, aventurier.");
    }
}
```

---

## 9. Chargement automatique des scripts

```rust
// src/engine.rs (ajout)
use std::fs;
use std::path::Path;

impl ScriptEngine {
    /// Charge tous les scripts .rhai d'un repertoire
    pub fn load_directory(&mut self, dir: &Path) -> ScriptResult<usize> {
        let mut count = 0;
        if !dir.exists() {
            log::warn!("Script directory not found: {}", dir.display());
            return Ok(0);
        }

        for entry in fs::read_dir(dir).map_err(|e| ScriptError::NotFound(e.to_string()))? {
            let entry = entry.map_err(|e| ScriptError::NotFound(e.to_string()))?;
            let path = entry.path();
            if path.extension().map(|e| e == "rhai").unwrap_or(false) {
                let id = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                let source = fs::read_to_string(&path)
                    .map_err(|e| ScriptError::NotFound(e.to_string()))?;
                match self.compile(&id, &source) {
                    Ok(()) => {
                        log::info!("Loaded script: {}", id);
                        count += 1;
                    }
                    Err(e) => {
                        log::error!("Script compile error {}: {}", id, e);
                    }
                }
            }
        }
        Ok(count)
    }
}
```

---

## 10. Tests

```rust
// src/tests.rs
#[cfg(test)]
mod tests {
    use crate::{ScriptEngine, ScriptContext, QuestState};

    fn make_engine() -> ScriptEngine {
        ScriptEngine::new()
    }

    fn make_ctx() -> ScriptContext {
        ScriptContext::new("char_001", "rogue_encampment")
    }

    #[test]
    fn test_quest_set_and_get() {
        let mut engine = make_engine();
        engine.compile("test_quest", r#"
            quest_set(ctx, "den_of_evil", "active");
            let state = quest_get(ctx, "den_of_evil");
            if state != "active" {
                log_warn("Quest state mismatch!");
            }
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_quest", &mut ctx).unwrap();
        assert_eq!(ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()), Some("active"));
    }

    #[test]
    fn test_reward_xp() {
        let mut engine = make_engine();
        engine.compile("test_xp", r#"
            reward_xp(ctx, 1500);
            reward_xp(ctx, 500);
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_xp", &mut ctx).unwrap();
        assert_eq!(ctx.pending_xp, 2000);
    }

    #[test]
    fn test_reward_item() {
        let mut engine = make_engine();
        engine.compile("test_item", r#"
            reward_item(ctx, "identify_scroll", 5);
            reward_item(ctx, "town_portal", 3);
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_item", &mut ctx).unwrap();
        assert_eq!(ctx.pending_item_grants.len(), 2);
        assert_eq!(ctx.pending_item_grants[0], ("identify_scroll".to_string(), 5));
    }

    #[test]
    fn test_world_warp() {
        let mut engine = make_engine();
        engine.compile("test_warp", r#"
            world_warp(ctx, "tristram");
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_warp", &mut ctx).unwrap();
        assert_eq!(ctx.pending_warp.as_deref(), Some("tristram"));
    }

    #[test]
    fn test_ui_message() {
        let mut engine = make_engine();
        engine.compile("test_ui", r#"
            ui_message(ctx, "Hello World");
            ui_dialog(ctx, "Bonjour aventurier");
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_ui", &mut ctx).unwrap();
        assert_eq!(ctx.pending_messages.len(), 2);
        assert!(ctx.pending_messages[0].contains("Hello World"));
    }

    #[test]
    fn test_conditional_quest_logic() {
        let mut engine = make_engine();
        engine.compile("test_conditional", r#"
            quest_set(ctx, "den_of_evil", "active");
            if quest_get(ctx, "den_of_evil") == "active" {
                reward_xp(ctx, 1000);
                quest_set(ctx, "den_of_evil", "complete");
            }
        "#).unwrap();

        let mut ctx = make_ctx();
        engine.run("test_conditional", &mut ctx).unwrap();
        assert_eq!(ctx.pending_xp, 1000);
        assert_eq!(ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()), Some("complete"));
    }

    #[test]
    fn test_kill_count() {
        let mut engine = make_engine();
        engine.compile("test_kills", r#"
            let kills = player_kill_count(ctx, "fallen");
            if kills >= 10 {
                quest_set(ctx, "den_of_evil", "complete");
            }
        "#).unwrap();

        let mut ctx = make_ctx();
        ctx.kill_counts.insert("fallen".to_string(), 15);
        engine.run("test_kills", &mut ctx).unwrap();
        assert_eq!(ctx.quest_flags.get("den_of_evil").map(|s| s.as_str()), Some("complete"));
    }

    #[test]
    fn test_max_operations_limit() {
        let mut engine = make_engine();
        engine.compile("test_infinite_loop", r#"
            let i = 0;
            loop {
                i += 1;
            }
        "#).unwrap();

        let mut ctx = make_ctx();
        let result = engine.run("test_infinite_loop", &mut ctx);
        assert!(result.is_err()); // Doit echouer a cause de la limite d'operations
    }
}
```

---

## 11. TOML des quetes

```toml
# assets/data/quests/act1_quests.toml

[[quests]]
id = "den_of_evil"
name = "L'Antre du Mal"
act = 1
description = "Nettoyez l'Antre du Mal de ses habitants corrompus."
script_path = "scripts/quests/den_of_evil.rhai"
prerequisites = []
reward_xp = 1500
reward_skill_point = true
reward_items = []

[[quests]]
id = "sisters_burial_grounds"
name = "Le Cimetiere des Soeurs"
act = 1
description = "Eliminez Blood Raven qui profane le Cimetiere des Soeurs."
script_path = "scripts/quests/sisters_burial_grounds.rhai"
prerequisites = ["den_of_evil"]
reward_xp = 3500
reward_skill_point = false
reward_items = [["emerald", 1]]

[[quests]]
id = "search_for_cain"
name = "A la Recherche de Cain"
act = 1
description = "Retrouvez Deckard Cain a Tristram et ramenez-le au camp."
script_path = "scripts/quests/search_for_cain.rhai"
prerequisites = []
reward_xp = 2500
reward_skill_point = false
reward_items = [["identify_scroll", 5]]
```

---

## 12. Checklist integration

- [ ] `sd-scripting` ajoute au workspace `Cargo.toml`
- [ ] `ScriptEngine::new()` ne panique pas
- [ ] `engine.load_directory("assets/scripts/quests/")` charge N scripts sans erreur
- [ ] `engine.load_directory("assets/scripts/npcs/")` charge les scripts NPC
- [ ] `TriggerSystem` enregistre : zone_enter Blood Moor -> den_of_evil.rhai
- [ ] Script den_of_evil.rhai s'execute sans erreur Rhai
- [ ] Boucle infinie dans un script -> erreur propre (pas de crash)
- [ ] `cargo test -p sd-scripting -- --nocapture` : tous les tests passent
- [ ] `cargo clippy -p sd-scripting -- -D warnings` : aucun warning

---

*Fin IMPL-09 --- Scripting Rhai quest engine. Voir IMPL-10 pour le GameUI.*
