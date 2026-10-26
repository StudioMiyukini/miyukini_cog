//! @id     miyudouane_hote
//! @do     generer_table_hote_zone_cog
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — table hôte « miyudouane_hote » : zone COG sans sortie pour les services COG lancés sur l'hôte (comptes de service, ports d'écoute, exceptions par arrêté signé).

//! # Zone COG sur l'hôte (`table inet miyudouane_hote`)
//!
//! En déploiement réel, les services COG tournent **sur l'hôte** (comptes de
//! service dédiés) et non dans les espaces de noms de [`crate::zones`]. La
//! règle « zone COG sans sortie » (INV-Z1, INV-Z4, EX-12) y est portée par une
//! table nftables **propre à la Douane**, générée ici depuis une description
//! déclarative bornée et validée ([`DescriptionHote`]).
//!
//! ## Règles produites
//!
//! - chaîne `sortie` (crochet `output`, `policy accept` : la table ne gouverne
//!   que les comptes COG, jamais le reste de l'hôte ; **aucune** règle n'y
//!   accepte `oifname "lo"` sans qualification de compte) :
//!   1. `meta skuid != @comptes_cog oifname "lo" accept` : la boucle locale
//!      des **autres** comptes de l'hôte, inchangée ;
//!   2. `meta skuid @comptes_cog oifname "lo" ct state established,related
//!      accept` : suite des connexions déjà admises, et surtout **réponses**
//!      des services COG à leurs clients locaux (connexion entrante admise par
//!      la chaîne `entree`). Une connexion n'est établie que si son premier
//!      paquet a été admis : un processus COG ne peut pas s'en servir pour
//!      ouvrir lui-même un flux vers un relais en écoute locale ;
//!   3. flux de boucle locale **par compte** (directive `boucle`) :
//!      `meta skuid <compte> oifname "lo" ip daddr 127.0.0.1 tcp dport <port>
//!      accept` (ou `ip6 daddr ::1`, ou `udp`) ;
//!   4. exceptions : une règle par flux **COG → portail** d'un arrêté signé
//!      ([`ExceptionSortie`]), `comment "voie=<id> service=<s> flux=<n>"` ;
//!   5. `meta skuid @comptes_cog` → journal `MDOU|zone-cog|sortie|SORTIE-COG|`
//!      (groupe NFLOG de la description, 110 par défaut) + rejet. Tout trafic
//!      `lo` des comptes COG non admis par (2) ou (3) y tombe : 127.0.0.53:53
//!      (systemd-resolved), autre adresse 127/8, adresse de l'hôte livrée par
//!      `lo`, port non déclaré, autre compte COG que celui déclaré, ICMP…
//!
//!   Journalisation bornée comme le reste de la Douane (`crate::nft`, section
//!   « Journalisation bornée ») : une ligne par flux et par seconde, 50
//!   lignes/s pour la table, rejet inconditionnel compté par le compteur
//!   nommé `rejets`.
//! - chaîne `entree` (crochet `input`, `policy accept`) : `iifname "lo"
//!   accept`, puis tout paquet vers un port COG déclaré → journal
//!   `MDOU|zone-cog|entree|NON-DECLARE|` + rejet.
//!
//! **Résolution DNS : aucune.** Les services COG n'en ont pas besoin
//! (adresses littérales 127.0.0.1 / ::1, portails par adresse du plan) ; un
//! résolveur est un relais vers l'extérieur. Il n'existe aucune directive
//! d'exception DNS : `boucle … 53` est refusé, et 127.0.0.53 n'est jamais une
//! destination admise (seules 127.0.0.1 et ::1 le sont).
//!
//! IPv4 **et** IPv6 : famille `inet`, sélecteurs indépendants de la famille
//! (`meta skuid`, `oifname`, `th dport`) ; seules les destinations de boucle
//! locale (127.0.0.1 / ::1) et des exceptions sont écrites par famille
//! (`ip daddr` / `ip6 daddr`).
//!
//! ## Chargement atomique, sans toucher au reste
//!
//! Le texte commence par `add table` + `delete table` puis redéfinit la table :
//! un seul `nft -f` = une seule transaction ; la table est **remplacée** (règles
//! idempotentes : recharger donne le même état), aucune autre table n'est lue
//! ni modifiée, jamais de `flush ruleset` ([`auditer_hote`] le vérifie).
//!
//! ## Motifs de journal (compatibilité Vigies, A-004 / A-005)
//!
//! Le lecteur strict des Vigies (`miyuvigie::nftables`) n'accepte, pour le
//! lieu `zone-cog`, que `SORTIE-COG` dans la chaîne `sortie` et `NON-DECLARE`
//! dans `entree` / `transit` ; le lieu doit correspondre au groupe 110. D'où :
//! sortie refusée = `SORTIE-COG` (même ligne que la chaîne `sortie` de
//! `table inet miyudouane_zone`), entrée refusée = `NON-DECLARE`. Un autre
//! groupe NFLOG reste possible (`groupe` de la description) mais les Vigies
//! tiendront alors ces lignes pour incohérentes.
//!
//! ## Format de la description (version 2)
//!
//! Texte ASCII, une directive par ligne, ≤ [`TAILLE_MAX_DESCRIPTION`] octets,
//! lignes ≤ [`LONGUEUR_MAX_LIGNE`] ; `#` en début de ligne = commentaire ;
//! lignes vides ignorées ; mots séparés par des espaces.
//!
//! Version 2 : la boucle locale n'est plus ouverte d'office aux comptes COG
//! (directive `boucle`). Une description version 1 est **refusée** (et non
//! réinterprétée) : chargée telle quelle, elle couperait sans bruit les flux
//! locaux dont les services ont besoin ; l'installateur doit la réécrire.
//!
//! ```text
//! miyudouane-hote 2                      # en-tête obligatoire, première ligne utile
//! groupe 110                             # groupe NFLOG (1..=65535), facultatif, une fois
//! compte miyukini-maia                   # compte de service par nom ([a-z_][a-z0-9_-]{0,31}, ni « root »)
//! uid 998                                # compte de service par UID (1..=65533)
//! port tcp 50051                         # port d'écoute COG (tcp | udp, 1..=65535)
//! boucle miyukini-maia tcp 127.0.0.1 50051   # flux de boucle locale admis pour CE compte
//! exception site                         # voie d'un arrêté signé portail-interne / portail-local
//! ```
//!
//! Besoins réels de la VM d'essai (`deploy/cog-vm`) : `boucle miyukini-maia
//! tcp 127.0.0.1 50051` (MAIA → KindMother, y compris l'unité d'attente qui
//! tourne sous le compte de MAIA) et `boucle miyukini-miyucloud tcp 127.0.0.1
//! 5432` (MiyuCloud → PostgreSQL). KindMother et le portail web n'ouvrent
//! aucun flux local sortant connu (le portail ne joint CentralRemote que s'il
//! est configuré, ce qui n'est pas le cas) : aucune `boucle` pour eux.
//!
//! - au moins un compte (`compte` ou `uid`) ; au plus [`MAX_COMPTES`] comptes,
//!   [`MAX_PORTS`] ports, [`MAX_BOUCLES`] flux de boucle, [`MAX_EXCEPTIONS`]
//!   exceptions ;
//! - tout doublon (même nom, même UID, même protocole + port, même flux de
//!   boucle, même voie, `groupe` répété) est refusé, comme toute directive
//!   inconnue ou valeur hors bornes ;
//! - `port` : port d'écoute d'un service COG, jamais joignable hors de `lo`
//!   (chaîne `entree`). Ne donne **aucun** droit de sortie : un compte COG qui
//!   doit joindre un port local le déclare par `boucle` ;
//! - `boucle <compte> <tcp|udp> <127.0.0.1|::1> <port>` : seul moyen pour un
//!   compte COG de joindre la boucle locale (hors réponses, règle 2).
//!   `<compte>` = nom ou UID **déclaré** par `compte` / `uid` (ordre des lignes
//!   indifférent) ; adresse littérale 127.0.0.1 ou ::1 uniquement (pas d'autre
//!   adresse 127/8, pas de nom) ; port 1..=65535 sauf 53 (aucune résolution
//!   DNS). Une ligne = une règle : déclarer l'IPv4 et l'IPv6 séparément si
//!   les deux servent. Le service visé ne doit pas relayer vers l'extérieur
//!   (proxy, tunnel, résolveur) : chaque `boucle` est une ouverture explicite ;
//! - un nom est résolu par `nft` au chargement (compte absent ⇒ chargement
//!   refusé, fail-closed) ;
//! - **aucune exception par défaut**. Une exception ne contient ni adresse ni
//!   port : elle **désigne** une voie, et n'ouvre que les flux dont la zone
//!   COG est la source dans un arrêté **signé, vérifié et en vigueur** de
//!   `portail-interne` ou `portail-local` (destination = adresse du portail
//!   côté zone COG, plan réseau). La constitution (INV-Z1, INV-Z2, INV-Z7)
//!   interdit tout flux COG → extérieur : aucune exception ne peut ouvrir
//!   l'Internet. Exception sans flux correspondant ⇒ refus.
//!
//! Le rendu est canonique (comptes, ports, flux de boucle et exceptions
//! triés) : deux descriptions équivalentes donnent le même texte.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Write as _};

