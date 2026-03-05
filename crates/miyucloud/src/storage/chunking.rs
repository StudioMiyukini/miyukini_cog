//! Decoupe et reassemblage de fichiers en chunks.
//!
//! @id: miyucloud_storage_chunking
//! @do: split_and_reassemble_file_data_in_chunks
//! @role: storage
//! @layer: infra
//!
//! Taille de chunk par defaut : 256 KB. Configurable.
//! Le dernier chunk peut etre plus petit que la taille cible.

/// Taille de chunk par defaut : 256 KB.
pub const DEFAULT_CHUNK_SIZE: usize = 256 * 1024;

/// Utilitaire de decoupe et reassemblage de donnees en chunks.
pub struct Chunker {
    /// Taille maximale d'un chunk en octets.
    chunk_size: usize,
}

impl Chunker {
    /// Cree un chunker avec la taille de chunk par defaut (256 KB).
    pub fn new() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
        }
    }

    /// Cree un chunker avec une taille de chunk personnalisee.
    pub fn with_size(chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "chunk_size must be > 0");
        Self { chunk_size }
    }

    /// Retourne la taille de chunk configuree.
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Decoupe des donnees en chunks. Retourne un vecteur de chunks.
    /// Le dernier chunk peut etre plus petit que `chunk_size`.
    /// Si `data` est vide, retourne un seul chunk vide.
    pub fn split(&self, data: &[u8]) -> Vec<Vec<u8>> {
        if data.is_empty() {
            return vec![vec![]];
        }
        data.chunks(self.chunk_size).map(<[u8]>::to_vec).collect()
    }

    /// Reassemble des chunks en un seul buffer de donnees.
    pub fn reassemble(&self, chunks: &[Vec<u8>]) -> Vec<u8> {
        let total_len: usize = chunks.iter().map(Vec::len).sum();
        let mut result = Vec::with_capacity(total_len);
        for chunk in chunks {
            result.extend_from_slice(chunk);
        }
        result
    }
}

impl Default for Chunker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_small_file() {
        let chunker = Chunker::new();
        let data = vec![42u8; 100];
        let chunks = chunker.split(&data);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 100);
    }

    #[test]
    fn test_chunk_large_file() {
        let chunker = Chunker::with_size(256 * 1024);
        // 600 KB = > 256 KB, should produce 3 chunks (256 + 256 + 88 KB)
        let data = vec![0xABu8; 600 * 1024];
        let chunks = chunker.split(&data);
        assert_eq!(chunks.len(), 3);
        assert_eq!(chunks[0].len(), 256 * 1024);
        assert_eq!(chunks[1].len(), 256 * 1024);
        assert_eq!(chunks[2].len(), 600 * 1024 - 2 * 256 * 1024);
    }

    #[test]
    fn test_reassemble_chunks() {
        let chunker = Chunker::with_size(100);
        let original = (0..250).map(|i| (i % 256) as u8).collect::<Vec<u8>>();
        let chunks = chunker.split(&original);
        assert_eq!(chunks.len(), 3);
        let reassembled = chunker.reassemble(&chunks);
        assert_eq!(reassembled, original);
    }

    #[test]
    fn test_empty_data_produces_one_chunk() {
        let chunker = Chunker::new();
        let chunks = chunker.split(&[]);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].is_empty());
    }

    #[test]
    fn test_exact_chunk_size() {
        let chunker = Chunker::with_size(100);
        let data = vec![1u8; 200];
        let chunks = chunker.split(&data);
        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0].len(), 100);
        assert_eq!(chunks[1].len(), 100);
    }
}
