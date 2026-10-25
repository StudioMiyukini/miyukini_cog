//! @id     central_canal_officiel
//! @do     relever_canal_officiel_central
//! @role   securite
//! @layer  app
//! @human  central — canal officiel côté Central : relevé périodique, installation gouvernée, état persistant, signaux WorrySentinel.

//! Canal officiel côté Central (lot 4, amendement A-006 ; `crates/miyucanal`).
//!
//! - [`installateur`] : [`InstallateurCentral`], branché sur le chemin
//!   d'installation StrongFather existant (Cores), refus explicite de l'OS,
//!   modèles IA délégués ;
//! - [`persistance`] : secret local `0600`, miroir, état du relevé et horloge
//!   de confiance, quarantaine scellée, sous le dossier de données ;
//! - [`alertes`] : refus du canal → signaux WorrySentinel (journal + puits).
//!
//! # Configuration (défaut : désactivé)
//!
//! `<dossier de configuration>/canal-officiel.json` :
//!
//! ```json
//! { "actif": true, "registre": "<64 hex>", "origin": "<64 hex>",
//!   "journal_origin": "<64 hex>", "editeur": "<64 hex>", "periode_secs": 3600,
//!   "modeles": { "dossier": "/chemin/absolu", "ancres": ["<64 hex>"] } }
//! ```
//!
//! Fichier absent, `actif` faux ou contenu invalide ⇒ canal **désactivé**
//! (journal explicite). Clés : Origin (signature), clé de journal d'Origin
//! (têtes), éditeur hors ligne (annonces 2 sur 2), toutes distinctes.
//! `modeles` (facultatif) branche l'installateur de modèles IA délégué à
//! maia-llm ([`modeles`]) ; absent ⇒ modèles refusés.
//!
//! # Reprise (fail-closed, A-006)
//!
//! - premier démarrage (aucun état) : confiance au premier usage ; un miroir
//!   non confirmé est retiré et signalé ;
//! - état illisible, secret local douteux, reprise refusée (signature, autre
//!   registre, stockage incohérent) ⇒ canal **désactivé**, rien n'est installé ;
//! - quarantaine absente, altérée ou illisible ⇒ quarantaine **relancée** pour
//!   les annonces concernées (jamais raccourcie), refus signalé.
//!
//! # Cycle
//!
//! [`CanalCentral::cycle`] (appel explicite) ou [`lancer_cycles`] (fil
//! périodique). La voie de relevé réelle (voie réservée du Portail) n'est pas
//! encore branchée : [`demarrer`] ouvre et vérifie l'état mais ne lance aucun
//! cycle tant qu'aucun [`TransportCanal`] n'est fourni.

pub mod alertes;
pub mod installateur;
pub mod modeles;
pub mod persistance;

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::Duration;

use miyucanal::{CleQuarantaine, ConfigCanal, ErreurCanal, Evenement, Releveur, TransportCanal};
use miyuregistre::{HorlogeConfiance, Stockage, StockageFichier};
use serde::Deserialize;
use worrysentinel::ThreatLevel;

pub use alertes::{PuitsJournal, PuitsMemoire, PuitsSignaux, SignalCanal};
pub use installateur::InstallateurCentral;
pub use modeles::ConfigModeles;

use crate::service_manager::ServiceManager;
use persistance::{EtatCanal, FICHIER_ETAT, FICHIER_MIROIR, FICHIER_QUARANTAINE};

/// Fichier de configuration du canal (dossier de configuration de Central).
pub const FICHIER_CONFIG: &str = "canal-officiel.json";
/// Sous-dossier d'état (dossier de données de Central).
pub const DOSSIER_ETAT: &str = "canal-officiel";
/// Période par défaut des relevés.
pub const PERIODE_DEFAUT_SECS: u64 = 3_600;
/// Période minimale des relevés.
pub const PERIODE_MIN_SECS: u64 = 300;

fn periode_defaut() -> u64 {
    PERIODE_DEFAUT_SECS
}

/// Configuration du canal officiel (voir module).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigCanalCentral {
    /// Canal actif (défaut : non).
    #[serde(default)]
    pub actif: bool,
    /// Identifiant du registre officiel (64 hex).
    pub registre: String,
    /// Clé publique de signature d'Origin (64 hex).
    pub origin: String,
    /// Clé publique de journal d'Origin (64 hex).
    pub journal_origin: String,
    /// Clé publique de l'éditeur hors ligne (64 hex).
    pub editeur: String,
    /// Période des relevés (secondes, ≥ 300).
    #[serde(default = "periode_defaut")]
    pub periode_secs: u64,
    /// Installateur de modèles IA délégué (absent ⇒ modèles refusés).
    #[serde(default)]
    pub modeles: Option<ConfigModeles>,
}

