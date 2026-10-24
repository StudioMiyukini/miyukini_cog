//! @id     miyudouane_instance
//! @do     gerer_instance_douane_par_frontiere
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — une instance Douane par frontière : chargement d'arrêté (anti-rejeu), règles, IPS.

//! # Instance Douane
//!
//! Une instance = une frontière (INV-Z9, EX-19). Elle n'accepte que des
//! arrêtés **vérifiés** (`ArreteVerifie`), de **sa** frontière, en vigueur, à
//! numéro **strictement croissant** (anti-rejeu / anti-retour arrière). Sans
//! arrêté valide, elle produit des règles de **fermeture** (tout rejeté).

use std::collections::BTreeSet;
use std::fmt;
use std::sync::mpsc::Sender;

use borderguard::{ArreteVerifie, Frontiere};

use crate::compilation::{compiler, ErreurCompilation, RegleCompilee};
use crate::ips::{EvenementDouane, MoteurIps};
use crate::nft;
use crate::plan::PlanReseau;

/// Refus de chargement d'un arrêté.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurDouane {
    /// L'arrêté vise une autre frontière.
    MauvaiseFrontiere {
        /// Frontière de l'instance.
        attendue: Frontiere,
        /// Frontière de l'arrêté.
        trouvee: Frontiere,
    },
    /// Numéro non strictement supérieur au dernier arrêté chargé.
    Rejeu {
        /// Dernier numéro chargé.
        dernier: u64,
        /// Numéro présenté.
        presente: u64,
    },
    /// Arrêté hors de sa fenêtre de validité.
    HorsVigueur,
    /// Frontière absente du plan.
    FrontiereHorsPlan,
    /// Compilation impossible.
    Compilation(ErreurCompilation),
    /// Audit des règles générées en échec (ne doit jamais arriver).
    Audit(Vec<String>),
}

impl fmt::Display for ErreurDouane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MauvaiseFrontiere { attendue, trouvee } => {
                write!(f, "arrêté de la frontière {trouvee} présenté à l'instance {attendue}")
            }
            Self::Rejeu { dernier, presente } => write!(f, "arrêté n° {presente} ≤ dernier chargé n° {dernier}"),
            Self::HorsVigueur => f.write_str("arrêté hors de sa fenêtre de validité"),
            Self::FrontiereHorsPlan => f.write_str("frontière absente du plan réseau"),
            Self::Compilation(e) => write!(f, "{e}"),
            Self::Audit(e) => write!(f, "audit des règles en échec : {}", e.join(" ; ")),
        }
    }
}

impl std::error::Error for ErreurDouane {}

/// Instance Douane d'une frontière.
#[derive(Debug)]
pub struct InstanceDouane {
    frontiere: Frontiere,
    plan: PlanReseau,
    arrete: Option<ArreteVerifie>,
    regles: Vec<RegleCompilee>,
    dernier_numero: u64,
}

impl InstanceDouane {
    /// Instance fermée (aucun arrêté). `dernier_numero` : dernier numéro déjà
    /// appliqué (persisté hors de l'instance), 0 au premier démarrage.
    pub fn nouvelle(frontiere: Frontiere, plan: PlanReseau, dernier_numero: u64) -> Result<Self, ErreurDouane> {
        if plan.frontiere(frontiere).is_none() {
            return Err(ErreurDouane::FrontiereHorsPlan);
        }
        Ok(Self { frontiere, plan, arrete: None, regles: Vec::new(), dernier_numero })
    }

    /// Frontière gardée.
    pub fn frontiere(&self) -> Frontiere {
        self.frontiere
    }

    /// Arrêté en cours.
    pub fn arrete(&self) -> Option<&ArreteVerifie> {
        self.arrete.as_ref()
    }

    /// Règles compilées en cours.
    pub fn regles(&self) -> &[RegleCompilee] {
        &self.regles
    }

    /// Dernier numéro chargé (à persister pour l'anti-rejeu).
    pub fn dernier_numero(&self) -> u64 {
        self.dernier_numero
    }

    /// Charge un arrêté vérifié : frontière, anti-rejeu, vigueur, compilation, audit.
    /// En cas d'erreur, l'état précédent est conservé.
    pub fn charger(&mut self, arrete: ArreteVerifie, maintenant_secs: u64) -> Result<(), ErreurDouane> {
        let a = arrete.arrete();
        if a.frontiere != self.frontiere {
            return Err(ErreurDouane::MauvaiseFrontiere { attendue: self.frontiere, trouvee: a.frontiere });
        }
        if a.numero <= self.dernier_numero {
            return Err(ErreurDouane::Rejeu { dernier: self.dernier_numero, presente: a.numero });
        }
        if !arrete.en_vigueur(maintenant_secs) {
            return Err(ErreurDouane::HorsVigueur);
        }
        let regles = compiler(&arrete, &self.plan).map_err(ErreurDouane::Compilation)?;
        let texte = nft::generer_frontiere(&arrete, &regles, &self.plan, &BTreeSet::new())
            .ok_or(ErreurDouane::FrontiereHorsPlan)?;
        let voies: BTreeSet<String> = a.voies.iter().map(|v| v.id.clone()).collect();
        let ecarts = nft::auditer(&texte, &voies);
        if !ecarts.is_empty() {
            return Err(ErreurDouane::Audit(ecarts));
        }
        self.dernier_numero = a.numero;
        self.regles = regles;
        self.arrete = Some(arrete);
        Ok(())
    }

    /// Règles nftables à appliquer à `maintenant_secs` : celles de l'arrêté
    /// s'il est en vigueur, sinon fermeture.
    pub fn ruleset(&self, maintenant_secs: u64, voies_coupees: &BTreeSet<String>) -> String {
        let def = self.plan.frontiere(self.frontiere);
        match (&self.arrete, def) {
            (Some(a), Some(_)) if a.en_vigueur(maintenant_secs) => {
                nft::generer_frontiere(a, &self.regles, &self.plan, voies_coupees).unwrap_or_default()
            }
            (Some(_), Some(d)) => nft::generer_fermeture(self.frontiere, d, "arrete hors vigueur"),
            (None, Some(d)) => nft::generer_fermeture(self.frontiere, d, "aucun arrete"),
            (_, None) => String::new(),
        }
    }

    /// Moteur IPS sur l'arrêté en cours (`None` sans arrêté : tout est fermé).
    pub fn moteur_ips(&self, evenements: Sender<EvenementDouane>) -> Option<MoteurIps> {
        self.arrete
            .as_ref()
            .map(|a| MoteurIps::nouveau(a, self.regles.clone(), self.plan.clone(), evenements))
    }
}
