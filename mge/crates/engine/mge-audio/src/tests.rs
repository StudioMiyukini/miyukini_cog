//! Unit tests for mge-audio -- all hardware-free.

#[cfg(test)]
mod tests {
    use crate::bank::SoundBank;
    use crate::bgm::BgmPlayer;
    use crate::config::AudioConfig;
    use crate::manager::AudioManager;
    use crate::sfx::SfxPool;

    // ── SoundBank ───────────────────────────────────────────────

    #[test]
    fn sound_bank_register_and_get() {
        let mut bank = SoundBank::new();
        bank.register("sword_hit", "assets/sfx/sword_hit.ogg");
        assert_eq!(bank.get_path("sword_hit"), Some("assets/sfx/sword_hit.ogg"));
    }

    #[test]
    fn sound_bank_missing_returns_none() {
        let bank = SoundBank::new();
        assert_eq!(bank.get_path("nonexistent"), None);
    }

    #[test]
    fn sound_bank_len() {
        let mut bank = SoundBank::new();
        assert_eq!(bank.len(), 0);
        assert!(bank.is_empty());
        bank.register("a", "a.ogg");
        bank.register("b", "b.ogg");
        assert_eq!(bank.len(), 2);
        assert!(!bank.is_empty());
    }

    // ── BgmPlayer ───────────────────────────────────────────────

    #[test]
    fn bgm_player_initial_state() {
        let player = BgmPlayer::new(0.8);
        assert!(player.current_track().is_none());
        assert!(!player.is_playing());
    }

    #[test]
    fn bgm_player_play_sets_track() {
        let mut player = BgmPlayer::new(0.7);
        player.play("act1_explore");
        assert_eq!(player.current_track(), Some("act1_explore"));
        assert!(player.is_playing());
    }

    #[test]
    fn bgm_player_stop_clears() {
        let mut player = BgmPlayer::new(0.7);
        player.play("act1_explore");
        player.stop();
        assert!(player.current_track().is_none());
        assert!(!player.is_playing());
    }

    #[test]
    fn bgm_player_volume_clamp() {
        let mut player = BgmPlayer::new(0.5);
        player.set_volume(-1.0);
        assert!((player.volume() - 0.0).abs() < f32::EPSILON);
        player.set_volume(2.0);
        assert!((player.volume() - 1.0).abs() < f32::EPSILON);
        player.set_volume(0.42);
        assert!((player.volume() - 0.42).abs() < f32::EPSILON);
    }

    // ── SfxPool ─────────────────────────────────────────────────

    #[test]
    fn sfx_pool_play_increments_count() {
        let mut pool = SfxPool::new(16);
        assert_eq!(pool.active_count(), 0);
        pool.play("hit_flesh");
        pool.play("hit_armor");
        assert_eq!(pool.active_count(), 2);
    }

    #[test]
    fn sfx_pool_max_concurrent_evicts_oldest() {
        let max = 10;
        let mut pool = SfxPool::new(max);
        for i in 0..20 {
            pool.play(&format!("sfx_{i}"));
        }
        assert_eq!(pool.active_count(), max);
    }

    #[test]
    fn sfx_pool_clear_resets() {
        let mut pool = SfxPool::new(8);
        pool.play("a");
        pool.play("b");
        pool.clear();
        assert_eq!(pool.active_count(), 0);
    }

    // ── AudioManager ────────────────────────────────────────────

    #[test]
    fn audio_manager_register_and_play_bgm() {
        let config = AudioConfig::default();
        let mut mgr = AudioManager::new(config);

        mgr.register_sound("act1_explore", "bgm/act1.ogg");
        assert_eq!(
            mgr.sound_bank().get_path("act1_explore"),
            Some("bgm/act1.ogg")
        );

        mgr.play_bgm("act1_explore");
        assert!(mgr.bgm_player().is_playing());
        assert_eq!(mgr.bgm_player().current_track(), Some("act1_explore"));

        mgr.stop_bgm();
        assert!(!mgr.bgm_player().is_playing());
    }

    // ── AudioConfig ─────────────────────────────────────────────

    #[test]
    fn audio_config_defaults() {
        let cfg = AudioConfig::default();
        assert!((cfg.master_volume - 1.0).abs() < f32::EPSILON);
        assert!((cfg.bgm_volume - 0.7).abs() < f32::EPSILON);
        assert!((cfg.sfx_volume - 1.0).abs() < f32::EPSILON);
        assert_eq!(cfg.max_sfx_concurrent, 16);
    }
}
