//! Comparaisons constant-time pour tokens et secrets.
//!
//! @id: miyucloud_utils_constant_time
//! @do: provide_constant_time_comparison_via_subtle
//! @role: security
//! @layer: infra

use subtle::ConstantTimeEq;

/// Compare deux slices en temps constant (pas de short-circuit).
///
/// Utilise `subtle::ConstantTimeEq` pour eviter les timing attacks.
/// Retourne `false` si les longueurs different.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_time_eq_equal() {
        let a = b"hello world secret token";
        let b = b"hello world secret token";
        assert!(constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different() {
        let a = b"hello world secret token";
        let b = b"hello world wrong! token";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_different_length() {
        let a = b"short";
        let b = b"longer string";
        assert!(!constant_time_eq(a, b));
    }

    #[test]
    fn test_constant_time_eq_empty() {
        assert!(constant_time_eq(b"", b""));
    }

    #[test]
    fn test_constant_time_eq_single_bit_diff() {
        let a = [0x00u8];
        let b = [0x01u8];
        assert!(!constant_time_eq(&a, &b));
    }
}