impl ConfigCanalCentral {
    /// Charge la configuration ; fichier absent ⇒ `Ok(None)`.
    pub fn charger(chemin: &Path) -> Result<Option<Self>, String> {
        match persistance::lire(chemin)? {
            None => Ok(None),
            Some(o) => serde_json::from_slice(&o).map(Some).map_err(|e| format!("{} : {e}", chemin.display())),
        }
    }

    /// Configuration vérifiée du canal (clés et registre décodés, clés distinctes).
    pub fn config_canal(&self) -> Result<ConfigCanal, String> {
        let hex = |nom: &str, v: &str| {
            miyumarket::signature::cle_publique_depuis_hex(v).ok_or_else(|| format!("{nom} : 64 hex attendus"))
        };
        ConfigCanal::nouvelle(
            hex("registre", &self.registre)?,
            &hex("origin", &self.origin)?,
            &hex("journal_origin", &self.journal_origin)?,
            &hex("editeur", &self.editeur)?,
        )
        .map_err(|e| e.to_string())
    }

    /// Installateur `installateur` avec, si configuré, l'installateur de
    /// modèles délégué branché ; configuration `modeles` invalide ⇒ erreur.
    pub fn brancher_modeles(&self, installateur: InstallateurCentral) -> Result<InstallateurCentral, String> {
        match &self.modeles {
            None => Ok(installateur),
            Some(m) => Ok(installateur.avec_modeles(Box::new(m.installateur()?))),
        }
    }

    /// Période effective des relevés.
    pub fn periode(&self) -> Duration {
        Duration::from_secs(self.periode_secs.max(PERIODE_MIN_SECS))
    }
}

/// Canal officiel de Central : relevé, installation, état persistant, signaux.
pub struct CanalCentral {
    releveur: Releveur<StockageFichier>,
    horloge: HorlogeConfiance,
    cle: CleQuarantaine,
    dossier: PathBuf,
    installateur: InstallateurCentral,
    puits: Box<dyn PuitsSignaux>,
}

impl std::fmt::Debug for CanalCentral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CanalCentral").field("dossier", &self.dossier).finish_non_exhaustive()
    }
}

impl CanalCentral {
    /// Ouvre (ou crée) l'état du canal dans `dossier` (voir module : reprise fail-closed).
    pub fn ouvrir(
        config: ConfigCanal,
        dossier: &Path,
        installateur: InstallateurCentral,
        puits: Box<dyn PuitsSignaux>,
    ) -> Result<Self, String> {
        miyukini_central::fichiers_secrets::creer_dossier_prive(dossier)
            .map_err(|e| format!("dossier du canal {} : {e}", dossier.display()))?;
        let cle = CleQuarantaine::deriver(&persistance::secret_local(dossier)?);
        let stockage = StockageFichier::ouvrir(dossier.join(FICHIER_MIROIR)).map_err(|e| format!("miroir : {e}"))?;
        let etat = persistance::lire(&dossier.join(FICHIER_ETAT))?;
        let quarantaine = persistance::lire(&dossier.join(FICHIER_QUARANTAINE))?;
        let (releveur, horloge) = match etat {
            None => {
                if stockage.taille() > 0 {
                    puits.signaler(&SignalCanal {
                        niveau: ThreatLevel::High,
                        motif: "état du relevé absent, miroir non confirmé retiré : reprise au premier usage".into(),
                    });
                }
                let r = Releveur::new(config, stockage).map_err(|e| format!("miroir : {e}"))?;
                (r, HorlogeConfiance::new())
            }
            Some(o) => {
                let e = EtatCanal::decoder(&o).ok_or("état du relevé illisible (etat.mcce)")?;
                let horloge = HorlogeConfiance::decoder(&e.horloge).map_err(|e| format!("horloge de confiance : {e}"))?;
                let r = Releveur::reprendre(config, stockage, &e.etat, &e.temps, quarantaine.as_deref().unwrap_or(&[]), &cle)
                    .map_err(|e| format!("reprise du relevé refusée : {e}"))?;
                (r, horloge)
            }
        };
        Ok(Self { releveur, horloge, cle, dossier: dossier.to_path_buf(), installateur, puits })
    }

    /// Relevé complet puis installation des mises à jour prêtes ; chaque refus
    /// est remonté au puits ; l'état est persisté dans tous les cas.
    pub fn cycle<T: TransportCanal>(&mut self, transport: &mut T, mural: u64) -> Result<Vec<Evenement>, ErreurCanal> {
        let resultat = self.releveur.cycle(transport, &mut self.installateur, &mut self.horloge, mural);
        match &resultat {
            Ok(evenements) => {
                for e in evenements {
                    alertes::remonter(self.puits.as_ref(), e);
                }
            }
            Err(e) => self.puits.signaler(&alertes::signal_erreur(e)),
        }
        if let Err(e) = self.persister() {
            self.puits.signaler(&SignalCanal { niveau: ThreatLevel::High, motif: format!("persistance : {e}") });
        }
        resultat
    }

