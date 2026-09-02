//! @id     miyucloud_application_dtos_folder_listing_dto
//! @do     definir_folder_listing_dto
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « application dtos folder listing dto ».

use serde::Serialize;

use super::file_dto::FileDto;
use super::folder_dto::FolderDto;

/// Combined DTO that returns both sub-folders and files for a given folder
/// in a single response, eliminating the double-fetch on every navigation.
#[derive(Debug, Serialize)]
pub struct FolderListingDto {
    /// Sub-folders inside the requested folder
    pub folders: Vec<FolderDto>,
    /// Files inside the requested folder
    pub files: Vec<FileDto>,
}
