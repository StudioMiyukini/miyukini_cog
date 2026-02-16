//! Store en mémoire pour JayRDV (v0).
//!
//! @id: jayrdv_memory_store
//! @do: provide_in_memory_persistence_for_jayrdv
//! @role: data
//! @layer: infra
//! @human: Implémente le stockage en mémoire (HashMap + RwLock) pour toutes les entités.
//!         Persistance KindMother (full KM) à ajouter ultérieurement.

use crate::data::types::{
    Appointment, AppointmentStatus, Client, Exception, Practitioner, Professional, Reminder,
    Resource, Schedule, Service, Slot, SlotStatus,
};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::RwLock;

/// Store en mémoire pour les entités JayRDV.
#[derive(Debug, Default)]
pub struct JayRdvStore {
    appointments: RwLock<HashMap<String, Appointment>>,
    slots: RwLock<HashMap<String, Slot>>,
    resources: RwLock<HashMap<String, Resource>>,
    reminders: RwLock<HashMap<String, Reminder>>,
    services: RwLock<HashMap<String, Service>>,
    clients: RwLock<HashMap<String, Client>>,
    professionals: RwLock<HashMap<String, Professional>>,
    practitioners: RwLock<HashMap<String, Practitioner>>,
    schedules: RwLock<HashMap<String, Schedule>>,
    exceptions: RwLock<HashMap<String, Exception>>,
}

