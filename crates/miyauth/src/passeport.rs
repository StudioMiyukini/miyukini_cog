//! @id     miyauth_passeport
//! @do     emettre_verifier_passeport_visa
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — émission et vérification des passeports et visas scellés, preuve de possession.

//! Émission et vérification des passeports et visas (doc gelée lot 2, §5 à §7).
//!
//! Chaîne de vérification d'un accès (fail-closed, premier refus = fin) :
//!
//! 1. Passeport : sceau v1.1 (`verify_strict`), clé d'émission épinglée dans
//!    l'[`AnnuaireEmetteurs`], fenêtre de validité (tolérance d'horloge bornée du
//!    format, 120 s), durée ≤ 30 j, signataire = `emetteur_kid` déclaré, `cog_id`
//!    émetteur = celui de l'ancre, niveau cohérent (son COG ⇔ `Souverain`),
//!    destination = COG local (EX-36), non révoqué (série, appareil, clé d'émission).
//! 2. Visa : sceau v1.1 signé par une ancre **souveraine du COG local**, durée
//!    ≤ 15 min, contenu dans la fenêtre du passeport, lié au passeport (empreinte),
//!    au sujet par paire et à la clé d'appareil, rôle cohérent, audience attendue,
//!    non révoqué.
//! 3. Preuve de possession (§7) par la clé d'appareil.
//!
//! Les types [`PasseportVerifie`], [`PossessionProuvee`], [`AccesVerifie`] et [`AccesProuve`] ont des
//! champs privés : les détenir prouve que les contrôles ont eu lieu.

use std::collections::HashMap;
use std::fmt;

use ed25519_dalek::{Signature, VerifyingKey};
use miyusceau::{
    empreinte_cle, sceller, verifier, AncresDeConfiance, CleSignature, ErreurSceau, Kid, NiveauConfiance, Sceau,
    Verifie,
};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

use crate::pairwise::{identifiant_pairwise, SecretTitulaire};
use crate::revocation::{CibleRevocation, ErreurRevocation, SourceRevocation};
use crate::titres::{audience_valide, CogId, Passeport, Portees, Serie, Visa};
use crate::types::IdentityRole;

/// Domaine de séparation de la preuve de possession (doc §7).
pub const DOMAINE_POSSESSION: &[u8] = b"miyukini/pop/v1";

// ═══════════════════════════════════════════════════════════════════════════════
// Refus
// ═══════════════════════════════════════════════════════════════════════════════

