//! Chargement de la configuration Central (env, Supabase Catakana).
//!
//! Charge `supabase-catakana.env` depuis le répertoire courant ou la racine
//! du workspace pour exposer les identifiants Supabase sans les committer.

const ENV_FILENAME: &str = "supabase-catakana.env";

/// Configuration Supabase (projet Catakana).
///
/// Chargée depuis `supabase-catakana.env` si le fichier est trouvé.
/// Tous les champs sont optionnels : l'absence de fichier ou de clé
/// laisse l'app fonctionner sans Supabase.
#[derive(Clone, Debug, Default)]
pub struct SupabaseConfig {
    /// ID du projet Supabase (ex. `fcwvcjtnfxyzojolpisd`).
    pub project_id: Option<String>,
    /// Clé anon (client-side, exposition limitée).
    pub anon_key: Option<String>,
    /// Clé service_role (secret — serveur uniquement, jamais côté client).
    pub service_role_key: Option<String>,
    /// URL de base de la DB mère (validation des ID de profils). Optionnel.
    pub mother_db_url: Option<String>,
}

impl SupabaseConfig {
    /// Indique si une configuration Supabase utilisable est présente.
    #[must_use] 
    pub fn is_available(&self) -> bool {
        !self.project_id.as_deref().unwrap_or("").is_empty()
            && !self.anon_key.as_deref().unwrap_or("").is_empty()
    }
}

/// Charge la configuration Supabase depuis `supabase-catakana.env`.
///
/// Cherche le fichier dans :
/// - le répertoire courant ;
/// - le parent (ex. `crates/miyukini-central` → racine workspace) ;
/// - le parent du parent.
///
/// Ne modifie pas les variables d'environnement déjà définies.
/// Retourne une config vide si le fichier est absent ou invalide.
#[must_use] 
pub fn load_supabase_config() -> SupabaseConfig {
    let path = find_env_file();
    let path = match path {
        Some(p) => p,
        None => return SupabaseConfig::default(),
    };

    match dotenvy::from_path(&path) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("[Central config] Fichier {} non chargé: {}", path.display(), e);
            return SupabaseConfig::default();
        }
    }

    SupabaseConfig {
        project_id: std::env::var("SUPABASE_PROJECT_ID").ok().filter(|s| !s.is_empty()),
        anon_key: std::env::var("SUPABASE_ANON_KEY").ok().filter(|s| !s.is_empty()),
        service_role_key: std::env::var("SUPABASE_SERVICE_ROLE_KEY").ok().filter(|s| !s.is_empty()),
        mother_db_url: std::env::var("MOTHER_DB_URL").ok().filter(|s| !s.is_empty()),
    }
}

/// Cherche `supabase-catakana.env` en remontant depuis le répertoire courant.
fn find_env_file() -> Option<std::path::PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut dir = cwd.as_path();

    for _ in 0..4 {
        let candidate = dir.join(ENV_FILENAME);
        if candidate.exists() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supabase_config_default_is_not_available() {
        let c = SupabaseConfig::default();
        assert!(!c.is_available());
    }

    #[test]
    fn supabase_config_with_ids_is_available() {
        let c = SupabaseConfig {
            project_id: Some("proj".into()),
            anon_key: Some("anon".into()),
            service_role_key: None,
            mother_db_url: None,
        };
        assert!(c.is_available());
    }
}
