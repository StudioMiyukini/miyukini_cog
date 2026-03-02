// @id: MGE-Render-Animation-IntegTest @do: test @role: back-end @layer: 2 @human: francois

//! Integration test: full animation pipeline from TOML to controller tick.

use mge_render::animation::{
    AnimationBank, AnimationController, AnimationState, Direction, FrameEventKind,
};

/// Full bank with Idle (looping) and Walk (looping), 4 rendered directions each.
const FULL_BANK_TOML: &str = r#"
entity_id = "test_warrior"

# --- Idle: 4 rendered directions, 8 frames each ---

[[clips]]
state = "Idle"
direction = "S"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 0

[[clips]]
state = "Idle"
direction = "SW"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 8

[[clips]]
state = "Idle"
direction = "W"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 16

[[clips]]
state = "Idle"
direction = "NW"
frame_count = 8
frame_duration_ms = 120
looping = true
atlas_start_frame = 24

# --- Walk: 4 rendered directions, 8 frames each, with SFX events ---

[[clips]]
state = "Walk"
direction = "S"
frame_count = 8
frame_duration_ms = 100
looping = true
atlas_start_frame = 32
events = [
    { frame = 0, kind = "Sfx", data = "footstep_l" },
    { frame = 4, kind = "Sfx", data = "footstep_r" },
]

[[clips]]
state = "Walk"
direction = "SW"
frame_count = 8
frame_duration_ms = 100
looping = true
atlas_start_frame = 40
events = [
    { frame = 0, kind = "Sfx", data = "footstep_l" },
    { frame = 4, kind = "Sfx", data = "footstep_r" },
]

[[clips]]
state = "Walk"
direction = "W"
frame_count = 8
frame_duration_ms = 100
looping = true
atlas_start_frame = 48
events = [
    { frame = 0, kind = "Sfx", data = "footstep_l" },
    { frame = 4, kind = "Sfx", data = "footstep_r" },
]

[[clips]]
state = "Walk"
direction = "NW"
frame_count = 8
frame_duration_ms = 100
looping = true
atlas_start_frame = 56
events = [
    { frame = 0, kind = "Sfx", data = "footstep_l" },
    { frame = 4, kind = "Sfx", data = "footstep_r" },
]
"#;

#[test]
fn full_pipeline_idle_2_seconds() {
    let bank = AnimationBank::from_toml(FULL_BANK_TOML).unwrap();
    assert_eq!(bank.entity_id, "test_warrior");
    assert_eq!(bank.clips.len(), 8);

    let mut ctrl = AnimationController::new();

    // Verify initial state.
    assert_eq!(ctrl.current_state(), AnimationState::Idle);
    assert_eq!(ctrl.current_direction(), Direction::S);
    assert_eq!(ctrl.current_frame(), 0);
    assert_eq!(ctrl.current_atlas_frame(&bank), Some(0));

    // Simulate 2 seconds at 16ms ticks (~125 ticks).
    let mut total_ms = 0u32;
    for _ in 0..125 {
        ctrl.tick(16, &bank);
        total_ms += 16;
    }
    assert_eq!(total_ms, 2000);

    // After 2000ms with Idle at 120ms/frame, 8 frames looping:
    // The animation should have looped multiple times (2000 / 960 ~= 2 full loops).
    // The exact frame depends on rounding, but it should be valid (0..7).
    assert!(ctrl.current_frame() < 8, "frame should be 0..7");
    assert_eq!(ctrl.current_state(), AnimationState::Idle);
}

#[test]
fn full_pipeline_state_transition() {
    let bank = AnimationBank::from_toml(FULL_BANK_TOML).unwrap();
    let mut ctrl = AnimationController::new();

    // Start in Idle, tick a bit.
    ctrl.tick(300, &bank);
    assert_eq!(ctrl.current_state(), AnimationState::Idle);
    assert!(ctrl.current_frame() > 0, "should have advanced");

    // Transition to Walk.
    ctrl.set_state(AnimationState::Walk);
    assert_eq!(ctrl.current_state(), AnimationState::Walk);
    assert_eq!(ctrl.current_frame(), 0, "frame resets on state change");

    // Tick in Walk state.
    ctrl.tick(500, &bank);
    assert!(ctrl.current_frame() > 0, "should advance in Walk");
    assert_eq!(ctrl.current_state(), AnimationState::Walk);
}

#[test]
fn full_pipeline_direction_changes_and_mirroring() {
    let bank = AnimationBank::from_toml(FULL_BANK_TOML).unwrap();
    let mut ctrl = AnimationController::new();

    // Walk facing East (mirrored from W).
    ctrl.set_state(AnimationState::Walk);
    ctrl.set_direction(Direction::E);
    assert!(ctrl.is_mirrored());

    // The atlas frame should come from the W clip (atlas_start_frame = 48).
    assert_eq!(ctrl.current_atlas_frame(&bank), Some(48));

    ctrl.tick(100, &bank);
    assert_eq!(ctrl.current_frame(), 1);
    assert_eq!(ctrl.current_atlas_frame(&bank), Some(49));

    // Switch to SE (mirrored from SW, atlas_start_frame = 40).
    ctrl.set_direction(Direction::SE);
    assert!(ctrl.is_mirrored());
    assert_eq!(ctrl.current_atlas_frame(&bank), Some(41)); // frame 1 + atlas_start 40
}

#[test]
fn full_pipeline_walk_events_collected() {
    let bank = AnimationBank::from_toml(FULL_BANK_TOML).unwrap();
    let mut ctrl = AnimationController::new();
    ctrl.set_state(AnimationState::Walk);

    // Walk at 100ms/frame. Tick 400ms to reach frame 4 where footstep_r fires.
    let mut all_events = Vec::new();
    for _ in 0..4 {
        let events = ctrl.tick(100, &bank);
        all_events.extend(events);
    }

    assert_eq!(ctrl.current_frame(), 4);

    // We should have collected the footstep_r event at frame 4.
    let sfx_events: Vec<_> = all_events
        .iter()
        .filter(|e| e.kind == FrameEventKind::Sfx)
        .collect();
    assert!(
        !sfx_events.is_empty(),
        "should have collected at least one SFX event"
    );
    assert!(
        sfx_events.iter().any(|e| e.data == "footstep_r"),
        "should have footstep_r event"
    );
}

#[test]
fn full_pipeline_all_directions_resolve() {
    let bank = AnimationBank::from_toml(FULL_BANK_TOML).unwrap();

    // Every direction should resolve a clip for Idle and Walk.
    for dir in Direction::all() {
        let idle_clip = bank.get_clip(AnimationState::Idle, *dir);
        assert!(
            idle_clip.is_some(),
            "Idle clip should exist for direction {dir:?}"
        );

        let walk_clip = bank.get_clip(AnimationState::Walk, *dir);
        assert!(
            walk_clip.is_some(),
            "Walk clip should exist for direction {dir:?}"
        );
    }
}
