//! Adaptateurs pour les dispositifs HTTP locaux.
//!
//! Chaque adaptateur traduit les reponses d'un fabricant vers `DeviceState` Alicia
//! et construit les requetes HTTP depuis `DeviceCommand`.

// @id: toolkit.alicia.http.adapters
// @role: adapter_registry
// @layer: 6
// @human: Registre des adaptateurs HTTP : Shelly, generique JSON.
// @do: expose_http_device_adapters

pub mod generic;
pub mod shelly;
