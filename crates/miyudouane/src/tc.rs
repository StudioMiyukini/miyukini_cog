//! @id     miyudouane_tc
//! @do     generer_auditer_mise_en_forme_tc
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — mise en forme du trafic (`tc -batch`) depuis les débits déclarés : une classe HTB par voie, une feuille fq_codel par flux, classification par marque, audit (A-022 §7).

//! # Mise en forme du trafic (`tc`, A-022 §7)
//!
//! Génération **pure** (aucun appel système) du texte passé à `tc -batch`
//! dans l'espace de noms de la frontière **externe**, depuis les débits
//! déclarés au manifeste (contrat tag 5, champ 8 « entrées externes », Kio/s),
//! **tels que scellés dans l'arrêté** par BorderGuard
//! (`miyuhybride::douane::entrees_douane` → flux de l'arrêté → [`compiler`]).
//! L'arrêté vérifié est la seule source des débits et des marques : la mise en
//! forme et les règles nftables ([`crate::nft`]) appliquent exactement les
//! mêmes flux, avec les mêmes marques conntrack.
//!
//! ## Dépassement : mise en file, jamais refus
//!
//! Chaque flux a une feuille HTB `rate = ceil = débit déclaré` et une file
//! `fq_codel` : au-delà du débit, les paquets **attendent** dans la file.
//!
//! **Une fois cette mise en forme appliquée avec succès** (et seulement
//! alors, sur la frontière externe seule), la règle nftables `DEBIT` peut
//! passer de la forme stricte (`limit rate over D`, [`crate::nft`]) à un
//! **filet de sécurité** au-delà d'une marge, jamais atteint par du trafic mis
//! en forme ([`crate::nft::MiseEnForme::Appliquee`], ordre sûr :
//! [`crate::application`]). Tant que `tc` n'est pas déclaré appliqué, et sur
//! toute autre frontière, la règle reste **stricte** (aucune marge, aucune
//! rafale ajoutée) : un plafond relevé sans mise en forme serait une
//! régression de contrôle.
//!
//! | | valeur | unité de compte |
//! |---|---|---|
//! | règle nftables stricte (`DEBIT`, sans `tc` déclaré) | `D` Kio/s par chaîne, sans rafale ajoutée | octets IP |
//! | plafond `tc` (feuille HTB) | `D` | octets de trame (en-tête L2 compris) |
//! | rafale `tc` (`burst` = `cburst`) | `B = ⌈D × 1024 / 100⌉ + 1600` o (10 ms de débit + une trame) | |
//! | filet nftables (`DEBIT`) | `⌈D × 5 / 4⌉` Kio/s **par sens**, rafale `B` o | octets IP |
//!
//! Rapport filet / plafond = [`FILET_NUMERATEUR`] / [`FILET_DENOMINATEUR`]
//! (125 %), même rafale : sur toute fenêtre `t`, HTB émet au plus `D·t + B`
//! octets de trame, soit moins d'octets IP ; le seau nftables en admet
//! `1,25·D·(t + 1 s) + B`. Le filet ne mord donc que sur du trafic **non mis en
//! forme** (classe absente, file contournée, configuration `tc` retirée).
//! Le filet est posé **par sens** (`ct direction original` / `reply`) : la
//! mise en forme agit sur chaque interface de sortie, donc sur chaque sens
//! séparément.
//!
//! `fq_codel` peut encore écarter un paquet (gestion active de file : signal
//! de congestion, remplacé par un marquage ECN quand le flux le permet ; file
//! pleine au-delà de [`FQ_LIMITE`] paquets) ; ce n'est pas un refus de
//! politique et aucune ligne de journal Douane n'en résulte. La cible CoDel
//! s'adapte aux bas débits : `cible = max(5 ms, 1,5 × temps d'émission d'un
//! paquet de taille maximale)`, `intervalle = 100 ms + (cible − 5 ms)`.
//!
//! ## Arbre produit (par interface de la frontière)
//!
//! ```text
//! 4d44:        htb default ffff                  racine (poignée « MD », propre à la Douane)
//! └─ 4d44:f000 capacité de l'interface
//!    ├─ 4d44:ffff  défaut (trafic non marqué) : rate = capacité − Σ, ceil = capacité ; file 4d45: fq_codel
//!    └─ 4d44:1xxx  une classe par voie : rate = ceil = Σ des débits de ses flux sur cette interface
//!       └─ 4d44:<m>  une feuille par flux (m = marque conntrack, 1..=fff) : rate = ceil = D ; file <m>: fq_codel
//! filtre : parent 4d44: protocol all prio 10 handle 0x<m> fw classid 4d44:<m>
//! ```
//!
//! - **Classification par marque**, jamais par adresse : la Douane copie la
//!   marque conntrack du flux dans la marque du paquet (`meta mark set ct
//!   mark`, chaînes de [`crate::nft`]) et le classifieur `fw` l'associe à la
//!   feuille du flux.
//! - Frontière externe en routage : le sens **original** d'un flux sort par
//!   son interface de sortie (`oif`), le sens **réponse** par son interface
//!   d'entrée (`iif`) : chaque interface porte la feuille du flux pour le sens
//!   qui la quitte, avec le même débit déclaré.
//! - Toutes les interfaces de la frontière (liens et interfaces extérieures)
//!   reçoivent l'arbre, même sans flux (racine et défaut seulement) : la liste
//!   des interfaces touchées est fixe, une voie retirée ou coupée disparaît.
//! - La voie `v` (ordre des identifiants) a la classe `0x1000 + rang` ; les
//!   voies coupées (`--couper`, EX-49) sont omises comme dans nftables.
//!
//! ## Remplacement, propriété, audit
//!
//! - Le texte n'utilise que `qdisc add`, `class add`, `filter add`, sur les
//!   seules interfaces de la frontière et sous la seule racine `4d44:` :
//!   aucun `del`, `replace`, `change`, aucune action, aucun `ingress`.
//! - `tc` n'a pas de transaction. L'étape privilégiée
//!   ([`crate::application::appliquer_tc`]) refuse **avant toute
//!   modification** si une interface porte une racine qui n'est ni celle de la
//!   Douane (`htb 4d44:`) ni la file par défaut du noyau (poignée `0:`) ;
//!   elle retire ensuite la seule racine `4d44:` de la Douane, puis exécute le
//!   texte en **un** `tc -batch`. Le premier `qdisc add … root handle 4d44:`
//!   échoue (`EEXIST`) si une racine explicite étrangère est apparue entre
//!   temps : une configuration qui n'est pas la sienne n'est jamais remplacée.
//!   Elle exige aussi que les règles **strictes** soient en place (table
//!   `inet miyudouane` présente, aucun filet) : pendant le remplacement
//!   (quelques millisecondes) le trafic passe par la file par défaut sous la
//!   règle stricte ; en cas d'échec, la racine de la Douane est retirée
//!   (retour à la file par défaut, règle stricte conservée).
//! - [`auditer_tc`] refuse tout écart au rendu attendu (comparaison ligne à
//!   ligne) et vérifie aussi la structure sans dépendre du générateur.
//!
//! ## Description (`miyudouane-tc 1`)
//!
//! Texte ASCII, une directive par ligne, ≤ [`TAILLE_MAX_DESCRIPTION`] octets,
//! lignes ≤ [`LONGUEUR_MAX_LIGNE`] ; `#` en début de ligne = commentaire.
//!
//! ```text
//! miyudouane-tc 1                     # en-tête obligatoire, première ligne utile
//! capacite wg-fed 12500               # capacité d'une interface de la frontière (Kio/s), facultatif
//! ```
//!
//! La description ne porte **aucun débit de flux** : ils viennent de l'arrêté
//! signé. Elle ne déclare que la capacité des interfaces (racine HTB), défaut
//! [`CAPACITE_DEFAUT_KIO_S`], bornée par [`CAPACITE_MAX_KIO_S`] ; interface
//! inconnue de la frontière, doublon, valeur hors bornes ⇒ refus. La somme des
//! débits des flux d'une interface doit rester **strictement** inférieure à sa
//! capacité (sinon refus : les garanties ne seraient pas tenues).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Write as _};