use borderguard::arrete::identifiant_valide;
use borderguard::{ArreteVerifie, Frontiere, Protocole, Reseau, Zone};

use crate::compilation::{compiler, Crochet};
use crate::nft::{objets_journal, rejet, selecteurs_adresses, COMPTEUR_REJETS, PREFIXE_JOURNAL};
use crate::plan::PlanReseau;

/// Nom de la table hôte (famille `inet`).
pub const TABLE_HOTE: &str = "miyudouane_hote";
/// En-tête de la description (version 2 : boucle locale déclarée par compte).
pub const ENTETE_DESCRIPTION: &str = "miyudouane-hote 2";
/// En-tête de la version 1 (boucle locale ouverte d'office), refusée.
const ENTETE_V1: &str = "miyudouane-hote 1";
/// Groupe NFLOG par défaut : celui de la zone COG (lieu `zone-cog`, A-004).
pub const GROUPE_NFLOG_DEFAUT: u16 = 110;
/// Lieu des journaux (convention `MDOU|<lieu>|<chaine>|<MOTIF>|`).
pub const LIEU_JOURNAL: &str = "zone-cog";
/// Taille maximale d'une description (octets).
pub const TAILLE_MAX_DESCRIPTION: usize = 16 * 1024;
/// Longueur maximale d'une ligne de description (octets).
pub const LONGUEUR_MAX_LIGNE: usize = 256;
/// Nombre maximal de comptes de service.
pub const MAX_COMPTES: usize = 64;
/// Nombre maximal de ports d'écoute.
pub const MAX_PORTS: usize = 64;
/// Nombre maximal d'exceptions de sortie.
pub const MAX_EXCEPTIONS: usize = 32;
/// Plus petit UID admis (0 = root, refusé : couper la sortie de root n'est pas
/// le rôle de cette table).
pub const UID_MIN: u32 = 1;
/// Plus grand UID admis (65534 = `nobody`, 65535 = « -1 » 16 bits : refusés).
pub const UID_MAX: u32 = 65_533;
/// Adresse IPv4 de boucle locale seule admise en sortie des comptes COG.
pub const BOUCLE_V4: &str = "127.0.0.1";
/// Adresse IPv6 de boucle locale seule admise en sortie des comptes COG.
pub const BOUCLE_V6: &str = "::1";
/// Port DNS : refusé en `boucle` (un résolveur relaie vers l'extérieur).
pub const PORT_DNS: u16 = 53;
/// Nombre maximal de flux de boucle locale.
pub const MAX_BOUCLES: usize = 128;
/// Longueur maximale d'un nom de compte.
pub const LONGUEUR_MAX_NOM: usize = 32;

