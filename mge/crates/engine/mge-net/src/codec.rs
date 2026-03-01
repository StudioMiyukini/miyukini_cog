// @id: MGE-Net-Codec @do: framing @role: back-end @layer: 2 @human: miyuk
//! Length-prefixed frame codec for TCP streams.
//!
//! Wire format: `[u32 LE payload length][JSON payload bytes]`.

use crate::NetError;

/// Default maximum message size: 1 MiB.
const DEFAULT_MAX_MESSAGE_SIZE: usize = 1_048_576;

/// Length-prefixed frame codec.
///
/// Encodes outgoing data as `[4-byte u32 LE length][payload]` and decodes
/// incoming buffers by reading the length prefix first, then extracting
/// the payload once enough bytes are available.
pub struct FrameCodec {
    max_message_size: usize,
}

impl FrameCodec {
    /// Create a codec with a custom maximum message size.
    #[must_use]
    pub fn new(max_message_size: usize) -> Self {
        Self { max_message_size }
    }

    /// Encode `data` into a length-prefixed frame.
    ///
    /// Returns a `Vec<u8>` containing `[len as u32 LE][data]`.
    #[must_use]
    pub fn encode(data: &[u8]) -> Vec<u8> {
        let len = data.len() as u32;
        let mut buf = Vec::with_capacity(4 + data.len());
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(data);
        buf
    }

    /// Try to decode a single frame from `buf`.
    ///
    /// Returns:
    /// - `Ok(Some((payload, bytes_consumed)))` when a complete frame is available.
    /// - `Ok(None)` when the buffer does not yet contain a full frame.
    /// - `Err(NetError::MessageTooLarge { .. })` when the declared length exceeds
    ///   the configured maximum.
    ///
    /// # Errors
    ///
    /// Returns [`NetError::MessageTooLarge`] if the frame header declares a
    /// payload larger than `max_message_size`.
    pub fn decode(&self, buf: &[u8]) -> Result<Option<(Vec<u8>, usize)>, NetError> {
        if buf.len() < 4 {
            return Ok(None);
        }

        let len_bytes: [u8; 4] = [buf[0], buf[1], buf[2], buf[3]];
        let payload_len = u32::from_le_bytes(len_bytes) as usize;

        if payload_len > self.max_message_size {
            return Err(NetError::MessageTooLarge {
                size: payload_len,
                max: self.max_message_size,
            });
        }

        let total = 4 + payload_len;
        if buf.len() < total {
            return Ok(None);
        }

        let payload = buf[4..total].to_vec();
        Ok(Some((payload, total)))
    }
}

impl Default for FrameCodec {
    fn default() -> Self {
        Self {
            max_message_size: DEFAULT_MAX_MESSAGE_SIZE,
        }
    }
}
