# Miyukini Conceptual References - Miyukini Webway System Normes et Standards

## Contexte

Ce document est un **annexe conceptuel** au [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md). Il dÃ©veloppe les **normes et standards** du MWS : norme de dÃ©claration sÃ©curisÃ©e, formats de messages, protocole de dÃ©couverte, matrice des statuts et rÃ¨gles d'Ã©change entre COGs, ainsi que les exigences de conformitÃ© pour les COGs Tracker.

**Principe directeur :**

> **Les normes et standards du MWS garantissent l'interopÃ©rabilitÃ©, l'authenticitÃ© et l'intÃ©gritÃ© des annonces de prÃ©sence ; ils ne dÃ©finissent pas la gouvernance des accÃ¨s (Passeport, Permis de circulation, Visa de Connexion).**

## PortÃ©e / Scope

- **Norme de dÃ©claration sÃ©curisÃ©e (MWS)** : authentification, intÃ©gritÃ©, format unifiÃ©, limitation des abus
- **Formats de messages** : annonce de prÃ©sence, annonce de services/adresses, dÃ©claration d'hÃ©bergement de session, requÃªte de dÃ©couverte, liste de statuts
- **Ports utilisables et ports exclus** : liste normative des ports exclus (IANA 0â€“1023, ports courants web/dev/DB) pour les Ã©changes MWS
- **Protocole MWS** : types de messages, sÃ©quences, rÃ¨gles de transport (orientation)
- **Matrice des statuts** et rÃ¨gles d'Ã©change entre COGs (Webway COG List)
- **Standards de conformitÃ©** pour les COGs Tracker (exigences minimales, vÃ©rification)
- **Versionnement** et rÃ©trocompatibilitÃ© des normes

Ce document **ne couvre pas** :
- Les spÃ©cifications techniques d'implÃ©mentation (protocoles rÃ©seau, algorithmes cryptographiques dÃ©taillÃ©s) â†’ contrats ou specs techniques dÃ©diÃ©s
- La gouvernance des visites (Passeport, Permis de circulation, Visa de Connexion) â†’ voir [Connexion Inter-COG](_index.md)

---

## 1. Norme de dÃ©claration sÃ©curisÃ©e (MWS)

La **norme de dÃ©claration sÃ©curisÃ©e** s'applique Ã  toute **annonce** Ã©mise par un COG participant sur le Webway : prÃ©sence, services exposÃ©s, adresses (IP/ports), dÃ©claration d'hÃ©bergement de session. Elle doit Ãªtre **crÃ©Ã©e, publiÃ©e et appliquÃ©e** par l'Ã©cosystÃ¨me MWS.

### 1.1 Objectifs

| Objectif | Description |
|----------|-------------|
| **Authentification** | Attester l'origine de la dÃ©claration (COG identifiÃ©, non usurpation) |
| **IntÃ©gritÃ©** | Garantir que la dÃ©claration n'a pas Ã©tÃ© altÃ©rÃ©e en transit |
| **Format unifiÃ©** | Permettre l'interopÃ©rabilitÃ© et la vÃ©rification par les Trackers et les participants |
| **Limitation des abus** | DÃ©clarations conformes, sans exposition de donnÃ©es sensibles ni de gouvernance |

### 1.2 Exigences minimales

**Authentification de l'origine :**
- La dÃ©claration doit Ãªtre **signÃ©e** ou **chiffrÃ©e** par une clÃ© ou un secret associÃ© au COG Ã©metteur (ex. clÃ© dÃ©rivÃ©e de l'identitÃ© du COG, certificat, mÃ©canisme attestÃ©).
- Le rÃ©cepteur (Tracker ou autre COG) doit pouvoir **vÃ©rifier** que l'Ã©metteur est bien le COG annoncÃ© (identifiant COG cohÃ©rent avec la signature ou le mÃ©canisme d'attestation).
- Aucune donnÃ©e mÃ©tier ni secret de gouvernance ne doit Ãªtre inclus dans la dÃ©claration.

**IntÃ©gritÃ© :**
- La dÃ©claration doit inclure un **mÃ©canisme d'intÃ©gritÃ©** (ex. signature numÃ©rique, MAC, hash signÃ©) permettant de dÃ©tecter toute modification en transit.
- Le format doit Ãªtre **dÃ©terministe** (sÃ©rialisation canonique) pour que la vÃ©rification soit reproductible.

**Format unifiÃ© :**
- **SchÃ©ma** : structure des champs obligatoires et optionnels, types, contraintes (longueur, plages de valeurs).
- **Encodage** : encodage standard (ex. JSON, CBOR) et jeu de caractÃ¨res (ex. UTF-8) pour l'interopÃ©rabilitÃ©.
- **Version** : chaque dÃ©claration doit indiquer la **version de la norme** (ex. `mws_declaration_v1`) pour permettre l'Ã©volution et la rÃ©trocompatibilitÃ©.

**Limitation des abus :**
- Les champs autorisÃ©s sont **restreints** Ã  la prÃ©sence et Ã  la dÃ©couverte (identifiant COG, services, adresses, ports, type de session, mÃ©tadonnÃ©es de traÃ§abilitÃ©).
- **Interdits** : donnÃ©es utilisateur, secrets, contenu mÃ©tier, informations permettant d'usurper une gouvernance.

