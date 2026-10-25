//! @id     miyufederation_federation_etat
//! @do     sceller_reprendre_etat_federal
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — sous-module « etat » (format MFED de la vue fédérale locale : titres signés revérifiés, reste scellé par HMAC, reprise fail-closed).

//! État fédéral local persistant, format **`MFED`** (non spécifié par la doc
//! gelée, **à consigner par amendement**).
//!
//! Principe : on persiste de préférence les **titres signés d'origine**
//! (candidatures, confirmations, accusations et leurs preuves, recours
//! acceptés, têtes témoignées), **revérifiés** au chargement par les mêmes
//! règles qu'à la réception ; les intervalles de confirmation sont
//! **recalculés** en rejouant les confirmations (même code qu'à la
//! réception), jamais lus. Ce qui ne peut pas être revérifié (heures de
//! réception, statut et drapeau « membre acquis », quotas, note locale) est
//! protégé par un **HMAC-SHA256** sur tout le fichier, clé dérivée du secret
//! local du COG ([`crate::persistance::CleEtatFederal`]).
//!
//! # Enveloppe (big-endian, taille exacte)
//!
//! ```text
//! "MFED" | version 0x02 | n (4, ≤ 64 Mio) | corps (n) | mac (32)
//! mac = HMAC-SHA256(K, "MFED" | 0x02 | n | corps)
//! K   = HMAC-SHA256(secret local du COG, "miyukini/miyufederation/etat/v1")
//! ```
//!
//! # Corps
//!
//! `bloc` = `longueur (4) | octets` ; `compte` = `u32` borné ; entiers `u64`
//! en secondes Unix (heure de confiance locale).
//!
//! | Champ | Encodage | Revérification au chargement |
//! |---|---|---|
//! | `heure` | `u64` | HMAC ; heure de reprise ≥ `heure` (sinon refus) |
//! | `vue_a_jour_au` | `0x00` ou `0x01 \| u64` | HMAC ; ≤ plus récente tête témoignée persistée |
//! | dossiers | `compte (≤ 65 536)` × dossier, `kid` strictement croissants | — |
//! | ↳ `inscrit_a` | `u64` | HMAC ; ≤ `heure` |
//! | ↳ `fondateur` | `0x00`/`0x01` | HMAC |
//! | ↳ `statut` | `0` candidat, `1` invité, `2` membre, `3` suspendu, `4` révoqué | HMAC + cohérence (membre ⇔ drapeau, 3 ⇔ suspension, 4 ⇔ révocation) puis **recalcul** |
//! | ↳ `membre` | `0x00`/`0x01` (membre acquis, collant ≥ 2) | HMAC ; ne vaut que si le recalcul retrouve ≥ 2 confirmations revérifiées de membres |
//! | ↳ candidature | `bloc` (sceau `0x06`/`0x43`) | signature de l'identité à `inscrit_a`, preuve de calcul ≥ plancher (8 bits) ; donne racine, journal, registre, `cog_id` |
//! | ↳ vue | `bloc` vide ou `MSRG` (dernière tête témoignée acceptée) | `EtatRegistre::decoder` : signature de la clé de journal, registre attendu |
//! | ↳ témoignage | `bloc` vide ou `MSRG` (dernière tête contresignée) | idem |
//! | ↳ confirmations | `compte (≤ 65 536)` × (`recue_a (8)` \| `bloc` sceau `0x07`) | signature du confirmateur (dossier connu) à `recue_a`, `reconnue` = ce dossier ; intervalles **rejoués** |
//! | suspensions | `compte` × (`depuis (8)` \| statut antérieur `0..=2` (1) \| membre antérieur (1) \| `bloc` accusation \| `bloc` preuve `MFPV`), `kid` croissants | accusation : ≥ 6 signataires distincts connus hors accusé à `depuis`, preuve concordante et vérifiée |
//! | révocations | `compte` × (`depuis (8)` \| `revoque_a (8)` \| `bloc` accusation \| `bloc` preuve) | idem ; `revoque_a` ≥ `depuis` + 72 h |
//! | accusations levées | `compte (≤ 65 536)` × (`depuis (8)` \| `levee_a (8)` \| `bloc` accusation \| `bloc` preuve \| `bloc` recours), identités croissantes | accusation idem ; recours `0x06`/`0x44` : ≥ 6 signataires distincts connus hors accusé à `levee_a`, même accusation et même empreinte, `depuis` ≤ `levee_a` < `depuis` + 72 h |
//! | quotas | `compte` × (`membre (32)` \| `confirme (32)` \| `premiere (8)` \| `derniere (8)`), groupés par membre | HMAC ; dossiers connus, `premiere` ≤ `derniere` ≤ `heure` |
//! | note locale | `compte` × (`kid (32)` \| `points (4)` \| `maj (8)`), `kid` croissants | HMAC (jamais exportée) |
//! | ancre du registre local | `taille (8)` \| `racine (32)` : tête du registre local à l'écriture | HMAC ; au chargement, doit être un **préfixe** du registre fourni ([`crate::persistance::StockageFederal::charger`]) |
//! | révocations à publier | `compte` × `kid (32)`, strictement croissants | HMAC ; chaque `kid` est une révocation ci-dessus (pièces revérifiées) |
//! | révocations inscrites | `compte` × (`kid (32)` \| `inscrite_a (8)`), `kid` strictement croissants | HMAC ; ni suspendu ni révoqué avec pièces, `inscrite_a` ≤ `heure` ; dossier éventuel au statut révoqué |
//!
//! Version `0x02` (amendement à consigner) : ajout des trois derniers champs
//! (ancre du registre local, file des révocations à publier, révocations
//! constatées au registre). La version `0x01` est **refusée**
//! ([`ErreurEtat::VersionInconnue`]) : l'accepter rouvrirait le rejeu d'un
//! état ancien sans ancre.
//!
//! # Reprise (fail-closed)
//!
//! - Magie, taille, version inconnue, HMAC faux, structure, titre refusé,
//!   incohérence : **refus de tout l'état** (aucun état partiel, la
//!   fédération reste désactivée ; l'appelant signale). Aucun statut n'est
//!   jamais obtenu d'un fichier refusé.
//! - Rattrapage : la vue est rejouée de `heure` à l'heure de reprise aux
//!   instants où un statut peut changer (débuts et fins d'intervalles, + 7 j)
//!   et aux échéances des suspensions : une suspension échue pendant l'arrêt
//!   devient une révocation définitive **à son échéance**, exactement comme
//!   si le processus avait tourné ; elle entre dans la file des révocations
//!   **à publier**, persistée, rendue à chaque reprise tant qu'elle n'est pas
//!   acquittée après inscription au registre.
//! - Limites : les têtes témoignées sont revérifiées mais leur fraîcheur
//!   (`vue_a_jour_au`) reste soumise au contrôle de 15 min ; au-delà de
//!   64 confirmations conservées par couple (confirmé, confirmateur), les
//!   plus anciennes sont oubliées (une reprise ne peut qu'y perdre de la
//!   couverture).

