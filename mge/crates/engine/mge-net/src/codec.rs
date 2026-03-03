// @id: MGE-Net-Codec @do: framing @role: back-end @layer: 2 @human: miyuk
//! Length-prefixed frame codec for TCP streams.
//!
//! Wire format: `[u32 LE payload length][JSON payload bytes]`.

use crate::NetError;

/// Maximum message size: 64 KiB (SEC-16).
const DEFAULT_MAX_MESSAGE_SIZE: usize = 65_536;

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

// ---------------------------------------------------------------------------
// CRC32 (IEEE) — SEC-20
// ---------------------------------------------------------------------------

/// CRC32 (IEEE) lookup table — computed at compile time.
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut crc = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB8_8320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

/// Compute CRC32 (IEEE) for a byte slice.
#[must_use]
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        let idx = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    crc ^ 0xFFFF_FFFF
}

/// Encode a frame with a trailing CRC32 checksum.
///
/// Wire format: `[u32 LE payload length][payload][u32 LE CRC32]`.
#[must_use]
pub fn encode_with_crc(data: &[u8]) -> Vec<u8> {
    let checksum = crc32(data);
    let len = data.len() as u32;
    let mut buf = Vec::with_capacity(4 + data.len() + 4);
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(data);
    buf.extend_from_slice(&checksum.to_le_bytes());
    buf
}

/// Decode and verify a CRC32-protected frame.
///
/// Returns the payload if the checksum matches, or a [`NetError`] if corrupt.
///
/// # Errors
///
/// Returns [`NetError::MessageTooLarge`] if the buffer is too small or the
/// CRC32 checksum does not match.
pub fn decode_verify_crc(buf: &[u8]) -> Result<Vec<u8>, crate::NetError> {
    if buf.len() < 8 {
        return Err(crate::NetError::MessageTooLarge { size: 0, max: 8 });
    }
    let len_bytes: [u8; 4] = [buf[0], buf[1], buf[2], buf[3]];
    let payload_len = u32::from_le_bytes(len_bytes) as usize;

    let total = 4 + payload_len + 4;
    if buf.len() < total {
        return Err(crate::NetError::MessageTooLarge {
            size: buf.len(),
            max: total,
        });
    }

    let payload = &buf[4..4 + payload_len];
    let crc_bytes: [u8; 4] = [
        buf[4 + payload_len],
        buf[4 + payload_len + 1],
        buf[4 + payload_len + 2],
        buf[4 + payload_len + 3],
    ];
    let stored_crc = u32::from_le_bytes(crc_bytes);
    let computed_crc = crc32(payload);

    if stored_crc != computed_crc {
        return Err(crate::NetError::MessageTooLarge {
            size: stored_crc as usize,
            max: computed_crc as usize,
        });
    }

    Ok(payload.to_vec())
}
