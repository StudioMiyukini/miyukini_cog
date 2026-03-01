# Rapport Fondateur — Alicia Home Assistante

<!-- @id: doc.alicia-home.rapport-fondateur -->
<!-- @role: project-charter -->
<!-- @layer: governance -->
<!-- @human: Rapport fondateur complet du projet Alicia Home Assistante — transformation de MiyukiniVoice -->
<!-- @do: define_alicia_home_project_scope_and_plan -->

**Auteur :** Maria, Chef de Projet Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Statut :** Actif — en cours de planification

---

## Résumé exécutif

Alicia Home Assistante est la transformation structurelle de MiyukiniVoice en une plateforme domotique locale complète. Le projet conserve l'acquis vocal (capture audio cpal, VAD, wake word "Hey Alicia" via rustpotter) et y greffe trois nouveaux piliers : un moteur domotique local (MQTT/Zigbee2MQTT/HTTP local), une API REST sécurisée exposable vers les serveurs MWS distants, et un pont vers les services COG internes (miou-llm-bridge, Market, KindMother).

La philosophie centrale est identique aux Lois d'Autonomie COG : Alicia fonctionne à 100 % hors-ligne, sans aucun cloud obligatoire. Le réseau distant (MWS, services tiers) est un complément optionnel, jamais une dépendance critique.

**Positionnement :** assistant domotique local souverain, piloté par la voix et accessible à distance via protocoles sécurisés, intégré nativement à l'écosystème COG Miyukini.

---

## 1. Contexte

### 1.1 Situation actuelle

MiyukiniVoice est opérationnel en Phase 1 :
- Crates backend : `miyuvoicecapture` (cpal, VAD, ring buffer SPSC) et `miyuwakeword` (rustpotter, détection "Hey Alicia")
- UI Dioxus 0.6 : tableau de bord, configuration des 4 pièces, paramètres
- Architecture : VoiceService avec threads de traitement par pièce, Arc<RwLock<>> pour état partagé, polling UI 200ms
- 4 pièces prédéfinies : chambre-theresa, chambre-parentale, chambre-eleanore, salon

### 1.2 Problème résolu

MiyukiniVoice Phase 1 détecte les mots-clés mais ne fait rien de cette détection : aucune commande domotique, aucune action déclenchée, aucune interopérabilité avec les autres services COG ni avec les serveurs MWS de la famille Miyukini. La maison "Alicia" ne peut pas encore contrôler la réalité physique du foyer.

### 1.3 Vision cible

Alicia Home Assistante est un système complet où :
1. La détection vocale déclenche des intentions reconnues (NLU local via miou-llm-bridge)
2. Les intentions commandent des dispositifs domotiques réels (lumières, volets, thermostats, prises, capteurs)
3. L'état de la maison est consultable et contrôlable depuis un serveur MWS distant via une API REST sécurisée (JWT)
4. Les services COG (LLM, Market, notifications) sont accessibles à Alicia selon les permissions configurées
5. Des automatisations et routines s'exécutent de façon autonome

---

## 2. Objectifs

### 2.1 Objectif principal

Transformer MiyukiniVoice en un assistant domotique local complet, contrôlable à distance, intégré aux services COG.

### 2.2 Objectifs secondaires

- Maintenir la philosophie 100 % local : Alicia fonctionne sans réseau
- Conserver la compatibilité ascendante : le wake word "Hey Alicia" et les 4 pièces existantes
- Exposer une API REST sécurisée pour les serveurs MWS Miyukini
- Fournir un système d'automatisation configurable sans code (TOML)
- Intégrer miou-llm-bridge pour la compréhension d'intentions vocales
- Respecter les Lois d'Autonomie COG dans chaque décision technique

### 2.3 Critères de succès mesurables

| Critère | Valeur cible |
|---------|--------------|
| Latence wake word → action domotique | < 1.5 secondes |
| Latence API REST (réseau local) | < 200 ms |
| Disponibilité service local | 99.5 % (hors mises à jour) |
| Nombre de protocoles domotique supportés | MQTT, HTTP local, Zigbee2MQTT |
| Nombre de types de dispositifs supportés | >= 6 (lumières, volets, thermostats, prises, capteurs, serrures) |
| Couverture tests unitaires crates nouveaux | >= 70 % |
| Audit George : zero régression sécurité | Passage obligatoire |

