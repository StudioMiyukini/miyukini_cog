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
            let msg = format!("Lecture voix Miou: {} — {}", path.display(), e);
            tracing::warn!("{}", msg);
            eprintln!("[Central] {}", msg);
        }
    });
}

/// Joue un fichier voix en résolvant le chemin depuis la base, puis lance la lecture en arrière-plan.
pub fn play_voice_background(base: &Path, filename: &str) {
    let path = resolve_voice_path(base, filename);
    if !path.exists() {
        let msg = format!("Fichier voix introuvable: {}", path.display());
        tracing::warn!("{}", msg);
        eprintln!("[Central] {}", msg);
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
                return Ok(());
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
