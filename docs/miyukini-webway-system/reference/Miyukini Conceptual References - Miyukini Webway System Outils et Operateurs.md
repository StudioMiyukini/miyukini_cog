# Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs

## Contexte

Ce document est un **annexe conceptuel** au [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md). Il dÃ©veloppe les **Outils (Strate 6)** et les **OpÃ©rateurs (Strate 7)** nÃ©cessaires au MWS : capacitÃ©s exÃ©cutables gouvernÃ©es pour la prÃ©sence et la dÃ©couverte, et entitÃ©s fonctionnelles qui les orchestrent.

**Principe directeur :**

> **Les Outils MWS font (construire, valider, envoyer, recevoir) ; ils ne dÃ©cident jamais. Les OpÃ©rateurs MWS exÃ©cutent les rÃ´les Participant et Tracker en s'appuyant sur ces Outils et sur la gouvernance des Cores.**

## PortÃ©e / Scope

- **Outils MWS** : capacitÃ©s atomiques pour dÃ©clarations, validation, transport, dÃ©couverte, listes de COGs, ports
- **Kits d'Outils MWS** : compositions officielles (Kit Participant Webway, Kit Tracker Webway)
- **OpÃ©rateurs MWS** : OpÃ©rateur Participant Webway, OpÃ©rateur Tracker Webway â€” rÃ´les, responsabilitÃ©s, dÃ©pendances aux Cores
- Positionnement par rapport Ã  Master Butler, Border Guard, WorrySentinel, BondingBrother

Ce document **ne couvre pas** :
- Le dÃ©tail des normes et standards (formats, protocole) â†’ voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- La gouvernance des visites (Passeport, Permis de circulation, Visa de Connexion) â†’ voir [Connexion Inter-COG](_index.md)
- Les spÃ©cifications d'implÃ©mentation (binding transport, librairies) â†’ contrats ou specs techniques dÃ©diÃ©s

---

## 1. Principes : Outils et OpÃ©rateurs dans le MWS

### 1.1 Rappel : Outil (Strate 6)

> **Un Outil est une capacitÃ© exÃ©cutable, sans autoritÃ©, sans dÃ©cision mÃ©tier, sans connaissance de l'OpÃ©rateur appelant, gouvernÃ©e par les Cores.**

**RÃ¨gle :** un Outil **fait**, mais ne **dÃ©cide** jamais. Les dÃ©cisions (quand annoncer, accepter ou rejeter une dÃ©claration, Ã  quel Tracker envoyer) relÃ¨vent des **Cores** (StrongFather, Border Guard, WorrySentinel) et sont traduites en **intentions** par BondingBrother ; les OpÃ©rateurs invoquent les Outils pour **exÃ©cuter** ces intentions.

