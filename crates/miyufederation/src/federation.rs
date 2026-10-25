//! @id     miyufederation_federation
//! @do     gouverner_admission_et_sanctions_federales
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « federation » (vue fédérale locale : candidatures, confirmations, statuts, témoins, accusations, suspensions, recours, révocations).

//! Vue fédérale **locale** d'un COG (zones §16, EX-30..33, EX-35, EX-37, EX-38).
//!
//! « Don't trust, verify » : chaque COG revalide lui-même candidatures,
//! confirmations, têtes témoignées et accusations ; aucun message reçu ne fait
//! autorité par sa seule provenance. Origin est **facultatif** (EX-30) : il ne
//! fait qu'ajouter une contresignature de témoin ; tout fonctionne sans lui.
//!
//! Règles appliquées (fail-closed) :
//!
//! - **Candidature** : auto-signée par l'identité, preuve de calcul
//!   ≥ difficulté de la politique, liée à la racine et au jour, validité 48 h ;
//!   racine révoquée = refus définitif (retour seulement par nouvelle racine)
//! - **Confirmation** : signée par un **membre** (un invité ne confirme pas),
//!   jamais par soi-même, valable ≤ 24 h ; compte à partir de sa **réception**
//!   (pas d'antidatage de l'historique des 7 jours) ; quota de nouveaux COG
//!   confirmés par membre et par 30 jours
//! - **Statuts** : invité ≥ 2 confirmations de membres distincts ; membre
//!   ≥ 4 **continues** pendant 7 jours (compteur remis à zéro sous 4) ; membre
//!   sous 2 → invité ; fondateurs (graines de l'image) membres d'office
//! - **Témoins (anti-éclipse)** : une tête de registre d'un COG n'est crue que
//!   signée par sa clé de journal **et** contresignée par ≥ 3 membres distincts
//!   (clés de journal ; Origin compte comme **un** témoin s'il est configuré) ;
//!   règle unique `miyuregistre::verifier_temoins` (A-007, EX-34) ;
//!   monotonie et équivoque par `miyusceau::EtatRegistre`
//! - **Accusation** : un sceau, preuve signée par le mis en cause vérifiée
//!   localement, **quorum de 6 membres** distincts ; issue (A-007, EX-32) :
//!   **suspension immédiate** ([`Statut::Suspendu`], titres de l'accusé
//!   refusés partout), puis **révocation définitive après 72 h**
//!   ([`Federation::actualiser`], entrée de registre
//!   `Revocation { cible: Cle(kid) }`) sauf **recours accepté** (6 membres
//!   distincts hors accusé, même empreinte de preuve) ⇒ suspension levée,
//!   statut précédent rétabli, compteur des 7 jours repris de zéro
//!   (fail-closed) ; accusation levée = rejeu refusé ; preuve fausse = note
//!   locale des signataires (faux signalement)
//! - **Fraîcheur** : la vue fédérale (révocations) date de la dernière tête
//!   témoignée acceptée ; au-delà de 15 min, accès fédéré refusé
//!
//! **Aucune destruction à distance** (EX-33, INV-Z11) : la seule sanction est
//! [`Sanction::RevocationIdentite`].

use std::collections::{BTreeMap, BTreeSet};

use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{verifier_temoins, Cible, Corps, EntreeRegistre, ErreurJournal};
use miyusceau::{
    decoder_titre, empreinte_cle, sceller, verifier, verifier_seuil, verifier_tete, Acceptation, AncresDeConfiance,
    CleSignature, ErreurSceau, EtatRegistre, Kid, NiveauConfiance, PolitiqueTete, PreuveCoherence, Sceau,
    TeteRegistre, AGE_MAX_TETE_DEFAUT_SECS, MAX_SIGNATURES, TOLERANCE_HORLOGE_SECS,
};

use crate::calcul::{DIFFICULTE_DEFAUT, DIFFICULTE_MAX, DIFFICULTE_PLANCHER};
use crate::eclipse::{reseau_valide, CarnetPairs, MAX_PAIRS_CARNET};
use crate::erreurs::ErreurFederation;
use crate::note::{NoteLocale, Observation};
use crate::preuve::Preuve;
use crate::statut::{
    Sanction, Statut, CONFIRMATIONS_INVITE, CONFIRMATIONS_MEMBRE, DELAI_CONTESTATION_SECS, DUREE_MEMBRE_SECS,
    QUORUM_BLACKLIST, QUORUM_RECOURS,
};
use crate::titres::{Accusation, Candidature, Confirmation, Recours};

mod etat;

pub use etat::TAILLE_MAX_ETAT;
/// Version du format `MFED` écrite (un état d'une version antérieure lue est réécrit).
pub(crate) const VERSION_ECRITE: u8 = etat::VERSION;

/// Nouveaux COG confirmés au plus par membre et par période (choix à amender :
/// la doc dit « limité par membre et par mois » sans valeur).
pub const QUOTA_CONFIRMATIONS: usize = 8;
/// Période du quota (30 jours).
pub const PERIODE_QUOTA_SECS: u64 = 30 * 86_400;
/// Témoins minimaux d'une tête : ≥ 3 membres distincts, Origin = un témoin
/// au plus (A-007, EX-34) ; même règle que `miyuregistre::federal`.
pub const TEMOINS_MIN: usize = miyuregistre::TEMOINS_MIN;
/// Historique de confirmations conservé (7 j + 1 j de marge).
const HISTORIQUE_SECS: u64 = DUREE_MEMBRE_SECS + 86_400;

/// Politique locale de la fédération. Les seuils de la doc gelée (2, 4, 7 j,
/// quorum 6) sont des constantes non réglables ; seuls peuvent être **durcis**
/// (ou, pour la difficulté, réglés au-dessus d'un plancher) les choix non fixés.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parametres {
    difficulte_min: u8,
    temoins_min: usize,
    fraicheur_vue_secs: u64,
    age_max_tete_secs: u64,
}

impl Default for Parametres {
    fn default() -> Self {
        Self {
            difficulte_min: DIFFICULTE_DEFAUT,
            temoins_min: TEMOINS_MIN,
            fraicheur_vue_secs: FRAICHEUR_MAX_DEFAUT_SECS,
            age_max_tete_secs: AGE_MAX_TETE_DEFAUT_SECS,
        }
    }
}

impl Parametres {
    /// Difficulté minimale de la preuve de calcul (`8..=40`).
    pub fn avec_difficulte(mut self, bits: u8) -> Result<Self, ErreurFederation> {
        if !(DIFFICULTE_PLANCHER..=DIFFICULTE_MAX).contains(&bits) {
            return Err(ErreurFederation::ParametreInvalide("difficulté"));
        }
        self.difficulte_min = bits;
        Ok(self)
    }

    /// Témoins exigés (`3..=7` : la tête porte aussi la signature du propriétaire).
    pub fn avec_temoins(mut self, n: usize) -> Result<Self, ErreurFederation> {
        if !(TEMOINS_MIN..MAX_SIGNATURES).contains(&n) {
            return Err(ErreurFederation::ParametreInvalide("témoins"));
        }
        self.temoins_min = n;
        Ok(self)
    }

