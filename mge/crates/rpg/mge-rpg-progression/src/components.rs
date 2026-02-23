//! @id mge.rpg.progression.v1.components
//! @role data
//! @layer plugin
//! @domain rpg

use mge_ecs::Component;

/// Verrouillage d'une compétence (style UO).
/// Up = gain possible, Down = peut diminuer, Locked = bloqué.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillLock {
    Up = 0,
    Down = 1,
    Locked = 2,
}

impl Default for SkillLock {
    fn default() -> Self {
        Self::Up
    }
}

/// @id mge.rpg.progression.v1.component.skillvalue
/// @fields id:u32,base:f64,gain_factor:f64,lock:u8
/// Valeur d'une compétence : niveau de base, facteur de gain, verrou.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillValue {
    pub id: u32,
    pub base: f64,
    pub gain_factor: f64,
    pub lock: u8,
}

impl SkillValue {
    pub fn new(id: u32, base: f64, gain_factor: f64) -> Self {
        Self {
            id,
            base,
            gain_factor,
            lock: SkillLock::Up as u8,
        }
    }

    pub fn lock(&self) -> SkillLock {
        match self.lock {
            1 => SkillLock::Down,
            2 => SkillLock::Locked,
            _ => SkillLock::Up,
        }
    }
}

impl Component for SkillValue {}

/// @id mge.rpg.progression.v1.component.skillset
/// @fields skills:Vec<SkillValue>
/// Ensemble des compétences d'une entité.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SkillSet {
    pub skills: Vec<SkillValue>,
}

impl SkillSet {
    pub fn new() -> Self {
        Self {
            skills: Vec::new(),
        }
    }

    pub fn with_skills(skills: Vec<SkillValue>) -> Self {
        Self { skills }
    }

    /// Trouve la compétence par id.
    pub fn get(&self, skill_id: u32) -> Option<&SkillValue> {
        self.skills.iter().find(|s| s.id == skill_id)
    }

    /// Trouve la compétence par id, mutable.
    pub fn get_mut(&mut self, skill_id: u32) -> Option<&mut SkillValue> {
        self.skills.iter_mut().find(|s| s.id == skill_id)
    }

    /// Ajoute ou met à jour une compétence.
    pub fn set(&mut self, skill_id: u32, base: f64, gain_factor: f64) {
        if let Some(s) = self.get_mut(skill_id) {
            s.base = base;
            s.gain_factor = gain_factor;
        } else {
            self.skills.push(SkillValue::new(skill_id, base, gain_factor));
        }
    }
}

impl Component for SkillSet {}
