// @id: MGE-Net-Tests @do: testing @role: back-end @layer: 2 @human: miyuk
//! Unit tests for mge-net types and codec.

#[cfg(test)]
mod tests {
    use crate::client_id::ClientId;
    use crate::codec::FrameCodec;
    use crate::config::ServerConfig;
    use crate::packet::{GameMessage, Packet};

    // ── ClientId ───────────────────────────────────────────────

    #[test]
    fn client_id_new_is_unique() {
        let a = ClientId::new();
        let b = ClientId::new();
        assert_ne!(a, b, "Two fresh ClientId values must differ");
    }

    #[test]
    fn client_id_display() {
        let id = ClientId::new();
        let text = format!("{id}");
        assert!(
            text.starts_with("Client("),
            "Display should start with 'Client('"
        );
        assert!(text.ends_with(')'), "Display should end with ')'");
    }

    // ── Packet ─────────────────────────────────────────────────

    #[test]
    fn packet_new_has_unique_id() {
        let cid = ClientId::new();
        let p1 = Packet::new(cid, String::from("hello"));
        let p2 = Packet::new(cid, String::from("world"));
        assert_ne!(p1.id, p2.id, "Each packet must have a unique UUID");
    }

    #[test]
    fn packet_serialize_deserialize() {
        let cid = ClientId::new();
        let msg = GameMessage::Ping {
            timestamp_ms: 1_234_567,
        };
        let pkt = Packet::new(cid, msg);

        let json = serde_json::to_string(&pkt).unwrap();
        let restored: Packet<GameMessage> = serde_json::from_str(&json).unwrap();

        assert_eq!(pkt.id, restored.id);
        assert_eq!(pkt.client_id, restored.client_id);
    }

    // ── FrameCodec ─────────────────────────────────────────────

    #[test]
    fn frame_codec_encode_decode_roundtrip() {
        let codec = FrameCodec::default();
        let data = b"hello, sodomight!";
        let frame = FrameCodec::encode(data);

        let result = codec.decode(&frame).unwrap();
        assert!(result.is_some());
        let (payload, consumed) = result.unwrap();
        assert_eq!(payload, data);
        assert_eq!(consumed, 4 + data.len());
    }

    #[test]
    fn frame_codec_decode_incomplete_buffer() {
        let codec = FrameCodec::default();
        // Header says 100 bytes but we only supply 10 total
        let mut buf = Vec::new();
        buf.extend_from_slice(&100_u32.to_le_bytes());
        buf.extend_from_slice(&[0u8; 6]);
        let result = codec.decode(&buf).unwrap();
        assert!(result.is_none(), "Incomplete buffer should return None");
    }

    #[test]
    fn frame_codec_decode_empty_buf() {
        let codec = FrameCodec::default();
        let result = codec.decode(&[]).unwrap();
        assert!(result.is_none(), "Empty buffer should return None");
    }

    #[test]
    fn frame_codec_decode_exact_boundary() {
        let codec = FrameCodec::default();
        let data = b"exact";
        let frame = FrameCodec::encode(data);
        // The frame is exactly [4 byte header + 5 byte payload]
        assert_eq!(frame.len(), 9);

        let (payload, consumed) = codec.decode(&frame).unwrap().unwrap();
        assert_eq!(payload, data);
        assert_eq!(consumed, 9);
    }

    #[test]
    fn frame_codec_message_too_large() {
        let codec = FrameCodec::new(16); // max 16 bytes
        // Header declares 1000 bytes
        let mut buf = Vec::new();
        buf.extend_from_slice(&1000_u32.to_le_bytes());
        buf.extend_from_slice(&[0u8; 1000]);

        let err = codec.decode(&buf).unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("too large"),
            "Error should mention 'too large': {msg}"
        );
    }

    // ── GameMessage ────────────────────────────────────────────

    #[test]
    fn game_message_player_move_serialize() {
        let msg = GameMessage::PlayerMove { dx: 1.5, dy: -0.5 };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: GameMessage = serde_json::from_str(&json).unwrap();
        match restored {
            GameMessage::PlayerMove { dx, dy } => {
                assert!((dx - 1.5).abs() < f32::EPSILON);
                assert!((dy - (-0.5)).abs() < f32::EPSILON);
            }
            _ => panic!("Expected PlayerMove variant"),
        }
    }

    #[test]
    fn game_message_entity_update_serialize() {
        let msg = GameMessage::EntityUpdate {
            entity_id: 42,
            x: 10.0,
            y: 20.0,
            hp: 100,
        };
        let json = serde_json::to_string(&msg).unwrap();
        let restored: GameMessage = serde_json::from_str(&json).unwrap();
        match restored {
            GameMessage::EntityUpdate {
                entity_id,
                x,
                y,
                hp,
            } => {
                assert_eq!(entity_id, 42);
                assert!((x - 10.0).abs() < f32::EPSILON);
                assert!((y - 20.0).abs() < f32::EPSILON);
                assert_eq!(hp, 100);
            }
            _ => panic!("Expected EntityUpdate variant"),
        }
    }

    // ── ServerConfig ───────────────────────────────────────────

    #[test]
    fn server_config_default() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.bind_addr, "0.0.0.0:7777");
        assert_eq!(cfg.max_clients, 200);
        assert_eq!(cfg.max_message_size, 1_048_576);
        assert_eq!(cfg.tick_rate, 25);
    }

    // ── Multi-message ──────────────────────────────────────────

    #[test]
    fn frame_codec_encode_multiple_messages() {
        let codec = FrameCodec::default();

        let msg1 = b"alpha";
        let msg2 = b"bravo";
        let msg3 = b"charlie";

        // Concatenate three encoded frames into one buffer
        let mut stream = Vec::new();
        stream.extend_from_slice(&FrameCodec::encode(msg1));
        stream.extend_from_slice(&FrameCodec::encode(msg2));
        stream.extend_from_slice(&FrameCodec::encode(msg3));

        let mut offset = 0;

        // Decode message 1
        let (payload, consumed) = codec.decode(&stream[offset..]).unwrap().unwrap();
        assert_eq!(payload, msg1);
        offset += consumed;

        // Decode message 2
        let (payload, consumed) = codec.decode(&stream[offset..]).unwrap().unwrap();
        assert_eq!(payload, msg2);
        offset += consumed;

        // Decode message 3
        let (payload, consumed) = codec.decode(&stream[offset..]).unwrap().unwrap();
        assert_eq!(payload, msg3);
        offset += consumed;

        // Buffer fully consumed
        assert_eq!(offset, stream.len());
    }
}
