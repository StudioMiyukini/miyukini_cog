<!-- @id: SD-Impl-08 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-08 -- Input & Audio : winit Events, Keybindings, kira Audio

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Architecture.md sections Input/Audio (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

Guide d'implementation de la gestion des entrees et de l'audio Sodomight.
Stack : winit 0.30, kira 0.8, serde JSON pour les keybindings.

---

## Table des matieres

1. [Crates concernees](#1-crates-concernees)
2. [GameAction -- actions abstraites](#2-gameaction--actions-abstraites)
3. [Keybinding -- configuration](#3-keybinding--configuration)
4. [InputEvent -- evenements jeu](#4-inputevent--evenements-jeu)
5. [InputProcessor -- conversion winit vers InputEvent](#5-inputprocessor--conversion-winit-vers-inputevent)
6. [AudioError](#6-audioerror)
7. [SoundBank -- pool de sons](#7-soundbank--pool-de-sons)
8. [BgmPlayer -- musique par acte](#8-bgmplayer--musique-par-acte)
9. [AudioContext -- facade unifiee](#9-audiocontext--facade-unifiee)
10. [Integration dans la boucle winit](#10-integration-dans-la-boucle-winit)
11. [Tests](#11-tests)
12. [Checklist integration](#12-checklist-integration)

---

## 1. Crates concernees

- `sd-input` -- mapping touches/actions, event buffer
- `sd-audio` -- SoundBank, BGM per act, SFX pool, kira

### sd-input/Cargo.toml

```toml
[package]
name = "sd-input"
version = "0.1.0"
edition = "2021"

[dependencies]
winit = "0.30"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
```

### sd-audio/Cargo.toml

```toml
[package]
name = "sd-audio"
version = "0.1.0"
edition = "2021"

[dependencies]
kira = { version = "0.8", features = ["cpal"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
thiserror = "2"
```

---

## 2. GameAction -- actions abstraites

```rust
// sd-input/src/actions.rs

/// Toutes les actions possibles en jeu -- independantes de la touche physique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GameAction {
    // Deplacement
    MoveUp, MoveDown, MoveLeft, MoveRight,
    // Combat
    AttackPrimary,       // clic gauche
    AttackSecondary,     // clic droit
    SkillSlot1, SkillSlot2, SkillSlot3, SkillSlot4,
    SkillSlot5, SkillSlot6, SkillSlot7, SkillSlot8,
    // UI
    OpenInventory,
    OpenCharacter,
    OpenSkillTree,
    OpenQuestLog,
    OpenMap,
    // Systeme
    Pause,
    Screenshot,
    ToggleChat,
    // Belt
    BeltPotion1, BeltPotion2, BeltPotion3, BeltPotion4,
}
```

---

## 3. Keybinding -- configuration

```rust
// sd-input/src/keybinding.rs
use std::collections::HashMap;
use winit::keyboard::{KeyCode, MouseButton};
use crate::actions::GameAction;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Binding {
    Key(KeyCode),
    Mouse(MouseButton),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Keybindings {
    pub map: HashMap<String, GameAction>, // serialise comme String -> GameAction
}

impl Keybindings {
    /// Keybindings D2 par defaut
    pub fn default_d2() -> Self {
        let mut map = HashMap::new();
        // Mouvement = souris (click-to-move, pas de mapping clavier)
        // Skills
        map.insert("Digit1".to_string(), GameAction::SkillSlot1);
        map.insert("Digit2".to_string(), GameAction::SkillSlot2);
        map.insert("Digit3".to_string(), GameAction::SkillSlot3);
        map.insert("Digit4".to_string(), GameAction::SkillSlot4);
        map.insert("KeyF1".to_string(), GameAction::SkillSlot5);
        map.insert("KeyF2".to_string(), GameAction::SkillSlot6);
        map.insert("KeyF3".to_string(), GameAction::SkillSlot7);
        map.insert("KeyF4".to_string(), GameAction::SkillSlot8);
        // UI
        map.insert("KeyI".to_string(), GameAction::OpenInventory);
        map.insert("KeyC".to_string(), GameAction::OpenCharacter);
        map.insert("KeyK".to_string(), GameAction::OpenSkillTree);
        map.insert("KeyQ".to_string(), GameAction::OpenQuestLog);
        map.insert("KeyM".to_string(), GameAction::OpenMap);
        // Systeme
        map.insert("Escape".to_string(), GameAction::Pause);
        map.insert("Enter".to_string(), GameAction::ToggleChat);
        // Belt
        map.insert("Digit5".to_string(), GameAction::BeltPotion1);
        map.insert("Digit6".to_string(), GameAction::BeltPotion2);
        map.insert("Digit7".to_string(), GameAction::BeltPotion3);
        map.insert("Digit8".to_string(), GameAction::BeltPotion4);
        Self { map }
    }

    /// Charge depuis un fichier JSON (config utilisateur)
    pub fn load_from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Sauvegarde vers JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    /// Resout une touche vers une GameAction (si mappee)
    pub fn resolve_key(&self, code: &str) -> Option<GameAction> {
        self.map.get(code).copied()
    }
}
```

---

## 4. InputEvent -- evenements jeu

```rust
// sd-input/src/events.rs
use winit::dpi::PhysicalPosition;
use crate::actions::GameAction;

/// Evenements d'entree normalises pour le jeu
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Touche clavier pressee -> action
    ActionPressed(GameAction),
    /// Touche clavier relachee -> action
    ActionReleased(GameAction),
    /// Clic gauche : move-to ou attack
    LeftClick { screen_x: f32, screen_y: f32 },
    /// Clic droit : skill secondaire
    RightClick { screen_x: f32, screen_y: f32 },
    /// Mouvement souris (pour le hover)
    MouseMoved { screen_x: f32, screen_y: f32 },
    /// Scroll molette (zoom minimap)
    MouseScroll { delta: f32 },
    /// Fenetre redimensionnee
    WindowResized { width: u32, height: u32 },
    /// Fermeture demandee
    Quit,
}
```

---

## 5. InputProcessor -- conversion winit vers InputEvent

```rust
// sd-input/src/lib.rs
pub mod actions;
pub mod keybinding;
pub mod events;

use winit::event::{WindowEvent, MouseButton, ElementState};
use winit::keyboard::KeyCode;
use events::InputEvent;
use keybinding::Keybindings;

pub struct InputProcessor {
    pub bindings: Keybindings,
    pub mouse_pos: (f32, f32),
}

impl InputProcessor {
    pub fn new(bindings: Keybindings) -> Self {
        Self { bindings, mouse_pos: (0.0, 0.0) }
    }

    /// Convertit un WindowEvent winit en InputEvent(s) jeu.
    pub fn process(&mut self, event: &WindowEvent) -> Vec<InputEvent> {
        let mut out = Vec::new();

        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let key_str = format!("{:?}", event.physical_key);
                if let Some(action) = self.bindings.resolve_key(&key_str) {
                    match event.state {
                        ElementState::Pressed  => out.push(InputEvent::ActionPressed(action)),
                        ElementState::Released => out.push(InputEvent::ActionReleased(action)),
                    }
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                let (sx, sy) = self.mouse_pos;
                match (button, state) {
                    (MouseButton::Left, ElementState::Pressed) => {
                        out.push(InputEvent::LeftClick { screen_x: sx, screen_y: sy });
                    }
                    (MouseButton::Right, ElementState::Pressed) => {
                        out.push(InputEvent::RightClick { screen_x: sx, screen_y: sy });
                    }
                    _ => {}
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                self.mouse_pos = (x, y);
                out.push(InputEvent::MouseMoved { screen_x: x, screen_y: y });
            }

            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => *y,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32 / 100.0,
                };
                out.push(InputEvent::MouseScroll { delta: scroll });
            }

            WindowEvent::Resized(size) => {
                out.push(InputEvent::WindowResized {
                    width: size.width,
                    height: size.height,
                });
            }

            WindowEvent::CloseRequested => {
                out.push(InputEvent::Quit);
            }

            _ => {}
        }

        out
    }
}
```

---

## 6. AudioError

```rust
// sd-audio/src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("kira error: {0}")]
    Kira(String),
    #[error("Sound not found: {0}")]
    NotFound(String),
    #[error("Decode error: {0}")]
    Decode(String),
}

pub type AudioResult<T> = Result<T, AudioError>;

pub use sound_bank::SoundBank;
pub use bgm::BgmPlayer;

mod sound_bank;
mod bgm;
```

---

## 7. SoundBank -- pool de sons

```rust
// sd-audio/src/sound_bank.rs
use std::collections::HashMap;
use std::path::Path;
use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::cpal::CpalBackend},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use crate::{AudioResult, AudioError};

/// Sons precharges en memoire (SFX courts)
pub struct SoundBank {
    manager: AudioManager<CpalBackend>,
    sounds: HashMap<String, StaticSoundData>,
}

impl SoundBank {
    pub fn new() -> AudioResult<Self> {
        let manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())
            .map_err(|e| AudioError::Kira(e.to_string()))?;
        Ok(Self { manager, sounds: HashMap::new() })
    }

    /// Charge un fichier OGG/WAV dans le bank
    pub fn load(&mut self, name: &str, path: &Path) -> AudioResult<()> {
        let data = StaticSoundData::from_file(path)
            .map_err(|e| AudioError::Decode(e.to_string()))?;
        self.sounds.insert(name.to_string(), data);
        Ok(())
    }

    /// Joue un son par son nom (non-bloquant)
    pub fn play(&mut self, name: &str) -> AudioResult<()> {
        let data = self.sounds.get(name)
            .ok_or_else(|| AudioError::NotFound(name.to_string()))?;
        self.manager.play(data.clone())
            .map_err(|e| AudioError::Kira(e.to_string()))?;
        Ok(())
    }

    /// Joue un son avec volume (0.0 = muet, 1.0 = normal)
    pub fn play_volume(&mut self, name: &str, volume: f64) -> AudioResult<()> {
        let data = self.sounds.get(name)
            .ok_or_else(|| AudioError::NotFound(name.to_string()))?;
        let settings = StaticSoundSettings::new().volume(volume);
        self.manager.play(data.clone().with_settings(settings))
            .map_err(|e| AudioError::Kira(e.to_string()))?;
        Ok(())
    }

    /// Prechargement des SFX de combat
    pub fn load_combat_sfx(&mut self, assets_dir: &Path) -> AudioResult<()> {
        let sfx = [
            ("hit_flesh", "sfx/hit_flesh.ogg"),
            ("hit_armor", "sfx/hit_armor.ogg"),
            ("death_generic", "sfx/death_generic.ogg"),
            ("potion_drink", "sfx/potion_drink.ogg"),
            ("item_pickup", "sfx/item_pickup.ogg"),
            ("gold_pickup", "sfx/gold_pickup.ogg"),
            ("skill_cast", "sfx/skill_cast.ogg"),
            ("bow_fire", "sfx/bow_fire.ogg"),
            ("arrow_hit", "sfx/arrow_hit.ogg"),
            ("level_up", "sfx/level_up.ogg"),
        ];
        for (name, rel_path) in &sfx {
            let full = assets_dir.join(rel_path);
            if full.exists() {
                self.load(name, &full)?;
            } else {
                log::warn!("SFX not found: {}", full.display());
            }
        }
        Ok(())
    }
}
```

---

## 8. BgmPlayer -- musique par acte

```rust
// sd-audio/src/bgm.rs
use std::collections::HashMap;
use std::path::Path;
use kira::{
    manager::{AudioManager, backend::cpal::CpalBackend},
    sound::streaming::{StreamingSoundData, StreamingSoundSettings, StreamingSoundHandle},
    tween::Tween,
};
use std::time::Duration;
use crate::{AudioResult, AudioError};

/// BGM Sodomight : une piste par acte x ambiance (combat/explore)
pub struct BgmPlayer {
    manager: AudioManager<CpalBackend>,
    tracks: HashMap<String, std::path::PathBuf>,
    current: Option<StreamingSoundHandle<()>>,
    current_key: Option<String>,
}

impl BgmPlayer {
    pub fn new() -> AudioResult<Self> {
        let manager = AudioManager::<CpalBackend>::new(Default::default())
            .map_err(|e| AudioError::Kira(e.to_string()))?;
        Ok(Self { manager, tracks: HashMap::new(), current: None, current_key: None })
    }

    /// Enregistre les pistes BGM
    pub fn register(&mut self, key: &str, path: &Path) {
        self.tracks.insert(key.to_string(), path.to_path_buf());
    }

    /// Charge les BGM des 4 actes
    pub fn load_sodomight_bgm(&mut self, assets_dir: &Path) {
        let tracks = [
            ("act1_explore",  "bgm/act1_rogue_encampment.ogg"),
            ("act1_combat",   "bgm/act1_combat.ogg"),
            ("act1_dungeon",  "bgm/act1_cave.ogg"),
            ("act2_explore",  "bgm/act2_lut_gholein.ogg"),
            ("act2_combat",   "bgm/act2_combat.ogg"),
            ("act2_tomb",     "bgm/act2_tomb.ogg"),
            ("act3_explore",  "bgm/act3_kurast_docks.ogg"),
            ("act3_combat",   "bgm/act3_combat.ogg"),
            ("act4_explore",  "bgm/act4_pandemonium.ogg"),
            ("act4_combat",   "bgm/act4_combat.ogg"),
            ("main_menu",     "bgm/main_menu.ogg"),
            ("char_select",   "bgm/character_select.ogg"),
        ];
        for (key, rel) in &tracks {
            self.register(key, &assets_dir.join(rel));
        }
    }

    /// Change la piste en cours avec cross-fade
    pub fn play(&mut self, key: &str) -> AudioResult<()> {
        if self.current_key.as_deref() == Some(key) {
            return Ok(()); // deja en cours
        }

        // Fade out la piste actuelle
        if let Some(handle) = &mut self.current {
            let fade = Tween { duration: Duration::from_millis(800), ..Default::default() };
            let _ = handle.set_volume(0.0, fade);
        }

        let path = self.tracks.get(key)
            .ok_or_else(|| AudioError::NotFound(key.to_string()))?
            .clone();

        if !path.exists() {
            log::warn!("BGM not found: {}", path.display());
            self.current = None;
            self.current_key = None;
            return Ok(());
        }

        let settings = StreamingSoundSettings::new()
            .loop_region(0.0..);

        let data = StreamingSoundData::from_file(path)
            .map_err(|e| AudioError::Decode(e.to_string()))?
            .with_settings(settings);

        let handle = self.manager.play(data)
            .map_err(|e| AudioError::Kira(e.to_string()))?;

        self.current = Some(handle);
        self.current_key = Some(key.to_string());
        Ok(())
    }

    /// Arret immediat
    pub fn stop(&mut self) {
        if let Some(handle) = &mut self.current {
            let _ = handle.stop(Tween { duration: Duration::from_millis(300), ..Default::default() });
        }
        self.current = None;
        self.current_key = None;
    }
}
```

---

## 9. AudioContext -- facade unifiee

```rust
// sd-audio/src/lib.rs (suite)

/// Facade audio -- SFX + BGM dans un seul objet partage
pub struct AudioContext {
    pub sfx: SoundBank,
    pub bgm: BgmPlayer,
}

impl AudioContext {
    pub fn init(assets_dir: &Path) -> AudioResult<Self> {
        let mut sfx = SoundBank::new()?;
        sfx.load_combat_sfx(assets_dir)?;

        let mut bgm = BgmPlayer::new()?;
        bgm.load_sodomight_bgm(assets_dir);

        Ok(Self { sfx, bgm })
    }

    /// A appeler lors du changement de zone
    pub fn on_zone_enter(&mut self, act: u8, in_combat: bool) {
        let key = if in_combat {
            format!("act{}_combat", act)
        } else {
            format!("act{}_explore", act)
        };
        if let Err(e) = self.bgm.play(&key) {
            log::warn!("BGM play error: {}", e);
        }
    }

    /// Raccourcis SFX courants
    pub fn play_hit(&mut self, is_armored: bool) {
        let name = if is_armored { "hit_armor" } else { "hit_flesh" };
        let _ = self.sfx.play(name);
    }

    pub fn play_death(&mut self) { let _ = self.sfx.play("death_generic"); }
    pub fn play_pickup_item(&mut self) { let _ = self.sfx.play("item_pickup"); }
    pub fn play_pickup_gold(&mut self) { let _ = self.sfx.play("gold_pickup"); }
    pub fn play_level_up(&mut self) { let _ = self.sfx.play("level_up"); }
    pub fn play_potion(&mut self) { let _ = self.sfx.play("potion_drink"); }
}
```

---

## 10. Integration dans la boucle winit

```rust
// sd-client/src/main.rs (extrait)
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use sd_input::{InputProcessor, keybinding::Keybindings};
use sd_audio::AudioContext;

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Sodomight")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();

    // Input
    let keybindings = Keybindings::default_d2();
    let mut input = InputProcessor::new(keybindings);

    // Audio
    let assets = std::path::PathBuf::from("assets");
    let mut audio = AudioContext::init(&assets).expect("Audio init failed");
    audio.bgm.play("main_menu").ok();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match &event {
            Event::WindowEvent { event: win_event, .. } => {
                let game_events = input.process(win_event);
                for ge in game_events {
                    match ge {
                        sd_input::events::InputEvent::Quit => elwt.exit(),
                        sd_input::events::InputEvent::LeftClick { screen_x, screen_y } => {
                            // Convertir en coord monde -> commande move
                            log::debug!("Left click at ({}, {})", screen_x, screen_y);
                        }
                        sd_input::events::InputEvent::ActionPressed(action) => {
                            log::debug!("Action: {:?}", action);
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // Render frame
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
}
```

---

## 11. Tests

```rust
// sd-input/src/tests.rs
#[cfg(test)]
mod tests {
    use crate::{InputProcessor, keybinding::Keybindings, actions::GameAction};

    #[test]
    fn test_keybinding_default_resolve() {
        let kb = Keybindings::default_d2();
        assert_eq!(kb.resolve_key("KeyI"), Some(GameAction::OpenInventory));
        assert_eq!(kb.resolve_key("Digit1"), Some(GameAction::SkillSlot1));
        assert_eq!(kb.resolve_key("Escape"), Some(GameAction::Pause));
        assert_eq!(kb.resolve_key("NonExistent"), None);
    }

    #[test]
    fn test_keybinding_json_roundtrip() {
        let kb = Keybindings::default_d2();
        let json = kb.to_json();
        let kb2 = Keybindings::load_from_json(&json).unwrap();
        assert_eq!(kb2.resolve_key("KeyI"), Some(GameAction::OpenInventory));
    }

    #[test]
    fn test_iso_screen_conversion() {
        use crate::iso::IsoCoord; // from sd-renderer
        // Tuile (0,0) -> ecran (0,0)
        let (sx, sy) = IsoCoord { tx: 0, ty: 0 }.to_screen();
        assert!((sx - 0.0).abs() < 0.001);
        assert!((sy - 0.0).abs() < 0.001);

        // Tuile (1,0) -> sx = 32, sy = 16
        let (sx, sy) = IsoCoord { tx: 1, ty: 0 }.to_screen();
        assert!((sx - 32.0).abs() < 0.001);
        assert!((sy - 16.0).abs() < 0.001);
    }
}
```

---

## 12. Checklist integration

- [ ] `sd-input` ajoute au workspace, keybindings par defaut charges
- [ ] `sd-audio` ajoute au workspace, `AudioContext::init()` ne panique pas sans assets
- [ ] Fichiers OGG dans `assets/sfx/` et `assets/bgm/`
- [ ] `InputProcessor::process()` appele pour chaque `WindowEvent` dans la event loop winit
- [ ] Clic gauche sur la carte -> log "Left click at (x, y)"
- [ ] Touche `I` -> `OpenInventory` visible dans les logs
- [ ] BGM `main_menu.ogg` joue au lancement (si fichier present)
- [ ] Pas de crash si un fichier audio manque (warning log uniquement)
- [ ] `cargo test -p sd-input -- --nocapture` : tests passent
- [ ] `cargo clippy -p sd-input -p sd-audio -- -D warnings` : aucun warning

---

*Fin IMPL-08 -- Input winit + Audio kira. Voir IMPL-09 pour le scripting Rhai.*
