use mge_core::{EngineRuntime, RuntimeConfig, RuntimeStage, SceneSummary};
use mge_net::NetMode;
use mge_proto::{ServerEvent, PROTOCOL_VERSION};

#[derive(Debug, Clone)]
pub struct ServerBootstrap {
    pub net_mode: NetMode,
    pub protocol_version: u32,
}

impl Default for ServerBootstrap {
    fn default() -> Self {
        Self { net_mode: NetMode::OfflineLocal, protocol_version: PROTOCOL_VERSION }
    }
}

#[derive(Debug)]
pub struct AuthoritativeSim {
    bootstrap: ServerBootstrap,
    runtime: EngineRuntime,
}

impl AuthoritativeSim {
    pub fn new(config: RuntimeConfig, scene: SceneSummary) -> Self {
        let mut runtime = EngineRuntime::new(config, scene);
        runtime.enter_stage(RuntimeStage::Boot);
        Self { bootstrap: ServerBootstrap::default(), runtime }
    }

    pub fn bootstrap(&self) -> &ServerBootstrap {
        &self.bootstrap
    }

    pub fn runtime(&self) -> &EngineRuntime {
        &self.runtime
    }

    pub fn tick(&mut self) -> ServerEvent {
        self.runtime.tick();
        ServerEvent::TickAcknowledged { tick: self.runtime.tick_index() }
    }
}
