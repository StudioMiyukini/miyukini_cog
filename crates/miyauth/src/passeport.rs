//! @id     miyauth_passeport
//! @do     emettre_verifier_passeport_visa
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — émission et vérification des passeports et visas scellés, preuve de possession.

//! Émission et vérification des passeports et visas (doc gelée lot 2, §5 à §7).
//!
//! Chaîne de vérification d'un accès (fail-closed, premier refus = fin) :
//!
//! 1. Passeport : sceau v1.1 (`verify_strict`), clé signataire connue de
//!    l'[`AnnuaireEmetteurs`], fenêtre de validité (tolérance d'horloge bornée du
//!    format, 120 s), durée ≤ 30 j, signataire = `emetteur_kid` déclaré, `cog_id`
//!    émetteur = celui de l'ancre, niveau cohérent (son COG ⇔ `Souverain`),
//!    destination = COG local (EX-36), non révoqué (série, appareil, clé d'émission).
//! 2. Visa : sceau v1.1 signé par une clé **souveraine du COG local**, durée
//!    ≤ 15 min, contenu dans la fenêtre du passeport, lié au passeport (empreinte),
//!    au sujet par paire et à la clé d'appareil, rôle cohérent, audience attendue
//!    (le service), portées ⊆ passeport, non révoqué.
//! 3. Preuve de possession (§7) par la clé d'appareil.
//!
//! ## Chaîne racine → clé d'usage (A-022 §2)
//!
//! Pour un COG dont la **racine** (clé d'identité, indice 0) est épinglée
//! ([`AnnuaireEmetteurs::epingler_racine`]), une signature n'est acceptée que
//! si la clé signataire est certifiée par une délégation `0x45`
//! ([`miyusceau::DelegationCle`]) connue de l'annuaire
//! ([`AnnuaireEmetteurs::ajouter_delegation`]) ou présentée avec le titre
//! ([`AnnuaireEmetteurs::avec_delegations`]) : signée par l'identité épinglée,
//! même `cog_id`, **indice attendu** ([`INDICE_USAGE_EMISSION`] = 1 pour un
//! passeport, [`INDICE_USAGE_VISAS`] = 5 pour un visa), fenêtre couvrant
//! l'instant **et** la fenêtre du titre, génération cohérente (une seule clé
//! par `(cog, indice, génération)`, une clé pour un seul usage). Sinon refus.
//!
//! Un visa signé par la clé d'indice 1 est refusé sans transition : aucune
//! délégation n'existait avant A-022 et un visa vit au plus 15 min.
//!
//! **Transition (ancres directes)** : une clé épinglée par
//! [`AnnuaireEmetteurs::ajouter`] (mode antérieur à A-022 : la même clé, sans
//! indice connu, signe passeports et visas) reste acceptée **uniquement** pour
//! un COG sans racine épinglée et tant que l'annuaire n'est pas passé en mode
//! strict ([`AnnuaireEmetteurs::exiger_delegations`]). Épingler la racine d'un
//! COG interdit ses ancres directes. Fin de transition : guichet des visas en
//! zone COG (A-022 §1), puis mode strict partout.
//!
//! Les types [`PasseportVerifie`], [`PossessionProuvee`], [`AccesVerifie`] et [`AccesProuve`] ont des
//! champs privés : les détenir prouve que les contrôles ont eu lieu.

use std::collections::HashMap;
use std::fmt;

use ed25519_dalek::{Signature, VerifyingKey};
use miyusceau::{
    cog_id_depuis_cle, decoder_titre, empreinte_cle, sceller, verifier, verifier_delegation, AncresDeConfiance,
    CleSignature, DelegationCle, ErreurSceau, Kid, NiveauConfiance, Sceau, Verifie,
};
use rand_core::{OsRng, RngCore};
use sha2::{Digest, Sha256};

