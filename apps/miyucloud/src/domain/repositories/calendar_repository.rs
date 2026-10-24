//! @id     miyucloud_domain_repositories_calendar_repository
//! @do     definir_calendar_repository_result
//! @role   donnee
//! @layer  app
//! @human  miyucloud — module « domain repositories calendar repository ».

use crate::common::errors::DomainError;
use crate::domain::entities::calendar::Calendar;
use crate::domain::entities::calendar_access::CalendarAccess;
use uuid::Uuid;

pub type CalendarRepositoryResult<T> = Result<T, DomainError>;

/// Repository interface for Calendar entity operations
pub trait CalendarRepository: Send + Sync + 'static {
    /// Creates a new calendar
    async fn create_calendar(&self, calendar: Calendar) -> CalendarRepositoryResult<Calendar>;

    /// Updates an existing calendar
    async fn update_calendar(&self, calendar: Calendar) -> CalendarRepositoryResult<Calendar>;

    /// Deletes a calendar by ID
    async fn delete_calendar(&self, id: &Uuid) -> CalendarRepositoryResult<()>;

    /// Finds a calendar by its ID
    async fn find_calendar_by_id(&self, id: &Uuid) -> CalendarRepositoryResult<Calendar>;

    /// Lists all calendars for a specific user
    async fn list_calendars_by_owner(
        &self,
        owner_id: Uuid,
    ) -> CalendarRepositoryResult<Vec<Calendar>>;

    /// Finds a calendar by name and owner
    async fn find_calendar_by_name_and_owner(
        &self,
        name: &str,
        owner_id: Uuid,
    ) -> CalendarRepositoryResult<Calendar>;

    /// Lists calendars shared with a specific user
    async fn list_calendars_shared_with_user(
        &self,
        user_id: Uuid,
    ) -> CalendarRepositoryResult<Vec<Calendar>>;

    /// List public calendars
    async fn list_public_calendars(
        &self,
        limit: i64,
        offset: i64,
    ) -> CalendarRepositoryResult<Vec<Calendar>>;

    /// Reads the access facts (owner, public, share level) of a user on a calendar.
    /// M-01 : la décision lecture/écriture est prise par `CalendarAccess`.
    async fn get_calendar_access(
        &self,
        calendar_id: &Uuid,
        user_id: Uuid,
    ) -> CalendarRepositoryResult<CalendarAccess>;

    /// Gets a custom property for a calendar
    async fn get_calendar_property(
        &self,
        calendar_id: &Uuid,
        property_name: &str,
    ) -> CalendarRepositoryResult<Option<String>>;

    /// Sets a custom property for a calendar
    async fn set_calendar_property(
        &self,
        calendar_id: &Uuid,
        property_name: &str,
        property_value: &str,
    ) -> CalendarRepositoryResult<()>;

    /// Removes a custom property from a calendar
    async fn remove_calendar_property(
        &self,
        calendar_id: &Uuid,
        property_name: &str,
    ) -> CalendarRepositoryResult<()>;

    /// Gets all custom properties for a calendar
    async fn get_calendar_properties(
        &self,
        calendar_id: &Uuid,
    ) -> CalendarRepositoryResult<std::collections::HashMap<String, String>>;

    /// Share calendar with another user
    async fn share_calendar(
        &self,
        calendar_id: &Uuid,
        user_id: Uuid,
        access_level: &str,
    ) -> CalendarRepositoryResult<()>;

    /// Remove calendar sharing for a user
    async fn remove_calendar_sharing(
        &self,
        calendar_id: &Uuid,
        user_id: Uuid,
    ) -> CalendarRepositoryResult<()>;

    /// Get calendar sharing information (who has access to this calendar)
    async fn get_calendar_shares(
        &self,
        calendar_id: &Uuid,
    ) -> CalendarRepositoryResult<Vec<(String, String)>>;
}