/// Compte de service COG.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Compte {
    /// Par nom (résolu par `nft` au chargement).
    Nom(String),
    /// Par UID.
    Uid(u32),
}

/// Adresse de boucle locale d'un flux `boucle` (127.0.0.1 ou ::1, seules).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdresseBoucle {
    /// 127.0.0.1.
    V4,
    /// ::1.
    V6,
}

impl AdresseBoucle {
    /// Sélecteur nft de destination (`ip daddr 127.0.0.1` / `ip6 daddr ::1`).
    pub fn selecteur(self) -> String {
        match self {
            Self::V4 => format!("ip daddr {BOUCLE_V4}"),
            Self::V6 => format!("ip6 daddr {BOUCLE_V6}"),
        }
    }
}

/// Flux de boucle locale admis pour un compte COG.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FluxBoucle {
    /// Compte COG émetteur (déclaré par `compte` / `uid`).
    pub compte: Compte,
    /// Destination (127.0.0.1 ou ::1).
    pub adresse: AdresseBoucle,
    /// Protocole.
    pub protocole: Protocole,
    /// Port de destination (1..=65535, jamais 53).
    pub port: u16,
}

/// Description déclarative validée de la zone COG sur l'hôte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptionHote {
    /// Groupe NFLOG des journaux.
    pub groupe: u16,
    /// Comptes de service COG (au moins un).
    pub comptes: BTreeSet<Compte>,
    /// Ports d'écoute COG, joignables par la seule boucle locale.
    pub ports: BTreeSet<(Protocole, u16)>,
    /// Flux de boucle locale admis, par compte COG.
    pub boucles: BTreeSet<FluxBoucle>,
    /// Voies dont les flux COG → portail sont ouverts (par arrêté signé).
    pub exceptions: BTreeSet<String>,
}

/// Description refusée (ligne 1.., 0 = fichier entier).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurDescription {
    /// Numéro de ligne (0 : le fichier).
    pub ligne: usize,
    /// Motif du refus.
    pub motif: String,
}

