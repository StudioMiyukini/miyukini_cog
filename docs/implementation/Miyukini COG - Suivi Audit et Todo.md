# Miyukini COG — Suivi Audit et Todo

**Complétion globale : ~88 %** (dernière mise à jour : 2026-02-16)

| Strate / Périmètre | Complété | Total | % |
|--------------------|----------|-------|--:|
| Cores (Strate 4) | 11 | 11 | 100 % |
| BondingBrother (Strate 5) | 1 | 1 | 100 % |
| Toolkits (Strate 6) — priorisés | 11 | 12 | 92 % |
| Services (Strate 7) — avec crate | 7 | 9 | 78 % |
| Services — sans crate | 3 | 4 | 75 % |
| Applications | 2 | 3 | 67 % |
| MWS | 3 | 3 | 100 % |
| **Total (items suivis)** | **36** | **42** | **86 %** |

*Les « 37 toolkits » en backlog comptent pour 1 item (ligne Backlog).*

---

## Contexte

Ce document est le **journal de suivi** dérivé de l’audit complet du projet (état documentation vs implémentation). Il sert à suivre l’avancement des tâches par strate et par composant, et à prioriser les manquements identifiés.

**Référence audit :** état des lieux réalisé à partir de `docs/` et du code (crates/, apps/, tools/).

## Portée / Scope

- **Périmètre :** Cores (Strate 4), BondingBrother (Strate 5), Toolkits (Strate 6), Services (Strate 7), Applications, MWS.
- **Objectif :** Réduire l’écart documentation / implémentation et traiter les manquements critiques.
- **Mise à jour :** À mettre à jour à chaque avancement significatif (date en en-tête de section « Suivi »).

---

## Synthèse état des lieux (audit)

| Dimension              | État |
|------------------------|------|
| Documentation          | ~1045+ fichiers .md, structure normée |
| Implémentation réelle  | ~25–30 % du code avec logique métier |
| Scaffolding            | ~70 % des crates en squelettes structurés |

---

## Todo par strate

### Strate 4 — Cores système

| # | Composant        | Doc | Implémentation actuelle | Todo | Priorité | Statut |
|---|------------------|:---:|--------------------------|------|:--------:|:------:|
| 1 | Kernel           | ✅  | ✅ Complète              | —    | —        | ✅ Fait |
| 2 | StrongFather     | ✅  | ✅ Complète              | —    | —        | ✅ Fait |
| 3 | KindMother       | ✅  | ✅ Complète (+ satellites)| —   | —        | ✅ Fait |
| 4 | BorderGuard      | ✅  | ✅ Registre + Classifier | BoundaryRegistry, DefaultTrustLevelClassifier, BoundaryMetadata | Haute | ✅ Fait |
| 5 | CaringNanny      | ✅  | ✅ Impl. par défaut     | DefaultObserver, DefaultHealthChecker | Moyenne | ✅ Fait |
| 6 | MasterButler     | ✅  | ✅ Impl. par défaut     | DefaultOrchestrator (exécution linéaire) | Moyenne | ✅ Fait |
| 7 | BondingBrother   | ✅  | ✅ Impl. par défaut     | DefaultConnectionManager, DefaultSyncManager, DefaultTranslator | Haute | ✅ Fait |
| 8 | EverBuddy        | ✅  | ✅ Impl. par défaut     | DefaultMigrationExecutor, DefaultVersionManager | Moyenne | ✅ Fait |
| 9 | WorrySentinel    | ✅  | ✅ Impl. par défaut     | DefaultThreatDetector, DefaultSecurityLevelManager, DefaultDegradationManager | Haute | ✅ Fait |
|10 | TAMR             | ✅  | ✅ Impl. par défaut     | DefaultTaxonomyManager, DefaultMetadataManager | Basse | ✅ Fait |
|11 | LogisticsSteward | ✅  | ✅ Impl. par défaut     | DefaultResourceManager, DefaultAllocationManager | Basse | ✅ Fait |

---

### Strate 6 — Toolkits

#### Déjà implémentés (à maintenir / étendre)

| # | Toolkit              | État | Todo optionnel | Statut |
|---|----------------------|:----:|----------------|:------:|
| 1 | MiyuSQL              | ✅   | —              | ✅ Fait |
| 2 | MiyuValidate        | ✅   | —              | ✅ Fait |
| 3 | MiyuLocale          | ✅   | —              | ✅ Fait |
| 4 | MiyuExport          | ✅   | —              | ✅ Fait |
| 5 | MiyuWebwayParticipant | ✅ | —              | ✅ Fait |

