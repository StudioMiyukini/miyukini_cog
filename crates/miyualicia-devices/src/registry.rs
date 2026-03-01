//! Registre in-memory des dispositifs domotiques.
//!
//! `DeviceRegistry` stocke les dispositifs et leur etat courant dans une `HashMap`.
//! Il est concu pour etre utilise derriere un `Arc<RwLock<DeviceRegistry>>`.

// @id: toolkit.alicia.devices.registry
// @role: in_memory_registry
// @layer: 6
// @human: Registre in-memory thread-safe des dispositifs domotiques Alicia.
// @do: provide_device_registry_crud

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::errors::DeviceError;
use crate::types::{Device, DeviceState};

/// Type alias pour le registre partage entre threads.
/// A utiliser dans `AliciaService` et les handlers API.
pub type SharedDeviceRegistry = Arc<RwLock<DeviceRegistry>>;

/// Registre in-memory de tous les dispositifs domotiques.
///
/// # Thread-safety
///
/// `DeviceRegistry` est concu pour un acces concurrent via `Arc<RwLock<DeviceRegistry>>`.
/// Les lectures sont sans contention entre elles. Les ecritures bloquent les lectures
/// le temps de la mutation. Les ecritures sont courtes (HashMap insert/remove).
///
/// # Invariants
///
/// - Un seul dispositif par UUID (unicite garantie par `add_device`).
/// - L'etat d'un dispositif absent du registre ne peut pas etre mis a jour.
/// - `list_by_room` retourne uniquement les dispositifs `active = true`.
#[derive(Debug, Default)]
pub struct DeviceRegistry {
    devices: HashMap<Uuid, Device>,
    states: HashMap<Uuid, DeviceState>,
}

impl DeviceRegistry {
    /// Cree un registre vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un dispositif et son etat initial.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::AlreadyExists` si un dispositif avec le meme UUID existe.
    pub fn add_device(&mut self, device: Device, state: DeviceState) -> Result<(), DeviceError> {
        if self.devices.contains_key(&device.id) {
            return Err(DeviceError::AlreadyExists(device.id));
        }
        let id = device.id;
        self.devices.insert(id, device);
        self.states.insert(id, state);
        Ok(())
    }

    /// Supprime un dispositif et son etat associe.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::NotFound` si l'UUID est absent.
    pub fn remove_device(&mut self, id: Uuid) -> Result<(), DeviceError> {
        if self.devices.remove(&id).is_none() {
            return Err(DeviceError::NotFound(id));
        }
        self.states.remove(&id);
        Ok(())
    }

    /// Retourne une reference au dispositif, ou `DeviceError::NotFound`.
    pub fn get_device(&self, id: Uuid) -> Result<&Device, DeviceError> {
        self.devices.get(&id).ok_or(DeviceError::NotFound(id))
    }

    /// Retourne une reference mutable au dispositif, ou `DeviceError::NotFound`.
    pub fn get_device_mut(&mut self, id: Uuid) -> Result<&mut Device, DeviceError> {
        self.devices.get_mut(&id).ok_or(DeviceError::NotFound(id))
    }

    /// Retourne l'etat courant d'un dispositif, ou `DeviceError::NotFound`.
    pub fn get_state(&self, id: Uuid) -> Result<&DeviceState, DeviceError> {
        self.states.get(&id).ok_or(DeviceError::NotFound(id))
    }

    /// Met a jour l'etat d'un dispositif existant.
    ///
    /// # Erreurs
    ///
    /// Retourne `DeviceError::NotFound` si le dispositif est absent du registre.
    pub fn update_state(&mut self, state: DeviceState) -> Result<(), DeviceError> {
        let id = state.device_id;
        if !self.devices.contains_key(&id) {
            return Err(DeviceError::NotFound(id));
        }
        self.states.insert(id, state);
        Ok(())
    }

    /// Retourne tous les dispositifs actifs d'une piece, avec leur etat.
    pub fn list_by_room(&self, room_id: &str) -> Vec<(&Device, &DeviceState)> {
        self.devices
            .values()
            .filter(|d| d.active && d.room_id == room_id)
            .filter_map(|d| self.states.get(&d.id).map(|s| (d, s)))
            .collect()
    }

    /// Retourne tous les dispositifs (actifs et inactifs), avec leur etat.
    pub fn list_all(&self) -> Vec<(&Device, &DeviceState)> {
        self.devices
            .values()
            .filter_map(|d| self.states.get(&d.id).map(|s| (d, s)))
            .collect()
    }