### 1.3 Non-conformitÃ©

- Un COG Tracker peut **refuser** d'accepter ou de relayer une dÃ©claration non conforme (systÃ¨me actif).
- Un COG participant peut **ignorer** ou **dÃ©grader** les dÃ©clarations non conformes selon sa politique locale.
- La non-conformitÃ© peut Ãªtre **signalÃ©e** dans le cadre des listes de statuts (ex. passage en Under review, Distrusted) selon les rÃ¨gles d'Ã©change (voir section 4).

---

## 2. Formats de messages MWS

Les **formats de messages** dÃ©finissent la structure des Ã©changes sur le Webway. Ils sont **normatifs** dÃ¨s lors qu'un protocole MWS les adopte.

### 2.1 Annonce de prÃ©sence (Presence Announcement)

**Usage :** un COG participant annonce sa prÃ©sence au rÃ©seau (ou Ã  un ou plusieurs Trackers).

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme (ex. `mws_declaration_v1`) |
| `type` | string | Type de message (ex. `presence_announcement`) |
| `cog_id` | string | Identifiant du COG Ã©metteur (ex. LSI ou Ã©quivalent attestÃ©) |
| `bridge_address` | object | Adresse du Bridge (voir 2.5) |
| `issued_at` | string (ISO 8601) | Horodatage d'Ã©mission (trace only) |
| `integrity` | object | MÃ©canisme d'intÃ©gritÃ© (signature, MAC, etc.) |

**Champs optionnels (orientation) :** `core_version_hint`, `valid_until`, selon politique locale.

### 2.2 Annonce de services et adresses (Service / Address Announcement)

**Usage :** un COG annonce les **services** qu'il expose et les **adresses** (IP ou nom de domaine, ports) associÃ©es.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `service_announcement` |
| `cog_id` | string | Identifiant du COG Ã©metteur |
| `services` | array | Liste d'entrÃ©es service (voir 2.5) |
| `issued_at` | string (ISO 8601) | Horodatage d'Ã©mission |
| `integrity` | object | MÃ©canisme d'intÃ©gritÃ© |

**Champs optionnels :** `valid_until`, par service : protocole (ex. TCP, UDP), niveau de sÃ©curitÃ© proposÃ©.

### 2.3 DÃ©claration d'hÃ©bergement de session (Host Session Declaration)

**Usage :** un COG HÃ©bergeur dÃ©clare qu'il **hÃ©berge une session** d'un service donnÃ© et **attend des connexions** vers lui.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `host_session_declaration` |
| `cog_id` | string | Identifiant du COG HÃ©bergeur |
| `service_id` | string | Identifiant du service (ou type de session) |
| `session_id` | string | Identifiant unique de la session (optionnel selon politique) |
| `connection_address` | object | Adresse de connexion (IP ou domaine, port) â€” voir 2.5 |
| `issued_at` | string (ISO 8601) | Horodatage d'Ã©mission |
| `integrity` | object | MÃ©canisme d'intÃ©gritÃ© |

**Champs optionnels :** `capacity_hint`, `security_level_hint`, `valid_until`, `protocol` (ex. TCP).

**RÃ¨gle :** cette dÃ©claration **ne donne aucun droit d'accÃ¨s** ; elle indique oÃ¹ se prÃ©senter pour demander un Permis de circulation (relay) ou un Visa de Connexion / accord d'hÃ´te (COG hÃ´te).

### 2.4 RequÃªte de dÃ©couverte (Discovery Request)

**Usage :** un COG (ou un Tracker) interroge le maillage pour **dÃ©couvrir** des COGs, des services ou des sessions hÃ©bergÃ©es.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme / protocole |
| `type` | string | Ex. `discovery_request` |
| `requester_cog_id` | string | Identifiant du COG demandeur |
| `query` | object | CritÃ¨res de recherche (ex. par service_id, par cog_id, liste de sessions) |
| `issued_at` | string (ISO 8601) | Horodatage |

**Champs optionnels :** `scope` (ex. un Tracker, plusieurs Trackers), `integrity` si la requÃªte doit Ãªtre authentifiÃ©e.

### 2.5 Liste de COGs avec statuts (Webway COG List / Status Update)

**Usage :** Ã©change entre COGs (ou avec les Trackers) de **listes ou mises Ã  jour de statuts** pour la sÃ©curitÃ© du maillage.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `cog_list` ou `status_update` |
| `sender_cog_id` | string | Identifiant du COG Ã©metteur |
| `entries` | array | Liste d'entrÃ©es (cog_id, status, source, updated_at) â€” voir section 4 |
| `issued_at` | string (ISO 8601) | Horodatage |
| `integrity` | object | MÃ©canisme d'intÃ©gritÃ© |

**Champs optionnels :** `partial` (true si mise Ã  jour partielle), `scope` (ex. liste limitÃ©e Ã  un sous-ensemble).

### 2.6 Structures communes (orientation)