use crate::pairwise::{identifiant_pairwise, SecretTitulaire};
use crate::revocation::{CibleRevocation, ErreurRevocation, SourceRevocation};
use crate::titres::{audience_valide, CogId, Passeport, Portees, Serie, Visa};
use crate::types::IdentityRole;

/// Domaine de séparation de la preuve de possession (doc §7).
pub const DOMAINE_POSSESSION: &[u8] = b"miyukini/pop/v1";
/// Indice d'usage SLIP-10 de la clé qui signe les **passeports** (`emission`,
/// `m/1296256839'/1'/g'`) ; depuis A-022 §1, elle ne signe plus les visas.
pub const INDICE_USAGE_EMISSION: u32 = 1;
/// Indice d'usage SLIP-10 de la clé qui signe les **visas** (`visas`,
/// `m/1296256839'/5'/g'`, A-022 §1).
pub const INDICE_USAGE_VISAS: u32 = 5;

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
    /// Clé d'usage sans délégation `0x45` valide (COG à racine épinglée ou mode strict).
    SansDelegation,
    /// Clé déléguée pour un autre usage que celui du titre (ex. visa signé par l'indice 1).
    UsageIncorrect {
        /// Indice attendu pour ce titre.
        attendu: u32,
        /// Indice certifié par la délégation.
        delegue: u32,
    },
    /// Fenêtre de la délégation ne couvrant pas l'instant ou la fenêtre du titre.
    DelegationHorsFenetre,
    /// Délégation incohérente avec l'annuaire (génération déjà certifiée pour une
    /// autre clé, clé déjà déléguée pour un autre usage, COG sans racine épinglée…).
    DelegationIncoherente,
    /// Racine incohérente (`cog_id` ≠ empreinte de la clé d'identité) ou mélange
    /// d'ancres directes et de racine pour un même COG.
    RacineIncoherente,
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
            Self::SansDelegation => write!(f, "clé d'usage sans délégation valide"),
            Self::UsageIncorrect { attendu, delegue } => {
                write!(f, "clé déléguée pour l'usage {delegue}, usage {attendu} attendu")
            }
            Self::DelegationHorsFenetre => write!(f, "délégation hors de sa fenêtre de validité"),
            Self::DelegationIncoherente => write!(f, "délégation incohérente avec l'annuaire"),
            Self::RacineIncoherente => write!(f, "racine incohérente"),
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

/// Clé d'usage certifiée par une délégation `0x45` vérifiée.
#[derive(Debug, Clone)]
struct CleDeleguee {
    cog: CogId,
    niveau: NiveauConfiance,
    delegation: DelegationCle,
}

/// Clés signataires de passeports et de visas, chacune rattachée à son `cog_id`.
///
/// Deux modes, par COG :
///
/// - **Chaîne (A-022 §2)** : racine épinglée ([`Self::epingler_racine`]) et
///   clés d'usage certifiées par délégation `0x45` ([`Self::ajouter_delegation`]).
///   Une clé n'est acceptée que pour l'indice que sa délégation certifie
///   (1 passeports, 5 visas) et dans la fenêtre de celle-ci.
/// - **Ancre directe (transition)** : clé épinglée sans délégation
///   ([`Self::ajouter`]), acceptée pour passeports et visas tant que le COG n'a
///   pas de racine épinglée et que le mode strict ([`Self::exiger_delegations`])
///   n'est pas actif.
#[derive(Debug, Clone, Default)]
pub struct AnnuaireEmetteurs {
    ancres: AncresDeConfiance,
    cogs: HashMap<Kid, (CogId, NiveauConfiance)>,
    racines: HashMap<CogId, ([u8; 32], NiveauConfiance)>,
    deleguees: HashMap<Kid, CleDeleguee>,
    strict: bool,
}

impl AnnuaireEmetteurs {
    /// Annuaire vide (refuse tout).
    pub fn new() -> Self {
        Self::default()
    }