#### Partiels (à compléter)

| # | Toolkit   | Manque identifié              | Todo | Priorité | Statut |
|---|-----------|-------------------------------|------|:--------:|:------:|
| 6 | MiyuJobs  | `process()` stub               | Implémenter exécution de la file | Moyenne | ✅ Fait |
| 7 | MiyuHR    | Pas de persistance            | Ajouter persistance time clock   | Basse  | ✅ Fait |
| 8 | MiyuWebwayTracker | Retours Unimplemented | Aligner sur Origin Tracker ou documenter délégation | Haute | ✅ Fait |

#### Scaffolding (42 toolkits) — priorisation par dépendance

| Priorité | Toolkits à implémenter en premier (bloquants services) | Todo | Statut |
|:--------:|--------------------------------------------------------|------|:------:|
| **Critique** | MiyuAuth | Implémenter resolve, verify, attest, role (tout est Unimplemented) | ✅ Fait |
| Haute     | MiyuProfile, MiyuContacts | Nécessaires pour Central / Jay1Tribu | ✅ Fait |
| Moyenne   | MiyuStore, MiyuBilling, MiyuForum | Pour services commerce / communautaire | ✅ Fait (bases) |
| Basse     | Autres 37 toolkits | Suivre Reference Implementation Guidelines par toolkit | ⬜ Backlog |

*Liste complète des 49 toolkits : voir `docs/tools/` et audit détaillé.*

---

### Strate 7 — Services

| # | Service           | Backend | Manque principal           | Todo | Priorité | Statut |
|---|-------------------|:-------:|----------------------------|------|:--------:|:------:|
| 1 | JayFestival       | ✅      | UI dans Central (Dioxus)   | — | — | ✅ Fait |
| 2 | JayKonta          | ✅      | UI dans Central (Dioxus)   | — | — | ✅ Fait |
| 3 | JayKoa            | ⚠️ Types| Pas de logique calendrier  | Implémenter calculs, CRUD événements/agenda | **Haute** | ✅ Fait (agrégation + conflits) |
| 4 | JayXpose          | ⚠️      | Dépend JayFestival         | Clarifier périmètre et compléter si besoin | Basse | ⬜ À faire |
| 5 | Jay1Tribu         | ✅      | —                          | — | — | ✅ Fait |
| 6 | MiyukiniWatch     | ✅      | —                          | — | — | ✅ Fait |
| 7 | MiyukiniClicker   | ✅      | UI retirée                 | Restaurer UI jeu si prévu | Basse | ⬜ À faire |
| 8 | Lord of the Castle| ✅      | —                          | — | — | ✅ Fait |
| 9 | Miyukini Central  | ✅      | —                          | — | — | ✅ Fait |

#### Services documentés sans crate

| # | Service         | Todo | Priorité | Statut |
|---|-----------------|------|:--------:|:------:|
|10 | JayRDV          | Créer crate + implémentation rendez-vous | Moyenne | ✅ Fait |
|11 | JayFaim         | Créer crate + implémentation réservations | Basse  | ✅ Fait |
|12 | MiyukiniSales   | Créer crate + implémentation devis/ventes | Moyenne | ✅ Fait |
|13 | MiyukiniSurvivor| Créer crate si scope distinct de Lord of the Castle | Basse | ⬜ À faire |

---

### Applications

| # | Application   | État | Todo | Statut |
|---|---------------|:----:|------|:------:|
| 1 | apps/central  | ✅   | —    | ✅ Fait |
| 2 | apps/origin   | ✅   | —    | ✅ Fait |
| 3 | apps/ui-builder | ✅ | Doc minimale si utile | ✅ Optionnel (doc ajoutée) |

---

### MWS (Miyukini Webway System)

| # | Composant     | État | Todo | Statut |
|---|---------------|:----:|------|:------:|
| 1 | Origin (Relay + Tracker + Web) | ✅ | — | ✅ Fait |
| 2 | MiyuWebwayParticipant (toolkit) | ✅ | — | ✅ Fait |
| 3 | MiyuWebwayTracker (toolkit)    | ✅ Stubs + cog_list | Aligner ou documenter | ✅ Fait |

---

## Détail de ce qu'il reste à faire par sujet

Les paragraphes ci-dessous décrivent, pour chaque sujet, les tâches concrètes à réaliser, les livrables attendus et les références utiles.

---

### 1. Cores système (Strate 4) — Détail des tâches

#### 1.1 BorderGuard

