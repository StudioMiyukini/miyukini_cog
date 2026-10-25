//! @id     miyufederation_fuzz_commun
//! @do     outiller_cibles_fuzzing_federation
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles de miyufederation : 7 COG fondateurs fixes, têtes signées, fédération prête (C-26).

#![allow(dead_code)]

use std::sync::OnceLock;

use miyufederation::{Candidature, CleEtatFederal, Federation, Parametres};
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
    pub fn nouveau(graine: u8) -> Self {
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

/// Secret local fixe des cibles (état `MFED`).
pub const SECRET_LOCAL: [u8; 32] = [0x5a; 32];

/// Politique des cibles : difficulté réduite, sans Origin.
pub fn parametres() -> Parametres {
    Parametres::default().avec_difficulte(DIFFICULTE).expect("difficulté")
}

/// Clé de scellement dérivée de [`SECRET_LOCAL`].
pub fn cle_etat() -> &'static CleEtatFederal {
    static CLE: OnceLock<CleEtatFederal> = OnceLock::new();
    CLE.get_or_init(|| CleEtatFederal::deriver(&SECRET_LOCAL))
}

/// Enveloppe `MFED` d'un corps quelconque, scellée par le secret local fixe
/// (même dérivation que `CleEtatFederal::deriver`, recalculée ici : le
/// fuzzing franchit le HMAC et atteint la revérification du corps).
pub fn envelopper(version: u8, corps: &[u8]) -> Vec<u8> {
    use hmac::{Mac, SimpleHmac};
    type H = SimpleHmac<sha2::Sha256>;
    let mut k = <H as Mac>::new_from_slice(&SECRET_LOCAL).expect("clé");
    k.update(b"miyukini/miyufederation/etat/v1");
    let cle = k.finalize().into_bytes();
    let mut s = Vec::with_capacity(9 + corps.len() + 32);
    s.extend_from_slice(b"MFED");
    s.push(version);
    s.extend_from_slice(&u32::try_from(corps.len()).unwrap_or(u32::MAX).to_be_bytes());
    s.extend_from_slice(corps);
    let mut m = <H as Mac>::new_from_slice(&cle).expect("clé");
    m.update(&s);
    s.extend_from_slice(&m.finalize().into_bytes());
    s
}

/// `cog_id` d'un COG de test.
pub fn cog_id(c: &CogFuzz) -> miyauth::CogId {
    miyauth::CogId::new(&miyuracine::cog_id_depuis_cle(&c.identite.cle_publique())).expect("cog_id")
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
        let mut f = Federation::new(parametres(), None);
        for c in cogs() {
            f.ajouter_fondateur(&c.candidature, T0).expect("fondateur");
        }
        f
    })
    .clone()
}
