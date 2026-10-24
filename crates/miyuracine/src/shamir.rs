//! @id     miyuracine_shamir
//! @do     partager_secret_shamir
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « shamir ».

//! Partage de Shamir octet par octet sur GF(256), conforme à SLIP-39
//! (sections « Polynomial interpolation », « Sharing a secret »).
//!
//! - Secret stocké en f(255), empreinte de contrôle en f(254).
//! - Empreinte D = HMAC-SHA256(clé = R, message = S)[0..4] ‖ R, R aléatoire.
//! - Toute valeur intermédiaire est effacée (`Zeroizing`).

use hmac::{Hmac, Mac};
use rand_core::{CryptoRng, RngCore};
use sha2::Sha256;
use zeroize::{Zeroize, Zeroizing};

use crate::erreurs::ErreurRacine;
use crate::gf256;

/// Abscisse du secret.
const INDICE_SECRET: u8 = 255;
/// Abscisse de l'empreinte.
const INDICE_EMPREINTE: u8 = 254;
/// Longueur de l'empreinte (octets).
const LONGUEUR_EMPREINTE: usize = 4;
/// Nombre maximal de parts (indices sur 4 bits).
pub(crate) const MAX_PARTS: u8 = 16;

/// Point (abscisse, vecteur de valeurs) d'un partage.
pub(crate) struct PartBrute {
    pub(crate) x: u8,
    pub(crate) y: Zeroizing<Vec<u8>>,
}

/// Valide la longueur d'un secret : ≥ 128 bits et multiple de 16 bits.
pub(crate) fn longueur_secret_valide(n: usize) -> bool {
    n >= 16 && n & 1 == 0
}

/// Interpolation de Lagrange en `x` sur les points donnés.
pub(crate) fn interpoler(
    points: &[(u8, &[u8])],
    x: u8,
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let Some(&(_, premier)) = points.first() else {
        return Err(ErreurRacine::AucunePart);
    };
    let n = premier.len();
    if points.iter().any(|(_, y)| y.len() != n) {
        return Err(ErreurRacine::LongueursDifferentes);
    }
    for (i, (xi, _)) in points.iter().enumerate() {
        if points[..i].iter().any(|(xj, _)| xj == xi) {
            return Err(ErreurRacine::IndicesDupliques);
        }
    }
    // Abscisse déjà connue : valeur directe (comme l'implémentation de référence).
    if let Some((_, y)) = points.iter().find(|(xi, _)| *xi == x) {
        return Ok(Zeroizing::new(y.to_vec()));
    }
    let mut resultat = Zeroizing::new(vec![0u8; n]);
    for (i, &(xi, yi)) in points.iter().enumerate() {
        // Base de Lagrange ℓ_i(x) = Π_{j≠i} (x − x_j) / (x_i − x_j).
        let mut base: u8 = 1;
        for (j, &(xj, _)) in points.iter().enumerate() {
            if i != j {
                base = gf256::multiplier(
                    base,
                    gf256::diviser(gf256::ajouter(x, xj), gf256::ajouter(xi, xj)),
                );
            }
        }
        for (r, &y) in resultat.iter_mut().zip(yi.iter()) {
            *r = gf256::ajouter(*r, gf256::multiplier(y, base));
        }
    }
    Ok(resultat)
}

fn empreinte(r: &[u8], secret: &[u8]) -> Result<[u8; LONGUEUR_EMPREINTE], ErreurRacine> {
    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(r)
        .map_err(|_| ErreurRacine::LongueurSecretInvalide)?;
    mac.update(secret);
    let mut sortie = mac.finalize().into_bytes();
    let mut d = [0u8; LONGUEUR_EMPREINTE];
    d.copy_from_slice(&sortie[..LONGUEUR_EMPREINTE]);
    sortie.as_mut_slice().zeroize();
    Ok(d)
}

fn alea<R: RngCore + CryptoRng>(rng: &mut R, n: usize) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let mut v = Zeroizing::new(vec![0u8; n]);
    rng.try_fill_bytes(&mut v)
        .map_err(|_| ErreurRacine::AleaIndisponible)?;
    Ok(v)
}

