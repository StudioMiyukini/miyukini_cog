# Miyukini Conceptual References - Miyukini Webway System Normes et Standards

## Contexte

Ce document est un **annexe conceptuel** au [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md). Il développe les **normes et standards** du MWS : norme de déclaration sécurisée, formats de messages, protocole de découverte, matrice des statuts et règles d'échange entre COGs, ainsi que les exigences de conformité pour les COGs Tracker.

**Principe directeur :**

> **Les normes et standards du MWS garantissent l'interopérabilité, l'authenticité et l'intégrité des annonces de présence ; ils ne définissent pas la gouvernance des accès (Passeport, Visa).**

## Portée / Scope

- **Norme de déclaration sécurisée (MWS)** : authentification, intégrité, format unifié, limitation des abus
- **Formats de messages** : annonce de présence, annonce de services/adresses, déclaration d'hébergement de session, requête de découverte, liste de statuts
- **Ports utilisables et ports exclus** : liste normative des ports exclus (IANA 0–1023, ports courants web/dev/DB) pour les échanges MWS
- **Protocole MWS** : types de messages, séquences, règles de transport (orientation)
- **Matrice des statuts** et règles d'échange entre COGs (Webway COG List)
- **Standards de conformité** pour les COGs Tracker (exigences minimales, vérification)
- **Versionnement** et rétrocompatibilité des normes

Ce document **ne couvre pas** :
- Les spécifications techniques d'implémentation (protocoles réseau, algorithmes cryptographiques détaillés) → contrats ou specs techniques dédiés
- La gouvernance des visites (Passeport, Visa) → voir [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)

---

## 1. Norme de déclaration sécurisée (MWS)

La **norme de déclaration sécurisée** s'applique à toute **annonce** émise par un COG participant sur le Webway : présence, services exposés, adresses (IP/ports), déclaration d'hébergement de session. Elle doit être **créée, publiée et appliquée** par l'écosystème MWS.

### 1.1 Objectifs

| Objectif | Description |
|----------|-------------|
| **Authentification** | Attester l'origine de la déclaration (COG identifié, non usurpation) |
| **Intégrité** | Garantir que la déclaration n'a pas été altérée en transit |
| **Format unifié** | Permettre l'interopérabilité et la vérification par les Trackers et les participants |
| **Limitation des abus** | Déclarations conformes, sans exposition de données sensibles ni de gouvernance |

### 1.2 Exigences minimales

**Authentification de l'origine :**
- La déclaration doit être **signée** ou **chiffrée** par une clé ou un secret associé au COG émetteur (ex. clé dérivée de l'identité du COG, certificat, mécanisme attesté).
- Le récepteur (Tracker ou autre COG) doit pouvoir **vérifier** que l'émetteur est bien le COG annoncé (identifiant COG cohérent avec la signature ou le mécanisme d'attestation).
- Aucune donnée métier ni secret de gouvernance ne doit être inclus dans la déclaration.

**Intégrité :**
- La déclaration doit inclure un **mécanisme d'intégrité** (ex. signature numérique, MAC, hash signé) permettant de détecter toute modification en transit.
- Le format doit être **déterministe** (sérialisation canonique) pour que la vérification soit reproductible.

**Format unifié :**
- **Schéma** : structure des champs obligatoires et optionnels, types, contraintes (longueur, plages de valeurs).
- **Encodage** : encodage standard (ex. JSON, CBOR) et jeu de caractères (ex. UTF-8) pour l'interopérabilité.
- **Version** : chaque déclaration doit indiquer la **version de la norme** (ex. `mws_declaration_v1`) pour permettre l'évolution et la rétrocompatibilité.

**Limitation des abus :**
- Les champs autorisés sont **restreints** à la présence et à la découverte (identifiant COG, services, adresses, ports, type de session, métadonnées de traçabilité).
- **Interdits** : données utilisateur, secrets, contenu métier, informations permettant d'usurper une gouvernance.

### 1.3 Non-conformité

- Un COG Tracker peut **refuser** d'accepter ou de relayer une déclaration non conforme (système actif).
- Un COG participant peut **ignorer** ou **dégrader** les déclarations non conformes selon sa politique locale.
- La non-conformité peut être **signalée** dans le cadre des listes de statuts (ex. passage en Under review, Distrusted) selon les règles d'échange (voir section 4).

