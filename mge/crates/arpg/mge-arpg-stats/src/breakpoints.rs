// @id: MGE-ARPG-Breakpoints @do: breakpoints @role: back-end @layer: 3 @human: miyuk
//! Breakpoint tables for animation frame calculations.
//!
//! Encodes D2-style breakpoint logic: FCR, FHR, FBR, IAS.
//! Each table maps a stat threshold to a frame count (lower = faster).

/// A breakpoint table mapping stat thresholds to animation frame counts.
///
/// Thresholds are stored in ascending order. Each threshold[i] is the
/// minimum stat value required to achieve frames[i]. If the stat is
/// below all thresholds, `base_frames` is returned.
#[derive(Debug, Clone)]
pub struct BreakpointTable {
    /// Threshold values (ascending). Each entry is a stat value needed.
    thresholds: Vec<i32>,
    /// Corresponding frame counts (descending). frames[i] = frames when stat >= thresholds[i].
    frames: Vec<u8>,
    /// Frame count when stat is below any threshold (the "base" frames).
    base_frames: u8,
}

impl BreakpointTable {
    /// Create a new breakpoint table.
    ///
    /// # Panics (tests only)
    /// Panics in debug if `thresholds.len() != frames.len()`.
    #[must_use]
    pub fn new(thresholds: Vec<i32>, frames: Vec<u8>, base_frames: u8) -> Self {
        debug_assert_eq!(
            thresholds.len(),
            frames.len(),
            "thresholds and frames must have the same length"
        );
        Self {
            thresholds,
            frames,
            base_frames,
        }
    }

    /// Return the frame count for the given stat value.
    ///
    /// Finds the highest threshold that is <= `stat_value` and returns
    /// the corresponding frame count. If the stat is below all thresholds,
    /// returns `base_frames`.
    #[must_use]
    pub fn frames_for_stat(&self, stat_value: i32) -> u8 {
        // Iterate from the highest threshold downward to find the first
        // threshold that stat_value satisfies.
        for (i, &threshold) in self.thresholds.iter().enumerate().rev() {
            if stat_value >= threshold {
                return self.frames[i];
            }
        }
        self.base_frames
    }
}

/// Necromancer FCR (Faster Cast Rate) breakpoint table.
///
/// Source: D2 community breakpoint charts for Necromancer.
/// Base: 15 frames. Best reachable: 9 frames at 125% FCR.
#[must_use]
pub fn necro_fcr() -> BreakpointTable {
    BreakpointTable::new(
        vec![9, 18, 30, 48, 75, 125],
        vec![14, 13, 12, 11, 10, 9],
        15,
    )
}

/// Necromancer FHR (Faster Hit Recovery) breakpoint table.
///
/// Source: D2 community breakpoint charts for Necromancer.
/// Base: 13 frames. Best reachable: 5 frames at 152% FHR.
#[must_use]
pub fn necro_fhr() -> BreakpointTable {
    BreakpointTable::new(
        vec![5, 10, 16, 26, 39, 56, 86, 152],
        vec![12, 11, 10, 9, 8, 7, 6, 5],
        13,
    )
}

/// Necromancer FBR (Faster Block Rate) breakpoint table.
///
/// Source: D2 community breakpoint charts for Necromancer.
/// Base: 5 frames. Best reachable: 0 frames at 52% FBR.
#[must_use]
pub fn necro_fbr() -> BreakpointTable {
    BreakpointTable::new(
        vec![6, 13, 20, 32, 52, 86],
        vec![4, 3, 2, 1, 0, 0],
        5,
    )
}

/// Necromancer IAS (Increased Attack Speed) breakpoint table.
///
/// Source: D2 community breakpoint charts for Necromancer.
/// Base: 17 frames. Best reachable: 12 frames at 86% IAS.
#[must_use]
pub fn necro_ias() -> BreakpointTable {
    BreakpointTable::new(
        vec![0, 11, 26, 48, 86],
        vec![16, 15, 14, 13, 12],
        17,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fcr_necro_75() {
        let table = necro_fcr();
        assert_eq!(table.frames_for_stat(75), 10);
    }

    #[test]
    fn fcr_necro_0() {
        let table = necro_fcr();
        assert_eq!(table.frames_for_stat(0), 15);
    }

    #[test]
    fn fhr_necro_0() {
        let table = necro_fhr();
        assert_eq!(table.frames_for_stat(0), 13);
    }

    #[test]
    fn fhr_necro_152() {
        let table = necro_fhr();
        assert_eq!(table.frames_for_stat(152), 5);
    }

    #[test]
    fn breakpoint_exact_threshold() {
        // Exactly 9% FCR = 14 frames (on threshold = uses that breakpoint)
        let table = necro_fcr();
        assert_eq!(table.frames_for_stat(9), 14);
    }

    #[test]
    fn breakpoint_between() {
        // 15% FCR is between threshold 9 and 18 — uses threshold 9 = 14 frames
        let table = necro_fcr();
        assert_eq!(table.frames_for_stat(15), 14);
    }
}
