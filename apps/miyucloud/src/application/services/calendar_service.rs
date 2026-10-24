//! @id     miyucloud_application_services_calendar_service
//! @do     definir_calendar_service
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « application services calendar service ».

use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::calendar_dto::{
    CalendarDto, CalendarEventDto, CreateCalendarDto, CreateEventDto, CreateEventICalDto,
    UpdateCalendarDto, UpdateEventDto,
};
use crate::application::ports::calendar_ports::{CalendarStoragePort, CalendarUseCase};
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::calendar_access::CalendarAccess;
use crate::infrastructure::adapters::calendar_storage_adapter::CalendarStorageAdapter;

pub struct CalendarService {
    calendar_storage: Arc<CalendarStorageAdapter>,
}

impl CalendarService {
    pub fn new(calendar_storage: Arc<CalendarStorageAdapter>) -> Self {
        Self { calendar_storage }
    }

    /// M-01 : contrôle d'accès par opération (lecture / écriture / gestion).
    async fn authorize(
        &self,
        calendar_id: &str,
        user_id: Uuid,
        op: CalendarOp,
    ) -> Result<(), DomainError> {
        let access = self
            .calendar_storage
            .get_calendar_access(calendar_id, user_id)
            .await?;
        check_calendar_op(&access, op)
    }
}

/// Opérations soumises au contrôle d'accès calendrier (M-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarOp {
    /// Lire le calendrier et ses événements.
    Read,
    /// Modifier les métadonnées du calendrier.
    UpdateCalendar,
    /// Supprimer le calendrier.
    DeleteCalendar,
    /// Créer, modifier ou supprimer des événements.
    WriteEvents,
}

/// Décision pure : refuse (fail-closed) si le droit requis par `op` manque.
pub fn check_calendar_op(access: &CalendarAccess, op: CalendarOp) -> Result<(), DomainError> {
    let (allowed, message) = match op {
        CalendarOp::Read => (
            access.can_read(),
            "You don't have permission to view this calendar",
        ),
        CalendarOp::UpdateCalendar => (
            access.can_write(),
            "You don't have permission to update this calendar",
        ),
        CalendarOp::DeleteCalendar => (
            access.can_manage(),
            "You don't have permission to delete this calendar",
        ),
        CalendarOp::WriteEvents => (
            access.can_write(),
            "You don't have permission to modify events in this calendar",
        ),
    };
    if allowed {
        Ok(())
    } else {
        Err(DomainError::new(
            ErrorKind::AccessDenied,
            "Calendar",
            message,
        ))
    }
}

impl CalendarUseCase for CalendarService {
    async fn create_calendar(
        &self,
        calendar: CreateCalendarDto,
        user_id: Uuid,
    ) -> Result<CalendarDto, DomainError> {
        self.calendar_storage
            .create_calendar(calendar, user_id)
            .await
    }

    async fn update_calendar(
        &self,
        calendar_id: &str,
        update: UpdateCalendarDto,
        user_id: Uuid,
    ) -> Result<CalendarDto, DomainError> {
        self.authorize(calendar_id, user_id, CalendarOp::UpdateCalendar)
            .await?;
        self.calendar_storage
            .update_calendar(calendar_id, update)
            .await
    }

    async fn delete_calendar(&self, calendar_id: &str, user_id: Uuid) -> Result<(), DomainError> {
        self.authorize(calendar_id, user_id, CalendarOp::DeleteCalendar)
            .await?;
        self.calendar_storage.delete_calendar(calendar_id).await
    }