---

## 2. Formats de messages MWS

Les **formats de messages** définissent la structure des échanges sur le Webway. Ils sont **normatifs** dès lors qu'un protocole MWS les adopte.

### 2.1 Annonce de présence (Presence Announcement)

**Usage :** un COG participant annonce sa présence au réseau (ou à un ou plusieurs Trackers).

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme (ex. `mws_declaration_v1`) |
| `type` | string | Type de message (ex. `presence_announcement`) |
| `cog_id` | string | Identifiant du COG émetteur (ex. LSI ou équivalent attesté) |
| `bridge_address` | object | Adresse du Bridge (voir 2.5) |
| `issued_at` | string (ISO 8601) | Horodatage d'émission (trace only) |
| `integrity` | object | Mécanisme d'intégrité (signature, MAC, etc.) |

**Champs optionnels (orientation) :** `core_version_hint`, `valid_until`, selon politique locale.

### 2.2 Annonce de services et adresses (Service / Address Announcement)

**Usage :** un COG annonce les **services** qu'il expose et les **adresses** (IP ou nom de domaine, ports) associées.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `service_announcement` |
| `cog_id` | string | Identifiant du COG émetteur |
| `services` | array | Liste d'entrées service (voir 2.5) |
| `issued_at` | string (ISO 8601) | Horodatage d'émission |
| `integrity` | object | Mécanisme d'intégrité |

**Champs optionnels :** `valid_until`, par service : protocole (ex. TCP, UDP), niveau de sécurité proposé.

### 2.3 Déclaration d'hébergement de session (Host Session Declaration)

**Usage :** un COG Hébergeur déclare qu'il **héberge une session** d'un service donné et **attend des connexions** vers lui.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `host_session_declaration` |
| `cog_id` | string | Identifiant du COG Hébergeur |
| `service_id` | string | Identifiant du service (ou type de session) |
| `session_id` | string | Identifiant unique de la session (optionnel selon politique) |
| `connection_address` | object | Adresse de connexion (IP ou domaine, port) — voir 2.5 |
| `issued_at` | string (ISO 8601) | Horodatage d'émission |
| `integrity` | object | Mécanisme d'intégrité |

**Champs optionnels :** `capacity_hint`, `security_level_hint`, `valid_until`, `protocol` (ex. TCP).

**Règle :** cette déclaration **ne donne aucun droit d'accès** ; elle indique où se présenter pour demander un Visa.

### 2.4 Requête de découverte (Discovery Request)

**Usage :** un COG (ou un Tracker) interroge le maillage pour **découvrir** des COGs, des services ou des sessions hébergées.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme / protocole |
| `type` | string | Ex. `discovery_request` |
| `requester_cog_id` | string | Identifiant du COG demandeur |
| `query` | object | Critères de recherche (ex. par service_id, par cog_id, liste de sessions) |
| `issued_at` | string (ISO 8601) | Horodatage |

**Champs optionnels :** `scope` (ex. un Tracker, plusieurs Trackers), `integrity` si la requête doit être authentifiée.

### 2.5 Liste de COGs avec statuts (Webway COG List / Status Update)

**Usage :** échange entre COGs (ou avec les Trackers) de **listes ou mises à jour de statuts** pour la sécurité du maillage.

**Champs obligatoires (orientation) :**

| Champ | Type | Description |
|-------|------|-------------|
| `version` | string | Version de la norme |
| `type` | string | Ex. `cog_list` ou `status_update` |
| `sender_cog_id` | string | Identifiant du COG émetteur |
| `entries` | array | Liste d'entrées (cog_id, status, source, updated_at) — voir section 4 |
| `issued_at` | string (ISO 8601) | Horodatage |
| `integrity` | object | Mécanisme d'intégrité |

**Champs optionnels :** `partial` (true si mise à jour partielle), `scope` (ex. liste limitée à un sous-ensemble).

### 2.6 Structures communes (orientation)

**Adresse (address) :**
- `host` : string (IP ou nom de domaine)
- `port` : integer — **doit appartenir à la plage des ports utilisables pour le MWS** (voir 2.7 ; les ports exclus ne doivent pas être utilisés pour les échanges MWS)
- Optionnel : `protocol` (ex. TCP, UDP)

