//! @id     miyucloud_interfaces_nextcloud_owned_paths
//! @do     resoudre_chemins_nextcloud_par_proprietaire
//! @role   securite
//! @layer  app
//! @human  miyucloud — résolution des chemins Nextcloud sous la racine de l'utilisateur authentifié uniquement (M-02).

//! Résolution des chemins Nextcloud filtrée par propriétaire (M-02).
//!
//! - Le dossier personnel est trouvé par **identifiant d'utilisateur** (dossiers
//!   racine dont `user_id` = appelant), jamais par le nom affiché
//!   `My Folder - {username}` seul : un dossier homonyme d'un autre compte
//!   n'est jamais retenu.
//! - Tout chemin est résolu sous ce dossier par
//!   `PathResolverService::resolve_path_for_user` (filtre SQL `user_id`).
//! - Fail-closed (C-12) : sans résolveur, ou si le propriétaire ne correspond
//!   pas, la ressource est introuvable.

use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::ports::inbound::FolderUseCase;
use crate::common::di::AppState;
use crate::common::errors::ErrorKind;
use crate::domain::services::home_folder::home_folder_name;
use crate::infrastructure::services::path_resolver_service::{
    PathResolverService, ResolvedResource,
};
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::CurrentUser;

/// Ressource Nextcloud résolue, appartenant à l'appelant.
#[derive(Debug, Clone)]
pub enum OwnedResource {
    Folder(FolderDto),
    File(FileDto),
}

/// Vrai si `owner_id` désigne exactement `user_id`.
pub fn is_owned_by(owner_id: Option<&str>, user_id: Uuid) -> bool {
    owner_id == Some(user_id.to_string().as_str())
}

/// Choisit le dossier personnel parmi les dossiers racine **de l'appelant**.
///
/// Seuls les dossiers racine (`parent_id = None`) dont le propriétaire est
/// `user_id` sont candidats ; on préfère celui nommé `My Folder - {username}`,
/// sinon le plus ancien.
pub fn select_home_folder<'a>(
    roots: &'a [FolderDto],
    user_id: Uuid,
    username: &str,
) -> Option<&'a FolderDto> {
    let expected = home_folder_name(username);
    let all: Vec<&FolderDto> = roots
        .iter()
        .filter(|f| f.parent_id.is_none() && is_owned_by(f.owner_id.as_deref(), user_id))
        .collect();
    all.iter()
        .find(|f| f.name == expected)
        .copied()
        .or_else(|| all.iter().min_by_key(|f| f.created_at).copied())
}

/// Chemin interne d'une ressource sous le dossier personnel `home`.
pub fn internal_path_under(home: &FolderDto, subpath: &str) -> Result<String, AppError> {
    let subpath = subpath.trim_matches('/');
    if subpath.split('/').any(|seg| seg == ".." || seg == ".") {
        return Err(AppError::bad_request("Invalid path: traversal not allowed"));
    }
    let home_path = home.path.trim_matches('/');
    if subpath.is_empty() {
        Ok(home_path.to_string())
    } else {
        Ok(format!("{home_path}/{subpath}"))
    }
}

fn resolver(state: &AppState) -> Result<&Arc<PathResolverService>, AppError> {
    state.path_resolver.as_ref().ok_or_else(|| {
        tracing::error!("Nextcloud DAV: path resolver unavailable — refusing request");
        AppError::internal_error("Path resolution unavailable")
    })
}

