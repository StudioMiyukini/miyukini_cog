//! @id     miyucloud_application_services_folder_service
//! @do     definir_folder_service
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « application services folder service ».

use crate::application::dtos::folder_dto::{
    CreateFolderDto, FolderDto, MoveFolderDto, RenameFolderDto,
};
use crate::application::ports::inbound::FolderUseCase;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::repositories::folder_repository::FolderRepository;
use crate::domain::services::home_folder::is_reserved_home_name;
use crate::domain::services::path_service::StoragePath;
use crate::infrastructure::repositories::pg::folder_db_repository::FolderDbRepository;
use std::sync::Arc;
use uuid::Uuid;

/// Implementation of the use case for folder operations
pub struct FolderService {
    folder_storage: Arc<FolderDbRepository>,
}

impl FolderService {
    /// Creates a new folder service
    pub fn new(folder_storage: Arc<FolderDbRepository>) -> Self {
        Self { folder_storage }
    }

    /// Creates a stub implementation for testing and middleware
    pub fn new_stub() -> impl FolderUseCase {
        struct FolderServiceStub;

        impl FolderUseCase for FolderServiceStub {
            async fn create_folder(&self, _dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn get_folder(&self, _id: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn get_folder_owned(
                &self,
                _id: &str,
                _caller_id: Uuid,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn get_folder_by_path(&self, _path: &str) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn list_folders(
                &self,
                _parent_id: Option<&str>,
            ) -> Result<Vec<FolderDto>, DomainError> {
                Ok(vec![])
            }

            async fn list_folders_for_owner(
                &self,
                _parent_id: Option<&str>,
                _owner_id: Uuid,
            ) -> Result<Vec<FolderDto>, DomainError> {
                Ok(vec![])
            }

            async fn list_folders_paginated(
                &self,
                _parent_id: Option<&str>,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto,
            ) -> Result<
                crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>,
                DomainError,
            > {
                Ok(
                    crate::application::dtos::pagination::PaginatedResponseDto::new(
                        vec![],
                        0,
                        10,
                        0,
                    ),
                )
            }

            async fn list_folders_for_owner_paginated(
                &self,
                _parent_id: Option<&str>,
                _owner_id: Uuid,
                _pagination: &crate::application::dtos::pagination::PaginationRequestDto,
            ) -> Result<
                crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>,
                DomainError,
            > {
                Ok(
                    crate::application::dtos::pagination::PaginatedResponseDto::new(
                        vec![],
                        0,
                        10,
                        0,
                    ),
                )
            }

            async fn rename_folder(
                &self,
                _id: &str,
                _dto: RenameFolderDto,
                _caller_id: Uuid,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn move_folder(
                &self,
                _id: &str,
                _dto: MoveFolderDto,
                _caller_id: Uuid,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }

            async fn delete_folder(&self, _id: &str, _caller_id: Uuid) -> Result<(), DomainError> {
                Ok(())
            }

            async fn create_home_folder(
                &self,
                _user_id: Uuid,
                _name: String,
            ) -> Result<FolderDto, DomainError> {
                Ok(FolderDto::empty())
            }
        }

        FolderServiceStub
    }
}

impl FolderUseCase for FolderService {
    /// Creates a new folder
    async fn create_folder(&self, dto: CreateFolderDto) -> Result<FolderDto, DomainError> {
        // Input validation
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "Folder name cannot be empty",
            ));
        }

        // M-02 : le préfixe des dossiers personnels est réservé.
        if is_reserved_home_name(&dto.name) {
            return Err(reserved_name_error());
        }

        // If a parent_id is provided, verify it exists
        if let Some(parent_id) = &dto.parent_id {
            let parent_exists = self.folder_storage.get_folder(parent_id).await.is_ok();
            if !parent_exists {
                return Err(DomainError::not_found("Folder", parent_id));
            }
        }

        // Create the folder
        let folder = self
            .folder_storage
            .create_folder(dto.name, dto.parent_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to create folder: {}", e),
                )
            })?;