**Entrée service (service entry) :**
- `service_id` : string
- `address` : object (host, port)
- Optionnel : `protocol`, `name_hint`

**Mécanisme d'intégrité (integrity) :**
- `method` : string (ex. `signature`, `mac`)
- `value` : string (ex. encodage base64 de la signature ou du MAC)
- Optionnel : `key_id` ou `algorithm` pour vérification

Les schémas détaillés (JSON Schema, CDDL, ou équivalent) seront définis dans un document de spécification technique rattaché à la norme.

### 2.7 Ports utilisables et ports exclus (MWS)

Pour éviter les **conflits** avec les services réseau courants et les **risques de confusion** (ex. exposition d'un Bridge MWS sur le port 80 ou 443, utilisés par le trafic web), les annonces MWS (adresses, déclarations d'hébergement de session) ne doivent **pas** utiliser un ensemble de **ports exclus**. Seuls les **ports utilisables** sont autorisés pour les échanges et déclarations MWS.

**Référence :** [IANA — Service Name and Transport Protocol Port Number Registry](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml).

#### 2.7.1 Plages IANA (rappel)

| Plage | Nom IANA | Ports |
|-------|----------|--------|
| **System Ports** | Well-Known | 0–1023 |
| **User Ports** | Registered | 1024–49151 |
| **Dynamic/Private** | Private | 49152–65535 |

Les **ports 0–1023** sont réservés par l'IANA à des services bien connus ; ils ne doivent **pas** être utilisés pour les échanges MWS (risque de conflit avec HTTP, HTTPS, SSH, SMTP, DNS, etc.).

#### 2.7.2 Ports exclus (liste normative)

Les **ports suivants sont exclus** des adresses et déclarations MWS. Un COG ne doit **pas** annoncer un service ou une session hébergée sur l'un de ces ports dans le cadre du Webway. Les Trackers peuvent **rejeter** ou **ignorer** les annonces référençant un port exclus.

**System Ports (0–1023) — exclus en totalité :**

| Port | Service typique | Raison d'exclusion |
|------|------------------|---------------------|
| 20, 21 | FTP | Très utilisé |
| 22 | SSH | Connexion administrateur |
| 25 | SMTP | Courriel |
| 53 | DNS | Résolution de noms |
| 80 | HTTP | Web |
| 110 | POP3 | Courriel |
| 143 | IMAP | Courriel |
| 443 | HTTPS | Web sécurisé |
| (0–1023) | — | **Toute la plage** réservée IANA |

**User Ports (1024–49151) — ports couramment utilisés à exclure :**

| Port | Service / usage typique | Raison d'exclusion |
|------|--------------------------|---------------------|
| 3000 | React, Next.js, Node.js (dev) | Très courant en développement |
| 3001 | Alternative React / Node | Courant |
| 3306 | MySQL | Base de données |
| 4000 | Jekyll (dev) | Développement |
| 4200 | Angular CLI (dev) | Développement |
| 5000 | Flask, Python (dev) | Très courant |
| 5173 | Vite (dev) | Développement |
| 5432 | PostgreSQL | Base de données |
| 8000 | Django, HTTP Python (dev) | Très courant |
| 8080 | HTTP alternatif (proxy, dev) | Très courant (équivalent 80) |
| 8081 | HTTP alternatif / proxy | Courant |
| 8443 | HTTPS alternatif | Courant (équivalent 443) |
| 8501 | Streamlit (dev) | Développement |
| 8888 | Jupyter Notebook / JupyterLab | Développement |
| 3389 | RDP (Remote Desktop) | Accès distant |
| 27017 | MongoDB | Base de données |

**Règle normative :** la norme MWS doit maintenir une **liste officielle des ports exclus** (au minimum : plage 0–1023 + liste explicite des User Ports ci-dessus). Les implémentations doivent **refuser** toute annonce dont l'adresse utilise un port exclus, ou **avertir** et dégrader selon politique locale.

#### 2.7.3 Ports utilisables recommandés (orientation)

