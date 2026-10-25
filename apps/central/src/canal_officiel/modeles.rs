//! @id     central_canal_officiel_modeles
//! @do     brancher_installateur_modeles_canal
//! @role   securite
//! @layer  app
//! @human  central — installateur des modèles IA du canal officiel, délégué à maia-llm (artefact MMIA), branché seulement s'il est configuré.

//! Installateur des modèles IA du canal officiel (catégorie `ModeleIa`,
//! A-007 : « modèle IA → installateur délégué (maia-llm), absent ⇒ refus »).
//!
//! # Délégation
//!
//! L'installateur est celui de **maia-llm** (`apps/maia-llm/src/canal.rs`,
//! artefact `MMIA`, provenance `X.gguf.sceau`) : maia-llm est un binaire sans
//! bibliothèque (et dépend de llama.cpp), sa source est donc compilée ici telle
//! quelle (`#[path]`), sans copie ni divergence. Les vérifications sont
//! celles de maia-llm : manifeste signé par une ancre des modèles, nom, taille,
//! licence, SHA-256, écriture atomique, contrôle de chargement rejoué, version
//! strictement supérieure (index `.canal-modeles.json`).
//!
//! # Configuration (champ `modeles` de `canal-officiel.json`, facultatif)
//!
//! ```json
//! "modeles": { "dossier": "/chemin/absolu/des/modeles", "ancres": ["<64 hex>"] }
//! ```
//!
//! - absent ⇒ **aucun** installateur de modèles : tout modèle est refusé
//!   ([`super::installateur::REFUS_MODELES`]) ;
//! - dossier relatif ou vide, aucune ancre, ancre invalide, champ inconnu ⇒
//!   configuration invalide ⇒ canal **désactivé** ;
//! - `dossier` = `models_dir` de maia-llm, `ancres` = ses `model_trust_anchors`
//!   (maia-llm revérifie au chargement : une ancre absente chez lui rend le
//!   modèle installé inchargeable, jamais l'inverse).

use std::path::PathBuf;

use serde::Deserialize;

#[path = "../../../maia-llm/src/canal.rs"]
#[allow(dead_code, unused_imports, clippy::all, clippy::pedantic)]
mod maia;

pub use maia::{encoder_artefact, InstallateurModeles, FICHIER_INDEX};

/// Configuration de l'installateur de modèles (voir module).
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigModeles {
    /// Dossier des modèles de maia-llm (chemin absolu).
    pub dossier: String,
    /// Ancres des manifestes de modèles (clés Ed25519, 64 hex), au moins une.
    pub ancres: Vec<String>,
}

impl ConfigModeles {
    /// Installateur délégué vérifié (dossier absolu, ancres décodées).
    pub fn installateur(&self) -> Result<InstallateurModeles, String> {
        let dossier = PathBuf::from(self.dossier.trim());
        if !dossier.is_absolute() {
            return Err(format!("modeles.dossier : chemin absolu attendu (« {} »)", self.dossier));
        }
        if self.ancres.is_empty() {
            return Err("modeles.ancres : au moins une ancre de modèles attendue".into());
        }
        let ancres = crate::provenance::ancres_depuis_hex(&self.ancres).map_err(|e| format!("modeles.ancres : {e}"))?;
        Ok(InstallateurModeles::new(dossier, ancres))
    }
}
