//! Messages du protocole de synchronisation P2P.
//!
//! @id: miyucloud_sync_protocol
//! @do: define_sync_protocol_messages_and_manifest_entry
//! @role: sync
//! @layer: infra
//!
//! Definit les messages echanges entre pairs pendant la synchronisation :
//! demandes de manifeste, echanges de chunks, notifications de modification,
//! et acquittements. Chaque message est serialisable/deserialisable via serde.

use crate::sync::vector_clock::VectorClock;
use serde::{Deserialize, Serialize};

/// Messages echanges entre pairs sync.
///
/// Le protocole est asymetrique : un pair demande, l'autre repond.
/// Les messages sont chiffres E2E avant d'etre envoyes sur le transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMessage {
    /// Demande : "envoie-moi ton manifeste de l'etat de tes fichiers".
    ManifestRequest {
        /// COG ID de l'emetteur.
        sender_cog_id: String,
        /// IDs des dossiers a synchroniser (vide = tous).
        folder_ids: Vec<String>,
    },
    /// Reponse : voici mon manifeste.
    ManifestResponse {
        /// Liste des entrees du manifeste.
        entries: Vec<ManifestEntry>,
    },
    /// Demande : "envoie-moi ces chunks".
    ChunkRequest {
        /// UUID du fichier.
        file_id: String,
        /// Indices des chunks demandes.
        chunk_indices: Vec<u32>,
    },
    /// Reponse : voici un chunk (chiffre E2E).
    ChunkData {
        /// UUID du fichier.
        file_id: String,
        /// Indice du chunk dans le fichier.
        chunk_index: u32,
        /// Donnees du chunk (chiffrees E2E).
        data: Vec<u8>,
        /// SHA-256 du chunk en clair (pour verification apres dechiffrement).
        checksum: String,
    },
    /// Notification : "j'ai modifie ce fichier".
    FileChanged {
        /// UUID du fichier modifie.
        file_id: String,
        /// Horloge vectorielle du fichier.
        clock: VectorClock,
        /// SHA-256 du contenu complet.
        checksum: String,
        /// Taille en octets.
        size_bytes: u64,
    },
    /// Acquittement : "j'ai bien recu".
    Ack {
        /// Identifiant du message acquitte.
        message_id: String,
    },
}

/// Entree du manifeste sync : etat d'un fichier vu par un noeud.
///
/// Le manifeste est la liste de tous les fichiers connus par un noeud,
/// avec leur horloge vectorielle et leur checksum. La comparaison de
/// deux manifestes permet de determiner quels fichiers doivent etre
/// transferes dans chaque direction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// UUID du fichier.
    pub file_id: String,
    /// Nom du fichier.
    pub name: String,
    /// UUID du dossier parent (None = racine).
    pub parent_id: Option<String>,
    /// SHA-256 du contenu original.
    pub checksum_sha256: String,
    /// Taille en octets.
    pub size_bytes: u64,
    /// Horloge vectorielle du fichier.
    pub clock: VectorClock,
    /// Date de derniere modification (ISO 8601).
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_message_serialize_roundtrip_manifest_request() {
        let msg = SyncMessage::ManifestRequest {
            sender_cog_id: "cog-abc".into(),
            folder_ids: vec!["folder-1".into(), "folder-2".into()],
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::ManifestRequest {
                sender_cog_id,
                folder_ids,
            } => {
                assert_eq!(sender_cog_id, "cog-abc");
                assert_eq!(folder_ids.len(), 2);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_sync_message_serialize_roundtrip_manifest_response() {
        let msg = SyncMessage::ManifestResponse {
            entries: vec![ManifestEntry {
                file_id: "file-1".into(),
                name: "photo.jpg".into(),
                parent_id: None,
                checksum_sha256: "abc123".into(),
                size_bytes: 1024,
                clock: VectorClock::from_entries(vec![("node-a".into(), 1)]),
                updated_at: "2026-03-01T00:00:00Z".into(),
            }],
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::ManifestResponse { entries } => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].file_id, "file-1");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_sync_message_serialize_roundtrip_chunk_request() {
        let msg = SyncMessage::ChunkRequest {
            file_id: "file-1".into(),
            chunk_indices: vec![0, 1, 2],
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::ChunkRequest {
                file_id,
                chunk_indices,
            } => {
                assert_eq!(file_id, "file-1");
                assert_eq!(chunk_indices, vec![0, 1, 2]);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_sync_message_serialize_roundtrip_chunk_data() {
        let msg = SyncMessage::ChunkData {
            file_id: "file-1".into(),
            chunk_index: 0,
            data: vec![1, 2, 3, 4],
            checksum: "sha256-hash".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::ChunkData {
                file_id,
                chunk_index,
                data,
                checksum,
            } => {
                assert_eq!(file_id, "file-1");
                assert_eq!(chunk_index, 0);
                assert_eq!(data, vec![1, 2, 3, 4]);
                assert_eq!(checksum, "sha256-hash");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_sync_message_serialize_roundtrip_file_changed() {
        let msg = SyncMessage::FileChanged {
            file_id: "file-1".into(),
            clock: VectorClock::from_entries(vec![("node-a".into(), 3)]),
            checksum: "sha256-xyz".into(),
            size_bytes: 2048,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::FileChanged {
                file_id,
                clock,
                checksum,
                size_bytes,
            } => {
                assert_eq!(file_id, "file-1");
                assert_eq!(clock.get("node-a"), 3);
                assert_eq!(checksum, "sha256-xyz");
                assert_eq!(size_bytes, 2048);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_sync_message_serialize_roundtrip_ack() {
        let msg = SyncMessage::Ack {
            message_id: "msg-42".into(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: SyncMessage = serde_json::from_str(&json).unwrap();
        match restored {
            SyncMessage::Ack { message_id } => {
                assert_eq!(message_id, "msg-42");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_manifest_entry_serialize() {
        let entry = ManifestEntry {
            file_id: "file-42".into(),
            name: "doc.pdf".into(),
            parent_id: Some("folder-1".into()),
            checksum_sha256: "deadbeef".into(),
            size_bytes: 65536,
            clock: VectorClock::from_entries(vec![("node-a".into(), 2), ("node-b".into(), 1)]),
            updated_at: "2026-03-01T12:00:00Z".into(),
        };
        let json = serde_json::to_string(&entry).unwrap();
        let restored: ManifestEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(entry, restored);
    }
}