    /// Persiste l'état du relevé (si une tête a été acceptée) et la quarantaine.
    pub fn persister(&self) -> Result<(), String> {
        if let Some((etat, temps)) = self.releveur.etat_persistant() {
            let e = EtatCanal { etat, temps, horloge: self.horloge.encoder() };
            persistance::ecrire(&self.dossier.join(FICHIER_ETAT), &e.encoder())?;
        }
        persistance::ecrire(&self.dossier.join(FICHIER_QUARANTAINE), &self.releveur.etat_quarantaine(&self.cle))
    }

    /// Releveur (lecture : miroir, annonces en attente).
    pub fn releveur(&self) -> &Releveur<StockageFichier> {
        &self.releveur
    }

    /// Installateur.
    pub fn installateur(&self) -> &InstallateurCentral {
        &self.installateur
    }
}

fn mural() -> u64 {
    crate::service_manager::confiance::maintenant_secs()
}

/// Arrêt d'un fil de cycles périodiques.
#[derive(Debug)]
pub struct CyclesPeriodiques {
    arret: Arc<AtomicBool>,
    fil: Option<std::thread::JoinHandle<()>>,
}

impl CyclesPeriodiques {
    /// Demande l'arrêt et attend la fin du fil.
    pub fn arreter(mut self) {
        self.arret.store(true, Ordering::SeqCst);
        if let Some(f) = self.fil.take() {
            let _ = f.join();
        }
    }
}

/// Lance un fil qui exécute un cycle toutes les `periode` (premier cycle immédiat).
pub fn lancer_cycles<T: TransportCanal + Send + 'static>(
    canal: Arc<Mutex<CanalCentral>>,
    mut transport: T,
    periode: Duration,
) -> CyclesPeriodiques {
    let arret = Arc::new(AtomicBool::new(false));
    let drapeau = arret.clone();
    let fil = std::thread::Builder::new()
        .name("canal-officiel".into())
        .spawn(move || {
            while !drapeau.load(Ordering::SeqCst) {
                {
                    let mut c = canal.lock().unwrap_or_else(PoisonError::into_inner);
                    if let Err(e) = c.cycle(&mut transport, mural()) {
                        tracing::warn!("canal officiel : relevé refusé : {e}");
                    }
                }
                let mut attendu = Duration::ZERO;
                while attendu < periode && !drapeau.load(Ordering::SeqCst) {
                    let pas = Duration::from_millis(200).min(periode.saturating_sub(attendu));
                    std::thread::sleep(pas);
                    attendu += pas;
                }
            }
        })
        .map_err(|e| tracing::error!("canal officiel : fil de relevé non lancé : {e}"))
        .ok();
    CyclesPeriodiques { arret, fil }
}

static CANAL: OnceLock<Option<Arc<Mutex<CanalCentral>>>> = OnceLock::new();

/// Démarrage (une fois par processus) : configuration, état vérifié. Aucun
/// cycle n'est lancé tant que la voie de relevé du Portail n'est pas branchée.
pub fn demarrer(sm: &ServiceManager) -> Option<Arc<Mutex<CanalCentral>>> {
    CANAL.get_or_init(|| ouvrir_depuis_configuration(sm)).clone()
}