use borderguard::{ArreteVerifie, Frontiere};

use crate::compilation::{compiler, Crochet};
use crate::hote::ErreurDescription;
use crate::plan::{DefinitionFrontiere, ModeFrontiere, PlanReseau, LONGUEUR_MAX_INTERFACE};

/// En-tête de la description.
pub const ENTETE_DESCRIPTION: &str = "miyudouane-tc 1";
/// Taille maximale d'une description (octets).
pub const TAILLE_MAX_DESCRIPTION: usize = 4 * 1024;
/// Longueur maximale d'une ligne de description.
pub const LONGUEUR_MAX_LIGNE: usize = 256;
/// Nombre maximal de directives `capacite`.
pub const MAX_CAPACITES: usize = 16;
/// Capacité par défaut d'une interface (Kio/s, ≈ 10 Gbit/s).
pub const CAPACITE_DEFAUT_KIO_S: u32 = 1_250_000;
/// Capacité maximale admise (Kio/s) : le débit en octets/s tient sur 32 bits.
pub const CAPACITE_MAX_KIO_S: u32 = 4_000_000;

/// Poignée (majeur) de la racine HTB de la Douane (« MD »).
pub const POIGNEE_RACINE: u16 = 0x4d44;
/// Poignée (majeur) de la file `fq_codel` de la classe par défaut.
pub const POIGNEE_FILE_DEFAUT: u16 = 0x4d45;
/// Mineur de la classe racine (capacité de l'interface).
pub const CLASSE_RACINE: u16 = 0xf000;
/// Mineur de la classe par défaut (trafic non marqué).
pub const CLASSE_DEFAUT: u16 = 0xffff;
/// Mineur de la première classe de voie (`0x1000 + rang`).
pub const BASE_CLASSE_VOIE: u16 = 0x1000;
/// Plus grande marque admise (mineur de feuille, sous [`BASE_CLASSE_VOIE`]).
pub const MARQUE_MAX: u32 = 0x0fff;
/// Priorité des filtres `fw`.
pub const PRIORITE_FILTRE: u16 = 10;
/// Quantum HTB (octets) : explicite, aucun avertissement `r2q`.
pub const QUANTUM: u32 = 1600;
/// Trame maximale prise en compte dans la rafale (octets, MTU HTB par défaut).
pub const TRAME_MAX: u64 = 1600;
/// Rapport filet / plafond : numérateur (5/4 = 125 %).
pub const FILET_NUMERATEUR: u64 = 5;
/// Rapport filet / plafond : dénominateur.
pub const FILET_DENOMINATEUR: u64 = 4;
/// Limite d'une file `fq_codel` (paquets).
pub const FQ_LIMITE: u32 = 1024;
/// Nombre de sous-files `fq_codel`.
pub const FQ_FLUX: u32 = 1024;
/// Cible CoDel minimale (µs).
pub const CIBLE_MIN_US: u64 = 5_000;
/// Intervalle CoDel de base (µs).
pub const INTERVALLE_BASE_US: u64 = 100_000;

