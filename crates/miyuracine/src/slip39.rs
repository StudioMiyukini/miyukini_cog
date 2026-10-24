//! @id     miyuracine_slip39
//! @do     coder_parts_slip39
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « slip39 ».

//! Parts mnémoniques SLIP-39 (schéma à deux niveaux), implémentées strictement
//! d'après la spécification SatoshiLabs (statut « Final ») et validées par les
//! 45 vecteurs officiels (`tests/vecteurs_slip39.rs`).
//!
//! Format d'une part (gros-boutiste, mots de 10 bits) :
//!
//! | id | ext | e | GI | Gt | g | I | t | valeur + remplissage | RS1024 |
//! |----|-----|---|----|----|---|---|---|----------------------|--------|
//! | 15 | 1   | 4 | 4  | 4  | 4 | 4 | 4 | 8n + remplissage     | 30     |
//!
//! Combinaison : toute incohérence (somme de contrôle, identifiant, seuils,
//! nombre de parts, remplissage, empreinte) est un refus. Aucune correction
//! automatique n'est tentée.

use std::collections::BTreeMap;

use rand_core::{CryptoRng, OsRng, RngCore};
use zeroize::{Zeroize, Zeroizing};

use crate::chiffrement::{self, EXPOSANT_MAX};
use crate::erreurs::ErreurRacine;
use crate::mots;
use crate::rs1024;
use crate::shamir::{self, MAX_PARTS};

/// Nombre minimal de mots d'une part (128 bits de secret).
const MOTS_MIN: usize = 20;
/// Mots d'en-tête (id, ext, e, puis GI, Gt, g, I, t).
const MOTS_ENTETE: usize = 4;
/// Mots de somme de contrôle.
const MOTS_CONTROLE: usize = 3;

/// Partage de second niveau d'un groupe : `seuil` parts sur `nombre`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Groupe {
    /// Nombre de parts de membres nécessaires (T).
    pub seuil: u8,
    /// Nombre de parts de membres produites (N).
    pub nombre: u8,
}

/// Paramètres publics d'une part (ceux encodés dans ses premiers mots).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametresPart {
    /// Identifiant aléatoire commun (15 bits).
    pub identifiant: u16,
    /// Sauvegarde extensible (`ext` = 1).
    pub extensible: bool,
    /// Exposant d'itération (10 000 × 2^e itérations PBKDF2 au total).
    pub exposant: u8,
    /// Indice du groupe (x de la part de groupe).
    pub indice_groupe: u8,
    /// Seuil de groupes (GT, valeur réelle).
    pub seuil_groupes: u8,
    /// Nombre de groupes (G, valeur réelle).
    pub nombre_groupes: u8,
    /// Indice du membre (x de la part de membre).
    pub indice_membre: u8,
    /// Seuil de membres du groupe (T, valeur réelle).
    pub seuil_membres: u8,
    /// Longueur de la valeur partagée (octets).
    pub longueur_valeur: usize,
}

/// Part décodée. La valeur est effacée de la mémoire à la destruction.
pub(crate) struct Part {
    pub(crate) parametres: ParametresPart,
    pub(crate) valeur: Zeroizing<Vec<u8>>,
}

impl std::fmt::Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Part")
            .field("parametres", &self.parametres)
            .field("valeur", &"<masqué>")
            .finish()
    }
}

// ─── Conversion bits ↔ mots ────────────────────────────────────────────────

/// Octets → mots de 10 bits, remplissage à gauche par des zéros.
fn octets_vers_mots(octets: &[u8]) -> Zeroizing<Vec<u16>> {
    let bits = octets.len() * 8;
    let nombre_mots = bits.div_ceil(10);
    let mut accu: u32 = 0;
    let mut nbits: u32 = (nombre_mots * 10 - bits) as u32;
    let mut sortie = Zeroizing::new(Vec::with_capacity(nombre_mots));
    for &o in octets {
        accu = (accu << 8) | u32::from(o);
        nbits += 8;
        while nbits >= 10 {
            nbits -= 10;
            sortie.push(((accu >> nbits) & 0x3ff) as u16);
        }
        accu &= (1u32 << nbits) - 1;
    }
    accu.zeroize();
    sortie
}

