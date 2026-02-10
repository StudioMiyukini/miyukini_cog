# JayXpose - Integration Central

## 1. Objectif

Documenter l'integration de JayXpose dans Miyukini Central.

## 2. Etat actuel

JayXpose est integre en mode embarque dans Central.

Fichiers cle:
- `crates/miyukini-central/src/services/jayxpose_service.rs`
- `crates/jayxpose/src/app.rs`

## 3. Contrat d'integration UI

### 3.1 Service Central

- Service ID: `ServiceId::JayXpose`
- Titre: `JayXpose`
- Interface: `ServiceUi`

### 3.2 Bridge d'affichage

- Instanciation: `JayXposeApp::new_embedded()`
- Rendu: `self.app.show_in_ui(ui.ctx())`

## 4. Navigation

- Central gere le contexte onglet/service
- JayXpose gere sa navigation interne via `AppState`
- Router interne:
- EXP: XP-E01..XP-E12
- PUB: PUB-E01..PUB-E06

## 5. Donnees

- DB locale JayXpose: `jayxpose.db`
- Isolation logique des donnees par service
- Gouvernance KindMother Daughter

## 6. Dependencies runtime

- `egui/eframe`
- `rusqlite`
- `kindmother`
- `serde/serde_json`

## 7. Preconditions d'execution

- workspace Rust compilable
- acces ecriture au dossier runtime (DB sqlite)
- central capable de charger service JayXpose

## 8. Verifications

Commandes:
- `cargo check -p jayxpose`
- `cargo check -p miyukini-central`

Critere de succes:
- compilation ok des 2 crates
- service visible dans Central
- ecran dashboard JayXpose rendu sans panic

## 9. Interface avec Cores

- StrongFather: permissions/mandats
- KindMother: persistance locale
- BorderGuard: bornage securite
- WorrySentinel: audit sync

## 10. Interfaces inter-services documentees

### 10.1 JayXpose -> JayFestival

- profil expose
- catalogue expose
- vitrine exposee
- documents partages via mandats

### 10.2 JayXpose -> JayKonta

- documents comptables (RIB, attestations)
- metadata facturation exposant

### 10.3 JayXpose -> JayRDV

- catalogue services publie
- slots/booking references

### 10.4 JayXpose -> MiyukiniPoS

- mapping SKU
- stock qty
- audit sync

## 11. Risques integration

- derive UI si central change API `ServiceUi`
- divergence schema DB si migrations non synchronisees
- conflits de session multi-services

## 12. Plan d'extension

1. Partager contexte user unifie depuis Central vers JayXpose
2. Ajouter telemetrie de service centralisee
3. Ajouter contrats typed RPC inter-services
4. Ajouter tests d'integration end-to-end central <-> jayxpose
