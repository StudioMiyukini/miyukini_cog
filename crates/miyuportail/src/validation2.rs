//! @id     miyuportail_validation2
//! @do     revalider_declaration_independamment
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — deuxième validation en zone COG, implémentation indépendante du décodeur du sas (EX-46).

//! **Décodeur n° 2.** Écrit séparément du décodeur n° 1 ([`crate::msce`]) :
//! découpe par tranches (`split_at`), table de signataires construite à
//! l'avance, schéma parcouru par itérateur fusionné. Seules les **règles**
//! (format MSCE, traité) sont communes ; aucune fonction de lecture ne l'est.
//!
//! La zone COG décode la trame d'origine avec ce validateur et compare le
//! résultat typé à la sortie du sas : toute divergence est un refus.

use ed25519_dalek::{Signature, VerifyingKey};

use crate::erreurs::RefusDecodage;
use crate::msce::{Ancres, Champ, Contexte, Declaration, Kid, Valeur};
use crate::traite::{IdService, NatureChamp, Sens, TypeDeclaration, MAX_ID_SERVICE, TAILLE_MAX_CHARGE};

const ENTETE_ATTENDUE: [u8; 7] = [b'M', b'S', b'C', b'E', 0x02, 0x01, 0x08];
const DOMAINE_V2: &[u8] = b"miyukini/sceau/v1.1\x00";
const LONGUEUR_ENTREE: usize = 99;
const TRAME_MAX: usize = 11 + TAILLE_MAX_CHARGE + 1 + 8 * LONGUEUR_ENTREE;

/// Coupe `n` octets en tête ; refus si insuffisant.
fn couper(t: &[u8], n: usize) -> Result<(&[u8], &[u8]), RefusDecodage> {
    if t.len() < n {
        Err(RefusDecodage::Taille)
    } else {
        Ok(t.split_at(n))
    }
}

fn entier(t: &[u8]) -> Result<u64, RefusDecodage> {
    if t.len() != 8 {
        return Err(RefusDecodage::Valeur);
    }
    Ok(t.iter().fold(0u64, |acc, b| (acc << 8) | u64::from(*b)))
}

fn longueur32(t: &[u8]) -> usize {
    t.iter().fold(0usize, |acc, b| (acc << 8) | usize::from(*b))
}

/// Itère un TLV canonique (tags strictement croissants).
fn tlv_v2(mut t: &[u8]) -> Result<Vec<(u8, &[u8])>, RefusDecodage> {
    let mut v: Vec<(u8, &[u8])> = Vec::new();
    while let Some((&tag, reste)) = t.split_first() {
        if let Some(&(dernier, _)) = v.last() {
            if tag <= dernier {
                return Err(RefusDecodage::Tlv);
            }
        }
        let (lg, reste) = couper(reste, 4).map_err(|_| RefusDecodage::Tlv)?;
        let (val, reste) = couper(reste, longueur32(lg)).map_err(|_| RefusDecodage::Tlv)?;
        v.push((tag, val));
        t = reste;
    }
    Ok(v)
}

fn texte_v2(t: &[u8]) -> Result<String, RefusDecodage> {
    let s = String::from_utf8(t.to_vec()).map_err(|_| RefusDecodage::Valeur)?;
    for c in s.chars() {
        let interdit = matches!(c, '\u{0}'..='\u{8}' | '\u{b}'..='\u{1f}' | '\u{7f}'..='\u{9f}');
        if interdit {
            return Err(RefusDecodage::Valeur);
        }
    }
    Ok(s)
}

fn valeur_v2(nature: NatureChamp, v: &[u8]) -> Result<Valeur, RefusDecodage> {
    Ok(match nature {
        NatureChamp::Entier => Valeur::Entier(entier(v)?),
        NatureChamp::Booleen => {
            if v.len() != 1 || v[0] > 1 {
                return Err(RefusDecodage::Valeur);
            }
            Valeur::Booleen(v[0] == 1)
        }
        NatureChamp::Texte { max } if v.len() <= max as usize => Valeur::Texte(texte_v2(v)?),
        NatureChamp::Octets { max } if v.len() <= max as usize => Valeur::Octets(v.to_vec()),
        NatureChamp::Document { max, .. } if v.len() <= max as usize => Valeur::Document(v.to_vec()),
        _ => return Err(RefusDecodage::Champ),
    })
}

fn champs_v2(t: &[u8], ty: &TypeDeclaration) -> Result<Vec<Champ>, RefusDecodage> {
    let recus = tlv_v2(t)?;
    let mut sortie = Vec::with_capacity(recus.len());
    let mut schema = ty.champs.iter().peekable();
    for (tag, v) in recus {
        // Avance dans le schéma (tags croissants des deux côtés) ; un champ
        // obligatoire sauté est un refus.
        loop {
            let s = schema.next().ok_or(RefusDecodage::Champ)?;
            if s.tag == tag {
                sortie.push(Champ { tag, valeur: valeur_v2(s.nature, v)? });
                break;
            }
            if s.tag > tag || s.obligatoire {
                return Err(RefusDecodage::Champ);
            }
        }
    }
    if schema.any(|s| s.obligatoire) {
        return Err(RefusDecodage::Champ);
    }
    Ok(sortie)
}

fn table_signataires(ancres: &Ancres) -> Result<Vec<(Kid, VerifyingKey, bool)>, RefusDecodage> {
    let mut table = Vec::with_capacity(1 + ancres.auteurs.len());
    let vk = VerifyingKey::from_bytes(&ancres.emetteur).map_err(|_| RefusDecodage::Signataires)?;
    table.push((crate::msce::kid(&ancres.emetteur), vk, true));
    for a in &ancres.auteurs {
        let vk = VerifyingKey::from_bytes(a).map_err(|_| RefusDecodage::Signataires)?;
        table.push((crate::msce::kid(a), vk, false));
    }
    Ok(table)
}