**État actuel :** Structs `Boundary`, `BoundaryType`, `CrossingRule`, `CrossingRules`, `TrustLevel` et trait `TrustLevelClassifier` existent. Constructeurs et tests unitaires de base présents. Aucune logique de registre ni de classification réelle.

**À faire :**

1. **Registre des frontières (en mémoire)**  
   - Implémenter un registre (ex. `BoundaryRegistry`) qui maintient la liste des `Boundary` avec métadonnées (createdAt, createdBy, justification, version) conformément à INV-BG-8.  
   - APIs de consultation uniquement (GET) : obtenir une frontière par id, lister les frontières, obtenir les règles de franchissement pour une frontière.  
   - Pas d’accès DB ni d’écriture fichier ; persistance déléguée à KindMother via canaux/événements (INV-BG-2).  
   - Référence : `docs/cores/BorderGuard/implementation/Border Guard - Reference Implementation Guidelines.md` (invariants INV-BG-1 à INV-BG-10).

2. **Classification des entités**  
   - Implémenter une implémentation concrète de `TrustLevelClassifier` : à partir d’un `entity_id` (source/destination), retourner un `TrustLevel` (trusted, verified, unknown, hostile).  
   - Défaut = `unknown` (INV-BG-4). Pas de décision ALLOW/DENY (INV-BG-3) : BorderGuard ne fait qu’informer.

3. **Cohérence et traçabilité**  
   - Vérifier l’absence de contradictions entre frontières et règles (INV-BG-9).  
   - S’assurer que toute définition exposée porte les métadonnées requises (INV-BG-8).

**Livrable :** Crate `borderguard` avec au moins une implémentation de `BoundaryRegistry` et de `TrustLevelClassifier` utilisables par StrongFather/BondingBrother en lecture seule.

---

#### 1.2 BondingBrother

**État actuel :** Traits `ConnectionManager` (connect), `SyncManager` (sync), `Translator` (translate). Structs `Connection`, `SyncStrategy`, `Translation`. Aucune implémentation concrète.

**À faire :**

1. **ConnectionManager**  
   - Implémentation concrète qui, pour un `operator_id` donné (sous mandat), établit ou récupère une `Connection` vers l’opérateur.  
   - Pas de décision ALLOW/DENY : le mandat est déjà accordé par StrongFather.  
   - Référence : `docs/interfaces/BondingBrother/implementation/BondingBrother - Reference Implementation Guidelines.md`.

2. **SyncManager**  
   - Implémentation qui exécute une stratégie de synchronisation (`SyncStrategy`) : par ex. sync bidirectionnel, push, pull.  
   - Délégation à KindMother pour toute persistance ; BondingBrother orchestre, ne stocke pas.

3. **Translator**  
   - Implémentation qui transforme des données (`data: &[u8]`) selon une `Translation` (format source → format cible).  
   - Utilisation possible pour adapter les flux entre strates ou entre COG (MWS).

**Livrable :** Au moins une implémentation de chaque trait (ex. `DefaultConnectionManager`, `DefaultSyncManager`, `DefaultTranslator`) avec tests unitaires.

---

#### 1.3 CaringNanny

**État actuel :** Trait `Observer` (notify), struct `SystemEvent`, trait `HealthChecker` (check), enum `HealthStatus`. Aucune implémentation concrète.

**À faire :**

1. **Observer**  
   - Implémentation qui reçoit des `SystemEvent` et les enregistre ou les transmet (ex. vers un logger Kernel, ou une file pour traitement ultérieur).  
   - Pas de décision métier ; observation et traçabilité uniquement.

2. **HealthChecker**  
   - Implémentation qui retourne un `HealthStatus` (ex. Healthy, Degraded, Unhealthy) en fonction de l’état des composants observés (ex. KindMother disponible, file d’événements non saturée).  
   - Référence : `docs/cores/CaringNanny/implementation/Caring Nanny - Reference Implementation Guidelines.md`.

**Livrable :** Au moins une implémentation de chaque trait, utilisable par le cycle de vie ou les écrans d’administration.

---

#### 1.4 MasterButler

**État actuel :** Trait `Orchestrator` (execute), struct `Workflow` (étapes). Aucune exécution réelle.

**À faire :**

1. **Orchestrator**  
   - Implémentation qui exécute un `Workflow` : enchaînement d’étapes (steps), avec gestion d’erreur et rollback si défini.  
   - Chaque étape peut être une délégation vers un Core ou un Toolkit (sous mandat).  
   - Référence : `docs/cores/MasterButler/implementation/Master Butler - Reference Implementation Guidelines.md`.