        // Convert to DTO
        Ok(FolderDto::from(folder))
    }

    /// Creates a root-level home folder for a user during registration.
    async fn create_home_folder(
        &self,
        user_id: Uuid,
        name: String,
    ) -> Result<FolderDto, DomainError> {
        let folder = self
            .folder_storage
            .create_home_folder(user_id, name)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to create home folder: {}", e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    async fn list_subtree_folders(&self, folder_id: &str) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self.folder_storage.list_subtree_folders(folder_id).await?;
        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Gets a folder by its ID
    async fn get_folder(&self, id: &str) -> Result<FolderDto, DomainError> {
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {}: {}", id, e),
            )
        })?;

        Ok(FolderDto::from(folder))
    }

    /// Gets a folder by its ID, enforcing that `caller_id` is the owner.
    async fn get_folder_owned(&self, id: &str, caller_id: Uuid) -> Result<FolderDto, DomainError> {
        let folder_dto = self.get_folder(id).await?;
        if folder_dto.owner_id.as_deref() != Some(&caller_id.to_string()) {
            tracing::warn!(
                "get_folder_owned: user '{}' attempted to access folder '{}' owned by '{:?}'",
                caller_id,
                id,
                folder_dto.owner_id
            );
            return Err(DomainError::not_found("Folder", id));
        }
        Ok(folder_dto)
    }

    /// Gets a folder by its path
    async fn get_folder_by_path(&self, path: &str) -> Result<FolderDto, DomainError> {
        // Convert the string path to StoragePath
        let storage_path = StoragePath::from_string(path);

        let folder = self
            .folder_storage
            .get_folder_by_path(&storage_path)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to get folder at path: {}: {}", path, e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    /// Lists folders within a parent folder
    async fn list_folders(&self, parent_id: Option<&str>) -> Result<Vec<FolderDto>, DomainError> {
        let folders = self
            .folder_storage
            .list_folders(parent_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to list folders in parent: {:?}: {}", parent_id, e),
                )
            })?;

        // Convert to DTOs
        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Lists folders scoped to a specific owner.
    /// Self-healing: if listing root folders and none exist, creates a home folder.
    async fn list_folders_for_owner(
        &self,
        parent_id: Option<&str>,
        owner_id: Uuid,
    ) -> Result<Vec<FolderDto>, DomainError> {
        let owner_id_short = {
            let s = owner_id.to_string();
            s[..8.min(s.len())].to_string()
        };
        let folders = self
            .folder_storage
            .list_folders_by_owner(parent_id, owner_id)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders for owner '{}' in parent {:?}: {}",
                        owner_id, parent_id, e
                    ),
                )
            })?;

        // Self-healing: if listing root folders and none exist, create a home folder
        // This ensures the frontend always gets a valid userHomeFolderId
        if parent_id.is_none() && folders.is_empty() {
            tracing::info!(
                "No root folders found for user {}, creating home folder automatically",
                owner_id
            );
            let folder_name = format!("My Folder - {}", owner_id_short);
            match self
                .folder_storage
                .create_home_folder(owner_id, folder_name.clone())
                .await
            {
                Ok(home_folder) => {
                    tracing::info!(
                        "Created home folder '{}' for user {}",
                        folder_name,
                        owner_id
                    );
                    return Ok(vec![FolderDto::from(home_folder)]);
                }
                Err(e) => {
                    tracing::warn!("Failed to create home folder for user {}: {}", owner_id, e);
                    // Return empty list rather than failing - user might not have storage quota, etc.
                }
            }
        }

        Ok(folders.into_iter().map(FolderDto::from).collect())
    }

    /// Lists folders with pagination
    async fn list_folders_paginated(
        &self,
        parent_id: Option<&str>,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>
    {
        let pagination = pagination.validate_and_adjust();

        let (folders, total_items) = self
            .folder_storage
            .list_folders_paginated(parent_id, pagination.offset(), pagination.limit(), true)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders with pagination in parent: {:?}: {}",
                        parent_id, e
                    ),
                )
            })?;

        let total = total_items.unwrap_or(folders.len());

        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total,
        );

        Ok(response)
    }

    /// Lists folders with pagination, scoped to a specific owner.
    async fn list_folders_for_owner_paginated(
        &self,
        parent_id: Option<&str>,
        owner_id: Uuid,
        pagination: &crate::application::dtos::pagination::PaginationRequestDto,
    ) -> Result<crate::application::dtos::pagination::PaginatedResponseDto<FolderDto>, DomainError>
    {
        let pagination = pagination.validate_and_adjust();

        let (folders, total_items) = self
            .folder_storage
            .list_folders_by_owner_paginated(
                parent_id,
                owner_id,
                pagination.offset(),
                pagination.limit(),
                true,
            )
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!(
                        "Failed to list folders for owner '{}' with pagination in parent {:?}: {}",
                        owner_id, parent_id, e
                    ),
                )
            })?;

        let total = total_items.unwrap_or(folders.len());

        let response = crate::application::dtos::pagination::PaginatedResponseDto::new(
            folders.into_iter().map(FolderDto::from).collect(),
            pagination.page,
            pagination.page_size,
            total,
        );

        Ok(response)
    }

    /// Renames a folder after verifying ownership.
    async fn rename_folder(
        &self,
        id: &str,
        dto: RenameFolderDto,
        caller_id: Uuid,
    ) -> Result<FolderDto, DomainError> {
        // Input validation
        if dto.name.is_empty() {
            return Err(DomainError::new(
                ErrorKind::InvalidInput,
                "Folder",
                "New folder name cannot be empty",
            ));
        }

        // Verify the folder exists and belongs to the caller
        let existing_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for renaming: {}", id, e),
            )
        })?;

        if existing_folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "rename_folder: user '{}' attempted to rename folder '{}' owned by '{:?}'",
                caller_id,
                id,
                existing_folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // M-02 : le dossier personnel (racine) ne se renomme pas et aucun
        // dossier ne peut prendre le nom réservé d'un dossier personnel.
        validate_folder_rename(&FolderDto::from(existing_folder), &dto.name)?;

        // Rename folder — UPDATE RETURNING gives us the updated row directly
        let folder = self
            .folder_storage
            .rename_folder(id, dto.name)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to rename folder with ID: {}: {}", id, e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    /// Moves a folder to a new parent after verifying ownership.
    async fn move_folder(
        &self,
        id: &str,
        dto: MoveFolderDto,
        caller_id: Uuid,
    ) -> Result<FolderDto, DomainError> {
        // Verify the source folder exists and belongs to the caller
        let source_folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for moving: {}", id, e),
            )
        })?;

        if source_folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "move_folder: user '{}' attempted to move folder '{}' owned by '{:?}'",
                caller_id,
                id,
                source_folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // M-03 : la destination doit appartenir à l'appelant, ne pas être la
        // source ni l'un de ses descendants ; pas de déplacement à la racine.
        let target = match &dto.parent_id {
            Some(parent_id) => Some(
                self.folder_storage
                    .get_folder(parent_id)
                    .await
                    .map(FolderDto::from)
                    .map_err(|_| DomainError::not_found("Folder", parent_id))?,
            ),
            None => None,
        };
        validate_folder_move(&FolderDto::from(source_folder), target.as_ref(), caller_id)?;

        // Move folder — UPDATE RETURNING gives us the updated row directly
        let parent_ref = dto.parent_id.as_deref();
        let folder = self
            .folder_storage
            .move_folder(id, parent_ref)
            .await
            .map_err(|e| {
                DomainError::internal_error(
                    "FolderStorage",
                    format!("Failed to move folder with ID: {}: {}", id, e),
                )
            })?;

        Ok(FolderDto::from(folder))
    }

    /// Deletes a folder after verifying ownership.
    async fn delete_folder(&self, id: &str, caller_id: Uuid) -> Result<(), DomainError> {
        // Verify the folder exists and belongs to the caller
        let folder = self.folder_storage.get_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to get folder with ID: {} for deletion: {}", id, e),
            )
        })?;

        if folder.owner_id() != Some(caller_id) {
            tracing::warn!(
                "delete_folder: user '{}' attempted to delete folder '{}' owned by '{:?}'",
                caller_id,
                id,
                folder.owner_id()
            );
            return Err(DomainError::not_found("Folder", id));
        }

        // Delete the folder
        self.folder_storage.delete_folder(id).await.map_err(|e| {
            DomainError::internal_error(
                "FolderStorage",
                format!("Failed to delete folder with ID: {}: {}", id, e),
            )
        })
    }
}