/// Filet nftables (`limit rate over`, Kio/s) d'un débit déclaré : `⌈D × 5/4⌉`.
pub fn filet_kio_s(debit_kio_s: u32) -> u64 {
    (u64::from(debit_kio_s) * FILET_NUMERATEUR).div_ceil(FILET_DENOMINATEUR)
}

/// Rafale (octets) d'un débit : 10 ms de débit + une trame. Sert de `burst` /
/// `cburst` HTB et de `burst` au filet nftables.
pub fn rafale_octets(debit_kio_s: u32) -> u64 {
    (u64::from(debit_kio_s) * 1024).div_ceil(100) + TRAME_MAX
}

/// Cible et intervalle CoDel (µs) pour un débit et une taille de paquet.
pub fn codel_us(debit_kio_s: u32, taille_max_paquet: u16) -> (u64, u64) {
    let octets_s = (u64::from(debit_kio_s) * 1024).max(1);
    let emission = (u64::from(taille_max_paquet) * 1_000_000).div_ceil(octets_s);
    let cible = CIBLE_MIN_US.max((emission * 3).div_ceil(2));
    (cible, INTERVALLE_BASE_US + (cible - CIBLE_MIN_US))
}

/// Refus de planification (aucun texte produit).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurTc(pub String);

impl fmt::Display for ErreurTc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mise en forme refusée : {}", self.0)
    }
}

impl std::error::Error for ErreurTc {}

/// Description validée (capacités d'interfaces).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DescriptionTc {
    /// Capacité déclarée par interface (Kio/s).
    pub capacites: BTreeMap<String, u32>,
}