/// Dossier personnel de l'utilisateur authentifié (par identifiant).
pub async fn resolve_home(state: &AppState, user: &CurrentUser) -> Result<FolderDto, AppError> {
    let roots = state
        .applications
        .folder_service
        .list_folders_for_owner(None, user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to resolve home folder: {}", e)))?;
    select_home_folder(&roots, user.id, &user.username)
        .cloned()
        .ok_or_else(|| AppError::not_found("User root folder not found"))
}

/// Résout `subpath` (relatif à la racine Nextcloud de l'utilisateur) vers une
/// ressource **appartenant à l'appelant**. `Ok(None)` si introuvable.
pub async fn resolve_owned(
    state: &AppState,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Option<OwnedResource>, AppError> {
    let home = resolve_home(state, user).await?;
    let internal = internal_path_under(&home, subpath)?;
    if internal == home.path.trim_matches('/') {
        return Ok(Some(OwnedResource::Folder(home)));
    }
    match resolver(state)?
        .resolve_path_for_user(&internal, user.id)
        .await
    {
        Ok(ResolvedResource::Folder(f)) if is_owned_by(f.owner_id.as_deref(), user.id) => {
            Ok(Some(OwnedResource::Folder(f)))
        }
        Ok(ResolvedResource::File(f)) if is_owned_by(f.owner_id.as_deref(), user.id) => {
            Ok(Some(OwnedResource::File(f)))
        }
        Ok(_) => Ok(None),
        Err(e) if e.kind == ErrorKind::NotFound => Ok(None),
        Err(e) => Err(AppError::internal_error(format!(
            "Failed to resolve path: {}",
            e
        ))),
    }
}

/// Résout `subpath` vers un dossier de l'appelant.
pub async fn resolve_owned_folder(
    state: &AppState,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Option<FolderDto>, AppError> {
    Ok(match resolve_owned(state, user, subpath).await? {
        Some(OwnedResource::Folder(f)) => Some(f),
        _ => None,
    })
}

/// Résout `subpath` vers un fichier de l'appelant.
pub async fn resolve_owned_file(
    state: &AppState,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Option<FileDto>, AppError> {
    Ok(match resolve_owned(state, user, subpath).await? {
        Some(OwnedResource::File(f)) => Some(f),
        _ => None,
    })
}

/// Sépare `subpath` en (parent, nom).
pub fn split_parent(subpath: &str) -> (&str, &str) {
    let subpath = subpath.trim_matches('/');
    match subpath.rsplit_once('/') {
        Some((parent, name)) => (parent, name),
        None => ("", subpath),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root(id: &str, name: &str, owner: Uuid, created_at: u64) -> FolderDto {
        FolderDto {
            id: id.to_string(),
            name: name.to_string(),
            path: name.to_string(),
            parent_id: None,
            owner_id: Some(owner.to_string()),
            created_at,
            is_root: true,
            ..FolderDto::empty()
        }
    }

    #[test]
    fn home_squatted_by_another_user_is_never_selected() {
        let bob = Uuid::new_v4();
        let mallory = Uuid::new_v4();
        // Mallory a préparé « My Folder - bob » avant l'inscription de bob.
        let squat = root("m", "My Folder - bob", mallory, 1);
        let real = root("b", "My Folder - bob", bob, 2);
        let roots = vec![squat.clone(), real.clone()];
        let home = select_home_folder(&roots, bob, "bob").expect("home");
        assert_eq!(home.id, "b");
        // Si la liste ne contient que le dossier de l'attaquant : rien.
        assert!(select_home_folder(&[squat], bob, "bob").is_none());
    }

    #[test]
    fn home_resolution_does_not_depend_on_display_name() {
        let alice = Uuid::new_v4();
        // Dossier auto-réparé nommé d'après l'identifiant, pas le nom.
        let healed = root("h", "My Folder - 1a2b3c4d", alice, 5);
        let roots = vec![healed];
        let home = select_home_folder(&roots, alice, "alice").expect("home");
        assert_eq!(home.id, "h");
    }

    #[test]
    fn non_root_folders_are_not_home_candidates() {
        let alice = Uuid::new_v4();
        let mut nested = root("n", "My Folder - alice", alice, 1);
        nested.parent_id = Some("x".to_string());
        assert!(select_home_folder(&[nested], alice, "alice").is_none());
    }

    #[test]
    fn ownership_check_is_strict() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        assert!(is_owned_by(Some(&alice.to_string()), alice));
        assert!(!is_owned_by(Some(&bob.to_string()), alice));
        assert!(!is_owned_by(None, alice));
    }

    #[test]
    fn internal_paths_stay_under_home() {
        let alice = Uuid::new_v4();
        let home = root("h", "My Folder - alice", alice, 1);
        assert_eq!(
            internal_path_under(&home, "/Docs/a.txt").expect("ok"),
            "My Folder - alice/Docs/a.txt"
        );
        assert_eq!(
            internal_path_under(&home, "").expect("ok"),
            "My Folder - alice"
        );
        assert!(internal_path_under(&home, "../My Folder - bob/x").is_err());
        assert!(internal_path_under(&home, "Docs/./x").is_err());
    }

    #[test]
    fn split_parent_works() {
        assert_eq!(split_parent("a/b/c.txt"), ("a/b", "c.txt"));
        assert_eq!(split_parent("c.txt"), ("", "c.txt"));
        assert_eq!(split_parent("/a/b/"), ("a", "b"));
    }
}
