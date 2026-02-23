# Miyukini COG — Plan de Développement App Android Terminal

## Contexte

Ce plan structure le développement de l'**App Android Terminal** (COG TERMINAL, `os_type` ANDROID), client mobile embarqué enfant d'un COG STABLE. La **Phase 1** est dédiée à une **documentation exhaustive** couvrant l'intégralité des aspects techniques, fonctionnels et architecturaux avant toute implémentation. La stack retenue est **Dioxus** pour une compatibilité maximale avec Miyukini Central (Rust, patterns UI partagés).

**Références :**

- [Étude préalable](./Miyukini%20COG%20-%20Etude%20App%20Android%20Terminal.md)
- [MWS - Passeport et Visa](../miyukini-webway-system/verification/MWS%20-%20Passeport%20et%20Visa.md)
- [Dioxus Mobile](https://dioxus.dev/learn/0.6/guides/mobile/)

---

## Portée / Scope

- Plan de développement en **phases séquentielles**
- **Phase 1** : Documentation exhaustive (large, riche, explicite)
- Phases suivantes : implémentation progressive
- Stack : **Dioxus** (mobile WebView/WGPU) + Rust (logique MWS, persistance)

---

## Vue d'ensemble des phases

| Phase | Nom | Objectif principal |
|-------|-----|-------------------|
| **1** | Documentation exhaustive | Produire toute la doc technique/fonctionnelle avant le code |
| **2** | Setup environnement & squelette | Toolchain Dioxus Android, projet initial, CI |
| **3** | POC liaison & MWS | Connexion Relay, Passeport TERMINAL, Permis |
| **4** | MVP fonctionnel | Écrans principaux, sync, offline basique |
| **5** | Intégration Central | Gestion Terminaux depuis Central, QR/lien |
| **6** | Production | Notifications, signature, doc utilisateur |

---

# Phase 1 — Documentation exhaustive

**Objectif :** Produire une documentation large, riche, explicite et englobante, couvrant un maximum de points techniques. Aucun code significatif avant validation de cette phase.

## 1.1 Livrables documentaires (liste exhaustive)

Chaque document doit respecter la nomenclature `docs/` : Contexte, Portée/Scope, contenu orienté action ou décision.

### A. Documents fondateurs (docs/services/MiyukiniTerminal/)

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 1 | `MiyukiniTerminal - Document Fondateur.md` | Vision, objectifs, non-objectifs, positionnement COG TERMINAL, limites 5 terminaux/STABLE, relation parent-enfant, conformité 8 Lois d'Autonomie | ☑ |
| 2 | `MiyukiniTerminal - Index Documentation.md` | Table des matières, arborescence docs Terminal, liens croisés, statut de chaque doc | ☑ |

### B. Architecture & technique

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 3 | `MiyukiniTerminal - Architecture Technique.md` | Schémas (Mermaid) : couches (UI Dioxus, Services, MWS, Stockage), flux données, interactions composants, décisions Rust vs Kotlin (confirmé Rust/Dioxus), réutilisation crates existants | ☑ |
| 4 | `MiyukiniTerminal - Stack Dioxus Mobile Spec.md` | Détail complet : version Dioxus (0.6+), features `mobile`, rendu WebView vs WGPU, APIs `dioxus-mobile`, limitations connues, cibles (aarch64, armv7, x86, x86_64), chemins de build, `dx serve` / `dx bundle` | ☑ |
| 5 | `MiyukiniTerminal - Environnement Dev Android.md` | Guide pas-à-pas : rustup targets Android, Android Studio, SDK/NDK/CMAKE, variables JAVA_HOME/ANDROID_HOME/NDK_HOME/PATH (Windows + Linux), émulateurs, device physique, dépannage fréquent | ☑ |
| 6 | `MiyukiniTerminal - Alignement Central Dioxus.md` | Mapping patterns Central → Terminal : AppContext/use_app_state, theme (ThemePalette, styles), navigation (MainTab), composants (Props, Signal, RSX), différences mobile (écran tactile, taille, gestures), réutilisation theme Gaming | ☑ |

### C. MWS & protocoles

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 7 | `MiyukiniTerminal - Spec MWS Passeport Permis.md` | Passeport TERMINAL : champs obligatoires (cog_id, cog_type=0x05, os_type=ANDROID, parent_cog_id, core_version, service_list, environment_health), format binaire, séquence présentation Relay, codes erreur, Permis de circulation, durée, renouvellement | ☑ |
| 8 | `MiyukiniTerminal - Spec Protocole Relay Terminal.md` | Séquence REGISTER avec parent_cog_id, trames (version, type, flags, payload), messages 0x01–0x0B, 0x10–0x13, heartbeats, gestion TLS, timeouts, reconnexion, cas REFUS parent invalide | ☑ |
| 9 | `MiyukiniTerminal - Spec MiyuWebwayParticipant Adapt.md` | Parties réutilisables de `miyuwebway_participant` : declaration, transport, discovery ; parties à adapter : pas de Tracker serveur (client seul), pas de port 21000 en écoute ; API minimale exposée au Terminal ; dépendances `no_std` si applicable | ☑ |
| 9b | `MiyukiniTerminal - Spec MSCM MIP Conformite.md` | Balisage MSCM obligatoire, structure MIP, Phase B adaptée Terminal (blocs code local), inventaire blocs, pipeline génération index | ☑ |

### D. Liaison Central ↔ Terminal

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 10 | `MiyukiniTerminal - Spec Flux Liaison Parent.md` | Flux complet : Central génère token, QR/lien, Terminal scanne/saisit, validation token, création cog_id Terminal, stockage parent_cog_id, schéma séquence Mermaid, contenu QR (JSON/URL), durée vie token, sécurité (chiffrement, expiration) | ☑ |
| 11 | `MiyukiniTerminal - Spec Central Gestion Terminaux.md` | Écran Central "Gérer mes Terminaux" : liste terminaux liés, bouton "Ajouter Terminal", génération token/QR, révoquer terminal, limite 5, UX wireframes texte, intégration BondingBrother, stockage côté STABLE | ☑ |
| 12 | `MiyukiniTerminal - Spec Token Liaison Securite.md` | Format token (JWT ou custom), payload (parent_cog_id, user_id, expiration, nonce), signature, stockage sécurisé Android (EncryptedSharedPreferences/Keystore), protection contre rejeu | ☑ |

### E. Stockage & persistance

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 13 | `MiyukiniTerminal - Spec Stockage Local.md` | Choix : SQLite/libSQL (KindMother) vs rusqlite seul ; schéma tables : identity (cog_id, parent_cog_id), cache_services, queue_actions, preferences ; migrations ; chiffrement optionnel (db-encryption) ; chemins Android (getFilesDir, databases) | ☑ |
| 14 | `MiyukiniTerminal - Spec Queue Actions Offline.md` | Structure queue : type action, payload, timestamp, statut (pending/sent/failed), retry policy, conflits, politique merge (dernier écrit, merge manuel), rejeu à la reconnexion, limite taille queue | ☑ |

### F. Synchronisation

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 15 | `MiyukiniTerminal - Spec Synchronisation Parent.md` | Protocole sync : init, incrémental, full refresh ; données synchronisées (services, préférences, cache JayKonta/JayKoa) ; fréquence (batch, adaptatif batterie) ; compression ; détection conflits, résolution | ☑ |
| 16 | `MiyukiniTerminal - Spec Mode Offline.md` | États : online, offline, degrading ; comportement par état ; lecture cache ; écriture queue ; indicateur UI ; reconnexion automatique ; stratégie retry (exponential backoff) | ☑ |

### G. UI / UX

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 17 | `MiyukiniTerminal - Spec Ecrans et Navigation.md` | Liste écrans : Liaison, Salon (services), Service (détail), Paramètres, Profil ; navigation (bottom nav, drawer) ; flux premier lancement vs utilisateur lié ; wireframes texte/Mermaid ; transitions | ☑ |
| 18 | `MiyukiniTerminal - Spec Design System Mobile.md` | Composants : boutons, cartes, listes, champs, modals ; thème (palette Gaming héritée, adaptée mobile) ; tailles touch (44pt min), grille responsive ; typo ; icônes ; dark/light | ☑ |
| 19 | `MiyukiniTerminal - Spec Parcours Utilisateur.md` | Parcours : liaison, première sync, consultation services, action différée, erreur réseau ; edge cases : token expiré, parent déconnecté, limite 5 atteinte ; messages d'erreur | ☑ |

### H. Services & fonctionnalités

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 20 | `MiyukiniTerminal - Spec Services Consultatifs.md` | Quels services exposés en vue consultative : JayKonta (soldes, mouvements récents), JayKoa (agenda, événements) ; format données (JSON, protobuf?) ; pagination ; limites (derniers 30 jours, etc.) | ☑ |
| 21 | `MiyukiniTerminal - Spec Actions Simples.md` | Actions déléguées au parent : saisie dépense JayKonta, création événement JayKoa ; flow : Terminal → queue → sync → parent exécute ; format requête ; confirmation | ☑ |
| 22 | `MiyukiniTerminal - Spec Notifications.md` | Types : rappels JayKoa, seuils JayKonta ; canal : push (FCM?) vs pull (polling) vs local (alarmes) ; permissions Android ; design notifications (style, actions) | ☑ |

### I. Sécurité & conformité

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 23 | `MiyukiniTerminal - Spec Securite.md` | Stockage sensible (Keystore, EncryptedSharedPreferences) ; TLS obligatoire ; validation certificats ; pas de log de tokens ; verrouillage app (PIN/biométrie) ; Android permissions (INTERNET, etc.) | ☑ |
| 24 | `MiyukiniTerminal - Spec Conformite Cores.md` | Mapping Cores : StrongFather (autorisation actions), KindMother (persistance), MasterButler (capacités), BorderGuard (frontières), WorrySentinel (niveaux sécurité), TAMR (conflits) ; contrats respectés | ☑ |

### J. Déploiement & CI

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 25 | `MiyukiniTerminal - Spec Build et Signature.md` | `dx bundle` Android ; configuration build.gradle (ou équivalent Dioxus) ; signing (debug/release) ; versions (versionCode, versionName) ; ProGuard/R8 ; APK vs AAB | ☑ |
| 26 | `MiyukiniTerminal - Spec CI CD.md` | Pipeline : build Android (GitHub Actions ou équivalent), tests, lint, artifact APK ; étapes ; variables secrètes ; déclencheurs | ☑ |

### K. Tests & qualité

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 27 | `MiyukiniTerminal - Spec Strategy Tests.md` | Tests unitaires (Rust), tests intégration (Relay mock), tests E2E (émulateur), couverture cible ; outils (cargo test, dx) | ☑ |

### L. Référence & glossaire

| # | Document | Contenu détaillé | Statut |
|---|----------|-----------------|--------|
| 28 | `MiyukiniTerminal - Reference Technique Complete.md` | Référence condensée : tous les champs, formats, codes, constantes ; tables ; index rapide ; liens vers docs détaillées | ☑ |
| 29 | `reference/_index.md` | Index des documents MiyukiniTerminal ; liens vers MWS, Central, Étude | ☑ |

---

## 1.2 Critères de passage Phase 1

- [x] Les **29 documents** (ou leur équivalent regroupé) sont rédigés
- [x] Chaque document a **Contexte** et **Portée/Scope**
- [x] Les schémas techniques (Mermaid, ASCII) sont présents où pertinent
- [ ] Revue par un pair ou validation architecturale
- [x] Index `_index.md` à jour avec tous les liens

---

## 1.3 Ordre de rédaction suggéré

1. Document Fondateur + Index
2. Architecture Technique + Stack Dioxus + Env Dev
3. Spec MWS (Passeport, Protocole, MiyuWebwayAdapt)
4. Spec Liaison (Flux, Central, Token)
5. Spec Stockage + Queue Offline
6. Spec Sync + Mode Offline
7. Spec UI (Écrans, Design, Parcours)
8. Spec Services + Actions + Notifications
9. Spec Sécurité + Conformité
10. Spec Build + CI + Tests
11. Référence Technique + Index final

---

# Phase 2 — Setup environnement & squelette

**Objectif :** Environnement de dev opérationnel, projet Dioxus Android créé, structure de base.

## 2.1 Livrables

| # | Livrable | Description |
|---|----------|-------------|
| 1 | Toolchain Android | rustup targets, Android Studio, SDK, NDK, variables configurées |
| 2 | Projet `apps/terminal` | `dx new` ou structure manuelle, Cargo.toml, deps (dioxus mobile, tokio, etc.) |
| 3 | Build Android OK | `dx serve` ou `cargo build --target aarch64-linux-android` fonctionne |
| 4 | Écran blanc Dioxus | App minimale s'exécute sur émulateur |
| 5 | CI skeleton | Workflow GitHub Actions (ou équivalent) build Android |

## 2.2 Critères de passage

- [ ] Build Android réussi
- [ ] App affichée sur émulateur
- [ ] Documentation Env Dev validée par pratique

---

# Phase 3 — POC liaison & MWS

**Objectif :** Connexion au Relay, Passeport TERMINAL, obtention Permis.

## 3.1 Livrables

| # | Livrable | Description |
|---|----------|-------------|
| 1 | Module MWS client | Connexion TCP/TLS Relay, envoi REGISTER avec parent_cog_id |
| 2 | Passeport TERMINAL | Construction Passeport conforme (cog_id, parent_cog_id, etc.) |
| 3 | Permis reçu | Réception REGISTER_OK, stockage Permis (mémoire ou fichier) |
| 4 | Écran liaison | Saisie manuelle parent_cog_id + cog_id (pour test) ou token mock |
| 5 | Stockage identité | Persistance cog_id, parent_cog_id en local |

## 3.2 Critères de passage

- [ ] Connexion Relay réussie depuis émulateur/app
- [ ] Permis de circulation obtenu
- [ ] Heartbeats fonctionnels (optionnel phase 3)

---

# Phase 4 — MVP fonctionnel

**Objectif :** Écrans principaux, synchronisation basique, mode offline.

## 4.1 Livrables

| # | Livrable | Description |
|---|----------|-------------|
| 1 | Navigation | Bottom nav : Salon, Paramètres |
| 2 | Écran Salon | Liste services du parent (données mock ou sync) |
| 3 | Vue consultative | Détail JayKonta ou JayKoa (lecture seule) |
| 4 | Mode offline | Cache local, indicateur connecté/déconnecté |
| 5 | Queue actions | Structure queue, rejeu à la reconnexion (basique) |

## 4.2 Critères de passage

- [ ] Parcours liaison → Salon → détail service fonctionnel
- [ ] Offline : lecture cache possible
- [ ] Sync : données rafraîchies à la connexion

---

# Phase 5 — Intégration Central

**Objectif :** Gestion Terminaux depuis Central, flux liaison complet.

## 5.1 Livrables

| # | Livrable | Description |
|---|----------|-------------|
| 1 | Écran Central "Gérer Terminaux" | Liste, ajout, révocation |
| 2 | Génération token/QR | Central produit token, affiche QR |
| 3 | Scan QR Terminal | Scan + extraction token + validation |
| 4 | Flux liaison complet | Central ↔ Terminal bout en bout |

## 5.2 Critères de passage

- [ ] Création Terminal depuis Central → liaison depuis app → connexion MWS
- [ ] Révocation testée

---

# Phase 6 — Production

**Objectif :** Notifications, signature release, documentation utilisateur.

## 6.1 Livrables

| # | Livrable | Description |
|---|----------|-------------|
| 1 | Notifications | Rappels, seuils (canal à définir) |
| 2 | Signature release | Keystore, APK signé, AAB si Play Store |
| 3 | Documentation utilisateur | Guide liaison, utilisation, dépannage |
| 4 | Tests E2E | Scénarios critiques automatisés |

## 6.2 Critères de passage

- [ ] APK release installable
- [ ] Doc utilisateur publiée
- [ ] Tests E2E passants

---

# Synthèse

| Phase | Durée estimée | Dépendances |
|-------|---------------|-------------|
| 1 Documentation | 2–4 semaines | Étude préalable |
| 2 Setup | 1 semaine | Phase 1 |
| 3 POC | 2–3 semaines | Phase 2, docs MWS |
| 4 MVP | 3–4 semaines | Phase 3 |
| 5 Intégration | 2 semaines | Phase 4, Central |
| 6 Production | 2–3 semaines | Phase 5 |

**Stack confirmée :** Dioxus 0.6+ (mobile) + Rust (logique, MWS, persistance).

---

## Références

- **Étude** : `docs/implementation/Miyukini COG - Etude App Android Terminal.md`
- **MWS** : `docs/miyukini-webway-system/`
- **Central** : `apps/central/`, skill miyukini-dioxus-ui
- **Dioxus Mobile** : https://dioxus.dev/learn/0.6/guides/mobile/
