//! @id     miyurelais_veille
//! @do     detecter_capteur_muet
//! @role   securite
//! @layer  toolkit
//! @human  miyurelais — veille du capteur muet : aucun paquet NFLOG pendant N s alors que la voie est active ⇒ voie aveugle.

//! Veille du capteur muet.
//!
//! Le relais ne voit que ce que le noyau journalise : rejets de la Douane et,
//! si la règle existe, témoins de vie (`MDOU-TEMOIN|`). Un capteur **muet**
//! (aucun paquet pendant [`Veille::delai_ms`] alors que la voie est **active**)
//! ne prouve plus rien : la relève échoue, la Vigie rend un lot en échec sans
//! pouls, CaringNanny passe la santé de la voie à `Inconnu` et WorrySentinel
//! la **coupe** (voie aveugle, décision du propriétaire A-004).
//!
//! « Voie active » est déclaré par l'appelant ([`Veille::activer`]) : voie
//! ouverte et trafic attendu (ou témoin configuré). Une voie inactive n'est
//! jamais muette. Le délai court depuis le dernier paquet ou, à défaut,
//! depuis l'activation.

/// État de la veille.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatVeille {
    /// Voie non active : le silence est normal.
    Inactive,
    /// Voie active, paquet récent.
    Vivante,
    /// Voie active, aucun paquet depuis `silence_ms` (≥ délai).
    Muette {
        /// Durée du silence.
        silence_ms: u64,
    },
}

/// Veille d'un capteur NFLOG.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_field_names)]
pub struct Veille {
    delai_ms: u64,
    active_depuis_ms: Option<u64>,
    dernier_paquet_ms: Option<u64>,
}

impl Veille {
    /// Veille de délai `delai_ms` (au moins 1 ms).
    pub fn new(delai_ms: u64) -> Self {
        Self { delai_ms: delai_ms.max(1), active_depuis_ms: None, dernier_paquet_ms: None }
    }

    /// Délai de silence toléré.
    pub fn delai_ms(&self) -> u64 {
        self.delai_ms
    }

    /// Déclare la voie active à partir de `t_ms` (sans effet si déjà active).
    pub fn activer(&mut self, t_ms: u64) {
        self.active_depuis_ms.get_or_insert(t_ms);
    }

    /// Déclare la voie inactive.
    pub fn desactiver(&mut self) {
        self.active_depuis_ms = None;
    }

    /// Vrai si la voie est déclarée active.
    pub fn est_active(&self) -> bool {
        self.active_depuis_ms.is_some()
    }

    /// Un paquet (ligne ou témoin) a été reçu à `t_ms`.
    pub fn observer(&mut self, t_ms: u64) {
        self.dernier_paquet_ms = Some(self.dernier_paquet_ms.map_or(t_ms, |d| d.max(t_ms)));
    }

    /// État à `maintenant_ms`.
    pub fn etat(&self, maintenant_ms: u64) -> EtatVeille {
        let Some(active) = self.active_depuis_ms else {
            return EtatVeille::Inactive;
        };
        let reference = self.dernier_paquet_ms.map_or(active, |d| d.max(active));
        let silence_ms = maintenant_ms.saturating_sub(reference);
        if silence_ms >= self.delai_ms {
            EtatVeille::Muette { silence_ms }
        } else {
            EtatVeille::Vivante
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inactive_jamais_muette() {
        let v = Veille::new(5_000);
        assert_eq!(v.etat(u64::MAX), EtatVeille::Inactive);
    }

    #[test]
    fn muette_apres_le_delai_sans_paquet() {
        let mut v = Veille::new(5_000);
        v.activer(1_000);
        assert_eq!(v.etat(5_999), EtatVeille::Vivante);
        assert_eq!(v.etat(6_000), EtatVeille::Muette { silence_ms: 5_000 });
        v.observer(6_500);
        assert_eq!(v.etat(11_000), EtatVeille::Vivante);
        assert_eq!(v.etat(11_500), EtatVeille::Muette { silence_ms: 5_000 });
    }

    #[test]
    fn paquet_ancien_ne_rajeunit_pas() {
        let mut v = Veille::new(1_000);
        v.observer(10_000);
        v.observer(2_000);
        v.activer(10_500);
        v.activer(99_999); // déjà active : sans effet
        assert_eq!(v.etat(11_400), EtatVeille::Vivante);
        assert_eq!(v.etat(11_500), EtatVeille::Muette { silence_ms: 1_000 });
        v.desactiver();
        assert_eq!(v.etat(50_000), EtatVeille::Inactive);
    }
}