Pour les **échanges MWS** (Bridge, déclaration d'hébergement de session, Tracker), il est recommandé d'utiliser des ports dans la plage **User Ports (1024–49151)** **hors** de la liste des ports exclus ci-dessus. Exemples de plages souvent libres (à valider selon environnement) :

- **9000–9999** : souvent utilisés pour services applicatifs dédiés (à éviter si conflit local)
- **10000–19999** : plage large, peu assignée par défaut
- **MWS dédiée** : une plage peut être **réservée** par la norme MWS (ex. 19100–19199) pour les services et sessions Webway uniquement ; la gouvernance MWS pourra en décider.

Les **Dynamic/Private Ports (49152–65535)** sont en principe libres mais souvent utilisés par le système pour les connexions éphémères ; leur usage pour des **services MWS annoncés** (écoute) est déconseillé sauf politique locale explicite.

#### 2.7.4 Port officiel des COGs Tracker MWS

Le **port officiel** des COGs Tracker du MWS est le **port 21000**.

| Élément | Valeur |
|--------|--------|
| **Port** | **21000** (TCP ou selon binding défini) |
| **Usage** | Point d'écoute des COGs Tracker pour les annonces (présence, services, sessions hébergées), les requêtes de découverte et l'échange de listes de statuts |
| **Norme** | Les COGs Tracker MWS **exposent par défaut** leur endpoint sur le port **21000**. Les COGs participants **se connectent** aux Trackers sur ce port (ou sur l'adresse indiquée en config si override). |

**Règle :** un COG qui endosse le rôle de **Tracker** doit exposer son service de découverte sur le **port 21000** (ou annoncer explicitement un autre port s'il s'agit d'une configuration dérogatoire). Les implémentations doivent utiliser **21000** comme port par défaut pour joindre un Tracker lorsque seule l'adresse (host) est fournie.

Le port **21000** appartient à la plage User Ports (1024–49151) et n'est pas dans la liste des ports exclus ; il est **réservé** par la norme MWS au rôle Tracker.

#### 2.7.5 Conformité

- Une **annonce** (présence, service, session hébergée) dont le champ `port` (ou `connection_address.port`) appartient à la **liste des ports exclus** est considérée **non conforme** pour ce champ.
- Les COGs Tracker peuvent **rejeter** ou **ignorer** ces annonces (système actif) et **signaler** la non-conformité (système passif).
- La liste des ports exclus est **versionnée** avec la norme (ex. `mws_declaration_v1`) et peut être étendue sans changement de version majeure (ajout de ports à exclure).

---

## 3. Protocole MWS (orientation)

Le **protocole MWS** définit les **types de messages**, les **séquences** et les **règles de transport** pour la découverte et les annonces.

### 3.1 Types de messages

| Type | Direction | Description |
|------|-----------|-------------|
| `presence_announcement` | COG → Tracker / maillage | Annonce de présence |
| `service_announcement` | COG → Tracker / maillage | Annonce de services et adresses |
| `host_session_declaration` | COG Hébergeur → Tracker / maillage | Déclaration d'hébergement de session |
| `discovery_request` | COG / Tracker → Tracker | Requête de découverte |
| `discovery_response` | Tracker → COG | Réponse (liste de COGs, services, sessions) |
| `cog_list` / `status_update` | COG ↔ COG, Tracker ↔ COG | Échange de listes de statuts |

### 3.2 Séquences typiques

**Annonce de présence / services / session :**
1. Le COG émetteur construit la déclaration conforme à la norme (format, signature/intégrité).
2. Envoi vers un ou plusieurs Trackers (ou diffusion sur le maillage selon le mode).
3. Le(s) Tracker(s) vérifient la conformité et l'intégrité ; acceptation ou rejet (systèmes passifs/actifs).

**Découverte :**
1. Le COG demandeur envoie une `discovery_request` (éventuellement authentifiée) à un ou plusieurs Trackers.
2. Le(s) Tracker(s) répondent par une `discovery_response` contenant les entrées correspondant aux critères (COGs, services, sessions hébergées), en respectant les listes de statuts (ex. exclure Rejected).

**Échange de statuts :**
1. Les COGs (et Trackers) s'échangent des `cog_list` ou `status_update` selon le protocole et la politique (périodicité, déclencheur).
2. Chaque COG met à jour sa liste locale et peut appliquer des règles (filtrer, dégrader, rejeter) selon la matrice des statuts (section 4).

### 3.3 Règles de transport

- **Transport** : le protocole MWS ne impose pas un transport unique (ex. HTTP(S), WebSocket, UDP, autre). La norme peut spécifier un ou plusieurs **bindings** (transport + encodage).
- **Sécurité du transport** : recommandation de confidentialité et d'intégrité en transit (ex. TLS) pour limiter l'écoute et la modification.
- **Port officiel des Trackers** : les COGs Tracker MWS exposent leur endpoint sur le **port 21000** (voir section 2.7.4). Les COGs participants se connectent aux Trackers sur `host:21000` par défaut (ou sur l'adresse complète indiquée en config si override).
- **Découverte des Trackers** : les COGs doivent pouvoir connaître l'adresse (host) des Trackers (config locale, bootstrap, ou annuaire connu). Le port par défaut pour joindre un Tracker est **21000**. Le détail du bootstrap est hors scope de ce document conceptuel.

---

## 4. Matrice des statuts et règles d'échange

### 4.1 Statuts normatifs

Les **statuts** de la Webway COG List sont **normatifs** pour l'interopérabilité. Valeurs standard :

| Statut | Code (orientation) | Signification | Usage typique |
|--------|---------------------|---------------|---------------|
| **Trusted** | `trusted` | COG considéré comme fiable pour la présence / découverte | Annonces acceptées, relayées |
| **Neutral** | `neutral` | Aucun signal positif ou négatif | Traité par défaut selon politique locale |
| **Under review** | `under_review` | En cours d'analyse (comportement suspect, signalement) | Limitation ou surveillance |
| **Distrusted** | `distrusted` | COG considéré comme non fiable | Annonces ou connexions dégradées / filtrées |
| **Rejected** | `rejected` | COG ou connexion rejetée (malveillant ou politique locale) | Refus d'annonce, blocage de connexion Webway |

### 4.2 Structure d'une entrée de liste (orientation)

| Champ | Type | Description |
|-------|------|-------------|
| `cog_id` | string | Identifiant du COG |
| `status` | string | Une des valeurs normatives ci-dessus |
| `source` | string | Identifiant du COG ou Tracker ayant fourni/mis à jour le statut |
| `updated_at` | string (ISO 8601) | Dernière mise à jour du statut |
| Optionnel | `reason_hint` | Indication courte (ex. code) — pas de donnée sensible |

### 4.3 Règles d'échange entre COGs

- **Souveraineté** : chaque COG reste libre d'ignorer un statut fourni par un autre et d'appliquer sa propre politique (analyser, rejeter, accepter).
- **Pas de données métier** : l'échange de listes ne contient que des identifiants COG, des statuts et des métadonnées de traçabilité.
- **Agrégation** : les COGs Tracker peuvent agréger les signaux de plusieurs sources pour mettre à jour ou proposer des statuts (selon contrats passifs/actifs).
- **Propagation** : les règles de propagation (qui envoie à qui, fréquence, périmètre) sont définies par le protocole MWS ou par politique locale ; la norme impose la **structure** et les **valeurs de statut**, pas la topologie d'échange.

### 4.4 Comportement attendu selon statut

| Statut | Annonces du COG | Requêtes du COG | Réponse des Trackers / participants |
|--------|------------------|-----------------|--------------------------------------|
| Trusted | Acceptées, relayées | Traitées normalement | Inclus dans les réponses de découverte |
| Neutral | Acceptées par défaut | Traitées par défaut | Inclus selon politique |
| Under review | Limitées ou surveillées | Limitées ou surveillées | Dégradation possible (throttle, délai) |
| Distrusted | Filtrées / dégradées | Filtrées / dégradées | Exclus ou dégradés selon politique |
| Rejected | Refusées | Refusées ou bloquées | Exclus des réponses, blocage connexion Webway |

Les comportements précis (ex. seuils, durées) relèvent des contrats des systèmes passifs et actifs des Trackers et des politiques locales.

---

## 5. Standards de conformité pour les COGs Tracker

Les **COGs Tracker** doivent respecter des **exigences minimales** pour participer au maillage en tant que Tracker et assurer la cohérence des normes.

### 5.1 Exigences minimales

| Exigence | Description |
|----------|-------------|
| **Norme de déclaration** | Accepter et vérifier uniquement les déclarations conformes à la norme de déclaration sécurisée (MWS) en vigueur. |
| **Formats** | Comprendre et traiter les formats de messages normatifs (annonces, requêtes, listes de statuts) selon la version supportée. |
| **Statuts** | Maintenir et échanger des listes de COGs avec les statuts normatifs ; appliquer les règles d'échange (section 4). |
| **Protection du réseau** | Mettre en œuvre des mécanismes **passifs** et **actifs** conformes aux contrats MWS (à créer) pour protéger le maillage. |
| **Pas de gouvernance** | Ne jamais décider d'accès (Visa, Passeport) ; ne pas exposer de données métier ni de gouvernance. |

### 5.2 Vérification de conformité

- La conformité peut être **vérifiée** par des tests ou des audits (formats, comportement face à déclarations valides/invalides, respect des statuts).
- Un COG Tracker **non conforme** peut être **signalé** (statut Distrusted ou Rejected) par d'autres COGs ou Trackers selon les règles d'échange.
- Les exigences détaillées (critères de test, niveaux de conformité) seront définies dans un contrat ou un protocole MWS dédié.

### 5.3 Versionnement supporté

- Chaque Tracker doit **annoncer** les **versions de norme** qu'il supporte (ex. `mws_declaration_v1`, `mws_protocol_v1`).
- Les déclarations émises avec une version non supportée peuvent être **refusées** ou **ignorées**.
- La **rétrocompatibilité** et les fenêtres de dépréciation sont définies par la gouvernance du MWS (évolution des normes).

---

## 6. Versionnement et évolution des normes

### 6.1 Version des normes

- **Norme de déclaration sécurisée** : versionnée (ex. `mws_declaration_v1`). Toute évolution non rétrocompatible doit incrémenter la version majeure.
- **Formats de messages** : version indiquée dans chaque message (`version`). Les anciennes versions peuvent être supportées pendant une période de dépréciation.
- **Statuts** : les valeurs normatives sont stables ; de nouveaux statuts peuvent être ajoutés avec extension (ex. statut optionnel) sans invalider les existants.

### 6.2 Rétrocompatibilité

- Les **nouveaux champs** (optionnels) ne doivent pas casser les implémentations existantes.
- Les **champs supprimés** ou modifiés de manière incompatible doivent suivre une période de dépréciation annoncée.
- Les Trackers et participants sont encouragés à supporter au moins la version courante et la version précédente pendant la transition.

### 6.3 Gouvernance des normes

- L'**évolution** des normes et standards du MWS (nouveaux formats, nouveaux statuts, modifications de la norme de déclaration) relève d'un processus de gouvernance à définir (communauté, mainteneurs, release).
- Ce document conceptuel fixe le **cadre** ; les versions numérotées et les changelogs seront maintenus dans des documents ou dépôts dédiés.

---

## 7. Synthèse

| Domaine | Norme / Standard | Statut |
|---------|------------------|--------|
| **Déclaration sécurisée** | Authentification, intégrité, format unifié, limitation des abus | À créer et appliquer (cadre défini) |
| **Formats de messages** | Annonce présence, services, session hébergée, requête découverte, liste statuts | Schémas à finaliser (orientation donnée) |
| **Ports utilisables / exclus** | Liste normative ports exclus (0–1023 + ports courants web/dev/DB) ; ports utilisables pour MWS | Défini conceptuellement (section 2.7) |
| **Protocole MWS** | Types de messages, séquences, règles de transport | Orientation ; bindings à spécifier |
| **Matrice des statuts** | Valeurs normatives, structure d'entrée, règles d'échange, comportement attendu | Défini conceptuellement |
| **Conformité Trackers** | Exigences minimales, vérification, versionnement supporté | Cadre défini ; critères détaillés à créer |
| **Versionnement** | Version des normes, rétrocompatibilité, gouvernance de l'évolution | Principes définis |

---

## Références croisées

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) — document principal
- [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) — annexe conceptuelle (Outils, Kits d'Outils, Opérateurs MWS)
- [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- [Doctrine Securite Fondamentale](./Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Norme de déclaration sécurisée MWS, Déclaration d'hébergement de session, COG Tracker, Liste de COGs avec statuts)

---

*Document créé le 30/01/2026*  
*Classification : Reference conceptuelle — Annexe MWS (Normes et Standards)*
