//! @id     miyucanal_transport
//! @do     relever_depot_officiel
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — module « transport » (relevé du dépôt officiel : demandes, réponses, dépôt en mémoire).

//! Transport du canal officiel : la zone COG **relève** le dépôt officiel
//! (Franche) ; le dépôt ne pousse jamais rien et n'a aucune route vers la zone
//! COG (EX-20, INV-Z4, INV-Z7).
//!
//! - [`TransportCanal`] : une seule opération, `relever(demande) → octets`,
//!   initiée par la zone COG. Implémentation réelle à venir : voie réservée du
//!   Portail (`crates/miyuportail`, non branchée ici) ; implémentation de test :
//!   [`DepotMemoire`].
//! - Demandes **génériques** : identiques pour tous les COG, sans donnée
//!   locale autre que des positions publiques du registre officiel et des
//!   empreintes publiées (menaces §3 « requêtes génériques »).
//! - Le dépôt est **non fiable** : tout ce qu'il rend est revérifié (sceaux,
//!   racine, empreintes) par le relevé.
//!
//! # Encodages (big-endian, décodage strict, taille exacte)
//!
//! ```text
//! demande  : "MCAN" | version 0x01 | code (1) | champs
//!   0x01 tête          : —
//!   0x02 entrées       : depuis (8) | nombre (4, 1..=1024)
//!   0x03 artefact      : empreinte (32) | taille (8)
//! réponse tête     : sceau de la tête
//! réponse entrées  : nombre (4) | nombre × (longueur (4) | sceau d'entrée)
//! réponse artefact : octets bruts de l'artefact
//! ```

use std::collections::HashMap;

use miyusceau::TAILLE_MAX_SCEAU;
use sha2::{Digest, Sha256};

use crate::erreurs::{ErreurCanal, ErreurTransport};

const MAGIC: &[u8; 4] = b"MCAN";
const VERSION: u8 = 0x01;

/// Nombre maximal d'entrées par demande.
pub const MAX_ENTREES_PAR_DEMANDE: u32 = 1_024;

/// Demande de relevé adressée au dépôt officiel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Demande {
    /// Dernière tête signée du registre officiel.
    Tete,
    /// Entrées `[depuis, depuis + nombre)` du registre officiel.
    Entrees {
        /// Premier index.
        depuis: u64,
        /// Nombre d'entrées (1..=[`MAX_ENTREES_PAR_DEMANDE`]).
        nombre: u32,
    },
    /// Artefact désigné par son empreinte SHA-256 publiée.
    Artefact {
        /// Empreinte attendue.
        empreinte: [u8; 32],
        /// Taille annoncée (borne de lecture pour le transport).
        taille: u64,
    },
}

impl Demande {
    /// Encodage binaire (voir module).
    pub fn encoder(&self) -> Vec<u8> {
        let mut s = MAGIC.to_vec();
        s.push(VERSION);
        match self {
            Self::Tete => s.push(0x01),
            Self::Entrees { depuis, nombre } => {
                s.push(0x02);
                s.extend_from_slice(&depuis.to_be_bytes());
                s.extend_from_slice(&nombre.to_be_bytes());
            }
            Self::Artefact { empreinte, taille } => {
                s.push(0x03);
                s.extend_from_slice(empreinte);
                s.extend_from_slice(&taille.to_be_bytes());
            }
        }
        s
    }

    /// Décodage strict (côté dépôt).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurCanal> {
        let refus = ErreurCanal::ReponseInvalide;
        if octets.len() < 6 || &octets[0..4] != MAGIC || octets[4] != VERSION {
            return Err(refus);
        }
        let corps = &octets[6..];
        match (octets[5], corps.len()) {
            (0x01, 0) => Ok(Self::Tete),
            (0x02, 12) => {
                let depuis = u64::from_be_bytes(corps[0..8].try_into().map_err(|_| ErreurCanal::ReponseInvalide)?);
                let nombre = u32::from_be_bytes(corps[8..12].try_into().map_err(|_| ErreurCanal::ReponseInvalide)?);
                if nombre == 0 || nombre > MAX_ENTREES_PAR_DEMANDE {
                    return Err(refus);
                }
                Ok(Self::Entrees { depuis, nombre })
            }
            (0x03, 40) => {
                let mut empreinte = [0u8; 32];
                empreinte.copy_from_slice(&corps[0..32]);
                let taille = u64::from_be_bytes(corps[32..40].try_into().map_err(|_| ErreurCanal::ReponseInvalide)?);
                Ok(Self::Artefact { empreinte, taille })
            }
            _ => Err(refus),
        }
    }
}

/// Encode une réponse « entrées ».
pub fn encoder_entrees(entrees: &[Vec<u8>]) -> Vec<u8> {
    let mut s = Vec::with_capacity(4 + entrees.iter().map(|e| 4 + e.len()).sum::<usize>());
    s.extend_from_slice(&(entrees.len() as u32).to_be_bytes());
    for e in entrees {
        s.extend_from_slice(&(e.len() as u32).to_be_bytes());
        s.extend_from_slice(e);
    }
    s
}