/// Motif de refus d'un titre d'identité. Toute variante signifie « refus ».
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusIdentite {
    /// Sceau invalide (format, signature, signataire inconnu, fenêtre de validité…).
    Sceau(ErreurSceau),
    /// Signataire absent de l'annuaire des émetteurs, ou différent de l'émetteur déclaré.
    EmetteurIncoherent,
    /// Niveau de confiance de l'ancre incohérent avec le COG émetteur.
    NiveauIncoherent,
    /// Passeport émis pour un autre COG (EX-36).
    DestinationIncorrecte,
    /// Visa non émis par le COG local.
    VisaEtranger,
    /// Visa émis pour un autre passeport.
    PasseportDifferent,
    /// Sujet du visa différent de celui du passeport.
    SujetDifferent,
    /// Clé d'appareil du visa différente de celle du passeport.
    CleAppareilDifferente,
    /// Rôle incohérent (visiteur / citoyen).
    RoleIncoherent,
    /// Audience différente du service vérificateur.
    AudienceIncorrecte,
    /// Fenêtre du visa hors de celle du passeport.
    FenetreHorsPasseport,
    /// Portée du visa absente du passeport (pas d'élévation par le visa).
    PorteeHorsPasseport,
    /// Cible révoquée.
    Revoque(CibleRevocation),
    /// Statut de révocation indisponible ou périmé (fail-closed).
    Revocation(ErreurRevocation),
    /// Preuve de possession invalide (ou clé d'appareil invalide).
    PossessionInvalide,
    /// Paramètre d'émission invalide.
    ParametreInvalide(&'static str),
    /// Artefact (passeport ou visa) absent.
    ArtefactManquant,
}

impl fmt::Display for RefusIdentite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sceau(e) => write!(f, "sceau refusé : {e}"),
            Self::EmetteurIncoherent => write!(f, "émetteur incohérent ou non épinglé"),
            Self::NiveauIncoherent => write!(f, "niveau de confiance incohérent avec le COG émetteur"),
            Self::DestinationIncorrecte => write!(f, "passeport émis pour un autre COG"),
            Self::VisaEtranger => write!(f, "visa non émis par ce COG"),
            Self::PasseportDifferent => write!(f, "visa lié à un autre passeport"),
            Self::SujetDifferent => write!(f, "sujet du visa différent du passeport"),
            Self::CleAppareilDifferente => write!(f, "clé d'appareil du visa différente du passeport"),
            Self::RoleIncoherent => write!(f, "rôle incohérent"),
            Self::AudienceIncorrecte => write!(f, "audience incorrecte"),
            Self::FenetreHorsPasseport => write!(f, "validité du visa hors de celle du passeport"),
            Self::PorteeHorsPasseport => write!(f, "portée du visa absente du passeport"),
            Self::Revoque(c) => write!(f, "révoqué : {c:?}"),
            Self::Revocation(e) => write!(f, "révocation : {e}"),
            Self::PossessionInvalide => write!(f, "preuve de possession invalide"),
            Self::ParametreInvalide(p) => write!(f, "paramètre d'émission invalide : {p}"),
            Self::ArtefactManquant => write!(f, "passeport ou visa manquant"),
        }
    }
}

impl std::error::Error for RefusIdentite {}

impl From<ErreurSceau> for RefusIdentite {
    fn from(e: ErreurSceau) -> Self {
        Self::Sceau(e)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Annuaire et politique
// ═══════════════════════════════════════════════════════════════════════════════

/// Clés d'émission épinglées, chacune rattachée à son `cog_id`.
///
/// Seul mode admis : clés d'émission **explicitement épinglées** (ancres directes).
/// La chaîne racine → clé d'émission (certificat `0x06`) n'est pas implémentée :
/// la doc gelée ne définit ni la `nature` d'un certificat d'émission (table du
/// lot 2 §4.1 : `0x01`, `0x41`–`0x44` seulement, tout autre code refusé) ni ses
/// champs TLV (A-003, « à décider ultérieurement »). Une clé non épinglée est
/// refusée (`EmetteurIncoherent` ou signataire inconnu).
#[derive(Debug, Clone, Default)]
pub struct AnnuaireEmetteurs {
    ancres: AncresDeConfiance,
    cogs: HashMap<Kid, (CogId, NiveauConfiance)>,
}

impl AnnuaireEmetteurs {
    /// Annuaire vide (refuse tout).
    pub fn new() -> Self {
        Self::default()
    }

    /// Épingle la clé d'émission `cle_publique` du COG `cog`. `Souverain` pour son
    /// propre COG, `Reconnu` pour un COG reconnu. Refuse les clés faibles.
    pub fn ajouter(&mut self, cog: CogId, cle_publique: &[u8; 32], niveau: NiveauConfiance) -> Result<Kid, ErreurSceau> {
        let kid = self.ancres.ajouter(cle_publique, niveau)?;
        self.cogs.insert(kid, (cog, niveau));
        Ok(kid)
    }

    /// Retire une clé (rotation ou compromission).
    pub fn retirer(&mut self, kid: &Kid) -> bool {
        self.cogs.remove(kid);
        self.ancres.retirer(kid)
    }

    fn cog_de(&self, kid: &Kid) -> Option<&(CogId, NiveauConfiance)> {
        self.cogs.get(kid)
    }
}

/// Politique de vérification du COG local.
pub struct PolitiqueIdentite<'a> {
    /// `cog_id` du COG qui vérifie.
    pub cog_local: CogId,
    /// Clés d'émission épinglées.
    pub annuaire: &'a AnnuaireEmetteurs,
    /// Statut de révocation (fail-closed).
    pub revocations: &'a dyn SourceRevocation,
}

impl fmt::Debug for PolitiqueIdentite<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PolitiqueIdentite").field("cog_local", &self.cog_local).finish_non_exhaustive()
    }
}