    /// Fraîcheur de la vue fédérale, **plus courte** seulement (1 s à 15 min).
    pub fn avec_fraicheur(mut self, secs: u64) -> Result<Self, ErreurFederation> {
        if secs == 0 || secs > FRAICHEUR_MAX_DEFAUT_SECS {
            return Err(ErreurFederation::ParametreInvalide("fraîcheur"));
        }
        self.fraicheur_vue_secs = secs;
        Ok(self)
    }

    /// Difficulté minimale.
    pub fn difficulte_min(&self) -> u8 {
        self.difficulte_min
    }

    /// Témoins exigés.
    pub fn temoins_min(&self) -> usize {
        self.temoins_min
    }

    /// Fraîcheur maximale de la vue fédérale (secondes).
    pub fn fraicheur_vue_secs(&self) -> u64 {
        self.fraicheur_vue_secs
    }
}

/// Préfixe du motif des entrées de registre de révocation fédérale
/// ([`RevocationFederale::entree`]) : sert à les reconnaître au registre.
pub const MOTIF_REVOCATION_FEDERALE: &str = "blacklist federale ";

/// Révocation fédérale décidée localement après vérification du quorum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationFederale {
    /// Clé publique d'identité révoquée.
    pub racine: [u8; 32],
    /// `kid` de la clé d'identité (cible de l'entrée de registre).
    pub kid: Kid,
    /// `cog_id` révoqué.
    pub cog: CogId,
    /// Identité de l'accusation (SHA-256 de ses octets signés).
    pub accusation: [u8; 32],
    /// Heure de la décision locale.
    pub revoque_a: u64,
    /// Sanction appliquée (toujours la révocation d'identité).
    pub sanction: Sanction,
}

impl RevocationFederale {
    /// Entrée de registre (lot 2 §8.3, nature révocation, cible clé).
    pub fn entree(&self) -> EntreeRegistre {
        use std::fmt::Write;
        let mut motif = String::from(MOTIF_REVOCATION_FEDERALE);
        for o in &self.accusation[..8] {
            let _ = write!(motif, "{o:02x}");
        }
        EntreeRegistre { inscrite_a: self.revoque_a, corps: Corps::Revocation { cible: Cible::Cle(self.kid), motif } }
    }

    /// Octets de l'entrée scellée par la clé de journal du COG local.
    pub fn sceller_entree(&self, journal: &CleSignature) -> Result<Vec<u8>, ErreurFederation> {
        Ok(sceller(&self.entree(), journal)?)
    }
}

/// Suspension fédérale en cours (accusation au quorum, EX-32, A-007) :
/// titres de l'accusé refusés, révocation définitive à [`Self::echeance`]
/// sauf recours accepté.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuspensionFederale {
    /// Clé publique d'identité suspendue.
    pub racine: [u8; 32],
    /// `kid` de la clé d'identité.
    pub kid: Kid,
    /// `cog_id` suspendu.
    pub cog: CogId,
    /// Identité de l'accusation (SHA-256 de ses octets signés).
    pub accusation: [u8; 32],
    /// Type de preuve de l'accusation.
    pub type_preuve: u64,
    /// Empreinte de la preuve de l'accusation.
    pub empreinte_preuve: [u8; 32],
    /// Heure locale de la suspension.
    pub depuis: u64,
}

impl SuspensionFederale {
    /// Heure de la révocation définitive sans recours accepté (`depuis` + 72 h).
    pub fn echeance(&self) -> u64 {
        self.depuis.saturating_add(DELAI_CONTESTATION_SECS)
    }
}

/// Pièces signées d'une sanction, conservées pour la persistance : elles
/// sont **revérifiées** au chargement ([`etat`]).
#[derive(Debug, Clone)]
struct Pieces {
    /// Octets de l'accusation reçue (sceau cosigné, quorum 6).
    accusation: Vec<u8>,
    /// Preuve de faute.
    preuve: Preuve,
}

/// Suspension et statut à rétablir si un recours est accepté.
#[derive(Debug, Clone)]
struct EnSuspension {
    publique: SuspensionFederale,
    statut: Statut,
    membre: bool,
    pieces: Pieces,
}

/// Révocation définitive : pièces et début de la suspension qui l'a précédée.
#[derive(Debug, Clone)]
struct PiecesRevocation {
    pieces: Pieces,
    depuis: u64,
}

/// Accusation levée par un recours accepté (rejeu refusé).
#[derive(Debug, Clone)]
struct Levee {
    accuse: Kid,
    pieces: Pieces,
    /// Octets du recours accepté (sceau cosigné, quorum 6).
    recours: Vec<u8>,
    depuis: u64,
    /// Heure locale de la levée.
    instant: u64,
}

/// Confirmation reçue, conservée (fenêtre d'historique) pour la persistance.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TitreRecu {
    recue_a: u64,
    expire_a: u64,
    octets: Vec<u8>,
}

/// Confirmations conservées au plus par couple (confirmé, confirmateur) ;
/// au-delà, les plus anciennes sont oubliées (fail-closed : une reprise ne
/// peut qu'y perdre de la couverture, jamais en gagner).
const MAX_TITRES_PAR_CONFIRMATEUR: usize = 64;

#[derive(Debug, Clone)]
struct Dossier {
    racine: [u8; 32],
    cog: CogId,
    journal: [u8; 32],
    registre: [u8; 32],
    fondateur: bool,
    /// Heure locale d'inscription (vérification de la candidature).
    inscrit_a: u64,
    /// Candidature signée d'origine (revérifiée au chargement).
    candidature: Vec<u8>,
    statut: Statut,
    /// Statut membre acquis (collant tant qu'il reste ≥ 2 confirmations).
    membre: bool,
    /// Intervalles de validité des confirmations, par membre confirmateur.
    confirmations: BTreeMap<Kid, Vec<(u64, u64)>>,
    /// Confirmations signées d'origine, par membre confirmateur.
    titres: BTreeMap<Kid, Vec<TitreRecu>>,
    /// Dernière tête témoignée acceptée (anti-éclipse).
    vue: Option<EtatRegistre>,
    /// Dernière tête que le COG local a contresignée comme témoin.
    temoignage: Option<EtatRegistre>,
}

#[derive(Debug, Clone, Copy)]
struct Quota {
    confirme: Kid,
    premiere: u64,
    derniere: u64,
}