**Voir aussi :** [Tools et Toolkits](_index.md), [Glossaire](_index.md) (Outil, Kit d'Outils).

### 1.2 Rappel : OpÃ©rateur (Strate 7)

> **Un OpÃ©rateur est une entitÃ© fonctionnelle gouvernÃ©e qui exÃ©cute un rÃ´le pour le compte de l'utilisateur au sein d'un environnement Miyukini.**

Les **OpÃ©rateurs MWS** exÃ©cutent les rÃ´les **Participant Webway** et **Tracker Webway** : ils utilisent les **Outils MWS** (et les Kits d'Outils MWS) pour rÃ©aliser les actions de prÃ©sence et de dÃ©couverte ; les **dÃ©cisions** (autorisation d'annoncer, politique de rejet, choix des Trackers) viennent des **Cores** via BondingBrother.

**Voir aussi :** [Glossaire](_index.md) (OpÃ©rateur, OpÃ©rateur de Service).

### 1.3 Gouvernance des Outils et OpÃ©rateurs MWS

| Core | RÃ´le dans le MWS |
|------|------------------|
| **Master Butler** | DÃ©clare les capacitÃ©s MWS (Outils, Kits) ; dÃ©finit les permissions d'accÃ¨s aux Outils Participant / Tracker |
| **Border Guard** | RÃ¨gles de qui peut s'annoncer, interroger le maillage, utiliser quels Trackers ; politique des ports et des adresses |
| **WorrySentinel** | Niveau de confiance, signaux issus des listes de statuts ; peut bloquer ou dÃ©grader la participation MWS si l'environnement est dÃ©gradÃ© |
| **StrongFather** | DÃ©cision d'autoriser ou non la participation au Webway, le rÃ´le Tracker ; Ã©mission des Mandats pour les OpÃ©rateurs MWS |
| **BondingBrother** | Traduction des intentions (annoncer, dÃ©couvrir, rejeter) en appels aux OpÃ©rateurs et Outils MWS |

---

## 2. Outils MWS (orientation)

Les **Outils MWS** sont des capacitÃ©s exÃ©cutables gouvernÃ©es (Strate 6) utilisÃ©es par les OpÃ©rateurs Participant Webway et Tracker Webway. Ils sont **dÃ©clarÃ©s** par Master Butler et **autorisÃ©s** par StrongFather selon les Mandats et les politiques (Border Guard, WorrySentinel).

### 2.1 Outils de dÃ©claration (build, validate, sign, verify)

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.declaration.build** | Construire un message de dÃ©claration conforme au format MWS (prÃ©sence, service, session hÃ©bergÃ©e) | type, cog_id, payload (adresse, services, session_idâ€¦), version norme | message structurÃ© (prÃªt pour signature) | Participant, Tracker |
| **mws.declaration.validate** | Valider le format et les contraintes d'une dÃ©claration (champs obligatoires, types, ports non exclus) | message | ok / erreurs de validation | Tracker, Participant |
| **mws.declaration.sign** | Signer une dÃ©claration (intÃ©gritÃ©, authentification origine) selon la norme MWS | message, clÃ© / secret (gouvernÃ©) | message + bloc d'intÃ©gritÃ© | Participant, Tracker |
| **mws.declaration.verify** | VÃ©rifier la signature / intÃ©gritÃ© d'une dÃ©claration reÃ§ue | message | ok / Ã©chec | Tracker, Participant |

**RÃ¨gle :** ces Outils ne dÃ©cident pas *si* une dÃ©claration doit Ãªtre acceptÃ©e ou rejetÃ©e ; ils **construisent**, **valident** ou **vÃ©rifient**. La dÃ©cision d'accepter ou rejeter relÃ¨ve des Cores (Border Guard, WorrySentinel) et des politiques appliquÃ©es par l'OpÃ©rateur Tracker.

### 2.2 Outils de transport (send, receive)

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.transport.send** | Envoyer un message (dÃ©claration, requÃªte de dÃ©couverte, liste de statuts) vers une adresse (Tracker ou COG) | message, adresse (host, port) | succÃ¨s / erreur transport | Participant, Tracker |
| **mws.transport.receive** | Recevoir un message sur un endpoint (Ã©coute) | endpoint (host, port) â€” ex. port 21000 pour Tracker | message reÃ§u / timeout / erreur | Tracker |

**RÃ¨gle :** le transport ne modifie pas le message ; il **transporte** uniquement. La sÃ©curitÃ© du transport (ex. TLS) relÃ¨ve du binding dÃ©fini dans les normes.

### 2.3 Outils de dÃ©couverte (request, response)

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.discovery.request.build** | Construire une requÃªte de dÃ©couverte conforme au format MWS | requester_cog_id, query (critÃ¨res), version | message requÃªte (prÃªt pour envoi) | Participant, Tracker |
| **mws.discovery.request.send** | Envoyer une requÃªte de dÃ©couverte vers un ou plusieurs Trackers | requÃªte, adresse(s) Tracker(s) | succÃ¨s / erreur | Participant |
| **mws.discovery.response.build** | Construire une rÃ©ponse de dÃ©couverte (liste de COGs, services, sessions) en respectant les listes de statuts (ex. exclure Rejected) | entrÃ©es (filtrÃ©es par politique), version | message rÃ©ponse | Tracker |
| **mws.discovery.response.send** | Envoyer la rÃ©ponse au demandeur | rÃ©ponse, adresse demandeur | succÃ¨s / erreur | Tracker |

**RÃ¨gle :** le filtrage (qui exclure selon statut) est une **politique** fournie par les Cores (Border Guard, WorrySentinel) ; l'Outil **applique** la liste filtrÃ©e sans dÃ©cider lui-mÃªme du statut.

### 2.4 Outils de liste de COGs (get, update, merge, filter)

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.cog_list.get** | Lire une entrÃ©e ou la liste locale de COGs avec statuts | cog_id (optionnel) | entrÃ©e(s) (cog_id, status, source, updated_at) | Participant, Tracker |
| **mws.cog_list.update** | Mettre Ã  jour une entrÃ©e (statut, source, updated_at) dans la liste locale | cog_id, status, source | ok / erreur | Participant, Tracker |
| **mws.cog_list.merge** | Fusionner une liste reÃ§ue avec la liste locale selon une rÃ¨gle fournie (ex. garder le statut le plus restrictif) | liste reÃ§ue, rÃ¨gle de fusion (gouvernÃ©e) | liste fusionnÃ©e / delta appliquÃ© | Participant, Tracker |
| **mws.cog_list.filter** | Filtrer la liste selon un critÃ¨re (ex. exclure Rejected, exclure Distrusted) | liste, critÃ¨re (gouvernÃ©) | liste filtrÃ©e | Tracker (pour discovery.response) |

**RÃ¨gle :** la **rÃ¨gle de fusion** et le **critÃ¨re de filtrage** viennent des Cores (Border Guard, WorrySentinel) ; l'Outil **applique** sans dÃ©cider de la politique.

### 2.5 Outils d'adresse et de port

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.port.check** | VÃ©rifier si un port est dans la liste normative des ports exclus MWS | port (integer) | true (exclus) / false (utilisable) | Participant, Tracker |
| **mws.address.tracker_default** | RÃ©soudre l'adresse complÃ¨te d'un Tracker Ã  partir d'un host (port officiel 21000) | host | adresse (host, 21000) | Participant |

**RÃ¨gle :** ces Outils sont **dÃ©terministes** et **sans Ã©tat mÃ©tier** ; la liste des ports exclus est **versionnÃ©e** avec la norme (voir Normes et Standards, section 2.7).

### 2.6 Outils relay (Webway Relay)

Les **Outils relay** permettent Ã  un COG d'utiliser le **relay Miyukini Webway** (tunnel Ã©tendu multi-tenant) pour Ãªtre joignable derriÃ¨re NAT ou pour joindre un autre COG via le relay. Ils s'appuient sur le [protocole relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) (REGISTER, CONNECT, HEARTBEAT, etc.).

| Outil | Action | EntrÃ©es (orientation) | Sorties (orientation) | UtilisÃ© par |
|-------|--------|------------------------|------------------------|-------------|
| **relay.register** | Enregistrer un tunnel vers le relay (connexion persistante associÃ©e au cog_id) | relay_adresse (host, port), token, cog_id, options (TLS, timeouts) | ok (tunnel enregistrÃ©) / REGISTER_ERR (code, message) | Participant (COG derriÃ¨re NAT) |
| **relay.connect** | Ã‰tablir une connexion vers un cog_id cible via le relay (cÃ´tÃ© appelant) | relay_adresse, cog_id_cible, options (TLS) | ok (connexion logique Ã©tablie, prÃªt pour DATA) / CONNECT_ERR | Participant (COG joignant un autre COG via relay) |
| **relay.heartbeat** | Envoyer un HEARTBEAT sur le tunnel pour maintenir l'enregistrement et dÃ©tecter les dÃ©connexions | tunnel (rÃ©fÃ©rence au tunnel enregistrÃ©) | ok (HEARTBEAT_ACK reÃ§u) / timeout / erreur | Participant (COG ayant enregistrÃ© un tunnel) |

**RÃ¨gle :** les Outils relay **ne dÃ©cident pas** si le COG doit s'enregistrer ni vers quel relay ; ils **exÃ©cutent** l'enregistrement, la connexion ou le heartbeat. La dÃ©cision d'utiliser le relay et le choix de l'adresse relay relÃ¨vent des Cores (Border Guard, StrongFather). Le transport vers le relay utilise TLS (voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md)).