fn controler_revocation(
    source: &dyn SourceRevocation,
    emetteur: &CogId,
    cibles: &[CibleRevocation],
    maintenant: u64,
) -> Result<(), RefusIdentite> {
    for cible in cibles {
        match source.est_revoque(emetteur, cible, maintenant) {
            Ok(false) => {}
            Ok(true) => return Err(RefusIdentite::Revoque(*cible)),
            Err(e) => return Err(RefusIdentite::Revocation(e)),
        }
    }
    Ok(())
}

fn cle_appareil_valide(cle: &[u8; 32]) -> Option<VerifyingKey> {
    VerifyingKey::from_bytes(cle).ok().filter(|k| !k.is_weak())
}

fn serie_aleatoire() -> Serie {
    let mut s = [0u8; 16];
    OsRng.fill_bytes(&mut s);
    s
}

/// SHA-256 des octets signés d'un sceau (identité du titre, doc §4.1).
pub fn empreinte_titre(octets: &[u8]) -> Result<[u8; 32], ErreurSceau> {
    Ok(Sha256::digest(Sceau::decoder(octets)?.octets_signes()).into())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Passeport
// ═══════════════════════════════════════════════════════════════════════════════

/// Demande d'émission d'un passeport (côté COG d'origine).
#[derive(Debug)]
pub struct DemandePasseport<'a> {
    /// Secret de pseudonymisation du titulaire (EX-36).
    pub secret: &'a SecretTitulaire,
    /// COG pour lequel le passeport est émis (le COG d'origine pour un usage local).
    pub destination: CogId,
    /// Clé publique de l'appareil titulaire.
    pub cle_appareil: [u8; 32],
    /// Droits de base.
    pub portees: Portees,
    /// Début de validité.
    pub pas_avant: u64,
    /// Fin de validité (≤ 30 j après `pas_avant`).
    pub pas_apres: u64,
}

/// Émet (scelle) un passeport avec la clé d'émission du COG d'origine `emetteur`.
pub fn emettre_passeport(
    emetteur: &CogId,
    cle_emission: &CleSignature,
    demande: &DemandePasseport<'_>,
) -> Result<Vec<u8>, RefusIdentite> {
    if cle_appareil_valide(&demande.cle_appareil).is_none() {
        return Err(RefusIdentite::ParametreInvalide("cle_appareil"));
    }
    let passeport = Passeport {
        serie: serie_aleatoire(),
        emetteur_cog: emetteur.clone(),
        emetteur_kid: cle_emission.kid(),
        sujet: identifiant_pairwise(demande.secret, &demande.destination),
        destination: demande.destination.clone(),
        cle_appareil: demande.cle_appareil,
        role: IdentityRole::Citizen,
        portees: demande.portees.clone(),
        pas_avant: demande.pas_avant,
        pas_apres: demande.pas_apres,
    };
    Ok(sceller(&passeport, cle_emission)?)
}

/// Passeport vérifié pour le COG local (champs privés).
#[derive(Debug, Clone)]
pub struct PasseportVerifie {
    verifie: Verifie<Passeport>,
    empreinte: [u8; 32],
    niveau: NiveauConfiance,
}

impl PasseportVerifie {
    /// Passeport.
    pub fn passeport(&self) -> &Passeport {
        self.verifie.titre()
    }

    /// SHA-256 des octets signés (lien du visa).
    pub fn empreinte(&self) -> &[u8; 32] {
        &self.empreinte
    }

    /// Niveau de confiance de l'émetteur (`Souverain` = son COG).
    pub fn niveau(&self) -> NiveauConfiance {
        self.niveau
    }
}

