//! @id     kindmother_heritage_lib
//! @do     importer_bases_sqlite_heritees
//! @role   donnee
//! @layer  toolkit
//! @human  Reprise unique, idempotente et non destructive des bases SQLite héritées
//!         des services COG dans KindMother (K-13), identifiants conservés.

//! # KindMother Héritage
//!
//! Les services COG passés à KindMother (`kindmother-only`, K-13) refusent de démarrer
//! tant que leur ancienne base SQLite non vide n'est pas importée
//! (`kindmother_client::heritage`). Ce crate fournit le moteur d'import commun :
//!
//! 1. ouverture de la base héritée en **lecture seule** (`SQLITE_OPEN_READ_ONLY`) ; base
//!    SQLCipher : même dérivation de clé que l'ancien chemin `db-encryption`
//!    (feature `sqlcipher`) ; journal `-journal`/`-wal` non vide : refus ;
//! 2. empreinte SHA-256 du fichier avant lecture ; lecture complète en mémoire
//!    (instantané cohérent), puis fermeture ;
//! 3. migrations KindMother du service appliquées ;
//! 4. table par table, dans l'ordre des clés étrangères du plan : upserts typés sur la
//!    clé primaire, en lots atomiques de ≤ `MAX_INTENTS_PER_BATCH` intentions ; les
//!    lignes déjà présentes et identiques ne sont pas réécrites (reprise sûre) ;
//! 5. vérification ligne à ligne après import (relecture KindMother), avec correction
//!    des lignes modifiées par les déclencheurs du schéma, puis contrôle des comptes ;
//! 6. empreinte SHA-256 recontrôlée : le fichier hérité doit être octet pour octet
//!    identique ;
//! 7. seulement alors, écriture du marqueur `<base>.importe-kindmother` (date, comptes,
//!    empreinte), qui lève la garde de bascule.
//!
//! Relancer après succès : no-op (marqueur présent). Relancer après un échec partiel :
//! reprise (les lots déjà appliqués sont reconnus, les autres rejoués).
//!
//! Chaque service décrit son [`PlanImport`] à côté de sa couche KindMother
//! (feature `import-heritage` des services) ; le binaire `kindmother-import` les expose.

mod import;
mod lecture;

#[cfg(feature = "fabrique-test")]
pub mod fabrique;

pub use import::{
    importer, ClefEtrangere, ErreurImport, PlanImport, PlanTable, RapportImport, RapportTable,
};
pub use kindmother_client::heritage::chemin_marqueur_import;
pub use lecture::{empreinte_sha256, BaseHeritee};
