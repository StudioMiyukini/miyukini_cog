//! @id     maia_modeles_lib
//! @do     exposer_provenance_modeles
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — provenance signée des modèles IA (manifeste, artefact MMIA, politique de chargement, installateur du canal officiel), partagée par maia-llm, Central et le fuzzing.

//! # Modèles IA de MAIA — provenance signée (EX-26, C-53, C-72)
//!
//! Crate partagée par **maia-llm** (chargement), **Central** (installateur
//! délégué du canal officiel) et le **fuzzing** : une seule source, aucune
//! inclusion par chemin.
//!
//! | Module          | Rôle                                                                 | Effet de bord |
//! |-----------------|----------------------------------------------------------------------|---------------|
//! | [`manifeste`]   | manifeste signé `ManifesteModele`, contrôles purs (signature, nom, taille, licence, empreinte), ancres | aucun |
//! | [`artefact`]    | artefact « modèle » `MMIA` du canal officiel (encodage, décodage strict) | aucun |
//! | [`fichier`]     | vérification d'un modèle **sur disque** : ouverture sans lien symbolique, droits, hachage du descripteur, TOCTOU | lecture |
//! | [`politique`]   | politique de chargement (ancres + dérogation **décidée par le binaire**) | journal |
//! | [`outillage`]   | scellement d'un manifeste, graine de développement (`cle-dev`, `sceller-modele`) | écriture |
//! | `canal` (feature `canal`) | installateur de la catégorie `ModeleIa` du canal officiel | écriture |
//!
//! # Dérogation « modèles non signés »
//!
//! La bibliothèque ne lit **jamais** `cfg!(debug_assertions)` : sa propre
//! compilation (profil des dépendances) ne dit rien du binaire qui la charge.
//! Le binaire décide ([`derogation_effective`], appelé avec son propre
//! `cfg!(debug_assertions)`) puis passe le booléen à [`PolitiqueModeles::new`].
//!
//! # Fonctionnalités
//!
//! - `canal` : [`canal::InstallateurModeles`] (dépend de `miyucanal`,
//!   `miyuregistre`, `serde`, `serde_json`) — activée par Central et maia-llm.
//!
//! Aucune dépendance au moteur d'inférence, au serveur HTTP ni au client HTTP.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

pub mod artefact;
#[cfg(feature = "canal")]
pub mod canal;
pub mod fichier;
pub mod manifeste;
pub mod outillage;
pub mod politique;

pub use artefact::{
    decoder_artefact, encoder_artefact, MAGIC_ARTEFACT, TAILLE_MAX_MANIFESTE, VERSION_ARTEFACT,
};
#[cfg(feature = "canal")]
pub use canal::{nom_fichier_du_composant, InstallateurModeles, EXTENSION_MODELE, FICHIER_INDEX};
pub use fichier::{
    chemin_manifeste, lire_manifeste, sha256_fichier, verifier_modele, verifier_modele_ouvert,
    verifier_modele_sous_nom, ModeleOuvert,
};
pub use manifeste::{
    ancres_depuis_hex, controler_empreinte, controler_manifeste, ErreurProvenance, ManifesteModele,
    EXTENSION_MANIFESTE, GENRE_MODELE, LICENCES_AUTORISEES,
};
pub use outillage::{executer_outil, generer_graine_dev, lire_graine, sceller_modele};
pub use politique::{
    derogation_demandee, derogation_effective, Autorisation, PolitiqueModeles, ENV_DEROGATION,
    OPTION_DEROGATION,
};

/// Secondes Unix courantes (0 si l'horloge précède l'époque).
pub(crate) fn maintenant_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

#[cfg(test)]
pub(crate) mod essai {
    use std::path::{Path, PathBuf};

    use miyusceau::{AncresDeConfiance, CleSignature};

    /// Dossier temporaire neuf.
    pub fn dossier(prefixe: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!(
            "maia-modeles-{prefixe}-{}-{}",
            std::process::id(),
            rand::random::<u64>()
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    pub fn cle() -> CleSignature {
        CleSignature::depuis_graine(&[9u8; 32])
    }

    pub fn ancres(c: &CleSignature) -> AncresDeConfiance {
        crate::ancres_depuis_hex(&[hex::encode(c.cle_publique())]).unwrap()
    }

    pub fn modele(d: &Path) -> PathBuf {
        let p = d.join("qwen-mini.gguf");
        std::fs::write(&p, b"GGUF\x03\x00\x00\x00 faux modele de test").unwrap();
        p
    }
}