/// SplitSecret(T, N, S) : parts d'indices 0 … N − 1.
pub(crate) fn partager<R: RngCore + CryptoRng>(
    seuil: u8,
    nombre: u8,
    secret: &[u8],
    rng: &mut R,
) -> Result<Vec<PartBrute>, ErreurRacine> {
    if seuil == 0 || seuil > nombre || nombre > MAX_PARTS {
        return Err(ErreurRacine::ParametresInvalides("0 < seuil ≤ nombre ≤ 16"));
    }
    if !longueur_secret_valide(secret.len()) {
        return Err(ErreurRacine::LongueurSecretInvalide);
    }
    if seuil == 1 {
        return Ok((0..nombre)
            .map(|x| PartBrute {
                x,
                y: Zeroizing::new(secret.to_vec()),
            })
            .collect());
    }

    let n = secret.len();
    let partie_aleatoire = alea(rng, n - LONGUEUR_EMPREINTE)?;
    let mut d = Zeroizing::new(Vec::with_capacity(n));
    d.extend_from_slice(&empreinte(&partie_aleatoire, secret)?);
    d.extend_from_slice(&partie_aleatoire);

    let nombre_aleatoires = seuil - 2;
    let mut parts: Vec<PartBrute> = Vec::with_capacity(usize::from(nombre));
    for x in 0..nombre_aleatoires {
        parts.push(PartBrute {
            x,
            y: alea(rng, n)?,
        });
    }
    let mut base: Vec<(u8, &[u8])> = parts.iter().map(|p| (p.x, p.y.as_slice())).collect();
    base.push((INDICE_EMPREINTE, d.as_slice()));
    base.push((INDICE_SECRET, secret));
    let mut calculees = Vec::with_capacity(usize::from(nombre - nombre_aleatoires));
    for x in nombre_aleatoires..nombre {
        calculees.push(PartBrute {
            x,
            y: interpoler(&base, x)?,
        });
    }
    drop(base);
    parts.extend(calculees);
    Ok(parts)
}

/// RecoverSecret(T, parts) : contrôle l'empreinte (comparaison en temps constant).
pub(crate) fn recuperer(
    seuil: u8,
    parts: &[(u8, &[u8])],
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let Some(&(_, premier)) = parts.first() else {
        return Err(ErreurRacine::AucunePart);
    };
    if seuil == 1 {
        return Ok(Zeroizing::new(premier.to_vec()));
    }
    let secret = interpoler(parts, INDICE_SECRET)?;
    let d = interpoler(parts, INDICE_EMPREINTE)?;
    if d.len() < LONGUEUR_EMPREINTE {
        return Err(ErreurRacine::LongueurSecretInvalide);
    }
    let attendu = empreinte(&d[LONGUEUR_EMPREINTE..], &secret)?;
    let ecart = attendu
        .iter()
        .zip(d[..LONGUEUR_EMPREINTE].iter())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b));
    if ecart != 0 {
        return Err(ErreurRacine::EmpreinteInvalide);
    }
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    fn vue(parts: &[PartBrute]) -> Vec<(u8, &[u8])> {
        parts.iter().map(|p| (p.x, p.y.as_slice())).collect()
    }

    #[test]
    fn tout_sous_ensemble_au_seuil_reconstitue() {
        let secret: Vec<u8> = (0..32u8).collect();
        let parts = partager(3, 5, &secret, &mut OsRng).unwrap();
        let v = vue(&parts);
        for a in 0..5 {
            for b in (a + 1)..5 {
                for c in (b + 1)..5 {
                    let s = recuperer(3, &[v[a], v[b], v[c]]).unwrap();
                    assert_eq!(s.as_slice(), secret.as_slice());
                }
            }
        }
        // Sous le seuil : empreinte invalide (avec probabilité 1 − 2⁻³²).
        assert_eq!(
            recuperer(3, &[v[0], v[1]]).unwrap_err(),
            ErreurRacine::EmpreinteInvalide
        );
    }

    #[test]
    fn parametres_refuses() {
        let s = [0u8; 16];
        assert!(partager(0, 3, &s, &mut OsRng).is_err());
        assert!(partager(4, 3, &s, &mut OsRng).is_err());
        assert!(partager(2, 17, &s, &mut OsRng).is_err());
        assert!(partager(2, 3, &[0u8; 15], &mut OsRng).is_err());
        assert!(partager(2, 3, &[0u8; 17], &mut OsRng).is_err());
    }

    #[test]
    fn indices_dupliques_refuses() {
        let a = [1u8; 16];
        let b = [2u8; 16];
        assert_eq!(
            interpoler(&[(1, &a), (1, &b)], 255).unwrap_err(),
            ErreurRacine::IndicesDupliques
        );
    }
}
