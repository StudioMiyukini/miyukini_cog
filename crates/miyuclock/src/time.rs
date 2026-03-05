//! Tools MiyuClock — tool.time.now, tool.time.delta.
//!
//! Mesure du temps locale (Kernel Clock), sans temps global (LOI-4).
//! Référence : [MiyuClock - Reference Outils](../../../docs/tools/MiyuClock/MiyuClock%20-%20Reference%20Outils.md).

use miyukini_kernel::time::{Clock, DefaultClock};
use std::time::SystemTime;

use crate::context::GovernedContext;
use crate::errors::MiyuclockError;

/// @id: miyuclock_tool_time_now
/// @role: mutator
/// @layer: tool
/// @human: Retourne l'instant présent selon l'horloge locale (Kernel Clock).
/// @do: time_now_under_governance
/// tool.time.now — pas de timezone imposée, pas de persistance.
pub fn now(ctx: &GovernedContext) -> Result<SystemTime, MiyuclockError> {
    if !ctx.has_mandate() {
        return Err(MiyuclockError::NoMandate);
    }
    let clock = DefaultClock;
    Ok(clock.now())
}

/// @id: miyuclock_tool_time_now_injected
/// @role: mutator
/// @layer: tool
/// @human: Retourne l'instant présent avec horloge injectée (tests).
/// @do: time_now_with_clock
pub fn now_with_clock<C: Clock>(
    ctx: &GovernedContext,
    clock: &C,
) -> Result<SystemTime, MiyuclockError> {
    if !ctx.has_mandate() {
        return Err(MiyuclockError::NoMandate);
    }
    Ok(clock.now())
}

/// @id: miyuclock_tool_time_delta
/// @role: mutator
/// @layer: tool
/// @human: Retourne la durée écoulée entre deux instants fournis dans le flux.
/// @do: time_delta_under_governance
/// tool.time.delta — ne décide pas, ne persiste pas.
pub fn delta(
    ctx: &GovernedContext,
    t_prev: SystemTime,
    t_now: SystemTime,
) -> Result<std::time::Duration, MiyuclockError> {
    if !ctx.has_mandate() {
        return Err(MiyuclockError::NoMandate);
    }
    t_now
        .duration_since(t_prev)
        .map_err(|_| MiyuclockError::InvalidDelta)
}
