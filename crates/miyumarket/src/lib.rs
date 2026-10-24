//! @id     miyumarket_lib
//! @do     declarer_module_manifest
//! @role   orchestration
//! @layer  core
//! @human  miyumarket — module « lib ».

//! Miyukini Service Market — Protocole partagé entre Central (client) et Origin (serveur).
//!
//! Ce crate définit les types, manifestes et messages utilisés par le Market
//! pour installer, désinstaller et publier des services Miyukini.
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐         HTTPS/JSON          ┌──────────────┐
//! │   Central    │ ◄──────────────────────────► │    Origin     │
//! │ (MarketClient)│  GET /api/market/catalog   │ (MarketStore) │
//! │              │  GET /api/market/package/…  │              │
//! │              │  POST /api/market/publish   │   packages/  │
//! └─────────────┘                              └──────────────┘
//! ```
//!
//! # Signature obligatoire des paquets (CA-01)
//!
//! Central n'installe que des paquets signés (module [`signature`]) par un
//! éditeur dont la clé est épinglée (`EDITEURS_OFFICIELS` de Central ou
//! `market-editeurs.json` dans son dossier de configuration).
//!
//! Flux de développement :
//!
//! ```text
//! # 1. Clé d'éditeur de développement (graine hex, fichier 0600)
//! cargo run -p miyumarket --bin miyumarket-signer -- cle-nouvelle ~/.miyukini/editeur-dev.cle
//! #    → affiche la clé publique (64 hex)
//! # 2. Épingler la clé publique dans Central :
//! #    ~/.config/Miyukini-COG/market-editeurs.json
//! #    {"editeurs":[{"nom":"dev","cle_publique":"<64 hex>"}]}
//! # 3. Signer l'archive ZIP du service (id/version/binary_name = ceux du manifeste)
//! cargo run -p miyumarket --bin miyumarket-signer -- signer --cle ~/.miyukini/editeur-dev.cle \
//!     --id monservice --version 0.1.0 [--binaire monservice.exe] \
//!     --entree monservice.zip --sortie monservice-0.1.0.msp
//! # 4. Publier le .msp signé sur Origin (stockage opaque, checksum inchangé)
//! ```
//!
//! Dérogation (build debug uniquement) : `MIYUKINI_MARKET_DEV_NON_SIGNE=1`
//! permet d'installer et de lancer une archive non signée, journalisé en erreur,
//! sans jamais lui transmettre de secret.

pub mod manifest;
pub mod package;
pub mod protocol;
pub mod signature;
