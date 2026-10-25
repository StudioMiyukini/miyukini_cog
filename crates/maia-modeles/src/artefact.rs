//! @id     maia_modeles_artefact
//! @do     coder_artefact_modele_canal
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — artefact « modèle » MMIA du canal officiel : encodage et décodage strict, sans effet de bord.

//! Artefact « modèle » du canal officiel (non spécifié par la doc gelée, à
//! amender) :
//!
//! ```text
//! "MMIA" | version 0x01 | longueur du manifeste (4, BE, 1..=128 Kio)
//!        | manifeste signé (Sceau ManifesteModele) | octets du modèle (≥ 1)
//! ```
//!
//! Le décodage est **strict** et ne vérifie rien d'autre que la forme : la
//! signature du manifeste et l'empreinte du modèle relèvent de
//! [`crate::manifeste`].

/// Magic de l'artefact « modèle » du canal officiel.
pub const MAGIC_ARTEFACT: &[u8; 4] = b"MMIA";
/// Version de l'artefact.
pub const VERSION_ARTEFACT: u8 = 0x01;
/// Taille maximale d'un manifeste (embarqué ou lu sur disque, C-13).
pub const TAILLE_MAX_MANIFESTE: usize = 128 * 1024;

const ENTETE: usize = 9;

/// Encode un artefact « modèle » (outillage, tests). Un manifeste de plus de
/// `u32::MAX` octets produit un artefact que [`decoder_artefact`] refuse.
pub fn encoder_artefact(manifeste: &[u8], modele: &[u8]) -> Vec<u8> {
    let longueur = u32::try_from(manifeste.len()).unwrap_or(u32::MAX);
    let mut s = Vec::with_capacity(ENTETE + manifeste.len() + modele.len());
    s.extend_from_slice(MAGIC_ARTEFACT);
    s.push(VERSION_ARTEFACT);
    s.extend_from_slice(&longueur.to_be_bytes());
    s.extend_from_slice(manifeste);
    s.extend_from_slice(modele);
    s
}

/// Décodage strict : `(manifeste, modèle)`.
pub fn decoder_artefact(octets: &[u8]) -> Result<(&[u8], &[u8]), String> {
    let refus = || "artefact modèle mal formé".to_string();
    let Some(&[m0, m1, m2, m3, version, l0, l1, l2, l3]) = octets.get(..ENTETE) else {
        return Err(refus());
    };
    let reste = octets.get(ENTETE..).ok_or_else(refus)?;
    if [m0, m1, m2, m3] != *MAGIC_ARTEFACT || version != VERSION_ARTEFACT {
        return Err(refus());
    }
    let n = usize::try_from(u32::from_be_bytes([l0, l1, l2, l3])).map_err(|_| refus())?;
    if n == 0 || n > TAILLE_MAX_MANIFESTE {
        return Err(refus());
    }
    let manifeste = reste.get(..n).ok_or_else(refus)?;
    let modele = reste.get(n..).ok_or_else(refus)?;
    if modele.is_empty() {
        return Err(refus());
    }
    Ok((manifeste, modele))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour() {
        let a = encoder_artefact(b"manifeste", b"modele");
        assert_eq!(
            decoder_artefact(&a).unwrap(),
            (&b"manifeste"[..], &b"modele"[..])
        );
    }

    #[test]
    fn formes_invalides_refusees() {
        let bon = encoder_artefact(b"m", b"x");
        assert!(decoder_artefact(&[]).is_err());
        assert!(decoder_artefact(b"MMIA\x01").is_err());
        assert!(
            decoder_artefact(&bon[..bon.len() - 1]).is_err(),
            "modèle vide"
        );
        let mut magic = bon.clone();
        magic[0] = b'X';
        assert!(decoder_artefact(&magic).is_err());
        let mut version = bon.clone();
        version[4] = 2;
        assert!(decoder_artefact(&version).is_err());
        assert!(
            decoder_artefact(&encoder_artefact(b"", b"x")).is_err(),
            "manifeste vide"
        );
        let gros = vec![0u8; TAILLE_MAX_MANIFESTE + 1];
        assert!(decoder_artefact(&encoder_artefact(&gros, b"x")).is_err());
        let mut long = bon;
        long[8] = 0xFF;
        assert!(
            decoder_artefact(&long).is_err(),
            "longueur au-delà de l'artefact"
        );
    }
}
