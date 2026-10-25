//! @id     miyuregistre_temps
//! @do     calculer_median_time_past
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « temps » (Median-Time-Past des têtes, horloge de confiance monotone).

//! Temps de confiance (EX-22 ; zones §17 « rejet de toute heure antérieure à
//! la médiane des 11 dernières têtes de registre vues »).
//!
//! - [`TempsMedian`] : fenêtre des horodatages des [`FENETRE_MTP`] dernières
//!   têtes acceptées ; **Median-Time-Past** = médiane de la fenêtre. Un
//!   horodatage strictement inférieur au MTP est refusé (égal accepté).
//!   Fenêtre paire (amorçage) : médiane **haute** (choix le plus strict).
//!   Fenêtre vide : aucune contrainte (amorçage).
//! - [`HorlogeConfiance`] : horloge monotone (ne recule jamais) alimentée par
//!   **toutes** les têtes acceptées (registre local, canal officiel, COG
//!   fédérés) ; une heure murale antérieure au MTP est refusée (horloge
//!   recalée pour ressusciter des titres, M10).
//!
//! Encodage persistant : `"MSMT" | version 0x01 | dernier (8) | n (1) | n × horodatage (8)`.

use std::collections::VecDeque;

use crate::erreurs::ErreurTemps;

/// Nombre de têtes de la fenêtre du Median-Time-Past (zones §17 : 11).
pub const FENETRE_MTP: usize = 11;

const MAGIC: &[u8; 4] = b"MSMT";
const VERSION: u8 = 0x01;

/// Fenêtre des derniers horodatages de têtes et Median-Time-Past.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TempsMedian {
    fenetre: VecDeque<u64>,
}

impl TempsMedian {
    /// Fenêtre vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Median-Time-Past, `None` si aucune tête vue.
    pub fn mtp(&self) -> Option<u64> {
        if self.fenetre.is_empty() {
            return None;
        }
        let mut tri: Vec<u64> = self.fenetre.iter().copied().collect();
        tri.sort_unstable();
        Some(tri[tri.len() / 2])
    }

    /// Refuse un horodatage strictement antérieur au MTP.
    pub fn controler(&self, horodatage: u64) -> Result<(), ErreurTemps> {
        match self.mtp() {
            Some(mtp) if horodatage < mtp => Err(ErreurTemps::AnterieurMedian { horodatage, mtp }),
            _ => Ok(()),
        }
    }

    /// Ajoute l'horodatage d'une tête acceptée (la plus ancienne sort au-delà de 11).
    pub fn observer(&mut self, horodatage: u64) {
        self.fenetre.push_back(horodatage);
        while self.fenetre.len() > FENETRE_MTP {
            self.fenetre.pop_front();
        }
    }

    /// Horodatages de la fenêtre (du plus ancien au plus récent).
    pub fn fenetre(&self) -> impl Iterator<Item = u64> + '_ {
        self.fenetre.iter().copied()
    }

    fn encoder_dans(&self, sortie: &mut Vec<u8>) {
        sortie.push(self.fenetre.len() as u8);
        for h in &self.fenetre {
            sortie.extend_from_slice(&h.to_be_bytes());
        }
    }

    fn decoder_depuis(octets: &[u8]) -> Result<Self, ErreurTemps> {
        let (&n, reste) = octets.split_first().ok_or(ErreurTemps::EtatInvalide)?;
        let n = usize::from(n);
        if n > FENETRE_MTP || reste.len() != n * 8 {
            return Err(ErreurTemps::EtatInvalide);
        }
        let fenetre = reste.as_chunks::<8>().0.iter().map(|c| u64::from_be_bytes(*c)).collect();
        Ok(Self { fenetre })
    }

    /// Encodage persistant : `n (1) | n × horodatage (8)`.
    pub fn encoder(&self) -> Vec<u8> {
        let mut sortie = Vec::with_capacity(1 + 8 * self.fenetre.len());
        self.encoder_dans(&mut sortie);
        sortie
    }

    /// Décodage strict de [`Self::encoder`].
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurTemps> {
        Self::decoder_depuis(octets)
    }
}

/// Horloge de confiance : monotone, bornée par le MTP de toutes les têtes vues.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HorlogeConfiance {
    median: TempsMedian,
    dernier: u64,
}

impl HorlogeConfiance {
    /// Horloge sans historique (amorçage).
    pub fn new() -> Self {
        Self::default()
    }