/// Vue fédérale locale.
#[derive(Debug, Clone)]
pub struct Federation {
    parametres: Parametres,
    origin: Option<[u8; 32]>,
    dossiers: BTreeMap<Kid, Dossier>,
    par_cog: BTreeMap<CogId, Kid>,
    revocations: BTreeMap<Kid, RevocationFederale>,
    suspensions: BTreeMap<Kid, EnSuspension>,
    /// Pièces des révocations définitives.
    pieces_revocations: BTreeMap<Kid, PiecesRevocation>,
    /// Accusations levées par un recours accepté (identités) : rejeu refusé.
    levees: BTreeMap<[u8; 32], Levee>,
    /// Révocations définitives **à publier** (à inscrire au registre local) :
    /// purgées seulement par [`Federation::acquitter`] (M2, persistées).
    a_publier: BTreeSet<Kid>,
    /// Révocations fédérales constatées au registre local sans les pièces
    /// (reprise d'un état plus ancien que le registre, M3) : `kid` ⇒ heure
    /// d'inscription. Définitives, comme [`Self::revocations`].
    inscrites: BTreeMap<Kid, u64>,
    quotas: BTreeMap<Kid, Vec<Quota>>,
    heure: u64,
    vue_a_jour_au: Option<u64>,
    note: NoteLocale,
    /// Carnet de pairs anti-éclipse (EX-37), persisté avec la vue.
    carnet: CarnetPairs,
}

fn cog_de(racine: &[u8; 32]) -> Result<CogId, ErreurFederation> {
    CogId::new(&miyuracine::cog_id_depuis_cle(racine)).ok_or(ErreurFederation::ParametreInvalide("cog_id"))
}

fn ancre(cle: &[u8; 32]) -> Result<AncresDeConfiance, ErreurFederation> {
    let mut a = AncresDeConfiance::new();
    a.ajouter(cle, NiveauConfiance::Reconnu)?;
    Ok(a)
}

/// Nombre de confirmateurs dont un intervalle couvre `t`.
fn couverture(intervalles: &[&Vec<(u64, u64)>], t: u64) -> usize {
    intervalles.iter().filter(|iv| iv.iter().any(|&(s, e)| s <= t && t <= e)).count()
}

/// Début de la plage continue se terminant à `maintenant` où la couverture
/// est ≥ `seuil` (`None` si elle ne l'est pas à `maintenant`).
fn depuis_seuil(intervalles: &[&Vec<(u64, u64)>], seuil: usize, maintenant: u64) -> Option<u64> {
    if couverture(intervalles, maintenant) < seuil {
        return None;
    }
    // La couverture ne change qu'aux débuts `s` et aux lendemains de fin `e + 1`.
    let mut changements: Vec<u64> = intervalles
        .iter()
        .flat_map(|iv| iv.iter().flat_map(|&(s, e)| [s, e.saturating_add(1)]))
        .filter(|c| *c <= maintenant)
        .collect();
    changements.sort_unstable();
    changements.dedup();
    let mut debut = maintenant;
    for &c in changements.iter().rev() {
        if couverture(intervalles, c) >= seuil {
            debut = c;
        } else {
            break;
        }
    }
    Some(debut)
}

fn fusionner(intervalles: &mut Vec<(u64, u64)>, nouveau: (u64, u64), oubli_avant: u64) {
    intervalles.push(nouveau);
    intervalles.sort_unstable();
    let mut fusion: Vec<(u64, u64)> = Vec::with_capacity(intervalles.len());
    for &(s, e) in intervalles.iter() {
        match fusion.last_mut() {
            Some(dernier) if s <= dernier.1.saturating_add(1) => dernier.1 = dernier.1.max(e),
            _ => fusion.push((s, e)),
        }
    }
    fusion.retain(|&(_, e)| e >= oubli_avant);
    *intervalles = fusion;
}

impl Federation {
    /// Vue vide. `origin` = clé de journal d'Origin comme témoin facultatif
    /// (`None` : fédération sans Origin, EX-30).
    pub fn new(parametres: Parametres, origin: Option<[u8; 32]>) -> Self {
        Self {
            parametres,
            origin,
            dossiers: BTreeMap::new(),
            par_cog: BTreeMap::new(),
            revocations: BTreeMap::new(),
            suspensions: BTreeMap::new(),
            pieces_revocations: BTreeMap::new(),
            levees: BTreeMap::new(),
            a_publier: BTreeSet::new(),
            inscrites: BTreeMap::new(),
            quotas: BTreeMap::new(),
            heure: 0,
            vue_a_jour_au: None,
            note: NoteLocale::new(),
            carnet: CarnetPairs::new(),
        }
    }

    /// Politique.
    pub fn parametres(&self) -> &Parametres {
        &self.parametres
    }

    /// Note locale des pairs (privée au COG, jamais partagée).
    pub fn note(&self) -> &NoteLocale {
        &self.note
    }

    /// Enregistre une observation locale sur un pair (disponibilité, faute de protocole).
    pub fn observer(&mut self, cog: &CogId, observation: Observation, maintenant: u64) -> Result<(), ErreurFederation> {
        let kid = *self.par_cog.get(cog).ok_or(ErreurFederation::Inconnu)?;
        self.note.observer(&kid, observation, maintenant);
        Ok(())
    }

    /// Carnet de pairs anti-éclipse de la vue (persisté avec elle).
    pub fn carnet(&self) -> &CarnetPairs {
        &self.carnet
    }

    /// Note un contact avec `cog` depuis `reseau` dans le carnet de la vue
    /// (premier contact et premier réseau conservés). Carnet plein
    /// ([`MAX_PAIRS_CARNET`]) : le pair **inéligible** le plus récent est
    /// évincé ; s'il n'y en a pas, le nouveau pair est ignoré (`Ok(false)`) —
    /// un afflux de nouveaux venus ne chasse jamais les anciens pairs
    /// éligibles (EX-37). Étiquette invalide : `Ok(false)`.
    pub fn noter_contact(&mut self, cog: &CogId, reseau: &str, maintenant: u64) -> Result<bool, ErreurFederation> {
        self.horloge(maintenant)?;
        if !reseau_valide(reseau) {
            return Ok(false);
        }
        if !self.carnet.contient(cog) && self.carnet.len() >= MAX_PAIRS_CARNET {
            let victime = self
                .carnet
                .pairs()
                .filter(|(c, _, _)| !self.pair_eligible(c, maintenant))
                .max_by_key(|(c, _, t)| (*t, (*c).clone()))
                .map(|(c, _, _)| c.clone());
            match victime {
                Some(v) => self.carnet.retirer(&v),
                None => return Ok(false),
            }
        }
        Ok(self.carnet.noter_contact(cog, reseau, maintenant))
    }

    /// Choisit au plus `n` pairs éligibles du carnet de la vue (réseaux
    /// alternés, anciens d'abord : [`CarnetPairs::choisir`]).
    pub fn choisir_pairs(&self, n: usize, maintenant: u64) -> Vec<CogId> {
        self.carnet.choisir(n, self, maintenant)
    }

    fn horloge(&mut self, maintenant: u64) -> Result<(), ErreurFederation> {
        if maintenant < self.heure {
            return Err(ErreurFederation::RetourHorloge);
        }
        self.heure = maintenant;
        Ok(())
    }

    fn est_revoque(&self, kid: &Kid) -> bool {
        self.revocations.contains_key(kid) || self.inscrites.contains_key(kid)
    }

    fn est_suspendu(&self, kid: &Kid) -> bool {
        self.suspensions.contains_key(kid)
    }