2. **Workflow**  
   - Rendre le workflow exécutable : définition des steps, ordre, paramètres, et branchements conditionnels si nécessaire (selon contrats).

**Livrable :** Une implémentation d’orchestrateur capable d’exécuter au moins un workflow linéaire (séquence d’étapes).

---

#### 1.5 EverBuddy

**État actuel :** Trait `MigrationExecutor`, struct `Migration`, trait `VersionManager`, struct `Version`. Aucune implémentation.

**À faire :**

1. **MigrationExecutor**  
   - Exécuter une `Migration` (schéma, données) de manière ordonnée et traçable.  
   - Délégation à KindMother pour l’exécution SQL / persistance réelle.

2. **VersionManager**  
   - Gérer les versions (environnement, schéma, compatibilité).  
   - Fournir la version courante et la liste des migrations appliquées (sans décision métier).

**Référence :** `docs/cores/EverBuddy/implementation/Ever Buddy - Reference Implementation Guidelines.md`.  
**Livrable :** Implémentations concrètes + tests (ex. migration vide, version lue).

---

#### 1.6 WorrySentinel

**État actuel :** Trait `ThreatDetector` (detect → ThreatLevel), traits `SecurityLevel` et `Degradation`. Aucune implémentation.

**À faire :**

1. **ThreatDetector**  
   - Implémentation qui agrège des signaux (ex. échecs d’auth, taux d’erreur, anomalies) et retourne un `ThreatLevel` (Low, Medium, High, Critical).  
   - Ne décide pas de l’action ; StrongFather ou un opérateur décide à partir de cette information.

2. **SecurityLevel / Degradation**  
   - Implémentations qui exposent l’état courant (niveau de sécurité, état de dégradation) à partir des observations (ex. CaringNanny, métriques).

**Référence :** `docs/cores/WorrySentinel/implementation/WorrySentinel - Reference Implementation Guidelines.md`.  
**Livrable :** Au moins un détecteur concret (ex. basé sur compteurs ou seuils) et une implémentation des états sécurité/dégradation.

---

#### 1.7 TAMR

**État actuel :** Trait `TaxonomyManager`, struct `Taxonomy`, trait `MetadataManager`, struct `Metadata`. Aucune implémentation.

**À faire :** Implémenter des gestionnaires en mémoire (ou déléguant la persistance à KindMother) pour taxonomies et métadonnées : création, lecture, mise à jour, liste. Pas de logique métier applicative ; uniquement structure et classification.  
**Référence :** `docs/cores/TAMR/implementation/TAMR - Reference Implementation Guidelines.md`.  
**Livrable :** Au moins une implémentation de chaque trait avec API de base (CRUD déclaratif).

---

#### 1.8 LogisticsSteward

**État actuel :** Trait `ResourceManager`, struct `Resource`, trait `AllocationManager`, struct `Allocation`. Aucune implémentation.

**À faire :** Implémentations qui définissent et listent des ressources et des allocations (qui utilise quoi, combien). Décision d’allocation = StrongFather ; LogisticsSteward enregistre et informe.  
**Référence :** `docs/cores/LogisticsSteward/implementation/LogisticsSteward - Reference Implementation Guidelines.md`.  
**Livrable :** Implémentations concrètes avec consultation (GET) des ressources et allocations.

---

### 2. MiyuAuth (Toolkit) — Détail des tâches

**État actuel :**  
- `identity_resolve` : retourne `Unimplemented`.  
- `identity_verify` : retourne `Unimplemented`.  
- `identity_attest` : retourne `Unimplemented`.  
- `identity_role` : implémenté (retourne `context.role`).

**À faire :**

1. **resolve(ctx, artefacts) → IdentityContext**  
   - À partir d’`IdentityArtefacts` déjà validés par KindMother (ou fournis dans un flux gouverné), construire un `IdentityContext` (identité résolue, rôle dérivé si possible).  
   - Ne jamais décider ALLOW/DENY (BOUND-1). Ne pas utiliser de confiance non validée par KindMother (BOUND-3).  
   - Référence : `docs/tools/MiyuAuth/implementation/MiyuAuth - Reference Implementation Guidelines.md`, contrats Security and States, KindMother Integration.

2. **verify(ctx, …) → VerificationResult**  
   - Vérifier un Passeport/Visa (signature, expiration, format) à partir d’artefacts validés.  
   - Retourner un résultat structuré (valide / invalide + raison technique) sans exposer de données sensibles.

3. **attest(ctx, …) → Attestation**  
   - Produire une attestation (preuve d’identité ou de rôle) à partir du contexte validé.  
   - Format et contenu selon contrats MiyuAuth ; traçabilité via Logger Kernel.

