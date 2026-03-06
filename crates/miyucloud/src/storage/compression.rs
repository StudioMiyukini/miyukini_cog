use crate::errors::MiyucloudError;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};

/// Minimum size (bytes) before compression is attempted
const COMPRESSION_THRESHOLD: usize = 4096;

/// Compress data using gzip if it exceeds the threshold
pub fn maybe_compress(data: &[u8]) -> Result<(Vec<u8>, bool), MiyucloudError> {
    if data.len() < COMPRESSION_THRESHOLD {
        return Ok((data.to_vec(), false));
    }

    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(data).map_err(|e| {
        MiyucloudError::Internal(format!("Compression failed: {e}"))
    })?;
    let compressed = encoder.finish().map_err(|e| {
        MiyucloudError::Internal(format!("Compression finalize failed: {e}"))
    })?;

    // Only use compressed version if it's actually smaller
    if compressed.len() < data.len() {
        Ok((compressed, true))
    } else {
        Ok((data.to_vec(), false))
    }
}

/// Decompress gzip-compressed data
pub fn decompress(data: &[u8]) -> Result<Vec<u8>, MiyucloudError> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        MiyucloudError::Internal(format!("Decompression failed: {e}"))
    })?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_threshold_not_compressed() {
        let data = b"small data";
        let (result, compressed) = maybe_compress(data).unwrap();
        assert!(!compressed);
        assert_eq!(result, data);
    }

    #[test]
    fn test_roundtrip_compression() {
        // Create data above threshold with high compressibility
        let data: Vec<u8> = (0..8192).map(|i| (i % 256) as u8).collect();
        let (compressed_data, was_compressed) = maybe_compress(&data).unwrap();

        if was_compressed {
            let decompressed = decompress(&compressed_data).unwrap();
            assert_eq!(decompressed, data);
        }
    }
}
