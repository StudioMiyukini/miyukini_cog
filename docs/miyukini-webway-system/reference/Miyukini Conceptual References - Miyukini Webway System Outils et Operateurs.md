# Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs

## Contexte

Ce document est un **annexe conceptuel** au [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md). Il développe les **Outils (Strate 6)** et les **Opérateurs (Strate 7)** nécessaires au MWS : capacités exécutables gouvernées pour la présence et la découverte, et entités fonctionnelles qui les orchestrent.

**Principe directeur :**

> **Les Outils MWS font (construire, valider, envoyer, recevoir) ; ils ne décident jamais. Les Opérateurs MWS exécutent les rôles Participant et Tracker en s'appuyant sur ces Outils et sur la gouvernance des Cores.**

## Portée / Scope

- **Outils MWS** : capacités atomiques pour déclarations, validation, transport, découverte, listes de COGs, ports
- **Kits d'Outils MWS** : compositions officielles (Kit Participant Webway, Kit Tracker Webway)
- **Opérateurs MWS** : Opérateur Participant Webway, Opérateur Tracker Webway — rôles, responsabilités, dépendances aux Cores
- Positionnement par rapport à Master Butler, Border Guard, WorrySentinel, BondingBrother

Ce document **ne couvre pas** :
- Le détail des normes et standards (formats, protocole) → voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- La gouvernance des visites (Passeport, Permis de circulation, Visa de Connexion) → voir [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- Les spécifications d'implémentation (binding transport, librairies) → contrats ou specs techniques dédiés

---

## 1. Principes : Outils et Opérateurs dans le MWS

### 1.1 Rappel : Outil (Strate 6)

> **Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.**

**Règle :** un Outil **fait**, mais ne **décide** jamais. Les décisions (quand annoncer, accepter ou rejeter une déclaration, à quel Tracker envoyer) relèvent des **Cores** (StrongFather, Border Guard, WorrySentinel) et sont traduites en **intentions** par BondingBrother ; les Opérateurs invoquent les Outils pour **exécuter** ces intentions.

**Voir aussi :** [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Outil, Kit d'Outils).

### 1.2 Rappel : Opérateur (Strate 7)

> **Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.**

Les **Opérateurs MWS** exécutent les rôles **Participant Webway** et **Tracker Webway** : ils utilisent les **Outils MWS** (et les Kits d'Outils MWS) pour réaliser les actions de présence et de découverte ; les **décisions** (autorisation d'annoncer, politique de rejet, choix des Trackers) viennent des **Cores** via BondingBrother.

**Voir aussi :** [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Opérateur, Opérateur de Service).

### 1.3 Gouvernance des Outils et Opérateurs MWS

| Core | Rôle dans le MWS |
|------|------------------|
| **Master Butler** | Déclare les capacités MWS (Outils, Kits) ; définit les permissions d'accès aux Outils Participant / Tracker |
| **Border Guard** | Règles de qui peut s'annoncer, interroger le maillage, utiliser quels Trackers ; politique des ports et des adresses |
| **WorrySentinel** | Niveau de confiance, signaux issus des listes de statuts ; peut bloquer ou dégrader la participation MWS si l'environnement est dégradé |
| **StrongFather** | Décision d'autoriser ou non la participation au Webway, le rôle Tracker ; émission des Mandats pour les Opérateurs MWS |
| **BondingBrother** | Traduction des intentions (annoncer, découvrir, rejeter) en appels aux Opérateurs et Outils MWS |

---

## 2. Outils MWS (orientation)

Les **Outils MWS** sont des capacités exécutables gouvernées (Strate 6) utilisées par les Opérateurs Participant Webway et Tracker Webway. Ils sont **déclarés** par Master Butler et **autorisés** par StrongFather selon les Mandats et les politiques (Border Guard, WorrySentinel).

### 2.1 Outils de déclaration (build, validate, sign, verify)

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.declaration.build** | Construire un message de déclaration conforme au format MWS (présence, service, session hébergée) | type, cog_id, payload (adresse, services, session_id…), version norme | message structuré (prêt pour signature) | Participant, Tracker |
| **mws.declaration.validate** | Valider le format et les contraintes d'une déclaration (champs obligatoires, types, ports non exclus) | message | ok / erreurs de validation | Tracker, Participant |
| **mws.declaration.sign** | Signer une déclaration (intégrité, authentification origine) selon la norme MWS | message, clé / secret (gouverné) | message + bloc d'intégrité | Participant, Tracker |
| **mws.declaration.verify** | Vérifier la signature / intégrité d'une déclaration reçue | message | ok / échec | Tracker, Participant |

**Règle :** ces Outils ne décident pas *si* une déclaration doit être acceptée ou rejetée ; ils **construisent**, **valident** ou **vérifient**. La décision d'accepter ou rejeter relève des Cores (Border Guard, WorrySentinel) et des politiques appliquées par l'Opérateur Tracker.

### 2.2 Outils de transport (send, receive)

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.transport.send** | Envoyer un message (déclaration, requête de découverte, liste de statuts) vers une adresse (Tracker ou COG) | message, adresse (host, port) | succès / erreur transport | Participant, Tracker |
| **mws.transport.receive** | Recevoir un message sur un endpoint (écoute) | endpoint (host, port) — ex. port 21000 pour Tracker | message reçu / timeout / erreur | Tracker |

**Règle :** le transport ne modifie pas le message ; il **transporte** uniquement. La sécurité du transport (ex. TLS) relève du binding défini dans les normes.

### 2.3 Outils de découverte (request, response)

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.discovery.request.build** | Construire une requête de découverte conforme au format MWS | requester_cog_id, query (critères), version | message requête (prêt pour envoi) | Participant, Tracker |
| **mws.discovery.request.send** | Envoyer une requête de découverte vers un ou plusieurs Trackers | requête, adresse(s) Tracker(s) | succès / erreur | Participant |
| **mws.discovery.response.build** | Construire une réponse de découverte (liste de COGs, services, sessions) en respectant les listes de statuts (ex. exclure Rejected) | entrées (filtrées par politique), version | message réponse | Tracker |
| **mws.discovery.response.send** | Envoyer la réponse au demandeur | réponse, adresse demandeur | succès / erreur | Tracker |

**Règle :** le filtrage (qui exclure selon statut) est une **politique** fournie par les Cores (Border Guard, WorrySentinel) ; l'Outil **applique** la liste filtrée sans décider lui-même du statut.

### 2.4 Outils de liste de COGs (get, update, merge, filter)

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.cog_list.get** | Lire une entrée ou la liste locale de COGs avec statuts | cog_id (optionnel) | entrée(s) (cog_id, status, source, updated_at) | Participant, Tracker |
| **mws.cog_list.update** | Mettre à jour une entrée (statut, source, updated_at) dans la liste locale | cog_id, status, source | ok / erreur | Participant, Tracker |
| **mws.cog_list.merge** | Fusionner une liste reçue avec la liste locale selon une règle fournie (ex. garder le statut le plus restrictif) | liste reçue, règle de fusion (gouvernée) | liste fusionnée / delta appliqué | Participant, Tracker |
| **mws.cog_list.filter** | Filtrer la liste selon un critère (ex. exclure Rejected, exclure Distrusted) | liste, critère (gouverné) | liste filtrée | Tracker (pour discovery.response) |

**Règle :** la **règle de fusion** et le **critère de filtrage** viennent des Cores (Border Guard, WorrySentinel) ; l'Outil **applique** sans décider de la politique.

### 2.5 Outils d'adresse et de port

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **mws.port.check** | Vérifier si un port est dans la liste normative des ports exclus MWS | port (integer) | true (exclus) / false (utilisable) | Participant, Tracker |
| **mws.address.tracker_default** | Résoudre l'adresse complète d'un Tracker à partir d'un host (port officiel 21000) | host | adresse (host, 21000) | Participant |

**Règle :** ces Outils sont **déterministes** et **sans état métier** ; la liste des ports exclus est **versionnée** avec la norme (voir Normes et Standards, section 2.7).

### 2.6 Outils relay (Webway Relay)

Les **Outils relay** permettent à un COG d'utiliser le **relay Miyukini Webway** (tunnel étendu multi-tenant) pour être joignable derrière NAT ou pour joindre un autre COG via le relay. Ils s'appuient sur le [protocole relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) (REGISTER, CONNECT, HEARTBEAT, etc.).

| Outil | Action | Entrées (orientation) | Sorties (orientation) | Utilisé par |
|-------|--------|------------------------|------------------------|-------------|
| **relay.register** | Enregistrer un tunnel vers le relay (connexion persistante associée au cog_id) | relay_adresse (host, port), token, cog_id, options (TLS, timeouts) | ok (tunnel enregistré) / REGISTER_ERR (code, message) | Participant (COG derrière NAT) |
| **relay.connect** | Établir une connexion vers un cog_id cible via le relay (côté appelant) | relay_adresse, cog_id_cible, options (TLS) | ok (connexion logique établie, prêt pour DATA) / CONNECT_ERR | Participant (COG joignant un autre COG via relay) |
| **relay.heartbeat** | Envoyer un HEARTBEAT sur le tunnel pour maintenir l'enregistrement et détecter les déconnexions | tunnel (référence au tunnel enregistré) | ok (HEARTBEAT_ACK reçu) / timeout / erreur | Participant (COG ayant enregistré un tunnel) |

**Règle :** les Outils relay **ne décident pas** si le COG doit s'enregistrer ni vers quel relay ; ils **exécutent** l'enregistrement, la connexion ou le heartbeat. La décision d'utiliser le relay et le choix de l'adresse relay relèvent des Cores (Border Guard, StrongFather). Le transport vers le relay utilise TLS (voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md)).

### 2.7 Synthèse des Outils MWS

| Domaine | Outils |
|---------|--------|
| **Déclaration** | mws.declaration.build, mws.declaration.validate, mws.declaration.sign, mws.declaration.verify |
| **Transport** | mws.transport.send, mws.transport.receive |
| **Découverte** | mws.discovery.request.build, mws.discovery.request.send, mws.discovery.response.build, mws.discovery.response.send |
| **Liste COG** | mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge, mws.cog_list.filter |
| **Adresse / port** | mws.port.check, mws.address.tracker_default |
| **Relay** | relay.register, relay.connect, relay.heartbeat |

### 2.8 Contrats formels des Outils MWS

Les contrats ci-dessous fixent les **signatures**, **préconditions** et **postconditions** pour chaque Outil MWS et relay. Ils sont déclarés par Master Butler et respectés par les implémentations (Kits, Opérateurs).

**Convention :** *préconditions* = conditions requises avant l'appel ; *postconditions* = garanties en cas de succès ; *invariants* = propriétés préservées. Les décisions métier (accepter, rejeter, filtrer) restent hors du contrat — elles sont fournies par les Cores.

#### 2.8.1 Outils de déclaration

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **mws.declaration.build** | `(type, cog_id, payload, version_norme) → message` | type, cog_id, version_norme fournis ; payload conforme au type | message contient tous les champs obligatoires du format MWS pour ce type ; message prêt pour signature |
| **mws.declaration.validate** | `(message) → ok \| erreurs` | message non vide, structure parsable | si ok : message conforme (champs, types, ports non exclus) ; sinon : liste d'erreurs de validation non vide |
| **mws.declaration.sign** | `(message, clé/secret) → message_signé` | message valide (build ou validate) ; clé/secret gouverné disponible | message_signé inclut bloc d'intégrité vérifiable ; origine authentifiable |
| **mws.declaration.verify** | `(message) → ok \| échec` | message contient bloc d'intégrité | si ok : signature valide, intégrité préservée ; sinon : échec (signature invalide ou absente) |

#### 2.8.2 Outils de transport

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **mws.transport.send** | `(message, adresse) → succès \| erreur` | message sérialisable ; adresse (host, port) valide | message transmis sur le canal vers adresse ; pas de modification du contenu |
| **mws.transport.receive** | `(endpoint) → message \| timeout \| erreur` | endpoint (host, port) en écoute, liaison réussie | si message : message reçu complet ; timeout si aucune donnée dans la fenêtre configurée |

#### 2.8.3 Outils de découverte

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **mws.discovery.request.build** | `(requester_cog_id, query, version) → requête` | requester_cog_id, version fournis | requête conforme au format MWS discovery request ; prête pour envoi |
| **mws.discovery.request.send** | `(requête, adresse(s)) → succès \| erreur` | requête construite ; au moins une adresse Tracker | requête envoyée vers chaque adresse (transport.send) |
| **mws.discovery.response.build** | `(entrées_filtrées, version) → réponse` | entrées déjà filtrées selon politique (fournie par Cores) ; version fournie | réponse conforme au format MWS discovery response ; contient uniquement les entrées fournies |
| **mws.discovery.response.send** | `(réponse, adresse_demandeur) → succès \| erreur` | réponse construite ; adresse demandeur valide | réponse envoyée vers adresse_demandeur |

#### 2.8.4 Outils de liste COG

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **mws.cog_list.get** | `(cog_id?) → entrée(s)` | liste locale accessible | si cog_id : entrée correspondante (cog_id, status, source, updated_at) ou vide ; si omis : liste complète |
| **mws.cog_list.update** | `(cog_id, status, source) → ok \| erreur` | cog_id fourni ; status dans l'ensemble normatif (Trusted, Neutral, etc.) | entrée mise à jour ; updated_at rafraîchi |
| **mws.cog_list.merge** | `(liste_reçue, règle_fusion) → liste_fusionnée \| delta` | liste_reçue et règle_fusion fournis (règle gouvernée) | liste fusionnée conforme à la règle ; pas de suppression de statut sans politique |
| **mws.cog_list.filter** | `(liste, critère) → liste_filtrée` | liste et critère fournis (critère gouverné) | liste_filtrée ⊆ liste ; tous les éléments satisfont le critère |

#### 2.8.5 Outils d'adresse et de port

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **mws.port.check** | `(port) → true \| false` | port entier dans la plage valide | true si port dans la liste normative des ports exclus MWS ; false sinon |
| **mws.address.tracker_default** | `(host) → (host, 21000)` | host non vide | adresse (host, 21000) ; 21000 est le port officiel Tracker MWS |

#### 2.8.6 Outils relay

| Outil | Signature (orientation) | Préconditions | Postconditions (succès) |
|-------|-------------------------|---------------|-------------------------|
| **relay.register** | `(relay_adresse, token, cog_id, options?) → ok \| REGISTER_ERR` | Connexion TCP+TLS vers relay possible ; token et cog_id fournis | Tunnel enregistré côté relay ; connexion persistante associée à cog_id ; REGISTER_OK reçu |
| **relay.connect** | `(relay_adresse, cog_id_cible, options?) → ok \| CONNECT_ERR` | Connexion TCP+TLS vers relay possible ; cog_id_cible fourni | Connexion logique vers cog_id_cible établie ; CONNECT_OK reçu ; prêt pour échange DATA |
| **relay.heartbeat** | `(tunnel) → ok \| timeout \| erreur` | tunnel référence une connexion déjà enregistrée (relay.register) | HEARTBEAT envoyé ; HEARTBEAT_ACK reçu dans le délai configuré ; tunnel maintenu actif |

**Invariant commun aux Outils :** aucun Outil ne prend de décision d'autorisation, de rejet ou de politique ; il exécute une capacité et retourne un résultat. Les choix (quand annoncer, accepter, filtrer) sont fournis par les Cores via BondingBrother aux Opérateurs.

Les noms et signatures détaillés (types exacts, codes d'erreur) sont fixés dans les contrats d'implémentation (Master Butler, contrats Outils MWS) et dans le [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) pour les outils relay.

---

## 3. Kits d'Outils MWS (orientation)

Les **Kits d'Outils MWS** sont des **compositions officielles** d'Outils MWS, validées et déclarées par l'environnement (Master Butler), optimisées pour l'efficience et la cohérence. Ils n'ajoutent **aucune capacité nouvelle** ; ils **orchestrent** les Outils.

### 3.1 Kit Participant Webway (MWS Participant Toolkit)

**Usage :** fournir à l'Opérateur Participant Webway l'ensemble des Outils nécessaires pour **participer** au maillage (annoncer, découvrir, maintenir la liste de statuts) et, le cas échéant, utiliser le **relay Webway** pour être joignable derrière NAT ou pour joindre un autre COG via le relay.

**Composition (orientation) :**
- mws.declaration.build, mws.declaration.sign, mws.declaration.validate, mws.declaration.verify
- mws.transport.send
- mws.discovery.request.build, mws.discovery.request.send
- mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge
- mws.port.check, mws.address.tracker_default
- **relay.register**, **relay.heartbeat** (quand le COG s’annonce via relay ; tunnel persistant)
- **relay.connect** (quand le Participant initie une connexion vers un COG joignable uniquement via relay)

**Règle :** le Kit Participant **ne décide pas** quand annoncer ni à quels Trackers ou relay envoyer ; il fournit les capacités. Les **décisions** sont fournies par les Cores via BondingBrother à l'Opérateur Participant Webway.

### 3.2 Kit Tracker Webway (MWS Tracker Toolkit)

**Usage :** fournir à l'Opérateur Tracker Webway l'ensemble des Outils nécessaires pour **tenir le rôle Tracker** (recevoir, valider, enregistrer, répondre aux requêtes de découverte, maintenir et échanger les listes de statuts, appliquer les mécanismes passifs et actifs).

**Composition (orientation) :**
- mws.declaration.validate, mws.declaration.verify
- mws.transport.receive, mws.transport.send
- mws.discovery.response.build, mws.discovery.response.send
- mws.cog_list.get, mws.cog_list.update, mws.cog_list.merge, mws.cog_list.filter
- mws.port.check, mws.address.tracker_default (pour communication Tracker-à-Tracker si besoin)

**Règle :** le Kit Tracker **ne décide pas** d'accepter ou rejeter une déclaration ; il fournit les capacités de validation, vérification, filtrage. Les **décisions** (politique de rejet, statuts) viennent des Cores (Border Guard, WorrySentinel) et sont appliquées par l'Opérateur Tracker Webway.

**Mécanismes passifs et actifs :** les Outils de liste (merge, filter) et de transport (receive, send) sont utilisés par l'Opérateur Tracker pour appliquer les **systèmes passifs** (observer, signaler, alimenter les listes) et **actifs** (filtrer, rejeter) ; les contrats détaillés des systèmes passifs/actifs sont définis dans [MiyuWebwayTracker - Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [MiyuWebwayTracker - Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).

### 3.3 Kit Relay Webway (MWS Relay Toolkit)

**Usage :** fournir l'ensemble des Outils nécessaires pour **utiliser le relay Miyukini Webway** : enregistrement d'un tunnel (COG derrière NAT), maintien du tunnel par heartbeat, et connexion vers un COG cible via le relay (côté appelant). Ce Kit est typiquement **composé** dans le Kit Participant Webway lorsque le COG participe via relay ; il peut aussi être référencé séparément pour les implémentations qui isolent la couche relay.

**Composition (orientation) :**
- relay.register
- relay.connect
- relay.heartbeat

**Règle :** le Kit Relay Webway **ne décide pas** si le COG doit s'enregistrer ni vers quel relay ; il fournit les capacités. Les **décisions** (utilisation du relay, adresse relay, token) viennent des Cores (Border Guard, StrongFather). Voir [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) et [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md).

---

## 4. Opérateurs MWS (orientation)

Les **Opérateurs MWS** sont des **Opérateurs de Service** (Strate 7) qui exécutent les rôles **Participant Webway** et **Tracker Webway** au sein d'un environnement Miyukini. Ils sont gouvernés par les Cores et utilisent les Outils et Kits d'Outils MWS.

### 4.1 Opérateur Participant Webway (Webway Participant Operator)

**Nom conceptuel (orientation) :** *Opérateur Participant Webway* (ou *MiyuWebwayParticipant* si nommage produit).

**Rôle :**
- **Se déclarer** : annoncer la présence du COG (identité, adresse du Bridge) auprès d'un ou plusieurs Trackers selon les décisions fournies par les Cores.
- **Annoncer services et adresses** : publier les services exposés et les adresses (IP/ports) conformes à la norme MWS et à la politique (ports exclus, format).
- **Déclarer une session hébergée** : lorsqu'il agit comme COG Hébergeur, annoncer « j'héberge une session de tel service et j'attends des connexions » sur le maillage.
- **Découvrir** : envoyer des requêtes de découverte aux Trackers (selon autorisation et politique), traiter les réponses.
- **Participer à la sécurité du maillage** : maintenir la liste locale de COGs avec statuts, échanger des mises à jour avec les Trackers ou d'autres COGs selon le protocole et la politique.

**Capacités utilisées :**
- **Kit Participant Webway** (voir section 3.1).
- Outils MWS : build, sign, validate, verify, transport.send, discovery.request.build/send, cog_list.get/update/merge, port.check, address.tracker_default.

**Gouvernance :**
- **StrongFather** : autorise ou non la participation au Webway (Mandat, politique).
- **Border Guard** : règles de qui peut s'annoncer, quels Trackers utiliser, quels ports/adresses exposer.
- **WorrySentinel** : peut bloquer ou dégrader la participation si l'environnement est en état dégradé (T2, T3, T4).
- **BondingBrother** : reçoit les intentions (annoncer, découvrir, mettre à jour la liste) et les traduit en appels à l'Opérateur Participant Webway ; l'Opérateur invoque les Outils.

**Règle :** l'Opérateur Participant Webway **n'a pas d'autorité** sur le contenu des annonces ni sur le choix des Trackers ; il **exécute** les décisions des Cores. Il ne transmet **aucune donnée métier** ni gouvernance via le Webway.

### 4.2 Opérateur Tracker Webway (Webway Tracker Operator)

**Nom conceptuel (orientation) :** *Opérateur Tracker Webway* (ou *MiyuWebwayTracker* si nommage produit).

**Rôle :**
- **Point de rendez-vous** : exposer l'endpoint sur le **port 21000** (ou adresse configurée), recevoir les annonces (présence, services, sessions hébergées), les enregistrer et les rendre découvrables.
- **Répondre aux requêtes de découverte** : traiter les discovery_request, construire et envoyer les discovery_response en appliquant les listes de statuts (ex. exclure Rejected, filtrer selon politique).
- **Protéger le réseau** : appliquer les **systèmes passifs** (observer, enregistrer, signaler, alimenter les listes de statuts) et **actifs** (refuser les annonces non conformes ou provenant de COGs Rejected/Distrusted, filtrer les réponses).
- **Maintenir et échanger les listes de COGs avec statuts** : mettre à jour la liste locale, fusionner les listes reçues selon la politique, envoyer des mises à jour aux autres Trackers ou COGs selon le protocole.
- **Conformité** : exiger la conformité à la norme de déclaration sécurisée (format, intégrité, ports non exclus) ; rejeter ou ignorer les annonces non conformes.

**Capacités utilisées :**
- **Kit Tracker Webway** (voir section 3.2).
- Outils MWS : validate, verify, transport.receive/send, discovery.response.build/send, cog_list.get/update/merge/filter, port.check.

**Gouvernance :**
- **StrongFather** : autorise ou non le rôle Tracker pour ce COG (Mandat, politique).
- **Border Guard** : règles d'acceptation des annonces, politique de filtrage, liste des ports exclus, politique de propagation des statuts.
- **WorrySentinel** : signaux de confiance, politique de rejet (Distrusted, Rejected), dégradation en cas d'environnement dégradé.
- **BondingBrother** : reçoit les intentions (accepter, rejeter, mettre à jour statut, répondre à une requête) et les traduit en appels à l'Opérateur Tracker Webway ; l'Opérateur invoque les Outils.

**Règle :** l'Opérateur Tracker Webway **ne gouverne pas** les accès (Passeport, Permis de circulation, Visa de Connexion) ; il **protège le maillage** en appliquant les politiques des Cores. Il ne transmet **aucune donnée métier** ni gouvernance ; il ne fait qu'exposer la **présence** et la **découverte**.

### 4.3 Synthèse des Opérateurs MWS

| Opérateur | Rôle principal | Kit d'Outils | Port / exposition |
|-----------|----------------|--------------|-------------------|
| **Participant Webway** | Annoncer, découvrir, maintenir liste de statuts ; optionnellement relay (tunnel, heartbeat, connect) | Kit Participant Webway (inclut Kit Relay Webway si relay utilisé) | Pas d'écoute obligatoire (client vers Trackers / relay) |
| **Tracker Webway** | Point de rendez-vous, découverte, protection (passif/actif) | Kit Tracker Webway | **21000** (officiel) |

---

## 5. Positionnement dans la pyramide Miyukini

```
        Opérateurs (Strate 7)
        Participant Webway / Tracker Webway
               ↓
        Kits d'Outils MWS (Strate 6)
        Kit Participant Webway / Kit Tracker Webway / Kit Relay Webway
               ↓
        Outils MWS (Strate 6)
        mws.declaration.*, mws.transport.*, mws.discovery.*, mws.cog_list.*, mws.port.*, mws.address.*, relay.*
               ↓
        BondingBrother (Strate 5) — intentions → appels Opérateurs
               ↓
        Cores (Strate 4) — Master Butler (capacités), Border Guard (règles), WorrySentinel (confiance), StrongFather (autorisation)
               ↓
        Kernel (Strate 0–3)
```

Les **Outils MWS** et les **Opérateurs MWS** sont **déclarés** par Master Butler et **autorisés** par StrongFather ; les **règles** (ports, statuts, rejet) sont définies par Border Guard et WorrySentinel.

---

## 6. Évolutions futures

- [x] Formaliser les **contrats d'Outils MWS** (signatures, préconditions, postconditions) et les faire déclarer par Master Butler — voir section 2.8 ; ajout des **Outils relay** (relay.register, relay.connect, relay.heartbeat) et **Kit Relay Webway**.
- [ ] Formaliser les **contrats des Opérateurs** Participant Webway et Tracker Webway (frontières, intégration BondingBrother, Border Guard, WorrySentinel).
- [x] Définir les **contrats des systèmes passifs et actifs** des Trackers — [Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md).
- [ ] Spécifier le **nommage produit** des Opérateurs et Kits (ex. MiyuWebwayParticipant, MiyuWebwayTracker) et les enregistrer dans le registre d'Opérateurs.

---

## Références croisées

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) — document principal
- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) — normes, formats, protocole, ports
- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) — architecture du relay, intégration MWS
- [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) — protocole relay (REGISTER, CONNECT, HEARTBEAT, etc.)
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) — déploiement relay (VM, TLS, systemd, tests)
- [MiyuWebwayTracker - Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) — contrats systèmes passifs
- [MiyuWebwayTracker - Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) — contrats systèmes actifs
- [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Outil, Kit d'Outils, Opérateur, Master Butler, Border Guard, WorrySentinel, COG Tracker, MWS, Relay Webway)

---

*Document créé le 30/01/2026*  
*Classification : Reference conceptuelle — Annexe MWS (Outils et Opérateurs)*