4. **Tests**  
   - Mettre à jour les tests unitaires : remplacer les assertions sur `Unimplemented` par des scénarios avec mocks KindMother / artefacts valides.  
   - Ajouter un test de cycle (résolution → rôle → vérification) conforme au Cycle Tests Contract.

**Livrable :** Les quatre tools exécutables sous mandat, avec tests unitaires et un test de cycle documenté.

---

### 3. JayKoa (Service calendrier) — Détail des tâches

**État actuel :** Types domaine (Agenda, TemporalEntry, TemporalConflict, etc.), persistance via KindMother (legacy-sqlite / kindmother-only), adaptateurs lecture seule vers JayFestival et JayRDV. Aucune logique d’agrégation temporelle ni d’orchestration des vues calendrier.

**À faire :**

1. **Agrégation temporelle**  
   - À partir des adaptateurs JayFestival et JayRDV (et futurs services temporels), agréger les entrées dans une vue unifiée (par utilisateur, par période).  
   - Gérer les conflits (TemporalConflict) : détection et exposition, sans décision automatique de résolution (JayKoa « reflète », ne décide pas du temps).

2. **API / fonctions métier**  
   - Exposer des fonctions du type : « entrées pour un agenda et une plage de dates », « conflits pour un agenda », « synthèse par jour/semaine/mois ».  
   - Pas de création/suppression d’événements dans les services externes (lecture seule stricte, cf. commentaire dans `lib.rs`).

3. **Export**  
   - Compléter le module `export` (iCal, éventuellement PDF) pour les vues agrégées.

**Livrable :** Module(s) d’agrégation et de requêtes temporelles utilisables par Central ou une future UI calendrier, avec tests sur des jeux de données (éditions Festival + RDV mock).

---

### 4. MiyuJobs (Toolkit) — Détail des tâches

**État actuel :** `queue_enqueue` implémenté (génération d’id, validation queue_id). `queue_process` retourne toujours `ProcessResult { processed: false, task_id: None }`.

**À faire :**

1. **Stockage des tâches**  
   - Persister les tâches enqueue (payload, queue_id, options, id) via KindMother ou un store gouverné.  
   - Définir un format de tâche (payload, métadonnées, statut).

2. **process(ctx, queue_id, batch_size)**  
   - Lire jusqu’à `batch_size` tâches dans la queue (statut « pending »).  
   - Pour chaque tâche : appeler un handler fourni par le flux (callback ou trait), marquer comme traité ou en erreur.  
   - Retourner `ProcessResult { processed: true, task_id: Some(id) }` ou résumé des tâches traitées.  
   - Gestion des erreurs : retry ou dead-letter selon contrat (à préciser dans la Reference Implementation Guidelines si besoin).

**Livrable :** `queue_process` fonctionnel avec au moins un backend de file (ex. en mémoire ou KindMother), tests unitaires et un test d’intégration (enqueue → process).

---

### 5. MiyuWebwayTracker (Toolkit) — Détail des tâches

**État actuel :** Toutes les fonctions publiques retournent `Unimplemented` après vérification du mandat :  
`address::tracker_default`, `cog_list::filter/get/merge/update`, `declaration::validate/verify`, `discovery::response_build/response_send`, `port::check`, `transport::receive/send`.

**À faire (au choix, à trancher par l’équipe) :**

**Option A — Implémentation côté participant**  
   - Implémenter les fonctions du toolkit pour un COG **participant** (client) :  
     - `declaration_validate` / `declaration_verify` : validation et vérification de signature des déclarations reçues (alignement format MWS).  
     - `discovery_response_build` / `discovery_response_send` : construire et envoyer une réponse de découverte.  
     - `transport_receive` / `transport_send` : encapsulation des échanges avec le Tracker (Origin).  
     - `cog_list_*` : représentation locale de la liste des COG (sync depuis Tracker).  
   - S’appuyer sur le protocole décrit dans `docs/miyukini-webway-system/` et sur le comportement réel de `apps/origin` (Tracker).

**Option B — Délégation explicite**  
   - Si le participant utilise uniquement `miyuwebway_participant` et que le toolkit Tracker est réservé à Origin : documenter clairement que MiyuWebwayTracker (crate) est un « client léger » ou une façade qui délègue à Origin, et implémenter des appels vers le Tracker distant (HTTP/TCP selon MWS) au lieu de logique locale.  
   - Dans ce cas : remplacer `Unimplemented` par des appels réels et des erreurs explicites si le Tracker est indisponible.

