//! @id     central_service_manager_gouvernance
//! @do     gouverner_actions_market
//! @role   securite
//! @layer  app
//! @human  central — porte StrongFather obligatoire du Market et du cycle de vie des services (K-11).

//! Gouvernance StrongFather du Market et des services (K-11, lot C).
//!
//! - Toute installation, mise à jour, désinstallation, tout lancement et toute remise de
//!   secrets passe par [`GouvernanceMarket::executer_action_market`] — point d'entrée unique
//!   du domaine — qui délègue à `bondingbrother::executer_si_autorise`
//! - Les politiques viennent de `<dossier>/market.toml` ; dossier = `COG_POLITIQUES_DIR`,
//!   défaut `config/strongfather`
//! - Fichier absent ou invalide ⇒ porte absente ⇒ toute action gouvernée est refusée
//!   (fail-closed), avec un journal explicite ; jamais « tout autoriser »
//! - Les contrôles existants (signature Sceau, ids réservés, provenance) restent en place :
//!   StrongFather est une décision obligatoire supplémentaire, pas un remplacement

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use bondingbrother::strongfather::{ActionType, CallContext, Intent, IntentData};
use bondingbrother::{executer_si_autorise, DecisionAutorisee, PorteStrongFather};
use miyusceau::NiveauConfiance;

use super::launcher::Provenance;

/// Variable d'environnement : dossier des politiques StrongFather.
pub const VAR_DOSSIER_POLITIQUES: &str = "COG_POLITIQUES_DIR";
/// Dossier par défaut des politiques (relatif au répertoire courant).
pub const DOSSIER_POLITIQUES_DEFAUT: &str = "config/strongfather";
/// Fichier de politiques du domaine Market.
pub const FICHIER_MARKET: &str = "market.toml";
/// Sujet des intentions d'installation / mise à jour / désinstallation.
pub const SUJET_MARKET: &str = "central.market";
/// Sujet des intentions de lancement et de remise de secrets.
pub const SUJET_SERVICES: &str = "central.services";
/// Appelant par défaut quand aucun principal authentifié n'est connu.
pub const APPELANT_LOCAL: &str = "local";

/// Dossier des politiques StrongFather (`COG_POLITIQUES_DIR` ou défaut).
pub fn dossier_politiques() -> PathBuf {
    std::env::var_os(VAR_DOSSIER_POLITIQUES)
        .filter(|v| !v.is_empty())
        .map_or_else(|| PathBuf::from(DOSSIER_POLITIQUES_DEFAUT), PathBuf::from)
}

/// Opération gouvernée du domaine Market / services.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationMarket {
    /// Installation d'un paquet `.msp`.
    Installer,
    /// Mise à jour d'un service installé.
    MettreAJour,
    /// Désinstallation d'un service.
    Desinstaller,
    /// Lancement du processus d'un service.
    Lancer,
    /// Remise de secrets du COG (jeton, phrase de chiffrement) à un service.
    InjecterSecrets,
}

impl OperationMarket {
    /// Nom canonique (`data.operation`).
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Installer => "installer",
            Self::MettreAJour => "mettre_a_jour",
            Self::Desinstaller => "desinstaller",
            Self::Lancer => "lancer",
            Self::InjecterSecrets => "injecter_secrets",
        }
    }

    /// Type d'action StrongFather.
    pub const fn action(self) -> ActionType {
        match self {
            Self::Installer | Self::Lancer => ActionType::Creation,
            Self::MettreAJour => ActionType::Modification,
            Self::Desinstaller => ActionType::Deletion,
            Self::InjecterSecrets => ActionType::Read,
        }
    }

    /// Sujet de l'intention (opérateur qui agit).
    pub const fn sujet(self) -> &'static str {
        match self {
            Self::Installer | Self::MettreAJour | Self::Desinstaller => SUJET_MARKET,
            Self::Lancer | Self::InjecterSecrets => SUJET_SERVICES,
        }
    }
}