    fn est_membre(&self, kid: &Kid) -> bool {
        !self.est_revoque(kid) && !self.est_suspendu(kid) && self.dossiers.get(kid).is_some_and(|d| d.statut == Statut::Membre)
    }

    // ── Candidatures ────────────────────────────────────────────────────────

    /// Vérifie une candidature (signature de l'identité, fenêtre, preuve de calcul).
    pub fn verifier_candidature(&self, octets: &[u8], maintenant: u64) -> Result<Candidature, ErreurFederation> {
        let brute = decoder_titre::<Candidature>(&Sceau::decoder(octets)?.charge)?;
        let c = verifier::<Candidature>(octets, &ancre(&brute.racine)?, maintenant)?.into_titre();
        if c.difficulte < self.parametres.difficulte_min || !c.preuve_valide() {
            return Err(ErreurFederation::CalculInsuffisant);
        }
        Ok(c)
    }

    fn inscrire(&mut self, octets: &[u8], fondateur: bool, maintenant: u64) -> Result<CogId, ErreurFederation> {
        self.horloge(maintenant)?;
        let c = self.verifier_candidature(octets, maintenant)?;
        let kid = empreinte_cle(&c.racine);
        if self.est_revoque(&kid) {
            return Err(ErreurFederation::Revoque);
        }
        if let Some(d) = self.dossiers.get(&kid) {
            return if d.journal == c.journal && d.registre == c.registre && d.fondateur == fondateur {
                Ok(d.cog.clone())
            } else {
                Err(ErreurFederation::Contradictoire)
            };
        }
        // Une clé ne sert qu'à un usage et à un COG.
        let reemploi = self.dossiers.values().any(|d| {
            [d.racine, d.journal].contains(&c.journal) || d.journal == c.racine || d.registre == c.registre
        }) || self.origin == Some(c.journal);
        if reemploi {
            return Err(ErreurFederation::Contradictoire);
        }
        let cog = cog_de(&c.racine)?;
        self.par_cog.insert(cog.clone(), kid);
        self.dossiers.insert(
            kid,
            Dossier {
                racine: c.racine,
                cog: cog.clone(),
                journal: c.journal,
                registre: c.registre,
                fondateur,
                inscrit_a: maintenant,
                candidature: octets.to_vec(),
                statut: if fondateur { Statut::Membre } else { Statut::Candidat },
                membre: fondateur,
                confirmations: BTreeMap::new(),
                titres: BTreeMap::new(),
                vue: None,
                temoignage: None,
            },
        );
        Ok(cog)
    }

    /// Membre fondateur (graine de l'image, configuration locale) : membre d'office.
    pub fn ajouter_fondateur(&mut self, candidature: &[u8], maintenant: u64) -> Result<CogId, ErreurFederation> {
        self.inscrire(candidature, true, maintenant)
    }

    /// Candidature reçue (commérage) : statut candidat.
    pub fn recevoir_candidature(&mut self, candidature: &[u8], maintenant: u64) -> Result<CogId, ErreurFederation> {
        self.inscrire(candidature, false, maintenant)
    }

    // ── Confirmations et statuts ────────────────────────────────────────────

    /// Confirmation reçue (commérage). Le statut est recalculé par [`Self::actualiser`].
    pub fn recevoir_confirmation(&mut self, octets: &[u8], maintenant: u64) -> Result<(), ErreurFederation> {
        self.horloge(maintenant)?;
        let brute = decoder_titre::<Confirmation>(&Sceau::decoder(octets)?.charge)?;
        let kid_m = empreinte_cle(&brute.confirmatrice);
        if self.est_revoque(&kid_m) {
            return Err(ErreurFederation::Revoque);
        }
        if self.est_suspendu(&kid_m) {
            return Err(ErreurFederation::Suspendu);
        }
        if !self.est_membre(&kid_m) {
            return Err(ErreurFederation::NonMembre);
        }
        if self.note.est_banni(&kid_m, maintenant) {
            return Err(ErreurFederation::Banni);
        }
        let c = verifier::<Confirmation>(octets, &ancre(&brute.confirmatrice)?, maintenant)?.into_titre();
        let kid_c = empreinte_cle(&c.reconnue);
        if self.est_revoque(&kid_c) {
            return Err(ErreurFederation::Revoque);
        }
        if !self.dossiers.contains_key(&kid_c) {
            return Err(ErreurFederation::Inconnu);
        }
        let quota = self.quotas.entry(kid_m).or_default();
        quota.retain(|q| q.derniere.saturating_add(PERIODE_QUOTA_SECS) >= maintenant);
        if let Some(q) = quota.iter_mut().find(|q| q.confirme == kid_c) {
            q.derniere = maintenant;
        } else {
            let recents =
                quota.iter().filter(|q| q.premiere.saturating_add(PERIODE_QUOTA_SECS) >= maintenant).count();
            if recents >= QUOTA_CONFIRMATIONS {
                return Err(ErreurFederation::QuotaDepasse);
            }
            quota.push(Quota { confirme: kid_c, premiere: maintenant, derniere: maintenant });
        }
        if let Some(d) = self.dossiers.get_mut(&kid_c) {
            d.ajouter_confirmation(kid_m, &c, octets, maintenant);
        }
        Ok(())
    }

    /// Fait avancer la vue à `maintenant` (horloge fournie par l'appelant) :
    /// les suspensions échues (72 h sans recours accepté) deviennent des
    /// révocations définitives, renvoyées (entrée de registre :
    /// [`RevocationFederale::entree`]) ; puis les statuts sont recalculés
    /// (point fixe : seules les confirmations de membres actuels comptent).
    pub fn actualiser(&mut self, maintenant: u64) -> Result<Vec<RevocationFederale>, ErreurFederation> {
        self.horloge(maintenant)?;
        let definitives = self.echeances(maintenant);
        self.recalculer(maintenant);
        Ok(definitives)
    }

    /// Suspensions échues à `maintenant` ⇒ révocations définitives.
    fn echeances(&mut self, maintenant: u64) -> Vec<RevocationFederale> {
        let echues: Vec<Kid> = self
            .suspensions
            .iter()
            .filter(|(_, s)| maintenant >= s.publique.echeance())
            .map(|(k, _)| *k)
            .collect();
        let mut definitives = Vec::with_capacity(echues.len());
        for kid in echues {
            let Some(en) = self.suspensions.remove(&kid) else { continue };
            let s = en.publique;
            self.pieces_revocations.insert(kid, PiecesRevocation { pieces: en.pieces, depuis: s.depuis });
            let revocation = RevocationFederale {
                racine: s.racine,
                kid,
                cog: s.cog,
                accusation: s.accusation,
                revoque_a: maintenant,
                sanction: Sanction::RevocationIdentite,
            };
            self.revocations.insert(kid, revocation.clone());
            // À publier jusqu'à l'accusé d'inscription au registre (M2).
            self.a_publier.insert(kid);
            self.marquer_revoque(&kid, maintenant);
            definitives.push(revocation);
        }
        if !definitives.is_empty() {
            self.recalculer(maintenant);
        }
        definitives
    }