    /// **Transition** : épingle directement la clé `cle_publique` du COG `cog`
    /// (mode antérieur à A-022 : passeports et visas). `Souverain` pour son
    /// propre COG, `Reconnu` pour un COG reconnu. Refuse les clés faibles, et
    /// (`ErreurSceau::CleInvalide`) toute clé d'un COG dont la racine est
    /// épinglée, une clé déjà déléguée, ou tout ajout en mode strict.
    pub fn ajouter(&mut self, cog: CogId, cle_publique: &[u8; 32], niveau: NiveauConfiance) -> Result<Kid, ErreurSceau> {
        let kid = empreinte_cle(cle_publique);
        if self.strict || self.racines.contains_key(&cog) || self.deleguees.contains_key(&kid) {
            return Err(ErreurSceau::CleInvalide);
        }
        let kid = self.ancres.ajouter(cle_publique, niveau)?;
        self.cogs.insert(kid, (cog, niveau));
        Ok(kid)
    }

    /// Mode strict : toute ancre directe est refusée (à l'ajout comme à la
    /// vérification) ; seules les clés déléguées par une racine épinglée signent.
    pub fn exiger_delegations(&mut self) {
        self.strict = true;
    }

    /// Épingle la clé d'**identité** (indice 0) du COG `cog`. Exige
    /// `cog = cog_id(cle_identite)` et aucune ancre directe pour ce COG. La clé
    /// d'identité ne signe ni passeport ni visa : elle n'est pas une ancre de
    /// titres, seulement de délégations.
    pub fn epingler_racine(
        &mut self,
        cog: CogId,
        cle_identite: &[u8; 32],
        niveau: NiveauConfiance,
    ) -> Result<Kid, RefusIdentite> {
        if cog_id_depuis_cle(cle_identite) != cog.as_str() {
            return Err(RefusIdentite::RacineIncoherente);
        }
        // Clé valide et non faible (même contrôle que les ancres).
        let kid = AncresDeConfiance::new().ajouter(cle_identite, niveau)?;
        if self.cogs.values().any(|(c, _)| c == &cog) || self.cogs.contains_key(&kid) {
            return Err(RefusIdentite::RacineIncoherente);
        }
        if let Some((existante, _)) = self.racines.get(&cog) {
            if existante != cle_identite {
                return Err(RefusIdentite::RacineIncoherente);
            }
        }
        self.racines.insert(cog, (*cle_identite, niveau));
        Ok(kid)
    }

    /// Vérifie et enregistre une délégation `0x45` (sceau complet) d'un COG
    /// dont la racine est épinglée. Contrôles : signature de l'identité
    /// épinglée, `cog_id`, fenêtre couvrant `maintenant`, puis cohérence avec
    /// l'annuaire (une clé par `(cog, indice, génération)` ; une clé déléguée
    /// pour un seul `(cog, indice, génération)` ; jamais une ancre directe ni
    /// une clé d'identité). Rend le `kid` de la clé déléguée.
    pub fn ajouter_delegation(&mut self, octets: &[u8], maintenant: u64) -> Result<Kid, RefusIdentite> {
        let annoncee = decoder_titre::<DelegationCle>(&Sceau::decoder(octets)?.charge)?;
        let cog = CogId::new(annoncee.cog_id()).ok_or(RefusIdentite::DelegationIncoherente)?;
        let (cle_identite, niveau) = *self.racines.get(&cog).ok_or(RefusIdentite::DelegationIncoherente)?;
        let delegation = verifier_delegation(octets, &cle_identite, niveau, maintenant)?.into_titre();
        let kid = delegation.kid();
        if self.cogs.contains_key(&kid) || self.racines.values().any(|(r, _)| r == delegation.cle_publique()) {
            return Err(RefusIdentite::DelegationIncoherente);
        }
        if let Some(existante) = self.deleguees.get(&kid) {
            let d = &existante.delegation;
            if existante.cog != cog || d.indice() != delegation.indice() || d.generation() != delegation.generation() {
                return Err(RefusIdentite::DelegationIncoherente);
            }
        }
        let meme_generation_autre_cle = self.deleguees.iter().any(|(k, e)| {
            k != &kid
                && e.cog == cog
                && e.delegation.indice() == delegation.indice()
                && e.delegation.generation() == delegation.generation()
        });
        if meme_generation_autre_cle {
            return Err(RefusIdentite::DelegationIncoherente);
        }
        self.ancres.ajouter(delegation.cle_publique(), niveau)?;
        self.deleguees.insert(kid, CleDeleguee { cog, niveau, delegation });
        Ok(kid)
    }