    /// Retourne le nombre de dispositifs dans le registre.
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    /// Retourne `true` si le registre est vide.
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        DeviceCapabilities, DeviceConfig, DeviceProtocol, DeviceState, DeviceType,
    };
    use chrono::Utc;

    fn make_device(room_id: &str) -> (Device, DeviceState) {
        let id = Uuid::new_v4();
        let device = Device {
            id,
            room_id: room_id.to_string(),
            device_type: DeviceType::Light,
            name: "Lampe test".to_string(),
            protocol: DeviceProtocol::Mqtt,
            address: "alicia/salon/lampe".to_string(),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let state = DeviceState::unknown(id);
        (device, state)
    }

    fn make_device_typed(
        room_id: &str,
        device_type: DeviceType,
        active: bool,
    ) -> (Device, DeviceState) {
        let id = Uuid::new_v4();
        let device = Device {
            id,
            room_id: room_id.to_string(),
            device_type,
            name: format!("Test {device_type}"),
            protocol: DeviceProtocol::Mqtt,
            address: format!("alicia/{room_id}/test"),
            capabilities: DeviceCapabilities::default(),
            config: DeviceConfig::default(),
            active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let state = DeviceState::unknown(id);
        (device, state)
    }

    // TC-DEV-01 : ajouter un dispositif au registre
    #[test]
    fn test_add_device_success() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        assert!(reg.add_device(device, state).is_ok());
        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());
    }

    // TC-DEV-02 : double ajout retourne AlreadyExists
    #[test]
    fn test_add_device_duplicate_fails() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        let device2 = device.clone();
        let state2 = state.clone();
        reg.add_device(device, state).unwrap();
        let err = reg.add_device(device2, state2).unwrap_err();
        assert!(matches!(err, DeviceError::AlreadyExists(_)));
    }

    // TC-DEV-03 : get_device retourne le bon dispositif
    #[test]
    fn test_get_device_found() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        let id = device.id;
        reg.add_device(device, state).unwrap();
        let found = reg.get_device(id).unwrap();
        assert_eq!(found.id, id);
        assert_eq!(found.name, "Lampe test");
    }

    // TC-DEV-04 : get_device sur UUID absent retourne NotFound
    #[test]
    fn test_get_device_not_found() {
        let reg = DeviceRegistry::new();
        let err = reg.get_device(Uuid::new_v4()).unwrap_err();
        assert!(matches!(err, DeviceError::NotFound(_)));
    }

    // TC-DEV-05 : update_state met a jour l'etat
    #[test]
    fn test_update_state_success() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        let id = device.id;
        reg.add_device(device, state).unwrap();

        let mut new_state = DeviceState::unknown(id);
        new_state.on = Some(true);
        new_state.brightness = Some(80);
        reg.update_state(new_state).unwrap();

        let retrieved = reg.get_state(id).unwrap();
        assert_eq!(retrieved.on, Some(true));
        assert_eq!(retrieved.brightness, Some(80));
    }

    // TC-DEV-06 : update_state sur dispositif absent retourne NotFound
    #[test]
    fn test_update_state_not_found() {
        let mut reg = DeviceRegistry::new();
        let state = DeviceState::unknown(Uuid::new_v4());
        let err = reg.update_state(state).unwrap_err();
        assert!(matches!(err, DeviceError::NotFound(_)));
    }

    // TC-DEV-07 : list_by_room filtre correctement
    #[test]
    fn test_list_by_room_filters_correctly() {
        let mut reg = DeviceRegistry::new();
        let (d1, s1) = make_device("salon");
        let (d2, s2) = make_device("cuisine");
        let (d3, s3) = make_device("salon");
        reg.add_device(d1, s1).unwrap();
        reg.add_device(d2, s2).unwrap();
        reg.add_device(d3, s3).unwrap();

        let salon = reg.list_by_room("salon");
        assert_eq!(salon.len(), 2);
        let cuisine = reg.list_by_room("cuisine");
        assert_eq!(cuisine.len(), 1);
        let chambre = reg.list_by_room("chambre");
        assert!(chambre.is_empty());
    }

    // TC-DEV-08 : list_by_room exclut les dispositifs inactifs
    #[test]
    fn test_list_by_room_excludes_inactive() {
        let mut reg = DeviceRegistry::new();
        let (d_active, s_active) = make_device_typed("salon", DeviceType::Light, true);
        let (d_inactive, s_inactive) = make_device_typed("salon", DeviceType::Outlet, false);
        reg.add_device(d_active, s_active).unwrap();
        reg.add_device(d_inactive, s_inactive).unwrap();

        let salon = reg.list_by_room("salon");
        assert_eq!(salon.len(), 1);
        // list_all retourne les deux
        assert_eq!(reg.list_all().len(), 2);
    }

    // TC-DEV-09 : remove_device puis get_device retourne NotFound
    #[test]
    fn test_remove_device() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        let id = device.id;
        reg.add_device(device, state).unwrap();
        assert_eq!(reg.len(), 1);

        reg.remove_device(id).unwrap();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());
        assert!(reg.get_device(id).is_err());
        assert!(reg.get_state(id).is_err());
    }

    // TC-DEV-10 : remove_device sur UUID absent retourne NotFound
    #[test]
    fn test_remove_device_not_found() {
        let mut reg = DeviceRegistry::new();
        let err = reg.remove_device(Uuid::new_v4()).unwrap_err();
        assert!(matches!(err, DeviceError::NotFound(_)));
    }

    // TC-DEV-11 : get_device_mut permet la modification
    #[test]
    fn test_get_device_mut_modification() {
        let mut reg = DeviceRegistry::new();
        let (device, state) = make_device("salon");
        let id = device.id;
        reg.add_device(device, state).unwrap();

        let dev = reg.get_device_mut(id).unwrap();
        dev.name = "Lampe modifiee".to_string();

        let found = reg.get_device(id).unwrap();
        assert_eq!(found.name, "Lampe modifiee");
    }
}