---

## 3. Périmètre (Scope)

### 3.1 Inclus

**Renommage et consolidation**
- Renommage global "MiyukiniVoice" → "Alicia Home Assistante" dans tous les fichiers sources, commentaires, UI et documentation
- Conservation intégrale du code existant (capture audio, VAD, wake word)

**Moteur domotique local**
- Contrôle lumières (on/off, niveau, couleur si RGBW)
- Contrôle volets (ouvrir, fermer, position en pourcentage)
- Contrôle thermostats (température cible, mode chauffage/refroidissement/veille)
- Contrôle prises connectées (on/off, mesure consommation si disponible)
- Lecture capteurs (température, humidité, mouvement, luminosité, contact porte/fenêtre)
- Contrôle serrures connectées (verrouiller, déverrouiller, statut)
- Protocoles : MQTT (broker Mosquitto local), Zigbee2MQTT (bridge local), HTTP local (API propriétaires)

**Pipeline NLU vocal**
- Reconnaissance d'intention après détection wake word (via miou-llm-bridge, modèle local)
- Commandes vocales structurées : "Alicia, éteins la lumière du salon", "Alicia, règle le thermostat à 20 degrés", etc.
- Routines vocales nommées : "Alicia, bonne nuit" déclenche une routine prédéfinie
- Transcription STT locale (Whisper.cpp ou équivalent local via miou-llm-bridge)