fn reserved_name_error() -> DomainError {
    DomainError::new(
        ErrorKind::InvalidInput,
        "Folder",
        "This folder name is reserved",
    )
}

/// M-02 : refuse le renommage d'un dossier racine (dossier personnel) et
/// tout nom imitant un dossier personnel.
pub(crate) fn validate_folder_rename(
    folder: &FolderDto,
    new_name: &str,
) -> Result<(), DomainError> {
    if folder.parent_id.is_none() {
        return Err(DomainError::new(
            ErrorKind::AccessDenied,
            "Folder",
            "The home folder cannot be renamed",
        ));
    }
    if is_reserved_home_name(new_name) {
        return Err(reserved_name_error());
    }
    Ok(())
}

/// M-03 : règles de déplacement d'un dossier (`source` déjà vérifié comme
/// appartenant à `caller_id`).
///
/// - pas de déplacement du dossier personnel (racine) ;
/// - pas de déplacement vers la racine (`target = None`) ;
/// - la destination appartient à `caller_id` (sinon `NotFound`, sans fuite) ;
/// - la destination n'est ni la source ni l'un de ses descendants (cycle).
pub(crate) fn validate_folder_move(
    source: &FolderDto,
    target: Option<&FolderDto>,
    caller_id: Uuid,
) -> Result<(), DomainError> {
    if source.parent_id.is_none() {
        return Err(DomainError::new(
            ErrorKind::AccessDenied,
            "Folder",
            "The home folder cannot be moved",
        ));
    }
    let Some(target) = target else {
        return Err(DomainError::new(
            ErrorKind::AccessDenied,
            "Folder",
            "Folders cannot be moved to the storage root",
        ));
    };
    let caller = caller_id.to_string();
    if target.owner_id.as_deref() != Some(caller.as_str()) {
        tracing::warn!(
            "move_folder: user '{}' attempted to move folder '{}' into folder '{}' owned by '{:?}'",
            caller_id,
            source.id,
            target.id,
            target.owner_id
        );
        return Err(DomainError::not_found("Folder", &target.id));
    }
    let descendant_prefix = format!("{}/", source.path.trim_end_matches('/'));
    if target.id == source.id
        || target.path == source.path
        || target.path.starts_with(&descendant_prefix)
    {
        return Err(DomainError::new(
            ErrorKind::InvalidInput,
            "Folder",
            "Cannot move a folder into itself or one of its descendants",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod security_tests {
    use super::*;

    fn folder(id: &str, path: &str, parent: Option<&str>, owner: Uuid) -> FolderDto {
        FolderDto {
            id: id.to_string(),
            name: path.rsplit('/').next().unwrap_or(path).to_string(),
            path: path.to_string(),
            parent_id: parent.map(str::to_string),
            owner_id: Some(owner.to_string()),
            is_root: parent.is_none(),
            ..FolderDto::empty()
        }
    }

    #[test]
    fn move_into_another_users_tree_is_refused() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let src = folder("s", "My Folder - alice/Docs", Some("ha"), alice);
        let bob_dir = folder("b", "My Folder - bob/Photos", Some("hb"), bob);
        let err = validate_folder_move(&src, Some(&bob_dir), alice).expect_err("refused");
        assert_eq!(err.kind, ErrorKind::NotFound);
    }

    #[test]
    fn move_to_root_is_refused() {
        let alice = Uuid::new_v4();
        let src = folder("s", "My Folder - alice/Docs", Some("ha"), alice);
        assert!(validate_folder_move(&src, None, alice).is_err());
    }

    #[test]
    fn move_home_folder_is_refused() {
        let alice = Uuid::new_v4();
        let home = folder("ha", "My Folder - alice", None, alice);
        let dst = folder("d", "My Folder - alice/Docs", Some("ha"), alice);
        assert!(validate_folder_move(&home, Some(&dst), alice).is_err());
    }

    #[test]
    fn move_into_descendant_is_refused() {
        let alice = Uuid::new_v4();
        let src = folder("s", "My Folder - alice/Docs", Some("ha"), alice);
        let child = folder("c", "My Folder - alice/Docs/2027", Some("s"), alice);
        let err = validate_folder_move(&src, Some(&child), alice).expect_err("cycle");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
        assert!(validate_folder_move(&src, Some(&src), alice).is_err());
    }

    #[test]
    fn move_into_sibling_with_common_prefix_is_allowed() {
        let alice = Uuid::new_v4();
        let src = folder("s", "My Folder - alice/Docs", Some("ha"), alice);
        let sib = folder("x", "My Folder - alice/Docs2", Some("ha"), alice);
        assert!(validate_folder_move(&src, Some(&sib), alice).is_ok());
    }

    #[test]
    fn home_squatting_by_rename_is_refused() {
        let alice = Uuid::new_v4();
        let home = folder("ha", "My Folder - alice", None, alice);
        // Renommer son dossier personnel en « My Folder - bob » : refusé.
        assert!(validate_folder_rename(&home, "My Folder - bob").is_err());
        // Renommer un sous-dossier avec le préfixe réservé : refusé.
        let sub = folder("s", "My Folder - alice/Docs", Some("ha"), alice);
        let err = validate_folder_rename(&sub, "my folder - bob").expect_err("reserved");
        assert_eq!(err.kind, ErrorKind::InvalidInput);
        assert!(validate_folder_rename(&sub, "Archives").is_ok());
    }
}