fn interface_syntaxe(nom: &str) -> bool {
    !nom.is_empty()
        && nom.len() <= LONGUEUR_MAX_INTERFACE
        && nom.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

impl DescriptionTc {
    /// Analyse stricte (voir le format au module).
    pub fn analyser(texte: &str) -> Result<Self, ErreurDescription> {
        if texte.len() > TAILLE_MAX_DESCRIPTION {
            return Err(ErreurDescription { ligne: 0, motif: "taille maximale dépassée".into() });
        }
        let mut entete = false;
        let mut d = Self::default();
        for (n, brute) in texte.split('\n').enumerate() {
            let n = n + 1;
            let refus = |motif: String| ErreurDescription { ligne: n, motif };
            if brute.len() > LONGUEUR_MAX_LIGNE {
                return Err(refus("ligne trop longue".into()));
            }
            if !brute.bytes().all(|b| (0x20..=0x7e).contains(&b) || b == b'\t') {
                return Err(refus("caractère non ASCII ou de contrôle".into()));
            }
            let l = brute.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            let mots: Vec<&str> = l.split_ascii_whitespace().collect();
            if !entete {
                if mots.join(" ") != ENTETE_DESCRIPTION {
                    return Err(refus(format!("en-tête « {ENTETE_DESCRIPTION} » attendu")));
                }
                entete = true;
                continue;
            }
            match mots.as_slice() {
                ["capacite", interface, valeur] => {
                    if !interface_syntaxe(interface) {
                        return Err(refus(format!("nom d'interface invalide « {interface} »")));
                    }
                    let c = crate::hote::entier(valeur)
                        .and_then(|v| u32::try_from(v).ok())
                        .filter(|v| (1..=CAPACITE_MAX_KIO_S).contains(v))
                        .ok_or_else(|| refus(format!("capacité hors bornes « {valeur} » (1..={CAPACITE_MAX_KIO_S} Kio/s)")))?;
                    if d.capacites.insert((*interface).to_owned(), c).is_some() {
                        return Err(refus(format!("capacité en double pour « {interface} »")));
                    }
                    if d.capacites.len() > MAX_CAPACITES {
                        return Err(refus("nombre maximal d'entrées dépassé".into()));
                    }
                }
                [mot, ..] => return Err(refus(format!("directive inconnue ou mal formée « {mot} »"))),
                [] => {}
            }
        }
        if !entete {
            return Err(ErreurDescription { ligne: 0, motif: "en-tête absent".into() });
        }
        Ok(d)
    }
}

/// Sens d'un flux sur une interface de sortie.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sens {
    /// Sens de l'initiateur (sort par l'`oif` du flux).
    Original,
    /// Réponses (sortent par l'`iif` du flux).
    Reponse,
}

impl Sens {
    /// Mot nftables (`ct direction …`).
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Reponse => "reply",
        }
    }
}

/// Feuille d'un flux sur une interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxTc {
    /// Marque conntrack du flux (mineur de la feuille).
    pub marque: u32,
    /// Rang du flux dans l'arrêté.
    pub index_flux: usize,
    /// Sens qui quitte cette interface.
    pub sens: Sens,
    /// Débit déclaré (Kio/s).
    pub debit_kio_s: u32,
    /// Taille maximale de paquet déclarée.
    pub taille_max_paquet: u16,
}

/// Classe d'une voie sur une interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoieTc {
    /// Identifiant de voie.
    pub voie: String,
    /// Service propriétaire.
    pub service: String,
    /// Mineur de la classe (`0x1000 + rang`).
    pub classe: u16,
    /// Feuilles (triées par marque).
    pub flux: Vec<FluxTc>,
}

impl VoieTc {
    /// Débit de la classe de voie : somme des débits de ses feuilles.
    pub fn debit_kio_s(&self) -> u64 {
        self.flux.iter().map(|f| u64::from(f.debit_kio_s)).sum()
    }
}

/// Arbre d'une interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceTc {
    /// Nom de l'interface (espace de la frontière).
    pub interface: String,
    /// Capacité (Kio/s).
    pub capacite_kio_s: u32,
    /// Classes de voie (triées par identifiant).
    pub voies: Vec<VoieTc>,
}

impl InterfaceTc {
    /// Somme des débits garantis des voies.
    pub fn somme_kio_s(&self) -> u64 {
        self.voies.iter().map(VoieTc::debit_kio_s).sum()
    }
}

/// Plan de mise en forme d'une frontière (rendu par [`generer_tc`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanTc {
    /// Espace de noms de la frontière.
    pub netns: String,
    /// Lignes d'information de l'en-tête (arrêté, empreinte).
    pub informations: Vec<String>,
    /// Interfaces (triées par nom), toutes celles de la frontière.
    pub interfaces: Vec<InterfaceTc>,
}

/// Interfaces d'une frontière (liens et interfaces extérieures) : toutes
/// reçoivent l'arbre `tc`, et toutes doivent le porter avant le filet
/// ([`crate::application::appliquer_avec_filet`]).
pub fn interfaces_frontiere(def: &DefinitionFrontiere) -> BTreeSet<String> {
    let mut noms: BTreeSet<String> = def.liens.iter().map(|l| l.interface.clone()).collect();
    noms.extend(def.exterieures.iter().map(|e| e.interface.clone()));
    noms
}