/// Mots de 10 bits → octets ; le remplissage (≤ 8 bits) doit être nul.
fn mots_vers_octets(mots: &[u16]) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let bits_total = mots.len() * 10;
    let remplissage = bits_total % 16;
    if remplissage > 8 {
        return Err(ErreurRacine::LongueurPartInvalide);
    }
    let mut sortie = Zeroizing::new(Vec::with_capacity((bits_total - remplissage) / 8));
    let mut accu: u32 = 0;
    let mut nbits: u32 = 0;
    let mut a_retirer = remplissage as u32;
    for &m in mots {
        accu = (accu << 10) | u32::from(m);
        nbits += 10;
        if a_retirer > 0 {
            if accu >> (nbits - a_retirer) != 0 {
                accu.zeroize();
                return Err(ErreurRacine::RemplissageInvalide);
            }
            nbits -= a_retirer;
            accu &= (1u32 << nbits) - 1;
            a_retirer = 0;
        }
        while nbits >= 8 {
            nbits -= 8;
            sortie.push(((accu >> nbits) & 0xff) as u8);
        }
        accu &= (1u32 << nbits) - 1;
    }
    accu.zeroize();
    Ok(sortie)
}

// ─── Encodage d'une part ───────────────────────────────────────────────────

fn encoder_part(p: &ParametresPart, valeur: &[u8]) -> Zeroizing<String> {
    let id_exp =
        (u32::from(p.identifiant) << 5) | (u32::from(p.extensible) << 4) | u32::from(p.exposant);
    let params = (u32::from(p.indice_groupe) << 16)
        | (u32::from(p.seuil_groupes - 1) << 12)
        | (u32::from(p.nombre_groupes - 1) << 8)
        | (u32::from(p.indice_membre) << 4)
        | u32::from(p.seuil_membres - 1);
    let mut indices: Zeroizing<Vec<u16>> = Zeroizing::new(vec![
        ((id_exp >> 10) & 0x3ff) as u16,
        (id_exp & 0x3ff) as u16,
        ((params >> 10) & 0x3ff) as u16,
        (params & 0x3ff) as u16,
    ]);
    indices.extend_from_slice(&octets_vers_mots(valeur));
    let controle = rs1024::creer(p.extensible, &indices);
    indices.extend_from_slice(&controle);
    let mut texte = Zeroizing::new(String::with_capacity(indices.len() * 9));
    for (i, &m) in indices.iter().enumerate() {
        if i > 0 {
            texte.push(' ');
        }
        texte.push_str(mots::mot(m));
    }
    texte
}

/// Décode et contrôle une part (mots, longueur, somme de contrôle, remplissage).
pub(crate) fn decoder_part(texte: &str) -> Result<Part, ErreurRacine> {
    let indices: Zeroizing<Vec<u16>> = Zeroizing::new(
        texte
            .split_ascii_whitespace()
            .map(mots::indice)
            .collect::<Result<_, _>>()?,
    );
    if indices.len() < MOTS_MIN {
        return Err(ErreurRacine::LongueurPartInvalide);
    }
    let mots_valeur = &indices[MOTS_ENTETE..indices.len() - MOTS_CONTROLE];
    if (mots_valeur.len() * 10) % 16 > 8 {
        return Err(ErreurRacine::LongueurPartInvalide);
    }
    let id_exp = (u32::from(indices[0]) << 10) | u32::from(indices[1]);
    let extensible = (id_exp >> 4) & 1 == 1;
    if !rs1024::verifier(extensible, &indices) {
        return Err(ErreurRacine::SommeControleInvalide);
    }
    let params = (u32::from(indices[2]) << 10) | u32::from(indices[3]);
    let parametres_sans_longueur = ParametresPart {
        identifiant: (id_exp >> 5) as u16,
        extensible,
        exposant: (id_exp & 0xf) as u8,
        indice_groupe: ((params >> 16) & 0xf) as u8,
        seuil_groupes: ((params >> 12) & 0xf) as u8 + 1,
        nombre_groupes: ((params >> 8) & 0xf) as u8 + 1,
        indice_membre: ((params >> 4) & 0xf) as u8,
        seuil_membres: (params & 0xf) as u8 + 1,
        longueur_valeur: 0,
    };
    if parametres_sans_longueur.nombre_groupes < parametres_sans_longueur.seuil_groupes {
        return Err(ErreurRacine::SeuilGroupesSuperieurAuNombre);
    }
    let valeur = mots_vers_octets(mots_valeur)?;
    if !shamir::longueur_secret_valide(valeur.len()) {
        return Err(ErreurRacine::LongueurSecretInvalide);
    }
    Ok(Part {
        parametres: ParametresPart {
            longueur_valeur: valeur.len(),
            ..parametres_sans_longueur
        },
        valeur,
    })
}

/// Paramètres publics d'une part, après contrôle complet de sa forme.
pub fn parametres_part(texte: &str) -> Result<ParametresPart, ErreurRacine> {
    decoder_part(texte).map(|p| p.parametres)
}

// ─── Génération ────────────────────────────────────────────────────────────