**API REST sécurisée**
- Serveur axum dédié, port configurable (défaut 7890)
- Authentification JWT (HS256, clé locale, rotation configurable)
- Endpoints : état maison, contrôle dispositifs, historique, automatisations, config
- TLS optionnel (certificat auto-signé local ou Let's Encrypt si domaine MWS disponible)
- Rate limiting et audit trail complet

**Accès services COG**
- miou-llm-bridge : NLU, génération réponses Alicia, analyse contexte
- miyunotify : notifications push internes COG
- miyukiniwatch : intégration monitoring (optionnel)
- KindMother : persistance état maison, historique, configuration

**Services distants (optionnels, non critiques)**
- Météo : API locale (openmeteo, pas d'API key requise) ou autre source configurable
- Notifications push : via MWS si connecté
- Webhooks sortants : pour intégrations tierces optionnelles

**Automatisations**
- Scénarios TOML : déclencheurs (heure, capteur, vocal, API), conditions, actions
- Routines horaires (lever du soleil/coucher configurable, heures fixes)
- Déclencheurs conditionnels (si capteur mouvement salon ET heure > 22h → éteindre lumières)

**Sécurité**
- Authentification JWT pour API externe
- Permissions par pièce et par dispositif (ACL locale en DB)
- Audit trail complet en KindMother (toutes les commandes avec timestamp et source)
- Chiffrement de la configuration sensible (clés JWT, tokens MWS) via KindMother cipher

**Interface UI (Dioxus 0.6)**
- Refonte de l'écran Alicia : tableau de bord domotique en temps réel
- Panneau de contrôle par pièce (état dispositifs, commandes rapides)
- Éditeur de scénarios/routines (formulaire TOML visuel)
- Gestionnaire des connexions domotique (statut MQTT broker, Zigbee2MQTT, dispositifs)
- Journal d'activité étendu (commandes vocales, API, automatisations)
- Écran paramètres : API REST config, tokens MWS, permissions

### 3.2 Exclus

- Cloud domotique propriétaire (Google Home, Alexa, HomeKit) : aucune dépendance
- Application mobile native Alicia : hors scope Phase 2
- Vidéo/caméras IP : hors scope, trop spécifique (sécurité physique séparée)
- Gestion alarme intrusion : hors scope, domaine réglementé
- Intégration Z-Wave : hors scope Phase 2 (Zigbee suffisant)
- Multi-maisons / multi-COG fédérés : hors scope Phase 2
- Interface vocale multilingue : Phase 2 (français uniquement Phase 1)

---

## 4. Analyse des besoins

### 4.1 Besoins fonctionnels

#### BF-01 : Détection vocale et NLU

| ID | Besoin | Priorité |
|----|--------|----------|
| BF-01-1 | Détection wake word "Hey Alicia" (existant, conservé) | P0 |
| BF-01-2 | Transcription audio → texte post wake word (STT local via miou-llm-bridge) | P0 |
| BF-01-3 | Extraction d'intention depuis transcription (NLU via LLM local) | P0 |
| BF-01-4 | Mapping intention → commande domotique | P0 |
| BF-01-5 | Réponse vocale synthétisée (TTS local optionnel, Phase 1 : log uniquement) | P1 |
| BF-01-6 | Routines vocales nommées déclenchables par mot-clé | P1 |

#### BF-02 : Contrôle domotique

| ID | Besoin | Priorité |
|----|--------|----------|
| BF-02-1 | Contrôle lumières (on/off, niveau 0-100%, couleur RGB) | P0 |
| BF-02-2 | Contrôle volets (ouvrir, fermer, position %) | P0 |
| BF-02-3 | Contrôle thermostats (température cible, mode) | P0 |
| BF-02-4 | Contrôle prises connectées (on/off) | P0 |
| BF-02-5 | Lecture capteurs (temp, humidité, mouvement, contact) | P0 |
| BF-02-6 | Contrôle serrures (verrouiller/déverrouiller) | P1 |
| BF-02-7 | État global maison en temps réel | P0 |

#### BF-03 : API REST externe

| ID | Besoin | Priorité |
|----|--------|----------|
| BF-03-1 | Endpoint GET /alicia/state — état complet maison | P0 |
| BF-03-2 | Endpoint POST /alicia/command — envoyer une commande | P0 |
| BF-03-3 | Endpoint GET /alicia/devices — liste dispositifs | P0 |
| BF-03-4 | Endpoint POST /alicia/auth/token — obtenir JWT | P0 |
| BF-03-5 | Endpoint GET /alicia/history — journal activité | P1 |
| BF-03-6 | Endpoint POST /alicia/automations — créer automatisation | P1 |
| BF-03-7 | Endpoint GET /alicia/rooms/{id}/state — état d'une pièce | P1 |
| BF-03-8 | WebSocket /alicia/stream — événements temps réel | P2 |

#### BF-04 : Automatisations

| ID | Besoin | Priorité |
|----|--------|----------|
| BF-04-1 | Routines horaires (heure fixe, lever/coucher soleil) | P1 |
| BF-04-2 | Déclencheurs conditionnels (capteur + heure + état) | P1 |
| BF-04-3 | Scénarios multi-actions (séquence d'actions) | P1 |
| BF-04-4 | Routines vocales nommées | P1 |
| BF-04-5 | Configuration TOML des automatisations | P1 |

#### BF-05 : Services COG

| ID | Besoin | Priorité |
|----|--------|----------|
| BF-05-1 | Appel miou-llm-bridge pour NLU vocal | P0 |
| BF-05-2 | Persistance état via KindMother | P0 |
| BF-05-3 | Notifications internes via miyunotify | P1 |
| BF-05-4 | Intégration météo locale (openmeteo.com, sans clé API) | P2 |

### 4.2 Besoins techniques

#### BT-01 : Nouveaux crates Rust

| Crate | Strate | Rôle |
|-------|--------|------|
| `miyualicia` | 7 (Service) | Orchestrateur principal Alicia, moteur domotique, NLU bridge |
| `miyualicia-mqtt` | 6 (Toolkit) | Client MQTT (rumqttc), abstraction broker local |
| `miyualicia-http` | 6 (Toolkit) | Client HTTP local pour dispositifs non-MQTT |
| `miyualicia-api` | 7 (Service) | Serveur axum REST, JWT auth, rate limiting |
| `miyualicia-automations` | 6 (Toolkit) | Moteur d'automatisations, scheduler, évaluateur de conditions |
| `miyualicia-devices` | 6 (Toolkit) | Registre des dispositifs, types, mapping MQTT topics |

#### BT-02 : Dépendances nouvelles

| Crate tiers | Usage | Justification |
|-------------|-------|---------------|
| `rumqttc` | Client MQTT async | Léger, pas de dépendance cloud |
| `jsonwebtoken` | JWT HS256 | Auth API externe |
| `tower-http` | Middlewares axum (rate limit, CORS, auth) | Standard ecosystème axum |
| `reqwest` | HTTP client local (timeout strict 2s) | Appels HTTP vers dispositifs locaux |
| `cron` | Parser expressions cron (routines horaires) | Léger, no_std compatible |
| `serde_toml` | Parser config automatisations | Déjà présent dans workspace |

#### BT-03 : Schéma KindMother (SQLite)

**Table `alicia_devices`**
```sql
CREATE TABLE alicia_devices (
    id TEXT PRIMARY KEY,          -- UUID v4
    room_id TEXT NOT NULL,
    device_type TEXT NOT NULL,    -- light|shutter|thermostat|outlet|sensor|lock
    name TEXT NOT NULL,
    protocol TEXT NOT NULL,       -- mqtt|http_local|zigbee2mqtt
    address TEXT NOT NULL,        -- topic MQTT ou URL HTTP
    capabilities TEXT NOT NULL,   -- JSON : {"on_off": true, "dimmer": true, "rgb": false}
    config TEXT NOT NULL,         -- JSON : config spécifique au protocole
    active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Table `alicia_device_states`**
```sql
CREATE TABLE alicia_device_states (
    device_id TEXT NOT NULL REFERENCES alicia_devices(id),
    state TEXT NOT NULL,          -- JSON : état courant du dispositif
    updated_at TEXT NOT NULL,
    PRIMARY KEY (device_id)
);
```

**Table `alicia_commands_log`**
```sql
CREATE TABLE alicia_commands_log (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,         -- voice|api|automation|manual
    source_detail TEXT,           -- room_id si voice, IP si API, automation_id si auto
    device_id TEXT REFERENCES alicia_devices(id),
    command TEXT NOT NULL,        -- JSON : commande envoyée
    success INTEGER NOT NULL,
    error_message TEXT,
    latency_ms INTEGER,
    executed_at TEXT NOT NULL
);
```

**Table `alicia_automations`**
```sql
CREATE TABLE alicia_automations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    enabled INTEGER NOT NULL DEFAULT 1,
    trigger_type TEXT NOT NULL,   -- cron|sensor|voice|api_event
    trigger_config TEXT NOT NULL, -- JSON
    conditions TEXT NOT NULL,     -- JSON : liste de conditions (ET)
    actions TEXT NOT NULL,        -- JSON : liste d'actions séquentielles
    last_triggered_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Table `alicia_api_tokens`**
```sql
CREATE TABLE alicia_api_tokens (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,           -- label lisible : "MWS-principal", "MWS-monitoring"
    token_hash TEXT NOT NULL,     -- BLAKE3 du token brut
    permissions TEXT NOT NULL,    -- JSON : scopes autorisés
    last_used_at TEXT,
    expires_at TEXT,              -- NULL = pas d'expiration
    created_at TEXT NOT NULL
);
```

#### BT-04 : Protocoles domotique

**MQTT (recommandé)**
- Broker Mosquitto local (non fourni par Alicia, configuré à l'installation)
- Topics Zigbee2MQTT : `zigbee2mqtt/{device_friendly_name}/get` et `set`
- Topics custom : `alicia/home/{room_id}/{device_id}/command` et `state`
- QoS 1 pour les commandes, QoS 0 pour les états en temps réel

**HTTP local**
- Timeout strict : 2 secondes
- Retry : 1 seul retry automatique
- Authentification : Basic ou Bearer token stocké chiffré en KindMother
- Exemples : ampoules Shelly, prises TP-Link Tapo (API locale)

**Zigbee2MQTT**
- Bridge Zigbee2MQTT sur le même réseau local (pas fourni)
- Communication via MQTT (voir ci-dessus)
- Découverte automatique des dispositifs depuis `zigbee2mqtt/bridge/devices`

#### BT-05 : API REST — Spécification

Préfixe : `/api/v1/alicia`
Port : 7890 (configurable dans `alicia.toml`)
Base URL depuis MWS : `http://{host_cog}:7890/api/v1/alicia`

```
POST   /auth/token                 Corps : {"client_id": "...", "secret": "..."}
GET    /state                      État maison complet
GET    /rooms                      Liste des pièces avec dispositifs
GET    /rooms/{room_id}            État d'une pièce
GET    /rooms/{room_id}/devices    Dispositifs d'une pièce
GET    /devices                    Tous les dispositifs
GET    /devices/{id}               État d'un dispositif
POST   /devices/{id}/command       Commande sur un dispositif
GET    /automations                Liste des automatisations
POST   /automations                Créer une automatisation
PUT    /automations/{id}           Modifier
DELETE /automations/{id}           Supprimer
POST   /automations/{id}/trigger   Déclencher manuellement
GET    /history                    Journal des commandes (paginate)
GET    /health                     Santé du service
```

### 4.3 Besoins en ressources

#### Humains

| Rôle | Agent | Charge estimée |
|------|-------|----------------|
| Chef de Projet | Maria | Suivi continu, 2h/sprint |
| Analyste PR | Fabrice | Audit initial concurrence (phase 0) |
| Documentation technique | Denis | Spec complète + coordination |
| Backend Rust | Francois | Crates miyualicia*, API, DB, MQTT |
| Frontend Dioxus | Lise | UI refonte complète + nouveaux écrans |
| Audit final | George | Conformité COG, sécurité, UX |
| Archivage qualité | Arianne | Mémoire, anti-hallucination |

#### Infrastructure locale

| Composant | Requis | Optionnel |
|-----------|--------|-----------|
| Mosquitto MQTT broker | Non (si pas de MQTT) | Oui (recommandé) |
| Zigbee2MQTT + dongle Zigbee | Non | Oui (pour Zigbee) |
| miou-llm-bridge opérationnel | Oui (NLU vocal) | |
| KindMother SQLite local | Oui | |
| Réseau LAN local | Oui | |
| Connexion MWS | Non | Oui (API externe) |

#### Outillage

- Rust 1.75+, Cargo workspace existant
- Dioxus 0.6 (déjà présent)
- cpal (existant), rumqttc (nouveau), axum (déjà présent dans workspace)

---

## 5. Plan de projet

### 5.1 Phases et jalons

| Phase | Description | Jalon | Durée estimée | Dépendances |
|-------|-------------|-------|---------------|-------------|
| 0 | Analyse et préparation | Rapport fondateur validé | 1 semaine | — |
| 1 | Renommage et refactoring | Code renommé, tests verts | 1 semaine | Phase 0 |
| 2 | Crates domotique backend | `miyualicia-devices`, `miyualicia-mqtt`, `miyualicia-http` compilent + testés | 2 semaines | Phase 1 |
| 3 | Orchestrateur Alicia | `miyualicia` avec NLU pipeline intégré | 2 semaines | Phase 2 |
| 4 | API REST sécurisée | `miyualicia-api` opérationnel, JWT, endpoints principaux | 1.5 semaines | Phase 3 |
| 5 | Moteur d'automatisations | `miyualicia-automations`, scénarios TOML | 1.5 semaines | Phase 3 |
| 6 | UI Dioxus refonte | Tous les écrans Alicia fonctionnels | 2 semaines | Phase 3 |
| 7 | Intégration COG | miou-llm-bridge NLU, miyunotify, KindMother final | 1 semaine | Phases 4, 5, 6 |
| 8 | Tests, audit George, corrections | Zéro régression, conformité COG | 1.5 semaines | Phase 7 |
| 9 | Documentation finale + archivage | Arianne archive, Denis finalise docs | 0.5 semaine | Phase 8 |

**Durée totale estimée :** 13.5 à 17 semaines (fourchette pessimiste incluant corrections George)

### 5.2 Distribution des tâches

| Agent | Phase(s) | Responsabilité | Livrables |
|-------|----------|----------------|-----------|
| Denis | 0, 1, 7, 8, 9 | Doc technique, plan détaillé, coordination, tests finaux | Spec API, checklist implémentation, rapport tests |
| Fabrice | 0 | Analyse concurrence domotique (Home Assistant, Gladys, Jeedom) | Rapport positionnement concurrentiel |
| Francois | 2, 3, 4, 5, 7 | Crates backend, API, DB, MQTT, NLU pipeline | miyualicia*, tests unitaires, migrations DB |
| Lise | 6, 7 | UI Dioxus : refonte écrans Alicia, nouveaux panneaux | dashboard, devices, automations, parametres |
| George | 8 | Audit conformité COG, sécurité API, UX | Rapport audit, liste de corrections |
| Arianne | 9 | Archivage, mémoire, anti-hallucination | Archives MIP, index documentation |
| Maria | Continu | Suivi, blocages, priorisation | Ce rapport, mises à jour jalons |

---

## 6. Risques et mitigations

| Risque | Probabilité | Impact | Mitigation |
|--------|-------------|--------|------------|
| miou-llm-bridge NLU insuffisant pour intentions domotiques | Moyen | Élevé | Fallback : parsing regex sur transcription STT pour P0 |
| Incompatibilité cpal sur certains systèmes audio | Faible | Moyen | Existant depuis Phase 1 MiyukiniVoice, déjà validé |
| Broker MQTT absent chez utilisateur | Élevé | Moyen | Alicia démarre sans MQTT (mode HTTP-only dégradé) |
| Latence STT > 1.5s (modèle LLM lent) | Moyen | Moyen | Timeout configurable, réponse non-bloquante UI |
| Sécurité API REST : token volé via réseau local | Faible | Élevé | TLS obligatoire pour API si MWS distant, JWT courte durée 1h |
| Complexité automatisations TOML : erreurs utilisateur | Élevé | Faible | Validation schema TOML stricte + messages d'erreur clairs UI |
| Drift des états dispositifs (perte sync MQTT) | Moyen | Moyen | Reconciliation périodique toutes les 30s, indicateur "état inconnu" |
| Scope creep Phase 2 (fonctionnalités non prévues) | Élevé | Moyen | Maria maintient liste OUT stricte, tout nouveau besoin → Phase 3 |
| Dioxus 0.6 nested braces RSX | Élevé | Faible | Conventions CLAUDE.md appliquées, variables locales obligatoires |

---

## 7. Budget et ressources

### 7.1 Coûts estimés (fourchette)

| Poste | Optimiste | Pessimiste |
|-------|-----------|------------|
| Temps développement Francois (backend) | 6 semaines | 9 semaines |
| Temps développement Lise (frontend) | 2 semaines | 3.5 semaines |
| Temps Denis (doc + coordination) | 2 semaines | 3 semaines |
| Temps George (audit) | 1 semaine | 1.5 semaines |
| Temps Fabrice (analyse) | 3 jours | 5 jours |
| Temps Maria (suivi) | Continue | Continue |
| Infrastructure (Mosquitto, dongle Zigbee) | 30 EUR | 80 EUR |
| Crates Rust tiers (tous open source) | 0 EUR | 0 EUR |
| Total hors infrastructure logicielle | 11.5 semaines | 18 semaines |

### 7.2 Risque budget

- Le NLU via miou-llm-bridge est le poste d'incertitude le plus élevé : si le LLM local est trop lent ou imprécis, il faudra développer un parser d'intentions custom (NLU simplifié basé sur regex + slots), ce qui représente +1 à +2 semaines Francois.

---

## 8. Architecture proposée — Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────┐
│                    Alicia Home Assistante                        │
│                    (Strate 7 — Service)                         │
│                                                                  │
│  ┌─────────────────┐    ┌──────────────────────────────────┐   │
│  │   UI Dioxus 0.6 │    │         miyualicia               │   │
│  │  apps/central   │◄──►│   Orchestrateur principal        │   │
│  │                 │    │   - NLU pipeline vocal            │   │
│  │  - Dashboard    │    │   - Dispatcher commandes          │   │
│  │  - Devices      │    │   - Moteur état maison            │   │
│  │  - Automations  │    │   - Bridge miou-llm-bridge        │   │
│  │  - Parametres   │    └──────────────┬───────────────────┘   │
│  └─────────────────┘                   │                        │
│                                        │                        │
│  ┌──────────────┐   ┌────────────┐    │    ┌────────────────┐  │
│  │ miyualicia-  │   │ miyualicia-│    │    │  miyualicia-   │  │
│  │     api      │   │   mqtt     │◄───┤    │ automations    │  │
│  │ (axum REST   │   │(rumqttc +  │    │    │ (scheduler +   │  │
│  │  JWT, :7890) │   │ Zigbee2MQ) │    │    │  évaluateur)   │  │
│  └──────┬───────┘   └─────┬──────┘    │    └───────┬────────┘  │
│         │                 │           │             │            │
│  ┌──────────────┐   ┌────────────┐    │    ┌────────────────┐  │
│  │  miyualicia- │   │ miyualicia-│    │    │   miyualicia-  │  │
│  │   devices    │   │   http     │◄───┘    │    devices     │  │
│  │  (registre)  │   │(dispositifs│         │  (partagé)     │  │
│  │              │   │ HTTP loc.) │         │                │  │
│  └──────────────┘   └────────────┘         └────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Crates existants (inchangés)                 │   │
│  │   miyuvoicecapture (cpal, VAD)  |  miyuwakeword (rpw)    │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                  COG Strates 0-5                          │   │
│  │  KindMother (SQLite)  |  miyunotify  |  miou-llm-bridge  │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
          │ API REST JWT
          ▼
┌─────────────────────────┐
│  Serveurs MWS distants  │
│  (contrôle à distance)  │
└─────────────────────────┘
          │ MQTT / HTTP local
          ▼
┌─────────────────────────┐
│  Dispositifs domotiques  │
│  (Zigbee, WiFi, MQTT)   │
└─────────────────────────┘
```

---

## 9. Suivi d'avancement

_Sera mis à jour à chaque sprint._

| Date | Phase | Statut | Notes |
|------|-------|--------|-------|
| 2026-03-01 | 0 | En cours | Rapport fondateur rédigé par Maria |

---

## 10. Décisions de conception verrouillées

Ces décisions sont prises et ne seront pas remises en question sans validation explicite de Maria :

1. **Pas de cloud domotique** : Alicia est 100 % local, le réseau MWS est optionnel
2. **MQTT comme protocole principal** : Mosquitto local, pas de cloud MQTT (HiveMQ, EMQX cloud)
3. **JWT HS256** pour l'API REST (clé symétrique locale, plus simple que RSA pour usage interne MWS)
4. **KindMother SQLite** pour toute persistance (pas de Redis, pas de fichiers JSON libres)
5. **miou-llm-bridge** pour le NLU, pas de service cloud LLM
6. **Renommage complet** : plus aucune référence à "MiyukiniVoice" dans les strings utilisateur (les noms de crates techniques peuvent être renommés progressivement)
7. **Port 7890** pour l'API REST Alicia (à documenter dans le registre des ports COG)
8. **Automatisations TOML** stockées en DB, pas en fichiers (pour versionning et modification à chaud)

---

## 11. Transmission aux agents

**Denis** reçoit ce rapport pour produire :
- La spécification technique détaillée de chaque nouveau crate
- Le plan dev détaillé (voir fichier annexe)
- La checklist d'implémentation pour Francois et Lise

**Fabrice** reçoit ce rapport pour :
- Analyse comparative Home Assistant / Gladys Assistant / Jeedom / Domoticz
- Identification des fonctionnalités différenciantes d'Alicia

**Arianne** archive :
- Ce rapport fondateur v1.0 (2026-03-01)
- Décision : passage de MiyukiniVoice à Alicia Home Assistante
- Décisions de conception verrouillées (section 10)

---

*Maria — Chef de Projet Miyukini AI Studio — 2026-03-01*