use std::collections::BTreeMap;

use miyusceau::{
    decoder_titre, empreinte_cle, verifier, verifier_seuil, AncresDeConfiance, EtatRegistre, Kid, NiveauConfiance,
    PolitiqueTete, Sceau, TAILLE_MAX_SCEAU,
};

use super::{
    ancre, cog_de, sha2_256, Dossier, EnSuspension, Federation, Levee, Parametres, Pieces, PiecesRevocation, Quota,
    RevocationFederale, SuspensionFederale, TitreRecu,
};
use crate::calcul::DIFFICULTE_PLANCHER;
use crate::erreurs::ErreurFederation;
use crate::persistance::{AncreRegistre, CleEtatFederal, ErreurEtat};
use crate::preuve::Preuve;
use crate::statut::{
    Sanction, Statut, DELAI_CONTESTATION_SECS, DUREE_MEMBRE_SECS, QUORUM_BLACKLIST, QUORUM_RECOURS,
};
use crate::titres::{Accusation, Candidature, Confirmation, Recours};

/// Magie du format.
pub(crate) const MAGIC: &[u8; 4] = b"MFED";
/// Version du format.
pub(crate) const VERSION: u8 = 0x02;
/// Entête : magie, version, longueur du corps.
pub(crate) const TAILLE_ENTETE: usize = 4 + 1 + 4;
/// Taille du MAC final.
pub(crate) const TAILLE_MAC: usize = 32;
/// Taille maximale du corps (et, à l'entête et au MAC près, du fichier).
pub const TAILLE_MAX_ETAT: usize = 64 << 20;
/// Nombre maximal de dossiers, de confirmations par dossier, d'accusations levées.
const MAX_ELEMENTS: usize = 65_536;
/// Nombre maximal d'enregistrements de quota.
const MAX_QUOTAS: usize = 1 << 20;
/// Taille maximale d'une preuve `MFPV` (deux sceaux de tête).
const TAILLE_MAX_PREUVE: usize = 14 + 2 * TAILLE_MAX_SCEAU;
/// Taille maximale d'un état `MSRG`.
const TAILLE_MAX_MSRG: usize = 9 + TAILLE_MAX_SCEAU;

// ── Écriture / lecture strictes ─────────────────────────────────────────────

struct Ecrivain(Vec<u8>);

impl Ecrivain {
    fn u8(&mut self, v: u8) {
        self.0.push(v);
    }

    fn u32(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_be_bytes());
    }

    fn u64(&mut self, v: u64) {
        self.0.extend_from_slice(&v.to_be_bytes());
    }

    fn b32(&mut self, v: &[u8; 32]) {
        self.0.extend_from_slice(v);
    }

    fn compte(&mut self, n: usize, max: usize) -> Result<(), ErreurEtat> {
        if n > max {
            return Err(ErreurEtat::TropGrand);
        }
        self.u32(u32::try_from(n).map_err(|_| ErreurEtat::TropGrand)?);
        Ok(())
    }

    fn bloc(&mut self, octets: &[u8], max: usize) -> Result<(), ErreurEtat> {
        self.compte(octets.len(), max)?;
        self.0.extend_from_slice(octets);
        Ok(())
    }
}

struct Lecteur<'a> {
    octets: &'a [u8],
    pos: usize,
}

const fn illisible(motif: &'static str) -> ErreurEtat {
    ErreurEtat::Illisible(motif)
}

impl<'a> Lecteur<'a> {
    fn prendre(&mut self, n: usize) -> Result<&'a [u8], ErreurEtat> {
        let fin = self.pos.checked_add(n).ok_or(illisible("longueur"))?;
        let s = self.octets.get(self.pos..fin).ok_or(illisible("corps tronqué"))?;
        self.pos = fin;
        Ok(s)
    }

    fn u8(&mut self) -> Result<u8, ErreurEtat> {
        Ok(self.prendre(1)?[0])
    }

    fn drapeau(&mut self) -> Result<bool, ErreurEtat> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(illisible("drapeau")),
        }
    }

    fn u32(&mut self) -> Result<u32, ErreurEtat> {
        let s = self.prendre(4)?;
        Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    }

    fn u64(&mut self) -> Result<u64, ErreurEtat> {
        let mut b = [0u8; 8];
        b.copy_from_slice(self.prendre(8)?);
        Ok(u64::from_be_bytes(b))
    }

    fn b32(&mut self) -> Result<[u8; 32], ErreurEtat> {
        let mut b = [0u8; 32];
        b.copy_from_slice(self.prendre(32)?);
        Ok(b)
    }

    fn compte(&mut self, max: usize) -> Result<usize, ErreurEtat> {
        let n = self.u32()? as usize;
        if n > max {
            return Err(illisible("compte hors bornes"));
        }
        Ok(n)
    }

    fn bloc(&mut self, max: usize) -> Result<&'a [u8], ErreurEtat> {
        let n = self.compte(max)?;
        self.prendre(n)
    }

    fn bloc_non_vide(&mut self, max: usize) -> Result<&'a [u8], ErreurEtat> {
        let b = self.bloc(max)?;
        if b.is_empty() {
            return Err(illisible("bloc vide"));
        }
        Ok(b)
    }
}