impl fmt::Display for ErreurDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ligne == 0 {
            write!(f, "description refusée : {}", self.motif)
        } else {
            write!(f, "description refusée, ligne {} : {}", self.ligne, self.motif)
        }
    }
}

impl std::error::Error for ErreurDescription {}

/// Vrai pour un nom de compte admis : `[a-z_][a-z0-9_-]{0,31}`, ni `root`.
pub fn nom_compte_valide(nom: &str) -> bool {
    let mut octets = nom.bytes();
    let Some(premier) = octets.next() else { return false };
    nom.len() <= LONGUEUR_MAX_NOM
        && (premier.is_ascii_lowercase() || premier == b'_')
        && octets.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
        && nom != "root"
}

/// Entier décimal strict (sans signe, sans zéro de tête, ≤ 10 chiffres).
pub(crate) fn entier(s: &str) -> Option<u64> {
    if s.is_empty() || s.len() > 10 || (s.len() > 1 && s.starts_with('0')) || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

fn protocole(p: &str) -> Result<Protocole, String> {
    match p {
        "tcp" => Ok(Protocole::Tcp),
        "udp" => Ok(Protocole::Udp),
        autre => Err(format!("protocole inconnu « {autre} » (tcp | udp)")),
    }
}

fn port(p: &str) -> Result<u16, String> {
    entier(p)
        .and_then(|p| u16::try_from(p).ok())
        .filter(|p| *p != 0)
        .ok_or_else(|| format!("port hors bornes « {p} » (1..=65535)"))
}

fn uid(u: &str) -> Result<u32, String> {
    entier(u)
        .and_then(|u| u32::try_from(u).ok())
        .filter(|u| (UID_MIN..=UID_MAX).contains(u))
        .ok_or_else(|| format!("UID hors bornes « {u} » ({UID_MIN}..={UID_MAX})"))
}

impl DescriptionHote {
    /// Analyse stricte d'une description (voir le format au module).
    pub fn analyser(texte: &str) -> Result<Self, ErreurDescription> {
        let fichier = |motif: &str| ErreurDescription { ligne: 0, motif: motif.into() };
        if texte.len() > TAILLE_MAX_DESCRIPTION {
            return Err(fichier("taille maximale dépassée"));
        }
        let mut entete = false;
        let mut groupe = None;
        let mut comptes_boucles: Vec<(usize, Compte)> = Vec::new();
        let mut d = Self {
            groupe: GROUPE_NFLOG_DEFAUT,
            comptes: BTreeSet::new(),
            ports: BTreeSet::new(),
            boucles: BTreeSet::new(),
            exceptions: BTreeSet::new(),
        };
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
                let lue = mots.join(" ");
                if lue == ENTETE_V1 {
                    return Err(refus(format!(
                        "format 1 périmé : en-tête « {ENTETE_DESCRIPTION} » attendu, la boucle locale des \
                         comptes COG se déclare par « boucle »"
                    )));
                }
                if lue != ENTETE_DESCRIPTION {
                    return Err(refus(format!("en-tête « {ENTETE_DESCRIPTION} » attendu")));
                }
                entete = true;
                continue;
            }
            match mots.as_slice() {
                ["groupe", g] => {
                    if groupe.is_some() {
                        return Err(refus("« groupe » répété".into()));
                    }
                    let g = entier(g)
                        .and_then(|g| u16::try_from(g).ok())
                        .filter(|g| *g != 0)
                        .ok_or_else(|| refus(format!("groupe NFLOG hors bornes « {g} » (1..=65535)")))?;
                    groupe = Some(g);
                }
                ["compte", nom] => {
                    if !nom_compte_valide(nom) {
                        return Err(refus(format!("nom de compte invalide « {nom} »")));
                    }
                    if !d.comptes.insert(Compte::Nom((*nom).to_owned())) {
                        return Err(refus(format!("compte en double « {nom} »")));
                    }
                }
                ["uid", u] => {
                    let uid = uid(u).map_err(refus)?;
                    if !d.comptes.insert(Compte::Uid(uid)) {
                        return Err(refus(format!("UID en double « {uid} »")));
                    }
                }
                ["port", proto, p] => {
                    let proto = protocole(proto).map_err(refus)?;
                    let port = port(p).map_err(refus)?;
                    if !d.ports.insert((proto, port)) {
                        return Err(refus(format!("port en double « {proto} {port} »")));
                    }
                }
                ["boucle", compte, proto, adresse, p] => {
                    let compte = if compte.bytes().next().is_some_and(|b| b.is_ascii_digit()) {
                        Compte::Uid(uid(compte).map_err(refus)?)
                    } else if nom_compte_valide(compte) {
                        Compte::Nom((*compte).to_owned())
                    } else {
                        return Err(refus(format!("nom de compte invalide « {compte} »")));
                    };
                    let protocole = protocole(proto).map_err(refus)?;
                    let adresse = match *adresse {
                        BOUCLE_V4 => AdresseBoucle::V4,
                        BOUCLE_V6 => AdresseBoucle::V6,
                        autre => {
                            return Err(refus(format!(
                                "adresse de boucle « {autre} » refusée ({BOUCLE_V4} | {BOUCLE_V6} seulement)"
                            )))
                        }
                    };
                    let port = port(p).map_err(refus)?;
                    if port == PORT_DNS {
                        return Err(refus(format!("port {PORT_DNS} refusé : aucune résolution DNS pour les comptes COG")));
                    }
                    let flux = FluxBoucle { compte: compte.clone(), adresse, protocole, port };
                    if !d.boucles.insert(flux) {
                        return Err(refus(format!("flux de boucle en double « {} »", mots[1..].join(" "))));
                    }
                    comptes_boucles.push((n, compte));
                }
                ["exception", voie] => {
                    if !identifiant_valide(voie) {
                        return Err(refus(format!("identifiant de voie invalide « {voie} »")));
                    }
                    if !d.exceptions.insert((*voie).to_owned()) {
                        return Err(refus(format!("exception en double « {voie} »")));
                    }
                }
                [mot, ..] => return Err(refus(format!("directive inconnue ou mal formée « {mot} »"))),
                [] => {}
            }
            if d.comptes.len() > MAX_COMPTES
                || d.ports.len() > MAX_PORTS
                || d.boucles.len() > MAX_BOUCLES
                || d.exceptions.len() > MAX_EXCEPTIONS
            {
                return Err(refus("nombre maximal d'entrées dépassé".into()));
            }
        }
        if !entete {
            return Err(fichier("en-tête absent"));
        }
        if d.comptes.is_empty() {
            return Err(fichier("au moins un compte de service attendu"));
        }
        if let Some((n, c)) = comptes_boucles.iter().find(|(_, c)| !d.comptes.contains(c)) {
            return Err(ErreurDescription {
                ligne: *n,
                motif: format!("boucle : compte « {} » non déclaré (compte / uid)", element_compte(c)),
            });
        }
        if let Some(g) = groupe {
            d.groupe = g;
        }
        Ok(d)
    }
}