**Livrable :** Soit implémentations locales (validation, transport, cog_list) conformes au MWS, soit délégation documentée + implémentation des appels réseau ; plus tests.

---

### 6. MiyuProfile et MiyuContacts (Toolkits) — Détail des tâches

**État actuel :** Structure admin_cell, contrats de gouvernance. Fonctions retournent `Unimplemented` ou résultats vides.

**À faire (MiyuProfile) :**  
- Modèle de profil (avatar, nom affiché, préférences, visibilité) ; persistance via KindMother.  
- API : get_profile, set_profile, search_public_profiles (selon Reference Implementation Guidelines dans `docs/tools/MiyuProfile/`).  
- Intégration avec MiyuAuth (contexte identité) sans décision d’autorisation dans le toolkit.

**À faire (MiyuContacts) :**  
- Modèle contacts (liste de contacts, statut ami/demandes).  
- API : add_contact, remove_contact, list_contacts, get_contact_status.  
- Alignement avec Jay1Tribu (amis, présence) si nécessaire.

**Livrable :** Implémentations utilisables par Central et Jay1Tribu (profils et liste d’amis/contacts), avec tests.

---

### 7. Services — UI et nouveaux crates

#### 7.1 UI à restaurer ou finaliser

- **JayFestival, JayKonta** ✅  
  - UI hébergée dans **apps/central** (Dioxus) : vues complètes (JayFestivalView, JayKontaView), sidebars par rôle (Organisateur / Exposant / Visiteur ; Purse / Account), écrans branchés sur `jayfestival` et `jaykonta` (JayFestivalDb, JayKontaDb). Migration future vers Tauri + React/TypeScript optionnelle (voir `docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md`).

- **MiyukiniClicker**  
  - Restaurer une UI jeu (Dioxus ou Tauri) qui consomme la logique de `miyuclicker` (IdleSim, Carte, Combat, Save) si le produit est toujours ciblé.

#### 7.2 Services documentés sans crate

- **JayRDV** ✅ (crate créé, Phase 2 livrée)  
  - Crate `jayrdv` : types complets (Appointment, Slot, Resource, Service, Client, Reminder, Professional, Practitioner, Schedule, Exception, ProfessionalSettings, PractitionerRole, ScheduleOwner) ; store en mémoire + persistance kindmother-only ; domain (appointment_create, slot_hold/release, slot_release_expired, professional_create, practitioner_create, schedule_create, exception_create, etc.) ; filtres `appointment_list` par dates (chrono) ; index MIP à jour.  
  - Intégration JayKoa : sync_appointments_from_store (test d’intégration). À venir : MiyuBooking (créneaux), MiyuNotify (rappels). Référence : `docs/services/JayRDV/`.

- **MiyukiniSales** ✅ (crate créé)  
  - Crate `miyukinisales` : types Quote, Order, OrderLine, QuoteStatus, OrderStatus ; store en mémoire ; domain (quote_create, quote_send, quote_accept, order_create, order_create_from_quote, order_confirm).  
  - Intégration à venir : MiyuStore, MiyuInvoice, JayKonta.

- **JayFaim** ✅ (crate créé)  
  - Crate `jayfaim` : types Table, ReservationSlot, Reservation, Guest, ReservationStatus ; store en mémoire ; domain (table_create, slot_create, reservation_create, reservation_confirm, reservation_cancel, guest_add).  
  - Intégration à venir : MiyuBooking, JayFestival (mode couplé), JayKonta.

- **MiyukiniSurvivor**  
  - Créer le crate seulement si le scope est distinct de Lord of the Castle (autre gameplay, autre progression). Sinon, documenter que Lord of the Castle est le service Survivor/Tower Defense de la gamme.

**Livrable :** Pour chaque service retenu : crate avec data/auth/services + intégration aux toolkits listés + documentation de portée.

---

### 8. Toolkits restants (42) — Périmètre

Les 42 toolkits en scaffolding (MiyuWeb, MiyuCMS, MiyuStore, MiyuBilling, MiyuForum, etc.) ont chacun une **Reference Implementation Guidelines** dans `docs/tools/Miyu<Nom>/implementation/`. Pour chaque toolkit :

1. Lire la Documentation Fondatrice et les contrats (governance, security, KindMother si applicable).  
2. Implémenter les tools listés (ex. tool.store.cart.add, tool.store.checkout.submit) en respectant mandat, pas de décision ALLOW/DENY, délégation persistance à KindMother.  
3. Remplacer les `Unimplemented` par une logique réelle ou une erreur explicite (ex. « Feature not available ») si hors périmètre v0.1.  
4. Ajouter tests unitaires et, si pertinent, tests d’intégration avec KindMother.