// ── Codes de statut ─────────────────────────────────────────────────────────

const fn code_statut(s: Statut) -> u8 {
    match s {
        Statut::Candidat => 0,
        Statut::Invite => 1,
        Statut::Membre => 2,
        Statut::Suspendu { .. } => 3,
        Statut::Revoque => 4,
    }
}

/// Statut de base (hors suspension et révocation, reconstruits à part).
fn statut_de_base(code: u8) -> Result<Statut, ErreurEtat> {
    match code {
        0 => Ok(Statut::Candidat),
        1 => Ok(Statut::Invite),
        2 => Ok(Statut::Membre),
        _ => Err(ErreurEtat::Incoherent("statut")),
    }
}

fn titre(e: impl Into<ErreurFederation>) -> ErreurEtat {
    ErreurEtat::Titre(e.into())
}

// ── Lecture brute (aucune vérification) ─────────────────────────────────────

struct DossierBrut<'a> {
    inscrit_a: u64,
    fondateur: bool,
    statut: u8,
    membre: bool,
    candidature: &'a [u8],
    vue: &'a [u8],
    temoignage: &'a [u8],
    titres: Vec<(u64, &'a [u8])>,
}

struct SanctionBrute<'a> {
    depuis: u64,
    accusation: &'a [u8],
    preuve: &'a [u8],
}

struct Corps<'a> {
    heure: u64,
    vue_a_jour_au: Option<u64>,
    dossiers: Vec<DossierBrut<'a>>,
    suspensions: Vec<(SanctionBrute<'a>, u8, bool)>,
    revocations: Vec<(SanctionBrute<'a>, u64)>,
    levees: Vec<(SanctionBrute<'a>, u64, &'a [u8])>,
    quotas: Vec<(Kid, Kid, u64, u64)>,
    fiches: Vec<(Kid, u32, u64)>,
    ancre: AncreRegistre,
    a_publier: Vec<Kid>,
    inscrites: Vec<(Kid, u64)>,
}

fn lire_sanction<'a>(l: &mut Lecteur<'a>, depuis: u64) -> Result<SanctionBrute<'a>, ErreurEtat> {
    Ok(SanctionBrute { depuis, accusation: l.bloc_non_vide(TAILLE_MAX_SCEAU)?, preuve: l.bloc_non_vide(TAILLE_MAX_PREUVE)? })
}

fn lire_corps(octets: &[u8]) -> Result<Corps<'_>, ErreurEtat> {
    let mut l = Lecteur { octets, pos: 0 };
    let heure = l.u64()?;
    let vue_a_jour_au = if l.drapeau()? { Some(l.u64()?) } else { None };
    let nd = l.compte(MAX_ELEMENTS)?;
    let mut dossiers = Vec::with_capacity(nd.min(1024));
    for _ in 0..nd {
        let inscrit_a = l.u64()?;
        let fondateur = l.drapeau()?;
        let statut = l.u8()?;
        let membre = l.drapeau()?;
        let candidature = l.bloc_non_vide(TAILLE_MAX_SCEAU)?;
        let vue = l.bloc(TAILLE_MAX_MSRG)?;
        let temoignage = l.bloc(TAILLE_MAX_MSRG)?;
        let nc = l.compte(MAX_ELEMENTS)?;
        let mut titres = Vec::with_capacity(nc.min(1024));
        for _ in 0..nc {
            titres.push((l.u64()?, l.bloc_non_vide(TAILLE_MAX_SCEAU)?));
        }
        dossiers.push(DossierBrut { inscrit_a, fondateur, statut, membre, candidature, vue, temoignage, titres });
    }
    let ns = l.compte(MAX_ELEMENTS)?;
    let mut suspensions = Vec::with_capacity(ns.min(1024));
    for _ in 0..ns {
        let depuis = l.u64()?;
        let statut = l.u8()?;
        let membre = l.drapeau()?;
        suspensions.push((lire_sanction(&mut l, depuis)?, statut, membre));
    }
    let nr = l.compte(MAX_ELEMENTS)?;
    let mut revocations = Vec::with_capacity(nr.min(1024));
    for _ in 0..nr {
        let depuis = l.u64()?;
        let revoque_a = l.u64()?;
        revocations.push((lire_sanction(&mut l, depuis)?, revoque_a));
    }
    let nl = l.compte(MAX_ELEMENTS)?;
    let mut levees = Vec::with_capacity(nl.min(1024));
    for _ in 0..nl {
        let depuis = l.u64()?;
        let levee_a = l.u64()?;
        let s = lire_sanction(&mut l, depuis)?;
        levees.push((s, levee_a, l.bloc_non_vide(TAILLE_MAX_SCEAU)?));
    }
    let nq = l.compte(MAX_QUOTAS)?;
    let mut quotas = Vec::with_capacity(nq.min(1024));
    for _ in 0..nq {
        quotas.push((l.b32()?, l.b32()?, l.u64()?, l.u64()?));
    }
    let nn = l.compte(MAX_ELEMENTS)?;
    let mut fiches = Vec::with_capacity(nn.min(1024));
    for _ in 0..nn {
        fiches.push((l.b32()?, l.u32()?, l.u64()?));
    }
    let ancre = AncreRegistre { taille: l.u64()?, racine: l.b32()? };
    let np = l.compte(MAX_ELEMENTS)?;
    let mut a_publier = Vec::with_capacity(np.min(1024));
    for _ in 0..np {
        a_publier.push(l.b32()?);
    }
    let ni = l.compte(MAX_ELEMENTS)?;
    let mut inscrites = Vec::with_capacity(ni.min(1024));
    for _ in 0..ni {
        inscrites.push((l.b32()?, l.u64()?));
    }
    if l.pos != octets.len() {
        return Err(illisible("octets en trop"));
    }
    Ok(Corps {
        heure,
        vue_a_jour_au,
        dossiers,
        suspensions,
        revocations,
        levees,
        quotas,
        fiches,
        ancre,
        a_publier,
        inscrites,
    })
}

