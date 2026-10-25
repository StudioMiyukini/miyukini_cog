//! @id     miyurelais_relais
//! @do     relayer_paquets_nflog_vers_vigies
//! @role   securite
//! @layer  toolkit
//! @human  miyurelais — relais NFLOG : réception, décodage, détection des pertes, lignes MDOU| pour les Vigies, métadonnées pour l'IPS, capteur muet.

//! Relais NFLOG, indépendant du transport (socket netlink réelle ou trames
//! d'essai) : implémente [`SourceJournal`] pour une Vigie.
//!
//! Une relève :
//!
//! 1. lit les datagrammes disponibles (au plus [`LIGNES_MAX_PAR_RELEVE`]
//!    lignes) ;
//! 2. décode chaque datagramme strictement ; datagramme refusé ⇒ compté
//!    « mal formé » (la Vigie passe dégradée) ;
//! 3. vérifie la **séquence** locale de chaque groupe (`NFULNL_CFG_F_SEQ`) :
//!    un trou = paquets perdus ;
//! 4. convertit chaque paquet en ligne canonique (ou témoin de vie) ;
//! 5. nourrit la veille du capteur muet.
//!
//! **Échecs** (la relève rend `Err` : lot en échec, voie aveugle coupée) :
//! perte (trou de séquence, `ENOBUFS`, `NLMSG_OVERRUN`, datagramme tronqué),
//! erreur de la socket, capteur muet. Si des lignes ont été lues dans la même
//! relève, elles sont rendues d'abord et l'échec l'est à la relève suivante :
//! un événement critique n'est jamais retenu derrière une panne.

use std::collections::{BTreeMap, VecDeque};
use std::fmt;

use miyuvigie::{CapteurId, ErreurSource, ReleveJournal, SourceJournal};

use crate::ligne::{convertir, hote_valide, interfaces, meta_flux, nom_interface_valide, Contexte, Conversion, ECART_HORODATAGE_MAX_US};
use crate::netlink::{decoder_datagramme, Message, PaquetNflog};
use crate::veille::{EtatVeille, Veille};

/// Lignes au plus par relève (le reste attend dans la socket).
pub const LIGNES_MAX_PAR_RELEVE: usize = 4096;
/// Métadonnées de flux en attente au plus pour l'IPS (au-delà : comptées perdues).
pub const META_FLUX_MAX: usize = 4096;
/// Taille du tampon de réception (octets).
pub const TAMPON_RECEPTION: usize = 65_536;

/// Échec du transport netlink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurTransport {
    /// `ENOBUFS` : la file de la socket a débordé, paquets perdus.
    Debordement,
    /// Datagramme plus long que le tampon (taille réelle).
    Tronque(usize),
    /// Autre erreur d'entrée / sortie.
    Io(String),
}

impl fmt::Display for ErreurTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debordement => f.write_str("ENOBUFS : file netlink débordée, paquets perdus"),
            Self::Tronque(n) => write!(f, "datagramme de {n} octets tronqué"),
            Self::Io(e) => write!(f, "socket : {e}"),
        }
    }
}

/// Transport des datagrammes NFLOG (socket netlink, ou essai).
pub trait Transport {
    /// Reçoit un datagramme sans attendre ; `Ok(None)` : rien de disponible.
    fn recevoir(&mut self, tampon: &mut [u8]) -> Result<Option<usize>, ErreurTransport>;

    /// Nom de l'interface d'index `index` dans l'espace réseau du relais.
    fn nom_interface(&mut self, index: u32) -> Option<String>;
}

/// Configuration du relais.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigRelais {
    /// Capteur représenté (celui de la Vigie qui relève).
    pub capteur: CapteurId,
    /// Nom d'hôte écrit dans les lignes (`[A-Za-z0-9._-]{1,64}`).
    pub hote: String,
    /// Groupes NFLOG attendus (tout autre groupe ⇒ paquet refusé).
    pub groupes: Vec<u16>,
    /// Silence toléré d'une voie active (ms) avant « capteur muet ».
    pub delai_muet_ms: u64,
}

/// Configuration refusée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurConfig(pub &'static str);

impl fmt::Display for ErreurConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "configuration du relais : {}", self.0)
    }
}

impl std::error::Error for ErreurConfig {}

/// Compteurs cumulés du relais.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Statistiques {
    /// Datagrammes reçus.
    pub datagrammes: u64,
    /// Datagrammes refusés par le décodeur.
    pub datagrammes_refuses: u64,
    /// Paquets décodés.
    pub paquets: u64,
    /// Lignes produites.
    pub lignes: u64,
    /// Témoins de vie reçus.
    pub temoins: u64,
    /// Paquets refusés à la conversion (ou d'un groupe inattendu).
    pub paquets_refuses: u64,
    /// Paquets perdus (trous de séquence).
    pub pertes: u64,
    /// Débordements et troncatures de la socket.
    pub debordements: u64,
    /// Métadonnées de flux écartées (file de l'IPS pleine).
    pub meta_perdues: u64,
}