Priorisation recommandée (en plus de MiyuAuth, MiyuProfile, MiyuContacts) : MiyuStore, MiyuBilling, MiyuForum pour débloquer des cas d’usage commerce et communautaire ; le reste en backlog selon la roadmap produit.

---

### 9. Références rapides par sujet

| Sujet | Document principal |
|-------|--------------------|
| BorderGuard | `docs/cores/BorderGuard/implementation/Border Guard - Reference Implementation Guidelines.md` |
| BondingBrother | `docs/interfaces/BondingBrother/implementation/BondingBrother - Reference Implementation Guidelines.md` |
| CaringNanny | `docs/cores/CaringNanny/implementation/Caring Nanny - Reference Implementation Guidelines.md` |
| MasterButler | `docs/cores/MasterButler/implementation/Master Butler - Reference Implementation Guidelines.md` |
| EverBuddy | `docs/cores/EverBuddy/implementation/Ever Buddy - Reference Implementation Guidelines.md` |
| WorrySentinel | `docs/cores/WorrySentinel/implementation/WorrySentinel - Reference Implementation Guidelines.md` |
| TAMR | `docs/cores/TAMR/implementation/TAMR - Reference Implementation Guidelines.md` |
| LogisticsSteward | `docs/cores/LogisticsSteward/implementation/LogisticsSteward - Reference Implementation Guidelines.md` |
| **Vérification Cores** | `docs/implementation/Miyukini COG - Verification Cores Conformite Doc.md` |
| MiyuAuth | `docs/tools/MiyuAuth/implementation/MiyuAuth - Reference Implementation Guidelines.md` |
| MiyuJobs | `docs/tools/MiyuJobs/implementation/` (Reference Implementation Guidelines) |
| MiyuWebwayTracker / MWS | `docs/miyukini-webway-system/` + `apps/origin` (Tracker) |
| Migration UI | `docs/implementation/Miyukini - Plan Migration Tauri React TypeScript.md` |

---

## Priorisation globale

| Priorité | Items |
|:--------:|-------|
| **Critique** | ~~MiyuAuth~~ (fait), ~~JayKoa~~ (fait : agrégation + conflits) |
| **Haute**    | ~~BorderGuard~~, ~~BondingBrother~~, ~~WorrySentinel~~, ~~MiyuWebwayTracker~~, ~~MiyuProfile~~, ~~MiyuContacts~~ (faits) |
| **Moyenne**  | ~~CaringNanny~~, ~~MasterButler~~, ~~EverBuddy~~ (faits) ; MiyuJobs ; ~~UI JayFestival/JayKonta~~, ~~JayRDV~~, ~~MiyukiniSales~~ (faits) |
| **Basse**    | ~~TAMR~~, ~~LogisticsSteward~~ (faits) ; MiyuHR ; 37 toolkits restants ; ~~JayFaim~~ (fait) ; MiyukiniSurvivor ; UI MiyukiniClicker |

---

## Suivi (mises à jour)