/// Provenance d'un paquet telle que vue par StrongFather (`data.provenance`).
pub const fn nom_provenance(p: Option<Provenance>) -> &'static str {
    match p {
        Some(Provenance::SigneeOfficielle) => "signee_officielle",
        Some(Provenance::SigneeTierce) => "signee_tierce",
        Some(Provenance::NonSigneeDev) => "non_signee_dev",
        None => "non_signee",
    }
}

/// Niveau de confiance du signataire (`data.niveau_confiance`).
pub const fn nom_niveau(n: Option<NiveauConfiance>) -> &'static str {
    match n {
        Some(NiveauConfiance::Souverain) => "souverain",
        Some(NiveauConfiance::Reconnu) => "reconnu",
        None => "aucun",
    }
}

/// Action Market / services décrite pour StrongFather.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMarket {
    /// Opération demandée.
    pub operation: OperationMarket,
    /// Identifiant du paquet / service visé.
    pub paquet: String,
    /// Version visée (ou installée) ; vide si inconnue.
    pub version: String,
    /// Provenance vérifiée (`None` = aucune signature valide).
    pub provenance: Option<Provenance>,
    /// Niveau de confiance du signataire (`None` = aucun signataire).
    pub niveau: Option<NiveauConfiance>,
    /// `kid` du signataire (hex abrégé) ; `None` = aucun.
    pub editeur: Option<String>,
    /// Principal qui agit (profil connecté) ; `None` ⇒ `local`.
    pub appelant: Option<String>,
}

impl ActionMarket {
    /// Action sans signataire connu (désinstallation, contrôle préalable).
    pub fn nouvelle(operation: OperationMarket, paquet: &str, version: &str) -> Self {
        Self {
            operation,
            paquet: paquet.to_string(),
            version: version.to_string(),
            provenance: None,
            niveau: None,
            editeur: None,
            appelant: None,
        }
    }

    /// Renseigne la provenance vérifiée et le signataire.
    #[must_use]
    pub fn avec_provenance(
        mut self,
        provenance: Provenance,
        niveau: Option<NiveauConfiance>,
        editeur: Option<&[u8]>,
    ) -> Self {
        self.provenance = Some(provenance);
        self.niveau = niveau;
        self.editeur = editeur.map(|k| miyumarket::signature::hex_encoder(&k[..k.len().min(8)]));
        self
    }

    /// Renseigne le principal qui agit (ignoré s'il est vide).
    #[must_use]
    pub fn par(mut self, appelant: &str) -> Self {
        let a = appelant.trim();
        self.appelant = (!a.is_empty()).then(|| a.to_string());
        self
    }

    /// Intention StrongFather correspondante (forme stable, voir `config/strongfather/market.toml`).
    pub fn intention(&self) -> Intent {
        let mut data = HashMap::new();
        data.insert("operation".to_string(), self.operation.nom().to_string());
        data.insert("paquet".to_string(), self.paquet.clone());
        data.insert("version".to_string(), self.version.clone());
        data.insert("provenance".to_string(), nom_provenance(self.provenance).to_string());
        data.insert("niveau_confiance".to_string(), nom_niveau(self.niveau).to_string());
        data.insert(
            "editeur".to_string(),
            self.editeur.clone().unwrap_or_else(|| "aucun".to_string()),
        );
        data.insert(
            "id_reserve".to_string(),
            super::confiance::id_reserve_officiel(&self.paquet).to_string(),
        );
        let mut meta = HashMap::new();
        meta.insert(
            "build".to_string(),
            if cfg!(debug_assertions) { "debug" } else { "release" }.to_string(),
        );
        Intent::new(
            uuid::Uuid::new_v4().to_string(),
            self.operation.action(),
            self.operation.sujet().to_string(),
            CallContext {
                caller_identity: self
                    .appelant
                    .clone()
                    .unwrap_or_else(|| APPELANT_LOCAL.to_string()),
                origin: "central".to_string(),
                instance: "cog".to_string(),
            },
            IntentData { content: data },
        )
        .with_metadata(meta)
    }
}

/// Porte StrongFather du domaine Market (absente ⇒ tout refuser).
#[derive(Clone)]
pub struct GouvernanceMarket {
    porte: Result<PorteStrongFather, String>,
}

impl std::fmt::Debug for GouvernanceMarket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GouvernanceMarket")
            .field("porte_chargee", &self.porte.is_ok())
            .finish()
    }
}