/// Relais NFLOG.
pub struct Relais<T: Transport> {
    config: ConfigRelais,
    transport: T,
    horloge_us: Box<dyn FnMut() -> u64 + Send>,
    veille: Veille,
    dernier_us: u64,
    sequences: BTreeMap<u16, u32>,
    echec_differe: Option<String>,
    stats: Statistiques,
    tampon: Vec<u8>,
    meta: Option<VecDeque<miyudouane::MetaFlux>>,
}

impl<T: Transport> fmt::Debug for Relais<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Relais")
            .field("config", &self.config)
            .field("veille", &self.veille)
            .field("stats", &self.stats)
            .finish_non_exhaustive()
    }
}

fn horloge_systeme_us() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_micros()).unwrap_or(u64::MAX))
}

impl<T: Transport> Relais<T> {
    /// Relais sur `transport`, horloge système.
    pub fn new(config: ConfigRelais, transport: T) -> Result<Self, ErreurConfig> {
        if !hote_valide(&config.hote) {
            return Err(ErreurConfig("nom d'hôte invalide"));
        }
        if config.groupes.is_empty() || config.groupes.contains(&0) {
            return Err(ErreurConfig("groupes NFLOG : au moins un, jamais 0"));
        }
        let veille = Veille::new(config.delai_muet_ms);
        Ok(Self {
            config,
            transport,
            horloge_us: Box::new(horloge_systeme_us),
            veille,
            dernier_us: 0,
            sequences: BTreeMap::new(),
            echec_differe: None,
            stats: Statistiques::default(),
            tampon: vec![0; TAMPON_RECEPTION],
            meta: None,
        })
    }

    /// Remplace l'horloge (µs depuis l'époque Unix) — essais.
    #[must_use]
    pub fn avec_horloge(mut self, horloge_us: Box<dyn FnMut() -> u64 + Send>) -> Self {
        self.horloge_us = horloge_us;
        self
    }

    /// Conserve aussi les métadonnées de flux pour l'IPS ([`Self::prendre_meta_flux`]).
    #[must_use]
    pub fn avec_meta_flux(mut self) -> Self {
        self.meta = Some(VecDeque::new());
        self
    }

    /// Déclare la voie active (la veille du capteur muet s'arme).
    pub fn activer_voie(&mut self) {
        let t = (self.horloge_us)() / 1000;
        self.veille.activer(t);
    }

    /// Déclare la voie inactive (silence normal).
    pub fn desactiver_voie(&mut self) {
        self.veille.desactiver();
    }

    /// État de la veille maintenant.
    pub fn etat_veille(&mut self) -> EtatVeille {
        let t = (self.horloge_us)() / 1000;
        self.veille.etat(t)
    }

    /// Compteurs cumulés.
    pub fn statistiques(&self) -> Statistiques {
        self.stats
    }

    /// Transport (diagnostic).
    pub fn transport(&self) -> &T {
        &self.transport
    }

    /// Métadonnées de flux accumulées depuis le dernier appel (IPS :
    /// `moteur.traiter(&mut relais.prendre_meta_flux().into_iter())`).
    pub fn prendre_meta_flux(&mut self) -> Vec<miyudouane::MetaFlux> {
        self.meta.as_mut().map(|f| f.drain(..).collect()).unwrap_or_default()
    }

    fn horodatage(&mut self, noyau_us: Option<u64>, maintenant_us: u64) -> u64 {
        let t = noyau_us
            .filter(|k| k.abs_diff(maintenant_us) <= ECART_HORODATAGE_MAX_US)
            .unwrap_or(maintenant_us);
        let t = if t <= self.dernier_us { self.dernier_us.saturating_add(1) } else { t };
        self.dernier_us = t;
        t
    }

    fn nom(&mut self, index: Option<u32>) -> String {
        match index {
            None => String::new(),
            Some(i) => self
                .transport
                .nom_interface(i)
                .filter(|n| nom_interface_valide(n))
                .unwrap_or_else(|| format!("if{i}")),
        }
    }

    /// Traite un paquet ; `Some(ligne)` à rendre.
    fn traiter(&mut self, p: &PaquetNflog, maintenant_us: u64, releve: &mut ReleveJournal, pertes: &mut u64) {
        self.stats.paquets += 1;
        self.veille.observer(maintenant_us / 1000);
        if !self.config.groupes.contains(&p.groupe) {
            self.stats.paquets_refuses += 1;
            releve.mal_formees += 1;
            return;
        }
        if let Some(s) = p.sequence {
            if let Some(precedente) = self.sequences.insert(p.groupe, s) {
                let attendue = precedente.wrapping_add(1);
                if s != attendue {
                    *pertes += u64::from(s.wrapping_sub(attendue));
                }
            }
        }
        let horodatage_us = self.horodatage(p.horodatage_us, maintenant_us);
        let (index_entree, index_sortie) = interfaces(p);
        let entree = self.nom(index_entree);
        let sortie = self.nom(index_sortie);
        let ctx = Contexte { hote: &self.config.hote, horodatage_us, entree: &entree, sortie: &sortie };
        match convertir(p, &ctx) {
            Ok(Conversion::Ligne(l)) => {
                self.stats.lignes += 1;
                releve.lignes.push(l);
                if let Some(file) = self.meta.as_mut() {
                    let e = (!entree.is_empty()).then_some(entree.as_str());
                    if let Some(m) = meta_flux(p, horodatage_us / 1_000_000, e) {
                        if file.len() >= META_FLUX_MAX {
                            file.pop_front();
                            self.stats.meta_perdues += 1;
                        }
                        file.push_back(m);
                    }
                }
            }
            Ok(Conversion::Temoin) => self.stats.temoins += 1,
            Err(_) => {
                self.stats.paquets_refuses += 1;
                releve.mal_formees += 1;
            }
        }
    }