/// Vérifie un passeport présenté au COG local (étape 1 de la chaîne).
pub fn verifier_passeport(
    octets: &[u8],
    politique: &PolitiqueIdentite<'_>,
    maintenant: u64,
) -> Result<PasseportVerifie, RefusIdentite> {
    let verifie = verifier::<Passeport>(octets, &politique.annuaire.ancres, maintenant)?;
    let p = verifie.titre();
    if verifie.signataires() != [p.emetteur_kid] {
        return Err(RefusIdentite::EmetteurIncoherent);
    }
    let (cog_ancre, niveau) = politique.annuaire.cog_de(&p.emetteur_kid).ok_or(RefusIdentite::EmetteurIncoherent)?;
    if cog_ancre != &p.emetteur_cog {
        return Err(RefusIdentite::EmetteurIncoherent);
    }
    let local = p.emetteur_cog == politique.cog_local;
    if local != (*niveau == NiveauConfiance::Souverain) {
        return Err(RefusIdentite::NiveauIncoherent);
    }
    if p.destination != politique.cog_local {
        return Err(RefusIdentite::DestinationIncorrecte);
    }
    if cle_appareil_valide(&p.cle_appareil).is_none() {
        return Err(RefusIdentite::PossessionInvalide);
    }
    let cibles = [
        CibleRevocation::Serie(p.serie),
        CibleRevocation::appareil(&p.cle_appareil),
        CibleRevocation::Cle(p.emetteur_kid),
    ];
    controler_revocation(politique.revocations, &p.emetteur_cog, &cibles, maintenant)?;
    if !local {
        // Le COG hôte peut aussi bannir localement un passeport ou un appareil étranger.
        controler_revocation(politique.revocations, &politique.cog_local, &cibles[..2], maintenant)?;
    }
    let niveau = *niveau;
    let empreinte = empreinte_titre(octets)?;
    Ok(PasseportVerifie { verifie, empreinte, niveau })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Preuve de possession (§7)
// ═══════════════════════════════════════════════════════════════════════════════

/// Octets à signer par l'appareil :
/// `"miyukini/pop/v1" || defi (32 o) || audience || liaison_canal (32 o)`.
///
/// Défi et liaison de longueur fixe : l'audience, au milieu, est non ambiguë.
pub fn message_possession(defi: &[u8; 32], audience: &str, liaison_canal: &[u8; 32]) -> Vec<u8> {
    let mut m = Vec::with_capacity(DOMAINE_POSSESSION.len() + 64 + audience.len());
    m.extend_from_slice(DOMAINE_POSSESSION);
    m.extend_from_slice(defi);
    m.extend_from_slice(audience.as_bytes());
    m.extend_from_slice(liaison_canal);
    m
}

fn controler_possession(
    cle_appareil: &[u8; 32],
    defi: &[u8; 32],
    audience: &str,
    liaison_canal: &[u8; 32],
    signature: &[u8; 64],
) -> Result<(), RefusIdentite> {
    if !audience_valide(audience) {
        return Err(RefusIdentite::AudienceIncorrecte);
    }
    let cle = cle_appareil_valide(cle_appareil).ok_or(RefusIdentite::PossessionInvalide)?;
    cle.verify_strict(&message_possession(defi, audience, liaison_canal), &Signature::from_bytes(signature))
        .map_err(|_| RefusIdentite::PossessionInvalide)
}

/// Passeport dont le porteur a prouvé détenir la clé d'appareil (champs privés).
#[derive(Debug, Clone)]
pub struct PossessionProuvee {
    passeport: PasseportVerifie,
}

impl PossessionProuvee {
    /// Passeport vérifié.
    pub fn passeport(&self) -> &PasseportVerifie {
        &self.passeport
    }
}

/// Vérifie la preuve de possession du porteur d'un passeport. Le défi (32 o
/// aléatoires, usage unique) est généré et invalidé par l'appelant (anti-rejeu).
pub fn prouver_possession(
    passeport: PasseportVerifie,
    defi: &[u8; 32],
    audience: &str,
    liaison_canal: &[u8; 32],
    signature: &[u8; 64],
) -> Result<PossessionProuvee, RefusIdentite> {
    controler_possession(&passeport.passeport().cle_appareil, defi, audience, liaison_canal, signature)?;
    Ok(PossessionProuvee { passeport })
}

// ═══════════════════════════════════════════════════════════════════════════════
// Visa
// ═══════════════════════════════════════════════════════════════════════════════

/// Demande d'émission d'un visa (côté COG hôte).
#[derive(Debug, Clone)]
pub struct DemandeVisa {
    /// Service visé.
    pub audience: String,
    /// Droits restreints au service.
    pub portees: Portees,
    /// Début de validité.
    pub pas_avant: u64,
    /// Fin de validité (≤ 15 min, et ≤ fin du passeport).
    pub pas_apres: u64,
}

fn role_attendu(passeport: &Passeport, cog_local: &CogId) -> IdentityRole {
    if &passeport.emetteur_cog == cog_local {
        IdentityRole::Citizen
    } else {
        IdentityRole::Visitor
    }
}

/// Émet un visa pour un passeport dont la possession a été prouvée. `cog_local`
/// est le COG hôte, `cle_emission` sa clé d'émission.
pub fn emettre_visa(
    cog_local: &CogId,
    cle_emission: &CleSignature,
    possession: &PossessionProuvee,
    demande: &DemandeVisa,
) -> Result<Vec<u8>, RefusIdentite> {
    let pv = &possession.passeport;
    let p = pv.passeport();
    if &p.destination != cog_local {
        return Err(RefusIdentite::DestinationIncorrecte);
    }
    if demande.pas_avant < p.pas_avant || demande.pas_apres > p.pas_apres {
        return Err(RefusIdentite::FenetreHorsPasseport);
    }
    // Atténuation : un visa ne peut accorder que des portées du passeport.
    if !portees_incluses(&demande.portees, &p.portees) {
        return Err(RefusIdentite::PorteeHorsPasseport);
    }
    let visa = Visa {
        serie: serie_aleatoire(),
        passeport: pv.empreinte,
        emetteur_cog: cog_local.clone(),
        sujet: p.sujet,
        cle_appareil: p.cle_appareil,
        audience: demande.audience.clone(),
        role: role_attendu(p, cog_local),
        portees: demande.portees.clone(),
        pas_avant: demande.pas_avant,
        pas_apres: demande.pas_apres,
    };
    Ok(sceller(&visa, cle_emission)?)
}

/// Accès vérifié : passeport + visa liés, pour une audience (champs privés).
#[derive(Debug, Clone)]
pub struct AccesVerifie {
    passeport: PasseportVerifie,
    visa: Verifie<Visa>,
}

impl AccesVerifie {
    /// Passeport vérifié.
    pub fn passeport(&self) -> &PasseportVerifie {
        &self.passeport
    }

    /// Visa vérifié.
    pub fn visa(&self) -> &Visa {
        self.visa.titre()
    }

    /// Rôle accordé par le visa.
    pub fn role(&self) -> IdentityRole {
        self.visa.titre().role
    }

    /// Identifiant par paire du titulaire pour le COG local.
    pub fn sujet(&self) -> &[u8; 32] {
        &self.visa.titre().sujet
    }

    /// Vérifie la preuve de possession de session (défi du service, audience du
    /// visa) et rend l'accès **prouvé**, seul type qui accorde un rôle.
    ///
    /// Un visa vérifié sans possession ne prouve rien (il peut avoir été
    /// intercepté) : appeler avec un défi neuf à chaque session.
    pub fn verifier_possession(
        &self,
        defi: &[u8; 32],
        liaison_canal: &[u8; 32],
        signature: &[u8; 64],
    ) -> Result<AccesProuve, RefusIdentite> {
        let v = self.visa.titre();
        controler_possession(&v.cle_appareil, defi, &v.audience, liaison_canal, signature)?;
        Ok(AccesProuve { acces: self.clone() })
    }
}

/// Accès prouvé : visa vérifié **et** possession de la clé d'appareil prouvée
/// pour un défi de session. Constructible seulement par
/// [`AccesVerifie::verifier_possession`] ; c'est lui qu'exige `identity_resolve`.
#[derive(Debug, Clone)]
pub struct AccesProuve {
    acces: AccesVerifie,
}

impl AccesProuve {
    /// Accès vérifié sous-jacent.
    pub fn acces(&self) -> &AccesVerifie {
        &self.acces
    }

    /// Rôle accordé par le visa.
    pub fn role(&self) -> IdentityRole {
        self.acces.role()
    }

    /// Identifiant par paire du titulaire pour le COG local.
    pub fn sujet(&self) -> &[u8; 32] {
        self.acces.sujet()
    }

    /// COG d'origine du titulaire (émetteur du passeport) : l'identifiant par
    /// paire n'est unique que dans l'espace de ce COG.
    pub fn cog_origine(&self) -> &CogId {
        &self.acces.passeport.passeport().emetteur_cog
    }
}

/// Vérifie un visa et son passeport pour le service `audience` du COG local.
pub fn verifier_visa(
    visa_octets: &[u8],
    passeport_octets: &[u8],
    audience: &str,
    politique: &PolitiqueIdentite<'_>,
    maintenant: u64,
) -> Result<AccesVerifie, RefusIdentite> {
    let passeport = verifier_passeport(passeport_octets, politique, maintenant)?;
    let visa = verifier::<Visa>(visa_octets, &politique.annuaire.ancres, maintenant)?;
    let v = visa.titre();
    let [signataire] = visa.signataires() else {
        return Err(RefusIdentite::EmetteurIncoherent);
    };
    let (cog_ancre, niveau) = politique.annuaire.cog_de(signataire).ok_or(RefusIdentite::EmetteurIncoherent)?;
    if cog_ancre != &politique.cog_local || *niveau != NiveauConfiance::Souverain || v.emetteur_cog != politique.cog_local {
        return Err(RefusIdentite::VisaEtranger);
    }
    let p = passeport.passeport();
    if v.passeport != passeport.empreinte {
        return Err(RefusIdentite::PasseportDifferent);
    }
    if v.sujet != p.sujet {
        return Err(RefusIdentite::SujetDifferent);
    }
    if v.cle_appareil != p.cle_appareil {
        return Err(RefusIdentite::CleAppareilDifferente);
    }
    if v.role != role_attendu(p, &politique.cog_local) {
        return Err(RefusIdentite::RoleIncoherent);
    }
    if v.audience != audience {
        return Err(RefusIdentite::AudienceIncorrecte);
    }
    if v.pas_avant < p.pas_avant || v.pas_apres > p.pas_apres {
        return Err(RefusIdentite::FenetreHorsPasseport);
    }
    // Revérifié côté vérificateur : un émetteur fautif ne contourne pas l'atténuation.
    if !portees_incluses(&v.portees, &p.portees) {
        return Err(RefusIdentite::PorteeHorsPasseport);
    }
    controler_revocation(
        politique.revocations,
        &politique.cog_local,
        &[CibleRevocation::Serie(v.serie), CibleRevocation::Cle(*signataire)],
        maintenant,
    )?;
    Ok(AccesVerifie { passeport, visa })
}

/// Vrai si chaque portée de `visa` figure dans `passeport` (comparaison exacte,
/// sans joker : la doc gelée ne définit pas de portée générique).
fn portees_incluses(visa: &Portees, passeport: &Portees) -> bool {
    visa.as_slice().iter().all(|x| passeport.contient(x))
}

/// `kid` d'une clé d'appareil (cible de révocation « appareil »).
pub fn kid_appareil(cle_appareil: &[u8; 32]) -> Kid {
    empreinte_cle(cle_appareil)
}
