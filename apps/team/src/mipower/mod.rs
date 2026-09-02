//! @id     team_mipower_mod
//! @do     declarer_module_runner
//! @role   orchestration
//! @layer  app
//! @human  team — module « mipower mod ».

pub mod runner;
pub mod sequence;

pub use runner::{MiPowerRunner, StepExecRequest, StepResult};
pub use sequence::{load_sequences, MiPowerSequence, SequenceStep};