### 2.7 SynthÃ¨se des Outils MWS

| Domaine | Outils |
|---------|--------|
| **DÃ©claration** | mws.declaration.build, mws.declaration.validate, mws.declaration.sign, mws.declaration.verify |
| **Transport** | mws.transport.send, mws.transport.receive |
| **DÃ©couverte** | mws.discovery.request.build, mws.discovery.request.send, mws.discovery.response.build, mws.discovery.response.send |
| **Liste COG** | mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge, mws.cog_list.filter |
| **Adresse / port** | mws.port.check, mws.address.tracker_default |
| **Relay** | relay.register, relay.connect, relay.heartbeat |

### 2.8 Contrats formels des Outils MWS

Les contrats ci-dessous fixent les **signatures**, **prÃ©conditions** et **postconditions** pour chaque Outil MWS et relay. Ils sont dÃ©clarÃ©s par Master Butler et respectÃ©s par les implÃ©mentations (Kits, OpÃ©rateurs).

**Convention :** *prÃ©conditions* = conditions requises avant l'appel ; *postconditions* = garanties en cas de succÃ¨s ; *invariants* = propriÃ©tÃ©s prÃ©servÃ©es. Les dÃ©cisions mÃ©tier (accepter, rejeter, filtrer) restent hors du contrat â€” elles sont fournies par les Cores.

