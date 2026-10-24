//! @id     miyuracine_gf256
//! @do     calculer_corps_gf256
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « gf256 ».

//! Arithmétique du corps GF(256) de SLIP-39 : représentation polynomiale,
//! réduction par le polynôme de Rijndael x⁸ + x⁴ + x³ + x + 1 (FIPS 197 §4.2).
//!
//! Multiplication sans table ni branche dépendant des données (masques),
//! pour ne pas exposer de motif d'accès mémoire lié aux secrets.

/// Addition (= soustraction) dans GF(256).
#[inline]
pub(crate) const fn ajouter(a: u8, b: u8) -> u8 {
    a ^ b
}

/// Multiplication dans GF(256), temps constant.
pub(crate) const fn multiplier(a: u8, b: u8) -> u8 {
    let mut a = a;
    let mut b = b;
    let mut produit = 0u8;
    let mut tour = 0;
    while tour < 8 {
        // Masque 0xFF si le bit de poids faible de b vaut 1, sinon 0x00.
        produit ^= a & 0u8.wrapping_sub(b & 1);
        // Réduction modulo 0x11B si le bit de poids fort de a vaut 1.
        let reduction = 0u8.wrapping_sub(a >> 7);
        a = (a << 1) ^ (0x1b & reduction);
        b >>= 1;
        tour += 1;
    }
    produit
}

/// Inverse multiplicatif : a²⁵⁴ (a ≠ 0). L'exposant est public : la suite
/// d'opérations est fixe. `inverser(0)` vaut 0 (jamais utilisé en division :
/// les abscisses sont deux à deux distinctes).
pub(crate) const fn inverser(a: u8) -> u8 {
    let mut resultat = 1u8;
    let mut base = a;
    let mut exposant: u8 = 254;
    let mut tour = 0;
    while tour < 8 {
        if exposant & 1 == 1 {
            resultat = multiplier(resultat, base);
        }
        base = multiplier(base, base);
        exposant >>= 1;
        tour += 1;
    }
    resultat
}

/// Division a / b (b ≠ 0).
#[inline]
pub(crate) const fn diviser(a: u8, b: u8) -> u8 {
    multiplier(a, inverser(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Multiplication de référence (FIPS 197 §4.2.1, « xtime »), avec branches.
    fn multiplier_reference(mut a: u8, mut b: u8) -> u8 {
        let mut p = 0u8;
        while b != 0 {
            if b & 1 == 1 {
                p ^= a;
            }
            let haut = a & 0x80 != 0;
            a <<= 1;
            if haut {
                a ^= 0x1b;
            }
            b >>= 1;
        }
        p
    }

    #[test]
    fn exemple_fips_197() {
        // FIPS 197 §4.2 : {57} • {83} = {c1} ; {57} • {13} = {fe}.
        assert_eq!(multiplier(0x57, 0x83), 0xc1);
        assert_eq!(multiplier(0x57, 0x13), 0xfe);
    }

    #[test]
    fn multiplication_conforme_a_la_reference() {
        for a in 0..=255u8 {
            for b in 0..=255u8 {
                assert_eq!(multiplier(a, b), multiplier_reference(a, b));
            }
        }
    }

    #[test]
    fn inverse_de_tout_element_non_nul() {
        for a in 1..=255u8 {
            assert_eq!(multiplier(a, inverser(a)), 1, "a = {a}");
            assert_eq!(diviser(multiplier(a, 0x53), a), 0x53);
        }
        assert_eq!(ajouter(0x53, 0x53), 0);
    }
}