// ── Scellement ──────────────────────────────────────────────────────────────

fn ecrire_pieces(e: &mut Ecrivain, p: &Pieces) -> Result<(), ErreurEtat> {
    e.bloc(&p.accusation, TAILLE_MAX_SCEAU)?;
    e.bloc(&p.preuve.encoder(), TAILLE_MAX_PREUVE)
}

fn mac(cle: &CleEtatFederal, entete_et_corps: &[u8]) -> [u8; 32] {
    use hmac::Mac;
    let mut m = cle.mac();
    m.update(entete_et_corps);
    m.finalize().into_bytes().into()
}

impl Federation {
    /// Scelle la vue fédérale locale au format `MFED` (voir [`etat`](self)),
    /// ancrée sur la tête `ancre` du registre local. À écrire atomiquement
    /// après chaque opération qui la modifie
    /// ([`crate::persistance::StockageFederal::enregistrer`]).
    pub fn sceller_etat(&self, cle: &CleEtatFederal, ancre: &AncreRegistre) -> Result<Vec<u8>, ErreurEtat> {
        let mut e = Ecrivain(Vec::with_capacity(4096));
        e.u64(self.heure);
        match self.vue_a_jour_au {
            None => e.u8(0),
            Some(v) => {
                e.u8(1);
                e.u64(v);
            }
        }
        e.compte(self.dossiers.len(), MAX_ELEMENTS)?;
        for d in self.dossiers.values() {
            e.u64(d.inscrit_a);
            e.u8(u8::from(d.fondateur));
            e.u8(code_statut(d.statut));
            e.u8(u8::from(d.membre));
            e.bloc(&d.candidature, TAILLE_MAX_SCEAU)?;
            e.bloc(&d.vue.as_ref().map(EtatRegistre::encoder).unwrap_or_default(), TAILLE_MAX_MSRG)?;
            e.bloc(&d.temoignage.as_ref().map(EtatRegistre::encoder).unwrap_or_default(), TAILLE_MAX_MSRG)?;
            let titres: Vec<&TitreRecu> = d.titres.values().flatten().collect();
            e.compte(titres.len(), MAX_ELEMENTS)?;
            for t in titres {
                e.u64(t.recue_a);
                e.bloc(&t.octets, TAILLE_MAX_SCEAU)?;
            }
        }
        e.compte(self.suspensions.len(), MAX_ELEMENTS)?;
        for s in self.suspensions.values() {
            e.u64(s.publique.depuis);
            e.u8(code_statut(s.statut));
            e.u8(u8::from(s.membre));
            ecrire_pieces(&mut e, &s.pieces)?;
        }
        e.compte(self.revocations.len(), MAX_ELEMENTS)?;
        for (kid, r) in &self.revocations {
            let p = self.pieces_revocations.get(kid).ok_or(ErreurEtat::Incoherent("pièces de révocation"))?;
            e.u64(p.depuis);
            e.u64(r.revoque_a);
            ecrire_pieces(&mut e, &p.pieces)?;
        }
        e.compte(self.levees.len(), MAX_ELEMENTS)?;
        for l in self.levees.values() {
            e.u64(l.depuis);
            e.u64(l.instant);
            ecrire_pieces(&mut e, &l.pieces)?;
            e.bloc(&l.recours, TAILLE_MAX_SCEAU)?;
        }
        let quotas: Vec<(&Kid, &Quota)> = self.quotas.iter().flat_map(|(m, v)| v.iter().map(move |q| (m, q))).collect();
        e.compte(quotas.len(), MAX_QUOTAS)?;
        for (m, q) in quotas {
            e.b32(m);
            e.b32(&q.confirme);
            e.u64(q.premiere);
            e.u64(q.derniere);
        }
        let fiches = self.note.fiches();
        e.compte(fiches.len(), MAX_ELEMENTS)?;
        for (kid, points, maj) in fiches {
            e.b32(&kid);
            e.u32(points);
            e.u64(maj);
        }
        e.u64(ancre.taille);
        e.b32(&ancre.racine);
        e.compte(self.a_publier.len(), MAX_ELEMENTS)?;
        for kid in &self.a_publier {
            e.b32(kid);
        }
        e.compte(self.inscrites.len(), MAX_ELEMENTS)?;
        for (kid, inscrite_a) in &self.inscrites {
            e.b32(kid);
            e.u64(*inscrite_a);
        }
        let corps = e.0;
        if corps.len() > TAILLE_MAX_ETAT {
            return Err(ErreurEtat::TropGrand);
        }
        let mut s = Vec::with_capacity(TAILLE_ENTETE + corps.len() + TAILLE_MAC);
        s.extend_from_slice(MAGIC);
        s.push(VERSION);
        s.extend_from_slice(&(corps.len() as u32).to_be_bytes());
        s.extend_from_slice(&corps);
        let m = mac(cle, &s);
        s.extend_from_slice(&m);
        Ok(s)
    }