impl GouvernanceMarket {
    /// Charge `market.toml` depuis le dossier des politiques (journalise l'échec).
    pub fn charger() -> Self {
        Self::depuis_fichier(&dossier_politiques().join(FICHIER_MARKET))
    }

    /// Charge un fichier de politiques explicite ; illisible ou invalide ⇒ porte absente.
    pub fn depuis_fichier(chemin: &Path) -> Self {
        match PorteStrongFather::depuis_fichier(chemin) {
            Ok(porte) => {
                tracing::info!("StrongFather Market : politiques chargées depuis {}", chemin.display());
                Self { porte: Ok(porte) }
            }
            Err(e) => {
                let motif = format!("politiques Market indisponibles ({}) : {e}", chemin.display());
                tracing::error!(
                    "StrongFather Market : {motif} — installation, mise à jour, désinstallation, \
                     lancement et secrets REFUSÉS (fail-closed)"
                );
                Self { porte: Err(motif) }
            }
        }
    }

    /// Porte déjà construite (tests, injection).
    pub fn depuis_porte(porte: PorteStrongFather) -> Self {
        Self { porte: Ok(porte) }
    }

    /// Vrai si une porte valide est chargée.
    pub fn est_chargee(&self) -> bool {
        self.porte.is_ok()
    }

