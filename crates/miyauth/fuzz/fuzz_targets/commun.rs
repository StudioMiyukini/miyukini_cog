//! @id     miyauth_fuzz_commun
//! @do     outiller_cibles_fuzzing_identite
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles de miyauth : COG local et étranger, annuaire, passeport fixe (EX-10).

#![allow(dead_code)]

use miyauth::{
    AnnuaireEmetteurs, CibleRevocation, CogId, ErreurRevocation, IdentityRole, Passeport, PolitiqueIdentite, Portees,
    SourceRevocation,
};
use miyusceau::{sceller, CleSignature, NiveauConfiance};

pub const T0: u64 = 1_800_000_000;
pub const AUDIENCE: &str = "jayoffice";

pub fn cog_local() -> CogId {
    CogId::new("aaaaaaaaaaaaaaaaaaaaaaaaaa").expect("cog")
}

pub fn cog_etranger() -> CogId {
    CogId::new("bbbbbbbbbbbbbbbbbbbbbbbbbb").expect("cog")
}

/// Clé d'émission du COG local (souveraine).
pub fn cle_locale() -> CleSignature {
    CleSignature::depuis_graine(&[61; 32])
}

/// Clé d'émission du COG étranger (reconnu).
pub fn cle_etrangere() -> CleSignature {
    CleSignature::depuis_graine(&[62; 32])
}

/// Clé d'appareil du titulaire.
pub fn cle_appareil() -> CleSignature {
    CleSignature::depuis_graine(&[63; 32])
}

pub fn annuaire() -> AnnuaireEmetteurs {
    let mut a = AnnuaireEmetteurs::new();
    a.ajouter(cog_local(), &cle_locale().cle_publique(), NiveauConfiance::Souverain).expect("ancre locale");
    a.ajouter(cog_etranger(), &cle_etrangere().cle_publique(), NiveauConfiance::Reconnu).expect("ancre étrangère");
    a
}

/// Aucune révocation.
pub struct SansRevocation;

impl SourceRevocation for SansRevocation {
    fn est_revoque(&self, _: &CogId, _: &CibleRevocation, _: u64) -> Result<bool, ErreurRevocation> {
        Ok(false)
    }
}

pub fn politique<'a>(annuaire: &'a AnnuaireEmetteurs, revocations: &'a SansRevocation) -> PolitiqueIdentite<'a> {
    PolitiqueIdentite { cog_local: cog_local(), annuaire, revocations }
}

/// Passeport valide fixe (série fixe : déterministe), émis par `etranger` ou le COG local.
pub fn passeport(etranger: bool) -> Passeport {
    let (cog, cle) = if etranger { (cog_etranger(), cle_etrangere()) } else { (cog_local(), cle_locale()) };
    Passeport {
        serie: [5; 16],
        emetteur_cog: cog,
        emetteur_kid: cle.kid(),
        sujet: [6; 32],
        destination: cog_local(),
        cle_appareil: cle_appareil().cle_publique(),
        role: IdentityRole::Citizen,
        portees: Portees::new(["lecture", "ecriture"]).expect("portées"),
        pas_avant: T0,
        pas_apres: T0 + 86_400,
    }
}

/// Scelle un passeport avec la clé de son émetteur.
pub fn sceller_passeport(p: &Passeport) -> Vec<u8> {
    let cle = if p.emetteur_cog == cog_etranger() { cle_etrangere() } else { cle_locale() };
    sceller(p, &cle).expect("passeport scellable")
}