    /// Reprend une vue fédérale scellée (`MFED`) puis la fait avancer jusqu'à
    /// `maintenant` comme si le processus avait tourné (voir [`etat`](self)).
    ///
    /// Renvoie la vue, les révocations définitives produites pendant l'arrêt
    /// (suspensions échues, révoquées à leur échéance ; elles sont aussi dans
    /// la file à publier, [`Federation::revocations_a_publier`]) et l'ancre
    /// du registre local scellée avec l'état (à contrôler par l'appelant :
    /// [`crate::persistance::StockageFederal::charger`]).
    ///
    /// Tout refus (illisible, altéré, version inconnue, titre refusé,
    /// incohérence, heure en recul) ⇒ `Err` : aucun état n'est repris, la
    /// fédération reste désactivée, l'appelant signale.
    pub fn reprendre(
        octets: &[u8],
        cle: &CleEtatFederal,
        parametres: Parametres,
        origin: Option<[u8; 32]>,
        maintenant: u64,
    ) -> Result<(Self, Vec<RevocationFederale>, AncreRegistre), ErreurEtat> {
        if octets.len() < TAILLE_ENTETE + TAILLE_MAC || &octets[0..4] != MAGIC {
            return Err(illisible("magie"));
        }
        if octets[4] != VERSION {
            return Err(ErreurEtat::VersionInconnue(octets[4]));
        }
        let n = u32::from_be_bytes([octets[5], octets[6], octets[7], octets[8]]) as usize;
        if n > TAILLE_MAX_ETAT || octets.len() != TAILLE_ENTETE + n + TAILLE_MAC {
            return Err(illisible("taille"));
        }
        let (contenu, m) = octets.split_at(TAILLE_ENTETE + n);
        {
            use hmac::Mac;
            let mut verif = cle.mac();
            verif.update(contenu);
            verif.verify_slice(m).map_err(|_| ErreurEtat::Altere)?;
        }
        let corps = lire_corps(&contenu[TAILLE_ENTETE..])?;
        let mut f = Self::new(parametres, origin);
        f.restaurer(&corps)?;
        let definitives = f.rattraper(maintenant)?;
        Ok((f, definitives, corps.ancre))
    }

    // ── Restauration (revérification) ──────────────────────────────────────