/// Planifie la mise en forme de la frontière externe depuis un arrêté vérifié
/// (débits et marques) et une description (capacités). Les voies de `coupees`
/// sont omises (EX-49).
pub fn planifier(
    arrete: &ArreteVerifie,
    plan: &PlanReseau,
    description: &DescriptionTc,
    coupees: &BTreeSet<String>,
) -> Result<PlanTc, ErreurTc> {
    let a = arrete.arrete();
    if a.frontiere != Frontiere::Externe {
        return Err(ErreurTc(format!(
            "frontière {} : la mise en forme ne porte que sur les entrées externes (frontière externe)",
            a.frontiere
        )));
    }
    let def = plan.frontiere(a.frontiere).ok_or_else(|| ErreurTc("frontière externe absente du plan".into()))?;
    if def.mode != ModeFrontiere::Routage {
        return Err(ErreurTc("frontière externe hors mode routage".into()));
    }
    let noms = interfaces_frontiere(def);
    if let Some(i) = description.capacites.keys().find(|i| !noms.contains(*i)) {
        return Err(ErreurTc(format!("capacité pour « {i} » : interface absente de la frontière externe")));
    }
    let regles = compiler(arrete, plan).map_err(|e| ErreurTc(e.to_string()))?;
    let rangs: BTreeMap<&str, u16> = {
        let ids: BTreeSet<&str> = regles.iter().map(|r| r.voie.as_str()).collect();
        let mut m = BTreeMap::new();
        for (rang, id) in ids.into_iter().enumerate() {
            let rang = u16::try_from(rang)
                .ok()
                .filter(|r| *r < CLASSE_RACINE - BASE_CLASSE_VOIE)
                .ok_or_else(|| ErreurTc("trop de voies".into()))?;
            m.insert(id, BASE_CLASSE_VOIE + rang);
        }
        m
    };
    // interface → voie → (service, classe, feuilles)
    let mut arbre: BTreeMap<String, BTreeMap<String, VoieTc>> = noms.iter().map(|n| (n.clone(), BTreeMap::new())).collect();
    for r in regles.iter().filter(|r| !coupees.contains(&r.voie)) {
        if r.crochet != Crochet::Transit {
            return Err(ErreurTc(format!("voie {} : règle hors transit sur la frontière externe", r.voie)));
        }
        let (Some(iif), Some(oif)) = (r.iif.as_deref(), r.oif.as_deref()) else {
            return Err(ErreurTc(format!("voie {} : interfaces d'entrée / de sortie attendues", r.voie)));
        };
        if r.marque == 0 || r.marque > MARQUE_MAX {
            return Err(ErreurTc(format!("voie {} : marque {:#x} hors 1..={MARQUE_MAX:#x}", r.voie, r.marque)));
        }
        if r.debit_max_kio_s == 0 || r.debit_max_kio_s > CAPACITE_MAX_KIO_S {
            return Err(ErreurTc(format!(
                "voie {} : débit {} Kio/s hors 1..={CAPACITE_MAX_KIO_S}",
                r.voie, r.debit_max_kio_s
            )));
        }
        let classe = rangs.get(r.voie.as_str()).copied().ok_or_else(|| ErreurTc("voie sans rang".into()))?;
        for (interface, sens) in [(oif, Sens::Original), (iif, Sens::Reponse)] {
            let voies = arbre
                .get_mut(interface)
                .ok_or_else(|| ErreurTc(format!("voie {} : interface « {interface} » hors frontière", r.voie)))?;
            let v = voies.entry(r.voie.clone()).or_insert_with(|| VoieTc {
                voie: r.voie.clone(),
                service: r.service.clone(),
                classe,
                flux: Vec::new(),
            });
            v.flux.push(FluxTc {
                marque: r.marque,
                index_flux: r.index_flux,
                sens,
                debit_kio_s: r.debit_max_kio_s,
                taille_max_paquet: r.taille_max_paquet,
            });
        }
    }
    let mut interfaces = Vec::new();
    for (interface, voies) in arbre {
        let capacite = description.capacites.get(&interface).copied().unwrap_or(CAPACITE_DEFAUT_KIO_S);
        let mut voies: Vec<VoieTc> = voies.into_values().collect();
        for v in &mut voies {
            v.flux.sort_by_key(|f| f.marque);
            if v.flux.windows(2).any(|w| w[0].marque == w[1].marque) {
                return Err(ErreurTc(format!("voie {} : marque en double sur {interface}", v.voie)));
            }
        }
        let i = InterfaceTc { interface, capacite_kio_s: capacite, voies };
        if i.somme_kio_s() >= u64::from(capacite) {
            return Err(ErreurTc(format!(
                "{} : somme des débits déclarés {} Kio/s ≥ capacité {capacite} Kio/s (déclarer « capacite »)",
                i.interface,
                i.somme_kio_s()
            )));
        }
        interfaces.push(i);
    }
    let mut informations = vec![
        format!("frontiere={} arrete={} empreinte={}", a.frontiere, a.numero, arrete.empreinte_hex()),
        format!("validite={}..{} emetteur={}", a.pas_avant, a.pas_apres, a.emetteur),
    ];
    if !coupees.is_empty() {
        informations.push(format!("voies coupees : {}", coupees.iter().cloned().collect::<Vec<_>>().join(",")));
    }
    Ok(PlanTc { netns: def.netns.clone(), informations, interfaces })
}

