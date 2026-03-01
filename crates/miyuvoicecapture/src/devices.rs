//! Enumeration et identification des peripheriques audio.
//!
//! Utilise cpal pour lister les peripheriques d'entree/sortie via le host natif
//! (WASAPI sur Windows). Genere un identifiant stable par device via blake3.

// @id: alicia_capture_devices
// @role: infrastructure
// @layer: toolkit
// @human: Enumeration des peripheriques audio, filtrage par nom/ID stable, hash blake3.
// @do: enumerate_audio_devices

use cpal::traits::{DeviceTrait, HostTrait};
use serde::{Deserialize, Serialize};

use crate::errors::VoiceCaptureError;

/// Informations sur un peripherique audio detecte.
///
/// Contient l'identifiant stable (hash blake3), le nom, les taux
/// d'echantillonnage supportes, le nombre de canaux et le statut par defaut.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceInfo {
    /// Identifiant stable genere par blake3 (hash du nom + host).
    pub id: String,
    /// Nom du peripherique tel que rapporte par le systeme.
    pub name: String,
    /// Taux d'echantillonnage supportes (Hz).
    pub sample_rates: Vec<u32>,
    /// Nombre maximum de canaux supportes.
    pub channels: u16,
    /// `true` si c'est le peripherique par defaut du systeme.
    pub is_default: bool,
}

/// Genere un identifiant stable pour un peripherique audio.
///
/// Normalise le nom (lowercase, trim) puis utilise blake3 pour hasher
/// la combinaison nom + identifiant host, produisant un ID deterministe
/// et reproductible entre sessions. Tronque a 16 caracteres hex.
#[must_use]
pub fn generate_stable_id(device_name: &str, host_id: &str) -> String {
    let normalized_name = device_name.trim().to_lowercase();
    let normalized_host = host_id.trim().to_lowercase();
    let input = format!("{normalized_name}::{normalized_host}");
    let hash = blake3::hash(input.as_bytes());
    hash.to_hex()[..16].to_string()
}

/// Enumerateur de peripheriques audio.
///
/// Encapsule l'acces au host audio cpal et fournit des methodes
/// pour lister, filtrer et identifier les peripheriques.
pub struct DeviceEnumerator {
    /// Identifiant du host audio (ex. "WASAPI").
    host_id: String,
}

impl DeviceEnumerator {
    /// Cree un enumerateur utilisant le host audio par defaut du systeme.
    ///
    /// Sur Windows, cela utilise WASAPI. Sur Linux, ALSA. Sur macOS, CoreAudio.
    #[must_use]
    pub fn new() -> Self {
        let host = cpal::default_host();
        Self {
            host_id: format!("{:?}", host.id()),
        }
    }

    /// Liste tous les peripheriques d'entree audio disponibles.
    pub fn list_input_devices(&self) -> Result<Vec<AudioDeviceInfo>, VoiceCaptureError> {
        let host = cpal::default_host();

        let default_input_name = host
            .default_input_device()
            .and_then(|d| d.description().ok().map(|desc| desc.name().to_string()));

        let devices = host
            .input_devices()
            .map_err(|e| VoiceCaptureError::DeviceEnumeration(e.to_string()))?;

        let mut result = Vec::new();
        for device in devices {
            let info = self.build_device_info(&device, default_input_name.as_ref())?;
            result.push(info);
        }

        Ok(result)
    }

    /// Liste tous les peripheriques de sortie audio disponibles.
    pub fn list_output_devices(&self) -> Result<Vec<AudioDeviceInfo>, VoiceCaptureError> {
        let host = cpal::default_host();

        let default_output_name = host
            .default_output_device()
            .and_then(|d| d.description().ok().map(|desc| desc.name().to_string()));

        let devices = host
            .output_devices()
            .map_err(|e| VoiceCaptureError::DeviceEnumeration(e.to_string()))?;

        let mut result = Vec::new();
        for device in devices {
            let info = self.build_device_info(&device, default_output_name.as_ref())?;
            result.push(info);
        }

        Ok(result)
    }

    /// Recherche un peripherique d'entree par son nom (correspondance partielle).
    pub fn find_input_by_name(
        &self,
        name_fragment: &str,
    ) -> Result<Option<AudioDeviceInfo>, VoiceCaptureError> {
        let devices = self.list_input_devices()?;
        let lower_fragment = name_fragment.to_lowercase();
        Ok(devices
            .into_iter()
            .find(|d| d.name.to_lowercase().contains(&lower_fragment)))
    }

    /// Recherche un peripherique d'entree par son identifiant stable.
    pub fn find_input_by_id(
        &self,
        stable_id: &str,
    ) -> Result<Option<AudioDeviceInfo>, VoiceCaptureError> {
        let devices = self.list_input_devices()?;
        Ok(devices.into_iter().find(|d| d.id == stable_id))
    }

