//! @id mge.gs.trade.v1
//! @role plugin
//! @layer plugin
//! @domain grand_strategy
//! @do scaffold_trade_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