    /// Relève : voir la documentation du module.
    pub fn relever(&mut self) -> Result<ReleveJournal, ErreurSource> {
        let mut releve = ReleveJournal::default();
        let mut echec = self.echec_differe.take();
        let mut pertes = 0u64;
        let mut tampon = std::mem::take(&mut self.tampon);
        let mut lectures = 0usize;
        while releve.lignes.len() < LIGNES_MAX_PAR_RELEVE && lectures < LIGNES_MAX_PAR_RELEVE {
            lectures += 1;
            let n = match self.transport.recevoir(&mut tampon) {
                Ok(None) => break,
                Ok(Some(n)) => n,
                Err(e) => {
                    if matches!(e, ErreurTransport::Debordement | ErreurTransport::Tronque(_)) {
                        self.stats.debordements += 1;
                    }
                    echec.get_or_insert_with(|| e.to_string());
                    // Débordement : la socket reste utilisable, on continue de lire.
                    if matches!(e, ErreurTransport::Io(_)) {
                        break;
                    }
                    continue;
                }
            };
            self.stats.datagrammes += 1;
            let maintenant_us = (self.horloge_us)();
            match decoder_datagramme(tampon.get(..n).unwrap_or_default()) {
                Ok(messages) => {
                    for m in messages {
                        if let Message::Paquet(p) = m {
                            self.traiter(&p, maintenant_us, &mut releve, &mut pertes);
                        }
                    }
                }
                Err(e) => {
                    self.stats.datagrammes_refuses += 1;
                    releve.mal_formees += 1;
                    // Un NLMSG_OVERRUN annonce une perte : échec, pas seulement mal formé.
                    if e == crate::netlink::ErreurTrame::Debordement {
                        self.stats.debordements += 1;
                        echec.get_or_insert_with(|| e.to_string());
                    } else {
                        // Le datagramme refusé a pu porter des paquets : la veille voit le noyau vivant.
                        self.veille.observer(maintenant_us / 1000);
                    }
                }
            }
        }
        self.tampon = tampon;
        if pertes > 0 {
            self.stats.pertes += pertes;
            echec.get_or_insert_with(|| format!("{pertes} paquet(s) NFLOG perdu(s) (trou de séquence)"));
        }
        let maintenant_ms = (self.horloge_us)() / 1000;
        if let EtatVeille::Muette { silence_ms } = self.veille.etat(maintenant_ms) {
            echec.get_or_insert_with(|| {
                format!("capteur muet : aucun paquet NFLOG depuis {silence_ms} ms alors que la voie est active")
            });
        }
        match echec {
            None => Ok(releve),
            Some(e) if releve.lignes.is_empty() => Err(ErreurSource(e)),
            Some(e) => {
                // Lignes d'abord, échec à la relève suivante.
                self.echec_differe = Some(e);
                Ok(releve)
            }
        }
    }
}

impl<T: Transport> SourceJournal for Relais<T> {
    fn capteur(&self) -> &CapteurId {
        &self.config.capteur
    }

    fn relever_lignes(&mut self) -> Result<ReleveJournal, ErreurSource> {
        self.relever()
    }
}

/// Transport d'essai : datagrammes en file, noms d'interfaces fixés.
#[derive(Debug, Clone, Default)]
pub struct TransportEssai {
    /// Datagrammes (ou erreurs) à rendre, dans l'ordre.
    pub file: VecDeque<Result<Vec<u8>, ErreurTransport>>,
    /// Noms des interfaces par index.
    pub noms: BTreeMap<u32, String>,
}

impl Transport for TransportEssai {
    fn recevoir(&mut self, tampon: &mut [u8]) -> Result<Option<usize>, ErreurTransport> {
        match self.file.pop_front() {
            None => Ok(None),
            Some(Err(e)) => Err(e),
            Some(Ok(d)) if d.len() > tampon.len() => Err(ErreurTransport::Tronque(d.len())),
            Some(Ok(d)) => {
                tampon[..d.len()].copy_from_slice(&d);
                Ok(Some(d.len()))
            }
        }
    }

    fn nom_interface(&mut self, index: u32) -> Option<String> {
        self.noms.get(&index).cloned()
    }
}
