use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventResult};
use tokio::sync::broadcast;

/// Lance un file watcher sur `watch_path`.
/// Retourne un receiver de broadcast : chaque evenement = slug ou chemin modifie.
pub fn start_watcher(
    watch_path: &Path,
    tx: broadcast::Sender<String>,
) -> anyhow::Result<()> {
    let watch_path = watch_path.to_path_buf();
    let tx_clone = tx.clone();

    std::thread::spawn(move || {
        let tx_inner = Arc::new(Mutex::new(tx_clone));
        let tx_cb = Arc::clone(&tx_inner);

        let mut debouncer = match new_debouncer(
            Duration::from_millis(500),
            move |res: DebounceEventResult| {
                if let Ok(events) = res {
                    for event in events {
                        let path_str = event.path.to_string_lossy().to_string();
                        // Extraire le slug depuis le chemin : sequences/<date-slug>/...
                        let slug = extract_slug(&path_str).unwrap_or_else(|| path_str.clone());
                        if let Ok(tx) = tx_cb.lock() {
                            let _ = tx.send(slug);
                        }
                    }
                }
            },
        ) {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Watcher init error: {e}");
                return;
            }
        };

        if let Err(e) = debouncer.watcher().watch(&watch_path, RecursiveMode::Recursive) {
            tracing::error!("Watcher watch error: {e}");
            return;
        }

        tracing::info!("File watcher actif sur : {}", watch_path.display());
        // Garder le thread en vie (debouncer drop = arret)
        loop { std::thread::sleep(Duration::from_secs(60)); }
    });

    Ok(())
}

fn extract_slug(path: &str) -> Option<String> {
    // Cherche un segment qui ressemble a YYYY-MM-DD-<slug>
    let normalized = path.replace('\\', "/");
    let parts: Vec<&str> = normalized.split('/').collect();
    parts.iter().find(|&&p| {
        let chars: Vec<char> = p.chars().collect();
        chars.len() > 10
            && chars[4] == '-'
            && chars[7] == '-'
            && chars[..4].iter().all(|c| c.is_ascii_digit())
    }).map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_slug() {
        let path = "C:/Users/miyuk/Cursor/Miyukini-COG/.mip/sequences/2026-03-06-mipower-workflow-editor/phases/p0/temps/temps-01.md";
        let slug = extract_slug(path);
        assert_eq!(slug.as_deref(), Some("2026-03-06-mipower-workflow-editor"));
    }

    #[test]
    fn test_extract_slug_no_match() {
        let path = "C:/some/random/path/file.md";
        let slug = extract_slug(path);
        assert!(slug.is_none());
    }
}