#### 2.8.1 Outils de dÃ©claration

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **mws.declaration.build** | `(type, cog_id, payload, version_norme) â†’ message` | type, cog_id, version_norme fournis ; payload conforme au type | message contient tous les champs obligatoires du format MWS pour ce type ; message prÃªt pour signature |
| **mws.declaration.validate** | `(message) â†’ ok \| erreurs` | message non vide, structure parsable | si ok : message conforme (champs, types, ports non exclus) ; sinon : liste d'erreurs de validation non vide |
| **mws.declaration.sign** | `(message, clÃ©/secret) â†’ message_signÃ©` | message valide (build ou validate) ; clÃ©/secret gouvernÃ© disponible | message_signÃ© inclut bloc d'intÃ©gritÃ© vÃ©rifiable ; origine authentifiable |
| **mws.declaration.verify** | `(message) â†’ ok \| Ã©chec` | message contient bloc d'intÃ©gritÃ© | si ok : signature valide, intÃ©gritÃ© prÃ©servÃ©e ; sinon : Ã©chec (signature invalide ou absente) |

#### 2.8.2 Outils de transport

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **mws.transport.send** | `(message, adresse) â†’ succÃ¨s \| erreur` | message sÃ©rialisable ; adresse (host, port) valide | message transmis sur le canal vers adresse ; pas de modification du contenu |
| **mws.transport.receive** | `(endpoint) â†’ message \| timeout \| erreur` | endpoint (host, port) en Ã©coute, liaison rÃ©ussie | si message : message reÃ§u complet ; timeout si aucune donnÃ©e dans la fenÃªtre configurÃ©e |

#### 2.8.3 Outils de dÃ©couverte

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **mws.discovery.request.build** | `(requester_cog_id, query, version) â†’ requÃªte` | requester_cog_id, version fournis | requÃªte conforme au format MWS discovery request ; prÃªte pour envoi |
| **mws.discovery.request.send** | `(requÃªte, adresse(s)) â†’ succÃ¨s \| erreur` | requÃªte construite ; au moins une adresse Tracker | requÃªte envoyÃ©e vers chaque adresse (transport.send) |
| **mws.discovery.response.build** | `(entrÃ©es_filtrÃ©es, version) â†’ rÃ©ponse` | entrÃ©es dÃ©jÃ  filtrÃ©es selon politique (fournie par Cores) ; version fournie | rÃ©ponse conforme au format MWS discovery response ; contient uniquement les entrÃ©es fournies |
| **mws.discovery.response.send** | `(rÃ©ponse, adresse_demandeur) â†’ succÃ¨s \| erreur` | rÃ©ponse construite ; adresse demandeur valide | rÃ©ponse envoyÃ©e vers adresse_demandeur |

