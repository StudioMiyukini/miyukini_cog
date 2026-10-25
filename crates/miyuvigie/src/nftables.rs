//! @id     miyuvigie_nftables
//! @do     analyser_journaux_nftables
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « nftables » : lecture stricte des journaux NFLOG de la Douane (convention MDOU, A-004).

//! Lecteur strict des journaux du pare-feu générés par la Douane
//! (`miyudouane::nft`) et relevés sur ses groupes NFLOG.
//!
//! # Convention canonique (amendement A-004)
//!
//! Chaque règle de rejet journalisée de la Douane porte :
//!
//! ```text
//! log prefix "MDOU|<lieu>|<chaine>|<MOTIF>|" group <G>
//! ```
//!
//! | élément | valeurs |
//! |---|---|
//! | `<lieu>` | `externe` (G = 101), `portail-interne` (102), `portail-local` (103), `zone-cog` (110), `zone-franche` (111), `zone-locale` (112) |
//! | `<chaine>` | `entree` (crochet `input`), `transit` (`forward`), `sortie` (`output`) |
//! | `<MOTIF>` | `NON-DECLARE`, `SORTIE-COG`, `ENTREE-COG`, `INVALIDE`, `DEBIT`, `TAILLE`, `LATERAL`, `FERME` |
//!
//! Toutes ces règles **rejettent** (`… drop`) : le verdict d'un événement lu
//! est toujours [`Verdict::Bloque`].
//!
//! # Format de ligne attendu
//!
//! Ligne textuelle rendue par le relais NFLOG (`miyurelais`, un abonné par
//! groupe ; format détaillé et origine de chaque jeton : `miyurelais::ligne`),
//! au format des journaux netfilter du noyau précédé de l'horodatage, de
//! l'hôte et du groupe :
//!
//! ```text
//! 1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv0 OUT=z-pi SRC=10.77.0.10 DST=93.184.216.34 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=443 SYN
//! ```
//!
//! Analyse **stricte** : un seul espace entre jetons, émetteur `nflog-<G>:`,
//! préfixe complet en 4e position, clés `[A-Z0-9]+`, clé en double refusée,
//! champs `IN`, `OUT`, `SRC`, `DST`, `LEN`, `PROTO` obligatoires (`IN` / `OUT`
//! peuvent être vides selon le crochet), `DPT` obligatoire pour TCP / UDP.
//! Exception : TCP / UDP sans en-tête de transport lisible (fragment non
//! initial, troncature) est écrit `PROTO=6` / `PROTO=17` suivi du drapeau
//! `FRAG` ou `TRONQUE`, sans `SPT` / `DPT` ; la ligne est acceptée (protocole
//! TCP / UDP, port inconnu). `FRAG` / `TRONQUE` hors de ce cas, ou un numéro
//! 6 / 17 sans eux, est mal formé.
//!
//! # Recoupements (ce que la Douane peut réellement produire)
//!
//! Une ligne bien formée mais impossible au regard des règles générées est
//! **incohérente** :
//!
//! - le groupe NFLOG correspond au lieu (101..103, 110..112) ;
//! - le motif est produit à ce lieu et dans cette chaîne (ex. `LATERAL` :
//!   pont de zone, chaîne `transit` seulement ; `TAILLE` / `DEBIT` / `FERME` :
//!   frontières seulement) ;
//! - les interfaces suivent le crochet (`entree` : `OUT` vide ; `sortie` : `IN`
//!   vide ; `transit` : les deux renseignées ; `LATERAL` : `sv*` → `sv*`) ;
//! - les adresses suivent l'ordre des règles : `SORTIE-COG` (règle
//!   `ip saddr <cog>`) ⇒ source en zone COG ; `ENTREE-COG` (après la
//!   précédente) ⇒ source hors zone COG et destination en zone COG ;
//!   `NON-DECLARE` d'une frontière (après les deux) ⇒ ni source ni
//!   destination en zone COG ; aucune adresse de boucle locale (le bouclage
//!   est accepté avant toute journalisation).
//!
//! Exception : une ligne du lieu `zone-cog` en chaîne `sortie` (crochet
//! `output` de `inet miyudouane_zone` ou de `inet miyudouane_hote`, comptes de
//! service COG de l'hôte) est émise **par** la zone COG : sa source est en
//! zone COG quelle que soit son adresse (sur l'hôte, une adresse de l'hôte) ⇒
//! `R-EX12-SORTIE` critique (EX-12).
//!
//! Zones : déduites des adresses par le [`PlanAdressage`]. Voie : attribuée
//! seulement si l'interface d'**entrée** (`IN=`) d'un **pont de zone** est le
//! port de la sandbox d'un service déclaré au lecteur
//! ([`LecteurNftables::declarer_interface_voie`]) : la voie est celle de la
//! sandbox qui a **émis** le trafic. L'interface de sortie (`OUT=`) n'attribue
//! jamais de voie : un flux reçu par une sandbox (adresse source usurpée ou
//! non) ne doit pas faire couper sa voie (anti-usurpation, A-005). Une
//! frontière est partagée par toutes les voies, ses lignes restent sans voie.
//!
//! Le motif du préfixe est porté par l'événement ([`EvenementFlux::motif`]) :
//! les règles intégrées A-005 (`LATERAL`, `NON-DECLARE`) le lisent.
//!
//! Une ligne mal formée ou incohérente est ignorée et **comptée** ; elle n'est
//! jamais une preuve bénigne et ne vaut pas preuve de vie du capteur.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::net::IpAddr;