    fn restaurer(&mut self, c: &Corps<'_>) -> Result<(), ErreurEtat> {
        self.heure = c.heure;
        let politique = self.politique_tete().map_err(ErreurEtat::Titre)?;

        // Dossiers : candidature et têtes revérifiées.
        let mut precedent: Option<Kid> = None;
        for b in &c.dossiers {
            if b.inscrit_a > c.heure {
                return Err(ErreurEtat::Incoherent("inscription future"));
            }
            let brute = decoder_titre::<Candidature>(&Sceau::decoder(b.candidature).map_err(titre)?.charge).map_err(titre)?;
            let cand = verifier::<Candidature>(b.candidature, &ancre(&brute.racine).map_err(ErreurEtat::Titre)?, b.inscrit_a)
                .map_err(titre)?
                .into_titre();
            if cand.difficulte < DIFFICULTE_PLANCHER || !cand.preuve_valide() {
                return Err(ErreurEtat::Titre(ErreurFederation::CalculInsuffisant));
            }
            let kid = empreinte_cle(&cand.racine);
            if precedent.is_some_and(|p| p >= kid) {
                return Err(ErreurEtat::Incoherent("ordre des dossiers"));
            }
            precedent = Some(kid);
            let tete = |o: &[u8]| -> Result<Option<EtatRegistre>, ErreurEtat> {
                if o.is_empty() {
                    return Ok(None);
                }
                let a = ancre(&cand.journal).map_err(ErreurEtat::Titre)?;
                EtatRegistre::decoder(o, &a, &politique, &cand.registre).map(Some).map_err(titre)
            };
            let cog = cog_de(&cand.racine).map_err(ErreurEtat::Titre)?;
            self.par_cog.insert(cog.clone(), kid);
            self.dossiers.insert(
                kid,
                Dossier {
                    racine: cand.racine,
                    cog,
                    journal: cand.journal,
                    registre: cand.registre,
                    fondateur: b.fondateur,
                    inscrit_a: b.inscrit_a,
                    candidature: b.candidature.to_vec(),
                    statut: Statut::Candidat,
                    membre: b.membre,
                    confirmations: BTreeMap::new(),
                    titres: BTreeMap::new(),
                    vue: tete(b.vue)?,
                    temoignage: tete(b.temoignage)?,
                },
            );
        }

        // Suspensions, révocations, levées : pièces revérifiées.
        let mut precedent: Option<Kid> = None;
        for (s, statut, membre) in &c.suspensions {
            let (acc, id, kid, pieces) = self.verifier_pieces(s, c.heure)?;
            if precedent.is_some_and(|p| p >= kid) {
                return Err(ErreurEtat::Incoherent("ordre des suspensions"));
            }
            precedent = Some(kid);
            let anterieur = statut_de_base(*statut)?;
            if (anterieur == Statut::Membre) != *membre {
                return Err(ErreurEtat::Incoherent("statut antérieur"));
            }
            let d = self.dossiers.get(&kid).ok_or(ErreurEtat::Incoherent("accusé inconnu"))?;
            let publique = SuspensionFederale {
                racine: d.racine,
                kid,
                cog: d.cog.clone(),
                accusation: id,
                type_preuve: acc.type_preuve,
                empreinte_preuve: acc.empreinte_preuve,
                depuis: s.depuis,
            };
            self.suspensions.insert(kid, EnSuspension { publique, statut: anterieur, membre: *membre, pieces });
        }
        let mut precedent: Option<Kid> = None;
        for (s, revoque_a) in &c.revocations {
            let (_, id, kid, pieces) = self.verifier_pieces(s, c.heure)?;
            if precedent.is_some_and(|p| p >= kid) {
                return Err(ErreurEtat::Incoherent("ordre des révocations"));
            }
            precedent = Some(kid);
            if *revoque_a < s.depuis.saturating_add(DELAI_CONTESTATION_SECS) || *revoque_a > c.heure {
                return Err(ErreurEtat::Incoherent("date de révocation"));
            }
            if self.suspensions.contains_key(&kid) {
                return Err(ErreurEtat::Incoherent("révoqué et suspendu"));
            }
            let d = self.dossiers.get(&kid).ok_or(ErreurEtat::Incoherent("accusé inconnu"))?;
            self.revocations.insert(
                kid,
                RevocationFederale {
                    racine: d.racine,
                    kid,
                    cog: d.cog.clone(),
                    accusation: id,
                    revoque_a: *revoque_a,
                    sanction: Sanction::RevocationIdentite,
                },
            );
            self.pieces_revocations.insert(kid, PiecesRevocation { pieces, depuis: s.depuis });
        }
        let mut precedent: Option<[u8; 32]> = None;
        for (s, levee_a, recours) in &c.levees {
            let (_, id, kid, pieces) = self.verifier_pieces(s, c.heure)?;
            if precedent.is_some_and(|p| p >= id) {
                return Err(ErreurEtat::Incoherent("ordre des levées"));
            }
            precedent = Some(id);
            if *levee_a < s.depuis || *levee_a >= s.depuis.saturating_add(DELAI_CONTESTATION_SECS) || *levee_a > c.heure {
                return Err(ErreurEtat::Incoherent("date de levée"));
            }
            let d = self.dossiers.get(&kid).ok_or(ErreurEtat::Incoherent("accusé inconnu"))?;
            let r = decoder_titre::<Recours>(&Sceau::decoder(recours).map_err(titre)?.charge).map_err(titre)?;
            if r.accuse != d.racine
                || r.accusation != id
                || r.empreinte_preuve != pieces.preuve.empreinte()
                || r.type_preuve != pieces.preuve.type_preuve()
            {
                return Err(ErreurEtat::Titre(ErreurFederation::PreuveNonConcordante));
            }
            verifier_seuil::<Recours>(recours, &self.ancres_hors(&kid)?, QUORUM_RECOURS, *levee_a).map_err(titre)?;
            let en_cours = self.suspensions.get(&kid).is_some_and(|x| x.publique.accusation == id)
                || self.revocations.get(&kid).is_some_and(|x| x.accusation == id);
            if en_cours {
                return Err(ErreurEtat::Incoherent("accusation levée et en vigueur"));
            }
            self.levees.insert(
                id,
                Levee { accuse: kid, pieces, recours: recours.to_vec(), depuis: s.depuis, instant: *levee_a },
            );
        }

        // Révocations constatées au registre (sans pièces) : ni suspendu ni révoqué avec pièces.
        let mut precedent: Option<Kid> = None;
        for (kid, inscrite_a) in &c.inscrites {
            if precedent.is_some_and(|p| p >= *kid) {
                return Err(ErreurEtat::Incoherent("ordre des révocations inscrites"));
            }
            precedent = Some(*kid);
            if *inscrite_a > c.heure {
                return Err(ErreurEtat::Incoherent("révocation inscrite future"));
            }
            if self.suspensions.contains_key(kid) || self.revocations.contains_key(kid) {
                return Err(ErreurEtat::Incoherent("révocation inscrite en double"));
            }
            self.inscrites.insert(*kid, *inscrite_a);
        }
        // File des révocations à publier : révocations (avec pièces) connues.
        let mut precedent: Option<Kid> = None;
        for kid in &c.a_publier {
            if precedent.is_some_and(|p| p >= *kid) {
                return Err(ErreurEtat::Incoherent("ordre des révocations à publier"));
            }
            precedent = Some(*kid);
            if !self.revocations.contains_key(kid) {
                return Err(ErreurEtat::Incoherent("révocation à publier inconnue"));
            }
            self.a_publier.insert(*kid);
        }

        // Statuts : cohérence avec les sanctions ; le recalcul suit (rattrapage).
        let mut statuts = Vec::with_capacity(c.dossiers.len());
        for ((kid, d), b) in self.dossiers.iter().zip(&c.dossiers) {
            let statut = if self.est_revoque(kid) {
                (b.statut == 4 && !b.membre).then_some(Statut::Revoque)
            } else if let Some(s) = self.suspensions.get(kid) {
                (b.statut == 3 && !b.membre)
                    .then_some(Statut::Suspendu { depuis: s.publique.depuis, accusation: s.publique.accusation })
            } else {
                statut_de_base(b.statut).ok().filter(|s| (*s == Statut::Membre) == b.membre)
            };
            let statut = statut.ok_or(ErreurEtat::Incoherent("statut"))?;
            if d.fondateur && matches!(statut, Statut::Candidat | Statut::Invite) {
                return Err(ErreurEtat::Incoherent("fondateur"));
            }
            statuts.push((*kid, statut));
        }
        for (kid, statut) in statuts {
            if let Some(d) = self.dossiers.get_mut(&kid) {
                d.statut = statut;
            }
        }

        // Confirmations revérifiées, intervalles rejoués (avec les levées).
        let kids: Vec<Kid> = self.dossiers.keys().copied().collect();
        for (kid, b) in kids.iter().zip(&c.dossiers) {
            enum Evenement {
                Titre(Kid, Confirmation, Vec<u8>),
                Levee,
            }
            let racine = self.dossiers.get(kid).map(|d| d.racine).ok_or(ErreurEtat::Incoherent("dossier"))?;
            let mut evenements: Vec<(u64, Evenement)> = Vec::with_capacity(b.titres.len());
            for (recue_a, octets) in &b.titres {
                if *recue_a > c.heure {
                    return Err(ErreurEtat::Incoherent("confirmation future"));
                }
                let brute =
                    decoder_titre::<Confirmation>(&Sceau::decoder(octets).map_err(titre)?.charge).map_err(titre)?;
                let kid_m = empreinte_cle(&brute.confirmatrice);
                if !self.dossiers.contains_key(&kid_m) || brute.reconnue != racine {
                    return Err(ErreurEtat::Incoherent("confirmation"));
                }
                let conf = verifier::<Confirmation>(octets, &ancre(&brute.confirmatrice).map_err(ErreurEtat::Titre)?, *recue_a)
                    .map_err(titre)?
                    .into_titre();
                evenements.push((*recue_a, Evenement::Titre(kid_m, conf, octets.to_vec())));
            }
            for l in self.levees.values().filter(|l| l.accuse == *kid) {
                evenements.push((l.instant, Evenement::Levee));
            }
            // Ordre de réception ; à égalité, la confirmation d'abord (sans effet : elle
            // commence au plus tôt à sa réception).
            evenements.sort_by_key(|(t, e)| (*t, matches!(e, Evenement::Levee)));
            if let Some(d) = self.dossiers.get_mut(kid) {
                for (t, e) in evenements {
                    match e {
                        Evenement::Titre(kid_m, conf, octets) => d.ajouter_confirmation(kid_m, &conf, &octets, t),
                        Evenement::Levee => d.relancer_compteur(t),
                    }
                }
            }
        }

        // Quotas (HMAC), groupés par membre.
        let mut dernier: Option<Kid> = None;
        for (m, confirme, premiere, derniere) in &c.quotas {
            if dernier.is_some_and(|d| d > *m) {
                return Err(ErreurEtat::Incoherent("ordre des quotas"));
            }
            dernier = Some(*m);
            if !self.dossiers.contains_key(m)
                || !self.dossiers.contains_key(confirme)
                || premiere > derniere
                || *derniere > c.heure
            {
                return Err(ErreurEtat::Incoherent("quota"));
            }
            self.quotas.entry(*m).or_default().push(Quota { confirme: *confirme, premiere: *premiere, derniere: *derniere });
        }

        // Note locale (HMAC).
        let mut precedent: Option<Kid> = None;
        for (kid, points, maj) in &c.fiches {
            if precedent.is_some_and(|p| p >= *kid) {
                return Err(ErreurEtat::Incoherent("ordre de la note"));
            }
            precedent = Some(*kid);
            self.note.restaurer(*kid, *points, *maj);
        }

        // Fraîcheur de la vue : jamais plus récente que la plus récente tête persistée.
        if let Some(v) = c.vue_a_jour_au {
            let plus_recente = self.dossiers.values().filter_map(|d| d.vue.as_ref()).map(|e| e.tete().emise_a).max();
            if plus_recente.is_none_or(|p| v > p) {
                return Err(ErreurEtat::Incoherent("fraîcheur de la vue"));
            }
            self.vue_a_jour_au = Some(v);
        }
        Ok(())
    }