| Date       | Modifié par / Résumé | Prochaine étape |
|------------|----------------------|-----------------|
| *(à remplir)* | Création document à partir de l’audit | Prioriser MiyuAuth et JayKoa |
| 2026-02-16 | Implémentation MiyuAuth (resolve, attest, verify, role) ; tests mis à jour | JayKoa agrégation temporelle ou BorderGuard |
| 2026-02-16 | BorderGuard : BoundaryRegistry, BoundaryMetadata, DefaultTrustLevelClassifier | BondingBrother ou JayKoa |
| 2026-02-16 | BondingBrother : DefaultConnectionManager, DefaultSyncManager, DefaultTranslator | WorrySentinel ou JayKoa |
| 2026-02-16 | Cores : WorrySentinel, CaringNanny, MasterButler, EverBuddy, TAMR, LogisticsSteward (impl. par défaut) | Toolkits / Services |
| 2026-02-16 | Vérification conformité Cores / doc : rapport `Miyukini COG - Verification Cores Conformite Doc.md` ; tous Cores conformes ; WorrySentinel INV-WS-4 clarifié (setters = vue déclarative par adaptateur) | Suite toolkits / services |
| 2026-02-16 | Conformité maximale WorrySentinel : suppression de `set_level`, `set_state`, `push_signal` ; API immuable (constructeurs `new` / `with_signals` uniquement) ; doc Reference Implementation Guidelines + rapport vérification mis à jour | — |
| 2026-02-16 | Toolkits : MiyuJobs (queue store + process), MiyuProfile (profile/field/preferences store), MiyuContacts (friend/foe store + list), MiyuWebwayTracker (stubs + cog_list en mémoire, TrackerUnavailable pour transport), MiyuStore (product + cart en mémoire), MiyuBilling (invoice generate/list, tenant resolve), MiyuForum (déjà stubs Ok) | Autres 37 toolkits en backlog |
| 2026-02-16 | Toolkits (suite) : MiyuStore checkout (validate, submit→order.create), order (create/update/status/list en mémoire), payment (capture/refund/status), shipping (rate=0, zones=[default]) ; MiyuProfile avatar (get/set/resolve binaire), signature (champ profil), rank (list défaut member/vip/mod, resolve=champ rank) ; MiyuBilling subscription (create/update/cancel/status) + payment.record | Poursuivre autres toolkits en backlog |
| 2026-02-16 | Toolkits (suite) : MiyuBookmarks (store add/remove/list par mandate_id, filtres type/limit) ; MiyuPolls (create/vote/list/result, store polls + votes par option) ; MiyuBooking (create/update/cancel en mémoire) ; MiyuCMS content (create/update/publish/schedule, statuts draft/scheduled/published) | Autres toolkits en backlog |
| 2026-02-16 | MiyuHR : store en mémoire des pointages (clock_in/clock_out enregistrés), list_clock_events(employee_id). JayKoa : module aggregation + compute_conflicts(entries) ; entrées en plage = JayKoaDb::entries_in_range (existant). Suivi : MiyuHR, JayKoa, MiyuWebwayTracker marqués Fait. | UI JayFestival/JayKonta, JayRDV, autres services |
| 2026-02-16 | JayRDV : crate `jayrdv` créé (data/types : Appointment, Slot, Resource, Reminder, AppointmentStatus ; data/memory_store ; domain : appointment_create, appointment_set_status, resource_create, slot_create, reminder_create). Intégration JayKoa : JayRDVAdapter::sync_appointments_from_store pour reflets depuis le store. Suivi : JayRDV marqué Fait ; complétion 79 %. | MiyukiniSales, JayFaim, UI services |
| 2026-02-16 | JayRDV rev.2 : modèle enrichi (Service, Client, SlotStatus Hold/Booked/Blocked, CancelledBy, ReminderChannel), validations `start_at < end_at` (chrono), gardes id dupliqué, appointment_cancel structuré, slot_hold/release, 19 tests unitaires (cycle complet), balisage MSCM conforme sur tous les modules, doc comments complets. Complétion JayRDV ~65 %. | Persistance legacy-sqlite, Schedule/Exception, index MIP, MiyukiniSales |
| 2026-02-16 | JayRDV suite (Phases A/B/C) : index MIP régénéré (blocs jayrdv) ; slot_release_expired + slot_list_expired_holds ; appointment_list filtres dates via chrono ; test d’intégration JayKoa sync_appointments_from_store ; entités Professional, Practitioner, Schedule, Exception + ProfessionalSettings, PractitionerRole, ScheduleOwner ; persistance kindmother-only (full KM, pas legacy-sqlite). JayKoa passé full KM (legacy-sqlite supprimé). | MiyukiniSales, UI JayFestival/JayKonta |
| 2026-02-16 | MiyukiniSales : crate `miyukinisales` créé (data/types Quote, Order, OrderLine, QuoteStatus, OrderStatus ; memory_store ; domain quote_create, quote_send, quote_accept, order_create, order_create_from_quote, order_confirm). Suivi : MiyukiniSales marqué Fait. | UI JayFestival/JayKonta, JayFaim, complétion % |
| 2026-02-16 | UI JayFestival/JayKonta : clarifié — UIs hébergées dans apps/central (Dioxus), écrans complets ; Suivi marqué Fait. JayFaim : crate `jayfaim` créé (Table, ReservationSlot, Reservation, Guest ; store mémoire ; domain table_create, slot_create, reservation_create, reservation_confirm/cancel, guest_add). Suivi : JayFaim marqué Fait. Complétion ~88 %. | MiyukiniSurvivor, optionnel ui-builder doc, MiyukiniSales KM |

---

## Références

- **Documentation implémentation :** `docs/implementation/Miyukini COG 0.1 - Documentation Implementation Reference.md`
- **Gel v0.1 :** `docs/implementation/Miyukini COG 0.1 - Document de Gel v0.1.md`
- **Checklist MSCM/MIP :** `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- **Arborescence docs :** `.cursor/skills/miyukini-docs/SKILL.md`