/// GenerateShares(GT, groupes, MS, P, e) avec `ext` = 1 (spécification courante).
///
/// Rend, pour chaque groupe, ses parts de membres en mots (effacées à la destruction).
pub fn generer_parts(
    seuil_groupes: u8,
    groupes: &[Groupe],
    secret_maitre: &[u8],
    phrase: &[u8],
    exposant: u8,
) -> Result<Vec<Vec<Zeroizing<String>>>, ErreurRacine> {
    generer_parts_avec(
        seuil_groupes,
        groupes,
        secret_maitre,
        phrase,
        exposant,
        &mut OsRng,
    )
}

pub(crate) fn generer_parts_avec<R: RngCore + CryptoRng>(
    seuil_groupes: u8,
    groupes: &[Groupe],
    secret_maitre: &[u8],
    phrase: &[u8],
    exposant: u8,
    rng: &mut R,
) -> Result<Vec<Vec<Zeroizing<String>>>, ErreurRacine> {
    let nombre_groupes = u8::try_from(groupes.len())
        .map_err(|_| ErreurRacine::ParametresInvalides("au plus 16 groupes"))?;
    if nombre_groupes == 0 || nombre_groupes > MAX_PARTS {
        return Err(ErreurRacine::ParametresInvalides("1 à 16 groupes"));
    }
    if seuil_groupes == 0 || seuil_groupes > nombre_groupes {
        return Err(ErreurRacine::ParametresInvalides(
            "0 < seuil de groupes ≤ nombre de groupes",
        ));
    }
    for g in groupes {
        if g.seuil == 1 && g.nombre > 1 {
            return Err(ErreurRacine::ParametresInvalides(
                "seuil de membres 1 avec plusieurs parts",
            ));
        }
        if g.seuil == 0 || g.seuil > g.nombre || g.nombre > MAX_PARTS {
            return Err(ErreurRacine::ParametresInvalides(
                "0 < seuil de membres ≤ nombre ≤ 16",
            ));
        }
    }
    if !shamir::longueur_secret_valide(secret_maitre.len()) {
        return Err(ErreurRacine::LongueurSecretInvalide);
    }
    if exposant > EXPOSANT_MAX {
        return Err(ErreurRacine::ParametresInvalides(
            "exposant d'itération > 15",
        ));
    }
    chiffrement::verifier_phrase(phrase)?;

    let mut id = [0u8; 2];
    rng.try_fill_bytes(&mut id)
        .map_err(|_| ErreurRacine::AleaIndisponible)?;
    let identifiant = u16::from_be_bytes(id) & 0x7fff;
    let extensible = true;

    let chiffre = chiffrement::chiffrer(secret_maitre, phrase, exposant, identifiant, extensible)?;
    let parts_groupes = shamir::partager(seuil_groupes, nombre_groupes, &chiffre, rng)?;

    let mut sortie = Vec::with_capacity(groupes.len());
    for (groupe, part_groupe) in groupes.iter().zip(parts_groupes.iter()) {
        let membres = shamir::partager(groupe.seuil, groupe.nombre, &part_groupe.y, rng)?;
        let mut textes = Vec::with_capacity(membres.len());
        for membre in &membres {
            let p = ParametresPart {
                identifiant,
                extensible,
                exposant,
                indice_groupe: part_groupe.x,
                seuil_groupes,
                nombre_groupes,
                indice_membre: membre.x,
                seuil_membres: groupe.seuil,
                longueur_valeur: membre.y.len(),
            };
            textes.push(encoder_part(&p, &membre.y));
        }
        sortie.push(textes);
    }
    Ok(sortie)
}

// ─── Combinaison ───────────────────────────────────────────────────────────

