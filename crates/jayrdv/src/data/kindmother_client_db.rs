//! Store JayRDV via KindMother Client (full KM).
//!
//! @id: jayrdv_kindmother_client_db
//! @do: provide_kindmother_backend_for_jayrdv
//! @role: data
//! @layer: infra
//! @human: Stub actuel : délègue au store mémoire. À remplacer par appels kindmother-client.

#![allow(missing_docs)]

use crate::data::memory_store::JayRdvStore as MemoryStore;
use crate::data::types::{
    Appointment, AppointmentStatus, Client, Exception, Practitioner, Professional, Reminder,
    Resource, Schedule, Service, Slot, SlotStatus,
};

/// Store JayRDV en mode KindMother (stub : délègue au store mémoire).
#[derive(Debug, Default)]
pub struct JayRdvStore {
    inner: MemoryStore,
}

impl JayRdvStore {
    /// Crée un store (stub : mémoire).
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: MemoryStore::new(),
        }
    }

    pub fn appointment_create(&self, a: Appointment) -> Result<(), crate::data::DbError> {
        self.inner.appointment_create(a)
    }
    pub fn appointment_update(&self, a: Appointment) -> Result<(), crate::data::DbError> {
        self.inner.appointment_update(a)
    }
    pub fn appointment_by_id(&self, id: &str) -> Result<Option<Appointment>, crate::data::DbError> {
        self.inner.appointment_by_id(id)
    }
    pub fn appointment_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<AppointmentStatus>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Appointment>, crate::data::DbError> {
        self.inner
            .appointment_list(resource_id, status_filter, from_date, to_date)
    }
    pub fn appointment_delete(&self, id: &str) -> Result<bool, crate::data::DbError> {
        self.inner.appointment_delete(id)
    }
    pub fn slot_create(&self, s: Slot) -> Result<(), crate::data::DbError> {
        self.inner.slot_create(s)
    }
    pub fn slot_update(&self, s: Slot) -> Result<(), crate::data::DbError> {
        self.inner.slot_update(s)
    }
    pub fn slot_by_id(&self, id: &str) -> Result<Option<Slot>, crate::data::DbError> {
        self.inner.slot_by_id(id)
    }
    pub fn slot_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<SlotStatus>,
    ) -> Result<Vec<Slot>, crate::data::DbError> {
        self.inner.slot_list(resource_id, status_filter)
    }
    pub fn slot_list_expired_holds(&self) -> Result<Vec<String>, crate::data::DbError> {
        self.inner.slot_list_expired_holds()
    }
    pub fn resource_create(&self, r: Resource) -> Result<(), crate::data::DbError> {
        self.inner.resource_create(r)
    }
    pub fn resource_list(&self) -> Result<Vec<Resource>, crate::data::DbError> {
        self.inner.resource_list()
    }
    pub fn resource_by_id(&self, id: &str) -> Result<Option<Resource>, crate::data::DbError> {
        self.inner.resource_by_id(id)
    }
    pub fn reminder_create(&self, r: Reminder) -> Result<(), crate::data::DbError> {
        self.inner.reminder_create(r)
    }
    pub fn reminder_list_for_appointment(
        &self,
        appointment_id: &str,
    ) -> Result<Vec<Reminder>, crate::data::DbError> {
        self.inner.reminder_list_for_appointment(appointment_id)
    }
    pub fn reminder_mark_sent(&self, id: &str) -> Result<(), crate::data::DbError> {
        self.inner.reminder_mark_sent(id)
    }
    pub fn service_create(&self, s: Service) -> Result<(), crate::data::DbError> {
        self.inner.service_create(s)
    }
    pub fn service_update(&self, s: Service) -> Result<(), crate::data::DbError> {
        self.inner.service_update(s)
    }
    pub fn service_by_id(&self, id: &str) -> Result<Option<Service>, crate::data::DbError> {
        self.inner.service_by_id(id)
    }
    pub fn service_list(&self, active_only: bool) -> Result<Vec<Service>, crate::data::DbError> {
        self.inner.service_list(active_only)
    }
    pub fn client_create(&self, c: Client) -> Result<(), crate::data::DbError> {
        self.inner.client_create(c)
    }
    pub fn client_update(&self, c: Client) -> Result<(), crate::data::DbError> {
        self.inner.client_update(c)
    }
    pub fn client_by_id(&self, id: &str) -> Result<Option<Client>, crate::data::DbError> {
        self.inner.client_by_id(id)
    }
    pub fn client_by_phone(&self, phone: &str) -> Result<Option<Client>, crate::data::DbError> {
        self.inner.client_by_phone(phone)
    }
    pub fn client_list(&self) -> Result<Vec<Client>, crate::data::DbError> {
        self.inner.client_list()
    }
    pub fn client_increment_appointments(&self, id: &str) -> Result<(), crate::data::DbError> {
        self.inner.client_increment_appointments(id)
    }
    pub fn client_increment_no_show(&self, id: &str) -> Result<(), crate::data::DbError> {
        self.inner.client_increment_no_show(id)
    }
    pub fn professional_create(&self, p: Professional) -> Result<(), crate::data::DbError> {
        self.inner.professional_create(p)
    }
    pub fn professional_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Professional>, crate::data::DbError> {
        self.inner.professional_by_id(id)
    }
    pub fn professional_list(&self) -> Result<Vec<Professional>, crate::data::DbError> {
        self.inner.professional_list()
    }
    pub fn practitioner_create(&self, p: Practitioner) -> Result<(), crate::data::DbError> {
        self.inner.practitioner_create(p)
    }
    pub fn practitioner_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Practitioner>, crate::data::DbError> {
        self.inner.practitioner_by_id(id)
    }
    pub fn practitioner_list(
        &self,
        professional_id: Option<&str>,
    ) -> Result<Vec<Practitioner>, crate::data::DbError> {
        self.inner.practitioner_list(professional_id)
    }
    pub fn schedule_create(&self, s: Schedule) -> Result<(), crate::data::DbError> {
        self.inner.schedule_create(s)
    }
    pub fn schedule_by_id(&self, id: &str) -> Result<Option<Schedule>, crate::data::DbError> {
        self.inner.schedule_by_id(id)
    }
    pub fn schedule_list(
        &self,
        owner_id: Option<&str>,
    ) -> Result<Vec<Schedule>, crate::data::DbError> {
        self.inner.schedule_list(owner_id)
    }
    pub fn exception_create(&self, e: Exception) -> Result<(), crate::data::DbError> {
        self.inner.exception_create(e)
    }
    pub fn exception_by_id(&self, id: &str) -> Result<Option<Exception>, crate::data::DbError> {
        self.inner.exception_by_id(id)
    }
    pub fn exception_list(
        &self,
        owner_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Exception>, crate::data::DbError> {
        self.inner.exception_list(owner_id, from_date, to_date)
    }
}