#### 2.8.4 Outils de liste COG

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **mws.cog_list.get** | `(cog_id?) â†’ entrÃ©e(s)` | liste locale accessible | si cog_id : entrÃ©e correspondante (cog_id, status, source, updated_at) ou vide ; si omis : liste complÃ¨te |
| **mws.cog_list.update** | `(cog_id, status, source) â†’ ok \| erreur` | cog_id fourni ; status dans l'ensemble normatif (Trusted, Neutral, etc.) | entrÃ©e mise Ã  jour ; updated_at rafraÃ®chi |
| **mws.cog_list.merge** | `(liste_reÃ§ue, rÃ¨gle_fusion) â†’ liste_fusionnÃ©e \| delta` | liste_reÃ§ue et rÃ¨gle_fusion fournis (rÃ¨gle gouvernÃ©e) | liste fusionnÃ©e conforme Ã  la rÃ¨gle ; pas de suppression de statut sans politique |
| **mws.cog_list.filter** | `(liste, critÃ¨re) â†’ liste_filtrÃ©e` | liste et critÃ¨re fournis (critÃ¨re gouvernÃ©) | liste_filtrÃ©e âŠ† liste ; tous les Ã©lÃ©ments satisfont le critÃ¨re |

#### 2.8.5 Outils d'adresse et de port

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **mws.port.check** | `(port) â†’ true \| false` | port entier dans la plage valide | true si port dans la liste normative des ports exclus MWS ; false sinon |
| **mws.address.tracker_default** | `(host) â†’ (host, 21000)` | host non vide | adresse (host, 21000) ; 21000 est le port officiel Tracker MWS |

#### 2.8.6 Outils relay

| Outil | Signature (orientation) | PrÃ©conditions | Postconditions (succÃ¨s) |
|-------|-------------------------|---------------|-------------------------|
| **relay.register** | `(relay_adresse, token, cog_id, options?) â†’ ok \| REGISTER_ERR` | Connexion TCP+TLS vers relay possible ; token et cog_id fournis | Tunnel enregistrÃ© cÃ´tÃ© relay ; connexion persistante associÃ©e Ã  cog_id ; REGISTER_OK reÃ§u |
| **relay.connect** | `(relay_adresse, cog_id_cible, options?) â†’ ok \| CONNECT_ERR` | Connexion TCP+TLS vers relay possible ; cog_id_cible fourni | Connexion logique vers cog_id_cible Ã©tablie ; CONNECT_OK reÃ§u ; prÃªt pour Ã©change DATA |
| **relay.heartbeat** | `(tunnel) â†’ ok \| timeout \| erreur` | tunnel rÃ©fÃ©rence une connexion dÃ©jÃ  enregistrÃ©e (relay.register) | HEARTBEAT envoyÃ© ; HEARTBEAT_ACK reÃ§u dans le dÃ©lai configurÃ© ; tunnel maintenu actif |

**Invariant commun aux Outils :** aucun Outil ne prend de dÃ©cision d'autorisation, de rejet ou de politique ; il exÃ©cute une capacitÃ© et retourne un rÃ©sultat. Les choix (quand annoncer, accepter, filtrer) sont fournis par les Cores via BondingBrother aux OpÃ©rateurs.