use crate::evenement::{EvenementFlux, Protocole, Verdict};
use crate::signal::{EvenementId, VoieId};
use crate::zone::{est_boucle_locale, PlanAdressage, Zone};

/// Marqueur de début du préfixe des journaux Douane (`MDOU|`).
pub const MARQUEUR_PREFIXE: &str = "MDOU|";

/// Longueur maximale d'une ligne acceptée (octets).
pub const LONGUEUR_MAX_LIGNE: usize = 4096;

/// Lieu de journalisation : une frontière (instance Douane) ou un pont de zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Lieu {
    /// Frontière externe (groupe 101).
    Externe,
    /// Portail interne (groupe 102).
    PortailInterne,
    /// Portail local (groupe 103).
    PortailLocal,
    /// Pont de la zone COG (groupe 110).
    ZoneCog,
    /// Pont de la zone Franche (groupe 111).
    ZoneFranche,
    /// Pont de la zone Franche locale (groupe 112).
    ZoneLocale,
}

impl Lieu {
    /// Tous les lieux.
    pub const TOUS: [Self; 6] = [
        Self::Externe,
        Self::PortailInterne,
        Self::PortailLocal,
        Self::ZoneCog,
        Self::ZoneFranche,
        Self::ZoneLocale,
    ];

    /// Nom dans le préfixe.
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Externe => "externe",
            Self::PortailInterne => "portail-interne",
            Self::PortailLocal => "portail-local",
            Self::ZoneCog => "zone-cog",
            Self::ZoneFranche => "zone-franche",
            Self::ZoneLocale => "zone-locale",
        }
    }

    /// Groupe NFLOG du lieu (plan de référence de la Douane).
    pub const fn groupe_nflog(self) -> u16 {
        match self {
            Self::Externe => 101,
            Self::PortailInterne => 102,
            Self::PortailLocal => 103,
            Self::ZoneCog => 110,
            Self::ZoneFranche => 111,
            Self::ZoneLocale => 112,
        }
    }

    /// Vrai pour un pont de zone (sinon : une frontière).
    pub const fn est_zone(self) -> bool {
        matches!(self, Self::ZoneCog | Self::ZoneFranche | Self::ZoneLocale)
    }

    /// Analyse stricte.
    pub fn depuis_nom(s: &str) -> Option<Self> {
        Self::TOUS.into_iter().find(|l| l.nom() == s)
    }
}

impl fmt::Display for Lieu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.nom())
    }
}

/// Chaîne nftables (crochet netfilter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chaine {
    /// `entree` (crochet `input`).
    Entree,
    /// `transit` (crochet `forward`).
    Transit,
    /// `sortie` (crochet `output`).
    Sortie,
}

impl Chaine {
    /// Nom dans le préfixe.
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Entree => "entree",
            Self::Transit => "transit",
            Self::Sortie => "sortie",
        }
    }

    /// Analyse stricte.
    pub fn depuis_nom(s: &str) -> Option<Self> {
        match s {
            "entree" => Some(Self::Entree),
            "transit" => Some(Self::Transit),
            "sortie" => Some(Self::Sortie),
            _ => None,
        }
    }
}