    /// Ancres : identités de tous les dossiers connus sauf `exclu`.
    fn ancres_hors(&self, exclu: &Kid) -> Result<AncresDeConfiance, ErreurEtat> {
        let mut ancres = AncresDeConfiance::new();
        for (k, d) in &self.dossiers {
            if k != exclu {
                ancres.ajouter(&d.racine, NiveauConfiance::Reconnu).map_err(titre)?;
            }
        }
        Ok(ancres)
    }

    /// Accusation et preuve revérifiées à `depuis` : quorum de 6 signataires
    /// distincts connus (hors accusé), preuve concordante et valide.
    fn verifier_pieces(
        &self,
        s: &SanctionBrute<'_>,
        heure: u64,
    ) -> Result<(Accusation, [u8; 32], Kid, Pieces), ErreurEtat> {
        if s.depuis > heure {
            return Err(ErreurEtat::Incoherent("sanction future"));
        }
        let sceau = Sceau::decoder(s.accusation).map_err(titre)?;
        let brute = decoder_titre::<Accusation>(&sceau.charge).map_err(titre)?;
        let id = sha2_256(&sceau.octets_signes());
        let kid = empreinte_cle(&brute.accuse);
        let d = self.dossiers.get(&kid).ok_or(ErreurEtat::Incoherent("accusé inconnu"))?;
        let a = verifier_seuil::<Accusation>(s.accusation, &self.ancres_hors(&kid)?, QUORUM_BLACKLIST, s.depuis)
            .map_err(titre)?
            .into_titre();
        let preuve = Preuve::decoder(s.preuve).map_err(ErreurEtat::Titre)?;
        if a.empreinte_preuve != preuve.empreinte() || a.type_preuve != preuve.type_preuve() {
            return Err(ErreurEtat::Titre(ErreurFederation::PreuveNonConcordante));
        }
        preuve.verifier(&d.journal, &d.registre, s.depuis).map_err(ErreurEtat::Titre)?;
        Ok((a, id, kid, Pieces { accusation: s.accusation.to_vec(), preuve }))
    }