    /// Retourne le peripherique d'entree par defaut du systeme.
    pub fn default_input_device(&self) -> Result<AudioDeviceInfo, VoiceCaptureError> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or(VoiceCaptureError::NoInputDevice)?;

        let name = device
            .description()
            .map(|d| d.name().to_string())
            .map_err(|e| VoiceCaptureError::DeviceEnumeration(e.to_string()))?;

        let (sample_rates, channels) = self.query_supported_configs(&device)?;

        Ok(AudioDeviceInfo {
            id: generate_stable_id(&name, &self.host_id),
            name,
            sample_rates,
            channels,
            is_default: true,
        })
    }

    /// Construit les informations d'un peripherique a partir d'un `cpal::Device`.
    fn build_device_info(
        &self,
        device: &cpal::Device,
        default_name: Option<&String>,
    ) -> Result<AudioDeviceInfo, VoiceCaptureError> {
        let name = device
            .description()
            .map(|d| d.name().to_string())
            .map_err(|e| VoiceCaptureError::DeviceEnumeration(e.to_string()))?;

        let is_default = default_name
            .is_some_and(|default| *default == name);

        let (sample_rates, channels) = self.query_supported_configs(device)?;

        Ok(AudioDeviceInfo {
            id: generate_stable_id(&name, &self.host_id),
            name,
            sample_rates,
            channels,
            is_default,
        })
    }

    /// Interroge les configurations supportees par un peripherique.
    ///
    /// Retourne la liste des taux d'echantillonnage et le nombre max de canaux.
    /// Tente d'abord les configs d'entree, puis de sortie en cas d'echec.
    fn query_supported_configs(
        &self,
        device: &cpal::Device,
    ) -> Result<(Vec<u32>, u16), VoiceCaptureError> {
        let mut sample_rates = Vec::new();
        let mut max_channels: u16 = 1;

        // Collecter les ranges depuis les configs d'entree ou de sortie
        let configs: Vec<cpal::SupportedStreamConfigRange> =
            if let Ok(input_configs) = device.supported_input_configs() {
                input_configs.collect()
            } else if let Ok(output_configs) = device.supported_output_configs() {
                output_configs.collect()
            } else {
                return Err(VoiceCaptureError::DeviceEnumeration(
                    "Cannot query supported configs for device".to_string(),
                ));
            };

        for config in &configs {
            let min_rate = config.min_sample_rate();
            let max_rate = config.max_sample_rate();
            let ch = config.channels();

            if ch > max_channels {
                max_channels = ch;
            }

            // Ajouter les taux standards supportes dans la plage
            for &standard_rate in &[8_000u32, 16_000, 22_050, 44_100, 48_000, 96_000] {
                if standard_rate >= min_rate
                    && standard_rate <= max_rate
                    && !sample_rates.contains(&standard_rate)
                {
                    sample_rates.push(standard_rate);
                }
            }
        }

        sample_rates.sort_unstable();
        Ok((sample_rates, max_channels))
    }
}

impl Default for DeviceEnumerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_stable_id_deterministic() {
        let id1 = generate_stable_id("Microphone USB", "WASAPI");
        let id2 = generate_stable_id("Microphone USB", "WASAPI");
        assert_eq!(id1, id2);
        // Hash blake3 tronque a 16 caracteres hex
        assert_eq!(id1.len(), 16);
    }

    #[test]
    fn test_generate_stable_id_case_insensitive() {
        let id1 = generate_stable_id("Microphone USB", "WASAPI");
        let id2 = generate_stable_id("microphone usb", "wasapi");
        assert_eq!(id1, id2, "IDs should be identical regardless of case");
    }

    #[test]
    fn test_generate_stable_id_trims_whitespace() {
        let id1 = generate_stable_id("Microphone USB", "WASAPI");
        let id2 = generate_stable_id("  Microphone USB  ", "  WASAPI  ");
        assert_eq!(id1, id2, "IDs should be identical regardless of whitespace");
    }

    #[test]
    fn test_generate_stable_id_different_inputs() {
        let id1 = generate_stable_id("Microphone USB", "WASAPI");
        let id2 = generate_stable_id("Microphone Builtin", "WASAPI");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_stable_id_different_hosts() {
        let id1 = generate_stable_id("Microphone USB", "WASAPI");
        let id2 = generate_stable_id("Microphone USB", "ALSA");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_device_info_serialization() {
        let info = AudioDeviceInfo {
            id: "abc123".to_string(),
            name: "Test Device".to_string(),
            sample_rates: vec![16_000, 48_000],
            channels: 2,
            is_default: true,
        };
        let json = serde_json::to_string(&info);
        assert!(json.is_ok());
    }

    // Note: les tests d'enumeration reelle des devices necessitent
    // un environnement avec peripheriques audio (CI/CD peut echouer).
    // Ils sont couverts par les tests d'integration.
}