/// Motif de rejet (dernier champ du préfixe).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MotifJournal {
    /// Connexion nouvelle hors arrêté.
    NonDeclare,
    /// Source en zone COG (INV-Z1, EX-12).
    SortieCog,
    /// Destination en zone COG (INV-Z2).
    EntreeCog,
    /// `ct state invalid`.
    Invalide,
    /// Débit du flux au-delà de l'arrêté.
    Debit,
    /// Paquet au-delà de la taille de l'arrêté.
    Taille,
    /// Sandbox → sandbox dans un pont de zone (INV-Z8).
    Lateral,
    /// Frontière fermée (aucun arrêté en vigueur).
    Ferme,
}

impl MotifJournal {
    /// Nom dans le préfixe.
    pub const fn nom(self) -> &'static str {
        match self {
            Self::NonDeclare => "NON-DECLARE",
            Self::SortieCog => "SORTIE-COG",
            Self::EntreeCog => "ENTREE-COG",
            Self::Invalide => "INVALIDE",
            Self::Debit => "DEBIT",
            Self::Taille => "TAILLE",
            Self::Lateral => "LATERAL",
            Self::Ferme => "FERME",
        }
    }

    /// Analyse stricte.
    pub fn depuis_nom(s: &str) -> Option<Self> {
        [
            Self::NonDeclare,
            Self::SortieCog,
            Self::EntreeCog,
            Self::Invalide,
            Self::Debit,
            Self::Taille,
            Self::Lateral,
            Self::Ferme,
        ]
        .into_iter()
        .find(|m| m.nom() == s)
    }

    /// Vrai si la Douane produit ce motif à ce lieu, dans cette chaîne.
    pub fn produit_par_la_douane(self, lieu: Lieu, chaine: Chaine) -> bool {
        use Chaine::{Entree, Sortie, Transit};
        if lieu.est_zone() {
            match (self, chaine) {
                (Self::Invalide | Self::Lateral, Transit)
                | (Self::NonDeclare, Entree | Transit)
                | (Self::SortieCog, Sortie) => true,
                (Self::SortieCog, Transit) => lieu == Lieu::ZoneCog,
                _ => false,
            }
        } else {
            !matches!(self, Self::Lateral)
        }
    }
}

impl fmt::Display for MotifJournal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.nom())
    }
}

/// Préfixe analysé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixeDouane {
    /// Lieu.
    pub lieu: Lieu,
    /// Chaîne.
    pub chaine: Chaine,
    /// Motif.
    pub motif: MotifJournal,
}

