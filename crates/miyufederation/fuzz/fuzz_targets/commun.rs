//! @id     miyufederation_fuzz_commun
//! @do     outiller_cibles_fuzzing_federation
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles de miyufederation : 7 COG fondateurs fixes, têtes signées, fédération prête (C-26).

#![allow(dead_code)]

use std::sync::OnceLock;

use miyufederation::{Candidature, Federation, Parametres};
use miyuracine::{Racine, Usage};
use miyusceau::{empreinte_cle, sceller, CleSignature, TeteRegistre};

pub const T0: u64 = 1_800_000_000;
pub const JOUR: u64 = 86_400;
pub const DIFFICULTE: u8 = 8;
/// Instant des vérifications (candidatures valides 2 jours après T0).
pub const MAINTENANT: u64 = T0 + 100;
/// Nombre de COG fondateurs (l'accusé + 6 membres = quorum).
pub const FONDATEURS: usize = 7;

pub struct CogFuzz {
    pub identite: CleSignature,
    pub journal: CleSignature,
    pub registre: [u8; 32],
    pub candidature: Vec<u8>,
}

impl CogFuzz {
    fn nouveau(graine: u8) -> Self {
        let racine = Racine::depuis_secret(&[graine; 32]);
        let identite = racine.cle(Usage::Identite, 0).expect("identité");
        let journal = racine.cle(Usage::Journal, 0).expect("journal");
        let registre = empreinte_cle(&identite.cle_publique());
        let candidature =
            Candidature::preparer(identite.cle_publique(), journal.cle_publique(), registre, T0, DIFFICULTE)
                .expect("candidature")
                .sceller(&identite)
                .expect("sceau");
        Self { identite, journal, registre, candidature }
    }

    /// Tête de registre signée par la clé de journal du COG (`None` : tête refusée à l'encodage).
    pub fn tete(&self, sequence: u64, taille: u64, racine: [u8; 32], emise_a: u64) -> Option<Vec<u8>> {
        let tete = TeteRegistre { registre: self.registre, taille, racine, sequence, emise_a, validite_max_secs: JOUR };
        sceller(&tete, &self.journal).ok()
    }
}

/// Les 7 COG fondateurs (graines 1 à 7).
pub fn cogs() -> &'static [CogFuzz] {
    static COGS: OnceLock<Vec<CogFuzz>> = OnceLock::new();
    COGS.get_or_init(|| (1..=FONDATEURS as u8).map(CogFuzz::nouveau).collect())
}

/// Fédération où les 7 COG sont membres fondateurs.
pub fn federation() -> Federation {
    static FED: OnceLock<Federation> = OnceLock::new();
    FED.get_or_init(|| {
        let mut f = Federation::new(Parametres::default().avec_difficulte(DIFFICULTE).expect("difficulté"), None);
        for c in cogs() {
            f.ajouter_fondateur(&c.candidature, T0).expect("fondateur");
        }
        f
    })
    .clone()
}