    // ── Rattrapage de l'arrêt ──────────────────────────────────────────────

    /// Fait avancer la vue reprise de `heure` à `maintenant` aux instants où
    /// un statut peut changer et aux échéances des suspensions (révocation
    /// datée de l'échéance, comme si le processus avait tourné).
    fn rattraper(&mut self, maintenant: u64) -> Result<Vec<RevocationFederale>, ErreurEtat> {
        if maintenant < self.heure {
            return Err(ErreurEtat::RetourHorloge);
        }
        let depart = self.heure;
        let dans = |t: u64| (depart..=maintenant).contains(&t);
        let mut instants: Vec<u64> =
            self.suspensions.values().map(|s| s.publique.echeance().max(depart)).filter(|t| dans(*t)).collect();
        for d in self.dossiers.values() {
            for &(s, e) in d.confirmations.values().flatten() {
                let fin = e.saturating_add(1);
                for t in [s, fin, s.saturating_add(DUREE_MEMBRE_SECS), fin.saturating_add(DUREE_MEMBRE_SECS)] {
                    if dans(t) {
                        instants.push(t);
                    }
                }
            }
        }
        instants.sort_unstable();
        instants.dedup();
        let mut definitives = Vec::new();
        for t in instants {
            self.heure = t;
            definitives.extend(self.echeances(t));
            self.recalculer(t);
        }
        definitives.extend(self.actualiser(maintenant).map_err(|_| ErreurEtat::RetourHorloge)?);
        Ok(definitives)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyusceau::CleSignature;

    fn cle() -> CleEtatFederal {
        CleEtatFederal::deriver(&[3; 32])
    }

    const ANCRE: AncreRegistre = AncreRegistre { taille: 3, racine: [7; 32] };

    fn fed() -> Federation {
        Federation::new(Parametres::default().avec_difficulte(8).unwrap(), None)
    }

    const T: u64 = 1_800_000_000;

    /// Deux fondateurs, un candidat confirmé par les deux.
    fn exemple() -> (Federation, Vec<CleSignature>) {
        let ids: Vec<CleSignature> = (1..=3u8).map(|g| CleSignature::depuis_graine(&[g; 32])).collect();
        let mut f = fed();
        for (i, id) in ids.iter().enumerate() {
            let journal = CleSignature::depuis_graine(&[100 + i as u8; 32]);
            let c = Candidature::preparer(id.cle_publique(), journal.cle_publique(), [i as u8; 32], T, 8)
                .unwrap()
                .sceller(id)
                .unwrap();
            if i < 2 {
                f.ajouter_fondateur(&c, T).unwrap();
            } else {
                f.recevoir_candidature(&c, T).unwrap();
            }
        }
        for m in &ids[..2] {
            let c = Confirmation::nouvelle(ids[2].cle_publique(), m.cle_publique(), T + 10).sceller(m).unwrap();
            f.recevoir_confirmation(&c, T + 10).unwrap();
        }
        f.actualiser(T + 10).unwrap();
        (f, ids)
    }

    #[test]
    fn reseller_a_l_identique() {
        let (f, _) = exemple();
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        let (g, rev, ancre) = Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10).unwrap();
        assert_eq!(rev, Vec::new());
        assert_eq!(ancre, ANCRE);
        assert_eq!(g.sceller_etat(&cle(), &ANCRE).unwrap(), o);
    }

    #[test]
    fn titre_altere_refuse_malgre_un_mac_valide() {
        let (mut f, _) = exemple();
        // Signature d'une confirmation altérée, puis état scellé avec la bonne clé :
        // le HMAC est valide, la revérification refuse.
        let d = f.dossiers.values_mut().find(|d| !d.titres.is_empty()).unwrap();
        let t = d.titres.values_mut().next().unwrap().first_mut().unwrap();
        let dernier = t.octets.len() - 1;
        t.octets[dernier] ^= 1;
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        assert!(matches!(
            Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10),
            Err(ErreurEtat::Titre(_))
        ));
    }

    #[test]
    fn statut_membre_incoherent_refuse() {
        let (mut f, _) = exemple();
        // Invité marqué « membre » sans le drapeau : incohérent, refusé.
        let d = f.dossiers.values_mut().find(|d| !d.fondateur).unwrap();
        assert_eq!(d.statut, Statut::Invite);
        d.statut = Statut::Membre;
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        assert!(matches!(
            Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10),
            Err(ErreurEtat::Incoherent(_))
        ));
    }

    #[test]
    fn version_1_refusee() {
        let (f, _) = exemple();
        let mut o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        o[4] = 0x01;
        assert!(matches!(
            Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10),
            Err(ErreurEtat::VersionInconnue(0x01))
        ));
    }

    #[test]
    fn file_a_publier_et_inscrites_coherentes() {
        let (mut f, _) = exemple();
        // Un `kid` à publier sans révocation : incohérent (HMAC pourtant valide).
        f.a_publier.insert([9; 32]);
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        assert!(matches!(
            Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10),
            Err(ErreurEtat::Incoherent(_))
        ));
        f.a_publier.clear();
        // Révocation inscrite d'un `kid` inconnu : conservée telle quelle.
        f.inscrites.insert([9; 32], T);
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        let (g, _, _) = Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 10).unwrap();
        assert!(g.est_revoque(&[9; 32]));
        assert_eq!(g.sceller_etat(&cle(), &ANCRE).unwrap(), o);
    }

    #[test]
    fn heure_en_recul_refusee() {
        let (f, _) = exemple();
        let o = f.sceller_etat(&cle(), &ANCRE).unwrap();
        assert!(matches!(
            Federation::reprendre(&o, &cle(), *f.parametres(), None, T + 9),
            Err(ErreurEtat::RetourHorloge)
        ));
    }
}