/// Résultat de l'analyse d'une ligne.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultatLigne {
    /// Ligne valide : événement de flux.
    Evenement(EvenementFlux),
    /// Ligne sans le marqueur `MDOU|` (pas un journal de la Douane).
    HorsConvention,
    /// Ligne portant le marqueur mais mal formée.
    MalFormee(&'static str),
    /// Ligne bien formée mais impossible au regard des règles de la Douane.
    Incoherente(&'static str),
}

/// Lecteur des journaux NFLOG de la Douane.
#[derive(Debug, Clone)]
pub struct LecteurNftables {
    plan: PlanAdressage,
    interfaces_voies: BTreeMap<String, VoieId>,
}

impl LecteurNftables {
    /// Crée un lecteur avec le plan d'adressage servant au recoupement des zones.
    pub fn new(plan: PlanAdressage) -> Self {
        Self {
            plan,
            interfaces_voies: BTreeMap::new(),
        }
    }

    /// Déclare le port de pont de zone (`sv<n>`) de la sandbox d'un service et
    /// sa voie (plan réseau de la Douane : `Sandbox::interface_zone`).
    pub fn declarer_interface_voie(&mut self, interface: impl Into<String>, voie: VoieId) {
        self.interfaces_voies.insert(interface.into(), voie);
    }

    /// Analyse une ligne.
    pub fn analyser(&self, ligne: &str) -> ResultatLigne {
        if !ligne.contains(MARQUEUR_PREFIXE) {
            return ResultatLigne::HorsConvention;
        }
        match self.analyser_stricte(ligne) {
            Ok(evt) => ResultatLigne::Evenement(evt),
            Err(Rejet::MalFormee(m)) => ResultatLigne::MalFormee(m),
            Err(Rejet::Incoherente(m)) => ResultatLigne::Incoherente(m),
        }
    }

    fn analyser_stricte(&self, ligne: &str) -> Result<EvenementFlux, Rejet> {
        use Rejet::MalFormee as M;
        if ligne.len() > LONGUEUR_MAX_LIGNE {
            return Err(M("ligne trop longue"));
        }
        let ligne = ligne.strip_suffix('\n').unwrap_or(ligne);
        if !ligne.is_ascii() || ligne.bytes().any(|b| b.is_ascii_control()) {
            return Err(M("caractère non ASCII ou de contrôle"));
        }
        let jetons: Vec<&str> = ligne.split(' ').collect();
        if jetons.iter().any(|j| j.is_empty()) {
            return Err(M("espacement irrégulier"));
        }
        if jetons.len() < 5 {
            return Err(M("ligne tronquée"));
        }
        let horodatage_ms = analyser_horodatage(jetons[0]).ok_or(M("horodatage invalide"))?;
        if !jetons[1]
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_'))
        {
            return Err(M("nom d'hôte invalide"));
        }
        let groupe = analyser_emetteur(jetons[2]).ok_or(M("émetteur différent de « nflog-<G>: »"))?;
        let prefixe = analyser_prefixe(jetons[3])?;

        let champs = analyser_champs(&jetons[4..])?;
        for requis in ["IN", "OUT", "SRC", "DST", "LEN", "PROTO"] {
            if !champs.contains_key(requis) {
                return Err(M("champ obligatoire absent"));
            }
        }
        let ip_src: IpAddr = champs["SRC"].parse().map_err(|_| M("SRC invalide"))?;
        let ip_dst: IpAddr = champs["DST"].parse().map_err(|_| M("DST invalide"))?;
        let octets: u64 = champs["LEN"].parse().map_err(|_| M("LEN invalide"))?;
        // TCP / UDP sans en-tête de transport lisible (fragment non initial,
        // troncature) : le relais écrit le protocole en numéro (`PROTO=6` /
        // `17`) suivi du marqueur `FRAG` ou `TRONQUE`, sans ports.
        let sans_ports = jetons[4..].iter().any(|j| matches!(*j, "FRAG" | "TRONQUE"));
        let protocole = match champs["PROTO"] {
            "6" | "17" if !sans_ports => return Err(M("PROTO numérique TCP/UDP sans FRAG/TRONQUE")),
            "TCP" | "6" => Protocole::Tcp,
            "UDP" | "17" => Protocole::Udp,
            "ICMP" | "ICMPv6" => Protocole::Icmp,
            n => Protocole::Autre(n.parse().map_err(|_| M("PROTO invalide"))?),
        };
        if sans_ports && !matches!(champs["PROTO"], "6" | "17") {
            return Err(M("FRAG/TRONQUE hors TCP/UDP numérique"));
        }
        let port_dst = match champs.get("DPT") {
            Some(_) if sans_ports => return Err(M("DPT avec FRAG/TRONQUE")),
            Some(p) => Some(p.parse::<u16>().map_err(|_| M("DPT invalide"))?),
            None if matches!(protocole, Protocole::Tcp | Protocole::Udp) && !sans_ports => {
                return Err(M("DPT absent pour TCP/UDP"))
            }
            None => None,
        };
        let (iif, oif) = (champs["IN"], champs["OUT"]);
        let (zone_src, zone_dst) = self.recouper(prefixe, groupe, (iif, oif), (ip_src, ip_dst))?;
        let lieu = prefixe.lieu;

        // Côté d'entrée seulement (A-005) : jamais `OUT=`.
        let voie = if lieu.est_zone() {
            self.interfaces_voies.get(iif).cloned()
        } else {
            None
        };

        let id = EvenementId::new(format!("nft-{}-{:016x}", jetons[0], fnv1a(ligne.as_bytes())))
            .ok_or(M("identifiant d'événement"))?;
        Ok(EvenementFlux {
            id,
            horodatage_ms,
            zone_src,
            zone_dst,
            voie,
            verdict: Verdict::Bloque,
            octets,
            ip_src: Some(ip_src),
            ip_dst: Some(ip_dst),
            port_dst,
            protocole: Some(protocole),
            motif: Some(prefixe.motif),
        })
    }

    /// Recoupements avec les règles générées par la Douane ; rend les zones.
    fn recouper(
        &self,
        prefixe: PrefixeDouane,
        groupe: u16,
        (iif, oif): (&str, &str),
        (ip_src, ip_dst): (IpAddr, IpAddr),
    ) -> Result<(Zone, Zone), Rejet> {
        use Rejet::Incoherente as I;

        // Recoupements avec les règles générées par la Douane.
        let PrefixeDouane { lieu, chaine, motif } = prefixe;
        if groupe != lieu.groupe_nflog() {
            return Err(I("groupe NFLOG ≠ groupe du lieu"));
        }
        if !motif.produit_par_la_douane(lieu, chaine) {
            return Err(I("motif non produit à ce lieu ou dans cette chaîne"));
        }
        let interfaces_ok = match chaine {
            Chaine::Entree => !iif.is_empty() && oif.is_empty(),
            Chaine::Sortie => iif.is_empty() && !oif.is_empty(),
            Chaine::Transit => !iif.is_empty() && !oif.is_empty(),
        };
        if !interfaces_ok {
            return Err(I("interfaces IN / OUT incompatibles avec la chaîne"));
        }
        if motif == MotifJournal::Lateral && !(iif.starts_with("sv") && oif.starts_with("sv")) {
            return Err(I("LATERAL hors ports de sandbox"));
        }
        if est_boucle_locale(ip_src) || est_boucle_locale(ip_dst) {
            return Err(I("boucle locale journalisée"));
        }
        // Chaîne `sortie` du lieu `zone-cog` (crochet output : table
        // `inet miyudouane_zone` de l'espace COG, table `inet miyudouane_hote`
        // des comptes de service COG de l'hôte) : le paquet est émis **par la
        // zone COG**, quelle que soit son adresse source (sur l'hôte, une
        // adresse de l'hôte, classée « extérieur » par le plan). La source est
        // donc en zone COG : sortie de zone ⇒ `R-EX12-SORTIE` (ou
        // `R-A005-NON-DECLARE` vers la zone COG / le Portail), toujours critique.
        let zone_src = if lieu == Lieu::ZoneCog && chaine == Chaine::Sortie {
            Zone::Cog
        } else {
            self.plan.zone_de(ip_src)
        };
        let zone_dst = self.plan.zone_de(ip_dst);
        let filtre_cog = !(lieu.est_zone() && chaine == Chaine::Sortie);
        match motif {
            MotifJournal::SortieCog if filtre_cog && zone_src != Zone::Cog => {
                return Err(I("SORTIE-COG : source hors zone COG"));
            }
            MotifJournal::EntreeCog if zone_src == Zone::Cog || zone_dst != Zone::Cog => {
                return Err(I("ENTREE-COG : adresses incompatibles"));
            }
            MotifJournal::NonDeclare
                if !lieu.est_zone() && (zone_src == Zone::Cog || zone_dst == Zone::Cog) =>
            {
                return Err(I("NON-DECLARE : la zone COG aurait été journalisée à part"));
            }
            MotifJournal::NonDeclare
                if lieu == Lieu::ZoneCog && chaine == Chaine::Transit && zone_src == Zone::Cog =>
            {
                return Err(I("NON-DECLARE : SORTIE-COG attendu"));
            }
            _ => {}
        }
        Ok((zone_src, zone_dst))
    }
}

enum Rejet {
    MalFormee(&'static str),
    Incoherente(&'static str),
}

/// `nflog-<G>:` (G décimal, 1 à 65535, sans zéro de tête).
fn analyser_emetteur(jeton: &str) -> Option<u16> {
    let g = jeton.strip_prefix("nflog-")?.strip_suffix(':')?;
    if g.is_empty() || g.len() > 5 || g.starts_with('0') || !g.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    g.parse().ok()
}

/// Jetons `CLE=valeur` ou drapeaux `CLE` ; clé en double refusée.
fn analyser_champs<'a>(jetons: &[&'a str]) -> Result<HashMap<&'a str, &'a str>, Rejet> {
    use Rejet::MalFormee as M;
    let mut vus = HashSet::new();
    let mut champs = HashMap::new();
    for j in jetons {
        let (cle, valeur) = match j.split_once('=') {
            Some((c, v)) => (c, Some(v)),
            None => (*j, None),
        };
        if cle.is_empty() || !cle.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit()) {
            return Err(M("clé invalide"));
        }
        if !vus.insert(cle) {
            return Err(M("clé en double"));
        }
        if let Some(v) = valeur {
            if !v
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'-' | b'_'))
            {
                return Err(M("valeur invalide"));
            }
            champs.insert(cle, v);
        }
    }
    Ok(champs)
}