Les noms et signatures dÃ©taillÃ©s (types exacts, codes d'erreur) sont fixÃ©s dans les contrats d'implÃ©mentation (Master Butler, contrats Outils MWS) et dans le [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) pour les outils relay.

---

## 3. Kits d'Outils MWS (orientation)

Les **Kits d'Outils MWS** sont des **compositions officielles** d'Outils MWS, validÃ©es et dÃ©clarÃ©es par l'environnement (Master Butler), optimisÃ©es pour l'efficience et la cohÃ©rence. Ils n'ajoutent **aucune capacitÃ© nouvelle** ; ils **orchestrent** les Outils.

### 3.1 Kit Participant Webway (MWS Participant Toolkit)

**Usage :** fournir Ã  l'OpÃ©rateur Participant Webway l'ensemble des Outils nÃ©cessaires pour **participer** au maillage (annoncer, dÃ©couvrir, maintenir la liste de statuts) et, le cas Ã©chÃ©ant, utiliser le **relay Webway** pour Ãªtre joignable derriÃ¨re NAT ou pour joindre un autre COG via le relay.

**Composition (orientation) :**
- mws.declaration.build, mws.declaration.sign, mws.declaration.validate, mws.declaration.verify
- mws.transport.send
- mws.discovery.request.build, mws.discovery.request.send
- mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge
- mws.port.check, mws.address.tracker_default
- **relay.register**, **relay.heartbeat** (quand le COG sâ€™annonce via relay ; tunnel persistant)
- **relay.connect** (quand le Participant initie une connexion vers un COG joignable uniquement via relay)

**RÃ¨gle :** le Kit Participant **ne dÃ©cide pas** quand annoncer ni Ã  quels Trackers ou relay envoyer ; il fournit les capacitÃ©s. Les **dÃ©cisions** sont fournies par les Cores via BondingBrother Ã  l'OpÃ©rateur Participant Webway.

### 3.2 Kit Tracker Webway (MWS Tracker Toolkit)

**Usage :** fournir Ã  l'OpÃ©rateur Tracker Webway l'ensemble des Outils nÃ©cessaires pour **tenir le rÃ´le Tracker** (recevoir, valider, enregistrer, rÃ©pondre aux requÃªtes de dÃ©couverte, maintenir et Ã©changer les listes de statuts, appliquer les mÃ©canismes passifs et actifs).

**Composition (orientation) :**
- mws.declaration.validate, mws.declaration.verify
- mws.transport.receive, mws.transport.send
- mws.discovery.response.build, mws.discovery.response.send
- mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge, mws.cog_list.filter
- mws.port.check, mws.address.tracker_default (pour communication Tracker-Ã -Tracker si besoin)

**RÃ¨gle :** le Kit Tracker **ne dÃ©cide pas** d'accepter ou rejeter une dÃ©claration ; il fournit les capacitÃ©s de validation, vÃ©rification, filtrage. Les **dÃ©cisions** (politique de rejet, statuts) viennent des Cores (Border Guard, WorrySentinel) et sont appliquÃ©es par l'OpÃ©rateur Tracker Webway.

**MÃ©canismes passifs et actifs :** les Outils de liste (merge, filter) et de transport (receive, send) sont utilisÃ©s par l'OpÃ©rateur Tracker pour appliquer les **systÃ¨mes passifs** (observer, signaler, alimenter les listes) et **actifs** (filtrer, rejeter) ; les contrats dÃ©taillÃ©s des systÃ¨mes passifs/actifs sont dÃ©finis dans [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

### 3.3 Kit Relay Webway (MWS Relay Toolkit)

**Usage :** fournir l'ensemble des Outils nÃ©cessaires pour **utiliser le relay Miyukini Webway** : enregistrement d'un tunnel (COG derriÃ¨re NAT), maintien du tunnel par heartbeat, et connexion vers un COG cible via le relay (cÃ´tÃ© appelant). Ce Kit est typiquement **composÃ©** dans le Kit Participant Webway lorsque le COG participe via relay ; il peut aussi Ãªtre rÃ©fÃ©rencÃ© sÃ©parÃ©ment pour les implÃ©mentations qui isolent la couche relay.

**Composition (orientation) :**
- relay.register
- relay.connect
- relay.heartbeat

**RÃ¨gle :** le Kit Relay Webway **ne dÃ©cide pas** si le COG doit s'enregistrer ni vers quel relay ; il fournit les capacitÃ©s. Les **dÃ©cisions** (utilisation du relay, adresse relay, token) viennent des Cores (Border Guard, StrongFather). Voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) et [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md).

---

## 4. OpÃ©rateurs MWS (orientation)

Les **OpÃ©rateurs MWS** sont des **OpÃ©rateurs de Service** (Strate 7) qui exÃ©cutent les rÃ´les **Participant Webway** et **Tracker Webway** au sein d'un environnement Miyukini. Ils sont gouvernÃ©s par les Cores et utilisent les Outils et Kits d'Outils MWS.

### 4.1 OpÃ©rateur Participant Webway (Webway Participant Operator)

**Nom conceptuel (orientation) :** *OpÃ©rateur Participant Webway* (ou *MiyuWebwayParticipant* si nommage produit).

**RÃ´le :**
- **Se dÃ©clarer** : annoncer la prÃ©sence du COG (identitÃ©, adresse du Bridge) auprÃ¨s d'un ou plusieurs Trackers selon les dÃ©cisions fournies par les Cores.
- **Annoncer services et adresses** : publier les services exposÃ©s et les adresses (IP/ports) conformes Ã  la norme MWS et Ã  la politique (ports exclus, format).
- **DÃ©clarer une session hÃ©bergÃ©e** : lorsqu'il agit comme COG HÃ©bergeur, annoncer Â« j'hÃ©berge une session de tel service et j'attends des connexions Â» sur le maillage.
- **DÃ©couvrir** : envoyer des requÃªtes de dÃ©couverte aux Trackers (selon autorisation et politique), traiter les rÃ©ponses.
- **Participer Ã  la sÃ©curitÃ© du maillage** : maintenir la liste locale de COGs avec statuts, Ã©changer des mises Ã  jour avec les Trackers ou d'autres COGs selon le protocole et la politique.

**CapacitÃ©s utilisÃ©es :**
- **Kit Participant Webway** (voir section 3.1).
- Outils MWS : build, sign, validate, verify, transport.send, discovery.request.build/send, cog_list.get/update/merge, port.check, address.tracker_default.

**Gouvernance :**
- **StrongFather** : autorise ou non la participation au Webway (Mandat, politique).
- **Border Guard** : rÃ¨gles de qui peut s'annoncer, quels Trackers utiliser, quels ports/adresses exposer.
- **WorrySentinel** : peut bloquer ou dÃ©grader la participation si l'environnement est en Ã©tat dÃ©gradÃ© (T2, T3, T4).
- **BondingBrother** : reÃ§oit les intentions (annoncer, dÃ©couvrir, mettre Ã  jour la liste) et les traduit en appels Ã  l'OpÃ©rateur Participant Webway ; l'OpÃ©rateur invoque les Outils.

**RÃ¨gle :** l'OpÃ©rateur Participant Webway **n'a pas d'autoritÃ©** sur le contenu des annonces ni sur le choix des Trackers ; il **exÃ©cute** les dÃ©cisions des Cores. Il ne transmet **aucune donnÃ©e mÃ©tier** ni gouvernance via le Webway.

### 4.2 OpÃ©rateur Tracker Webway (Webway Tracker Operator)

**Nom conceptuel (orientation) :** *OpÃ©rateur Tracker Webway* (ou *MiyuWebwayTracker* si nommage produit).

**RÃ´le :**
- **Point de rendez-vous** : exposer l'endpoint sur le **port 21000** (ou adresse configurÃ©e), recevoir les annonces (prÃ©sence, services, sessions hÃ©bergÃ©es), les enregistrer et les rendre dÃ©couvrables.
- **RÃ©pondre aux requÃªtes de dÃ©couverte** : traiter les discovery_request, construire et envoyer les discovery_response en appliquant les listes de statuts (ex. exclure Rejected, filtrer selon politique).
- **ProtÃ©ger le rÃ©seau** : appliquer les **systÃ¨mes passifs** (observer, enregistrer, signaler, alimenter les listes de statuts) et **actifs** (refuser les annonces non conformes ou provenant de COGs Rejected/Distrusted, filtrer les rÃ©ponses).
- **Maintenir et Ã©changer les listes de COGs avec statuts** : mettre Ã  jour la liste locale, fusionner les listes reÃ§ues selon la politique, envoyer des mises Ã  jour aux autres Trackers ou COGs selon le protocole.
- **ConformitÃ©** : exiger la conformitÃ© Ã  la norme de dÃ©claration sÃ©curisÃ©e (format, intÃ©gritÃ©, ports non exclus) ; rejeter ou ignorer les annonces non conformes.

**CapacitÃ©s utilisÃ©es :**
- **Kit Tracker Webway** (voir section 3.2).
- Outils MWS : validate, verify, transport.receive/send, discovery.response.build/send, cog_list.get/update/merge/filter, port.check.

**Gouvernance :**
- **StrongFather** : autorise ou non le rÃ´le Tracker pour ce COG (Mandat, politique).
- **Border Guard** : rÃ¨gles d'acceptation des annonces, politique de filtrage, liste des ports exclus, politique de propagation des statuts.
- **WorrySentinel** : signaux de confiance, politique de rejet (Distrusted, Rejected), dÃ©gradation en cas d'environnement dÃ©gradÃ©.
- **BondingBrother** : reÃ§oit les intentions (accepter, rejeter, mettre Ã  jour statut, rÃ©pondre Ã  une requÃªte) et les traduit en appels Ã  l'OpÃ©rateur Tracker Webway ; l'OpÃ©rateur invoque les Outils.

**RÃ¨gle :** l'OpÃ©rateur Tracker Webway **ne gouverne pas** les accÃ¨s (Passeport, Permis de circulation, Visa de Connexion) ; il **protÃ¨ge le maillage** en appliquant les politiques des Cores. Il ne transmet **aucune donnÃ©e mÃ©tier** ni gouvernance ; il ne fait qu'exposer la **prÃ©sence** et la **dÃ©couverte**.

### 4.3 SynthÃ¨se des OpÃ©rateurs MWS

| OpÃ©rateur | RÃ´le principal | Kit d'Outils | Port / exposition |
|-----------|----------------|--------------|-------------------|
| **Participant Webway** | Annoncer, dÃ©couvrir, maintenir liste de statuts ; optionnellement relay (tunnel, heartbeat, connect) | Kit Participant Webway (inclut Kit Relay Webway si relay utilisÃ©) | Pas d'Ã©coute obligatoire (client vers Trackers / relay) |
| **Tracker Webway** | Point de rendez-vous, dÃ©couverte, protection (passif/actif) | Kit Tracker Webway | **21000** (officiel) |

---

## 5. Positionnement dans la pyramide Miyukini

```
        OpÃ©rateurs (Strate 7)
        Participant Webway / Tracker Webway
               â†“
        Kits d'Outils MWS (Strate 6)
        Kit Participant Webway / Kit Tracker Webway / Kit Relay Webway
               â†“
        Outils MWS (Strate 6)
        mws.declaration.*, mws.transport.*, mws.discovery.*, mws.cog_list.*, mws.port.*, mws.address.*, relay.*
               â†“
        BondingBrother (Strate 5) â€” intentions â†’ appels OpÃ©rateurs
               â†“
        Cores (Strate 4) â€” Master Butler (capacitÃ©s), Border Guard (rÃ¨gles), WorrySentinel (confiance), StrongFather (autorisation)
               â†“
        Kernel (Strate 0â€“3)
```

Les **Outils MWS** et les **OpÃ©rateurs MWS** sont **dÃ©clarÃ©s** par Master Butler et **autorisÃ©s** par StrongFather ; les **rÃ¨gles** (ports, statuts, rejet) sont dÃ©finies par Border Guard et WorrySentinel.

---

## 6. Ã‰volutions futures

- [x] Formaliser les **contrats d'Outils MWS** (signatures, prÃ©conditions, postconditions) et les faire dÃ©clarer par Master Butler â€” voir section 2.8 ; ajout des **Outils relay** (relay.register, relay.connect, relay.heartbeat) et **Kit Relay Webway**.
- [ ] Formaliser les **contrats des OpÃ©rateurs** Participant Webway et Tracker Webway (frontiÃ¨res, intÃ©gration BondingBrother, Border Guard, WorrySentinel).
- [x] DÃ©finir les **contrats des systÃ¨mes passifs et actifs** des Trackers â€” [Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).
- [ ] SpÃ©cifier le **nommage produit** des OpÃ©rateurs et Kits (ex. MiyuWebwayParticipant, MiyuWebwayTracker) et les enregistrer dans le registre d'OpÃ©rateurs.

---

## RÃ©fÃ©rences croisÃ©es

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) â€” document principal
- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) â€” normes, formats, protocole, ports
- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) â€” architecture du relay, intÃ©gration MWS
- [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) â€” protocole relay (REGISTER, CONNECT, HEARTBEAT, etc.)
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) â€” dÃ©ploiement relay (VM, TLS, systemd, tests)
- [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) â€” contrats systÃ¨mes passifs
- [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) â€” contrats systÃ¨mes actifs
- [Tools et Toolkits](_index.md)
- [Operators et Terminologie](_index.md)
- [Glossaire](_index.md) (Outil, Kit d'Outils, OpÃ©rateur, Master Butler, Border Guard, WorrySentinel, COG Tracker, MWS, Relay Webway)

---

*Document crÃ©Ã© le 30/01/2026*  
*Classification : Reference conceptuelle â€” Annexe MWS (Outils et OpÃ©rateurs)*