    /// Dossier révoqué : statut, drapeau membre, confirmateurs responsables
    /// (le COG confirmé est blacklisté).
    fn marquer_revoque(&mut self, kid: &Kid, maintenant: u64) {
        if let Some(d) = self.dossiers.get_mut(kid) {
            d.statut = Statut::Revoque;
            d.membre = false;
            for c in d.confirmations.keys() {
                self.note.observer(c, Observation::ConfirmateurResponsable, maintenant);
            }
        }
    }

    /// Révocations définitives **à publier** (non acquittées), à inscrire au
    /// registre local ([`RevocationFederale::entree`]) puis à acquitter
    /// ([`Self::acquitter`]). La file est persistée avec l'état (`MFED`) :
    /// un plantage entre la décision et l'inscription ne perd rien.
    pub fn revocations_a_publier(&self) -> Vec<RevocationFederale> {
        self.a_publier.iter().filter_map(|k| self.revocations.get(k)).cloned().collect()
    }

    /// Accusé d'inscription au registre : retire `kid` de la file des
    /// révocations à publier. Renvoie `false` si elle ne l'y trouvait pas.
    /// À n'appeler qu'**après** l'inscription durable (puis persister l'état ;
    /// voir `StockageFederal::acquitter`, qui vérifie l'entrée au registre).
    pub fn acquitter(&mut self, kid: &Kid) -> bool {
        self.a_publier.remove(kid)
    }

    /// Révocations fédérales constatées au registre local sans les pièces
    /// (`kid`, heure d'inscription) : voir `StockageFederal::charger`.
    pub fn revocations_inscrites(&self) -> impl Iterator<Item = (&Kid, &u64)> {
        self.inscrites.iter()
    }

    /// Ré-applique une révocation fédérale **déjà inscrite** au registre
    /// local (reprise d'un état plus ancien, M3). Sans effet si `kid` est
    /// déjà révoqué. Suspension en cours échue ⇒ révocation complète (pièces
    /// conservées), datée de l'inscription bornée à `[échéance, heure]` ;
    /// sinon révocation sans pièces ([`Self::revocations_inscrites`]), même
    /// pour un `kid` encore inconnu (sa candidature sera refusée). Jamais
    /// remise dans la file à publier (elle est déjà au registre).
    pub(crate) fn appliquer_revocation_inscrite(&mut self, kid: Kid, inscrite_a: u64) -> bool {
        if self.est_revoque(&kid) {
            return false;
        }
        let heure = self.heure;
        match self.suspensions.get(&kid).map(|s| s.publique.echeance()) {
            Some(echeance) if echeance <= heure => {
                let Some(en) = self.suspensions.remove(&kid) else { return false };
                let s = en.publique;
                self.pieces_revocations.insert(kid, PiecesRevocation { pieces: en.pieces, depuis: s.depuis });
                let revocation = RevocationFederale {
                    racine: s.racine,
                    kid,
                    cog: s.cog,
                    accusation: s.accusation,
                    revoque_a: inscrite_a.clamp(echeance, heure),
                    sanction: Sanction::RevocationIdentite,
                };
                self.revocations.insert(kid, revocation);
            }
            _ => {
                self.suspensions.remove(&kid);
                self.inscrites.insert(kid, inscrite_a.min(heure));
            }
        }
        self.marquer_revoque(&kid, heure);
        self.recalculer(heure);
        true
    }

    fn recalculer(&mut self, maintenant: u64) {
        for _ in 0..=self.dossiers.len() {
            let membres: BTreeSet<Kid> = self
                .dossiers
                .iter()
                .filter(|(k, d)| d.statut == Statut::Membre && !self.est_revoque(k))
                .map(|(k, _)| *k)
                .collect();
            let mut changements = Vec::new();
            for (kid, d) in &self.dossiers {
                let (statut, membre) = if self.est_revoque(kid) {
                    (Statut::Revoque, false)
                } else if let Some(s) = self.suspensions.get(kid) {
                    (Statut::Suspendu { depuis: s.publique.depuis, accusation: s.publique.accusation }, false)
                } else if d.fondateur {
                    (Statut::Membre, true)
                } else {
                    let iv: Vec<&Vec<(u64, u64)>> = d
                        .confirmations
                        .iter()
                        .filter(|(m, _)| *m != kid && membres.contains(*m))
                        .map(|(_, v)| v)
                        .collect();
                    let n = couverture(&iv, maintenant);
                    let continu = depuis_seuil(&iv, CONFIRMATIONS_MEMBRE, maintenant)
                        .is_some_and(|t| maintenant - t >= DUREE_MEMBRE_SECS);
                    let membre = (d.membre && n >= CONFIRMATIONS_INVITE) || continu;
                    let statut = if membre {
                        Statut::Membre
                    } else if n >= CONFIRMATIONS_INVITE {
                        Statut::Invite
                    } else {
                        Statut::Candidat
                    };
                    (statut, membre)
                };
                if statut != d.statut || membre != d.membre {
                    changements.push((*kid, statut, membre));
                }
            }
            if changements.is_empty() {
                return;
            }
            for (kid, statut, membre) in changements {
                if let Some(d) = self.dossiers.get_mut(&kid) {
                    d.statut = statut;
                    d.membre = membre;
                }
            }
        }
    }

    /// Statut d'un COG (`None` : inconnu).
    pub fn statut(&self, cog: &CogId) -> Option<Statut> {
        let kid = self.par_cog.get(cog)?;
        if self.est_revoque(kid) {
            return Some(Statut::Revoque);
        }
        self.dossiers.get(kid).map(|d| d.statut)
    }

    /// `cog_id` des membres actuels.
    pub fn membres(&self) -> Vec<CogId> {
        self.dossiers.iter().filter(|(k, _)| self.est_membre(k)).map(|(_, d)| d.cog.clone()).collect()
    }

    /// Clé de journal annoncée par un COG (sa candidature).
    pub fn journal_de(&self, cog: &CogId) -> Option<[u8; 32]> {
        self.dossiers.get(self.par_cog.get(cog)?).map(|d| d.journal)
    }

    /// Pair éligible à un contact (statut invité ou membre, ni révoqué ni banni).
    /// N'exige pas de vue fraîche : c'est en contactant des pairs qu'on la rafraîchit.
    pub fn pair_eligible(&self, cog: &CogId, maintenant: u64) -> bool {
        self.par_cog.get(cog).is_some_and(|k| {
            !self.est_revoque(k)
                && !self.note.est_banni(k, maintenant)
                && self.dossiers.get(k).is_some_and(|d| d.statut.acces_federe())
        })
    }