    /// Lit l'heure de confiance à partir de l'heure murale `mural` (secondes Unix).
    ///
    /// Refus si `mural` est antérieure au MTP des têtes vues ; sinon renvoie
    /// `max(mural, dernière heure rendue)` (ne recule jamais).
    pub fn maintenant(&mut self, mural: u64) -> Result<u64, ErreurTemps> {
        self.median.controler(mural)?;
        self.dernier = self.dernier.max(mural);
        Ok(self.dernier)
    }

    /// Prend en compte l'horodatage d'une tête acceptée (tout registre).
    pub fn observer_tete(&mut self, emise_a: u64) {
        self.median.observer(emise_a);
    }

    /// Median-Time-Past courant.
    pub fn mtp(&self) -> Option<u64> {
        self.median.mtp()
    }

    /// Encodage persistant (voir module).
    pub fn encoder(&self) -> Vec<u8> {
        let mut sortie = Vec::with_capacity(14 + 8 * FENETRE_MTP);
        sortie.extend_from_slice(MAGIC);
        sortie.push(VERSION);
        sortie.extend_from_slice(&self.dernier.to_be_bytes());
        self.median.encoder_dans(&mut sortie);
        sortie
    }

    /// Décodage strict.
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurTemps> {
        if octets.len() < 13 || &octets[0..4] != MAGIC || octets[4] != VERSION {
            return Err(ErreurTemps::EtatInvalide);
        }
        let dernier = u64::from_be_bytes(octets[5..13].try_into().map_err(|_| ErreurTemps::EtatInvalide)?);
        let median = TempsMedian::decoder_depuis(&octets[13..])?;
        Ok(Self { median, dernier })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mtp_mediane_des_onze_dernieres() {
        let mut t = TempsMedian::new();
        assert_eq!(t.mtp(), None);
        assert!(t.controler(0).is_ok());
        for h in 1..=11 {
            t.observer(h * 10);
        }
        assert_eq!(t.mtp(), Some(60));
        assert!(t.controler(60).is_ok());
        assert_eq!(t.controler(59), Err(ErreurTemps::AnterieurMedian { horodatage: 59, mtp: 60 }));
        // La plus ancienne sort de la fenêtre.
        t.observer(120);
        assert_eq!(t.fenetre().count(), FENETRE_MTP);
        assert_eq!(t.mtp(), Some(70));
    }

    #[test]
    fn mtp_insensible_a_quelques_horodatages_aberrants() {
        let mut t = TempsMedian::new();
        for h in [100, 101, 102, 103, 104, 105, 106, 107, 108] {
            t.observer(h);
        }
        // Deux têtes « du futur » ne déplacent pas la médiane au-delà des têtes honnêtes.
        t.observer(10_000);
        t.observer(10_001);
        assert_eq!(t.mtp(), Some(105));
    }

    #[test]
    fn fenetre_paire_mediane_haute() {
        let mut t = TempsMedian::new();
        t.observer(10);
        t.observer(20);
        assert_eq!(t.mtp(), Some(20));
    }

    #[test]
    fn horloge_monotone_et_refus_du_recul() {
        let mut h = HorlogeConfiance::new();
        assert_eq!(h.maintenant(1_000).unwrap(), 1_000);
        // Recul de l'horloge murale sans tête vue : l'heure de confiance ne recule pas.
        assert_eq!(h.maintenant(900).unwrap(), 1_000);
        for e in [2_000, 2_010, 2_020] {
            h.observer_tete(e);
        }
        assert_eq!(h.mtp(), Some(2_010));
        assert!(matches!(h.maintenant(1_500), Err(ErreurTemps::AnterieurMedian { .. })));
        assert_eq!(h.maintenant(2_015).unwrap(), 2_015);
    }

    #[test]
    fn encodage_aller_retour_et_strict() {
        let mut h = HorlogeConfiance::new();
        h.maintenant(5).unwrap();
        h.observer_tete(7);
        h.observer_tete(9);
        let o = h.encoder();
        assert_eq!(HorlogeConfiance::decoder(&o).unwrap(), h);
        let mut tronque = o.clone();
        tronque.pop();
        assert_eq!(HorlogeConfiance::decoder(&tronque), Err(ErreurTemps::EtatInvalide));
        let mut t = TempsMedian::new();
        t.observer(3);
        assert_eq!(TempsMedian::decoder(&t.encoder()).unwrap(), t);
        assert_eq!(TempsMedian::decoder(&[12]), Err(ErreurTemps::EtatInvalide));
    }
}
