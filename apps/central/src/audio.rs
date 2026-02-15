//! Lecture audio (voix Miou) — joue un MP3 en arrière-plan sans bloquer l'UI.

use std::path::{Path, PathBuf};

/// Retourne le premier chemin existant vers le fichier voix, en essayant plusieurs bases.
/// Ordre : base, parent de base (workspace si base = apps/central), exe dir, exe parent.
pub fn resolve_voice_path(base: &Path, subpath: &str) -> PathBuf {
    let rel = PathBuf::from("voices").join("fr").join(subpath);
    let mut candidates: Vec<PathBuf> = vec![base.join(&rel)];
    if let Some(parent) = base.parent() {
        candidates.push(parent.join(&rel));
        if parent.parent().is_some() {
            candidates.push(parent.parent().unwrap().join(&rel));
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join(&rel));
            if let Some(grandparent) = dir.parent() {
                candidates.push(grandparent.join(&rel));
            }
        }
    }
    for path in &candidates {
        if path.exists() {
            return path.clone();
        }
    }
    base.join(rel)
}

/// Lance la lecture d'un fichier MP3 dans un thread dédié.
/// Utilise `resolve_voice_path(base, filename)` si tu passes (base, filename), sinon le path direct.
/// Les erreurs sont loguées (tracing + stderr pour visibilité).
pub fn play_mp3_background(path: PathBuf) {
    std::thread::spawn(move || {
        if let Err(e) = play_mp3_sync(&path) {
            let msg = format!("Lecture voix Miou: {} — {e}", path.display());
            tracing::warn!("{}", msg);
            eprintln!("[Central] {msg}");
        }
    });
}

/// Joue un fichier voix en résolvant le chemin depuis la base, puis lance la lecture en arrière-plan.
pub fn play_voice_background(base: &Path, filename: &str) {
    let path = resolve_voice_path(base, filename);
    if !path.exists() {
        let msg = format!("Fichier voix introuvable: {}", path.display());
        tracing::warn!("{}", msg);
        eprintln!("[Central] {msg}");
        return;
    }
    play_mp3_background(path);
}

fn play_mp3_sync(path: &PathBuf) -> Result<(), String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    match rodio_play(file, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            #[cfg(windows)]
            {
                if let Err(fb) = play_via_shell_windows(path) {
                    return Err(format!("{e}; fallback Windows: {fb}"));
                }
                Ok(())
            }
            #[cfg(not(windows))]
            Err(e)
        }
    }
}

fn rodio_play(file: std::fs::File, _path: &Path) -> Result<(), String> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|e| e.to_string())?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    let source = rodio::Decoder::try_from(file).map_err(|e| e.to_string())?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

#[cfg(windows)]
fn play_via_shell_windows(path: &Path) -> Result<(), String> {
    use std::process::Command;
    let path_str = path.to_string_lossy();
    Command::new("cmd")
        .args(["/C", "start", "/min", &path_str])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ============================================================================
// TTS eSpeak pour Miou
// ============================================================================

/// Vérifie si eSpeak-ng est disponible sur le système.
#[allow(dead_code)]
pub fn is_espeak_available() -> bool {
    std::process::Command::new("espeak-ng")
        .arg("--version")
        .output()
        .is_ok()
}

/// Lit un texte à voix haute via eSpeak-ng (TTS) en arrière-plan.
/// Ne fait rien si eSpeak-ng n'est pas installé.
#[allow(dead_code)]
pub fn play_tts_background(text: &str) {
    if !is_espeak_available() {
        tracing::debug!("eSpeak-ng non disponible, TTS ignoré");
        return;
    }

    let text = text.to_string();
    std::thread::spawn(move || {
        if let Err(e) = play_tts_sync(&text) {
            let msg = format!("TTS Miou: {e}");
            tracing::warn!("{}", msg);
            eprintln!("[Central] {msg}");
        }
    });
}

/// Joue un texte via eSpeak-ng de manière synchrone.
#[allow(dead_code)]
fn play_tts_sync(text: &str) -> Result<(), String> {
    use std::process::Command;

    // Générer un fichier WAV temporaire
    let temp_dir = std::env::temp_dir();
    let wav_path = temp_dir.join(format!("miou_tts_{}.wav", uuid::Uuid::new_v4()));

    // Appeler eSpeak-ng pour générer le WAV
    let status = Command::new("espeak-ng")
        .args([
            "-v", "fr",           // Voix française
            "-s", "150",          // Vitesse (mots/minute)
            "-p", "60",           // Pitch (hauteur, plus élevé = plus féminin)
            "-w", wav_path.to_str().unwrap(),
            "--", text,
        ])
        .status()
        .map_err(|e| format!("espeak-ng launch: {e}"))?;

    if !status.success() {
        return Err(format!("espeak-ng exit code: {:?}", status.code()));
    }

    if !wav_path.exists() {
        return Err("WAV file not created".to_string());
    }

    // Lire le WAV avec rodio
    let result = play_wav_sync(&wav_path);

    // Nettoyer le fichier temporaire
    let _ = std::fs::remove_file(&wav_path);

    result
}

/// Joue un fichier WAV de manière synchrone.
fn play_wav_sync(path: &PathBuf) -> Result<(), String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|e| e.to_string())?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    let source = rodio::Decoder::try_from(file).map_err(|e| e.to_string())?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

/// Joue un fichier WAV en arrière-plan.
#[allow(dead_code)]
pub fn play_wav_background(path: PathBuf) {
    std::thread::spawn(move || {
        if let Err(e) = play_wav_sync(&path) {
            let msg = format!("WAV playback: {} — {e}", path.display());
            tracing::warn!("{}", msg);
            eprintln!("[Central] {msg}");
        }
    });
}