/// Décode strictement une réponse « entrées » : exactement `attendues`
/// entrées, chacune non vide et ≤ taille maximale d'un sceau, aucun octet en trop.
pub fn decoder_entrees(octets: &[u8], attendues: u32) -> Result<Vec<Vec<u8>>, ErreurCanal> {
    let refus = || ErreurCanal::ReponseInvalide;
    let nombre = u32::from_be_bytes(octets.get(0..4).ok_or_else(refus)?.try_into().map_err(|_| refus())?);
    if nombre != attendues {
        return Err(refus());
    }
    let mut pos = 4usize;
    let mut entrees = Vec::with_capacity(nombre as usize);
    for _ in 0..nombre {
        let entete = octets.get(pos..pos + 4).ok_or_else(refus)?;
        let n = u32::from_be_bytes(entete.try_into().map_err(|_| refus())?) as usize;
        if n == 0 || n > TAILLE_MAX_SCEAU {
            return Err(refus());
        }
        let e = octets.get(pos + 4..pos + 4 + n).ok_or_else(refus)?;
        entrees.push(e.to_vec());
        pos += 4 + n;
    }
    if pos != octets.len() {
        return Err(refus());
    }
    Ok(entrees)
}

/// Relevé du dépôt officiel, initié par la zone COG.
pub trait TransportCanal {
    /// Relève l'objet désigné par `demande` (octets bruts, non fiables).
    fn relever(&mut self, demande: &Demande) -> Result<Vec<u8>, ErreurTransport>;
}

/// Dépôt officiel en mémoire (simulation de la Franche pour les tests).
///
/// **Non fiable par construction** : les méthodes `remplacer_*` permettent de
/// simuler un dépôt compromis.
#[derive(Debug, Default, Clone)]
pub struct DepotMemoire {
    tete: Option<Vec<u8>>,
    entrees: Vec<Vec<u8>>,
    artefacts: HashMap<[u8; 32], Vec<u8>>,
    demandes: Vec<Demande>,
}

impl DepotMemoire {
    /// Dépôt vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Dépose la dernière tête signée.
    pub fn deposer_tete(&mut self, sceau: Vec<u8>) {
        self.tete = Some(sceau);
    }

    /// Remplace toutes les entrées (copie du registre officiel).
    pub fn remplacer_entrees(&mut self, entrees: Vec<Vec<u8>>) {
        self.entrees = entrees;
    }

    /// Dépose un artefact sous son empreinte réelle.
    pub fn deposer_artefact(&mut self, octets: Vec<u8>) -> [u8; 32] {
        let empreinte: [u8; 32] = Sha256::digest(&octets).into();
        self.artefacts.insert(empreinte, octets);
        empreinte
    }

    /// Dépose des octets sous une empreinte arbitraire (dépôt compromis).
    pub fn remplacer_artefact(&mut self, empreinte: [u8; 32], octets: Vec<u8>) {
        self.artefacts.insert(empreinte, octets);
    }

    /// Demandes reçues (audit : aucune donnée locale ne sort).
    pub fn demandes(&self) -> &[Demande] {
        &self.demandes
    }
}

impl TransportCanal for DepotMemoire {
    fn relever(&mut self, demande: &Demande) -> Result<Vec<u8>, ErreurTransport> {
        // Passage par l'encodage : même chemin qu'une voie réelle.
        let demande = Demande::decoder(&demande.encoder()).map_err(|_| ErreurTransport::Absent)?;
        self.demandes.push(demande);
        match demande {
            Demande::Tete => self.tete.clone().ok_or(ErreurTransport::Absent),
            Demande::Entrees { depuis, nombre } => {
                let debut = usize::try_from(depuis).map_err(|_| ErreurTransport::Absent)?;
                let fin = debut.saturating_add(nombre as usize).min(self.entrees.len());
                let tranche = self.entrees.get(debut..fin).ok_or(ErreurTransport::Absent)?;
                Ok(encoder_entrees(tranche))
            }
            Demande::Artefact { empreinte, .. } => self.artefacts.get(&empreinte).cloned().ok_or(ErreurTransport::Absent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demandes_aller_retour_et_strictes() {
        for d in [
            Demande::Tete,
            Demande::Entrees { depuis: 7, nombre: 3 },
            Demande::Artefact { empreinte: [4; 32], taille: 99 },
        ] {
            assert_eq!(Demande::decoder(&d.encoder()).unwrap(), d);
        }
        let mut o = Demande::Tete.encoder();
        o.push(0);
        assert!(Demande::decoder(&o).is_err());
        assert!(Demande::decoder(&Demande::Entrees { depuis: 0, nombre: 0 }.encoder()).is_err());
        assert!(Demande::decoder(&Demande::Entrees { depuis: 0, nombre: 5_000 }.encoder()).is_err());
    }

    #[test]
    fn entrees_strictes() {
        let o = encoder_entrees(&[vec![1, 2], vec![3]]);
        assert_eq!(decoder_entrees(&o, 2).unwrap(), vec![vec![1, 2], vec![3]]);
        assert!(decoder_entrees(&o, 3).is_err());
        let mut plus = o.clone();
        plus.push(0);
        assert!(decoder_entrees(&plus, 2).is_err());
        assert!(decoder_entrees(&encoder_entrees(&[vec![]]), 1).is_err());
    }
}