    /// Point d'entrée UNIQUE du domaine : exécute `f` seulement si StrongFather autorise
    /// l'action ; sinon (refus, porte absente) renvoie le motif sans appeler `f`.
    pub fn executer_action_market<T>(
        &self,
        action: &ActionMarket,
        f: impl FnOnce(&DecisionAutorisee) -> T,
    ) -> Result<T, String> {
        let op = action.operation.nom();
        let porte = self.porte.as_ref().map_err(|motif| {
            tracing::warn!("Market '{op}' de '{}' refusé : {motif}", action.paquet);
            format!("Action '{op}' refusée : {motif}")
        })?;
        let intent = action.intention();
        executer_si_autorise(porte, &intent, |jeton| {
            tracing::info!(
                "StrongFather autorise '{op}' de '{}' (règle {}, empreinte {})",
                action.paquet,
                jeton.regle(),
                jeton.empreinte()
            );
            f(jeton)
        })
        .map_err(|refus| {
            tracing::warn!("StrongFather refuse '{op}' de '{}' : {refus}", action.paquet);
            format!("Action '{op}' de '{}' refusée par StrongFather : {refus}", action.paquet)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bondingbrother::Gate;

    /// Politique de base livrée avec le dépôt.
    fn base() -> GouvernanceMarket {
        let chemin = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../config/strongfather")
            .join(FICHIER_MARKET);
        let g = GouvernanceMarket::depuis_fichier(&chemin);
        assert!(g.est_chargee(), "market.toml doit se charger");
        g
    }

    fn autorise(g: &GouvernanceMarket, a: &ActionMarket) -> bool {
        g.executer_action_market(a, |_| ()).is_ok()
    }

    fn signe(op: OperationMarket, id: &str, p: Provenance) -> ActionMarket {
        let niveau = match p {
            Provenance::SigneeOfficielle => Some(NiveauConfiance::Souverain),
            Provenance::SigneeTierce => Some(NiveauConfiance::Reconnu),
            Provenance::NonSigneeDev => None,
        };
        ActionMarket::nouvelle(op, id, "1.0.0").avec_provenance(p, niveau, Some(&[1u8; 32]))
    }

    #[test]
    fn politique_de_base_autorise_les_cas_legitimes() {
        let g = base();
        for op in [OperationMarket::Installer, OperationMarket::MettreAJour] {
            assert!(autorise(&g, &signe(op, "svc", Provenance::SigneeOfficielle)));
            // Éditeurs épinglés localement : refusés (officiels seulement, 2026-10-24).
            assert!(!autorise(&g, &signe(op, "svc", Provenance::SigneeTierce)));
            assert!(autorise(&g, &signe(op, "miyucloud", Provenance::SigneeOfficielle)));
        }
        assert!(autorise(&g, &signe(OperationMarket::Lancer, "svc", Provenance::SigneeOfficielle)));
        assert!(!autorise(&g, &signe(OperationMarket::Lancer, "svc", Provenance::SigneeTierce)));
        assert!(autorise(
            &g,
            &signe(OperationMarket::InjecterSecrets, "miyucloud", Provenance::SigneeOfficielle)
        ));
        assert!(autorise(&g, &ActionMarket::nouvelle(OperationMarket::Desinstaller, "svc", "")));
    }

    /// Même si un contrôle amont était contourné, StrongFather refuse seul.
    #[test]
    fn non_signe_ou_non_officiel_refuse_par_strongfather() {
        let g = base();
        // Aucune signature.
        for op in [OperationMarket::Installer, OperationMarket::MettreAJour, OperationMarket::Lancer] {
            assert!(!autorise(&g, &ActionMarket::nouvelle(op, "svc", "1.0.0")));
        }
        // Identifiant réservé signé par un éditeur local.
        assert!(!autorise(&g, &signe(OperationMarket::Installer, "miyucloud", Provenance::SigneeTierce)));
        assert!(!autorise(&g, &signe(OperationMarket::Lancer, "miyucloud", Provenance::SigneeTierce)));
        // Secrets : jamais hors éditeur officiel.
        for p in [Provenance::SigneeTierce, Provenance::NonSigneeDev] {
            assert!(!autorise(&g, &signe(OperationMarket::InjecterSecrets, "miyucloud", p)));
        }
        assert!(!autorise(
            &g,
            &ActionMarket::nouvelle(OperationMarket::InjecterSecrets, "miyucloud", "1.0.0")
        ));
        // Provenance officielle annoncée mais niveau incohérent : refus.
        let incoherent = ActionMarket::nouvelle(OperationMarket::Installer, "svc", "1.0.0")
            .avec_provenance(Provenance::SigneeOfficielle, Some(NiveauConfiance::Reconnu), None);
        assert!(!autorise(&g, &incoherent));
    }

    #[test]
    fn derogation_dev_suit_le_type_de_build() {
        let g = base();
        let a = signe(OperationMarket::Installer, "svc", Provenance::NonSigneeDev);
        assert_eq!(autorise(&g, &a), cfg!(debug_assertions));
    }

    #[test]
    fn fichier_absent_ou_invalide_refuse_tout() {
        let g = GouvernanceMarket::depuis_fichier(Path::new("/inexistant/market.toml"));
        assert!(!g.est_chargee());
        let a = signe(OperationMarket::Installer, "svc", Provenance::SigneeOfficielle);
        let mut appels = 0;
        assert!(g.executer_action_market(&a, |_| appels += 1).is_err());
        assert_eq!(appels, 0);

        let dir = std::env::temp_dir().join(format!("miyu-sf-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join(FICHIER_MARKET);
        std::fs::write(&f, "version = 1\n").unwrap(); // aucune politique
        assert!(!GouvernanceMarket::depuis_fichier(&f).est_chargee());
        std::fs::write(&f, "n'importe quoi").unwrap();
        assert!(!GouvernanceMarket::depuis_fichier(&f).est_chargee());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn intention_porte_la_cible_et_le_principal() {
        let a = signe(OperationMarket::Lancer, "jaykoa", Provenance::SigneeOfficielle).par("profil-1");
        let i = a.intention();
        assert_eq!(i.subject, SUJET_SERVICES);
        assert_eq!(i.action_type, ActionType::Creation);
        assert_eq!(i.call_context.caller_identity, "profil-1");
        assert_eq!(i.data.content["paquet"], "jaykoa");
        assert_eq!(i.data.content["provenance"], "signee_officielle");
        assert_eq!(i.data.content["editeur"], "0101010101010101");
        let local = ActionMarket::nouvelle(OperationMarket::Desinstaller, "x", "").par("  ");
        assert_eq!(local.intention().call_context.caller_identity, APPELANT_LOCAL);
        // La porte de base produit un jeton lié à l'intention.
        let g = base();
        let porte = g.porte.as_ref().unwrap();
        assert!(porte.autoriser(&i).unwrap().couvre(&i));
    }
}