    /// Copie de l'annuaire augmentée des délégations **présentées avec un
    /// titre** (mêmes contrôles que [`Self::ajouter_delegation`]) ; l'annuaire
    /// d'origine n'est pas modifié.
    pub fn avec_delegations(&self, delegations: &[&[u8]], maintenant: u64) -> Result<Self, RefusIdentite> {
        let mut copie = self.clone();
        for d in delegations {
            copie.ajouter_delegation(d, maintenant)?;
        }
        Ok(copie)
    }

    /// Retire une clé (rotation ou compromission), directe ou déléguée.
    pub fn retirer(&mut self, kid: &Kid) -> bool {
        self.cogs.remove(kid);
        self.deleguees.remove(kid);
        self.ancres.retirer(kid)
    }

    /// COG et niveau du signataire `kid` d'un titre d'usage `indice`
    /// (fenêtre `fenetre`), à `maintenant` ; refus si la chaîne ne tient pas.
    fn resoudre(
        &self,
        kid: &Kid,
        indice: u32,
        fenetre: (u64, u64),
        maintenant: u64,
    ) -> Result<(&CogId, NiveauConfiance), RefusIdentite> {
        if let Some(e) = self.deleguees.get(kid) {
            let d = &e.delegation;
            if d.indice() != indice {
                return Err(RefusIdentite::UsageIncorrect { attendu: indice, delegue: d.indice() });
            }
            if !d.couvre(maintenant) || fenetre.0 < d.pas_avant() || fenetre.1 > d.pas_apres() {
                return Err(RefusIdentite::DelegationHorsFenetre);
            }
            return Ok((&e.cog, e.niveau));
        }
        let (cog, niveau) = self.cogs.get(kid).ok_or(RefusIdentite::EmetteurIncoherent)?;
        if self.strict || self.racines.contains_key(cog) {
            return Err(RefusIdentite::SansDelegation);
        }
        Ok((cog, *niveau))
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

/// Émet (scelle) un passeport avec la clé d'émission du COG d'origine `emetteur`
/// (clé d'usage `emission`, indice 1, déléguée par `0x45` en mode chaîné).
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
    let (cog_ancre, niveau) = politique.annuaire.resoudre(
        &p.emetteur_kid,
        INDICE_USAGE_EMISSION,
        (p.pas_avant, p.pas_apres),
        maintenant,
    )?;
    if cog_ancre != &p.emetteur_cog {
        return Err(RefusIdentite::EmetteurIncoherent);
    }
    let local = p.emetteur_cog == politique.cog_local;
    if local != (niveau == NiveauConfiance::Souverain) {
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
/// est le COG hôte, `cle_emission` sa clé de visas : clé d'usage `visas`
/// (indice 5, déléguée par `0x45`) en mode chaîné, ou ancre directe pendant la
/// transition (voir l'en-tête du module). Le visa dure au plus 15 min, est
/// contenu dans la fenêtre du passeport, lié à son empreinte, au sujet, à la
/// clé d'appareil, à l'audience (service) et n'est émis qu'après preuve de
/// possession ([`PossessionProuvee`]).
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
    let (cog_ancre, niveau) =
        politique.annuaire.resoudre(signataire, INDICE_USAGE_VISAS, (v.pas_avant, v.pas_apres), maintenant)?;
    if cog_ancre != &politique.cog_local || niveau != NiveauConfiance::Souverain || v.emetteur_cog != politique.cog_local {
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