/// Exception de sortie : un flux COG → portail d'un arrêté vérifié.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionSortie {
    /// Frontière de l'arrêté (portail interne ou local).
    pub frontiere: Frontiere,
    /// Numéro de l'arrêté.
    pub numero: u64,
    /// Empreinte de l'arrêté (hex).
    pub empreinte: String,
    /// Voie.
    pub voie: String,
    /// Service propriétaire.
    pub service: String,
    /// Rang du flux dans l'arrêté.
    pub index_flux: usize,
    /// Destinations (adresse du portail côté zone COG).
    pub destinations: Vec<Reseau>,
    /// Protocole.
    pub protocole: Protocole,
    /// Port de destination.
    pub port: u16,
}

/// Exceptions de sortie déclarées par la description et **autorisées** par les
/// arrêtés fournis (déjà vérifiés : signature, ancre, constitution). Arrêté
/// hors vigueur, frontière autre qu'un portail, frontière donnée deux fois,
/// voie ambiguë (deux arrêtés) ou exception sans flux COG ⇒ refus. Les voies
/// de `coupees` sont omises (coupure de voie, EX-49).
pub fn exceptions_autorisees(
    description: &DescriptionHote,
    arretes: &[ArreteVerifie],
    plan: &PlanReseau,
    coupees: &BTreeSet<String>,
    maintenant: u64,
) -> Result<Vec<ExceptionSortie>, String> {
    let mut vues = BTreeSet::new();
    let mut origine_voie: BTreeMap<String, Frontiere> = BTreeMap::new();
    let mut sortie = Vec::new();
    for v in arretes {
        let a = v.arrete();
        if !matches!(a.frontiere, Frontiere::PortailInterne | Frontiere::PortailLocal) {
            return Err(format!("arrêté {} : seule la zone COG ouvre, par un portail (INV-Z1)", a.frontiere));
        }
        if !vues.insert(a.frontiere) {
            return Err(format!("frontière {} donnée deux fois", a.frontiere));
        }
        if !v.en_vigueur(maintenant) {
            return Err(format!("arrêté {} n°{} hors vigueur", a.frontiere, a.numero));
        }
        for voie in a.voies.iter().filter(|x| description.exceptions.contains(&x.id)) {
            if let Some(f) = origine_voie.insert(voie.id.clone(), a.frontiere) {
                return Err(format!("exception « {} » ambiguë : arrêtés {f} et {}", voie.id, a.frontiere));
            }
        }
        let regles = compiler(v, plan).map_err(|e| e.to_string())?;
        for r in regles {
            let retenue = r.crochet == Crochet::Entree
                && r.zone_source == Zone::Cog
                && description.exceptions.contains(&r.voie)
                && !coupees.contains(&r.voie);
            if retenue {
                sortie.push(ExceptionSortie {
                    frontiere: a.frontiere,
                    numero: a.numero,
                    empreinte: v.empreinte_hex(),
                    voie: r.voie,
                    service: r.service,
                    index_flux: r.index_flux,
                    destinations: r.destinations,
                    protocole: r.protocole,
                    port: r.port,
                });
            }
        }
    }
    for e in &description.exceptions {
        let ouvre = arretes.iter().any(|v| v.arrete().flux.iter().any(|f| f.voie == *e && f.source == Zone::Cog));
        if !ouvre {
            return Err(format!("exception « {e} » : aucun flux zone COG → portail dans les arrêtés fournis"));
        }
    }
    sortie.sort_by(|x, y| (x.frontiere.nom(), &x.voie, x.index_flux).cmp(&(y.frontiere.nom(), &y.voie, y.index_flux)));
    Ok(sortie)
}