fn ouvrir_depuis_configuration(sm: &ServiceManager) -> Option<Arc<Mutex<CanalCentral>>> {
    let chemin = match miyukini_central::chemins::dossier_config() {
        Ok(d) => d.join(FICHIER_CONFIG),
        Err(e) => {
            tracing::error!("canal officiel désactivé : dossier de configuration introuvable ({e})");
            return None;
        }
    };
    let conf = match ConfigCanalCentral::charger(&chemin) {
        Ok(Some(c)) if c.actif => c,
        Ok(Some(_)) => {
            tracing::info!("canal officiel désactivé (« actif » faux dans {})", chemin.display());
            return None;
        }
        Ok(None) => {
            tracing::info!(
                "canal officiel non configuré ({} absent : registre officiel, Origin, journal d'Origin, éditeur) : désactivé",
                chemin.display()
            );
            return None;
        }
        Err(e) => {
            tracing::error!("canal officiel désactivé : configuration invalide : {e}");
            return None;
        }
    };
    let ouvert = conf.config_canal().and_then(|config| {
        let dossier = miyukini_central::chemins::dossier_donnees().map_err(|e| e.to_string())?.join(DOSSIER_ETAT);
        let installateur = conf.brancher_modeles(InstallateurCentral::depuis_service_manager(sm))?;
        if conf.modeles.is_none() {
            tracing::info!("canal officiel : aucun installateur de modèles configuré (« modeles ») : modèles IA refusés");
        }
        CanalCentral::ouvrir(config, &dossier, installateur, Box::new(PuitsJournal))
    });
    match ouvert {
        Ok(canal) => {
            tracing::warn!(
                "canal officiel configuré et état vérifié, mais la voie de relevé du Portail n'est pas branchée : \
                 aucun cycle lancé (période prévue {:?})",
                conf.periode()
            );
            Some(Arc::new(Mutex::new(canal)))
        }
        Err(e) => {
            tracing::error!("canal officiel désactivé (fail-closed) : {e}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::installateur::{REFUS_MODELES, REFUS_OS};
    use super::*;
    use crate::service_manager::gouvernance::{GouvernanceMarket, OperationMarket};
    use crate::service_manager::installer::{verifier_puis_installer, ContexteVerification};
    use crate::service_manager::launcher::ServiceProcesses;
    use crate::service_manager::registry::ServiceRegistryData;
    use crate::service_manager::version_canal;
    use miyucanal::banc::BancCanal;
    use miyucanal::{Installateur as _, Motif, QUARANTAINE_SECS};
    use miyuregistre::{Categorie, StockageMemoire};
    use miyusceau::{AncresDeConfiance, CleSignature, NiveauConfiance};
    use std::io::Write as _;

    const T0: u64 = 1_800_000_000;
    const H: u64 = 3_600;
    const REGISTRE: [u8; 32] = [0x0C; 32];

    fn cle_editeur_msp() -> CleSignature {
        CleSignature::depuis_graine(&[0x31; 32])
    }

    /// Éditeur `.msp` officiel (Souverain) : seul cas autorisé par la politique de base.
    fn ancres_msp() -> AncresDeConfiance {
        let mut a = AncresDeConfiance::new();
        a.ajouter(&cle_editeur_msp().cle_publique(), NiveauConfiance::Souverain).unwrap();
        a
    }

    fn gouvernance_base() -> GouvernanceMarket {
        GouvernanceMarket::depuis_fichier(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/strongfather/market.toml"))
    }

    fn gouvernance_restrictive() -> GouvernanceMarket {
        let p = bondingbrother::strongfather::PolicySet::from_toml_str(
            "version = 1\n[[policy]]\nid = \"seulement-desinstaller\"\ntype = \"permission\"\neffect = \"authorize\"\nrule = \"data.operation == 'desinstaller'\"\n",
        )
        .unwrap();
        GouvernanceMarket::depuis_porte(bondingbrother::PorteStrongFather::depuis_politiques(p).unwrap())
    }

    /// Faux paquet `.msp` signé du service `svc`.
    fn paquet(version: &str) -> Vec<u8> {
        let manifeste = format!(
            r#"{{"id":"svc","name":"Svc","version":"{version}","developer":"dev","description":"d","icon":"x","service_type":"interne_cog","source":"tiers","min_central_version":"0.1.0"}}"#
        );
        let mut zip = Vec::new();
        {
            let mut w = zip::ZipWriter::new(std::io::Cursor::new(&mut zip));
            let o = zip::write::SimpleFileOptions::default();
            w.start_file("service.manifest.json", o).unwrap();
            w.write_all(manifeste.as_bytes()).unwrap();
            w.start_file("bin/svc", o).unwrap();
            w.write_all(format!("binaire {version}").as_bytes()).unwrap();
            w.finish().unwrap();
        }
        miyumarket::signature::signer_paquet(&zip, "svc", version, None, &cle_editeur_msp()).unwrap()
    }

    fn racine(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("central-canal-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// Installateur sur une racine temporaire, `svc` 1.0.0 déjà installé.
    fn installateur(racine: &Path, gouvernance: GouvernanceMarket) -> (InstallateurCentral, Arc<Mutex<ServiceRegistryData>>) {
        let mut reg = ServiceRegistryData::default();
        let a = ancres_msp();
        let contexte = ContexteVerification { ancres: &a, maintenant_secs: mural(), dev_non_signe: false };
        verifier_puis_installer(&paquet("1.0.0"), "svc", Some("1.0.0"), contexte, &gouvernance_base(), OperationMarket::Installer, &mut reg, racine)
            .unwrap();
        let reg = Arc::new(Mutex::new(reg));
        let i = InstallateurCentral::new(reg.clone(), Arc::new(ServiceProcesses::new()), gouvernance, racine.to_path_buf(), None, ancres_msp());
        (i, reg)
    }

    fn version_svc(reg: &Arc<Mutex<ServiceRegistryData>>) -> String {
        reg.lock().unwrap().services["svc"].manifest.version.clone()
    }

    /// Relevés jusqu'après la quarantaine ; renvoie tous les événements.
    fn traverser(banc: &mut BancCanal, mut cycle: impl FnMut(&mut BancCanal, u64) -> Vec<Evenement>) -> Vec<Evenement> {
        let mut tous = Vec::new();
        for t in [T0, T0 + 24 * H, T0 + QUARANTAINE_SECS + 1, T0 + QUARANTAINE_SECS + 2] {
            banc.emettre(t);
            tous.extend(cycle(banc, t));
        }
        tous
    }

    fn compte(ev: &[Evenement], f: impl Fn(&Evenement) -> bool) -> usize {
        ev.iter().filter(|e| f(e)).count()
    }

    #[test]
    fn maj_cores_signee_installee_une_fois_par_le_chemin_gouverne() {
        let root = racine("ok");
        let (mut inst, reg) = installateur(&root, gouvernance_base());
        let v2 = version_canal("2.0.0").unwrap();
        assert_eq!(inst.version_installee(Categorie::Cores, "svc"), version_canal("1.0.0"));
        let mut banc = BancCanal::new(REGISTRE);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        banc.publier(Categorie::Cores, "svc", v2, &paquet("2.0.0"), T0);
        let ev = traverser(&mut banc, |b, t| releveur.cycle(&mut b.depot, &mut inst, &mut horloge, t).unwrap());
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { version, .. } if *version == v2)), 1);
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::InstallationRefusee { .. })), 0);
        assert_eq!(version_svc(&reg), "2.0.0");
        assert_eq!(std::fs::read(root.join("svc/bin/svc")).unwrap(), b"binaire 2.0.0");
        assert_eq!(inst.version_installee(Categorie::Cores, "svc"), Some(v2));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn refus_strongfather_rien_installe() {
        let root = racine("sf");
        let (mut inst, reg) = installateur(&root, gouvernance_restrictive());
        let v2 = version_canal("2.0.0").unwrap();
        let mut banc = BancCanal::new(REGISTRE);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        banc.publier(Categorie::Cores, "svc", v2, &paquet("2.0.0"), T0);
        let ev = traverser(&mut banc, |b, t| releveur.cycle(&mut b.depot, &mut inst, &mut horloge, t).unwrap());
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { .. })), 0);
        assert!(ev.iter().any(|e| matches!(e,
            Evenement::InstallationRefusee { motif: Motif::Installateur(m), .. } if m.contains("StrongFather"))));
        assert_eq!(version_svc(&reg), "1.0.0");
        assert_eq!(std::fs::read(root.join("svc/bin/svc")).unwrap(), b"binaire 1.0.0");
        // L'annonce reste en attente (décision réversible du propriétaire).
        assert_eq!(releveur.en_attente().len(), 1);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn os_et_modele_sans_installateur_refuses_version_differente_refusee() {
        let root = racine("os");
        let (mut inst, reg) = installateur(&root, gouvernance_base());
        let mut banc = BancCanal::new(REGISTRE);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        banc.publier(Categorie::Os, "noyau-linux", 7, b"correctif", T0);
        banc.publier(Categorie::ModeleIa, "qwen", 1, b"modele", T0);
        // Annonce v3 portant un paquet signé 2.0.0 : refus (version du paquet ≠ annoncée).
        banc.publier(Categorie::Cores, "svc", version_canal("3.0.0").unwrap(), &paquet("2.0.0"), T0);
        let puits = PuitsMemoire::default();
        let ev = traverser(&mut banc, |b, t| {
            let ev = releveur.cycle(&mut b.depot, &mut inst, &mut horloge, t).unwrap();
            for e in &ev {
                alertes::remonter(&puits, e);
            }
            ev
        });
        let refus = |texte: &'static str| {
            move |e: &Evenement| matches!(e, Evenement::InstallationRefusee { motif: Motif::Installateur(m), .. } if m.contains(texte))
        };
        assert!(compte(&ev, refus(REFUS_OS)) >= 1);
        assert!(compte(&ev, refus(REFUS_MODELES)) >= 1);
        assert!(compte(&ev, refus("différente de la version annoncée")) >= 1);
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { .. })), 0);
        assert_eq!(version_svc(&reg), "1.0.0");
        // Refus remontés à WorrySentinel.
        assert!(puits.signaux().iter().any(|s| s.niveau == ThreatLevel::Medium && s.motif.contains("noyau-linux")));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn etat_persiste_recharge_et_fail_closed() {
        let root = racine("persist");
        let dossier = root.join("etat");
        let mut banc = BancCanal::new(REGISTRE);
        let v2 = version_canal("2.0.0").unwrap();
        banc.publier(Categorie::Cores, "svc", v2, &paquet("2.0.0"), T0);
        // Premier démarrage : quarantaine ouverte à T0.
        let cfg = banc.config();
        let config = || cfg.clone();
        let nouveau = |cfg: ConfigCanal, puits: &PuitsMemoire| {
            let (inst, reg) = installateur(&root.join(uuid::Uuid::new_v4().to_string()), gouvernance_base());
            (CanalCentral::ouvrir(cfg, &dossier, inst, Box::new(puits.clone())), reg)
        };
        let puits = PuitsMemoire::default();
        let (canal, _) = nouveau(config(), &puits);
        let mut canal = canal.unwrap();
        banc.emettre(T0);
        let ev = canal.cycle(&mut banc.depot, T0).unwrap();
        assert!(ev.iter().any(|e| matches!(e, Evenement::EnQuarantaine { installable_a, .. } if *installable_a == T0 + QUARANTAINE_SECS)));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for f in [persistance::FICHIER_SECRET, FICHIER_ETAT, FICHIER_QUARANTAINE] {
                assert_eq!(std::fs::metadata(dossier.join(f)).unwrap().permissions().mode() & 0o777, 0o600, "{f}");
            }
        }
        drop(canal);

        // Reprise : le décompte reprend (installable à T0 + 48 h), aucun refus signalé.
        let puits = PuitsMemoire::default();
        let (canal, reg) = nouveau(config(), &puits);
        let mut canal = canal.unwrap();
        banc.emettre(T0 + 24 * H);
        let ev = canal.cycle(&mut banc.depot, T0 + 24 * H).unwrap();
        assert!(ev.iter().any(|e| matches!(e, Evenement::EnQuarantaine { installable_a, .. } if *installable_a == T0 + QUARANTAINE_SECS)));
        assert!(!ev.iter().any(|e| matches!(e, Evenement::QuarantaineAlteree { .. })));
        assert!(puits.signaux().iter().all(|s| s.niveau < ThreatLevel::Medium));
        banc.emettre(T0 + QUARANTAINE_SECS + 1);
        let ev = canal.cycle(&mut banc.depot, T0 + QUARANTAINE_SECS + 1).unwrap();
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { .. })), 1);
        assert_eq!(version_svc(&reg), "2.0.0");
        drop(canal);

        // Quarantaine altérée : refus signalé à WorrySentinel (High) à la reprise.
        banc.publier(Categorie::Cores, "svc", version_canal("3.0.0").unwrap(), &paquet("3.0.0"), T0 + QUARANTAINE_SECS + 2);
        let puits = PuitsMemoire::default();
        let (canal, _) = nouveau(config(), &puits);
        let mut canal = canal.unwrap();
        banc.emettre(T0 + QUARANTAINE_SECS + 2);
        canal.cycle(&mut banc.depot, T0 + QUARANTAINE_SECS + 2).unwrap();
        drop(canal);
        let fq = dossier.join(FICHIER_QUARANTAINE);
        let mut q = std::fs::read(&fq).unwrap();
        let n = q.len();
        q[n - 1] ^= 1;
        std::fs::write(&fq, q).unwrap();
        let puits = PuitsMemoire::default();
        let (canal, _) = nouveau(config(), &puits);
        let mut canal = canal.unwrap();
        banc.emettre(T0 + QUARANTAINE_SECS + 3);
        let ev = canal.cycle(&mut banc.depot, T0 + QUARANTAINE_SECS + 3).unwrap();
        assert!(ev.iter().any(|e| matches!(e, Evenement::QuarantaineAlteree { refuses: 1, .. })));
        assert!(puits.signaux().iter().any(|s| s.niveau == ThreatLevel::High));
        drop(canal);

        // État illisible : canal désactivé (aucune reprise au premier usage).
        std::fs::write(dossier.join(FICHIER_ETAT), b"MCCE\x01tronque").unwrap();
        let (canal, _) = nouveau(config(), &PuitsMemoire::default());
        assert!(canal.unwrap_err().contains("illisible"));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// Annonce scellée par `cles` (hors banc : signature unique, clé non épinglée).
    fn annonce_scellee(version: u64, artefact: &[u8], cles: &[&CleSignature]) -> Vec<u8> {
        use sha2::Digest as _;
        let publication = miyuregistre::Publication {
            categorie: Categorie::Cores,
            composant: "svc".into(),
            version,
            empreinte: sha2::Sha256::digest(artefact).into(),
            taille: artefact.len() as u64,
        };
        let entree = miyuregistre::EntreeRegistre { inscrite_a: T0, corps: miyuregistre::Corps::Publication(publication) };
        miyusceau::sceller_multi(&entree, cles).unwrap()
    }

    #[test]
    fn maj_cores_exige_deux_sur_deux_sur_les_cles_epinglees() {
        let root = racine("2sur2");
        let (mut inst, reg) = installateur(&root, gouvernance_base());
        let v2 = version_canal("2.0.0").unwrap();
        let v3 = version_canal("3.0.0").unwrap();
        let mut banc = BancCanal::new(REGISTRE);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        let p3 = paquet("3.0.0");
        banc.depot.deposer_artefact(p3.clone());
        let intrus = CleSignature::generer();
        let editeur = CleSignature::depuis_graine(&banc.editeur().graine());
        // 1. Éditeur épinglé seul (une signature) ; 2. éditeur + clé non
        // épinglée à la place d'Origin ; 3. paire 2 sur 2 entièrement étrangère.
        let seule = banc.inscrire(&annonce_scellee(v3, &p3, &[&editeur]));
        let mixte = banc.inscrire(&annonce_scellee(v3, &p3, &[&intrus, &editeur]));
        let etrangere = banc.inscrire(&annonce_scellee(v3, &p3, &[&CleSignature::generer(), &intrus]));
        // Annonce 2 sur 2 authentique d'une version inférieure : seule installée.
        banc.publier(Categorie::Cores, "svc", v2, &paquet("2.0.0"), T0);
        let puits = PuitsMemoire::default();
        let ev = traverser(&mut banc, |b, t| {
            let ev = releveur.cycle(&mut b.depot, &mut inst, &mut horloge, t).unwrap();
            for e in &ev {
                alertes::remonter(&puits, e);
            }
            ev
        });
        for index in [seule, mixte] {
            assert!(
                ev.iter().any(|e| matches!(e, Evenement::AnnonceRefusee { index: i, motif: Motif::Signature(_) } if *i == index)),
                "annonce {index} : refus de signature attendu"
            );
        }
        // Aucune signature épinglée : l'entrée n'est même pas une entrée du canal.
        assert!(ev.iter().any(|e| matches!(e, Evenement::EntreeIgnoree { index } if *index == etrangere)));
        assert!(!ev.iter().any(|e| matches!(e, Evenement::EnQuarantaine { version, .. } if *version == v3)));
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { version, .. } if *version == v3)), 0);
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { version, .. } if *version == v2)), 1);
        assert_eq!(version_svc(&reg), "2.0.0");
        assert_eq!(std::fs::read(root.join("svc/bin/svc")).unwrap(), b"binaire 2.0.0");
        assert_eq!(releveur.en_attente().len(), 0, "aucune annonce en attente");
        // Refus remontés à WorrySentinel.
        assert!(puits.signaux().iter().filter(|s| s.niveau >= ThreatLevel::Medium && s.motif.contains("refusée")).count() >= 2);
        let _ = std::fs::remove_dir_all(&root);
    }

    fn cle_modeles() -> CleSignature {
        CleSignature::depuis_graine(&[0x4b; 32])
    }

    fn config_modeles(dossier: &Path, ancres: &[[u8; 32]]) -> ConfigModeles {
        let ancres = ancres.iter().map(|k| miyumarket::signature::hex_encoder(k)).collect::<Vec<_>>();
        serde_json::from_value(serde_json::json!({ "dossier": dossier, "ancres": ancres })).unwrap()
    }

    /// Artefact `MMIA` : manifeste signé par `cle` + octets du modèle.
    fn artefact_modele(nom: &str, modele: &[u8], cle: &CleSignature) -> Vec<u8> {
        use sha2::Digest as _;
        let m = maia_modeles::ManifesteModele {
            nom_fichier: nom.into(),
            sha256: sha2::Sha256::digest(modele).into(),
            taille: modele.len() as u64,
            licence: "Apache-2.0".into(),
            date_coupure: "2025-06".into(),
            emis_le: mural(),
        };
        modeles::encoder_artefact(&miyusceau::sceller(&m, cle).unwrap(), modele)
    }

    #[test]
    fn modele_ia_installe_par_l_installateur_delegue_branche() {
        let root = racine("modeles");
        let dossier_modeles = root.join("models");
        let (inst, _) = installateur(&root, gouvernance_base());
        let conf = config_modeles(&dossier_modeles, &[cle_modeles().cle_publique()]);
        let mut inst = inst.avec_modeles(Box::new(conf.installateur().unwrap()));
        let mut banc = BancCanal::new(REGISTRE);
        let mut releveur = Releveur::new(banc.config(), StockageMemoire::new()).unwrap();
        let mut horloge = HorlogeConfiance::new();
        let v1 = b"GGUF\x03 modele officiel v1".to_vec();
        banc.publier(Categorie::ModeleIa, "qwen-mini", 1, &artefact_modele("qwen-mini.gguf", &v1, &cle_modeles()), T0);
        // v2 : manifeste signé par une ancre de modèles non épinglée (annonce 2 sur 2 valide).
        let etranger = CleSignature::depuis_graine(&[0x5c; 32]);
        let v2 = b"GGUF\x03 modele v2 etranger".to_vec();
        banc.publier(Categorie::ModeleIa, "autre", 2, &artefact_modele("autre.gguf", &v2, &etranger), T0);
        let ev = traverser(&mut banc, |b, t| releveur.cycle(&mut b.depot, &mut inst, &mut horloge, t).unwrap());
        assert_eq!(compte(&ev, |e| matches!(e, Evenement::Installee { composant, version: 1 } if composant == "qwen-mini")), 1);
        assert!(ev.iter().any(|e| matches!(e,
            Evenement::InstallationRefusee { composant, motif: Motif::Installateur(m), .. } if composant == "autre" && m.contains("manifeste"))));
        assert!(!ev.iter().any(|e| matches!(e, Evenement::InstallationRefusee { motif: Motif::Installateur(m), .. } if m.contains(REFUS_MODELES))));
        assert_eq!(std::fs::read(dossier_modeles.join("qwen-mini.gguf")).unwrap(), v1);
        assert!(dossier_modeles.join("qwen-mini.gguf.sceau").is_file());
        assert!(!dossier_modeles.join("autre.gguf").exists());
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "qwen-mini"), Some(1));
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "autre"), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn configuration_des_modeles_absente_ou_invalide() {
        let root = racine("conf-modeles");
        let k = cle_modeles().cle_publique();
        // Absente : aucun installateur branché, modèles refusés.
        let banc = BancCanal::new(REGISTRE);
        let (origin, journal, editeur) = banc.cles_publiques();
        let hx = |k: [u8; 32]| miyumarket::signature::hex_encoder(&k);
        let base = format!(
            r#""actif": true, "registre": "{}", "origin": "{}", "journal_origin": "{}", "editeur": "{}""#,
            hx(REGISTRE), hx(origin), hx(journal), hx(editeur)
        );
        let fichier = root.join(FICHIER_CONFIG);
        let charger = |extra: &str| {
            std::fs::write(&fichier, format!("{{{base}{extra}}}")).unwrap();
            ConfigCanalCentral::charger(&fichier)
        };
        let sans = charger("").unwrap().unwrap();
        let (inst, _) = installateur(&root.join("a"), gouvernance_base());
        let inst = sans.brancher_modeles(inst).unwrap();
        assert!(format!("{inst:?}").contains("modeles: false"));
        assert_eq!(inst.version_installee(Categorie::ModeleIa, "m"), None);
        // Valide : branché.
        let ok = charger(&format!(r#", "modeles": {{"dossier": "{}", "ancres": ["{}"]}}"#, root.join("m").display(), hx(k)));
        let (inst2, _) = installateur(&root.join("b"), gouvernance_base());
        assert!(format!("{:?}", ok.unwrap().unwrap().brancher_modeles(inst2).unwrap()).contains("modeles: true"));
        // Invalides : dossier relatif, aucune ancre, ancre mal formée ⇒ erreur (canal désactivé).
        for extra in [
            format!(r#", "modeles": {{"dossier": "models", "ancres": ["{}"]}}"#, hx(k)),
            format!(r#", "modeles": {{"dossier": "{}", "ancres": []}}"#, root.display()),
            format!(r#", "modeles": {{"dossier": "{}", "ancres": ["zz"]}}"#, root.display()),
        ] {
            let c = charger(&extra).unwrap().unwrap();
            let (i, _) = installateur(&root.join(uuid::Uuid::new_v4().to_string()), gouvernance_base());
            assert!(c.brancher_modeles(i).is_err(), "{extra}");
        }
        // Champ inconnu : configuration refusée.
        assert!(charger(&format!(r#", "modeles": {{"dossier": "{}", "ancres": ["{}"], "x": 1}}"#, root.display(), hx(k))).is_err());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn configuration_absente_ou_invalide_desactive() {
        let dossier = racine("conf");
        let fichier = dossier.join(FICHIER_CONFIG);
        assert!(ConfigCanalCentral::charger(&fichier).unwrap().is_none());
        std::fs::write(&fichier, r#"{"actif": true}"#).unwrap();
        assert!(ConfigCanalCentral::charger(&fichier).is_err());
        let banc = BancCanal::new(REGISTRE);
        let (origin, journal, editeur) = banc.cles_publiques();
        let hx = |k: [u8; 32]| miyumarket::signature::hex_encoder(&k);
        std::fs::write(
            &fichier,
            format!(r#"{{"actif": true, "registre": "{}", "origin": "{}", "journal_origin": "{}", "editeur": "{}", "periode_secs": 1}}"#, hx(REGISTRE), hx(origin), hx(journal), hx(editeur)),
        )
        .unwrap();
        let conf = ConfigCanalCentral::charger(&fichier).unwrap().unwrap();
        assert!(conf.config_canal().is_ok());
        assert_eq!(conf.periode(), Duration::from_secs(PERIODE_MIN_SECS));
        // Clés identiques : refus.
        std::fs::write(
            &fichier,
            format!(r#"{{"actif": true, "registre": "{}", "origin": "{}", "journal_origin": "{}", "editeur": "{}"}}"#, hx(REGISTRE), hx(origin), hx(origin), hx(editeur)),
        )
        .unwrap();
        assert!(ConfigCanalCentral::charger(&fichier).unwrap().unwrap().config_canal().is_err());
        let _ = std::fs::remove_dir_all(&dossier);
    }
}