/// Deuxième validation complète d'une trame (même contrat que le décodeur n° 1).
pub fn valider(trame: &[u8], ctx: &Contexte<'_>) -> Result<Declaration, RefusDecodage> {
    if trame.len() > TRAME_MAX {
        return Err(RefusDecodage::Taille);
    }
    let (entete, reste) = couper(trame, 7)?;
    if entete != ENTETE_ATTENDUE {
        // Diagnostic par champ, dans l'ordre de l'entête.
        return Err(match entete {
            [m0, m1, m2, m3, ..] if [*m0, *m1, *m2, *m3] != *b"MSCE" => RefusDecodage::Magic,
            [_, _, _, _, v, ..] if *v != 0x02 => RefusDecodage::Version,
            [_, _, _, _, _, a, _] if *a != 0x01 => RefusDecodage::Algorithme,
            _ => RefusDecodage::TypeTrame,
        });
    }
    let (lg, reste) = couper(reste, 4)?;
    let lg = longueur32(lg);
    if lg > TAILLE_MAX_CHARGE {
        return Err(RefusDecodage::Taille);
    }
    let (charge, reste) = couper(reste, lg)?;
    let (&nombre, mut bloc) = reste.split_first().ok_or(RefusDecodage::Taille)?;
    if nombre == 0 || nombre > 8 {
        return Err(RefusDecodage::Signataires);
    }
    if bloc.len() != usize::from(nombre) * LONGUEUR_ENTREE {
        // Structure d'abord : la taille exacte est imposée par Ed25519.
        for _ in 0..nombre {
            let (e, r) = couper(bloc, 35)?;
            if e[32] != 0x01 {
                return Err(RefusDecodage::Algorithme);
            }
            if e[33..35] != [0, 64] {
                return Err(RefusDecodage::Taille);
            }
            bloc = couper(r, 64)?.1;
        }
        return Err(RefusDecodage::Taille);
    }

    // Signatures avant toute lecture de la charge.
    let attendu = if ctx.ancres.auteurs.is_empty() { 1 } else { 2 };
    let table = table_signataires(ctx.ancres)?;
    let mut message = Vec::with_capacity(DOMAINE_V2.len() + 7 + charge.len());
    message.extend_from_slice(DOMAINE_V2);
    message.extend_from_slice(&ENTETE_ATTENDUE[4..7]);
    message.extend_from_slice(&(lg as u32).to_be_bytes());
    message.extend_from_slice(charge);
    let mut vus: Vec<Kid> = Vec::new();
    let mut emetteur = false;
    let (entrees, _) = bloc.as_chunks::<LONGUEUR_ENTREE>();
    for entree in entrees {
        let (k, r) = entree.split_at(32);
        if r[0] != 0x01 {
            return Err(RefusDecodage::Algorithme);
        }
        if r[1..3] != [0, 64] {
            return Err(RefusDecodage::Taille);
        }
        let mut kid = [0u8; 32];
        kid.copy_from_slice(k);
        if vus.last().is_some_and(|p| *p >= kid) {
            return Err(RefusDecodage::Signataires);
        }
        let (_, vk, est_emetteur) = table.iter().find(|(t, _, _)| *t == kid).ok_or(RefusDecodage::Signataires)?;
        let mut sig = [0u8; 64];
        sig.copy_from_slice(&r[3..]);
        vk.verify_strict(&message, &Signature::from_bytes(&sig)).map_err(|_| RefusDecodage::Signature)?;
        emetteur |= *est_emetteur;
        vus.push(kid);
    }
    if vus.len() != attendu || !emetteur {
        return Err(RefusDecodage::Signataires);
    }

    // Charge.
    let entrees = tlv_v2(charge)?;
    let mut iter = entrees.into_iter();
    let mut suivant = |attendu_tag: u8| -> Result<&[u8], RefusDecodage> {
        match iter.next() {
            Some((t, v)) if t == attendu_tag => Ok(v),
            _ => Err(RefusDecodage::Tlv),
        }
    };
    let service_brut = suivant(1)?;
    let sens_brut = suivant(2)?;
    let version_brut = suivant(3)?;
    let type_brut = suivant(4)?;
    let sequence_brut = suivant(5)?;
    let emise_brut = suivant(6)?;
    let champs_brut = suivant(7)?;
    if iter.next().is_some() {
        return Err(RefusDecodage::Tlv);
    }

    if service_brut.is_empty() || service_brut.len() > MAX_ID_SERVICE {
        return Err(RefusDecodage::Valeur);
    }
    let service = IdService::new(&texte_v2(service_brut)?).map_err(|_| RefusDecodage::Valeur)?;
    if service.as_str() != ctx.service.as_str() {
        return Err(RefusDecodage::Service);
    }
    let sens = match sens_brut {
        [0x01] => Sens::Entrante,
        [0x02] => Sens::Sortante,
        _ => return Err(RefusDecodage::Valeur),
    };
    if sens != ctx.sens {
        return Err(RefusDecodage::Sens);
    }
    let version_traite = entier(version_brut)?;
    if version_traite != ctx.traite.version {
        return Err(RefusDecodage::VersionTraite);
    }
    let type_code = entier(type_brut)?;
    let ty = ctx
        .traite
        .types
        .iter()
        .find(|t| t.code == type_code && t.sens == sens)
        .ok_or(RefusDecodage::TypeDeclaration)?;
    let sequence = entier(sequence_brut)?;
    let emise_le = entier(emise_brut)?;
    let champs = champs_v2(champs_brut, ty)?;
    Ok(Declaration { service, sens, version_traite, type_code, sequence, emise_le, champs, signataires: vus })
}