    /// Accès fédéré d'un COG (EX-35 : vue fraîche exigée).
    pub fn acces_federe(&self, cog: &CogId, maintenant: u64) -> Result<Statut, ErreurFederation> {
        let kid = self.par_cog.get(cog).ok_or(ErreurFederation::Inconnu)?;
        if self.est_revoque(kid) {
            return Err(ErreurFederation::Revoque);
        }
        if self.est_suspendu(kid) {
            return Err(ErreurFederation::Suspendu);
        }
        self.controler_vue(maintenant).map_err(|_| ErreurFederation::VuePerimee)?;
        if self.note.est_banni(kid, maintenant) {
            return Err(ErreurFederation::Banni);
        }
        let statut = self.dossiers.get(kid).map_or(Statut::Candidat, |d| d.statut);
        if statut.acces_federe() {
            Ok(statut)
        } else {
            Err(ErreurFederation::StatutInsuffisant(statut))
        }
    }

    // ── Têtes témoignées (anti-éclipse) ─────────────────────────────────────

    fn politique_tete(&self) -> Result<PolitiqueTete, ErreurFederation> {
        Ok(PolitiqueTete::nouvelle(1, self.parametres.age_max_tete_secs)?)
    }

    fn dossier_actif(&self, cog: &CogId, maintenant: u64) -> Result<(Kid, &Dossier), ErreurFederation> {
        let kid = *self.par_cog.get(cog).ok_or(ErreurFederation::Inconnu)?;
        if self.est_revoque(&kid) {
            return Err(ErreurFederation::Revoque);
        }
        if self.est_suspendu(&kid) {
            return Err(ErreurFederation::Suspendu);
        }
        if self.note.est_banni(&kid, maintenant) {
            return Err(ErreurFederation::Banni);
        }
        let d = self.dossiers.get(&kid).ok_or(ErreurFederation::Inconnu)?;
        Ok((kid, d))
    }