fn element_compte(c: &Compte) -> String {
    match c {
        // Nom validé ([a-z0-9_-]) : aucun guillemet, aucune accolade possible.
        Compte::Nom(n) => format!("\"{n}\""),
        Compte::Uid(u) => u.to_string(),
    }
}

/// Ports d'un protocole, triés.
fn liste_ports(ports: &BTreeSet<(Protocole, u16)>, proto: Protocole) -> Vec<u16> {
    ports.iter().filter(|(p, _)| *p == proto).map(|(_, n)| *n).collect()
}

fn ensemble(ports: &[u16]) -> String {
    ports.iter().map(u16::to_string).collect::<Vec<_>>().join(", ")
}

/// Boucle locale des autres comptes de l'hôte (chaîne `sortie`).
const BOUCLE_AUTRES_COMPTES: &str = "meta skuid != @comptes_cog oifname \"lo\" accept";
/// Suite des connexions admises / réponses des comptes COG en boucle locale.
const BOUCLE_ETABLIES_COG: &str = "meta skuid @comptes_cog oifname \"lo\" ct state established,related accept";

/// Règle d'un flux de boucle locale admis.
fn regle_boucle(f: &FluxBoucle) -> String {
    format!(
        "meta skuid {} oifname \"lo\" {} {} dport {} accept",
        element_compte(&f.compte),
        f.adresse.selecteur(),
        f.protocole,
        f.port
    )
}

