//! Logique métier JayRDV : création, validation, mise à jour d'entités.
//!
//! @id: jayrdv_domain
//! @do: implement_jayrdv_business_logic
//! @role: domain
//! @layer: domain
//! @human: Couche domain pure — validations temporelles, cycles de vie des statuts,
//!         hold/release de créneaux, gestion des annulations.

use crate::data::{
    Appointment, AppointmentStatus, CancelledBy, Client, DbError, Exception, JayRdvStore,
    Practitioner, PractitionerRole, Professional, ProfessionalSettings, Reminder, ReminderChannel,
    Resource, Schedule, ScheduleOwner, Service, Slot, SlotStatus,
};
use chrono::{DateTime, Utc};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Vérifie que `start_at` < `end_at` (les deux doivent être ISO 8601).
fn validate_date_range(start_at: &str, end_at: &str) -> Result<(), DbError> {
    let start: DateTime<Utc> = start_at
        .parse()
        .map_err(|_| DbError(format!("invalid start_at date: {start_at}")))?;
    let end: DateTime<Utc> = end_at
        .parse()
        .map_err(|_| DbError(format!("invalid end_at date: {end_at}")))?;
    if start >= end {
        return Err(DbError(format!(
            "start_at ({start_at}) must be before end_at ({end_at})"
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Appointments
// ---------------------------------------------------------------------------

/// Crée un rendez-vous avec validation temporelle.
pub fn appointment_create(
    store: &JayRdvStore,
    title: String,
    start_at: String,
    end_at: String,
    resource_id: Option<String>,
    slot_id: Option<String>,
    service_id: Option<String>,
    client_email: Option<String>,
    client_name: Option<String>,
    client_phone: Option<String>,
    notes: Option<String>,
    location: Option<String>,
) -> Result<Appointment, DbError> {
    validate_date_range(&start_at, &end_at)?;
    let now = now_iso();
    let a = Appointment {
        id: new_uuid(),
        title,
        start_at,
        end_at,
        resource_id,
        slot_id,
        service_id,
        status: AppointmentStatus::Pending,
        client_email,
        client_name,
        client_phone,
        notes,
        location,
        cancellation_reason: None,
        cancelled_by: None,
        created_at: now.clone(),
        updated_at: now,
    };
    store.appointment_create(a.clone())?;
    Ok(a)
}

/// Met à jour le statut d'un rendez-vous (hors annulation — voir `appointment_cancel`).
pub fn appointment_set_status(
    store: &JayRdvStore,
    id: &str,
    status: AppointmentStatus,
) -> Result<Appointment, DbError> {
    if status == AppointmentStatus::Cancelled {
        return Err(DbError(
            "use appointment_cancel for cancellations".to_string(),
        ));
    }
    let mut a = store
        .appointment_by_id(id)?
        .ok_or_else(|| DbError(format!("appointment not found: {id}")))?;
    a.updated_at = now_iso();
    a.status = status;
    store.appointment_update(a.clone())?;
    Ok(a)
}

/// Annule un rendez-vous avec motif et acteur (client ou professionnel).
pub fn appointment_cancel(
    store: &JayRdvStore,
    id: &str,
    by: CancelledBy,
    reason: Option<String>,
) -> Result<Appointment, DbError> {
    let mut a = store
        .appointment_by_id(id)?
        .ok_or_else(|| DbError(format!("appointment not found: {id}")))?;
    if a.status == AppointmentStatus::Cancelled {
        return Err(DbError(format!("appointment already cancelled: {id}")));
    }
    a.status = AppointmentStatus::Cancelled;
    a.cancelled_by = Some(by);
    a.cancellation_reason = reason;
    a.updated_at = now_iso();
    store.appointment_update(a.clone())?;
    Ok(a)
}

// ---------------------------------------------------------------------------
// Resources
// ---------------------------------------------------------------------------

/// Crée une ressource.
pub fn resource_create(
    store: &JayRdvStore,
    name: String,
    kind: Option<String>,
) -> Result<Resource, DbError> {
    let now = now_iso();
    let r = Resource {
        id: new_uuid(),
        professional_id: None,
        name,
        kind,
        created_at: now.clone(),
        updated_at: now,
    };
    store.resource_create(r.clone())?;
    Ok(r)
}

// ---------------------------------------------------------------------------
// Slots
// ---------------------------------------------------------------------------

/// Crée un créneau disponible avec validation temporelle.
pub fn slot_create(
    store: &JayRdvStore,
    resource_id: String,
    service_id: Option<String>,
    start_at: String,
    end_at: String,
) -> Result<Slot, DbError> {
    validate_date_range(&start_at, &end_at)?;
    let now = now_iso();
    let s = Slot {
        id: new_uuid(),
        resource_id,
        service_id,
        start_at,
        end_at,
        status: SlotStatus::Available,
        held_until: None,
        held_by: None,
        created_at: now.clone(),
        updated_at: now,
    };
    store.slot_create(s.clone())?;
    Ok(s)
}

/// Verrouille temporairement un créneau (hold pendant le parcours client).
pub fn slot_hold(
    store: &JayRdvStore,
    slot_id: &str,
    session_id: &str,
    hold_duration_sec: i64,
) -> Result<Slot, DbError> {
    let mut s = store
        .slot_by_id(slot_id)?
        .ok_or_else(|| DbError(format!("slot not found: {slot_id}")))?;
    if s.status != SlotStatus::Available {
        return Err(DbError(format!(
            "slot not available (current: {:?}): {slot_id}",
            s.status
        )));
    }
    let until = Utc::now()
        + chrono::Duration::try_seconds(hold_duration_sec)
            .unwrap_or_else(|| chrono::Duration::try_seconds(300).unwrap());
    s.status = SlotStatus::Held;
    s.held_until = Some(until.to_rfc3339());
    s.held_by = Some(session_id.to_string());
    s.updated_at = now_iso();
    store.slot_update(s.clone())?;
    Ok(s)
}

/// Libère un créneau verrouillé (revient à Available).
pub fn slot_release(store: &JayRdvStore, slot_id: &str) -> Result<Slot, DbError> {
    let mut s = store
        .slot_by_id(slot_id)?
        .ok_or_else(|| DbError(format!("slot not found: {slot_id}")))?;
    if s.status != SlotStatus::Held {
        return Err(DbError(format!(
            "slot not held (current: {:?}): {slot_id}",
            s.status
        )));
    }
    s.status = SlotStatus::Available;
    s.held_until = None;
    s.held_by = None;
    s.updated_at = now_iso();
    store.slot_update(s.clone())?;
    Ok(s)
}

/// Libère tous les créneaux dont le hold a expiré (held_until < now). Retourne le nombre de créneaux libérés.
pub fn slot_release_expired(store: &JayRdvStore) -> Result<usize, DbError> {
    let ids = store.slot_list_expired_holds()?;
    let mut count = 0;
    for id in &ids {
        if slot_release(store, id).is_ok() {
            count += 1;
        }
    }
    Ok(count)
}

// ---------------------------------------------------------------------------
// Professional
// ---------------------------------------------------------------------------

/// Crée un professionnel avec paramètres par défaut.
pub fn professional_create(
    store: &JayRdvStore,
    name: String,
    slug: String,
    sector: String,
    contact_email: String,
    timezone: String,
    description: Option<String>,
    contact_phone: Option<String>,
    address: Option<String>,
    photo_url: Option<String>,
    settings: Option<ProfessionalSettings>,
) -> Result<Professional, DbError> {
    let now = now_iso();
    let p = Professional {
        id: new_uuid(),
        name,
        slug,
        description,
        sector,
        photo_url,
        contact_email,
        contact_phone,
        address,
        timezone,
        settings: settings.unwrap_or_default(),
        created_at: now.clone(),
        updated_at: now,
    };
    store.professional_create(p.clone())?;
    Ok(p)
}

// ---------------------------------------------------------------------------
// Practitioner
// ---------------------------------------------------------------------------

/// Crée un praticien (collaborateur).
pub fn practitioner_create(
    store: &JayRdvStore,
    professional_id: String,
    name: String,
    role: PractitionerRole,
    service_ids: Vec<String>,
    photo_url: Option<String>,
) -> Result<Practitioner, DbError> {
    let now = now_iso();
    let p = Practitioner {
        id: new_uuid(),
        professional_id,
        name,
        role,
        photo_url,
        service_ids,
        created_at: now.clone(),
        updated_at: now,
    };
    store.practitioner_create(p.clone())?;
    Ok(p)
}

// ---------------------------------------------------------------------------
// Schedule
// ---------------------------------------------------------------------------

/// Crée une plage horaire récurrente (planning).
pub fn schedule_create(
    store: &JayRdvStore,
    owner_type: ScheduleOwner,
    owner_id: String,
    day_of_week: u8,
    start_time: String,
    end_time: String,
    active: bool,
) -> Result<Schedule, DbError> {
    if day_of_week > 6 {
        return Err(DbError("day_of_week must be 0-6 (0=lundi)".to_string()));
    }
    let s = Schedule {
        id: new_uuid(),
        owner_type,
        owner_id,
        day_of_week,
        start_time,
        end_time,
        active,
    };
    store.schedule_create(s.clone())?;
    Ok(s)
}

// ---------------------------------------------------------------------------
// Exception
// ---------------------------------------------------------------------------

/// Crée une exception (congés, absence, fermeture).
pub fn exception_create(
    store: &JayRdvStore,
    owner_type: ScheduleOwner,
    owner_id: String,
    date: String,
    start_time: Option<String>,
    end_time: Option<String>,
    reason: Option<String>,
) -> Result<Exception, DbError> {
    let e = Exception {
        id: new_uuid(),
        owner_type,
        owner_id,
        date,
        start_time,
        end_time,
        reason,
    };
    store.exception_create(e.clone())?;
    Ok(e)
}

// ---------------------------------------------------------------------------
// Services
// ---------------------------------------------------------------------------

/// Crée un service (prestation).
pub fn service_create(
    store: &JayRdvStore,
    name: String,
    description: Option<String>,
    duration_min: u32,
    price: Option<f64>,
    category: Option<String>,
) -> Result<Service, DbError> {
    let now = now_iso();
    let s = Service {
        id: new_uuid(),
        professional_id: None,
        name,
        description,
        duration_min,
        price,
        category,
        active: true,
        created_at: now.clone(),
        updated_at: now,
    };
    store.service_create(s.clone())?;
    Ok(s)
}

// ---------------------------------------------------------------------------
// Clients
// ---------------------------------------------------------------------------

/// Crée une fiche client.
pub fn client_create(
    store: &JayRdvStore,
    name: String,
    email: Option<String>,
    phone: String,
    notes: Option<String>,
) -> Result<Client, DbError> {
    let now = now_iso();
    let c = Client {
        id: new_uuid(),
        professional_id: None,
        name,
        email,
        phone,
        notes,
        total_appointments: 0,
        no_show_count: 0,
        created_at: now.clone(),
        updated_at: now,
    };
    store.client_create(c.clone())?;
    Ok(c)
}

// ---------------------------------------------------------------------------
// Reminders
// ---------------------------------------------------------------------------

/// Enregistre un rappel pour un rendez-vous.
pub fn reminder_create(
    store: &JayRdvStore,
    appointment_id: String,
    channel: ReminderChannel,
    remind_at: String,
) -> Result<Reminder, DbError> {
    let r = Reminder {
        id: new_uuid(),
        appointment_id,
        channel,
        remind_at,
        sent: false,
        created_at: now_iso(),
    };
    store.reminder_create(r.clone())?;
    Ok(r)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_store() -> JayRdvStore {
        JayRdvStore::new()
    }

    // -- Validations -------------------------------------------------------

    #[test]
    fn validate_date_range_ok() {
        let r = validate_date_range("2026-03-01T10:00:00+00:00", "2026-03-01T11:00:00+00:00");
        assert!(r.is_ok());
    }

    #[test]
    fn validate_date_range_equal_fails() {
        let r = validate_date_range("2026-03-01T10:00:00+00:00", "2026-03-01T10:00:00+00:00");
        assert!(r.is_err());
    }

    #[test]
    fn validate_date_range_reversed_fails() {
        let r = validate_date_range("2026-03-01T11:00:00+00:00", "2026-03-01T10:00:00+00:00");
        assert!(r.is_err());
    }

    #[test]
    fn validate_date_range_bad_format_fails() {
        let r = validate_date_range("not-a-date", "2026-03-01T10:00:00+00:00");
        assert!(r.is_err());
    }

    // -- Appointments ------------------------------------------------------

    #[test]
    fn appointment_create_and_list() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV test".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        assert_eq!(a.status, AppointmentStatus::Pending);
        let list = store.appointment_list(None, None, None, None).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, a.id);
    }

    #[test]
    fn appointment_create_invalid_dates() {
        let store = make_store();
        let r = appointment_create(
            &store,
            "Bad".into(),
            "2026-03-01T12:00:00+00:00".into(),
            "2026-03-01T10:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        );
        assert!(r.is_err());
    }

    #[test]
    fn appointment_duplicate_id_rejected() {
        let store = make_store();
        let now = now_iso();
        let a = Appointment {
            id: "dup-id".into(),
            title: "First".into(),
            start_at: "2026-03-01T10:00:00+00:00".into(),
            end_at: "2026-03-01T11:00:00+00:00".into(),
            resource_id: None,
            slot_id: None,
            service_id: None,
            status: AppointmentStatus::Pending,
            client_email: None,
            client_name: None,
            client_phone: None,
            notes: None,
            location: None,
            cancellation_reason: None,
            cancelled_by: None,
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        store.appointment_create(a.clone()).unwrap();
        let r = store.appointment_create(a);
        assert!(r.is_err());
    }

    #[test]
    fn appointment_set_status_ok() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        let a2 = appointment_set_status(&store, &a.id, AppointmentStatus::Confirmed).unwrap();
        assert_eq!(a2.status, AppointmentStatus::Confirmed);
    }

    #[test]
    fn appointment_set_status_cancel_rejected() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        let r = appointment_set_status(&store, &a.id, AppointmentStatus::Cancelled);
        assert!(r.is_err());
    }

    // -- Cancellation ------------------------------------------------------

    #[test]
    fn appointment_cancel_by_client() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        let cancelled =
            appointment_cancel(&store, &a.id, CancelledBy::Client, Some("Empêché".into()))
                .unwrap();
        assert_eq!(cancelled.status, AppointmentStatus::Cancelled);
        assert_eq!(cancelled.cancelled_by, Some(CancelledBy::Client));
        assert_eq!(cancelled.cancellation_reason.as_deref(), Some("Empêché"));
    }

    #[test]
    fn appointment_cancel_twice_fails() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        appointment_cancel(&store, &a.id, CancelledBy::Professional, None).unwrap();
        let r = appointment_cancel(&store, &a.id, CancelledBy::Client, None);
        assert!(r.is_err());
    }

    // -- Slots -------------------------------------------------------------

    #[test]
    fn slot_create_and_list() {
        let store = make_store();
        let r = resource_create(&store, "Salle A".into(), Some("room".into())).unwrap();
        let s = slot_create(
            &store,
            r.id.clone(),
            None,
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
        )
        .unwrap();
        assert_eq!(s.status, SlotStatus::Available);
        let list = store.slot_list(Some(&r.id), None).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn slot_hold_and_release() {
        let store = make_store();
        let r = resource_create(&store, "Salle B".into(), None).unwrap();
        let s = slot_create(
            &store,
            r.id.clone(),
            None,
            "2026-03-01T14:00:00+00:00".into(),
            "2026-03-01T15:00:00+00:00".into(),
        )
        .unwrap();
        let held = slot_hold(&store, &s.id, "session-123", 300).unwrap();
        assert_eq!(held.status, SlotStatus::Held);
        assert!(held.held_until.is_some());
        assert_eq!(held.held_by.as_deref(), Some("session-123"));

        let released = slot_release(&store, &s.id).unwrap();
        assert_eq!(released.status, SlotStatus::Available);
        assert!(released.held_until.is_none());
    }

    #[test]
    fn slot_hold_not_available_fails() {
        let store = make_store();
        let r = resource_create(&store, "Salle C".into(), None).unwrap();
        let s = slot_create(
            &store,
            r.id.clone(),
            None,
            "2026-03-01T14:00:00+00:00".into(),
            "2026-03-01T15:00:00+00:00".into(),
        )
        .unwrap();
        slot_hold(&store, &s.id, "s1", 300).unwrap();
        let r2 = slot_hold(&store, &s.id, "s2", 300);
        assert!(r2.is_err());
    }

    #[test]
    fn slot_release_expired_releases_only_expired_holds() {
        let store = make_store();
        let r = resource_create(&store, "Salle D".into(), None).unwrap();
        let now = now_iso();
        let expired_slot = Slot {
            id: new_uuid(),
            resource_id: r.id.clone(),
            service_id: None,
            start_at: "2026-05-01T10:00:00+00:00".into(),
            end_at: "2026-05-01T11:00:00+00:00".into(),
            status: SlotStatus::Held,
            held_until: Some("2020-01-01T00:00:00+00:00".into()),
            held_by: Some("old-session".into()),
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        store.slot_create(expired_slot.clone()).unwrap();
        let count = slot_release_expired(&store).unwrap();
        assert_eq!(count, 1);
        let s = store.slot_by_id(&expired_slot.id).unwrap().unwrap();
        assert_eq!(s.status, SlotStatus::Available);
        assert!(s.held_until.is_none());
    }

    // -- Professional & Schedule & Exception -------------------------------

    #[test]
    fn professional_create_and_list() {
        let store = make_store();
        let p = professional_create(
            &store,
            "Marie Coiffure".into(),
            "marie-coiffure-paris".into(),
            "beauté".into(),
            "marie@example.com".into(),
            "Europe/Paris".into(),
            None,
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert_eq!(p.settings.hold_duration_min, 10);
        let list = store.professional_list().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].slug, "marie-coiffure-paris");
    }

    #[test]
    fn schedule_create_and_list() {
        let store = make_store();
        let s = schedule_create(
            &store,
            ScheduleOwner::Practitioner,
            "pract-1".into(),
            1, // mardi
            "09:00".into(),
            "18:00".into(),
            true,
        )
        .unwrap();
        assert!(s.active);
        let list = store.schedule_list(Some("pract-1")).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].day_of_week, 1);
    }

    #[test]
    fn exception_create_and_list() {
        let store = make_store();
        let e = exception_create(
            &store,
            ScheduleOwner::Professional,
            "pro-1".into(),
            "2026-08-15".into(),
            None,
            None,
            Some("Congés".into()),
        )
        .unwrap();
        assert_eq!(e.reason.as_deref(), Some("Congés"));
        let list = store.exception_list(Some("pro-1"), Some("2026-08-01"), Some("2026-08-31")).unwrap();
        assert_eq!(list.len(), 1);
    }

    // -- Services ----------------------------------------------------------

    #[test]
    fn service_create_and_list() {
        let store = make_store();
        let s = service_create(
            &store,
            "Coupe homme".into(),
            Some("Coupe classique".into()),
            30,
            Some(25.0),
            Some("Coiffure".into()),
        )
        .unwrap();
        assert!(s.active);
        let list = store.service_list(true).unwrap();
        assert_eq!(list.len(), 1);
    }

    // -- Clients -----------------------------------------------------------

    #[test]
    fn client_create_and_find() {
        let store = make_store();
        let c = client_create(
            &store,
            "Alice Dupont".into(),
            Some("alice@test.com".into()),
            "+33612345678".into(),
            None,
        )
        .unwrap();
        assert_eq!(c.total_appointments, 0);

        let found = store.client_by_phone("+33612345678").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, c.id);
    }

    #[test]
    fn client_increment_counters() {
        let store = make_store();
        let c = client_create(&store, "Bob".into(), None, "+33600000000".into(), None).unwrap();
        store.client_increment_appointments(&c.id).unwrap();
        store.client_increment_appointments(&c.id).unwrap();
        store.client_increment_no_show(&c.id).unwrap();
        let c2 = store.client_by_id(&c.id).unwrap().unwrap();
        assert_eq!(c2.total_appointments, 2);
        assert_eq!(c2.no_show_count, 1);
    }

    // -- Reminders ---------------------------------------------------------

    #[test]
    fn reminder_create_and_mark_sent() {
        let store = make_store();
        let a = appointment_create(
            &store,
            "RDV rappel".into(),
            "2026-03-01T10:00:00+00:00".into(),
            "2026-03-01T11:00:00+00:00".into(),
            None, None, None, None, None, None, None, None,
        )
        .unwrap();
        let r = reminder_create(
            &store,
            a.id.clone(),
            ReminderChannel::Email,
            "2026-02-28T10:00:00+00:00".into(),
        )
        .unwrap();
        assert!(!r.sent);

        store.reminder_mark_sent(&r.id).unwrap();
        let list = store.reminder_list_for_appointment(&a.id).unwrap();
        assert_eq!(list.len(), 1);
        assert!(list[0].sent);
    }

    // -- Full cycle --------------------------------------------------------

    #[test]
    fn full_booking_cycle() {
        let store = make_store();

        let svc = service_create(&store, "Massage 60min".into(), None, 60, Some(80.0), None).unwrap();
        let res = resource_create(&store, "Sophie".into(), Some("person".into())).unwrap();
        let client = client_create(&store, "Charlie".into(), Some("c@test.com".into()), "+33611111111".into(), None).unwrap();

        let slot = slot_create(
            &store, res.id.clone(), Some(svc.id.clone()),
            "2026-04-01T14:00:00+00:00".into(), "2026-04-01T15:00:00+00:00".into(),
        ).unwrap();

        let held = slot_hold(&store, &slot.id, "web-session-42", 300).unwrap();
        assert_eq!(held.status, SlotStatus::Held);

        let appt = appointment_create(
            &store, "Massage 60min — Charlie".into(),
            "2026-04-01T14:00:00+00:00".into(), "2026-04-01T15:00:00+00:00".into(),
            Some(res.id.clone()), Some(slot.id.clone()), Some(svc.id.clone()),
            Some("c@test.com".into()), Some("Charlie".into()), Some("+33611111111".into()),
            None, None,
        ).unwrap();

        let confirmed = appointment_set_status(&store, &appt.id, AppointmentStatus::Confirmed).unwrap();
        assert_eq!(confirmed.status, AppointmentStatus::Confirmed);
        store.client_increment_appointments(&client.id).unwrap();

        let _reminder = reminder_create(
            &store, appt.id.clone(), ReminderChannel::Sms, "2026-03-31T14:00:00+00:00".into(),
        ).unwrap();

        let completed = appointment_set_status(&store, &appt.id, AppointmentStatus::Completed).unwrap();
        assert_eq!(completed.status, AppointmentStatus::Completed);

        let c_final = store.client_by_id(&client.id).unwrap().unwrap();
        assert_eq!(c_final.total_appointments, 1);
    }
}