/// Erreur de couche données.
#[derive(Debug, Clone)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayRDV: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl JayRdvStore {
    /// Crée un store vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    // -----------------------------------------------------------------------
    // Appointments
    // -----------------------------------------------------------------------

    /// Insère un rendez-vous. Erreur si l'id existe déjà.
    pub fn appointment_create(&self, a: Appointment) -> Result<(), DbError> {
        let id = a.id.clone();
        let mut map = self.appointments.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("appointment already exists: {id}")));
        }
        map.insert(id, a);
        Ok(())
    }

    /// Met à jour un rendez-vous existant. Erreur si l'id n'existe pas.
    pub fn appointment_update(&self, a: Appointment) -> Result<(), DbError> {
        let id = a.id.clone();
        let mut map = self.appointments.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            map.insert(id, a);
            Ok(())
        } else {
            Err(DbError(format!("appointment not found: {id}")))
        }
    }

    /// Récupère un rendez-vous par id.
    pub fn appointment_by_id(&self, id: &str) -> Result<Option<Appointment>, DbError> {
        let map = self.appointments.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste les rendez-vous avec filtres optionnels.
    /// Les filtres de dates (from_date, to_date) sont parsés en `chrono::DateTime<Utc>` ;
    /// les rendez-vous dont les champs start_at/end_at ne sont pas des dates ISO 8601 valides sont exclus.
    pub fn appointment_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<AppointmentStatus>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Appointment>, DbError> {
        let from_ts = from_date.and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok());
        let to_ts = to_date.and_then(|s| s.parse::<chrono::DateTime<Utc>>().ok());

        let map = self.appointments.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Appointment> = map
            .values()
            .filter(|a| {
                if let Some(rid) = resource_id {
                    if a.resource_id.as_deref() != Some(rid) {
                        return false;
                    }
                }
                if let Some(s) = status_filter {
                    if a.status != s {
                        return false;
                    }
                }
                if let Some(from) = from_ts {
                    if let Ok(a_start) = a.start_at.parse::<chrono::DateTime<Utc>>() {
                        if a_start < from {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                if let Some(to) = to_ts {
                    if let Ok(a_end) = a.end_at.parse::<chrono::DateTime<Utc>>() {
                        if a_end > to {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        out.sort_by(|a, b| a.start_at.cmp(&b.start_at));
        Ok(out)
    }

    /// Supprime un rendez-vous par id.
    pub fn appointment_delete(&self, id: &str) -> Result<bool, DbError> {
        let mut map = self.appointments.write().map_err(|e| DbError(e.to_string()))?;
        Ok(map.remove(id).is_some())
    }

    // -----------------------------------------------------------------------
    // Slots
    // -----------------------------------------------------------------------

    /// Insère un créneau. Erreur si l'id existe déjà.
    pub fn slot_create(&self, s: Slot) -> Result<(), DbError> {
        let id = s.id.clone();
        let mut map = self.slots.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("slot already exists: {id}")));
        }
        map.insert(id, s);
        Ok(())
    }

    /// Met à jour un créneau existant.
    pub fn slot_update(&self, s: Slot) -> Result<(), DbError> {
        let id = s.id.clone();
        let mut map = self.slots.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            map.insert(id, s);
            Ok(())
        } else {
            Err(DbError(format!("slot not found: {id}")))
        }
    }

    /// Récupère un créneau par id.
    pub fn slot_by_id(&self, id: &str) -> Result<Option<Slot>, DbError> {
        let map = self.slots.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste les créneaux avec filtres optionnels.
    pub fn slot_list(
        &self,
        resource_id: Option<&str>,
        status_filter: Option<SlotStatus>,
    ) -> Result<Vec<Slot>, DbError> {
        let map = self.slots.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Slot> = map
            .values()
            .filter(|s| {
                if let Some(rid) = resource_id {
                    if s.resource_id != rid {
                        return false;
                    }
                }
                if let Some(st) = status_filter {
                    if s.status != st {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        out.sort_by(|a, b| a.start_at.cmp(&b.start_at));
        Ok(out)
    }

    /// Liste les ids de créneaux en statut Held dont le verrouillage a expiré (held_until < now).
    pub fn slot_list_expired_holds(&self) -> Result<Vec<String>, DbError> {
        let now = Utc::now();
        let map = self.slots.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<String> = map
            .values()
            .filter(|s| {
                if s.status != SlotStatus::Held {
                    return false;
                }
                let Some(ref until_str) = s.held_until else { return false };
                match until_str.parse::<chrono::DateTime<Utc>>() {
                    Ok(until) => until < now,
                    Err(_) => true, // format invalide : considérer comme expiré
                }
            })
            .map(|s| s.id.clone())
            .collect();
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // Resources
    // -----------------------------------------------------------------------

    /// Insère une ressource.
    pub fn resource_create(&self, r: Resource) -> Result<(), DbError> {
        let id = r.id.clone();
        let mut map = self.resources.write().map_err(|e| DbError(e.to_string()))?;
        map.insert(id, r);
        Ok(())
    }

    /// Liste toutes les ressources.
    pub fn resource_list(&self) -> Result<Vec<Resource>, DbError> {
        let map = self.resources.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Resource> = map.values().cloned().collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    /// Récupère une ressource par id.
    pub fn resource_by_id(&self, id: &str) -> Result<Option<Resource>, DbError> {
        let map = self.resources.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    // -----------------------------------------------------------------------
    // Reminders
    // -----------------------------------------------------------------------

    /// Insère un rappel.
    pub fn reminder_create(&self, r: Reminder) -> Result<(), DbError> {
        let id = r.id.clone();
        let mut map = self.reminders.write().map_err(|e| DbError(e.to_string()))?;
        map.insert(id, r);
        Ok(())
    }

    /// Liste les rappels d'un rendez-vous.
    pub fn reminder_list_for_appointment(&self, appointment_id: &str) -> Result<Vec<Reminder>, DbError> {
        let map = self.reminders.read().map_err(|e| DbError(e.to_string()))?;
        let out: Vec<Reminder> = map
            .values()
            .filter(|r| r.appointment_id == appointment_id)
            .cloned()
            .collect();
        Ok(out)
    }

    /// Marque un rappel comme envoyé.
    pub fn reminder_mark_sent(&self, id: &str) -> Result<(), DbError> {
        let mut map = self.reminders.write().map_err(|e| DbError(e.to_string()))?;
        if let Some(r) = map.get_mut(id) {
            r.sent = true;
            Ok(())
        } else {
            Err(DbError(format!("reminder not found: {id}")))
        }
    }

    // -----------------------------------------------------------------------
    // Services
    // -----------------------------------------------------------------------

    /// Insère un service (prestation).
    pub fn service_create(&self, s: Service) -> Result<(), DbError> {
        let id = s.id.clone();
        let mut map = self.services.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("service already exists: {id}")));
        }
        map.insert(id, s);
        Ok(())
    }

    /// Met à jour un service.
    pub fn service_update(&self, s: Service) -> Result<(), DbError> {
        let id = s.id.clone();
        let mut map = self.services.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            map.insert(id, s);
            Ok(())
        } else {
            Err(DbError(format!("service not found: {id}")))
        }
    }

    /// Récupère un service par id.
    pub fn service_by_id(&self, id: &str) -> Result<Option<Service>, DbError> {
        let map = self.services.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste tous les services (actifs par défaut).
    pub fn service_list(&self, active_only: bool) -> Result<Vec<Service>, DbError> {
        let map = self.services.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Service> = map
            .values()
            .filter(|s| !active_only || s.active)
            .cloned()
            .collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // Clients
    // -----------------------------------------------------------------------

    /// Insère un client.
    pub fn client_create(&self, c: Client) -> Result<(), DbError> {
        let id = c.id.clone();
        let mut map = self.clients.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("client already exists: {id}")));
        }
        map.insert(id, c);
        Ok(())
    }

    /// Met à jour un client.
    pub fn client_update(&self, c: Client) -> Result<(), DbError> {
        let id = c.id.clone();
        let mut map = self.clients.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            map.insert(id, c);
            Ok(())
        } else {
            Err(DbError(format!("client not found: {id}")))
        }
    }

    /// Récupère un client par id.
    pub fn client_by_id(&self, id: &str) -> Result<Option<Client>, DbError> {
        let map = self.clients.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Recherche un client par téléphone.
    pub fn client_by_phone(&self, phone: &str) -> Result<Option<Client>, DbError> {
        let map = self.clients.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.values().find(|c| c.phone == phone).cloned())
    }

    /// Liste tous les clients.
    pub fn client_list(&self) -> Result<Vec<Client>, DbError> {
        let map = self.clients.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Client> = map.values().cloned().collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    /// Incrémente le compteur de rendez-vous d'un client.
    pub fn client_increment_appointments(&self, id: &str) -> Result<(), DbError> {
        let mut map = self.clients.write().map_err(|e| DbError(e.to_string()))?;
        if let Some(c) = map.get_mut(id) {
            c.total_appointments += 1;
            Ok(())
        } else {
            Err(DbError(format!("client not found: {id}")))
        }
    }

    /// Incrémente le compteur no-show d'un client.
    pub fn client_increment_no_show(&self, id: &str) -> Result<(), DbError> {
        let mut map = self.clients.write().map_err(|e| DbError(e.to_string()))?;
        if let Some(c) = map.get_mut(id) {
            c.no_show_count += 1;
            Ok(())
        } else {
            Err(DbError(format!("client not found: {id}")))
        }
    }

    // -----------------------------------------------------------------------
    // Professionals
    // -----------------------------------------------------------------------

    /// Insère un professionnel.
    pub fn professional_create(&self, p: Professional) -> Result<(), DbError> {
        let id = p.id.clone();
        let mut map = self.professionals.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("professional already exists: {id}")));
        }
        map.insert(id, p);
        Ok(())
    }

    /// Récupère un professionnel par id.
    pub fn professional_by_id(&self, id: &str) -> Result<Option<Professional>, DbError> {
        let map = self.professionals.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste tous les professionnels.
    pub fn professional_list(&self) -> Result<Vec<Professional>, DbError> {
        let map = self.professionals.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Professional> = map.values().cloned().collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // Practitioners
    // -----------------------------------------------------------------------

    /// Insère un praticien.
    pub fn practitioner_create(&self, p: Practitioner) -> Result<(), DbError> {
        let id = p.id.clone();
        let mut map = self.practitioners.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("practitioner already exists: {id}")));
        }
        map.insert(id, p);
        Ok(())
    }

    /// Récupère un praticien par id.
    pub fn practitioner_by_id(&self, id: &str) -> Result<Option<Practitioner>, DbError> {
        let map = self.practitioners.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste les praticiens (optionnellement par professional_id).
    pub fn practitioner_list(&self, professional_id: Option<&str>) -> Result<Vec<Practitioner>, DbError> {
        let map = self.practitioners.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Practitioner> = map
            .values()
            .filter(|p| professional_id.map_or(true, |pid| p.professional_id == pid))
            .cloned()
            .collect();
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // Schedules
    // -----------------------------------------------------------------------

    /// Insère un planning récurrent.
    pub fn schedule_create(&self, s: Schedule) -> Result<(), DbError> {
        let id = s.id.clone();
        let mut map = self.schedules.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("schedule already exists: {id}")));
        }
        map.insert(id, s);
        Ok(())
    }

    /// Récupère un planning par id.
    pub fn schedule_by_id(&self, id: &str) -> Result<Option<Schedule>, DbError> {
        let map = self.schedules.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste les plannings (optionnellement par owner_id).
    pub fn schedule_list(&self, owner_id: Option<&str>) -> Result<Vec<Schedule>, DbError> {
        let map = self.schedules.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Schedule> = map
            .values()
            .filter(|s| owner_id.map_or(true, |oid| s.owner_id == oid))
            .cloned()
            .collect();
        out.sort_by(|a, b| (a.day_of_week, a.start_time.as_str()).cmp(&(b.day_of_week, b.start_time.as_str())));
        Ok(out)
    }

    // -----------------------------------------------------------------------
    // Exceptions
    // -----------------------------------------------------------------------

    /// Insère une exception.
    pub fn exception_create(&self, e: Exception) -> Result<(), DbError> {
        let id = e.id.clone();
        let mut map = self.exceptions.write().map_err(|e| DbError(e.to_string()))?;
        if map.contains_key(&id) {
            return Err(DbError(format!("exception already exists: {id}")));
        }
        map.insert(id, e);
        Ok(())
    }

    /// Récupère une exception par id.
    pub fn exception_by_id(&self, id: &str) -> Result<Option<Exception>, DbError> {
        let map = self.exceptions.read().map_err(|e| DbError(e.to_string()))?;
        Ok(map.get(id).cloned())
    }

    /// Liste les exceptions (optionnellement par owner_id, et par plage de dates).
    pub fn exception_list(
        &self,
        owner_id: Option<&str>,
        from_date: Option<&str>,
        to_date: Option<&str>,
    ) -> Result<Vec<Exception>, DbError> {
        let map = self.exceptions.read().map_err(|e| DbError(e.to_string()))?;
        let mut out: Vec<Exception> = map
            .values()
            .filter(|e| {
                if let Some(oid) = owner_id {
                    if e.owner_id != oid {
                        return false;
                    }
                }
                if let Some(from) = from_date {
                    if e.date.as_str() < from {
                        return false;
                    }
                }
                if let Some(to) = to_date {
                    if e.date.as_str() > to {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();
        out.sort_by(|a, b| a.date.cmp(&b.date));
        Ok(out)
    }
}