fn classe_htb(t: &mut String, dev: &str, parent: &str, classe: u16, rate: u64, ceil: u64) {
    let _ = writeln!(
        t,
        "class add dev {dev} parent {parent} classid {POIGNEE_RACINE:x}:{classe:x} htb rate {rate}KiBps ceil {ceil}KiBps \
         burst {} cburst {} quantum {QUANTUM}",
        rafale_u64(rate),
        rafale_u64(ceil)
    );
}

fn rafale_u64(debit_kio_s: u64) -> u64 {
    (debit_kio_s * 1024).div_ceil(100) + TRAME_MAX
}

fn file_fq_codel(t: &mut String, dev: &str, parent: u16, poignee: u16, debit_kio_s: u64, taille: u16) {
    let (cible, intervalle) = codel_us(u32::try_from(debit_kio_s).unwrap_or(u32::MAX), taille);
    let _ = writeln!(
        t,
        "qdisc add dev {dev} parent {POIGNEE_RACINE:x}:{parent:x} handle {poignee:x}: fq_codel limit {FQ_LIMITE} \
         flows {FQ_FLUX} target {cible}us interval {intervalle}us ecn"
    );
}

/// Taille de paquet de référence de la file par défaut (trame Ethernet).
const TAILLE_DEFAUT: u16 = 1500;

/// Texte `tc -batch` d'un plan (pur, déterministe).
pub fn generer_tc(p: &PlanTc) -> String {
    let mut t = String::new();
    let _ = writeln!(t, "# Généré par miyudouane tc — ne pas modifier à la main.");
    let _ = writeln!(t, "# Douane — mise en forme du trafic (A-022 §7), espace {} : tc -batch", p.netns);
    for l in &p.informations {
        let _ = writeln!(t, "# {l}");
    }
    let _ = writeln!(t, "# une classe HTB par voie, une feuille fq_codel par flux (marque conntrack -> meta mark -> fw) ;");
    let _ = writeln!(
        t,
        "# depassement = mise en file ; nftables DEBIT strict, puis (apres succes) filet x {FILET_NUMERATEUR}/{FILET_DENOMINATEUR} par sens."
    );
    let racine = format!("{POIGNEE_RACINE:x}:{CLASSE_RACINE:x}");
    for i in &p.interfaces {
        let dev = &i.interface;
        let cap = u64::from(i.capacite_kio_s);
        let somme = i.somme_kio_s();
        let _ = writeln!(t);
        let _ = writeln!(t, "# interface {dev} capacite {cap} KiB/s, garanti aux voies {somme} KiB/s");
        let _ = writeln!(t, "qdisc add dev {dev} root handle {POIGNEE_RACINE:x}: htb default {CLASSE_DEFAUT:x}");
        classe_htb(&mut t, dev, &format!("{POIGNEE_RACINE:x}:"), CLASSE_RACINE, cap, cap);
        classe_htb(&mut t, dev, &racine, CLASSE_DEFAUT, cap - somme, cap);
        file_fq_codel(&mut t, dev, CLASSE_DEFAUT, POIGNEE_FILE_DEFAUT, cap - somme, TAILLE_DEFAUT);
        for v in &i.voies {
            let dv = v.debit_kio_s();
            let _ = writeln!(t, "# voie={} service={} debit={dv} KiB/s", v.voie, v.service);
            classe_htb(&mut t, dev, &racine, v.classe, dv, dv);
            for f in &v.flux {
                let d = u64::from(f.debit_kio_s);
                let _ = writeln!(
                    t,
                    "# flux={} marque=0x{:08x} sens={} debit={d} KiB/s taille_max={}",
                    f.index_flux,
                    f.marque,
                    f.sens.nom(),
                    f.taille_max_paquet
                );
                let m = u16::try_from(f.marque).unwrap_or(u16::MAX);
                classe_htb(&mut t, dev, &format!("{POIGNEE_RACINE:x}:{:x}", v.classe), m, d, d);
                file_fq_codel(&mut t, dev, m, m, d, f.taille_max_paquet);
                let _ = writeln!(
                    t,
                    "filter add dev {dev} parent {POIGNEE_RACINE:x}: protocol all prio {PRIORITE_FILTRE} handle {:#x} fw \
                     classid {POIGNEE_RACINE:x}:{m:x}",
                    f.marque
                );
            }
        }
    }
    t
}