/// `MDOU|<lieu>|<chaine>|<MOTIF>|` exactement.
fn analyser_prefixe(jeton: &str) -> Result<PrefixeDouane, Rejet> {
    use Rejet::MalFormee as M;
    let reste = jeton
        .strip_prefix(MARQUEUR_PREFIXE)
        .ok_or(M("préfixe absent en 4e position"))?;
    let parties: Vec<&str> = reste.split('|').collect();
    // lieu, chaîne, motif, "" (le préfixe finit par '|')
    if parties.len() != 4 || !parties[3].is_empty() {
        return Err(M("préfixe : nombre de champs"));
    }
    Ok(PrefixeDouane {
        lieu: Lieu::depuis_nom(parties[0]).ok_or(M("préfixe : lieu"))?,
        chaine: Chaine::depuis_nom(parties[1]).ok_or(M("préfixe : chaîne"))?,
        motif: MotifJournal::depuis_nom(parties[2]).ok_or(M("préfixe : motif"))?,
    })
}

/// `secondes.microsecondes` (6 chiffres exactement) → millisecondes.
fn analyser_horodatage(s: &str) -> Option<u64> {
    let (sec, micro) = s.split_once('.')?;
    if sec.is_empty() || micro.len() != 6 {
        return None;
    }
    if !sec.bytes().all(|b| b.is_ascii_digit()) || !micro.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let sec: u64 = sec.parse().ok()?;
    let micro: u64 = micro.parse().ok()?;
    sec.checked_mul(1000)?.checked_add(micro / 1000)
}

