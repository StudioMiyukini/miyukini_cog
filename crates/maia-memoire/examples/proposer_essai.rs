//! @id     maia_memoire_exemple_proposer_essai
//! @do     proposer_candidat_essai
//! @role   test
//! @layer  8
//! @human  maia-memoire — outil d'essai des scénarios LIVE (deploy/cog-vm/scenarios.sh) : dépose un candidat comme le ferait le pipeline MAIA, sans modèle.

//! Outil d'essai : propose un candidat MAIA par l'API réelle
//! `MemoireMaia::proposer` (même chemin que le pipeline LLM), avec le justificatif
//! de l'opérateur `maia-<utilisateur>` fourni par KindMother.
//!
//! Usage : proposer_essai UTILISATEUR ADRESSE_KINDMOTHER DOSSIER_CREDENTIALS BRANCHE TEXTE
use std::path::Path;
use std::sync::Arc;

use maia_memoire::{Branche, BorneOctets, EntrepotKindMother, MemoireMaia, Proposition, Provenance, TypeUnite};

#[tokio::main]
async fn main() -> Result<(), String> {
    let a: Vec<String> = std::env::args().collect();
    if a.len() != 6 {
        return Err("usage : cog-proposer UTILISATEUR ADRESSE DOSSIER BRANCHE TEXTE".into());
    }
    let (utilisateur, adresse, dossier, branche, texte) = (&a[1], &a[2], &a[3], &a[4], &a[5]);
    let entrepot = EntrepotKindMother::connecter(adresse, utilisateur, Some(Path::new(dossier)))
        .await
        .map_err(|e| format!("connexion KindMother : {e:?}"))?;
    let memoire = MemoireMaia::ouvrir(entrepot, Arc::new(BorneOctets))
        .await
        .map_err(|e| format!("ouverture de la mémoire : {e:?}"))?;
    let id = memoire
        .proposer(Proposition {
            cible: None,
            branche: Branche::parse(branche).map_err(|e| format!("{e:?}"))?,
            type_unite: TypeUnite::Fait,
            texte: texte.clone(),
            importance: 5,
            confiance: 5,
            provenance: Provenance::Deduit,
            liens: vec![],
            sources: vec![],
        })
        .await
        .map_err(|e| format!("proposition : {e:?}"))?;
    println!("{id}");
    Ok(())
}