/// Génère le texte `nft -f` de la table hôte (pur, déterministe).
pub fn generer_hote(d: &DescriptionHote, exceptions: &[ExceptionSortie]) -> String {
    let g = d.groupe;
    let mut t = String::new();
    let _ = writeln!(t, "#!/usr/sbin/nft -f");
    let _ = writeln!(t, "# Généré par miyudouane hote — ne pas modifier à la main.");
    let _ = writeln!(t, "# Douane — zone COG sur l'hôte : comptes COG sans sortie, hors flux de boucle");
    let _ = writeln!(t, "# déclarés par compte (127.0.0.1 / ::1) ; ports COG joignables par la seule");
    let _ = writeln!(t, "# boucle locale (IPv4 et IPv6). Aucune résolution DNS.");
    let _ = writeln!(t, "# Table propre : remplacée seule (add + delete + table, une transaction) ;");
    let _ = writeln!(t, "# aucune autre table touchée.");
    let _ = writeln!(t, "# journal : nflog groupe {g}, prefixe \"{PREFIXE_JOURNAL}|{LIEU_JOURNAL}|<chaine>|<MOTIF>|\"");
    let mut arretes: Vec<String> = exceptions
        .iter()
        .map(|e| format!("arrete {} n°{} empreinte={}", e.frontiere, e.numero, e.empreinte))
        .collect();
    arretes.dedup();
    for a in &arretes {
        let _ = writeln!(t, "# {a}");
    }
    let _ = writeln!(t, "add table inet {TABLE_HOTE}");
    let _ = writeln!(t, "delete table inet {TABLE_HOTE}");
    let _ = writeln!(t);
    let _ = writeln!(t, "table inet {TABLE_HOTE} {{");
    objets_journal(&mut t);
    let comptes: Vec<String> = d.comptes.iter().map(element_compte).collect();
    let _ = writeln!(t, "\tset comptes_cog {{");
    let _ = writeln!(t, "\t\ttype uid");
    let _ = writeln!(t, "\t\telements = {{ {} }}", comptes.join(", "));
    let _ = writeln!(t, "\t}}");
    let _ = writeln!(t);

    let _ = writeln!(t, "\tchain sortie {{");
    let _ = writeln!(t, "\t\ttype filter hook output priority filter; policy accept;");
    let _ = writeln!(t, "\t\t{BOUCLE_AUTRES_COMPTES}");
    let _ = writeln!(t, "\t\t{BOUCLE_ETABLIES_COG}");
    for f in &d.boucles {
        let _ = writeln!(t, "\t\t{}", regle_boucle(f));
    }
    for e in exceptions {
        for sel in selecteurs_adresses(&[], &e.destinations) {
            let _ = writeln!(
                t,
                "\t\tmeta skuid @comptes_cog {sel} {} dport {} counter accept comment \"voie={} service={} flux={}\"",
                e.protocole, e.port, e.voie, e.service, e.index_flux
            );
        }
    }
    for r in rejet("meta skuid @comptes_cog", LIEU_JOURNAL, "sortie", "SORTIE-COG", g) {
        let _ = writeln!(t, "\t\t{r}");
    }
    let _ = writeln!(t, "\t}}");

    if !d.ports.is_empty() {
        let _ = writeln!(t);
        let _ = writeln!(t, "\tchain entree {{");
        let _ = writeln!(t, "\t\ttype filter hook input priority filter; policy accept;");
        let _ = writeln!(t, "\t\tiifname \"lo\" accept");
        for proto in [Protocole::Tcp, Protocole::Udp] {
            let ports = liste_ports(&d.ports, proto);
            if !ports.is_empty() {
                let sel = format!("meta l4proto {proto} th dport {{ {} }}", ensemble(&ports));
                for r in rejet(&sel, LIEU_JOURNAL, "entree", "NON-DECLARE", g) {
                    let _ = writeln!(t, "\t\t{r}");
                }
            }
        }
        let _ = writeln!(t, "\t}}");
    }
    let _ = writeln!(t, "}}");
    t
}

/// Vrai pour une règle de flux de boucle locale telle que produite par
/// [`regle_boucle`] : compte de `comptes` (éléments du set `comptes_cog`),
/// 127.0.0.1 ou ::1, tcp | udp, un port 1..=65535 autre que 53.
fn boucle_valide(l: &str, comptes: &BTreeSet<String>) -> bool {
    let Some(reste) = l.strip_prefix("meta skuid ") else { return false };
    let Some((compte, reste)) = reste.split_once(" oifname \"lo\" ") else { return false };
    if !comptes.contains(compte) {
        return false;
    }
    let v4 = AdresseBoucle::V4.selecteur() + " ";
    let v6 = AdresseBoucle::V6.selecteur() + " ";
    let Some(reste) = reste.strip_prefix(v4.as_str()).or_else(|| reste.strip_prefix(v6.as_str())) else {
        return false;
    };
    let Some(reste) = reste.strip_prefix("tcp dport ").or_else(|| reste.strip_prefix("udp dport ")) else {
        return false;
    };
    reste.strip_suffix(" accept").and_then(|p| port(p).ok()).is_some_and(|p| p != PORT_DNS)
}