**Adresse (address) :**
- `host` : string (IP ou nom de domaine)
- `port` : integer â€” **doit appartenir Ã  la plage des ports utilisables pour le MWS** (voir 2.7 ; les ports exclus ne doivent pas Ãªtre utilisÃ©s pour les Ã©changes MWS)
- Optionnel : `protocol` (ex. TCP, UDP)

**EntrÃ©e service (service entry) :**
- `service_id` : string
- `address` : object (host, port)
- Optionnel : `protocol`, `name_hint`

**MÃ©canisme d'intÃ©gritÃ© (integrity) :**
- `method` : string (ex. `signature`, `mac`)
- `value` : string (ex. encodage base64 de la signature ou du MAC)
- Optionnel : `key_id` ou `algorithm` pour vÃ©rification

### 2.6.1 JSON canonique et champs obligatoires / optionnels

Pour l'interopÃ©rabilitÃ© et la vÃ©rification d'intÃ©gritÃ©, les messages MWS utilisent un **encodage JSON canonique** (RFC 8785 ou Ã©quivalent : clÃ©s triÃ©es, pas d'espaces superflus, UTF-8) lors du calcul de la signature ou du MAC. Les implÃ©mentations doivent accepter tout JSON valide conforme au schÃ©ma ; la forme canonique est exigÃ©e **uniquement** pour la production et la vÃ©rification du champ `integrity`.

**Convention :** les champs listÃ©s comme **obligatoires** dans les tableaux des sections 2.1 Ã  2.5 sont **requis** ; leur absence rend le message **non conforme**. Les champs **optionnels** peuvent Ãªtre omis ; s'ils sont prÃ©sents, leur type et contraintes doivent Ãªtre respectÃ©s.

**RÃ©sumÃ© normatif des champs par type de message :**

| Type de message | Champs obligatoires | Champs optionnels |
|-----------------|---------------------|-------------------|
| `presence_announcement` | `version`, `type`, `cog_id`, `bridge_address`, `issued_at`, `integrity` | `core_version_hint`, `valid_until` |
| `service_announcement` | `version`, `type`, `cog_id`, `services`, `issued_at`, `integrity` | `valid_until` ; par service : `protocol`, niveau de sÃ©curitÃ© |
| `host_session_declaration` | `version`, `type`, `cog_id`, `service_id`, `connection_address`, `issued_at`, `integrity` | `session_id`, `capacity_hint`, `security_level_hint`, `valid_until`, `protocol` |
| `discovery_request` | `version`, `type`, `requester_cog_id`, `query`, `issued_at` | `scope`, `integrity` |
| `cog_list` / `status_update` | `version`, `type`, `sender_cog_id`, `entries`, `issued_at`, `integrity` | `partial`, `scope` |

**Structure `address` (obligatoire dans tout champ d'adresse) :** `host` (string), `port` (integer) obligatoires ; `protocol` (string) optionnel. La structure **relay_address** (annonce via relay) est dÃ©crite en section 2.8.

**Structure `integrity` (obligatoire sauf pour discovery_request) :** `method` (string), `value` (string) obligatoires ; `key_id`, `algorithm` (strings) optionnels.

Les schÃ©mas dÃ©taillÃ©s (JSON Schema, CDDL) pour validation machine peuvent Ãªtre publiÃ©s dans un document de spÃ©cification technique rattachÃ© Ã  la norme ; ce document en fixe le cadre normatif.

### 2.7 Ports utilisables et ports exclus (MWS)

Pour Ã©viter les **conflits** avec les services rÃ©seau courants et les **risques de confusion** (ex. exposition d'un Bridge MWS sur le port 80 ou 443, utilisÃ©s par le trafic web), les annonces MWS (adresses, dÃ©clarations d'hÃ©bergement de session) ne doivent **pas** utiliser un ensemble de **ports exclus**. Seuls les **ports utilisables** sont autorisÃ©s pour les Ã©changes et dÃ©clarations MWS.

**RÃ©fÃ©rence :** [IANA â€” Service Name and Transport Protocol Port Number Registry](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml).

#### 2.7.1 Plages IANA (rappel)

| Plage | Nom IANA | Ports |
|-------|----------|--------|
| **System Ports** | Well-Known | 0â€“1023 |
| **User Ports** | Registered | 1024â€“49151 |
| **Dynamic/Private** | Private | 49152â€“65535 |

Les **ports 0â€“1023** sont rÃ©servÃ©s par l'IANA Ã  des services bien connus ; ils ne doivent **pas** Ãªtre utilisÃ©s pour les Ã©changes MWS (risque de conflit avec HTTP, HTTPS, SSH, SMTP, DNS, etc.).

#### 2.7.2 Ports exclus (liste normative)

Les **ports suivants sont exclus** des adresses et dÃ©clarations MWS. Un COG ne doit **pas** annoncer un service ou une session hÃ©bergÃ©e sur l'un de ces ports dans le cadre du Webway. Les Trackers peuvent **rejeter** ou **ignorer** les annonces rÃ©fÃ©renÃ§ant un port exclus.

**System Ports (0â€“1023) â€” exclus en totalitÃ© :**

| Port | Service typique | Raison d'exclusion |
|------|------------------|---------------------|
| 20, 21 | FTP | TrÃ¨s utilisÃ© |
| 22 | SSH | Connexion administrateur |
| 25 | SMTP | Courriel |
| 53 | DNS | RÃ©solution de noms |
| 80 | HTTP | Web |
| 110 | POP3 | Courriel |
| 143 | IMAP | Courriel |
| 443 | HTTPS | Web sÃ©curisÃ© |
| (0â€“1023) | â€” | **Toute la plage** rÃ©servÃ©e IANA |

**User Ports (1024â€“49151) â€” ports couramment utilisÃ©s Ã  exclure :**

| Port | Service / usage typique | Raison d'exclusion |
|------|--------------------------|---------------------|
| 3000 | React, Next.js, Node.js (dev) | TrÃ¨s courant en dÃ©veloppement |
| 3001 | Alternative React / Node | Courant |
| 3306 | MySQL | Base de donnÃ©es |
| 4000 | Jekyll (dev) | DÃ©veloppement |
| 4200 | Angular CLI (dev) | DÃ©veloppement |
| 5000 | Flask, Python (dev) | TrÃ¨s courant |
| 5173 | Vite (dev) | DÃ©veloppement |
| 5432 | PostgreSQL | Base de donnÃ©es |
| 8000 | Django, HTTP Python (dev) | TrÃ¨s courant |
| 8080 | HTTP alternatif (proxy, dev) | TrÃ¨s courant (Ã©quivalent 80) |
| 8081 | HTTP alternatif / proxy | Courant |
| 8443 | HTTPS alternatif | Courant (Ã©quivalent 443) |
| 8501 | Streamlit (dev) | DÃ©veloppement |
| 8888 | Jupyter Notebook / JupyterLab | DÃ©veloppement |
| 3389 | RDP (Remote Desktop) | AccÃ¨s distant |
| 27017 | MongoDB | Base de donnÃ©es |

**RÃ¨gle normative :** la norme MWS doit maintenir une **liste officielle des ports exclus** (au minimum : plage 0â€“1023 + liste explicite des User Ports ci-dessus). Les implÃ©mentations doivent **refuser** toute annonce dont l'adresse utilise un port exclus, ou **avertir** et dÃ©grader selon politique locale.

#### 2.7.3 Ports utilisables recommandÃ©s (orientation)

Pour les **Ã©changes MWS** (Bridge, dÃ©claration d'hÃ©bergement de session, Tracker), il est recommandÃ© d'utiliser des ports dans la plage **User Ports (1024â€“49151)** **hors** de la liste des ports exclus ci-dessus. Exemples de plages souvent libres (Ã  valider selon environnement) :

- **9000â€“9999** : souvent utilisÃ©s pour services applicatifs dÃ©diÃ©s (Ã  Ã©viter si conflit local)
- **10000â€“19999** : plage large, peu assignÃ©e par dÃ©faut
- **MWS dÃ©diÃ©e** : une plage peut Ãªtre **rÃ©servÃ©e** par la norme MWS (ex. 19100â€“19199) pour les services et sessions Webway uniquement ; la gouvernance MWS pourra en dÃ©cider.

Les **Dynamic/Private Ports (49152â€“65535)** sont en principe libres mais souvent utilisÃ©s par le systÃ¨me pour les connexions Ã©phÃ©mÃ¨res ; leur usage pour des **services MWS annoncÃ©s** (Ã©coute) est dÃ©conseillÃ© sauf politique locale explicite.

#### 2.7.4 Port officiel des COGs Tracker MWS

Le **port officiel** des COGs Tracker du MWS est le **port 21000**.

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **Port** | **21000** (TCP ou selon binding dÃ©fini) |
| **Usage** | Point d'Ã©coute des COGs Tracker pour les annonces (prÃ©sence, services, sessions hÃ©bergÃ©es), les requÃªtes de dÃ©couverte et l'Ã©change de listes de statuts |
| **Norme** | Les COGs Tracker MWS **exposent par dÃ©faut** leur endpoint sur le port **21000**. Les COGs participants **se connectent** aux Trackers sur ce port (ou sur l'adresse indiquÃ©e en config si override). |

**RÃ¨gle :** un COG qui endosse le rÃ´le de **Tracker** doit exposer son service de dÃ©couverte sur le **port 21000** (ou annoncer explicitement un autre port s'il s'agit d'une configuration dÃ©rogatoire). Les implÃ©mentations doivent utiliser **21000** comme port par dÃ©faut pour joindre un Tracker lorsque seule l'adresse (host) est fournie.

Le port **21000** appartient Ã  la plage User Ports (1024â€“49151) et n'est pas dans la liste des ports exclus ; il est **rÃ©servÃ©** par la norme MWS au rÃ´le Tracker.

#### 2.7.5 ConformitÃ©

- Une **annonce** (prÃ©sence, service, session hÃ©bergÃ©e) dont le champ `port` (ou `connection_address.port`) appartient Ã  la **liste des ports exclus** est considÃ©rÃ©e **non conforme** pour ce champ.
- Les COGs Tracker peuvent **rejeter** ou **ignorer** ces annonces (systÃ¨me actif) et **signaler** la non-conformitÃ© (systÃ¨me passif).
- La liste des ports exclus est **versionnÃ©e** avec la norme (ex. `mws_declaration_v1`) et peut Ãªtre Ã©tendue sans changement de version majeure (ajout de ports Ã  exclure).

#### 2.8 Annonce d'adresse relay (intÃ©gration relay)

Lorsqu'un COG est joignable via un **relay Webway** (tunnel Ã©tendu multi-tenant), les annonces de prÃ©sence, de services ou d'hÃ©bergement de session peuvent indiquer une **adresse de type relay** au lieu d'une adresse IP/port directe. Voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md).

**Structure `relay_address` (optionnelle dans les annonces) :**

| Champ | Type | Obligatoire | Description |
|-------|------|-------------|-------------|
| `host` | string | oui | Adresse du relay (nom de domaine ou IP), ex. `webway.studiomiyukini.com` |
| `port` | integer | oui | Port du relay (ex. **7000** â€” port relay MWS) ; doit Ãªtre hors liste des ports exclus |
| `relay_type` | string | oui | Valeur fixe `relay` pour distinguer d'une adresse directe |
| `cog_id` | string | optionnel | Identifiant du COG cible cÃ´tÃ© relay (pour le routing) ; peut Ãªtre omis si dÃ©rivÃ© du token ou de la politique |
| `token_hint` | string | optionnel | Indication opaque (ex. prÃ©fixe, alias) pour l'appelant ; **jamais** le secret complet |

**RÃ¨gle :** une annonce peut contenir Ã  la fois une adresse directe (`address`) et une `relay_address` (ex. COG joignable en direct et via relay). Les champs `bridge_address`, `connection_address` ou une entrÃ©e dans `services` peuvent Ãªtre soit un objet `address`, soit un objet `relay_address` (discriminant : prÃ©sence de `relay_type: "relay"`).

**ConformitÃ© :** l'annonce d'une adresse relay doit respecter les mÃªmes contraintes de ports (port relay hors liste exclusive) et d'intÃ©gritÃ© que les annonces d'adresses directes. Le relay ne gouverne pas les accÃ¨s ; il assure uniquement le transport et le routing par `cog_id`.

---

## 3. Protocole MWS (orientation)

Le **protocole MWS** dÃ©finit les **types de messages**, les **sÃ©quences** et les **rÃ¨gles de transport** pour la dÃ©couverte et les annonces.

### 3.1 Types de messages

| Type | Direction | Description |
|------|-----------|-------------|
| `presence_announcement` | COG â†’ Tracker / maillage | Annonce de prÃ©sence |
| `service_announcement` | COG â†’ Tracker / maillage | Annonce de services et adresses |
| `host_session_declaration` | COG HÃ©bergeur â†’ Tracker / maillage | DÃ©claration d'hÃ©bergement de session |
| `discovery_request` | COG / Tracker â†’ Tracker | RequÃªte de dÃ©couverte |
| `discovery_response` | Tracker â†’ COG | RÃ©ponse (liste de COGs, services, sessions) |
| `cog_list` / `status_update` | COG â†” COG, Tracker â†” COG | Ã‰change de listes de statuts |

### 3.2 SÃ©quences typiques

**Annonce de prÃ©sence / services / session :**
1. Le COG Ã©metteur construit la dÃ©claration conforme Ã  la norme (format, signature/intÃ©gritÃ©).
2. Envoi vers un ou plusieurs Trackers (ou diffusion sur le maillage selon le mode).
3. Le(s) Tracker(s) vÃ©rifient la conformitÃ© et l'intÃ©gritÃ© ; acceptation ou rejet (systÃ¨mes passifs/actifs).

**DÃ©couverte :**
1. Le COG demandeur envoie une `discovery_request` (Ã©ventuellement authentifiÃ©e) Ã  un ou plusieurs Trackers.
2. Le(s) Tracker(s) rÃ©pondent par une `discovery_response` contenant les entrÃ©es correspondant aux critÃ¨res (COGs, services, sessions hÃ©bergÃ©es), en respectant les listes de statuts (ex. exclure Rejected).

**Ã‰change de statuts :**
1. Les COGs (et Trackers) s'Ã©changent des `cog_list` ou `status_update` selon le protocole et la politique (pÃ©riodicitÃ©, dÃ©clencheur).
2. Chaque COG met Ã  jour sa liste locale et peut appliquer des rÃ¨gles (filtrer, dÃ©grader, rejeter) selon la matrice des statuts (section 4).

### 3.3 RÃ¨gles de transport et bindings

- **Binding principal (normatif)** : le transport **TCP avec TLS** est le **binding principal** du protocole MWS pour les Ã©changes entre COGs participants et COGs Tracker (annonces, requÃªtes de dÃ©couverte, listes de statuts). Les implÃ©mentations conformes doivent **au minimum** supporter ce binding.
  - **Encodage** : messages en JSON (UTF-8), forme canonique pour le calcul d'intÃ©gritÃ© (voir 2.6.1).
  - **TLS** : confidentialitÃ© et intÃ©gritÃ© en transit ; certificat serveur recommandÃ© pour les Trackers. En production, TLS est **recommandÃ©** ; les dÃ©ploiements peuvent autoriser TCP nu uniquement en environnement contrÃ´lÃ© (ex. boucle locale).
- **Autres bindings (optionnels)** : d'autres bindings (ex. WebSocket sur TLS, HTTP/2, UDP pour annonces lÃ©gÃ¨res) peuvent Ãªtre dÃ©finis par extension de la norme ou par politique locale. Ils ne remplacent pas l'exigence de support du binding TCP + TLS pour l'interopÃ©rabilitÃ© de base.
- **Port officiel des Trackers** : les COGs Tracker MWS exposent leur endpoint sur le **port 21000** (voir section 2.7.4). Les COGs participants se connectent aux Trackers sur `host:21000` par dÃ©faut (ou sur l'adresse complÃ¨te indiquÃ©e en config si override).
- **DÃ©couverte des Trackers** : les COGs doivent pouvoir connaÃ®tre l'adresse (host) des Trackers (config locale, bootstrap, ou annuaire connu). Le port par dÃ©faut pour joindre un Tracker est **21000**. Le dÃ©tail du bootstrap est hors scope de ce document conceptuel.

---

## 4. Matrice des statuts et rÃ¨gles d'Ã©change

### 4.1 Statuts normatifs

Les **statuts** de la Webway COG List sont **normatifs** pour l'interopÃ©rabilitÃ©. Valeurs standard :

| Statut | Code (orientation) | Signification | Usage typique |
|--------|---------------------|---------------|---------------|
| **Trusted** | `trusted` | COG considÃ©rÃ© comme fiable pour la prÃ©sence / dÃ©couverte | Annonces acceptÃ©es, relayÃ©es |
| **Neutral** | `neutral` | Aucun signal positif ou nÃ©gatif | TraitÃ© par dÃ©faut selon politique locale |
| **Under review** | `under_review` | En cours d'analyse (comportement suspect, signalement) | Limitation ou surveillance |
| **Distrusted** | `distrusted` | COG considÃ©rÃ© comme non fiable | Annonces ou connexions dÃ©gradÃ©es / filtrÃ©es |
| **Rejected** | `rejected` | COG ou connexion rejetÃ©e (malveillant ou politique locale) | Refus d'annonce, blocage de connexion Webway |

### 4.2 Structure d'une entrÃ©e de liste (orientation)

| Champ | Type | Description |
|-------|------|-------------|
| `cog_id` | string | Identifiant du COG |
| `status` | string | Une des valeurs normatives ci-dessus |
| `source` | string | Identifiant du COG ou Tracker ayant fourni/mis Ã  jour le statut |
| `updated_at` | string (ISO 8601) | DerniÃ¨re mise Ã  jour du statut |
| Optionnel | `reason_hint` | Indication courte (ex. code) â€” pas de donnÃ©e sensible |

### 4.3 RÃ¨gles d'Ã©change entre COGs

- **SouverainetÃ©** : chaque COG reste libre d'ignorer un statut fourni par un autre et d'appliquer sa propre politique (analyser, rejeter, accepter).
- **Pas de donnÃ©es mÃ©tier** : l'Ã©change de listes ne contient que des identifiants COG, des statuts et des mÃ©tadonnÃ©es de traÃ§abilitÃ©.
- **AgrÃ©gation** : les COGs Tracker peuvent agrÃ©ger les signaux de plusieurs sources pour mettre Ã  jour ou proposer des statuts (selon contrats passifs/actifs).
- **Propagation** : les rÃ¨gles de propagation (qui envoie Ã  qui, frÃ©quence, pÃ©rimÃ¨tre) sont dÃ©finies par le protocole MWS ou par politique locale ; la norme impose la **structure** et les **valeurs de statut**, pas la topologie d'Ã©change.

### 4.4 Comportement attendu selon statut

| Statut | Annonces du COG | RequÃªtes du COG | RÃ©ponse des Trackers / participants |
|--------|------------------|-----------------|--------------------------------------|
| Trusted | AcceptÃ©es, relayÃ©es | TraitÃ©es normalement | Inclus dans les rÃ©ponses de dÃ©couverte |
| Neutral | AcceptÃ©es par dÃ©faut | TraitÃ©es par dÃ©faut | Inclus selon politique |
| Under review | LimitÃ©es ou surveillÃ©es | LimitÃ©es ou surveillÃ©es | DÃ©gradation possible (throttle, dÃ©lai) |
| Distrusted | FiltrÃ©es / dÃ©gradÃ©es | FiltrÃ©es / dÃ©gradÃ©es | Exclus ou dÃ©gradÃ©s selon politique |
| Rejected | RefusÃ©es | RefusÃ©es ou bloquÃ©es | Exclus des rÃ©ponses, blocage connexion Webway |

Les comportements prÃ©cis (ex. seuils, durÃ©es) relÃ¨vent des contrats des systÃ¨mes passifs et actifs des Trackers et des politiques locales.

### 4.5 RÃ¨gles de transition entre statuts (matrice normative)

Les **transitions** d'un statut Ã  un autre sont soumises aux rÃ¨gles suivantes. Seuls certains acteurs peuvent proposer ou appliquer une transition ; chaque COG reste souverain pour accepter ou ignorer une mise Ã  jour de statut (voir 4.3).

**Valeurs de statut normatives :** `trusted` | `neutral` | `under_review` | `distrusted` | `rejected`.

**Matrice des transitions autorisÃ©es (orientation normative) :**

| Statut source | Transition autorisÃ©e vers | Acteur typique Ã  l'origine | Condition / remarque |
|----------------|----------------------------|----------------------------|----------------------|
| * (initial) | `neutral` | Premier contact, Tracker | EntrÃ©e par dÃ©faut pour un COG jamais vu |
| `neutral` | `trusted` | Tracker, COG participant | Signal positif (historique, attestation, politique locale) |
| `neutral` | `under_review` | Tracker, COG participant | Comportement suspect, signalement, Ã  analyser |
| `neutral` | `distrusted` | Tracker, COG participant | Signal nÃ©gatif sans blocage total |
| `neutral` | `rejected` | Tracker, politique locale | Refus explicite (malveillance, politique) |
| `trusted` | `neutral` | Tracker, COG | RÃ©vocation du statut privilÃ©giÃ© (dÃ©lai, changement de politique) |
| `trusted` | `under_review` | Tracker | Signal nÃ©gatif sur un COG jusque-lÃ  fiable |
| `under_review` | `neutral` | Tracker, COG | Fin de revue sans sanction |
| `under_review` | `trusted` | Tracker, COG | Revue favorable |
| `under_review` | `distrusted` | Tracker, COG | Revue dÃ©favorable |
| `under_review` | `rejected` | Tracker, politique | Sanction maximale |
| `distrusted` | `neutral` | Tracker, COG | RÃ©habilitation (aprÃ¨s dÃ©lai ou preuve) |
| `distrusted` | `under_review` | Tracker | Nouvelle analyse |
| `distrusted` | `rejected` | Tracker, politique | Escalade |
| `rejected` | `under_review` | Tracker, politique | RÃ©examen exceptionnel (dÃ©blocage manuel) |
| `rejected` | `neutral` | â€” | Non recommandÃ© sans processus explicite de rÃ©habilitation |

**RÃ¨gles gÃ©nÃ©rales :**
- **UnidirectionnalitÃ© recommandÃ©e pour `rejected`** : le passage Ã  `rejected` est une dÃ©cision forte ; la sortie de `rejected` doit Ãªtre contrÃ´lÃ©e (processus de rÃ©habilitation, dÃ©lai, audit).
- **Source de la transition** : chaque entrÃ©e de liste (section 4.2) porte un `source` (COG ou Tracker) et `updated_at` ; la propagation et l'agrÃ©gation des statuts restent dÃ©finies par le protocole et les contrats des systÃ¨mes passifs/actifs.
- **Conflits** : si deux sources proposent des statuts diffÃ©rents pour le mÃªme `cog_id`, chaque COG applique sa politique locale (prioritÃ© Ã  la source, au timestamp, ou au statut le plus restrictif selon configuration).

---

## 5. Standards de conformitÃ© pour les COGs Tracker

Les **COGs Tracker** doivent respecter des **exigences minimales** pour participer au maillage en tant que Tracker et assurer la cohÃ©rence des normes.

### 5.1 Exigences minimales

| Exigence | Description |
|----------|-------------|
| **Norme de dÃ©claration** | Accepter et vÃ©rifier uniquement les dÃ©clarations conformes Ã  la norme de dÃ©claration sÃ©curisÃ©e (MWS) en vigueur. |
| **Formats** | Comprendre et traiter les formats de messages normatifs (annonces, requÃªtes, listes de statuts) selon la version supportÃ©e. |
| **Statuts** | Maintenir et Ã©changer des listes de COGs avec les statuts normatifs ; appliquer les rÃ¨gles d'Ã©change (section 4). |
| **Protection du rÃ©seau** | Mettre en Å“uvre des mÃ©canismes **passifs** et **actifs** conformes aux contrats MWS ([Passive Systems](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md), [Active Systems](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)) pour protÃ©ger le maillage. |
| **Pas de gouvernance** | Ne jamais dÃ©cider d'accÃ¨s (Permis de circulation, Visa de Connexion, Passeport) pour le mÃ©tier ; le MWS dÃ©livre le Permis de circulation et applique le contrÃ´le tracker ; ne pas exposer de donnÃ©es mÃ©tier ni de gouvernance. |

### 5.2 VÃ©rification de conformitÃ©

- La conformitÃ© peut Ãªtre **vÃ©rifiÃ©e** par des tests ou des audits (formats, comportement face Ã  dÃ©clarations valides/invalides, respect des statuts).
- Un COG Tracker **non conforme** peut Ãªtre **signalÃ©** (statut Distrusted ou Rejected) par d'autres COGs ou Trackers selon les rÃ¨gles d'Ã©change.
- Les exigences dÃ©taillÃ©es (critÃ¨res de test, niveaux de conformitÃ©) seront dÃ©finies dans un contrat ou un protocole MWS dÃ©diÃ©.

### 5.3 Versionnement supportÃ©

- Chaque Tracker doit **annoncer** les **versions de norme** qu'il supporte (ex. `mws_declaration_v1`, `mws_protocol_v1`).
- Les dÃ©clarations Ã©mises avec une version non supportÃ©e peuvent Ãªtre **refusÃ©es** ou **ignorÃ©es**.
- La **rÃ©trocompatibilitÃ©** et les fenÃªtres de dÃ©prÃ©ciation sont dÃ©finies par la gouvernance du MWS (Ã©volution des normes).

---

## 6. Versionnement et Ã©volution des normes

### 6.1 Version des normes

- **Norme de dÃ©claration sÃ©curisÃ©e** : versionnÃ©e (ex. `mws_declaration_v1`). Toute Ã©volution non rÃ©trocompatible doit incrÃ©menter la version majeure.
- **Formats de messages** : version indiquÃ©e dans chaque message (`version`). Les anciennes versions peuvent Ãªtre supportÃ©es pendant une pÃ©riode de dÃ©prÃ©ciation.
- **Statuts** : les valeurs normatives sont stables ; de nouveaux statuts peuvent Ãªtre ajoutÃ©s avec extension (ex. statut optionnel) sans invalider les existants.

### 6.2 RÃ©trocompatibilitÃ©

- Les **nouveaux champs** (optionnels) ne doivent pas casser les implÃ©mentations existantes.
- Les **champs supprimÃ©s** ou modifiÃ©s de maniÃ¨re incompatible doivent suivre une pÃ©riode de dÃ©prÃ©ciation annoncÃ©e.
- Les Trackers et participants sont encouragÃ©s Ã  supporter au moins la version courante et la version prÃ©cÃ©dente pendant la transition.

### 6.3 Gouvernance des normes

- L'**Ã©volution** des normes et standards du MWS (nouveaux formats, nouveaux statuts, modifications de la norme de dÃ©claration) relÃ¨ve d'un processus de gouvernance Ã  dÃ©finir (communautÃ©, mainteneurs, release).
- Ce document conceptuel fixe le **cadre** ; les versions numÃ©rotÃ©es et les changelogs seront maintenus dans des documents ou dÃ©pÃ´ts dÃ©diÃ©s.

---

## 7. SynthÃ¨se

| Domaine | Norme / Standard | Statut |
|---------|------------------|--------|
| **DÃ©claration sÃ©curisÃ©e** | Authentification, intÃ©gritÃ©, format unifiÃ©, limitation des abus | Cadre dÃ©fini (sections 1â€“2) ; implÃ©mentation Ã  appliquer |
| **Formats de messages** | Annonce prÃ©sence, services, session hÃ©bergÃ©e, requÃªte dÃ©couverte, liste statuts ; JSON canonique, champs obligatoires/optionnels (section 2.6.1) | SchÃ©mas finalisÃ©s (cadre normatif) |
| **Ports utilisables / exclus** | Liste normative ports exclus (0â€“1023 + ports courants web/dev/DB) ; ports utilisables pour MWS | DÃ©fini (section 2.7) |
| **Protocole MWS** | Types de messages, sÃ©quences, rÃ¨gles de transport ; **binding principal TCP + TLS** (section 3.3) | Bindings prÃ©cisÃ©s |
| **IntÃ©gration relay** | Annonce d'adresse relay (relay_address), structure et conformitÃ© (section 2.8) | DÃ©fini ; voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) |
| **Matrice des statuts** | Valeurs normatives, structure d'entrÃ©e, rÃ¨gles d'Ã©change, comportement attendu, **rÃ¨gles de transition** (section 4.5) | Formalisation complÃ¨te |
| **ConformitÃ© Trackers** | Exigences minimales, vÃ©rification, versionnement supportÃ© | Cadre dÃ©fini (section 5) ; contrats Tracker : [Passive](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) / [Active](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) |
| **Versionnement** | Version des normes, rÃ©trocompatibilitÃ©, gouvernance de l'Ã©volution | Principes dÃ©finis |

---

## RÃ©fÃ©rences croisÃ©es

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) â€” document principal
- [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) â€” annexe conceptuelle (Outils, Kits d'Outils, OpÃ©rateurs MWS)
- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) â€” architecture relay, annonce d'adresse relay (section 2.8)
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) â€” dÃ©ploiement relay (VM, TLS, systemd, tests)
- [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) â€” contrats systÃ¨mes passifs Tracker
- [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) â€” contrats systÃ¨mes actifs Tracker
- [Connexion Inter-COG](_index.md)
- [Doctrine Securite Fondamentale](_index.md)
- [Glossaire](_index.md) (Norme de dÃ©claration sÃ©curisÃ©e MWS, DÃ©claration d'hÃ©bergement de session, COG Tracker, Liste de COGs avec statuts)

---

*Document crÃ©Ã© le 30/01/2026*  
*Classification : Reference conceptuelle â€” Annexe MWS (Normes et Standards)*


