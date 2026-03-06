use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RuntimeStage {
    Boot,
    Town,
    Field,
    Pause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub tick_rate_hz: u32,
    pub startup_scene: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self { tick_rate_hz: 60, startup_scene: "act1_rogue_camp".to_owned() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSummary {
    pub id: String,
    pub biome: String,
    pub ambient_rgb: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct FixedStep {
    tick_rate_hz: u32,
    dt: Duration,
}

impl FixedStep {
    pub fn new(tick_rate_hz: u32) -> Self {
        let dt = Duration::from_secs_f64(1.0 / f64::from(tick_rate_hz.max(1)));
        Self { tick_rate_hz, dt }
    }

    pub fn dt(&self) -> Duration {
        self.dt
    }

    pub fn tick_rate_hz(&self) -> u32 {
        self.tick_rate_hz
    }
}

#[derive(Debug, Clone)]
pub struct EngineRuntime {
    config: RuntimeConfig,
    step: FixedStep,
    scene: SceneSummary,
    tick_index: u64,
    stage: RuntimeStage,
}

impl EngineRuntime {
    pub fn new(config: RuntimeConfig, scene: SceneSummary) -> Self {
        Self {
            step: FixedStep::new(config.tick_rate_hz),
            config,
            scene,
            tick_index: 0,
            stage: RuntimeStage::Boot,
        }
    }

    pub fn enter_stage(&mut self, stage: RuntimeStage) {
        self.stage = stage;
    }

    pub fn tick(&mut self) {
        self.tick_index += 1;
        if self.tick_index > 1 {
            self.stage = RuntimeStage::Town;
        }
    }

    pub fn tick_index(&self) -> u64 {
        self.tick_index
    }

    pub fn step(&self) -> &FixedStep {
        &self.step
    }

    pub fn scene(&self) -> &SceneSummary {
        &self.scene
    }

    pub fn stage(&self) -> RuntimeStage {
        self.stage
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_step_defaults_to_sixty_hz() {
        let step = FixedStep::new(60);
        assert_eq!(step.tick_rate_hz(), 60);
        assert!(step.dt().as_millis() >= 16);
    }

    #[test]
    fn runtime_advances_ticks() {
        let scene = SceneSummary {
            id: "camp".into(),
            biome: "rogue_camp".into(),
            ambient_rgb: [0.1, 0.08, 0.05],
        };
        let mut runtime = EngineRuntime::new(RuntimeConfig::default(), scene);
        runtime.tick();
        runtime.tick();
        assert_eq!(runtime.tick_index(), 2);
        assert_eq!(runtime.stage(), RuntimeStage::Town);
    }
}