/// Audit structurel du texte de la table hôte (défense en profondeur,
/// indépendant du générateur). Renvoie les écarts (vide = conforme) :
///
/// - aucun `flush`, aucun `jump` / `goto`, aucune autre table que
///   `inet miyudouane_hote` ; hors de la table, seuls `add table` et
///   `delete table` de cette même table ;
/// - chaînes de base en `policy accept` (la table ne gouverne pas l'hôte) ;
/// - chaque `accept` est :
///   - en `entree`, `iifname "lo" accept` ;
///   - en `sortie`, la boucle locale des **autres** comptes
///     (`meta skuid != @comptes_cog oifname "lo" accept`), les connexions
///     établies des comptes COG en boucle locale, ou un flux `boucle` d'un
///     compte du set `comptes_cog` vers 127.0.0.1 / ::1 (port ≠ 53) — un
///     `oifname "lo" accept` non qualifié, ou toute autre forme sur `lo`, est
///     un écart ;
///   - ou une exception `meta skuid @comptes_cog … daddr … dport … comment
///     "voie=<id> …"` (sans `oifname` / `iifname`) avec `<id>` dans
///     `voies_exceptions` ;
/// - la chaîne `sortie` existe, journalise les comptes COG (`SORTIE-COG`) et
///   se termine par leur rejet inconditionnel (`counter name "rejets" drop`).
pub fn auditer_hote(texte: &str, voies_exceptions: &BTreeSet<String>) -> Vec<String> {
    let mut ecarts = Vec::new();
    let ouverture = format!("table inet {TABLE_HOTE} {{");
    let admises_hors_table = [format!("add table inet {TABLE_HOTE}"), format!("delete table inet {TABLE_HOTE}")];
    let mut profondeur: i64 = 0;
    let mut chaine: Option<String> = None;
    let mut derniere = String::new();
    let mut sortie_conforme = false;
    let mut journal_sortie = false;
    let mut set_courant: Option<String> = None;
    let mut comptes_cog: BTreeSet<String> = BTreeSet::new();
    for (n, brute) in texte.lines().enumerate() {
        let n = n + 1;
        let l = brute.trim();
        if l.is_empty() || (profondeur == 0 && l.starts_with('#')) {
            continue;
        }
        let mots: Vec<&str> = l.split_whitespace().collect();
        if mots.iter().any(|m| matches!(*m, "flush" | "jump" | "goto")) {
            ecarts.push(format!("ligne {n} : instruction interdite"));
        }
        if profondeur == 0 {
            if l == ouverture {
                profondeur = 1;
            } else if !admises_hors_table.iter().any(|a| a == l) {
                ecarts.push(format!("ligne {n} : instruction hors de la table {TABLE_HOTE}"));
            }
            continue;
        }
        if profondeur == 1 {
            if let Some(nom) = l.strip_prefix("chain ").and_then(|r| r.strip_suffix(" {")) {
                chaine = Some(nom.to_owned());
                derniere.clear();
            }
            set_courant = l.strip_prefix("set ").and_then(|r| r.strip_suffix(" {")).map(str::to_owned);
        }
        if profondeur == 2 && set_courant.as_deref() == Some("comptes_cog") {
            if let Some(el) = l.strip_prefix("elements = { ").and_then(|r| r.strip_suffix(" }")) {
                comptes_cog.extend(el.split(", ").map(str::to_owned));
            }
        }
        let ouvrantes = i64::try_from(l.matches('{').count()).unwrap_or(i64::MAX);
        let fermantes = i64::try_from(l.matches('}').count()).unwrap_or(i64::MAX);
        let delta = ouvrantes - fermantes;
        if l == "}" && profondeur == 2 {
            if chaine.as_deref() == Some("sortie") {
                sortie_conforme = derniere == format!("meta skuid @comptes_cog counter name \"{COMPTEUR_REJETS}\" drop")
                    && journal_sortie;
            }
            chaine = None;
        }
        profondeur += delta;
        if profondeur < 0 {
            ecarts.push(format!("ligne {n} : accolade fermante en trop"));
            profondeur = 0;
        }
        if chaine.is_none() || l.starts_with("chain ") || l == "}" {
            continue;
        }
        if l.starts_with("type filter hook") {
            if !l.ends_with("policy accept;") {
                ecarts.push(format!("ligne {n} : chaîne de l'hôte hors policy accept"));
            }
            continue;
        }
        l.clone_into(&mut derniere);
        if chaine.as_deref() == Some("sortie")
            && l.starts_with("meta skuid @comptes_cog update @journal_v")
            && l.contains(&format!("log prefix \"{PREFIXE_JOURNAL}|{LIEU_JOURNAL}|sortie|SORTIE-COG|\""))
        {
            journal_sortie = true;
        }
        if !mots.contains(&"accept") {
            continue;
        }
        match chaine.as_deref() {
            Some("entree") if l == "iifname \"lo\" accept" => continue,
            Some("sortie")
                if l == BOUCLE_AUTRES_COMPTES || l == BOUCLE_ETABLIES_COG || boucle_valide(l, &comptes_cog) =>
            {
                continue
            }
            _ => {}
        }
        if mots.contains(&"oifname") || mots.contains(&"iifname") {
            ecarts.push(format!("ligne {n} : accept sur interface hors boucle locale restreinte des comptes COG"));
            continue;
        }
        let voie = l.split("comment \"voie=").nth(1).and_then(|s| s.split(' ').next());
        let forme_exception = chaine.as_deref() == Some("sortie")
            && l.starts_with("meta skuid @comptes_cog ")
            && l.contains(" daddr ")
            && l.contains(" dport ");
        match voie {
            Some(v) if voies_exceptions.contains(v) && forme_exception => {}
            Some(v) => ecarts.push(format!("ligne {n} : accept sur exception non déclarée « {v} »")),
            None => ecarts.push(format!("ligne {n} : accept sans exception déclarée")),
        }
    }
    if profondeur != 0 {
        ecarts.push("table non fermée".into());
    }
    if !sortie_conforme {
        ecarts.push("chaîne sortie absente ou non terminée par le rejet journalisé des comptes COG".into());
    }
    ecarts
}
