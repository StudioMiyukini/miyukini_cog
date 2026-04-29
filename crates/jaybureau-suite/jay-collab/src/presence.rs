//! Gestion de la présence (cursors, sélections, utilisateurs en ligne).

use jaybureau_core::{Presence, UserId};
use std::collections::HashMap;
use std::sync::RwLock;

/// Palette de couleurs distinctes pour assigner aux utilisateurs.
const USER_COLORS: &[&str] = &[
    "#ef4444", "#f97316", "#eab308", "#22c55e", "#14b8a6", "#3b82f6", "#8b5cf6", "#ec4899",
    "#06b6d4", "#84cc16", "#f59e0b", "#a855f7",
];

/// Gestionnaire de présence pour un document.
pub struct PresenceManager {
    local_user_id: UserId,
    local_display_name: String,
    local_color: String,
    local: RwLock<Presence>,
    /// Autres utilisateurs présents.
    remote: RwLock<HashMap<UserId, Presence>>,
}

impl PresenceManager {
    pub fn new(user_id: UserId, display_name: String) -> Self {
        let color = color_for_user(&user_id).to_string();
        let local = Presence {
            display_name: display_name.clone(),
            color: color.clone(),
            cursor: None,
            selection: None,
        };
        Self {
            local_user_id: user_id,
            local_display_name: display_name,
            local_color: color,
            local: RwLock::new(local),
            remote: RwLock::new(HashMap::new()),
        }
    }

    /// Retourne la présence locale.
    pub fn local_presence(&self) -> Presence {
        self.local.read().map(|p| p.clone()).unwrap_or_default()
    }

    /// Met à jour la présence locale (cursor, sélection).
    pub fn set_local(&self, presence: Presence) {
        if let Ok(mut lock) = self.local.write() {
            *lock = Presence {
                display_name: self.local_display_name.clone(),
                color: self.local_color.clone(),
                cursor: presence.cursor,
                selection: presence.selection,
            };
        }
    }

    /// Met à jour la présence d'un utilisateur distant.
    pub fn update_remote(&self, user_id: UserId, presence: Presence) {
        if user_id == self.local_user_id {
            return; // Ignore les echos
        }
        if let Ok(mut lock) = self.remote.write() {
            lock.insert(user_id, presence);
        }
    }

    /// Retire un utilisateur distant (a quitté).
    pub fn remove_remote(&self, user_id: &str) {
        if let Ok(mut lock) = self.remote.write() {
            lock.remove(user_id);
        }
    }

    /// Retourne la liste des utilisateurs distants et leurs présences.
    pub fn remote_participants(&self) -> Vec<(UserId, Presence)> {
        self.remote
            .read()
            .map(|lock| lock.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    /// Nombre total de participants (local + distants).
    pub fn participant_count(&self) -> usize {
        1 + self.remote.read().map(|r| r.len()).unwrap_or(0)
    }
}

/// Assigne une couleur déterministe à un user_id (basée sur le hash).
fn color_for_user(user_id: &str) -> &'static str {
    let mut hash = 0u32;
    for b in user_id.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(b as u32);
    }
    USER_COLORS[(hash as usize) % USER_COLORS.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_color() {
        let c1 = color_for_user("alice");
        let c2 = color_for_user("alice");
        assert_eq!(c1, c2);

        let c3 = color_for_user("bob");
        // Différents utilisateurs peuvent avoir la même couleur (palette limitée)
        // mais le même utilisateur doit toujours avoir la même.
        let _ = c3;
    }

    #[test]
    fn remote_participants() {
        let mgr = PresenceManager::new("alice".into(), "Alice".into());
        assert_eq!(mgr.participant_count(), 1);

        mgr.update_remote(
            "bob".into(),
            Presence {
                display_name: "Bob".into(),
                color: "#000000".into(),
                ..Default::default()
            },
        );
        assert_eq!(mgr.participant_count(), 2);

        // L'user local ne doit pas apparaître dans les remote
        mgr.update_remote("alice".into(), Presence::default());
        assert_eq!(mgr.participant_count(), 2);

        mgr.remove_remote("bob");
        assert_eq!(mgr.participant_count(), 1);
    }
}