/// Mots interdits dans un texte `tc` de la Douane (suppression, remplacement,
/// actions, entrée, programmes).
const MOTS_INTERDITS: [&str; 16] = [
    "del", "delete", "replace", "change", "link", "ingress", "clsact", "action", "police", "mirred", "bpf",
    "exec", "block", "flush", "egress", "skbedit",
];

/// Valeur suivant `cle` dans une ligne découpée.
fn apres<'a>(mots: &[&'a str], cle: &str) -> Option<&'a str> {
    mots.iter().position(|m| *m == cle).and_then(|i| mots.get(i + 1)).copied()
}

/// Audit d'un texte `tc -batch` contre le plan attendu. Renvoie les écarts
/// (vide = conforme) :
///
/// - structure (sans dépendre du générateur) : seulement `qdisc add`,
///   `class add`, `filter add` ; aucun mot de [`MOTS_INTERDITS`] ; seulement
///   les interfaces du plan ; une seule racine par interface, `handle 4d44:
///   htb` ; toute classe et tout filtre sous `4d44:` ; chaque feuille de flux
///   (`4d44:<m>`, `m` < `0x1000`) existe au plan pour cette interface, avec
///   `rate` = `ceil` = débit déclaré ; chaque filtre `fw` associe la marque `m`
///   à la feuille `4d44:<m>` et à elle seule ;
/// - rendu : le texte est identique, ligne à ligne, au rendu attendu
///   ([`generer_tc`]) ; les premières lignes divergentes sont signalées.
pub fn auditer_tc(texte: &str, attendu: &PlanTc) -> Vec<String> {
    let mut ecarts = Vec::new();
    let racine = format!("{POIGNEE_RACINE:x}:");
    let feuilles: BTreeMap<(&str, u32), &FluxTc> = attendu
        .interfaces
        .iter()
        .flat_map(|i| i.voies.iter().flat_map(move |v| v.flux.iter().map(move |f| ((i.interface.as_str(), f.marque), f))))
        .collect();
    let mut racines: BTreeSet<String> = BTreeSet::new();
    let mut classes: BTreeSet<(String, String)> = BTreeSet::new();
    let mut filtres: BTreeSet<(String, u32)> = BTreeSet::new();
    for (n, brute) in texte.lines().enumerate() {
        let n = n + 1;
        let l = brute.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        if l.contains('#') {
            ecarts.push(format!("ligne {n} : commentaire en fin de commande"));
        }
        let mots: Vec<&str> = l.split_whitespace().collect();
        if let Some(m) = mots.iter().find(|m| MOTS_INTERDITS.contains(m)) {
            ecarts.push(format!("ligne {n} : mot interdit « {m} »"));
            continue;
        }
        let (objet, verbe) = (mots.first().copied().unwrap_or(""), mots.get(1).copied().unwrap_or(""));
        if !matches!(objet, "qdisc" | "class" | "filter") || verbe != "add" || mots.get(2) != Some(&"dev") {
            ecarts.push(format!("ligne {n} : commande hors qdisc/class/filter add dev"));
            continue;
        }
        let dev = mots.get(3).copied().unwrap_or("");
        if !attendu.interfaces.iter().any(|i| i.interface == dev) {
            ecarts.push(format!("ligne {n} : interface « {dev} » hors frontière"));
            continue;
        }
        match objet {
            "qdisc" if mots.get(4) == Some(&"root") => {
                let defaut = format!("{CLASSE_DEFAUT:x}");
                if mots[4..] != ["root", "handle", racine.as_str(), "htb", "default", defaut.as_str()] {
                    ecarts.push(format!("ligne {n} : racine étrangère à la Douane"));
                }
                if !racines.insert(dev.to_owned()) {
                    ecarts.push(format!("ligne {n} : racine en double sur {dev}"));
                }
            }
            "qdisc" => {
                let parent = apres(&mots, "parent").unwrap_or("");
                if !parent.starts_with(&racine) || !mots.contains(&"fq_codel") {
                    ecarts.push(format!("ligne {n} : file hors de l'arbre de la Douane"));
                }
            }
            "class" => {
                let parent = apres(&mots, "parent").unwrap_or("");
                let classid = apres(&mots, "classid").unwrap_or("");
                if !parent.starts_with(&racine) || !classid.starts_with(&racine) || !mots.contains(&"htb") {
                    ecarts.push(format!("ligne {n} : classe hors de l'arbre de la Douane"));
                    continue;
                }
                if !classes.insert((dev.to_owned(), classid.to_owned())) {
                    ecarts.push(format!("ligne {n} : classe {classid} en double sur {dev}"));
                }
                let mineur = u32::from_str_radix(&classid[racine.len()..], 16).unwrap_or(u32::MAX);
                if (1..u32::from(BASE_CLASSE_VOIE)).contains(&mineur) {
                    let attendue = format!("{}KiBps", feuilles.get(&(dev, mineur)).map_or(0, |f| f.debit_kio_s));
                    match feuilles.get(&(dev, mineur)) {
                        None => ecarts.push(format!("ligne {n} : feuille {classid} non déclarée sur {dev}")),
                        Some(_) if apres(&mots, "rate") != Some(&attendue) || apres(&mots, "ceil") != Some(&attendue) => {
                            ecarts.push(format!("ligne {n} : feuille {classid} sur {dev} : débit ≠ {attendue}"));
                        }
                        Some(_) => {}
                    }
                }
            }
            _ => {
                let handle = apres(&mots, "handle").and_then(|h| u32::from_str_radix(h.trim_start_matches("0x"), 16).ok());
                let classid = apres(&mots, "classid").unwrap_or("");
                let correct = mots.get(4) == Some(&"parent")
                    && mots.get(5) == Some(&racine.as_str())
                    && mots.contains(&"fw")
                    && handle.is_some_and(|m| classid == format!("{racine}{m:x}") && feuilles.contains_key(&(dev, m)));
                if !correct {
                    ecarts.push(format!("ligne {n} : filtre hors classification marque → feuille déclarée"));
                } else if let Some(m) = handle {
                    filtres.insert((dev.to_owned(), m));
                }
            }
        }
    }
    for i in &attendu.interfaces {
        if !racines.contains(&i.interface) {
            ecarts.push(format!("interface {} sans racine de la Douane", i.interface));
        }
    }
    for (dev, m) in feuilles.keys() {
        if !classes.contains(&((*dev).to_owned(), format!("{racine}{m:x}"))) || !filtres.contains(&((*dev).to_owned(), *m)) {
            ecarts.push(format!("feuille ou filtre absent : {dev} marque {m:#x}"));
        }
    }
    let rendu = generer_tc(attendu);
    if texte != rendu {
        let mut lues = texte.lines();
        let mut attendues = rendu.lines();
        let mut signales = 0;
        for numero in 1.. {
            let (lue, attendue) = (lues.next(), attendues.next());
            if lue.is_none() && attendue.is_none() {
                break;
            }
            if lue != attendue {
                ecarts.push(format!(
                    "ligne {numero} : écart au rendu attendu (lu « {} », attendu « {} »)",
                    lue.unwrap_or("<fin>"),
                    attendue.unwrap_or("<fin>")
                ));
                signales += 1;
                if signales == 5 {
                    ecarts.push("…".into());
                    break;
                }
            }
        }
        if signales == 0 {
            ecarts.push("écart au rendu attendu (fins de ligne)".into());
        }
    }
    ecarts
}

/// État de la racine `tc` d'une interface, lu de `tc qdisc show dev <i> root`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RacineTc {
    /// File par défaut du noyau (poignée `0:`) ou aucune : remplaçable.
    Defaut,
    /// Racine de la Douane (`htb 4d44:`).
    Douane,
    /// Racine explicite d'un autre : jamais touchée.
    Etrangere(String),
}

/// Classe la sortie de `tc qdisc show dev <i> root` (pur).
pub fn classer_racine(sortie: &str) -> RacineTc {
    let Some(l) = sortie.lines().map(str::trim).find(|l| l.starts_with("qdisc ")) else {
        return RacineTc::Defaut;
    };
    let mots: Vec<&str> = l.split_whitespace().collect();
    let (genre, poignee) = (mots.get(1).copied().unwrap_or(""), mots.get(2).copied().unwrap_or(""));
    if poignee == "0:" {
        RacineTc::Defaut
    } else if genre == "htb" && poignee == format!("{POIGNEE_RACINE:x}:") {
        RacineTc::Douane
    } else {
        RacineTc::Etrangere(format!("{genre} {poignee}"))
    }
}