/// Combine des parts décodées et rend le secret maître (fail-closed).
pub(crate) fn combiner_decodees(
    parts: &[Part],
    phrase: &[u8],
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let Some(premiere) = parts.first() else {
        return Err(ErreurRacine::AucunePart);
    };
    let c = premiere.parametres;
    for p in parts {
        let q = p.parametres;
        if (
            q.identifiant,
            q.extensible,
            q.exposant,
            q.seuil_groupes,
            q.nombre_groupes,
        ) != (
            c.identifiant,
            c.extensible,
            c.exposant,
            c.seuil_groupes,
            c.nombre_groupes,
        ) {
            return Err(ErreurRacine::PartsIncompatibles);
        }
        if q.longueur_valeur != c.longueur_valeur {
            return Err(ErreurRacine::LongueursDifferentes);
        }
    }

    let mut groupes: BTreeMap<u8, Vec<&Part>> = BTreeMap::new();
    for p in parts {
        groupes
            .entry(p.parametres.indice_groupe)
            .or_default()
            .push(p);
    }
    for membres in groupes.values() {
        let seuil = membres[0].parametres.seuil_membres;
        if membres.iter().any(|m| m.parametres.seuil_membres != seuil) {
            return Err(ErreurRacine::SeuilsMembresDifferents);
        }
        for (i, m) in membres.iter().enumerate() {
            if membres[..i]
                .iter()
                .any(|n| n.parametres.indice_membre == m.parametres.indice_membre)
            {
                return Err(ErreurRacine::IndicesDupliques);
            }
        }
    }
    if groupes.len() != usize::from(c.seuil_groupes) {
        return Err(ErreurRacine::NombreGroupesInvalide);
    }
    if groupes
        .values()
        .any(|m| m.len() != usize::from(m[0].parametres.seuil_membres))
    {
        return Err(ErreurRacine::NombrePartsInvalide);
    }

    let mut parts_groupes: Vec<(u8, Zeroizing<Vec<u8>>)> = Vec::with_capacity(groupes.len());
    for (&indice_groupe, membres) in &groupes {
        let points: Vec<(u8, &[u8])> = membres
            .iter()
            .map(|m| (m.parametres.indice_membre, m.valeur.as_slice()))
            .collect();
        parts_groupes.push((
            indice_groupe,
            shamir::recuperer(membres[0].parametres.seuil_membres, &points)?,
        ));
    }
    let points: Vec<(u8, &[u8])> = parts_groupes
        .iter()
        .map(|(x, y)| (*x, y.as_slice()))
        .collect();
    let chiffre = shamir::recuperer(c.seuil_groupes, &points)?;
    chiffrement::dechiffrer(&chiffre, phrase, c.exposant, c.identifiant, c.extensible)
}

/// Combine des parts mnémoniques SLIP-39 et rend le secret maître.
///
/// Refus (fail-closed) : somme de contrôle invalide, parts d'origines
/// différentes, nombre de groupes ou de parts différent du seuil, indices
/// dupliqués, remplissage non nul, empreinte invalide.
pub fn combiner_parts(parts: &[&str], phrase: &[u8]) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let decodees = parts
        .iter()
        .map(|t| decoder_part(t))
        .collect::<Result<Vec<_>, _>>()?;
    combiner_decodees(&decodees, phrase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_bits_aller_retour() {
        for n in [16usize, 18, 32, 64] {
            let octets: Vec<u8> = (0..n).map(|i| (i * 37 + 11) as u8).collect();
            let mots = octets_vers_mots(&octets);
            assert_eq!(mots.len(), (n * 8).div_ceil(10));
            assert_eq!(
                mots_vers_octets(&mots).unwrap().as_slice(),
                octets.as_slice()
            );
        }
    }

    #[test]
    fn deux_niveaux_aller_retour() {
        let ms: Vec<u8> = (100..132u8).collect();
        let groupes = [
            Groupe {
                seuil: 2,
                nombre: 3,
            },
            Groupe {
                seuil: 1,
                nombre: 1,
            },
            Groupe {
                seuil: 3,
                nombre: 5,
            },
        ];
        let parts = generer_parts(2, &groupes, &ms, b"", 0).unwrap();
        let a: Vec<&str> = vec![&parts[0][0], &parts[0][2], &parts[1][0]];
        assert_eq!(combiner_parts(&a, b"").unwrap().as_slice(), ms.as_slice());
        let b: Vec<&str> = vec![
            &parts[2][4],
            &parts[2][1],
            &parts[2][3],
            &parts[0][1],
            &parts[0][2],
        ];
        assert_eq!(combiner_parts(&b, b"").unwrap().as_slice(), ms.as_slice());
        // Trois groupes pour un seuil de deux : refus (nombre de groupes ≠ seuil).
        let c: Vec<&str> = vec![
            &parts[0][0],
            &parts[0][2],
            &parts[1][0],
            &parts[2][0],
            &parts[2][1],
            &parts[2][2],
        ];
        assert_eq!(
            combiner_parts(&c, b"").unwrap_err(),
            ErreurRacine::NombreGroupesInvalide
        );
    }

    #[test]
    fn generation_refuse_parametres_hors_specification() {
        let ms = [7u8; 32];
        let g = |seuil, nombre| Groupe { seuil, nombre };
        assert!(generer_parts(1, &[g(1, 2)], &ms, b"", 0).is_err());
        assert!(generer_parts(2, &[g(2, 3)], &ms, b"", 0).is_err());
        assert!(generer_parts(0, &[g(2, 3)], &ms, b"", 0).is_err());
        assert!(generer_parts(1, &[g(2, 17)], &ms, b"", 0).is_err());
        assert!(generer_parts(1, &[g(2, 3)], &ms, b"", 16).is_err());
        assert!(generer_parts(1, &[g(2, 3)], &ms[..15], b"", 0).is_err());
        assert!(generer_parts(1, &[], &ms, b"", 0).is_err());
    }
}