/// FNV-1a 64 bits (identifiant d'événement, non cryptographique).
pub(crate) fn fnv1a(octets: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &o in octets {
        h ^= u64::from(o);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Préfixe réel du pont de zone COG (golden `zone-cog.nft`).
    const LIGNE: &str = "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv0 OUT=z-pi SRC=10.77.0.10 DST=93.184.216.34 LEN=60 TTL=64 DF PROTO=TCP SPT=40000 DPT=443 SYN";

    fn lecteur() -> LecteurNftables {
        let mut l = LecteurNftables::new(PlanAdressage::indicatif());
        l.declarer_interface_voie("sv0", VoieId::new("jayoffice").unwrap());
        l
    }

    /// Non-régression EX-12 : ligne réelle de `miyudouane hote` (compte de
    /// service COG sur l'hôte, source = adresse de l'hôte, chaîne `sortie`) et
    /// de la chaîne `sortie` de `inet miyudouane_zone` : source en zone COG ⇒
    /// `R-EX12-SORTIE` critique.
    #[test]
    fn sortie_zone_cog_hote_et_espace_cog_critique() {
        use crate::regles::{JeuRegles, REGLE_SORTIE};
        use crate::signal::Severite;
        for l in [
            // Hôte : adresse publique de l'hôte vers l'extérieur, IPv4 et IPv6.
            "1729760000.123456 hote nflog-110: MDOU|zone-cog|sortie|SORTIE-COG| IN= OUT=eth0 SRC=192.0.2.10 DST=93.184.216.34 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=443 SYN",
            "1729760000.123457 hote nflog-110: MDOU|zone-cog|sortie|SORTIE-COG| IN= OUT=eth0 SRC=2001:db8::10 DST=2606:2800:220:1::1 LEN=80 TTL=64 PROTO=UDP SPT=40000 DPT=53",
            // Espace COG (table inet miyudouane_zone), source dans la zone COG.
            "1729760000.123458 hote nflog-110: MDOU|zone-cog|sortie|SORTIE-COG| IN= OUT=z-pi SRC=10.77.0.10 DST=1.1.1.1 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=443 SYN",
        ] {
            let ResultatLigne::Evenement(e) = lecteur().analyser(l) else {
                panic!("ligne refusée : {:?}", lecteur().analyser(l));
            };
            assert_eq!(e.zone_src, Zone::Cog, "{l}");
            let d = JeuRegles::vide().evaluer(&e);
            assert!(
                d.iter().any(|d| d.regle.as_str() == REGLE_SORTIE && d.severite == Severite::Critique),
                "{l} : {d:?}"
            );
        }
        // Hors zone-cog/sortie : l'adresse décide toujours (pas de forçage).
        let ResultatLigne::Evenement(e) = lecteur().analyser(LIGNE) else { panic!() };
        assert_eq!(e.zone_src, Zone::Cog);
    }

    #[test]
    fn ligne_valide() {
        let ResultatLigne::Evenement(e) = lecteur().analyser(LIGNE) else {
            panic!("ligne valide refusée");
        };
        assert_eq!(e.horodatage_ms, 1_729_760_000_123);
        assert_eq!(e.zone_src, Zone::Cog);
        assert_eq!(e.zone_dst, Zone::Exterieur);
        assert_eq!(e.verdict, Verdict::Bloque);
        assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jayoffice"));
        assert_eq!(e.octets, 60);
        assert_eq!(e.port_dst, Some(443));
    }

    /// B1 : TCP / UDP sans ports (fragment non initial, troncature) ⇒ ligne
    /// acceptée, protocole TCP / UDP, port inconnu ; marqueur incohérent ⇒
    /// mal formée.
    #[test]
    fn sans_ports_frag_tronque() {
        let base = "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv0 OUT=z-pi SRC=10.77.0.10 DST=93.184.216.34 LEN=1480 TTL=64";
        for (suffixe, proto) in [(" PROTO=6 FRAG", Protocole::Tcp), (" PROTO=17 TRONQUE", Protocole::Udp)] {
            let l = format!("{base}{suffixe}");
            let ResultatLigne::Evenement(e) = lecteur().analyser(&l) else {
                panic!("refusée : {l} → {:?}", lecteur().analyser(&l));
            };
            assert_eq!((e.protocole, e.port_dst), (Some(proto), None));
            assert_eq!(e.zone_src, Zone::Cog);
        }
        for mauvais in [" PROTO=6", " PROTO=17", " PROTO=TCP FRAG", " PROTO=6 SPT=1 DPT=2 FRAG", " PROTO=1 TRONQUE"] {
            let l = format!("{base}{mauvais}");
            assert!(matches!(lecteur().analyser(&l), ResultatLigne::MalFormee(_)), "acceptée à tort : {l}");
        }
    }

    #[test]
    fn frontiere_sans_voie_et_interfaces_vides() {
        // Crochet input : OUT vide (format du noyau).
        let l = "1729760000.123456 hote nflog-102: MDOU|portail-interne|entree|ENTREE-COG| IN=pi-franche OUT= SRC=10.77.1.10 DST=10.77.0.253 LEN=60 PROTO=TCP SPT=40000 DPT=7443";
        let ResultatLigne::Evenement(e) = lecteur().analyser(l) else {
            panic!("refusée : {:?}", lecteur().analyser(l));
        };
        assert_eq!((e.zone_src, e.zone_dst), (Zone::Franche, Zone::Cog));
        assert_eq!(e.voie, None);
    }

    #[test]
    fn hors_convention() {
        assert_eq!(
            lecteur().analyser("1729760000.123456 hote kernel: usb 1-1: new device"),
            ResultatLigne::HorsConvention
        );
        // L'ancienne convention `MIYU:` n'est plus reconnue.
        assert_eq!(
            lecteur().analyser("1729760000.123456 hote kernel: MIYU:bloque:cog>ext:jaymail: IN=a OUT=b SRC=10.77.0.5 DST=1.1.1.1 LEN=60 PROTO=TCP DPT=443"),
            ResultatLigne::HorsConvention
        );
    }

    #[test]
    fn lignes_mal_formees() {
        let l = lecteur();
        let cas = [
            LIGNE.replace("1729760000.123456", "1729760000.12"),
            LIGNE.replace(" SRC=10.77.0.10", ""),
            LIGNE.replace("SRC=10.77.0.10", "SRC=10.77.0.10 SRC=10.77.0.11"),
            LIGNE.replace("|SORTIE-COG|", "|SORTIE|"),
            LIGNE.replace("|transit|", "|forward|"),
            LIGNE.replace("zone-cog|", "zone-dmz|"),
            LIGNE.replace("SORTIE-COG| ", "SORTIE-COG "),
            LIGNE.replace("SORTIE-COG|", "SORTIE-COG|x|"),
            LIGNE.replace("LEN=60", "LEN=-1"),
            LIGNE.replace(" DPT=443", ""),
            LIGNE.replace("nflog-110:", "kernel:"),
            LIGNE.replace("nflog-110:", "nflog-0110:"),
            LIGNE.replace("nflog-110:", "nflog-:"),
            LIGNE.replace(" IN=", "  IN="),
            LIGNE.replace("DST=93.184.216.34", "DST=93.184.216.999"),
            format!("{LIGNE}\u{7}"),
            "MDOU|".to_string(),
            "x".repeat(LONGUEUR_MAX_LIGNE + 1) + MARQUEUR_PREFIXE,
        ];
        for c in cas {
            assert!(
                matches!(l.analyser(&c), ResultatLigne::MalFormee(_)),
                "acceptée à tort : {c} → {:?}",
                l.analyser(&c)
            );
        }
    }

    #[test]
    fn lignes_incoherentes() {
        let l = lecteur();
        let cas = [
            // Groupe d'un autre lieu.
            LIGNE.replace("nflog-110:", "nflog-101:"),
            // SORTIE-COG avec une source hors zone COG.
            LIGNE.replace("SRC=10.77.0.10", "SRC=10.77.1.10"),
            // LATERAL hors pont de zone / hors ports de sandbox.
            LIGNE.replace("nflog-110: MDOU|zone-cog|", "nflog-101: MDOU|externe|").replace("SORTIE-COG", "LATERAL"),
            LIGNE.replace("SORTIE-COG", "LATERAL"),
            // TAILLE n'existe qu'aux frontières.
            LIGNE.replace("SORTIE-COG", "TAILLE"),
            // Chaîne `entree` avec OUT renseigné.
            LIGNE.replace("|transit|", "|entree|").replace("SORTIE-COG", "NON-DECLARE"),
            // Boucle locale (acceptée avant journalisation).
            LIGNE.replace("DST=93.184.216.34", "DST=127.0.0.1"),
            // NON-DECLARE d'une frontière impliquant la zone COG.
            "1729760000.123456 hote nflog-101: MDOU|externe|transit|NON-DECLARE| IN=ext0 OUT=ext-franche SRC=8.8.8.8 DST=10.77.0.10 LEN=60 PROTO=TCP SPT=1 DPT=22".to_string(),
        ];
        for c in cas {
            assert!(
                matches!(l.analyser(&c), ResultatLigne::Incoherente(_)),
                "acceptée à tort : {c} → {:?}",
                l.analyser(&c)
            );
        }
    }

    #[test]
    fn motif_porte_par_l_evenement() {
        let ResultatLigne::Evenement(e) = lecteur().analyser(LIGNE) else {
            panic!("ligne valide refusée");
        };
        assert_eq!(e.motif, Some(MotifJournal::SortieCog));
    }

    /// Anti-usurpation (A-005) : une ligne qui n'atteint la sandbox d'une voie
    /// que par `OUT=` n'attribue aucune voie (la victime n'est pas coupée).
    #[test]
    fn voie_jamais_attribuee_par_l_interface_de_sortie() {
        let mut l = lecteur();
        l.declarer_interface_voie("sv1", VoieId::new("jaymail").unwrap());
        // Trafic entrant vers la sandbox de « jaymail » depuis le port du Portail.
        let entrant = "1729760000.123456 hote nflog-111: MDOU|zone-franche|transit|NON-DECLARE| IN=z-pi OUT=sv1 SRC=10.77.1.253 DST=10.77.1.11 LEN=60 PROTO=TCP SPT=40000 DPT=22";
        let ResultatLigne::Evenement(e) = l.analyser(entrant) else {
            panic!("refusée : {:?}", l.analyser(entrant));
        };
        assert_eq!(e.voie, None, "OUT= n'attribue pas de voie");
        // Latéral sv0 → sv1 : la voie est celle de l'émetteur (sv0), jamais sv1.
        let lateral = "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|LATERAL| IN=sv0 OUT=sv1 SRC=10.77.0.10 DST=10.77.0.11 LEN=60 PROTO=TCP SPT=40000 DPT=22";
        let ResultatLigne::Evenement(e) = l.analyser(lateral) else {
            panic!("refusée : {:?}", l.analyser(lateral));
        };
        assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jayoffice"));
        assert_eq!(e.motif, Some(MotifJournal::Lateral));
        // Émetteur non déclaré : aucune voie, même si la cible l'est.
        let inconnu = lateral.replace("IN=sv0 OUT=sv1", "IN=sv7 OUT=sv1");
        let ResultatLigne::Evenement(e) = l.analyser(&inconnu) else {
            panic!("refusée");
        };
        assert_eq!(e.voie, None);
    }

    #[test]
    fn identifiant_stable() {
        let a = lecteur().analyser(LIGNE);
        let b = lecteur().analyser(LIGNE);
        assert_eq!(a, b);
    }
}
