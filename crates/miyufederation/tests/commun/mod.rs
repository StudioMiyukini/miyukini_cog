//! @id     miyufederation_tests_commun
//! @do     fabriquer_cog_de_test
//! @role   test
//! @layer  toolkit
//! @human  miyufederation — outillage de test (COG de test dérivés de racines miyuracine, têtes, diffusion).

#![allow(dead_code)]

use miyauth::CogId;
use miyufederation::{Candidature, Confirmation, Federation, Parametres};
use miyuracine::{Racine, Usage};
use miyusceau::{empreinte_cle, sceller, CleSignature, TeteRegistre};

/// Instant de départ (secondes Unix).
pub const T0: u64 = 1_800_000_000;
/// Un jour.
pub const JOUR: u64 = 86_400;
/// Difficulté réduite pour les tests (la politique par défaut exige 22 bits).
pub const DIFFICULTE_TEST: u8 = 8;

/// COG de test : clés dérivées d'une racine miyuracine déterministe.
pub struct CogTest {
    pub identite: CleSignature,
    pub journal: CleSignature,
    pub emission: CleSignature,
    pub registre: [u8; 32],
    pub cog: CogId,
    pub candidature: Vec<u8>,
}

impl CogTest {
    pub fn nouveau(graine: u8, t: u64) -> Self {
        let racine = Racine::depuis_secret(&[graine; 32]);
        let identite = racine.cle(Usage::Identite, 0).unwrap();
        let journal = racine.cle(Usage::Journal, 0).unwrap();
        let emission = racine.cle(Usage::Emission, 0).unwrap();
        let registre = empreinte_cle(&identite.cle_publique());
        let cog = CogId::new(&racine.cog_id()).unwrap();
        let candidature =
            Candidature::preparer(identite.cle_publique(), journal.cle_publique(), registre, t, DIFFICULTE_TEST)
                .unwrap()
                .sceller(&identite)
                .unwrap();
        Self { identite, journal, emission, registre, cog, candidature }
    }

    /// Confirmation de `autre` par ce COG.
    pub fn confirmer(&self, autre: &CogTest, t: u64) -> Vec<u8> {
        Confirmation::nouvelle(autre.identite.cle_publique(), self.identite.cle_publique(), t)
            .sceller(&self.identite)
            .unwrap()
    }

    /// Tête de son registre, signée par sa clé de journal.
    pub fn tete(&self, sequence: u64, taille: u64, racine: [u8; 32], t: u64) -> Vec<u8> {
        let tete = TeteRegistre { registre: self.registre, taille, racine, sequence, emise_a: t, validite_max_secs: JOUR };
        sceller(&tete, &self.journal).unwrap()
    }
}

/// Politique de test : difficulté réduite, sans Origin.
pub fn federation() -> Federation {
    Federation::new(Parametres::default().avec_difficulte(DIFFICULTE_TEST).unwrap(), None)
}