    /// Le COG local contresigne, comme témoin, une tête du COG `cog` (clé de
    /// journal locale). Refuse toute tête contradictoire avec une tête déjà
    /// témoignée (preuve d'équivoque rendue).
    pub fn temoigner(
        &mut self,
        cog: &CogId,
        octets: &[u8],
        coherence: Option<&PreuveCoherence>,
        journal_local: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurFederation> {
        self.horloge(maintenant)?;
        let politique = self.politique_tete()?;
        let (kid, d) = self.dossier_actif(cog, maintenant)?;
        if journal_local.cle_publique() == d.journal {
            return Err(ErreurFederation::AutoConfirmation);
        }
        let tv = verifier_tete(octets, &ancre(&d.journal)?, &politique, &d.registre, maintenant)?;
        let mut etat = d.temoignage.clone();
        let resultat = match etat.as_mut() {
            None => {
                etat = Some(EtatRegistre::initial(tv));
                Ok(())
            }
            Some(e) => e.accepter(tv, coherence).map(|_| ()),
        };
        if let Err(e) = resultat {
            let e = ErreurFederation::from(e);
            if matches!(e, ErreurFederation::Equivoque(_)) {
                self.note.observer(&kid, Observation::ViolationGrave, maintenant);
            }
            return Err(e);
        }
        let contresignee = miyusceau::cosigner::<TeteRegistre>(octets, journal_local)?;
        if let Some(d) = self.dossiers.get_mut(&kid) {
            d.temoignage = etat;
        }
        Ok(contresignee)
    }

    /// Tête du COG `cog` reçue : crue seulement si signée par sa clé de
    /// journal et contresignée par ≥ `temoins_min` membres distincts (Origin
    /// compte comme un témoin s'il est configuré). Rafraîchit la vue fédérale.
    pub fn recevoir_tete(
        &mut self,
        cog: &CogId,
        octets: &[u8],
        coherence: Option<&PreuveCoherence>,
        maintenant: u64,
    ) -> Result<Acceptation, ErreurFederation> {
        self.horloge(maintenant)?;
        let politique = self.politique_tete()?;
        let (kid, d) = self.dossier_actif(cog, maintenant)?;
        let tv = verifier_tete(octets, &ancre(&d.journal)?, &politique, &d.registre, maintenant)?;
        let emise_a = tv.tete().emise_a;
        // Monotonie d'abord : une équivoque signée par le propriétaire est une
        // preuve, même si les témoins manquent.
        let mut etat = d.vue.clone();
        let acceptation = match etat.as_mut() {
            None => {
                etat = Some(EtatRegistre::initial(tv));
                Ok(Acceptation::Nouvelle)
            }
            Some(e) => e.accepter(tv, coherence),
        };
        let acceptation = match acceptation {
            Ok(a) => a,
            Err(e) => {
                let e = ErreurFederation::from(e);
                if matches!(e, ErreurFederation::Equivoque(_)) {
                    self.note.observer(&kid, Observation::ViolationGrave, maintenant);
                }
                return Err(e);
            }
        };
        // Témoins : clés de journal des membres actuels (ni le propriétaire,
        // ni un banni local), Origin = un témoin au plus (règle partagée).
        let journaux: Vec<[u8; 32]> = self
            .dossiers
            .iter()
            .filter(|(k, _)| **k != kid && self.est_membre(k) && !self.note.est_banni(k, maintenant))
            .map(|(_, autre)| autre.journal)
            .collect();
        let requis = self.parametres.temoins_min;
        match verifier_temoins(octets, &d.journal, &journaux, self.origin.as_ref(), requis, maintenant) {
            Ok(_) => {}
            Err(ErreurJournal::TemoinsInsuffisants { requis, obtenu }) => {
                return Err(ErreurFederation::TemoinsInsuffisants { requis, obtenu });
            }
            Err(ErreurJournal::Sceau(e)) => return Err(e.into()),
            Err(_) => return Err(ErreurFederation::TemoinsInsuffisants { requis, obtenu: 0 }),
        }
        if let Some(d) = self.dossiers.get_mut(&kid) {
            d.vue = etat;
        }
        let vue = emise_a.min(maintenant);
        self.vue_a_jour_au = Some(self.vue_a_jour_au.map_or(vue, |a| a.max(vue)));
        Ok(acceptation)
    }

    /// Fraîcheur de la vue fédérale (révocations), EX-35.
    pub fn controler_vue(&self, maintenant: u64) -> Result<(), ErreurRevocation> {
        let a_jour_au = self.vue_a_jour_au.ok_or(ErreurRevocation::ListeAbsente)?;
        if a_jour_au > maintenant.saturating_add(TOLERANCE_HORLOGE_SECS) {
            return Err(ErreurRevocation::HorlogeIncoherente);
        }
        if a_jour_au.saturating_add(self.parametres.fraicheur_vue_secs) < maintenant {
            return Err(ErreurRevocation::Perimee { a_jour_au, maintenant });
        }
        Ok(())
    }

    /// Horodatage de la vue fédérale.
    pub fn vue_a_jour_au(&self) -> Option<u64> {
        self.vue_a_jour_au
    }

    // ── Accusations et révocations ──────────────────────────────────────────

    fn verifier_preuve(&self, accuse: &[u8; 32], preuve: &Preuve, maintenant: u64) -> Result<(), ErreurFederation> {
        let d = self.dossiers.get(&empreinte_cle(accuse)).ok_or(ErreurFederation::Inconnu)?;
        preuve.verifier(&d.journal, &d.registre, maintenant)
    }

    fn exiger_membre_signataire(&self, identite: &CleSignature, accuse: &[u8; 32]) -> Result<(), ErreurFederation> {
        if &identite.cle_publique() == accuse {
            return Err(ErreurFederation::AutoConfirmation);
        }
        if !self.est_membre(&identite.kid()) {
            return Err(ErreurFederation::NonMembre);
        }
        Ok(())
    }

    /// Ouvre une accusation (premier signataire : un membre) après avoir
    /// vérifié la preuve localement.
    pub fn accuser(
        &self,
        accuse: &CogId,
        preuve: &Preuve,
        identite: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurFederation> {
        let kid = self.par_cog.get(accuse).ok_or(ErreurFederation::Inconnu)?;
        let racine = self.dossiers.get(kid).ok_or(ErreurFederation::Inconnu)?.racine;
        self.exiger_membre_signataire(identite, &racine)?;
        self.verifier_preuve(&racine, preuve, maintenant)?;
        Accusation::depuis_preuve(racine, preuve, maintenant).sceller(identite)
    }

    /// Un membre vérifie lui-même la preuve, puis cosigne l'accusation.
    pub fn contresigner_accusation(
        &self,
        octets: &[u8],
        preuve: &Preuve,
        identite: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurFederation> {
        let a = decoder_titre::<Accusation>(&Sceau::decoder(octets)?.charge)?;
        if a.empreinte_preuve != preuve.empreinte() || a.type_preuve != preuve.type_preuve() {
            return Err(ErreurFederation::PreuveNonConcordante);
        }
        self.exiger_membre_signataire(identite, &a.accuse)?;
        self.verifier_preuve(&a.accuse, preuve, maintenant)?;
        Accusation::cosigner(octets, identite)
    }

    /// Accusation reçue : preuve revérifiée, quorum de 6 membres distincts →
    /// **suspension immédiate** (EX-32, A-007) ; la révocation définitive
    /// (seule sanction) suit après 72 h sans recours accepté
    /// ([`Self::actualiser`]). Idempotent pendant la suspension.
    pub fn recevoir_accusation(
        &mut self,
        octets: &[u8],
        preuve: &Preuve,
        maintenant: u64,
    ) -> Result<SuspensionFederale, ErreurFederation> {
        self.horloge(maintenant)?;
        self.echeances(maintenant);
        let sceau = Sceau::decoder(octets)?;
        let brute = decoder_titre::<Accusation>(&sceau.charge)?;
        let identite_accusation = sha2_256(&sceau.octets_signes());
        let kid_accuse = empreinte_cle(&brute.accuse);
        if self.est_revoque(&kid_accuse) {
            return Err(ErreurFederation::Revoque);
        }
        if let Some(s) = self.suspensions.get(&kid_accuse) {
            return Ok(s.publique.clone());
        }
        if self.levees.contains_key(&identite_accusation) {
            return Err(ErreurFederation::AccusationLevee);
        }
        let dossier = self.dossiers.get(&kid_accuse).ok_or(ErreurFederation::Inconnu)?;
        let mut ancres = AncresDeConfiance::new();
        for (k, d) in &self.dossiers {
            if *k != kid_accuse && self.est_membre(k) {
                ancres.ajouter(&d.racine, NiveauConfiance::Reconnu)?;
            }
        }
        let verifie = match verifier_seuil::<Accusation>(octets, &ancres, 1, maintenant) {
            Err(ErreurSceau::SignataireInconnu) => return Err(ErreurFederation::NonMembre),
            autre => autre?,
        };
        let signataires = verifie.signataires().to_vec();
        let a = verifie.into_titre();
        if a.empreinte_preuve != preuve.empreinte() || a.type_preuve != preuve.type_preuve() {
            return Err(ErreurFederation::PreuveNonConcordante);
        }
        if let Err(e) = preuve.verifier(&dossier.journal, &dossier.registre, maintenant) {
            // Faux signalement prouvé : sanction (locale) des auteurs.
            for s in &signataires {
                self.note.observer(s, Observation::FauxSignalement, maintenant);
            }
            return Err(e);
        }
        if signataires.len() < QUORUM_BLACKLIST {
            return Err(ErreurFederation::QuorumNonAtteint { requis: QUORUM_BLACKLIST, obtenu: signataires.len() });
        }
        let suspension = SuspensionFederale {
            racine: dossier.racine,
            kid: kid_accuse,
            cog: dossier.cog.clone(),
            accusation: identite_accusation,
            type_preuve: a.type_preuve,
            empreinte_preuve: a.empreinte_preuve,
            depuis: maintenant,
        };
        self.suspensions.insert(
            kid_accuse,
            EnSuspension {
                publique: suspension.clone(),
                statut: dossier.statut,
                membre: dossier.membre,
                pieces: Pieces { accusation: octets.to_vec(), preuve: preuve.clone() },
            },
        );
        self.note.observer(&kid_accuse, Observation::PreuveCorroboree, maintenant);
        if let Some(d) = self.dossiers.get_mut(&kid_accuse) {
            d.statut = Statut::Suspendu { depuis: maintenant, accusation: identite_accusation };
            d.membre = false;
        }
        self.recalculer(maintenant);
        Ok(suspension)
    }

    // ── Recours (contestation, EX-32, A-007) ────────────────────────────────

    /// Suspension en cours (non échue à `maintenant`) d'un COG.
    fn suspension_ouverte(&self, kid: &Kid, maintenant: u64) -> Result<&SuspensionFederale, ErreurFederation> {
        if self.est_revoque(kid) {
            return Err(ErreurFederation::Revoque);
        }
        let s = &self.suspensions.get(kid).ok_or(ErreurFederation::AucuneSuspension)?.publique;
        if maintenant >= s.echeance() {
            // Délai écoulé : la révocation est acquise (appliquée à la prochaine actualisation).
            return Err(ErreurFederation::Revoque);
        }
        Ok(s)
    }

    /// Un membre (jamais l'accusé) ouvre un recours contre la suspension en
    /// cours d'un COG, après avoir examiné la contre-preuve.
    pub fn ouvrir_recours(
        &self,
        accuse: &CogId,
        identite: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurFederation> {
        let kid = self.par_cog.get(accuse).ok_or(ErreurFederation::Inconnu)?;
        let s = self.suspension_ouverte(kid, maintenant)?;
        self.exiger_membre_signataire(identite, &s.racine)?;
        Recours {
            accuse: s.racine,
            type_preuve: s.type_preuve,
            empreinte_preuve: s.empreinte_preuve,
            accusation: s.accusation,
            emise_a: maintenant,
        }
        .sceller(identite)
    }

    /// Un membre (jamais l'accusé) cosigne un recours portant sur la
    /// suspension en cours (même accusation, même empreinte de preuve).
    pub fn contresigner_recours(
        &self,
        octets: &[u8],
        identite: &CleSignature,
        maintenant: u64,
    ) -> Result<Vec<u8>, ErreurFederation> {
        let r = decoder_titre::<Recours>(&Sceau::decoder(octets)?.charge)?;
        let s = self.suspension_ouverte(&empreinte_cle(&r.accuse), maintenant)?;
        if !recours_concorde(&r, s) {
            return Err(ErreurFederation::PreuveNonConcordante);
        }
        self.exiger_membre_signataire(identite, &r.accuse)?;
        Recours::cosigner(octets, identite)
    }

    /// Recours reçu : avant l'échéance de 72 h, signé par ≥ 6 membres
    /// actuels distincts (l'accusé n'est jamais compté) sur la même
    /// accusation et la même empreinte de preuve ⇒ suspension levée, statut
    /// précédent rétabli ; compteur des 7 jours repris de zéro (fail-closed :
    /// seules comptent les confirmations à partir de la levée). L'accusation
    /// levée ne peut plus être rejouée.
    pub fn recevoir_recours(&mut self, octets: &[u8], maintenant: u64) -> Result<SuspensionFederale, ErreurFederation> {
        self.horloge(maintenant)?;
        self.echeances(maintenant);
        let brute = decoder_titre::<Recours>(&Sceau::decoder(octets)?.charge)?;
        let kid_accuse = empreinte_cle(&brute.accuse);
        let s = self.suspension_ouverte(&kid_accuse, maintenant)?;
        if !recours_concorde(&brute, s) {
            return Err(ErreurFederation::PreuveNonConcordante);
        }
        let mut ancres = AncresDeConfiance::new();
        for (k, d) in &self.dossiers {
            if *k != kid_accuse && self.est_membre(k) {
                ancres.ajouter(&d.racine, NiveauConfiance::Reconnu)?;
            }
        }
        let verifie = match verifier_seuil::<Recours>(octets, &ancres, 1, maintenant) {
            Err(ErreurSceau::SignataireInconnu) => return Err(ErreurFederation::NonMembre),
            autre => autre?,
        };
        let obtenu = verifie.signataires().len();
        if obtenu < QUORUM_RECOURS {
            return Err(ErreurFederation::QuorumNonAtteint { requis: QUORUM_RECOURS, obtenu });
        }
        let Some(levee) = self.suspensions.remove(&kid_accuse) else {
            return Err(ErreurFederation::AucuneSuspension);
        };
        self.levees.insert(
            levee.publique.accusation,
            Levee {
                accuse: kid_accuse,
                pieces: levee.pieces,
                recours: octets.to_vec(),
                depuis: levee.publique.depuis,
                instant: maintenant,
            },
        );
        if let Some(d) = self.dossiers.get_mut(&kid_accuse) {
            d.statut = levee.statut;
            d.membre = levee.membre;
            d.relancer_compteur(maintenant);
        }
        self.recalculer(maintenant);
        Ok(levee.publique)
    }

    /// Suspensions en cours.
    pub fn suspensions(&self) -> impl Iterator<Item = &SuspensionFederale> {
        self.suspensions.values().map(|s| &s.publique)
    }

    /// Vrai si le `cog_id` est suspendu (délai de contestation en cours).
    pub fn cog_suspendu(&self, cog: &CogId) -> bool {
        self.par_cog.get(cog).is_some_and(|k| self.est_suspendu(k))
    }

    /// Révocations fédérales décidées.
    pub fn revocations(&self) -> impl Iterator<Item = &RevocationFederale> {
        self.revocations.values()
    }

    /// Vrai si le `cog_id` est révoqué.
    pub fn cog_revoque(&self, cog: &CogId) -> bool {
        self.par_cog.get(cog).is_some_and(|k| self.est_revoque(k))
    }

    fn est_federe(&self, cog: &CogId) -> bool {
        self.par_cog.contains_key(cog)
    }
}

impl Dossier {
    /// Confirmation vérifiée reçue à `recue_a` : intervalle de validité compté
    /// à partir de la réception (un antidatage n'achète pas d'historique) ;
    /// le titre signé est conservé pour la persistance. Même chemin à la
    /// réception et à la reprise d'un état persisté.
    fn ajouter_confirmation(&mut self, confirmateur: Kid, c: &Confirmation, octets: &[u8], recue_a: u64) {
        let debut = c.emise_a.max(recue_a);
        if debut > c.expire_a {
            return;
        }
        let oubli_avant = recue_a.saturating_sub(HISTORIQUE_SECS);
        fusionner(self.confirmations.entry(confirmateur).or_default(), (debut, c.expire_a), oubli_avant);
        let titres = self.titres.entry(confirmateur).or_default();
        titres.retain(|t| t.expire_a >= oubli_avant);
        if !titres.iter().any(|t| t.octets == octets) {
            titres.push(TitreRecu { recue_a, expire_a: c.expire_a, octets: octets.to_vec() });
            if titres.len() > MAX_TITRES_PAR_CONFIRMATEUR {
                titres.remove(0);
            }
        }
    }

    /// Levée d'une suspension à `t` : compteur des 7 jours repris de zéro,
    /// les intervalles de confirmation commencent au plus tôt à la levée.
    fn relancer_compteur(&mut self, t: u64) {
        for iv in self.confirmations.values_mut() {
            iv.retain(|&(_, e)| e >= t);
            for (debut, _) in iv.iter_mut() {
                *debut = (*debut).max(t);
            }
        }
        self.confirmations.retain(|_, iv| !iv.is_empty());
        for titres in self.titres.values_mut() {
            titres.retain(|x| x.expire_a >= t);
        }
        self.titres.retain(|_, v| !v.is_empty());
    }
}

fn recours_concorde(r: &Recours, s: &SuspensionFederale) -> bool {
    r.accuse == s.racine
        && r.accusation == s.accusation
        && r.empreinte_preuve == s.empreinte_preuve
        && r.type_preuve == s.type_preuve
}

fn sha2_256(octets: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(octets).into()
}

/// Source de révocation miyauth complétée par la vue fédérale (EX-35) :
///
/// - émetteur fédéré révoqué **ou suspendu** (accusation au quorum, délai de
///   contestation en cours, A-007) ⇒ tout titre qu'il a émis est refusé ;
/// - émetteur fédéré et vue fédérale absente, périmée ou datée dans le futur
///   ⇒ erreur (refus) ;
/// - sinon (et pour un émetteur hors fédération, dont le COG local) ⇒ la
///   source locale décide.
pub struct RevocationsFederation<'a> {
    federation: &'a Federation,
    locales: &'a dyn SourceRevocation,
}

impl<'a> RevocationsFederation<'a> {
    /// Source composée.
    pub fn new(federation: &'a Federation, locales: &'a dyn SourceRevocation) -> Self {
        Self { federation, locales }
    }
}

impl std::fmt::Debug for RevocationsFederation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RevocationsFederation").finish_non_exhaustive()
    }
}

impl SourceRevocation for RevocationsFederation<'_> {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        if self.federation.est_federe(emetteur) {
            if self.federation.cog_revoque(emetteur) || self.federation.cog_suspendu(emetteur) {
                return Ok(true);
            }
            self.federation.controler_vue(maintenant_secs)?;
        }
        self.locales.est_revoque(emetteur, cible, maintenant_secs)
    }
}