    async fn get_calendar(
        &self,
        calendar_id: &str,
        user_id: Uuid,
    ) -> Result<CalendarDto, DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        self.authorize(calendar_id, user_id, CalendarOp::Read)
            .await?;
        Ok(calendar)
    }

    async fn list_my_calendars(&self, user_id: Uuid) -> Result<Vec<CalendarDto>, DomainError> {
        self.calendar_storage.list_calendars_by_owner(user_id).await
    }

    async fn list_shared_calendars(&self, user_id: Uuid) -> Result<Vec<CalendarDto>, DomainError> {
        self.calendar_storage
            .list_calendars_shared_with_user(user_id)
            .await
    }

    async fn list_public_calendars(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<CalendarDto>, DomainError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        self.calendar_storage
            .list_public_calendars(limit, offset)
            .await
    }

    async fn share_calendar(
        &self,
        calendar_id: &str,
        target_user_id: Uuid,
        access_level: &str,
        caller_user_id: Uuid,
    ) -> Result<(), DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != caller_user_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can change sharing settings",
            ));
        }
        match access_level {
            "read" | "write" | "owner" => {}
            _ => {
                return Err(DomainError::new(
                    ErrorKind::InvalidInput,
                    "Calendar",
                    format!(
                        "Invalid access level: {}. Valid values are: read, write, owner",
                        access_level
                    ),
                ));
            }
        }
        self.calendar_storage
            .share_calendar(calendar_id, target_user_id, access_level)
            .await
    }

    async fn remove_calendar_sharing(
        &self,
        calendar_id: &str,
        target_user_id: Uuid,
        caller_user_id: Uuid,
    ) -> Result<(), DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != caller_user_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can change sharing settings",
            ));
        }
        self.calendar_storage
            .remove_calendar_sharing(calendar_id, target_user_id)
            .await
    }

    async fn get_calendar_shares(
        &self,
        calendar_id: &str,
        user_id: Uuid,
    ) -> Result<Vec<(String, String)>, DomainError> {
        let calendar = self.calendar_storage.get_calendar(calendar_id).await?;
        if calendar.owner_id != user_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Calendar",
                "Only the calendar owner can view sharing settings",
            ));
        }
        self.calendar_storage.get_calendar_shares(calendar_id).await
    }

    async fn create_event(
        &self,
        event: CreateEventDto,
        user_id: Uuid,
    ) -> Result<CalendarEventDto, DomainError> {
        self.authorize(&event.calendar_id, user_id, CalendarOp::WriteEvents)
            .await?;
        self.calendar_storage.create_event(event).await
    }

    async fn create_event_from_ical(
        &self,
        event: CreateEventICalDto,
        user_id: Uuid,
    ) -> Result<CalendarEventDto, DomainError> {
        self.authorize(&event.calendar_id, user_id, CalendarOp::WriteEvents)
            .await?;
        self.calendar_storage.create_event_from_ical(event).await
    }

    async fn update_event(
        &self,
        event_id: &str,
        update: UpdateEventDto,
        user_id: Uuid,
    ) -> Result<CalendarEventDto, DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        self.authorize(&event.calendar_id, user_id, CalendarOp::WriteEvents)
            .await?;
        self.calendar_storage.update_event(event_id, update).await
    }

    async fn delete_event(&self, event_id: &str, user_id: Uuid) -> Result<(), DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        self.authorize(&event.calendar_id, user_id, CalendarOp::WriteEvents)
            .await?;
        self.calendar_storage.delete_event(event_id).await
    }

    async fn get_event(
        &self,
        event_id: &str,
        user_id: Uuid,
    ) -> Result<CalendarEventDto, DomainError> {
        let event = self.calendar_storage.get_event(event_id).await?;
        self.authorize(&event.calendar_id, user_id, CalendarOp::Read)
            .await?;
        Ok(event)
    }

    async fn list_events(
        &self,
        calendar_id: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        user_id: Uuid,
    ) -> Result<Vec<CalendarEventDto>, DomainError> {
        self.authorize(calendar_id, user_id, CalendarOp::Read)
            .await?;
        if limit.is_some() || offset.is_some() {
            let limit = limit.unwrap_or(100);
            let offset = offset.unwrap_or(0);
            self.calendar_storage
                .list_events_by_calendar_paginated(calendar_id, limit, offset)
                .await
        } else {
            self.calendar_storage
                .list_events_by_calendar(calendar_id)
                .await
        }
    }

    async fn get_events_in_range(
        &self,
        calendar_id: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        user_id: Uuid,
    ) -> Result<Vec<CalendarEventDto>, DomainError> {
        self.authorize(calendar_id, user_id, CalendarOp::Read)
            .await?;
        self.calendar_storage
            .get_events_in_time_range(calendar_id, &start, &end)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::calendar_access::CalendarShareLevel;

    fn access(
        is_owner: bool,
        is_public: bool,
        share: Option<CalendarShareLevel>,
    ) -> CalendarAccess {
        CalendarAccess {
            exists: true,
            is_owner,
            is_public,
            share,
        }
    }

    fn denied(r: Result<(), DomainError>) -> bool {
        matches!(r, Err(e) if e.kind == ErrorKind::AccessDenied)
    }

    #[test]
    fn read_only_share_cannot_write_events_update_or_delete() {
        let a = access(false, false, Some(CalendarShareLevel::Read));
        assert!(check_calendar_op(&a, CalendarOp::Read).is_ok());
        assert!(denied(check_calendar_op(&a, CalendarOp::WriteEvents)));
        assert!(denied(check_calendar_op(&a, CalendarOp::UpdateCalendar)));
        assert!(denied(check_calendar_op(&a, CalendarOp::DeleteCalendar)));
    }

    #[test]
    fn public_calendar_cannot_be_modified_by_non_owner() {
        let a = access(false, true, None);
        assert!(check_calendar_op(&a, CalendarOp::Read).is_ok());
        assert!(denied(check_calendar_op(&a, CalendarOp::WriteEvents)));
        assert!(denied(check_calendar_op(&a, CalendarOp::UpdateCalendar)));
        assert!(denied(check_calendar_op(&a, CalendarOp::DeleteCalendar)));
    }

    #[test]
    fn write_share_can_edit_events_but_not_delete_calendar() {
        let a = access(false, false, Some(CalendarShareLevel::Write));
        assert!(check_calendar_op(&a, CalendarOp::WriteEvents).is_ok());
        assert!(check_calendar_op(&a, CalendarOp::UpdateCalendar).is_ok());
        assert!(denied(check_calendar_op(&a, CalendarOp::DeleteCalendar)));
    }

    #[test]
    fn owner_can_do_everything() {
        let a = access(true, false, None);
        for op in [
            CalendarOp::Read,
            CalendarOp::UpdateCalendar,
            CalendarOp::DeleteCalendar,
            CalendarOp::WriteEvents,
        ] {
            assert!(check_calendar_op(&a, op).is_ok());
        }
    }

    #[test]
    fn stranger_and_missing_calendar_are_refused() {
        for a in [access(false, false, None), CalendarAccess::none()] {
            assert!(denied(check_calendar_op(&a, CalendarOp::Read)));
            assert!(denied(check_calendar_op(&a, CalendarOp::WriteEvents)));
        }
    }
}
