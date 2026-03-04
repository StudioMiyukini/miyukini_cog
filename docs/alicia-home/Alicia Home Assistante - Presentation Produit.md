# Alicia Home Assistante -- Presentation Produit

<!-- @id: doc.alicia-home.presentation-produit -->
<!-- @role: product-review -->
<!-- @layer: governance -->
<!-- @human: Presentation produit complete d'Alicia Home Assistante, analyse PR par Fabrice -->
<!-- @do: present_alicia_home_product_for_stakeholders -->

**Auteur :** Fabrice, Analyste PR -- Miyukini AI Studio
**Date :** 2026-03-01
**Version :** 1.0
**Statut :** Valide

---

## Table des matieres

1. [Pitch](#1-pitch)
2. [Vision et positionnement](#2-vision-et-positionnement)
3. [Fonctionnalites cles](#3-fonctionnalites-cles)
4. [Architecture technique](#4-architecture-technique)
5. [Cibles utilisateurs](#5-cibles-utilisateurs)
6. [Stack technique](#6-stack-technique)
7. [Analyse qualites / defauts](#7-analyse-qualites--defauts)
8. [Audit concurrentiel](#8-audit-concurrentiel)
9. [Roadmap suggeree](#9-roadmap-suggeree)
10. [Chiffres cles](#10-chiffres-cles)

---

## 1. Pitch

**Alicia Home Assistante** est un assistant domotique local souverain, pilote par la voix, qui controle les equipements de la maison (lumieres, volets, thermostats, prises, capteurs, serrures) sans jamais envoyer une seule donnee vers le cloud. Concu pour les familles qui veulent une maison connectee respectueuse de leur vie privee, Alicia fonctionne a 100 % hors-ligne grace a un moteur vocal embarque ("Hey Alicia"), une intelligence de comprehension locale (NLU via LLM local), et un systeme d'automatisations configurable sans code. Il s'integre nativement dans l'ecosysteme COG Miyukini et expose une API REST securisee (JWT) pour un controle a distance depuis les serveurs MWS de la famille.

---

## 2. Vision et positionnement

### 2.1 Philosophie : souverainete numerique au foyer

Alicia repose sur un principe fondateur inflexible : **aucune dependance externe critique a l'execution**. Ce principe, issu des Lois d'Autonomie du framework COG Miyukini, se traduit concretement par :

- **Pas de cloud obligatoire.** Le micro, la detection vocale, la transcription audio (STT) et la comprehension de langue naturelle (NLU) tournent integralement sur la machine locale.
- **Pas de compte utilisateur externe.** Pas de login Google, Amazon ou Apple. Le systeme demarre et fonctionne avec zero connexion Internet.
- **Les donnees restent chez vous.** Les echantillons audio ne quittent jamais le reseau local. Les transcriptions vocales ne sont pas persistees. Seules les intentions finales et les commandes executees sont tracees dans la base locale (SQLite KindMother).
- **Le reseau distant est un complement, jamais une dependance.** L'API REST, les notifications MWS et la meteo sont des modules optionnels qui enrichissent l'experience sans conditionner le fonctionnement.

### 2.2 Integration dans l'ecosysteme COG Miyukini

Alicia n'est pas un produit isole : c'est un **service de Strate 7** dans la pyramide COG. Il beneficie de :

| Composant COG | Role pour Alicia |
|---|---|
| **miou-llm-bridge** | Transcription audio (STT Whisper local) et comprehension d'intentions (NLU via LLM local) |
| **KindMother** | Base de donnees SQLite locale gouvernee : dispositifs, etats, historique, automatisations, tokens API |
| **miyunotify** | Notifications push internes COG (evenements domotiques) |
| **MiyukiniWatch** | Monitoring systeme (charge CPU, memoire, sante des services) |
| **Central** | Application desktop Dioxus 0.6 qui heberge l'interface Alicia (5 onglets) |
| **Market (miyumarket)** | Catalogue des services ; Alicia y est enregistree comme service officiel |

### 2.3 Differenciation vs la concurrence

| Critere | Alicia | Home Assistant | Gladys Assistant | Google Home / Alexa |
|---|---|---|---|---|
| **Vie privee** | 100 % local, zero cloud obligatoire | Local par defaut, cloud optionnel | Local, cloud optionnel (Gladys Plus) | Cloud obligatoire, collecte massive |
| **Controle vocal** | Wake word embarque (Rustpotter), STT/NLU local | Necessite add-on (Whisper + Rhasspy/Piper) | Pas de vocal natif | Vocal cloud (Google/Amazon) |
| **Langage** | Rust (performance, securite memoire, forbid unsafe) | Python (plus lent, plus de RAM) | Node.js/JavaScript | Proprietaire |
| **Ecosysteme integre** | COG natif (LLM, DB, Market, Monitoring) | Standalone (add-ons a installer) | Standalone (plugins) | Ecosysteme ferme |
| **Configuration** | TOML + UI visuelle | YAML + UI (config complexe) | UI simple (pas de fichiers) | App mobile uniquement |
| **Cout** | Gratuit, open source interne | Gratuit, open source | Gratuit, open source | Gratuit mais donnees monetisees |
| **Cible** | Famille Miyukini, utilisateurs souverainistes | Enthousiastes tech avances | Grand public tech-friendly | Grand public |

---

## 3. Fonctionnalites cles

### 3.1 Controle vocal "Hey Alicia"

**Ce que ca fait :** L'utilisateur dit "Hey Alicia, allume la lumiere du salon" et la lumiere s'allume. Tout se passe en local : detection du mot-cle, capture audio, transcription en texte, comprehension de l'intention, execution de la commande.

**Comment c'est implemente :**

| Etape | Crate | Technologie |
|---|---|---|
| Capture audio micro | `miyualicia-capture` | cpal (multi-plateforme), ring buffer SPSC, 16kHz mono |
| Detection d'activite vocale (VAD) | `miyualicia-capture` | Analyse RMS avec debounce et rejection de faux positifs |
| Detection wake word "Hey Alicia" | `miyualicia-wakeword` | Rustpotter (modele CPU-only, pas de GPU requis) |
| Transcription audio vers texte (STT) | `miyualicia` via `miou-llm-bridge` | Whisper local (modele base, francais) |
| Comprehension d'intention (NLU) | `miyualicia` via `miou-llm-bridge` | LLM local avec prompt domotique contextualise |
| Fallback NLU (si LLM indisponible) | `miyualicia` (nlu_fallback.rs) | Parser regex compile (LazyLock), couvre les commandes courantes |

**Exemple d'usage :**
- "Hey Alicia, mets le chauffage a 20 degres" -- Alicia reconnait l'intention `ControlDevice{thermostat, set_temperature, 20.0}` et envoie la commande via MQTT au thermostat.
- "Hey Alicia, bonne nuit" -- Alicia reconnait l'intention `ActivateRoutine{bonne nuit}` et execute la routine : extinction des lumieres, thermostat a 18 degres C, verrouillage de la porte.

**Taxonomie des intentions reconnues :**

| Intention | Exemples de phrases | Action |
|---|---|---|
| `ControlDevice` | "Allume la lumiere", "Ferme les volets", "Regle le thermostat a 22" | Commande domotique directe |
| `ActivateRoutine` | "Bonne nuit", "Je pars", "Mode cinema" | Declenchement d'une routine nommee |
| `QueryState` | "Quelle est la temperature du salon ?" | Lecture d'etat (reponse Phase 2) |
| `QueryWeather` | "Quel temps fait-il ?" | Meteo locale (Phase 7) |
| `Help` | "Qu'est-ce que tu sais faire ?" | Aide contextuelle |
| `Unknown` | "Commande une pizza" | Fallback : "Je n'ai pas compris" |

---

### 3.2 Domotique locale (dispositifs, MQTT, Zigbee2MQTT, HTTP)

**Ce que ca fait :** Alicia controle les equipements physiques de la maison via trois protocoles locaux. Elle maintient un registre en temps reel de tous les dispositifs et de leur etat.

**Types de dispositifs supportes :**

| Type | Capacites | Exemples physiques |
|---|---|---|
| **Lumiere** (Light) | On/off, variateur 0-100%, couleur RGB | Ampoules Zigbee, LED strips, spots |
| **Volet** (Shutter) | Ouvrir, fermer, position 0-100% | Volets roulants Zigbee, stores motorises |
| **Thermostat** | Temperature cible, mode chauffage/refroidissement/veille | Tetes thermostatiques Zigbee, thermostats connectes |
| **Prise** (Outlet) | On/off, mesure consommation (watts) | Prises connectees Zigbee, TP-Link Tapo |
| **Capteur** (Sensor) | Temperature, humidite, mouvement, contact porte/fenetre, luminosite | Capteurs Aqara, Sonoff |
| **Serrure** (Lock) | Verrouiller/deverrouiller, statut | Serrures connectees Zigbee |

**Protocoles de communication :**

| Protocole | Usage | QoS | Crate |
|---|---|---|---|
| **MQTT** (Mosquitto local) | Protocole principal, topiques `alicia/home/{room}/{device}/command` | QoS 1 (commandes), QoS 0 (etats) | `miyualicia-mqtt` (rumqttc async) |
| **Zigbee2MQTT** | Bridge Zigbee local, decouverte automatique des dispositifs | Via MQTT | `miyualicia-mqtt` (module zigbee2mqtt) |
| **HTTP local** | Dispositifs WiFi a API locale (Shelly, Tapo) | Timeout 2s, 1 retry | `miyualicia-http` (reqwest) |

**Exemple d'usage :** L'utilisateur possede un bridge Zigbee2MQTT avec 3 ampoules, 2 capteurs et un thermostat. Au demarrage, Alicia decouvre automatiquement les dispositifs depuis le topic `zigbee2mqtt/bridge/devices`, les enregistre dans le registre local, et commence a recevoir les mises a jour d'etat en temps reel. L'utilisateur peut alors les controler vocalement ou via l'interface.

---

### 3.3 API REST securisee (JWT, endpoints, acces MWS distant)

**Ce que ca fait :** Alicia expose une API HTTP/JSON sur le port 7890 (configurable) qui permet de consulter l'etat de la maison et d'envoyer des commandes depuis n'importe quel serveur MWS de la famille Miyukini, ou tout client autorise.

**Comment c'est implemente :**

| Composant | Technologie |
|---|---|
| Serveur HTTP | axum 0.8, lance dans `tokio::spawn` |
| Authentification | JWT HS256, cle locale generee par `OsRng`, stockee chiffree dans KindMother |
| Rate limiting | 100 requetes/minute par IP |
| Corps limite | 64 Ko maximum (protection DoS) |
| CORS | Desactive par defaut, activable dans `alicia.toml` |

**Table des endpoints :**

| Methode | Chemin | Auth | Description |
|---|---|---|---|
| `POST` | `/api/v1/alicia/auth/token` | Non | Obtenir un token JWT (client_id + secret) |
| `GET` | `/api/v1/alicia/health` | Non | Sante du service (version, uptime, statut MQTT) |
| `GET` | `/api/v1/alicia/state` | JWT Read | Etat complet de la maison |
| `GET` | `/api/v1/alicia/rooms` | JWT Read | Liste des pieces avec dispositifs |
| `GET` | `/api/v1/alicia/rooms/{id}` | JWT Read | Etat d'une piece |
| `GET` | `/api/v1/alicia/devices` | JWT Read | Tous les dispositifs |
| `GET` | `/api/v1/alicia/devices/{id}` | JWT Read | Etat d'un dispositif |
| `POST` | `/api/v1/alicia/devices/{id}/command` | JWT Write | Envoyer une commande |
| `GET` | `/api/v1/alicia/automations` | JWT Read | Liste des automatisations |
| `POST` | `/api/v1/alicia/automations` | JWT Automations | Creer une automatisation |
| `PUT` | `/api/v1/alicia/automations/{id}` | JWT Automations | Modifier une automatisation |
| `DELETE` | `/api/v1/alicia/automations/{id}` | JWT Automations | Supprimer une automatisation |
| `POST` | `/api/v1/alicia/automations/{id}/trigger` | JWT Automations | Declenchement manuel |
| `GET` | `/api/v1/alicia/history` | JWT History | Journal des commandes (pagine) |

**Scopes JWT disponibles :** `Read`, `Write`, `Automations`, `History`, `Admin` (acces total).

**Exemple d'usage :** Un serveur MWS Miyukini situe chez un autre membre de la famille envoie `POST /api/v1/alicia/devices/{id}/command` avec un token JWT scope `Write` pour allumer la lumiere du salon a distance. La commande est loguee dans l'audit trail avec source="api" et l'adresse IP du client.

---

### 3.4 Automatisations (triggers, conditions, actions, routines vocales)

**Ce que ca fait :** L'utilisateur definit des scenarios "si X alors Y" : par exemple, "tous les soirs a 23h, si aucun mouvement dans le salon, eteindre les lumieres". Les automatisations se configurent en TOML, via l'API ou via l'interface visuelle.

**Types de declencheurs :**

| Declencheur | Description | Exemple |
|---|---|---|
| **Cron** | Expression cron horaire (6 champs) | `"0 0 22 * * *"` = tous les soirs a 22h |
| **SensorChange** | Changement d'etat d'un capteur | Temperature > 25 degres C, mouvement detecte |
| **VoiceCommand** | Routine vocale nommee | "Hey Alicia, bonne nuit" |
| **ApiEvent** | Declenchement via API REST | `POST /automations/{id}/trigger` |

**Conditions (ET logique) :**

| Operateur | Exemple |
|---|---|
| `eq` | motion == false |
| `gt`, `gte` | temperature_current >= 25 |
| `lt`, `lte` | humidity < 30 |
| `between` | hour Between [22, 6] |
| `ne` | locked != true |

**Actions sequentielles avec delai :**

```toml
[[automations]]
name = "Bonne nuit"
enabled = true

[automations.trigger]
type = "voice_command"
routine_name = "bonne nuit"

conditions = []

[[automations.actions]]
device_id = "uuid-lumiere-salon"
command = "off"
delay_ms = 0

[[automations.actions]]
device_id = "uuid-thermostat-chambre"
command = "set_temperature"
value = 18.0
delay_ms = 500

[[automations.actions]]
device_id = "uuid-serrure-entree"
command = "lock"
delay_ms = 1000
```

**Securite du moteur :** Les actions ne peuvent executer que des commandes connues du registre de dispositifs. Pas de shell, pas d'evaluation de code dynamique, pas de scripts arbitraires. Les conditions sur un etat inconnu sont evaluees `false` (principe de securite : l'inaction est preferable a une action non souhaitee).

---

### 3.5 Integration COG (LLM local, services Miyukini)

**Ce que ca fait :** Alicia s'appuie sur les briques existantes de l'ecosysteme COG pour enrichir ses capacites sans reinventer la roue.

| Service COG | Integration Alicia | Optionnel |
|---|---|---|
| **miou-llm-bridge** | STT (Whisper), NLU (comprehension d'intentions), generation de reponses | Oui (fallback regex si absent) |
| **KindMother** | Persistance : 5 tables SQLite (devices, states, commands_log, automations, api_tokens) | Non (requis) |
| **miyunotify** | Notifications push internes (evenements domotiques critiques) | Oui |
| **MiyukiniWatch** | Monitoring de la sante du service Alicia | Oui |
| **Market** | Enregistrement dans le catalogue officiel des services Central | Non (integre) |

**Fallback intelligent :** Si `miou-llm-bridge` est indisponible (service eteint, modele LLM pas encore charge), Alicia bascule automatiquement sur un parser NLU a base de regex compiles. Ce mode degrade couvre les commandes les plus courantes : allumer/eteindre lumieres, regler thermostat, ouvrir/fermer volets, activer routines nommees. La transition est transparente et loguee.

---

### 3.6 Interface desktop Dioxus (5 onglets, monitoring temps reel)

**Ce que ca fait :** L'interface Alicia est integree dans l'application desktop Central (Dioxus 0.6). Elle offre 5 onglets pour visualiser et controler toute la maison.

| Onglet | Contenu |
|---|---|
| **Tableau de bord** | Resume global : pieces actives, dispositifs connectes, statut MQTT, mot-cle, zone audio (VAD/RMS par piece), journal d'activite, widget meteo (Phase 7) |
| **Pieces** | Vue par piece avec les dispositifs et leur etat, commandes rapides |
| **Dispositifs** | Liste complete de tous les dispositifs avec type, protocole, adresse, etat courant |
| **Automatisations** | Liste des automatisations, activation/desactivation, editeur visuel |
| **Parametres** | Configuration API REST, tokens MWS, MQTT, wake word, preferences |

**Composants UI notables :**
- `StatusBadge` dynamique : "Alicia ecoute" (vert) ou "En veille" (gris)
- `AudioRoomCard` par piece : indicateur micro, VAD (voix detectee/silence), barre RMS temps reel, dernier "Hey Alicia" detecte
- `SummaryCard` : compteurs avec accent couleur (pieces actives, dispositifs, MQTT)
- Polling 200ms pour le rafraichissement des donnees audio et d'etat

**Cohesion visuelle :** L'interface respecte le theme COG Miyukini via `current_theme.palette()`, partage avec MiyukiniWatch et les autres services Central.

---

## 4. Architecture technique

### 4.1 Vue d'ensemble (schema ASCII)

```
+-------------------------------------------------------------------+
|                     Alicia Home Assistante                         |
|                     (COG Strate 7 -- Service)                      |
|                                                                    |
|  +------------------+    +------------------------------------+    |
|  |  UI Dioxus 0.6   |    |          miyualicia                |    |
|  |  (Central)       |<-->|   Orchestrateur principal           |    |
|  |                  |    |   - Pipeline NLU vocal              |    |
|  |  - Dashboard     |    |   - Dispatcher commandes            |    |
|  |  - Pieces        |    |   - Moteur etat maison              |    |
|  |  - Dispositifs   |    |   - Bridge miou-llm-bridge          |    |
|  |  - Automatisations|   |   - NLU fallback regex              |    |
|  |  - Parametres    |    +----------+-------------------------+    |
|  +------------------+               |                              |
|                                     |                              |
|  +---------------+  +----------+    |    +--------------------+    |
|  | miyualicia-   |  |miyualicia|    |    | miyualicia-        |    |
|  |     api       |  |  -mqtt   |<---+    | automations        |    |
|  | (axum REST    |  | (rumqttc |    |    | (scheduler cron    |    |
|  |  JWT :7890)   |  |  async)  |    |    |  + evaluateur)     |    |
|  +-------+-------+  +----+-----+    |    +--------+-----------+    |
|          |               |          |              |               |
|  +---------------+  +----------+    |    +--------------------+    |
|  | miyualicia-   |  |miyualicia|    |    |   miyualicia-      |    |
|  |   devices     |  |  -http   |<---+    |    devices         |    |
|  | (registre     |  |(Shelly,  |         |  (types partages)  |    |
|  |  in-memory)   |  | Tapo)    |         |                    |    |
|  +---------------+  +----------+         +--------------------+    |
|                                                                    |
|  +------------------------------------------------------------+   |
|  |              Crates existants (heritage vocal)               |   |
|  |  miyualicia-capture (cpal, VAD)  | miyualicia-wakeword (rpw)|   |
|  +------------------------------------------------------------+   |
|                                                                    |
|  +------------------------------------------------------------+   |
|  |                    COG Strates 0-6                           |   |
|  | KindMother (SQLite) | miyunotify | miou-llm-bridge (LLM)   |   |
|  +------------------------------------------------------------+   |
+-------------------------------------------------------------------+
         | API REST JWT (:7890)
         v
+----------------------------+
|   Serveurs MWS distants    |
|   (controle a distance)    |
+----------------------------+
         | MQTT / HTTP local
         v
+----------------------------+
|  Dispositifs domotiques    |
|  (Zigbee, WiFi, MQTT)     |
+----------------------------+
```

### 4.2 Flux de donnees : de la voix a l'action

```
Micro (cpal)
    |
    v
Ring Buffer SPSC (miyualicia-capture)
    |
    v
VAD - Detection activite vocale (RMS + debounce)
    |
    v
Rustpotter - Wake word "Hey Alicia" (miyualicia-wakeword)
    |
    v
Echantillons post wake word (2-10s, 16kHz mono)
    |
    v
miou-llm-bridge /api/stt  -->  Transcription texte
    |                               |
    |  (si bridge indisponible)     v
    |                          miou-llm-bridge /api/nlu
    v                               |
Fallback regex (nlu_fallback.rs)    v
    |                          Intent structure (JSON)
    +-------------------------------+
    |
    v
Orchestrateur miyualicia
    |
    +---> DeviceCommand (MQTT ou HTTP local)
    |         |
    |         v
    |    Dispositif physique
    |
    +---> ActivateRoutine (AutomationEngine)
    |         |
    |         v
    |    Sequence d'actions (dispositifs multiples)
    |
    +---> Audit trail (KindMother alicia_commands_log)
```

---

## 5. Alicia — L'agent

### 5.1 Qui est Alicia ?

Alicia n'est pas un produit destine a des "utilisateurs types". **Alicia est un agent IA** — une assistante personnelle bienveillante qui agit comme une gouvernante intelligente au service de l'utilisateur, de sa famille et de son cercle social.

**Definition :** Alicia est une assistante personnelle bienveillante qui gere le quotidien direct. Elle est capable d'appeler n'importe quel autre agent de l'ecosysteme COG et de trouver les informations pertinentes pour l'utilisateur. Elle communique par voix avec l'utilisateur.

### 5.2 Responsabilites d'Alicia

| Domaine | Capacites |
|---|---|
| **Gestion du domicile** | Controle domotique (lumieres, volets, thermostats, capteurs, serrures), surveillance des pieces, automatisations, routines |
| **Surveillance proactive** | Observe l'utilisateur via MiyukiniWatch et les donnees personnelles pour interagir de facon pertinente. Prend l'initiative de faire des propositions jugees utiles |
| **Gestion familiale** | Garante des donnees de l'utilisateur, de sa famille et de son cercle social. Gere les donnees personnelles sensibles a sa disposition |
| **Actions IRL** | Prise de rendez-vous, correspondance electronique, rappel de prise de medicaments, reassurance sur le quotidien |
| **Adaptation psychologique** | S'adapte au profil psychologique de l'utilisateur en se basant sur les etudes psychologiques |
| **Planification** | Planifie et suit les taches de l'utilisateur pour l'aider au quotidien |
| **Orchestration d'agents** | Peut invoquer Maria (projets), Denis (technique), Fabrice (analyse), Julie (formation), et tout autre agent COG si la situation l'exige |
| **Memoire** | Maintient sa memoire persistante avec Arianne |
| **Communication vocale** | Ecoute ("Hey Alicia"), comprend (NLU), repond (TTS prevu Phase 2), et agit |

### 5.3 Le module Home dans Alicia

**Alicia Home Assistante** est le module de gestion du domicile d'Alicia. C'est le volet domotique de ses responsabilites plus larges. Ce module lui permet de :

- **Controler la maison** — lumieres, volets, chauffage, prises, serrures
- **Surveiller le domicile** — capteurs de temperature, humidite, mouvement, ouverture
- **Automatiser le quotidien** — routines "Bonne nuit", "Je pars", alertes temperature
- **Etre accessible a distance** — API REST securisee pour les serveurs MWS de la famille
- **S'integrer aux services COG** — LLM local pour la comprehension, KindMother pour la persistance, notifications

### 5.4 Exemples de scenarios Alicia

| Scenario | Ce qu'Alicia fait |
|---|---|
| "Hey Alicia, bonne nuit" | Execute la routine : eteint les lumieres, baisse le thermostat a 18 degres, verrouille la porte, et dit "Bonne nuit" |
| Temperature < 16 degres dans la chambre de Theresa | Alicia le detecte via le capteur, remonte le chauffage, et previent l'utilisateur via notification |
| L'utilisateur est au travail | Depuis le serveur MWS distant, Alicia repond aux requetes API : etat de la maison, controle a distance |
| "Hey Alicia, rappelle-moi de prendre mes medicaments a 20h" | Alicia planifie le rappel et le declenche a l'heure dite (integration MiyukiniWatch) |
| "Hey Alicia, qu'est-ce que tu sais faire ?" | Alicia liste ses capacites actuelles et propose des suggestions adaptees au profil de l'utilisateur |
| Mouvement detecte la nuit alors que personne n'est cense etre la | Alicia envoie une alerte, allume les lumieres du couloir, et logue l'evenement dans l'audit trail |

---

## 6. Stack technique

### 6.1 Tableau resume

| Composant | Technologie | Version |
|---|---|---|
| **Langage** | Rust | edition 2021 |
| **UI desktop** | Dioxus | 0.6 |
| **Serveur API** | axum | 0.8 |
| **Client MQTT** | rumqttc | 0.24 |
| **Client HTTP** | reqwest | 0.12 |
| **Authentification** | jsonwebtoken (JWT HS256) | 9 |
| **Base de donnees** | SQLite via KindMother | -- |
| **Wake word** | Rustpotter | -- |
| **Capture audio** | cpal | -- |
| **Scheduler** | tokio-cron-scheduler | 0.10 |
| **Serialisation** | serde + serde_json + toml | 1 / 1 / 0.8 |
| **Erreurs** | thiserror | 2 |
| **Logs** | tracing | 0.1 |
| **UUIDs** | uuid v4 | 1 |
| **Dates** | chrono | 0.4 |
| **Regex (NLU fallback)** | regex | 1 |

### 6.2 Metriques du code

| Metrique | Valeur |
|---|---|
| Nombre de crates Alicia | **8** |
| Fichiers Rust (.rs) | **54** (crates) + **7** (UI) = **61 total** |
| Lignes de code Rust (crates) | **~10 500** |
| Lignes de code Rust (UI) | **~2 400** |
| **Total lignes de code** | **~12 900** |
| Tests unitaires | **279** (100 % pass) |
| Clippy warnings | **0** (pedantic, zero tolerance) |
| Occurrences `unwrap()` hors tests | **0** |
| Occurrences `unsafe` | **0** (forbid dans les 8 crates) |
| Annotations MSCM | **8/8 crates** complets (@id, @do, @role, @layer) |

---

## 7. Analyse qualites / defauts

### 7.1 Points forts

1. **Souverainete totale des donnees** -- Aucune donnee ne quitte le reseau local. Conforme RGPD par design. Pas de compte cloud, pas de tracking, pas de monetisation des donnees vocales.

2. **Architecture modulaire exemplaire** -- 8 crates independants, chacun avec sa responsabilite claire, ses types d'erreur explicites, ses tests. Un nouveau developpeur peut comprendre un crate sans connaitre les autres.

3. **Pipeline vocal natif complet** -- De la capture micro a l'execution de la commande, tout est en Rust. Pas besoin d'assembler 5 projets open source differents comme avec Home Assistant (Whisper + Rhasspy + Piper + Node-RED + ...).

4. **Securite rigoureuse** -- `forbid(unsafe_code)` dans tous les crates, zero `unwrap()` en production, JWT non-hardcode, rate limiting, audit trail complet, credentials chiffres via KindMother.

5. **Fallback intelligent** -- Si le LLM local est lent ou indisponible, le NLU bascule automatiquement sur un parser regex sans interruption de service. Si le broker MQTT est absent, Alicia demarre en mode HTTP-only.

6. **Qualite de code validee par audit** -- Score George : 96/100. 279 tests, 0 echec. Clippy pedantic sans warning. Annotations MSCM completes.

7. **API REST bien concue** -- 15 endpoints, scopes JWT fins (Read, Write, Automations, History, Admin), pagination de l'historique, rate limiting par IP, CORS configurable.

8. **Automatisations puissantes et securisees** -- 4 types de declencheurs, 7 operateurs de condition, actions sequentielles avec delai, validation stricte, pas d'execution de code arbitraire.

9. **Integration ecosysteme COG native** -- Pas un produit isole : profite du LLM local, de la DB gouvernee, du monitoring, du market, des notifications.

### 7.2 Points faibles / limites actuelles

1. **Pas de TTS (synthese vocale)** -- En Phase 1, Alicia ne "repond" pas vocalement. Les reponses sont uniquement affichees dans l'UI et loguees. La synthese vocale locale (Phase 2) est necessaire pour une experience complete.

2. **Francais uniquement** -- Le NLU (bridge et fallback regex) ne supporte que le francais. Le support multilingue est prevu en Phase 2 mais represente un effort significatif.

3. **Pas d'application mobile** -- L'interface est desktop uniquement (Dioxus 0.6 desktop). L'acces mobile passe par l'API REST, ce qui necessite un client tiers ou une future PWA.

4. **Dependance a du materiel specifique** -- Le dongle Zigbee et le bridge Zigbee2MQTT ne sont pas fournis. L'utilisateur doit les installer et les configurer manuellement avant de configurer Alicia.

5. **Nombre d'integrations limite** -- Contrairement a Home Assistant (1900+ integrations), Alicia supporte 3 protocoles (MQTT, Zigbee2MQTT, HTTP local). Pas de Z-Wave, pas de Matter, pas de Thread.

6. **Polling UI a 200ms** -- Le rafraichissement de l'interface se fait par polling toutes les 200ms. Un systeme push (channel) serait plus efficace et reduirait la charge CPU.

7. **Pas de WebSocket temps reel** -- L'API REST ne propose pas encore de flux temps reel. Les clients distants doivent polluer l'endpoint `/state` regulierement pour suivre les changements.

### 7.3 Axes d'amelioration futurs

| Axe | Priorite | Effort estime |
|---|---|---|
| TTS local (reponses vocales Alicia) | Haute | Moyen |
| Application mobile / PWA | Haute | Eleve |
| WebSocket temps reel (`/stream`) | Moyenne | Moyen |
| Support multilingue NLU | Moyenne | Eleve |
| Protocole Matter / Thread | Moyenne | Eleve |
| Protocole Z-Wave | Basse | Moyen |
| Doc-tests dans les crates publics | Basse | Faible |
| UI push (remplacement polling 200ms) | Basse | Moyen |

---

## 8. Audit concurrentiel

### 8.1 Comparatif detaille

| Critere | Alicia Home | Home Assistant | Gladys Assistant | Jeedom |
|---|---|---|---|---|
| **Respect vie privee** | Excellent -- 100 % local par design, zero cloud, zero tracking | Tres bon -- local par defaut, cloud optionnel (Nabu Casa) | Tres bon -- local, Gladys Plus optionnel (chiffre E2E) | Bon -- local, mais plugins payants via cloud Jeedom |
| **Facilite d'installation** | Moyenne -- necessite COG Miyukini + Mosquitto + Zigbee2MQTT | Difficile -- Raspberry Pi + config YAML + add-ons | Facile -- Docker en 1 commande, UI sans SSH | Facile -- box Jeedom Luna plug-and-play |
| **Protocoles supportes** | 3 (MQTT, Zigbee2MQTT, HTTP local) | 1900+ integrations (Z-Wave, Zigbee, Matter, Thread, WiFi, BLE...) | Zigbee, Z-Wave, MQTT, Philips Hue, Sonos... | Z-Wave, Zigbee, EnOcean, KNX, MQTT, Matter |
| **Controle vocal local** | Natif (wake word + STT + NLU) | Via add-ons (Wyoming/Whisper/Piper) | Non natif | Non natif |
| **Extensibilite** | Bonne -- crates Rust modulaires, API REST | Excellente -- 1900+ integrations, HACS community | Bonne -- plugins JavaScript, API | Bonne -- plugins (certains payants), API |
| **Cout** | Gratuit (+ materiel Zigbee ~30-80 EUR) | Gratuit (+ Nabu Casa 7.50 EUR/mois pour cloud) | Gratuit (+ Gladys Plus 9.99 EUR/mois pour acces distant) | Gratuit ou box Luna ~199 EUR (+ plugins payants) |
| **Performance** | Excellente -- Rust natif, < 1.5s voix-vers-action | Bonne -- Python, plus gourmand en RAM | Bonne -- Node.js, leger sur Raspberry Pi | Correcte -- PHP, plus lent sur gros setups |
| **Communaute** | Naissante (ecosysteme Miyukini) | Massive (GitHub 70k+ stars, forums actifs) | Moyenne (~4k stars, communaute francophone active) | Moyenne (forte en France, forums actifs) |
| **Automatisations** | TOML + API + vocales | YAML + UI + Node-RED | "Scenes" IFTTT-like dans l'UI | Editeur visuel + scenarios |
| **Langue UI/NLU** | Francais | Multilingue | Multilingue | Multilingue (forte base francophone) |

### 8.2 Positionnement strategique

Alicia ne cherche pas a concurrencer Home Assistant sur le nombre d'integrations -- c'est un combat perdu d'avance contre 1900+ connecteurs. Sa valeur se situe sur trois axes differenciants :

1. **Pipeline vocal natif integre** -- Aucun concurrent open source local n'offre un pipeline voix complet (wake word + STT + NLU) directement integre dans le produit, en Rust, fonctionnant hors-ligne.

2. **Securite et qualite de code Rust** -- `forbid(unsafe_code)`, Clippy pedantic, 279 tests, audit George 96/100. Home Assistant (Python), Gladys (JavaScript) et Jeedom (PHP) n'offrent pas ce niveau de garantie memoire et de securite a la compilation.

3. **Integration ecosysteme COG** -- Pour les utilisateurs de l'ecosysteme Miyukini, Alicia est une brique native qui beneficie du LLM local, de la DB gouvernee, du monitoring et des notifications sans installation supplementaire.

---

## 9. Roadmap suggeree

### Phase 2 (court terme -- 3 a 6 mois)

| Fonctionnalite | Description | Priorite |
|---|---|---|
| TTS local | Synthese vocale pour les reponses d'Alicia (Piper local ou equivalent Rust) | Haute |
| WebSocket `/stream` | Flux evenements temps reel pour les clients API distants | Haute |
| Support multilingue | Anglais comme deuxieme langue NLU | Moyenne |
| Meteo locale | Integration openmeteo.com (sans cle API) pour le widget meteo | Moyenne |
| UI push | Remplacer le polling 200ms par un systeme de channels push | Moyenne |
| Doc-tests | Ajouter des exemples executes comme tests dans les crates publics | Basse |

### Phase 3 (moyen terme -- 6 a 12 mois)

| Fonctionnalite | Description | Priorite |
|---|---|---|
| Application mobile PWA | Interface web responsive accessible depuis mobile via l'API REST | Haute |
| Protocole Matter | Support du nouveau standard pour les dispositifs compatibles | Haute |
| Multi-maisons | Federation COG : plusieurs instances Alicia qui communiquent | Moyenne |
| Apprentissage NLU | Enrichissement du modele NLU local avec les commandes non reconnues | Moyenne |
| Cameras IP (opt.) | Streaming video local pour les cameras IP du foyer | Basse |

### Phase 4 (long terme -- 12+ mois)

| Fonctionnalite | Description | Priorite |
|---|---|---|
| Z-Wave | Support du protocole Z-Wave via dongle | Moyenne |
| Thread / BLE | Support des protocoles basse consommation | Moyenne |
| Marketplace plugins | Systeme de plugins communautaires pour etendre les protocoles | Basse |
| Multi-utilisateurs | Profils vocaux par membre de la famille | Basse |

---

## 10. Chiffres cles

| Indicateur | Valeur |
|---|---|
| Score audit George | **96/100** |
| Crates Alicia | **8** |
| Tests unitaires | **279** (100 % pass) |
| Lignes de code Rust | **~12 900** |
| Fichiers source | **61** |
| Clippy warnings | **0** |
| `unwrap()` en production | **0** |
| `unsafe` | **0** (forbid dans 8/8 crates) |
| Protocoles domotiques | **3** (MQTT, Zigbee2MQTT, HTTP local) |
| Types de dispositifs | **6** (lumiere, volet, thermostat, prise, capteur, serrure) |
| Endpoints API REST | **15** |
| Scopes JWT | **5** (Read, Write, Automations, History, Admin) |
| Tables KindMother | **5** |
| Types de declencheurs automatisations | **4** (cron, sensor, vocal, API) |
| Intentions NLU reconnues | **6** (ControlDevice, ActivateRoutine, QueryState, QueryWeather, Help, Unknown) |
| Pieces preconfigures | **4** (salon, chambre-parentale, chambre-theresa, chambre-eleanore) |
| Latence cible voix-vers-action | **< 1.5 secondes** |
| Latence cible API REST (LAN) | **< 200 ms** |
| Port API REST | **7890** |
| Port miou-llm-bridge | **3003** |
| Port MQTT Mosquitto | **1883** |
| Anomalies audit | **4** (toutes mineures/cosmetiques) |
| Temps build (8 crates, dev) | **1m16s** |
| Temps execution des 279 tests | **~1.2s** |

---

## Sources

- Rapport Fondateur : `docs/alicia-home/Alicia Home Assistante - Rapport Fondateur.md`
- Plan Dev General : `docs/alicia-home/Alicia Home Assistante - Plan Dev General.md`
- Audit George : `docs/alicia-home/Alicia Home Assistante - Audit George.md`
- Spec devices : `docs/alicia-home/specs/miyualicia-devices-spec.md`
- Spec API : `docs/alicia-home/specs/miyualicia-api-spec.md`
- Spec MQTT : `docs/alicia-home/specs/miyualicia-mqtt-spec.md`
- Spec automations : `docs/alicia-home/specs/miyualicia-automations-spec.md`
- Spec NLU : `docs/alicia-home/specs/nlu-bridge-interface.md`
- Code source : `crates/miyualicia*/src/`, `apps/central/src/services/alicia/`
- [Gladys Assistant -- Site officiel](https://gladysassistant.com/)
- [Home Assistant -- Site officiel](https://www.home-assistant.io/)
- [Gladys vs Home Assistant -- iGeneration](https://www.igen.fr/domotique/2026/02/gladys-assistant-la-domotique-plus-avancee-que-maison-et-plus-accessible-que-home-assistant-154785)
- [Comparatif domotique open source 2025 -- pribey.com](https://pribey.com/blog/informatique/domotique-open-source)
- [Jeedom Luna Review -- Maison et Domotique](https://www.maison-et-domotique.com/en/159127-review-of-the-new-jeedom-luna-the-zigbee-zwave-matter-home-automation-box-for-less-than-e200/)
- [Home Assistant alternatives -- Product Hunt](https://www.producthunt.com/products/home-assistant/alternatives)

---

*Fabrice -- Analyste PR -- Miyukini AI Studio -- 2026-03-01*
*Document transmis a Maria pour validation, puis Denis pour exploitation technique.*
