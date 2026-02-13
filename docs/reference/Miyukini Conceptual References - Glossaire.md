# Miyukini Conceptual References — Glossaire

## Contexte

Ce document constitue le **dictionnaire officiel** de l'écosystème Miyukini. Il regroupe toutes les définitions canoniques, la terminologie officielle, et les concepts fondamentaux.

**Ce glossaire est la source de vérité terminologique.**

## Portée / Scope

- **Applicable à :** Toute documentation, communication, développement
- **Audience :** Tous (architectes, développeurs, marketing, IA)
- **Statut :** Document de référence normatif — GLOSSAIRE OFFICIEL

---

## Nomenclature des composants (préfixes)

Les préfixes suivants identifient le **type de composant** conçu par Miyukini ou appartenant à la famille officielle Jay :

| Préfixe / pattern | Signification |
|-------------------|---------------|
| **MiyuXxx** | Nom générique **Toolkit** conçu par Miyukini |
| **MiyukiniOpsXxx** | Nom générique **Opérateurs** conçu par Miyukini |
| **MiyukiniXxx** | Nom générique **Service** conçu par Miyukini |
| **JayXxx** | **Service Jay** — Services officiels de la famille « Jay » |

**Exemples :** MiyuClock (Toolkit), MiyukiniOpsAdmin (Opérateur), MiyukiniSales (Service), JayRDV, JayFestival, JayXpose (Services Jay).

**Voir aussi :** Outil, Kit d'Outils, Opérateur, Service, [Interpolarité des services Jay](./Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

---

## A

### Amis (COGs) (COG Friends)

**Relation explicite entre deux COGs** permettant une connexion **plus rapide** avec des **protocoles de controle allegees** et une **periodicite de re-verification plus longue**. Les demandes d'amis et leur confirmation sont **humaines** (initiees et acceptees par les utilisateurs). Les COGs peuvent exposer les **noms ou pseudos** de leurs utilisateurs pour la reconnaissance. La relation amis est une facilitation contractuelle, pas un contournement des Cores.

**Voir aussi :** Lobby (Webway), Permis de circulation, [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 9.4

---

### ACTIF (ACTIVE) — état de vie

État d'un élément en usage normal. L'élément est stable, documenté, supporté, et utilisable par tous les consommateurs autorisés. Les changements sont soumis aux règles de compatibilité.

**Voir aussi :** BROUILLON, DÉPRÉCIÉ, RETIRÉ, Ever Buddy

---

### Attestation d'environnement (Environment Attestation)

**Hash cryptographique signe** genere par les Cores d'un COG (principalement WorrySentinel) apres une **revue interne de l'environnement**. L'attestation certifie que l'environnement installe du COG (binaires des Cores, Services, configuration) correspond exactement a ce qui est declare dans l'empreinte de version.

**Processus :**

1. WorrySentinel verifie les checksums des binaires installes.
2. Border Guard verifie la coherence de l'identite et des certificats.
3. KindMother verifie l'integrite des donnees persistantes.
4. StrongFather valide la coherence des politiques avec la version des Cores.
5. L'attestation signee est transmise au relay lors du REGISTER.

**But :** empecher les COGs malveillants deguises (qui declarent une identite legitime tout en executant un environnement modifie) de rejoindre le maillage.

**Voir aussi :** Presentation d'identite, Verification de conformite, Relay Webway

---

## B

### BondingBrother

**Core de médiation** (Strate 5). Interface fraternelle qui traduit les intentions des Opérateurs en demandes pour les Cores, et traduit les réponses en résultats.

**Rôle :** Médiation uniquement, jamais d'autorité.

**Question fondamentale :** *"Comment traduire cette intention pour les autorités ?"*

**Voir aussi :** Cores, Opérateur

---

### Border Guard

**Core de frontières** (Strate 4). Définit les frontières du système et les niveaux de confiance.

**Rôle :** Définition conceptuelle des frontières, pas d'application directe.

**Question fondamentale :** *"Où sont les frontières du système, et quelles règles gouvernent leur franchissement ?"*

**Voir aussi :** Cores, Migration

---

### Bloc de code (verification) (Code Block Verification)

**Mecanisme de verification d'authenticite des Services** lors du controle de conformite. Chaque Service envoie un paquet chiffre contenant un **bloc de code** (au sens du MSCM/MIP) choisi **aleatoirement** parmi les blocs de code du Service. Le relay tente de dechiffrer le bloc en utilisant les references de la version connue (heritees d'Origin). Si le dechiffrement est correct, le Service est **authentique** et execute un code **non corrompu** (au moins sur le bloc verifie). En cas de doute, la verification peut etre etendue a **tout le code** (securite renforcee).

**Voir aussi :** Verification de conformite (Phase B), MSCM, MIP, Origin

---

### Bridge inter-COG

**Canal diplomatique** entre COG, extension de BondingBrother pour les communications inter-environnements.

**Rôle :**
- Transport des identités, intentions et autorisations
- **Aucun pouvoir décisionnel**
- **Aucun état métier**

**Règle fondamentale :**

> **Le bridge ne fait jamais confiance, il transporte.**

**Voir aussi :** BondingBrother, COG Hébergeur, COG Origine, Visite gouvernée inter-COG

---

## C

### Catalogue web (Tracker) (Web Catalog)

**Service web des trackers** (port 80) qui presente le **catalogue des services WEB publics** des COGs connectes au reseau, a la maniere d'un **moteur de recherche** ; il gere aussi les **adresses URL**. Seuls les COGs ayant une **surface web active et publique** (sites, SaaS, portails) y figurent. Les **Lobbys des autres services COG** (jeu, APIs, etc.) **ne sont pas visibles** depuis ce portail — le catalogue de Lobbys est visible **depuis les services COG** concernes. Les COGs n'ont pas besoin de nom de domaine ni d'IP fixe : le tracker agit comme facilitateur et tunnel (type No-IP). Le catalogue **redirige** vers les COGs ; il n'a aucune fonction de controle sur les connexions web. Mise a jour et diffusion automatiques ; catalogue **global** accessible depuis n'importe quel tracker.

**Voir aussi :** COG Tracker, Surface de connexion, Serveur web embarque (COG), [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 9.1

---

### Capacité (Capability)

Pouvoir technique qu'un composant possède. C'est ce qu'un module, un adaptateur, ou un Opérateur peut faire techniquement, indépendamment des permissions.

**Caractéristiques :**

- Intrinsèque au composant
- Technique (décrit un pouvoir fonctionnel)
- Déclarative (déclarée par le composant)
- Identifiable (identifiant unique et stable)

**Voir aussi :** Permission, Outil, Master Butler

---

### Caring Nanny

**Core d'observation d'état** (Strate 4). Observateur d'état du système qui détecte, classe et propage les états.

**Rôle :** Observer et rapporter l'état du système, sans jamais modifier, décider, ou exécuter.

**Question fondamentale :** *"Dans quel état se trouve le système à un instant donné ?"*

**Responsabilité Outils :** Cohérence d'état — bloque les Outils si l'environnement est dégradé.

**Voir aussi :** Cores, États de confiance

---

### Compatibilite de version COG (COG Version Compatibility)

**Capacite de deux COGs a interagir** sur le maillage MWS, determinee par leur empreinte de version respective. La compatibilite repose sur deux niveaux :

1. **Cores (immuables)** : deux COGs sont compatibles si et seulement si leur `core_version.MAJOR` est identique. Les Cores definissent le socle de gouvernance et de securite ; une difference de MAJOR implique une rupture de compatibilite.
2. **Services (patchables)** : a `core_version.MAJOR` identique, les Services peuvent avoir des versions differentes (MINOR, PATCH) et rester compatibles. Un patch de Service est transparent pour les interactions inter-COG.

**Verification** : la compatibilite est verifiee par le relay (lors du REGISTER et du CONNECT) et par le Tracker (lors des annonces et des reponses de decouverte).

**Voir aussi :** Empreinte de version COG, Version des Cores, Relay Webway, COG Tracker (Webway)

---

### Cle de conformite des Cores (Core Conformity Key)

**Cle secrete cachee dans le code des Cores**, connue d'Origin et de tous les relays. Lors de la verification d'un COG, les Cores transmettent cette cle au relay. La **concordance** entre la cle transmise et la cle attendue pour la `core_version` declaree prouve que les Cores sont **authentiques** et non modifies. La cle est specifique a chaque version des Cores.

**Voir aussi :** Verification de conformite (Phase A), Origin, Passeport COG

---

### Confinement reseau (Network Containment)

**Protocole d'urgence** declenche par les relays lorsque plusieurs COGs sont rejetes dans un tres court laps de temps (attaque ou corruption detectee). Le confinement comprend :

1. Alerte envoyee aux trackers et relays.
2. Controle renforce obligatoire de tous les COGs connectes.
3. Fermeture possible de toutes les connexions inter-COG par les trackers.
4. Origin et relays restent accessibles en **lecture seule** avec verification.
5. Reconstruction progressive par les COGs valides re-verifies.

**Voir aussi :** Quarantaine, Origin, Mode d'urgence reseau

---

### Conformite environnement (Environment Conformity)

**Etat d'un COG dont l'environnement installe** (Cores, Services, configuration) **est conforme aux criteres d'Origin** : tous les services sont presents dans le Registre, les checksums correspondent aux versions declarees, l'attestation d'environnement est valide et recente.

La conformite est verifiee par le **relay** (verification lourde : attestation, checksums, Registre) et non par le Tracker (qui ne verifie que l'identite).

**Voir aussi :** Attestation d'environnement, Verification de conformite, Relay Origin, Presentation d'identite

---

### Collaboration Mandatée (Mandated Collaboration)

**Coopération entre Opérateurs sous Mandat de Permission.** Les Opérateurs ne collaborent jamais librement — toute collaboration est encadrée par un mandat émis par StrongFather.

**Règles :**

- Pas de communication directe entre Opérateurs
- Passage obligatoire par BondingBrother
- Respect strict du Mandat de Permission

**Voir aussi :** Mandat de Permission, Équipe d'Opérateurs, BondingBrother

---

### Contrat d'Équipe (Team Contract)

**Règles statiques de collaboration** entre Opérateurs d'une même Équipe d'Opérateurs.

**Contenu du contrat :**

- Opérateurs membres
- Flux autorisés (qui parle à qui)
- Direction des flux
- Types d'échanges
- Types de données échangeables
- Conditions préalables
- Niveau de validation requis

**Caractéristiques :**

- Statique (défini à la conception)
- Validé par StrongFather
- Modification = processus formel

**Règle clé :** Le contrat est validé UNE FOIS, pas à chaque appel.

**Voir aussi :** Équipe d'Opérateurs, Mandat de Permission

---

### COG (Core-Orchestrated Governance Environment)

**Définition officielle de Miyukini.**

> **Miyukini est un COG — un environnement de gouvernance orchestré par des cores.**


| Lettre | Signification                                                 |
| ------ | ------------------------------------------------------------- |
| **C**  | Core — Les cores sont les unités fondamentales de gouvernance |
| **O**  | Orchestrated — Actif, coordonné (pas "operating")             |
| **G**  | Governance Environment — Environnement de gouvernance actif   |


**Ce que COG implique :**

- Orchestrated > Operating — Miyukini n'est pas un OS
- Governance > Governed — Actif, institutionnel
- Environment — Écosystème complet

**Voir aussi :** Environnement, Cores

---

### COG Hébergeur (Host COG)

**COG souverain qui accueille un Utilisateur Visiteur** provenant d'un autre environnement.

**Rôle :**
- Souverain exécutif de la session
- Unique source de vérité de l'état
- Autorité de sécurité et d'arbitrage

**Responsabilités :**
- Vérifier le visiteur
- Accorder ou refuser l'accès
- Encadrer strictement l'exécution
- Surveiller la session (WorrySentinel)
- Révoquer à tout moment

**Règle fondamentale :**

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

**Voir aussi :** COG Origine, Visa de Connexion, Visite gouvernée inter-COG

---

### COG Origine (Home COG)

**COG d'appartenance d'un Utilisateur Visiteur**, qui atteste de son identité.

**Rôle :**
- Autorité d'identité de l'utilisateur
- Garant de la conformité de l'environnement d'origine
- Émetteur du Passeport Utilisateur

**Responsabilités :**
- Vérifier l'intégrité locale
- Attester la version de la strate Core
- Fournir une identité vérifiable
- **Ne participe PAS à l'exécution distante**

**Voir aussi :** COG Hébergeur, Passeport Utilisateur, Visite gouvernée inter-COG

---

### COG participant (Webway Participant)

**COG qui choisit de participer au maillage MWS** : il se déclare auprès d'un ou plusieurs COGs Tracker, expose ses informations de présence (identité COG, adresse du Bridge ou point de contact) et peut consulter la présence d'autres COGs. Il peut annoncer une **adresse relay** (relay_host:port + token d'authentification relay) s'il est derrière NAT pour être joignable via un Relay Webway.

**Voir aussi :** Miyukini Webway System, COG Tracker, Relay Webway, Token d'authentification relay

---

### COG de référence (Reference COG / Official COG)

**COG désigné comme détenteur canonique** des données sensibles d'un domaine donné. Il héberge l'Instance Mère KindMother (ou l'équivalent « serveur ») pour ce domaine.

**Rôle :**
- Détenteur canonique des données à résidence centralisée
- Source de vérité pour les données sensibles du domaine
- Accessible par les acteurs autorisés (Visite gouvernée, sync) — les terminaux ou autres COG n'en sont pas propriétaires, ils accèdent sans en être la seule copie

**Règle fondamentale :**

> **Les données sensibles à résidence centralisée ne doivent pas avoir pour seule copie un terminal ou un COG tiers. Leur copie canonique réside sur le COG de référence.**

**Voir aussi :** Politique de résidence des données sensibles, KindMother (Instance Mère), COG Hébergeur, WorrySentinel, Niveaux de sécurité

---

### COG Tracker (Webway Tracker)

**Douanier du reseau Miyukini Webway.** Le Tracker assure les connexions entre COGs et en assure la securite par des controles d'identite et **contrôle tracker** (verification du **Permis de circulation**), comme un douanier a une frontiere. Seuls les **trackers connus d'Origin** (trackers officiels/sûrs) sont autorises : un COG ne peut et ne doit se connecter qu'aux trackers dont les adresses lui sont remises avec son Permis de circulation par le relay.

**Port officiel :** les COGs Tracker MWS exposent leur endpoint sur le **port 21000**.

**Role :**
- **Contrôle d'identite et contrôle tracker** : verifier que le COG possede un Permis de circulation valide delivre par un relay ou Origin (accord relay) avant de le laisser se connecter au maillage.
- **Pools par version des Cores** : diriger des pools separes par `core_version.MAJOR` pour ne jamais connecter des COGs avec des versions differentes.
- **Whitelists / Blacklists / Quarantaines** : gerer les listes d'autorisation, d'exclusion et de quarantaine.
- **Monitoring et congestion** : journaliser et monitorer l'etat du reseau, detecter les points de congestion. Renforcer la surveillance si un COG accumule beaucoup de connexions (COGs speciaux).
- **Fermeture de connexions** : pouvoir fermer tout ou partie des connexions pour circonscrire une attaque, sur annonce des relays.
- **Decouverte** : point de rendez-vous pour la decouverte (annonces de presence, requetes de decouverte) ; les adresses annoncees peuvent etre une adresse relay.

**Ce qu'un COG Tracker N'EST PAS :**
- Un verificateur de conformite (pas de verification Passeport/Cores/Services, c'est le role des relays)
- Un transporteur de donnees metier
- Un distributeur de mises a jour (redirige vers les relays)

**Voir aussi :** Origin, Relay Webway, Permis de circulation, accord relay, contrôle tracker, Pool de version, Quarantaine, Confinement reseau, [MiyuWebwayTracker - Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md), [MiyuWebwayTracker - Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)

---

### Cores

**Moteurs conceptuels** (Strate 4) qui gouvernent le comportement du système. Chaque core a une autorité exclusive dans son domaine.


| Core          | Domaine                   |
| ------------- | ------------------------- |
| StrongFather  | Décision stratégique      |
| KindMother    | Données et persistance    |
| Caring Nanny  | Observation d'état        |
| Master Butler | Capacités et permissions  |
| Border Guard  | Frontières et confiance   |
| Ever Buddy    | Cycle de vie et évolution |
| WorrySentinel | Gouvernance de sécurité   |
| TAMR          | Intervention humaine      |


**Règle fondamentale :** Les cores décident ou gouvernent, mais n'exécutent jamais.

**Voir aussi :** Chaque core individuellement

---

## D

### Déclaration d'hébergement de session (Webway Host Session Declaration)

**Annonce par un COG Hébergeur au réseau MWS** indiquant qu'il héberge une session d'un service donné et qu'il attend des connexions vers lui (adresse et port).

**Contenu minimal (orientation) :** identifiant du service ou type de session, identifiant du COG Hébergeur, adresse de connexion (IP ou nom de domaine, port). La déclaration **ne donne aucun droit d'accès** ; elle indique où se présenter pour demander un Permis de circulation ou un accord d'hôte.

**Voir aussi :** Miyukini Webway System, COG Hébergeur, Norme de déclaration sécurisée (MWS)

---

### Demande de Visite (Visit Intent)

**Intention d'accès** émise par un Utilisateur Visiteur vers un COG Hébergeur.

**Émise par :** Utilisateur Visiteur

**Contient :**
| Champ | Description |
|-------|-------------|
| `requested_services` | Liste des Services demandés |
| `usage_nature` | Nature de l'usage (lecture, interaction, temps réel, etc.) |
| `security_level` | Niveau de sécurité requis |
| `terminal_context` | Contexte terminal (PC, mobile, web…) |

**Règle fondamentale :**

> **C'est une intention, pas une permission.**

**Voir aussi :** Passeport Utilisateur, Visa de Connexion, Visite gouvernée inter-COG

---

### DÉPRÉCIÉ (DEPRECATED) — état de vie

État d'un élément toujours fonctionnel mais dont l'usage est découragé. Un successeur existe ou est en préparation.

**Caractéristiques :**

- Période de dépréciation définie
- Consommateurs avertis de migrer
- Passage obligatoire avant RETIRÉ

**Voir aussi :** ACTIF, RETIRÉ, Ever Buddy

---

### Divergence silencieuse (Silent Divergence)

**Situation détectable par le Kernel** où un système déclare une version mais présente une empreinte comportementale différente.

**Causes typiques :**
- Build recompilé différemment
- Dépendance modifiée silencieusement
- Compilation non reproductible
- Injection de code ou modification post-build

**Caractéristiques :**
- Signal de maintenance, pas d'erreur
- Détectable sans réseau
- Déterministe et rejouable

**Règle fondamentale :**

> **Le Kernel signale la divergence mais ne la corrige jamais.**

**Voir aussi :** Empreinte comportementale, Maintenance explicable, Kernel Maintenance Observability Contract

---

### BROUILLON (DRAFT) — état de vie

État d'un élément en cours de définition. Non utilisable en production, peut changer librement, aucun engagement de stabilité.

**Voir aussi :** ACTIF, DÉPRÉCIÉ, RETIRÉ, Ever Buddy

---

## E

### Empreinte comportementale (Behavior Fingerprint)

**Signature structurelle** du système chargé, produite par le Kernel.

**Éléments capturés :**

| Élément | Description |
|---------|-------------|
| Ordre de chargement | Séquence d'initialisation des composants |
| Graphe d'appel structurel | Relations entre composants (pas métier) |
| Contrats invoqués | Liste des contrats activés |
| Invariants sollicités | Invariants vérifiés au chargement |

**Caractéristiques :**
- C'est une signature, pas un log
- Aucun contenu métier
- Aucune donnée runtime
- Déterministe et rejouable

**Utilité :**
- Comparer deux versions du système
- Détecter une dérive silencieuse
- Prouver l'équivalence fonctionnelle de builds

**Règle fondamentale :**

> **L'empreinte observe et atteste, mais ne corrige jamais.**

**Voir aussi :** Divergence silencieuse, Maintenance explicable, Kernel Maintenance Observability Contract

---

### Empreinte de version COG (COG Version Fingerprint)

**Ensemble de donnees de version** transmis par un COG lorsqu'il se presente au relay ou au Tracker MWS. L'empreinte identifie le socle technique du COG et permet de verifier la compatibilite avant interaction.

**Composants :**

| Champ | Description |
|-------|-------------|
| `core_version` | Version des Cores (format `MAJOR.MINOR`). Les Cores sont **immuables** a version donnee ; le MAJOR determine la compatibilite stricte. |
| `service_manifest` | Liste des Services actifs avec leurs versions (`service_id` + `MAJOR.MINOR.PATCH`). Les Services sont patchables independamment des Cores. |
| `protocol_version` | Version du protocole relay ou MWS utilise. |
| `build_id` | Identifiant de build (optionnel, tracabilite). |

**Regle fondamentale :**

> **Deux COGs ne peuvent interagir que s'ils partagent la meme `core_version.MAJOR`. Les patchs de Service sont transparents a Cores identiques.**

**Voir aussi :** Version des Cores, Compatibilite de version COG, Enregistrement relay

---

### Enregistrement relay (Webway)

**Action par laquelle un COG enregistre son tunnel** auprès d'un Relay Webway : le COG ouvre une connexion persistante (souvent TLS) vers le relay, s'authentifie avec un token d'authentification relay, déclare son `cog_id` ; le relay associe alors la connexion (tunnel) à ce `cog_id` dans sa table de routage. Les connexions entrantes destinées à ce COG sont routées vers ce tunnel.

**Voir aussi :** Relay Webway, Tunnel (Webway), Token d'authentification relay, Miyukini Webway System

---

### Environnement (COG)

**Entité souveraine, versionnée, isolée et identifiée de manière unique.**


| Propriété                  | Description                |
| -------------------------- | -------------------------- |
| Version complète des cores | Ensemble cohérent et figé  |
| Itération unique           | Numéro de version distinct |
| ID unique                  | Généré par le kernel       |
| Opérateurs assujettis      | Liés à cet environnement   |
| Frontières strictes        | Limites claires            |


**Règle fondamentale :** La strate Cores est immuable. Toute évolution se fait par création d'un nouvel environnement complet.

**Voir aussi :** COG, Souveraineté, LOI-7

---

### États de confiance (T0-T4)

États caractérisant l'intégrité du système, gouvernés par WorrySentinel.


| État   | Nom       | Description                                 |
| ------ | --------- | ------------------------------------------- |
| **T0** | Normal    | Système sain, toutes capacités disponibles  |
| **T1** | Instable  | Anomalie détectée, surveillance accrue      |
| **T2** | Dégradé   | Incohérence persistante, capacités réduites |
| **T3** | Restreint | Suspicion forte, gel des non-essentiels     |
| **T4** | Bloqué    | Intégrité rompue, uniquement diagnostics    |


**Voir aussi :** WorrySentinel, Niveaux de sécurité

---

### Équipe d'Opérateurs (Operator Team)

**Collectif gouverné d'Opérateurs** qui collaborent sous règles explicites pour délivrer un Service.

**Définition canonique :**

> **Une Équipe d'Opérateurs est un collectif gouverné d'Opérateurs qui collaborent sous règles explicites pour délivrer un Service.**

**Composition :**

- Plusieurs Opérateurs (minimum 2)
- Hétérogènes en sécurité, responsabilités, exposition
- Liés par un Contrat d'Équipe
- Règles validées par StrongFather

**Ce qu'une Équipe N'EST PAS :**

- ❌ Un nouvel Opérateur
- ❌ Un produit
- ❌ Une hiérarchie libre

**👉 C'est une structure d'orchestration supérieure.**

**Règle clé :** Une Équipe d'Opérateurs ne peut exister opérationnellement que sous un Mandat de Permission valide.

**Voir aussi :** Opérateur, Service, Mandat de Permission, Contrat d'Équipe

---

### Ever Buddy

**Core de cycle de vie** (Strate 4). Gouverne l'évolution des structures, des contrats, et des entités dans le temps.

**Rôle :** Observer ce qui a été, ce qui est, et ce qui sera, sans jamais exécuter de migration.

**Question fondamentale :** *"Comment le système évolue-t-il sans jamais se rompre ?"*

**Responsabilité Tools :** Versions, dépréciation, compatibilité, migration.

**Voir aussi :** États de vie, Tool

---

### Heartbeat (relay Webway)

**Message périodique** envoyé par un COG vers le Relay Webway sur le tunnel établi pour maintenir le tunnel actif et permettre au relay de détecter les déconnexions. En l'absence de heartbeat (selon politique du relay), le tunnel peut être considéré comme inactif et retiré de la table de routage.

**Voir aussi :** Relay Webway, Tunnel (Webway), Enregistrement relay

---

## F

### Façade Publique Gouvernée (Public Exposure Surface)

**Zone tampon d'exposition** permettant aux utilisateurs externes d'interagir avec un COG sans y entrer.

**Caractéristiques :**
- Strictement unidirectionnelle
- Sans identité persistante obligatoire
- Sans accès aux cores
- Sans accès à la logique interne
- Sans état souverain

**Règle fondamentale :**

> **C'est le COG qui sort vers l'utilisateur externe, jamais l'inverse.**

**Voir aussi :** Utilisateur Externe, Mandat Public d'Accès, BorderGuard

---

### Favoris (COG) (Favorites)

**Liste de COGs hôtes** que l'utilisateur du COG client souhaite **retrouver rapidement** dans les listes et Lobbys distribues par le tracker. Les favoris peuvent etre stockes localement (cote client) ou signales au tracker pour un affichage prioritaire. Permet d'accelerer la decouverte du COG hôte desire lors de la consommation de services.

**Voir aussi :** Lobby (Webway), accord d'hôte, [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 9.3

---

## G

### Gel local (Local Freeze)

**Capacité du Kernel** à marquer un composant comme gelé structurellement, sans affecter le reste du système.

**Actions permises :**
- Marquer un composant comme gelé
- Refuser son remplacement ou rechargement
- Laisser le reste du système évoluer

**Utilité :**
- Stabiliser une zone critique pendant une intervention
- Corriger ailleurs sans risque de régression
- Maintenir des SLA forts sur des composants spécifiques

**Gouvernance :**

| Acteur | Rôle |
|--------|------|
| StrongFather | Décide l'autorisation du gel |
| EverBuddy | Valide la compatibilité du gel |
| Kernel | Exécute le gel et l'applique |

**Règle fondamentale :**

> **Le gel est décidé par la gouvernance, exécuté par le Kernel, jamais inversé.**

**Voir aussi :** Kernel Maintenance Observability, StrongFather, Ever Buddy

---

## I

### Opérateur d'Interface (Interface Operator)

**Type d'Opérateur** qui expose les services de façon utilisable.

**Exemples :** UI web, App mobile, Tableau de bord, Panneau d'administration

**Phrase type :** *"Expose les services de façon utilisable."*

**Voir aussi :** Opérateur, Opérateur de Service

---

### Isolation reseau (Webway Network Isolation)

**Mesure de protection appliquee par le Webway** lorsqu'un COG presente un service non repertorie dans son `service_manifest`. Le COG est exclu du maillage MWS actif tout en restant connecte en mode surveillance :

| Etat | Description |
|------|-------------|
| **Exclu** | Pas d'annonces de presence relayees, pas d'inclusion dans les reponses de decouverte, pas de routing de donnees. |
| **Maintenu** | Tunnel relay actif (heartbeats acceptes), notifications et consultations du Registre possibles. |
| **Notifie** | L'utilisateur du COG est informe de la raison de l'isolation et des actions correctives. |
| **Surveille** | L'evenement est journalise au niveau du maillage (Relay Origin + Trackers) ; le COG est reevalue periodiquement. |

**Levee :** automatique lorsque le COG se re-enregistre avec un manifest conforme au Registre de Services.

**Voir aussi :** Service non repertorie, Registre de Services, Relay Origin

---

## K

### Kernel

**Substrat technique neutre** (entre Strate 0 et Strate 3). Fondation technique réutilisable, agnostique, sans logique métier.


| Composant | Rôle                              |
| --------- | --------------------------------- |
| Id        | Génération d'identifiants uniques |
| Logger    | Logging structuré                 |
| Clock     | Horloge locale (trace only)       |
| Config    | Configuration locale              |
| Lifecycle | Gestion du cycle de vie           |


**Invariants :**

- Aucune logique métier
- Aucune dépendance externe critique
- Pas de protocole applicatif

**Voir aussi :** Pyramide, Cores

---

### Kernel Maintenance Observability

**Ensemble de capacités bas niveau du Kernel** pour assister la maintenance du code sans jamais exécuter de correction automatique.

**Capacités incluses :**

| Capacité | Description |
|----------|-------------|
| Empreinte comportementale | Signature structurelle du système |
| Détection de divergence | Même version, comportement différent |
| Carte de complexité | Zones de couplage et fragilité |
| Gel local | Stabilisation par composant |
| Détection d'ambiguïté | Contrats incomplets ou code mort |
| Maintenance explicable | Traçabilité gouvernée des incidents |

**Ce que le Kernel PEUT faire :**
- Observer, attester, comparer, signaler, expliquer

**Ce que le Kernel ne peut JAMAIS faire :**
- Corriger, muter, auto-réparer

**Phrase fondatrice :**

> **Miyukini ne maintient pas le code à la place de l'humain. Il rend le code maintenable sans ambiguïté.**

**Voir aussi :** Empreinte comportementale, Divergence silencieuse, Maintenance explicable, Gel local

---

### KindMother

**Core de données** (Strate 4). Autorité absolue des données et de la persistance.

**Rôle :** Persistance, synchronisation, cohérence des données.

**Question fondamentale :** *"Comment les données sont-elles persistées et synchronisées ?"*

**Voir aussi :** Cores, WriteIntent

---

### Kit d'Outils (Toolkit)

**Composition officielle d'Outils** (Strate 6), validée et déclarée par l'environnement.

**Définition canonique :**

> **Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience, cohérence et performance.**

**Caractéristiques :**

- Agrège des Outils existants
- N'ajoute aucune capacité nouvelle
- Sans logique métier
- Gouverné

**Règle fondamentale :**

> **👉 Un Kit d'Outils orchestre, mais n'ajoute pas de capacité.**

**Voir aussi :** Outil, Master Butler

---

## L

### Local Sovereign ID (LSI)

**Niveau 1 d'identité d'environnement.** Générée par le kernel local, toujours valide localement, garantie localement.

**Cas d'usage :** Environnement isolé, offline permanent.

**Confiance :** Souveraine — l'environnement s'auto-déclare.

**Voir aussi :** Verified ID, Witnessed ID

---

### Liste de COGs avec statuts (Webway COG List)

**Liste maintenue par chaque COG participant au Miyukini Webway System (MWS)** associant à chaque COG connu un **statut** (Trusted, Neutral, Under review, Distrusted, Rejected). Permet d'analyser et, le cas échéant, de rejeter un COG ou une connexion considérée comme malveillante ou non fiable.

**Échange :** les COGs se transfèrent des listes ou des mises à jour de statuts selon le protocole MWS ; chaque COG reste souverain dans l'usage qu'il en fait.

**Voir aussi :** Miyukini Webway System, COG Tracker

---

### Lobby (Webway) (Lobby)

**Entree du catalogue de Lobbys** tenu par les trackers, correspondant a l'exposition d'un ou plusieurs **services** d'un **COG hôte** sur des **ports donnes**. Cree lorsque le COG presente au tracker ses surfaces de connexion et indique qu'il accepte des connexions pour tels services et ports. Ce catalogue **n'est pas affiche sur le portail web des trackers** (reserve aux services WEB publics) : les Lobbys sont **visibles et joignables depuis les services COG** concernes (ex. client jeu, client SaaS). Un Lobby peut etre **public** ou **prive** (mot de passe) ; en prive, 5 echecs d'acces entrainent le ban du COG client, avec de-ban manuel uniquement par l'utilisateur du COG hôte.

**Voir aussi :** Catalogue web (Tracker), Surface de connexion, accord d'hôte, Favoris (COG), [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 9.1–9.2

---

### Limite de connexions (COG classique)

**Plafond de 100 connexions simultanees** (hors ports web 80 et 8080) pour un **COG classique**. Garantit une qualite de suivi des organes de securite ; les COGs ne sont pas des services type torrent. Les connexions sur les ports 80 et 8080 ne sont pas comptees. Les COGs avec **Passeport special** peuvent etre autorises a des plafonds superieurs.

**Voir aussi :** Surface de connexion, Passeport special, [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 8.2

---

### LOI-1 à LOI-8 (Lois d'Autonomie)

**8 lois d'autonomie non négociables** qui régissent l'architecture Miyukini.


| Loi       | Énoncé                                                     |
| --------- | ---------------------------------------------------------- |
| **LOI-1** | Aucune dépendance externe critique à l'exécution           |
| **LOI-2** | Le système accepte l'isolement comme état normal           |
| **LOI-3** | L'état local est souverain                                 |
| **LOI-4** | Pas de temps global requis                                 |
| **LOI-5** | Le coût doit être proportionnel au hardware                |
| **LOI-6** | L'autonomie n'empêche pas la fédération                    |
| **LOI-7** | La strate Cores est immuable — évolution par environnement |
| **LOI-8** | Migration = diplomatie entre environnements                |


**Voir aussi :** Autonomie, Souveraineté

---

## M

### Maintenance explicable (Explainable Maintenance)

**Mode de diagnostic du Kernel** pour fournir une traçabilité gouvernée lors d'incidents, sans exposer de données techniques sensibles.

**Informations fournies :**
- Pourquoi une décision est arrivée jusqu'ici
- Quels contrats ont été traversés
- Où la gouvernance s'est arrêtée

**Ce qui n'est JAMAIS fourni :**
- Stacktrace classique (fuite d'information technique)
- Dump mémoire (fuite de données sensibles)
- Données utilisateur (protection vie privée)

**Caractéristiques :**
- Traçabilité gouvernée, pas technique
- Compréhensible par un humain sans connaissance du code source
- Fonctionne offline

**Règle fondamentale :**

> **Le diagnostic explique le chemin de gouvernance, jamais l'implémentation.**

**Voir aussi :** Kernel Maintenance Observability, Caring Nanny

---

### Master Butler

**Core de capacités** (Strate 4). Registre central des capacités et permissions du système.

**Rôle :** Catalogue des capacités, définition des permissions, découverte.

**Question fondamentale :** *"Qu'est-ce qui est possible dans cet environnement ?"*

**Responsabilité Tools :** Déclare quels Tools existent, lie Capability → Tool, définit les permissions d'accès.

**Ce que Master Butler NE fait PAS :**

- N'implémente pas les Tools
- N'exécute pas les Tools
- Ne décide pas si un Tool doit être appelé

**Voir aussi :** Capability, Permission, Tool

---

### Mandat de Permission (Allow Mandate)

**Autorisation déléguée, temporaire et encadrée** émise par StrongFather, permettant à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.

**Définition canonique :**

> **Un Mandat de Permission est une autorisation déléguée, temporaire et encadrée, émise par StrongFather, qui permet à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.**

**Contenu d'un Mandat :**

- ID unique
- Opérateurs autorisés
- Flux autorisés
- Types de données
- Niveau de sécurité maximum
- Conditions de validité
- Règles de révocation

**Ce qu'un Mandat N'EST PAS :**

- ❌ Un token libre
- ❌ Une session classique
- ❌ Un cache de décision
- ❌ Un droit implicite
- ❌ Une permission globale

**Phrase fondatrice :**

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

**Causes de révocation :**

- Service terminé
- Condition hors cadre
- Violation de règle
- Alerte WorrySentinel
- Utilisateur quitte le flux
- Environnement change

**Voir aussi :** StrongFather, Équipe d'Opérateurs, Contrat d'Équipe

---

### Mandat Public d'Accès (Public Access Mandate)

**Autorisation attachée à un service public** pour encadrer l'accès des utilisateurs externes non certifiés.

**Défini par :** Host COG

**Contient :**
| Champ | Description |
|-------|-------------|
| `public_services` | Services publics accessibles |
| `allowed_methods` | Méthodes autorisées |
| `quotas` | Quotas d'utilisation |
| `rate_limits` | Limitations de fréquence |
| `security_level` | Niveau de sécurité (S1-S3) |
| `expected_behavior` | Comportement attendu |

**Différence avec le Visa de Connexion :**

| Aspect | Visa de Connexion | Mandat Public |
|--------|------------------|---------------|
| Destinataire | Utilisateur Visiteur | Service exposé |
| Identité requise | ✅ Passeport | ❌ Non |
| Accès cores | Indirect | ❌ Jamais |

**Règle fondamentale :**

> **Le mandat est attaché au service, pas à l'utilisateur.**

**Voir aussi :** Utilisateur Externe, Façade Publique Gouvernée, Visa de Connexion

---

### Migration

**Processus d'échange de données entre environnements.** Migration ≠ communication directe.

**Règles :**

- Migration = processus formel
- Migration = contrat explicite
- Migration = frontière contrôlée
- Migration = traduction, pas copie brute

**Acteurs :** Border Guard (règles), BondingBrother (traduction), StrongFather (décision), KindMother (persistance), Ever Buddy (compatibilité).

**Voir aussi :** LOI-8, Environnement

---

### Migration COG (père/fils)

**Mecanisme de passage a des Cores plus recents** : un COG (**COG pere**) prepare sa migration vers un **COG fils** (Cores plus recents). Le COG fils enregistre sa **parentalite** aupres du pere ; le pere archive sa DB par strates ; le fils installe les Services compatibles et effectue la migration DB. Les deux COGs gardent leur **propre Passeport** et sont uniques. Le lien de parentalite **renforce la securite** et la force du Passeport lors des controles ; un COG enfant d'un pere sûr de longue date peut passer plus rapidement les controles douaniers des trackers.

**Voir aussi :** Parentalite COG, Passeport COG, Versioning COG, [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 5.8

---

### Miyukini Webway System (MWS)

**Couche de présence et de découverte** des environnements COG disposant d'un accès réseau. Permet aux COGs de se déclarer, de savoir qui est présent sur le maillage, et de faciliter l'initiation des visites gouvernées (Passeport, Visa) sans transférer de données métier.

**Rôle :**
- Normaliser *qui est là* et *où se présenter* pour demander un Permis de circulation
- Système de sécurité fondé sur l'échange de listes de COGs avec statuts (Webway COG List)
- Les COGs Tracker ont le devoir de protéger le réseau par des mécanismes passifs ([Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md)) et actifs ([Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md))

**Règle fondamentale :**

> **Le Webway normalise la présence et facilite l'échange ; il ne transporte pas la gouvernance ni les données.**

**Développement Outils et Opérateurs :** voir [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) (annexe conceptuelle).

**Voir aussi :** COG Tracker, Liste de COGs avec statuts, Connexion Inter-COG, Bridge inter-COG

---

### MiyukiniAdmin

**Opérateur Souverain** — Console souveraine d'administration (Strate 9).

**Caractéristiques spéciales :**

- Exception à la logique Opérateur standard
- Autorité quasi institutionnelle
- N'est pas utilisable par d'autres Opérateurs
- Agit sous protocole spécial

**Fonctions :** Installation, diagnostic, arbitrage, accès exceptionnel.

**Voir aussi :** Opérateur Souverain, Opérateur

---

### Miyukini Central

**Service Fondamental** — Hub de gestion des Services, point d'entrée pour l'utilisateur du COG (Strate 7).

**Rôle :** Exposer le catalogue des Services (Registre d'Opérateurs), permettre de découvrir, activer et lancer des Services. Point d'entrée unique pour l'utilisateur du COG.

**Question fondamentale :** *"Quels Services sont disponibles, et comment y accéder ?"*

**Caractéristiques :**

- Service Fondamental (fait partie de l'environnement versionné)
- Opérateur d'Interface (Strate 7)
- Point d'entrée COG (gestion, administration, création)
- Aucune autorité propre — relaie vers les Cores

**Règle canonique :**

> **Central = COG. Tous les Services ont comme point d'accès utilisateur Miyukini Central.**

**Voir aussi :** Miyukini Web Portal, Service Fondamental, Opérateur d'Interface

---

### Miyukini Web Portal (Portail)

**Service Fondamental** — Point d'entrée web pour les utilisateurs externes (Strate 7).

**Rôle :** Exposer les Façades Publiques Gouvernées des Services de Type 2 aux utilisateurs externes via le web. Équivalent de Central pour le monde extérieur.

**Question fondamentale :** *"Comment les utilisateurs externes accèdent-ils aux surfaces web du COG ?"*

**Caractéristiques :**

- Service Fondamental (fait partie de l'environnement versionné)
- Opérateur d'Interface (Strate 7)
- Point d'entrée Web (utilisateurs externes sans COG)
- Identification et fichage des connexions entrantes
- Gouvernance via BorderGuard + Mandat Public d'Accès

**Règle canonique :**

> **Portail = Web. Central = COG, Portail = Web.**

**Ce que le Portail N'EST PAS :**

- ❌ Un serveur central unique (chaque COG a son Portail)
- ❌ Un remplacement de Central
- ❌ Une porte ouverte (tout passe par BorderGuard)

**Voir aussi :** Miyukini Central, Service Fondamental, Façade Publique Gouvernée, Mandat Public d'Accès

---

## N

### Norme de déclaration sécurisée (MWS)

**Norme définie** pour les annonces MWS : services exposés, adresses (IP/ports) et sessions hébergées. Elle assure l'**authentification** de l'origine des déclarations (signature COG), l'**intégrité** (sérialisation canonique + MAC/signature), un **format unifié** (schéma commun, champs obligatoires) et la **limitation des abus**. Les COGs Tracker peuvent exiger la conformité pour accepter ou relayer les annonces. Le cadre conceptuel est défini dans [MWS](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) section 3.3 ; les formats détaillés dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) sections 1–2.

**Développement :** voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (document conceptuel annexe).

**Voir aussi :** Miyukini Webway System, Déclaration d'hébergement de session, COG Tracker

---

### Niveaux de sécurité (0-4)

**Niveaux caractérisant le profil de risque**, gouvernés par WorrySentinel.


| Niveau | Nom       | Description                                  |
| ------ | --------- | -------------------------------------------- |
| **0**  | Public    | Données publiques, aucune contrainte stricte |
| **1**  | Standard  | Données standard, contraintes de base        |
| **2**  | Sensitive | Données sensibles, contraintes renforcées    |
| **3**  | Critical  | Données critiques, contraintes strictes      |
| **4**  | Highest   | Sécurité maximale, contraintes maximales     |


**Voir aussi :** WorrySentinel, États de confiance

---

## O

### Origin (Relay Origin)

**Point d'origine du Miyukini Webway System (MWS).** Origin possede les fonctions de **relay** et de **tracker** ; il est la **source de verite unique** de l'ecosysteme.

**Fonctions :**

- Verification de conformite des COGs (Passeport, cle Cores, blocs de code Services).
- Hebergement du Registre de Services officiel (maitre).
- Distribution des versions des Cores et des mises a jour des Services.
- Delivrance des Passeports speciaux (exclusif a Origin).
- Gestion des whitelists, blacklists et quarantaines (reference).
- Point d'entree initial de tout COG (si sature, redirige vers un relay).

Les **relays** sont des duplications d'Origin sous son autorite. Les **trackers** heritent les criteres legers d'Origin via les relays.

**Voir aussi :** Relay Webway, Registre de Services, Verite distribuee d'Origin, Passeport COG

---

### Opérateur de Domaine (Domain Operator)

**Type d'Opérateur** qui exerce un métier précis.

**Exemples :** Blog, Catalogue, Support, Base de connaissances, Forum

**Phrase type :** *"Exerce ce métier précis."*

**Voir aussi :** Opérateur, Opérateur de Service

---

### Opérateur (Operator)

**Entité fonctionnelle gouvernée** qui exécute un rôle pour le compte de l'utilisateur (Strate 7).

**Définition canonique :**

> **Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.**

**Types d'Opérateurs :**


| Type                       | Rôle                         |
| -------------------------- | ---------------------------- |
| Opérateur de Service       | Gère un domaine fonctionnel  |
| Opérateur d'Interface      | Expose les services          |
| Opérateur d'Automatisation | Agit automatiquement         |
| Opérateur de Domaine       | Exerce un métier             |
| Opérateur Souverain        | Autorité système (exception) |


**Ce qu'un Opérateur N'EST PAS :**

- ❌ Un produit
- ❌ Une app
- ❌ Autonome
- ❌ Souverain

**Phrase fondatrice :**

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

**Voir aussi :** Outil, Kit d'Outils

---

### Outil (Tool)

**Capacité exécutable gouvernée** (Strate 6), sans autorité, sans décision métier, sans connaissance du contexte.

**Définition canonique :**

> **Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.**

**Caractéristiques :**

- Capacité atomique
- Sans autorité
- Sans logique métier
- Gouverné par les Cores

**Règle fondamentale :**

> **👉 Un Outil fait, mais ne décide jamais.**

**Exemples :** `layout.render`, `form.validate`, `query.execute`

**Voir aussi :** Kit d'Outils, Opérateur

---

## P

### Passeport COG (COG Passport)

**Document d'identite complet** transmis par un COG lors de sa verification aupres d'Origin ou d'un relay. Le Passeport contient :

- `cog_id` : identifiant unique.
- `core_version` : version des Cores.
- `service_list` : liste des Services installes avec versions et checksums.
- `environment_health` : rapport de sante de l'environnement.
- `previous_permis` : historique des Permis de circulation precedents.
- `passport_type` : STANDARD ou SPECIAL.

**Voir aussi :** Passeport special, Permis de circulation, Presentation d'identite, Origin

---

### Parentalite COG (COG Parent-Child Link)

**Lien declare et verifie** entre un **COG pere** (Cores plus anciens) et un **COG fils** (Cores plus recents) dans le cadre d'une migration. Le COG fils enregistre sa parentalite aupres du pere. Ce lien **renforce la securite** et la force du Passeport lors des controles ; un enfant d'un pere sûr de longue date peut passer plus rapidement les controles douaniers des trackers. Chaque COG conserve son Passeport unique.

**Voir aussi :** Migration COG (pere/fils), Passeport COG

---

### Passeport special (Special Passport)

**Passeport delivre exclusivement par Origin** a des COGs a usage **professionnel ou a fort trafic** (sites de grandes entreprises, serveurs de services, jeux MMO). Le Passeport special comporte une **ID speciale** et une **cle speciale**.

**Caracteristiques :**

- Controle **allege au quotidien** pour gagner en performance.
- Controle **renforce lors des audits** planifies ou declenches.
- Facilites de connexion avec risques assumes.
- Delivrance via un protocole specifique d'audit prealable par Origin.

**Voir aussi :** Passeport COG, Origin, Pool de version

---

### Pool de version (Version Pool)

**Regroupement de COGs** gere par les trackers, isole par `core_version.MAJOR`. Les trackers dirigent chaque COG vers le pool correspondant a sa version des Cores. **Aucune connexion inter-pool n'est autorisee** : des COGs avec des versions majeures differentes ne sont jamais connectes entre eux.

**Voir aussi :** COG Tracker (Webway), Version des Cores, Permis de circulation

---

### Presentation d'identite (Identity Presentation)

**Processus par lequel un COG se presente au reseau Webway** en declarant son identite complete (cog_id, empreinte de version, attestation d'environnement). La presentation d'identite se deroule en deux temps :

1. **Auto-revue interne** : les Cores du COG (WorrySentinel, Border Guard, KindMother, StrongFather) auditent l'environnement installe pour verifier sa coherence et son integrite. Le resultat est une **attestation d'environnement** signee.
2. **Verification externe** : le relay verifie l'attestation, les checksums et la concordance entre l'identite declaree et l'environnement atteste (anti-deguisement). Le Tracker ne fait qu'une verification legere de l'identite.

**Voir aussi :** Attestation d'environnement, Conformite environnement, Verification de conformite, Relay Webway

---

### Permission

**Droit accordé pour accéder à une capacité.** Autorisation conceptuelle d'utiliser une capacité.

**Caractéristiques :**

- Définie explicitement
- Associée à des capacités
- Attribuable à des rôles
- Révocable
- Traçable

**Distinction Capability vs Permission :**


| Aspect   | Capability           | Permission          |
| -------- | -------------------- | ------------------- |
| Nature   | Pouvoir technique    | Droit accordé       |
| Question | "Peut-on le faire ?" | "A-t-on le droit ?" |


**Voir aussi :** Capability, Master Butler

---

### Passeport Utilisateur (User Passport)

**Attestation d'identité** émise par un COG Origine pour permettre à un utilisateur de visiter d'autres COG.

**Émis par :** COG Origine

**Contient :**
| Champ | Description |
|-------|-------------|
| `user_id` | Identité utilisateur unique |
| `cog_origin_id` | ID du COG d'origine |
| `core_version` | Version exacte de la strate Core |
| `integrity_hash` | Empreinte d'intégrité (Core + Kernel) |
| `issued_at` | Timestamp d'émission |
| `valid_until` | Durée de validité |
| `signature` | Signature du COG Origine |

**Garanties :**
- Non falsifiable
- Non transférable
- Lisible mais non modifiable

**Règle fondamentale :**

> **Le passeport ne donne aucun droit. Il prouve seulement qui tu es et d'où tu viens.**

**Voir aussi :** COG Origine, Visa de Connexion, Visite gouvernée inter-COG

---

### Politique de résidence des données sensibles (Sensitive Data Residence Policy)

**Règle gouvernant où réside la copie canonique** des données sensibles : certaines données (personnelles, métier critique) ne doivent pas être dupliquées comme seule copie sur des terminaux ou des COG tiers.

**Contenu de la politique :**
- **Données à résidence centralisée** : liste ou critères (domaine, niveau WorrySentinel 2+) des données dont la copie canonique doit résider sur un COG de référence
- **COG de référence** : désignation du COG détenteur canonique (ex. COG organisateur, COG du Service)
- **Terminaux / COG tiers** : accès en lecture via Visite gouvernée ou sync ; écritures soumises en WriteIntent, validées et persistées sur la Mère (COG de référence)
- **Interdiction** : la seule copie de ces données ne doit jamais résider uniquement sur un terminal ou un COG non désigné comme COG de référence

**Effet :** En cas de coupure du terminal (ex. exposant), les données restent disponibles sur le COG de référence (ex. pour les organisateurs).

**Voir aussi :** COG de référence, KindMother (Instance Mère), WorrySentinel, Niveaux de sécurité, Migration

---

### Pyramide Miyukini

**Architecture en strates** de l'écosystème Miyukini.


| Strate | Nom                     | Contenu                           |
| ------ | ----------------------- | --------------------------------- |
| **9**  | MiyukiniAdmin           | Opérateur Souverain (exception)   |
| **7**  | Opérateurs              | Entités fonctionnelles gouvernées |
| **6**  | Tools & Toolkits        | Capacités exécutables             |
| **5**  | Interfaces & Adaptation | BondingBrother                    |
| **4**  | Cores Système           | StrongFather, KindMother, etc.    |
| **3**  | Invariants & Contrats   | Principes architecturaux          |
| **K**  | Kernel                  | Substrat technique neutre         |
| **0**  | Hardware & OS           | Réalité physique                  |


**Voir aussi :** Chaque strate individuellement

---

## Q

### Quarantaine (Webway Quarantine)

**Etat d'isolation temporaire** applique a un COG non conforme sur le Webway. La quarantaine suit une **escalade progressive** :

| Etape | Delai | Action |
|-------|-------|--------|
| 1ere non-conformite | **1 heure** | Isolation, reseau informe, journalise |
| 2eme non-conformite | **2 heures** (x2) | Idem, delai double |
| 3eme non-conformite | **Blacklistage** | COG et IP blacklistes pour tout le reseau |

Le COG en quarantaine peut retenter la verification apres expiration du delai. Le COG blackliste suit le protocole d'auto-destruction et reconstruction (voir Relay Webway section 2.9).

**Voir aussi :** Origin, Relay Webway, Confinement reseau

---

## R

### Relay Webway (Miyukini Webway Relay)

**Duplication d'Origin** sous son autorite. Composant de **transport et de confiance** du Miyukini Webway System. Le relay garantit la conformite des COGs (verification en trois phases : cle Cores, blocs de code Services, sante environnement), assure la maintenance des environnements et la distribution des versions (mises a jour). Il possede la liste officielle des services disponibles aux COGs, heritee d'Origin. Le relay delivre les **Permis de circulation** (accord relay) aux COGs conformes.

**Voir aussi :** Origin, Passeport COG, Permis de circulation, accord relay, Verification de conformite, Cle de conformite des Cores, Bloc de code (verification), Quarantaine, [Miyukini Conceptual References - Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md), [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md)

---

### Relay Origin

Synonyme historique d'**Origin**. Designe Origin dans sa fonction de relay (source de verite pour le versioning, le Registre de Services, la delivrance des Passeports speciaux). Voir **Origin (Relay Origin)** pour la definition complete.

**Voir aussi :** Origin, Relay Webway, Registre de Services, [Miyukini Conceptual References - Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 1

---

### Registre de Services (Service Registry)

**Base de donnees maintenue par le Relay Origin** qui repertorie tous les services autorises sur le Webway :

| Categorie | Description |
|-----------|-------------|
| **Services officiels Miyukini** | Services developpes et maintenus par Miyukini (service_id, version courante, checksum, URL telechargement, core_compatibility, statut). |
| **Services tiers repertories** | Services tiers audites et autorises (service_id avec prefixe namespace editeur, editeur, source officielle, version, review_status : APPROVED, PENDING_REVIEW, SUSPENDED). |

**Fonctions :**

- **Verification** : les relays, Trackers et COGs consultent le Registre pour verifier la presence et le statut de chaque service.
- **Mises a jour** : le Registre fournit les versions courantes et les URLs de telechargement pour le suivi des mises a jour.
- **Redirection** : pour les services tiers, le Registre fournit l'URL de la source officielle de l'editeur.

**Voir aussi :** Relay Origin, Service repertorie, Service non repertorie, Suivi des mises a jour

---

### RETIRÉ (RETIRED) — état de vie

État d'un élément retiré du système. Non disponible, usage impossible.

**Caractéristiques :**

- Transition obligatoire depuis DÉPRÉCIÉ
- Pas de retour possible

**Voir aussi :** DÉPRÉCIÉ, Ever Buddy

---

## S

### Serveur web embarque (COG) (Embedded Web Server)

**Serveur web integre a un COG** permettant a certains Services de fonctionner en **headless** et en **permanence**, et de proposer leur service sur des **navigateurs web**. Exemples : site web, SaaS, portail visiteur (ex. disponibilite web JayFestival). Expose sur les ports 80 et/ou 8080 ; les trackers peuvent faciliter l'acces via le **catalogue web**.

**Voir aussi :** Surface de connexion, Catalogue web (Tracker), Limite de connexions (COG classique)

---

### Surface de connexion (Connection Surface)

**Perimetre explicite** des services et ports d'un COG ouverts aux connexions externes. Toute connexion **en dehors de la surface** est **systematiquement rejetee**. L'integrite des Cores et de la DB est prioritaire. La surface definit ce qui est autorise ; le reste est refuse. Un COG classique est en outre soumis a la **limite de 100 connexions simultanees** (hors ports 80 et 8080).

**Voir aussi :** Limite de connexions (COG classique), Serveur web embarque (COG), [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 8.1

---

### Sécurité Hétérogène (Heterogeneous Security)

**Principe selon lequel une Équipe d'Opérateurs peut combiner différents niveaux de sécurité**, chaque Opérateur gardant son propre niveau.

**Règle fondamentale :**

> **Un Opérateur n'a qu'un seul niveau de sécurité. Une Équipe peut en combiner plusieurs.**

**Exemple :**


| Opérateur         | Rôle      | Sécurité      |
| ----------------- | --------- | ------------- |
| Opérateur UI      | Affichage | 🟢 Faible (1) |
| Opérateur Contenu | CMS       | 🟡 Moyen (2)  |
| Opérateur Auth    | Identité  | 🔴 Élevé (3)  |


**Résultat :** Risque segmenté, pas sécurité uniforme forcée.

**Règles absolues :**

- Un Opérateur ne peut jamais élever son niveau
- Un flux ne peut jamais descendre en sécurité
- Les ponts entre niveaux sont explicites, rares, auditables
- Les ponts sont validés par WorrySentinel

**Voir aussi :** Niveaux de sécurité, Équipe d'Opérateurs, WorrySentinel

---

### Service

**Capacité perçue par l'utilisateur.** Le Service est ce que l'utilisateur voit et utilise.

**Distinction fondamentale :**


| Concept       | Définition                        |
| ------------- | --------------------------------- |
| **Service**   | Capacité perçue par l'utilisateur |
| **Opérateur** | Unité d'exécution gouvernée       |


**Règle clé :**

> **Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs.**

**Implications :**

- L'utilisateur voit des Services
- Le système exécute via des Opérateurs
- La complexité est gérée par collaboration, pas accumulation

**Voir aussi :** Opérateur, Équipe d'Opérateurs, Service Fondamental, Types de Services

---

### Service Fondamental (Fundamental Service)

**Service dont la présence fait partie de l'environnement versionné du COG.** Un Service Fondamental n'est pas optionnel ; il constitue un point d'entrée structurel de l'écosystème.

**Services Fondamentaux :**

| Service | Rôle | Cible |
|---------|------|-------|
| **Miyukini Central** | Hub de gestion des Services — point d'entrée utilisateur COG | Utilisateur du COG |
| **Miyukini Web Portal** | Hub des surfaces web — point d'entrée utilisateurs externes | Utilisateurs externes (web) |

**Règle canonique :**

> **Central = COG, Portail = Web.**
>
> Ces deux Services Fondamentaux font partie intégrante de l'environnement versionné du COG.

**Caractéristiques :**

- Fait partie de l'environnement versionné (comme les Cores)
- Non optionnel (un COG sans Central ne peut pas être administré)
- Opérateur d'Interface (Strate 7)

**Voir aussi :** Miyukini Central, Miyukini Web Portal, Types de Services, Environnement

---

### Service non repertorie (Unregistered Service)

**Service installe dans un COG mais absent du Registre de Services du Relay Origin.** Un service non repertorie peut avoir ete installe hors ligne ou retire du Registre. Sa presence dans le `service_manifest` d'un COG declenche l'**isolation reseau** du COG par le Webway.

**Consequences :**

- Le COG est **isole du maillage MWS** : pas d'annonces de presence, pas de reponses de decouverte, pas de routing de donnees.
- Le tunnel relay est maintenu en **mode surveillance** (heartbeats, notifications, consultation du Registre).
- L'utilisateur est **notifie** de la raison et des actions correctives.
- L'evenement est **journalise** au niveau du maillage (Relay Origin + Trackers).

**Levee d'isolation :** le COG se re-enregistre avec un manifest conforme ou le service est ajoute au Registre.

**Voir aussi :** Registre de Services, Relay Origin, Isolation reseau (Webway), Service repertorie

---

### Service repertorie (Registered Service)

**Service present dans le Registre de Services du Relay Origin**, qu'il soit officiel Miyukini ou tiers autorise. Un service repertorie a un statut dans le Registre : APPROVED (audite, autorise), PENDING_REVIEW (en attente), ou SUSPENDED (temporairement retire).

**Voir aussi :** Registre de Services, Relay Origin, Service non repertorie

---

### Suivi des mises a jour (Update Tracking)

**Capacite de chaque COG connecte au Webway** de suivre et gerer les mises a jour de ses Services :

- **Verification periodique** : le COG interroge le Registre du Relay Origin pour comparer son `service_manifest` aux versions courantes.
- **Notification push** : le relay peut envoyer un message UPDATE_AVAILABLE lorsqu'une mise a jour est disponible.
- **Registre local de versions** : le COG maintient un historique de ses mises a jour (version, date, statut : appliquee, reportee, ignoree).
- **Decision souveraine** : le COG decide d'appliquer ou non les mises a jour. Pour les mises a jour critiques (securite), une degradation ou isolation progressive peut etre appliquee si le delai est depasse.

**Voir aussi :** Relay Origin, Registre de Services, Empreinte de version COG

---

### Operateur de Service (Service Operator)

**Type d'Opérateur** qui gère un domaine fonctionnel.

**Exemples :** CMS, Auth, E-commerce, CRM, Surveillance, Recherche, Facturation

**Phrase type :** *"Gère ce domaine pour moi."*

**Voir aussi :** Opérateur, Opérateur de Domaine, Service

---

### Souveraineté (Environnement)

**Principe selon lequel un COG est une entité souveraine**, versionnée, isolée et identifiable.

**Règles fondamentales :**

- La strate Cores est immuable
- Pas de patch, que des environnements complets
- Un Opérateur est lié à un environnement unique
- Migration = diplomatie explicite

**Voir aussi :** Environnement, LOI-7, LOI-8

---

### Opérateur Souverain (Sovereign Operator)

**Type d'Opérateur d'exception** avec autorité quasi institutionnelle.

**Seul exemple :** MiyukiniAdmin

**Caractéristiques :**

- N'est pas un citoyen normal
- Agit sous protocole spécial
- N'est pas utilisable par d'autres Opérateurs

**Voir aussi :** MiyukiniAdmin, Opérateur

---

### StrongFather

**Core de décision** (Strate 4). Moteur de décision stratégique et politique. **Émetteur des Mandats de Permission.**

**Rôle :** Décider si une action devrait être faite, sans jamais l'exécuter.

**Question fondamentale :** *"Devrait-on faire cette action ?"*

**Responsabilités clés :**

- Décision stratégique
- Validation des Contrats d'Équipe
- Émission des Mandats de Permission
- Révocation des mandats si nécessaire

**Invariants clés :**

- Ne possède jamais d'autorité d'exécution
- Ne modifie jamais un état ou un fait
- Décision ≠ Exécution

**Voir aussi :** Cores, KindMother, Mandat de Permission, Contrat d'Équipe

---

## T

### TAMR (Trust & Authority Mediation Resolver)

**Core d'intervention humaine** (Strate 4). Définit les points d'intervention humaine dans le système.

**Rôle :** Définir quand l'humain a le droit d'intervenir.

**Question fondamentale :** *"Quand l'humain a-t-il le droit d'intervenir dans le système ?"*

**Voir aussi :** Cores, WorrySentinel

---

### Types de Services (Service Types)

**Classification empirique des Services** dans l'écosystème Miyukini COG. Tout Service doit se ranger dans l'un des trois types et prévoir les espaces correspondants.

| Type | Nom | Description | Espaces |
|------|-----|-------------|---------|
| **Type 1** | Service interne COG | Destiné uniquement à l'utilisateur du COG. Aucune surface externe. | Central uniquement |
| **Type 2** | Service à surface web externe | Gestion dans le COG + surface web pour utilisateurs externes. | Central + Portail |
| **Type 3** | Service Inter-COG | Interactions entre COGs (jeux multijoueur, fédération). | Central + Protocoles Inter-COG |

**Règle fondamentale :**

> **Tout Service doit déclarer son type (1, 2 ou 3) et prévoir les espaces correspondants.**

**Exemples :**

- **Type 1 :** JayKoa (agenda personnel)
- **Type 2 :** JayXpose (vitrine/e-shop), JayFestival (billets/visiteurs), JayRDV (réservation)
- **Type 3 :** Jeux multijoueur, collaboration inter-COG

**Voir aussi :** Service, Service Fondamental, Miyukini Central, Miyukini Web Portal, Façade Publique Gouvernée

---

### Token d'authentification relay (Webway)

**Secret ou jeton** utilisé par un COG pour s'authentifier auprès d'un Relay Webway lors de l'**enregistrement relay**. Le relay associe le tunnel à un `cog_id` après vérification du token ; les appels entrants vers ce COG sont alors routés vers ce tunnel. L'adresse annoncée sur le MWS peut être de la forme `relay_host:port` + token (ou identifiant dérivé) pour joindre le COG via le relay.

**Voir aussi :** Relay Webway, Enregistrement relay, COG participant (Webway)

---

### Tunnel (Webway)

**Connexion persistante** établie par un COG vers un Relay Webway après authentification (token) et enregistrement du `cog_id`. Le relay utilise ce tunnel pour router vers le COG le trafic entrant destiné à ce `cog_id`. Le COG peut envoyer des **heartbeats** pour maintenir le tunnel actif.

**Voir aussi :** Relay Webway, Enregistrement relay, Heartbeat (relay Webway)

---

## U

### Utilisateur Externe (Public User / Anonymous User / Web Visitor)

**Consommateur non certifié** de services exposés par un COG, sans aucune gouvernance propre.

**Ce qu'un utilisateur externe N'EST PAS :**
- ❌ Un citoyen
- ❌ Un visiteur inter-COG
- ❌ Un participant au système

**Caractéristiques :**
- Sans identité souveraine
- Sans COG d'origine
- Sans Passeport ni Visa
- Accès uniquement via Façade Publique Gouvernée
- Soumis à un Mandat Public d'Accès

**Dégradation agressive possible :**

| Action | Description |
|--------|-------------|
| Throttle | Ralentissement |
| Downgrade | Moins de fonctionnalités |
| Freeze | Lecture seule |
| Block | IP / session / pattern |
| Blackhole | Réponse neutre, pas d'erreur exploitable |

**Règle fondamentale :**

> **Un utilisateur externe n'entre jamais dans un COG. Il interagit uniquement avec une façade d'exposition gouvernée.**

**Phrase de synthèse :**

> **Les utilisateurs externes ne sont pas des visiteurs. Ce sont des consommateurs de surfaces exposées, sous mandat public.**

**Voir aussi :** Façade Publique Gouvernée, Mandat Public d'Accès, Utilisateur Visiteur

---

### Utilisateur Visiteur (Visitor User)

**Utilisateur accédant temporairement à un COG étranger** via un mécanisme de visite gouvernée.

**Statut :**
- Citoyen dans son COG d'origine
- Visiteur gouverné dans le COG hôte

**Caractéristiques :**
- Conserve son identité
- Perd toute souveraineté d'exécution
- Agit uniquement via un Visa de Connexion
- Ne transporte aucun core, aucune logique, aucun état

**Règle fondamentale :**

> **L'utilisateur n'est jamais souverain hors de son COG.**

**Voir aussi :** COG Origine, COG Hébergeur, Visa de Connexion, Passeport Utilisateur

---

## V

### Verification de conformite (Conformity Verification)

**Processus execute par Origin ou un relay** lors de la verification d'un COG pour s'assurer que l'identite declaree correspond a l'environnement reellement installe. La verification se decompose en **trois phases** :

1. **Phase A -- Cle de conformite des Cores** : les Cores transmettent une cle cachee dans le code, connue d'Origin/relay. La concordance prouve que les Cores sont authentiques.
2. **Phase B -- Blocs de code des Services** : chaque Service envoie un paquet chiffre contenant un bloc de code MIP choisi aleatoirement. Le relay dechiffre avec les references Origin ; un bon dechiffrement prouve que le Service est authentique et non corrompu. Verification renforcee possible sur tout le code en cas de doute.
3. **Phase C -- Sante de l'environnement** : verification du rapport de sante genere par les Cores (integrite du stockage, configuration, strates).

**Important :** cette verification lourde est la responsabilite du **relay**, pas du Tracker. Le Tracker ne fait qu'une verification legere de l'identite (cog_id, signature, core_version).

**Voir aussi :** Attestation d'environnement, Presentation d'identite, Conformite environnement, Relay Webway

---

### Permis de circulation (accord relay)

**Autorisation temporaire** delivree par Origin ou un relay (accord relay) a un COG ayant passe la verification de conformite. Le Permis de circulation permet au COG de se connecter au Webway via les **trackers officiels** ; les trackers effectuent le **contrôle tracker** (verification du permis).

**Validite :** Le Permis est **valable sur tout le reseau** accessible au COG qui le presente. Avec le Permis, le relay remet les **adresses des trackers officiels/sûrs** (trackers connus d'Origin). Un COG **ne peut pas et ne doit pas** se connecter a un tracker inconnu d'Origin ; il ne doit utiliser que les trackers de cette liste.

**Contenu du Permis de circulation :**

| Champ | Description |
|-------|-------------|
| `permis_id` | Identifiant unique du permis |
| `cog_id` | COG concerne |
| `issued_by` | Relay ou Origin emetteur |
| `issued_at` / `expires_at` | Validite temporelle |
| `scope` | Portee (intentions du COG : services, COGs a contacter) |
| `core_version` | Version des Cores validee |
| `passport_type` | STANDARD ou SPECIAL |
| `tracker_addresses` | Adresses des trackers officiels (le COG ne doit se connecter qu'a ces trackers). |

Les trackers verifient le Permis de circulation avant d'autoriser les connexions (contrôle tracker). Un Permis expire oblige le COG a se re-verifier aupres d'un relay.

**Voir aussi :** Passeport COG, accord relay, contrôle tracker, Quarantaine, Pool de version, Origin

---

### Accord d'hôte (Host Access)

**Autorisation delivree par le COG hôte** au COG client pour **consommer les services exposes** par ce hôte (Lobby). Distinct du **Permis de circulation** (delivre par le relay/Origin) : le Permis de circulation autorise a circuler sur le Webway ; l'accord d'hôte autorise l'acces aux ressources et services d'un COG hôte determine. Le COG client se connecte au COG hôte en suivant les protocoles de securite et consomme les services grace a cet accord.

**Voir aussi :** Permis de circulation, Lobby (Webway), COG Hébergeur, [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) section 9.3

---

### Verite distribuee d'Origin (Origin Distributed Truth)

**Mecanisme par lequel les criteres du Relay Origin** (Registre de Services, versions Cores, checksums, politiques de conformite) **sont distribues a travers le reseau** Webway via les relays et, de maniere allegee, via les Trackers.

**Principes :**

- Chaque relay heberge une **copie (partielle ou complete)** des criteres d'Origin, synchronisee periodiquement.
- Les Trackers heritent les **criteres legers** (min_core_version, min_protocol_version, alertes) des relays auxquels ils sont rattaches.
- Les criteres sont diffuses aux COGs via les reponses REGISTER_OK, REGISTRY_RESPONSE, UPDATE_AVAILABLE et les reponses de decouverte MWS.
- Un relay temporairement deconnecte d'Origin continue de fonctionner avec son cache local.

**Voir aussi :** Relay Origin, Registre de Services, Relay Webway

---

### Version des Cores (Core Version)

**Identifiant de version** du socle de gouvernance d'un COG, au format `MAJOR.MINOR`. Les Cores (StrongFather, KindMother, Caring Nanny, Master Butler, Border Guard, Ever Buddy, WorrySentinel, TAMR) sont **immuables** a version donnee.

**Regles :**

- Le `MAJOR` determine la **compatibilite stricte** entre COGs : deux COGs doivent avoir le meme `MAJOR` pour interagir.
- Le `MINOR` indique des ajustements internes compatibles (ex. correctifs de documentation, optimisations sans rupture d'interface).
- Un changement de `MAJOR` signifie une **rupture d'interface** ou de contrat au niveau des Cores ; les COGs avec des `MAJOR` differents ne peuvent pas interagir de maniere fiable.
- Les Services (Operateurs, Outils, Kits d'Outils) peuvent etre patches independamment sans changer la Version des Cores.

**Voir aussi :** Empreinte de version COG, Compatibilite de version COG, Cores

---

### Verified ID (VID)

**Niveau 2 d'identité d'environnement.** LSI vérifiée par un registre global.

**Cas d'usage :** Environnement connecté, fédéré.

**Confiance :** Attestée — un tiers a vérifié l'identité.

**Voir aussi :** Local Sovereign ID, Witnessed ID

---

## W

### Witnessed ID (WID)

**Niveau 3 d'identité d'environnement.** LSI vérifiée par échange indirect.

**Cas d'usage :** Environnement semi-connecté, clé USB, QR, signature.

**Confiance :** Témoignée — d'autres environnements attestent.

**Voir aussi :** Local Sovereign ID, Verified ID

---

### Visa de Connexion (Connection Visa)

**Autorisation temporaire** émise par un COG Hébergeur pour encadrer la session d'un Utilisateur Visiteur.

**Émis par :** Host COG

**Contient :**
| Champ | Description |
|-------|-------------|
| `authorized_services` | Services autorisés |
| `accessible_cores` | Cores accessibles (indirectement) |
| `security_level` | Niveau de sécurité accordé |
| `execution_rules` | Règles d'exécution |
| `time_limits` | Limites temporelles |
| `functional_limits` | Limites fonctionnelles |
| `terminal_constraints` | Contraintes terminal |
| `revocation_conditions` | Conditions de révocation |

**Caractéristiques :**
- Temporaire
- Révocable
- Non transférable
- Auditée
- Strictement interprétée

**Niveaux de sécurité du Visa :**

| Niveau | Nom | Usage typique |
|--------|-----|---------------|
| **S1** | Observation | Lecture, spectateur |
| **S2** | Interaction contrôlée | UI, formulaires |
| **S3** | Temps réel | Jeu, collaboration |
| **S4** | Sensible | Admin, finance |
| **S5** | Critique | MiyukiniAdmin |

**Règle fondamentale :**

> **Le Visa définit l'univers légal du visiteur. Un utilisateur = un Visa = un niveau unique.**

**Voir aussi :** COG Hébergeur, Utilisateur Visiteur, Visite gouvernée inter-COG

---

### Visite gouvernée inter-COG (Inter-COG Governed Visit)

**Modèle d'accès temporaire** permettant à un utilisateur d'un COG d'accéder aux services d'un autre COG sans importer sa gouvernance.

**Acteurs :**
| Acteur | Rôle |
|--------|------|
| **COG Origine** | Autorité d'identité, émetteur du Passeport |
| **Utilisateur Visiteur** | Citoyen visitant sous gouvernance étrangère |
| **COG Hébergeur** | Souverain exécutif, émetteur du Visa |
| **Bridge inter-COG** | Canal diplomatique (BondingBrother étendu) |

**Séquence :**
1. Pré-validation locale (COG Origine)
2. Présentation au Bridge (Passeport + Demande de Visite)
3. Douane du Host COG (vérification)
4. Émission du Visa
5. Session active (gouvernée)
6. Fin ou rupture

**Principes non négociables :**
- ❌ Aucun core n'est partagé
- ❌ Aucun état n'est migré en direct
- ❌ Aucun pouvoir n'est délégué
- ✅ Une seule gouvernance active
- ✅ Identité ≠ autorité
- ✅ Sécurité avant fluidité

**Phrase fondatrice :**

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

**Voir aussi :** COG Hébergeur, COG Origine, Passeport Utilisateur, Visa de Connexion, Utilisateur Visiteur, Bridge inter-COG

---

### WorrySentinel

**Core de gouvernance de sécurité** (Strate 4). Gouverne les niveaux de sécurité et les états de confiance.

**Rôle :** Gouverner la sécurité sans exécuter de contrôle technique.

**Question fondamentale :** *"Quel niveau de sécurité et quel état de confiance sont applicables ?"*

**Responsabilité Tools :** Niveau de sécurité requis, blocage en cas de menace, audit.

**Ce que WorrySentinel décide :**

- ✅ Niveau de confiance global
- ✅ Niveau de sécurité actif
- ✅ Mode de fonctionnement autorisé

**Ce que WorrySentinel ne décide PAS :**

- ❌ Des actions
- ❌ Des permissions
- ❌ Des données

**Voir aussi :** Niveaux de sécurité, États de confiance

---

### Intention d'Écriture (WriteIntent)

**Intention d'écriture** soumise à KindMother. Représente une demande de modification de données.

**Caractéristiques :**

- Soumise à validation
- Traçable
- Peut être acceptée, refusée, ou différée

**Voir aussi :** KindMother

---

## Phrases Fondatrices (Résumé)

### COG

> *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

### Opérateur (Operator)

> **Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

### Outil & Kit d'Outils (Tool & Toolkit)

> **Les Outils sont des capacités exécutables gouvernées. Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

### Souveraineté

> **Dans Miyukini, la strate Cores est immuable. Toute évolution se fait par la création d'un nouvel environnement complet. Les Opérateurs sont liés à un environnement unique et ne peuvent exister hors de celui-ci.**

### Autonomie

> **Le réseau améliore le système, il ne le conditionne pas.**

### Complexité

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

### Mandat de Permission

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Sécurité

> **Risque segmenté, pas sécurité uniforme.**

### Visite inter-COG

> **Un COG n'accueille jamais une gouvernance étrangère. Il n'accueille que des visiteurs, sous visa, dans un cadre qu'il définit seul.**

### Utilisateurs externes

> **Les utilisateurs externes ne sont pas des visiteurs. Ce sont des consommateurs de surfaces exposées, sous mandat public.**

> **Un utilisateur externe n'entre jamais dans un COG. C'est le COG qui sort vers lui, jamais l'inverse.**

### Services Fondamentaux

> **Central = COG, Portail = Web.**

> **Les Services Fondamentaux (Central, Portail) font partie de l'environnement versionné du COG.**

### Types de Services

> **Tout Service doit déclarer son type et prévoir les espaces correspondants.**

---

## Tableau de correspondance terminologique


| ❌ Terme incorrect               | ✅ Terme correct                            |
| ------------------------------- | ------------------------------------------ |
| Produit                         | **Opérateur**                              |
| App                             | **Opérateur** ou **Opérateur d'Interface** |
| Produit final                   | **Opérateur**                              |
| Produit intermédiaire           | **Outil** ou **Kit d'Outils**              |
| Créer un produit                | **Déployer un Opérateur**                  |
| Utiliser une app                | **Interagir avec un Opérateur**            |
| Marketplace                     | **Registre d'Opérateurs**                  |
| Decision Window                 | **Mandat de Permission**                   |
| Temporary Decision              | **Autorisation Mandatée**                  |
| Fast Path                       | **Chemin Mandaté**                         |
| Collaboration Opérateur (libre) | **Collaboration Mandatée**                 |
| Super-Opérateur                 | **Équipe d'Opérateurs**                    |
| Tool                            | **Outil**                                  |
| Toolkit                         | **Kit d'Outils**                           |
| Operator                        | **Opérateur**                              |
| User Passport                   | **Passeport Utilisateur**                  |
| Connection Visa                 | **Visa de Connexion**                      |
| Visit Intent                    | **Demande de Visite**                      |
| Visitor User                    | **Utilisateur Visiteur**                   |
| Host COG                        | **COG Hébergeur**                          |
| Home COG                        | **COG Origine**                            |
| Reference COG / Official COG    | **COG de référence**                       |
| Inter-COG Bridge                | **Bridge inter-COG**                       |
| Public User                     | **Utilisateur Externe**                    |
| Anonymous User                  | **Utilisateur Externe**                    |
| Web Visitor                     | **Utilisateur Externe**                    |
| Public Exposure Surface         | **Façade Publique Gouvernée**              |
| Public Access Mandate           | **Mandat Public d'Accès**                  |
| Tracker (rôle Webway)           | **COG Tracker**                            |
| Webway Participant              | **COG participant (Webway)**               |
| Webway COG List                 | **Liste de COGs avec statuts**             |
| Host Session Declaration        | **Déclaration d'hébergement de session**   |
| MWS Secure Declaration          | **Norme de déclaration sécurisée (MWS)**   |
| Miyukini Webway Relay           | **Relay Webway**                           |
| Webway Tunnel                   | **Tunnel (Webway)**                        |
| Relay Registration              | **Enregistrement relay**                   |
| Relay Heartbeat                 | **Heartbeat (relay Webway)**               |
| Relay Auth Token                | **Token d'authentification relay**         |
| MWS                             | **Miyukini Webway System**                 |
| COG Version Fingerprint         | **Empreinte de version COG**               |
| Core Version                    | **Version des Cores**                      |
| COG Version Compatibility       | **Compatibilite de version COG**           |
| Service Patch                   | **Patch de Service**                       |
| Relay Origin                    | **Relay Origin**                           |
| Service Registry                | **Registre de Services**                   |
| Registered Service              | **Service repertorie**                     |
| Unregistered Service            | **Service non repertorie**                 |
| Webway Network Isolation        | **Isolation reseau (Webway)**              |
| Update Tracking                 | **Suivi des mises a jour**                 |
| Environment Attestation         | **Attestation d'environnement**            |
| Environment Conformity          | **Conformite environnement**               |
| Identity Presentation           | **Presentation d'identite**                |
| Conformity Verification         | **Verification de conformite**             |
| Origin Distributed Truth        | **Verite distribuee d'Origin**             |
| COG Passport                    | **Passeport COG**                          |
| Special Passport                | **Passeport special**                      |
| Circulation Visa / Permis       | **Permis de circulation** (accord relay)    |
| Version Pool                    | **Pool de version**                        |
| Webway Quarantine               | **Quarantaine (Webway)**                   |
| Core Conformity Key             | **Cle de conformite des Cores**            |
| Code Block Verification         | **Bloc de code (verification)**            |
| Network Containment             | **Confinement reseau**                     |
| Origin (MWS)                    | **Origin (Relay Origin)**                  |
| Hub / Dashboard                 | **Miyukini Central** (pour utilisateur COG) |
| Web Portal / Public Portal      | **Miyukini Web Portal** (pour utilisateurs externes) |
| Service Type 1                  | **Service interne COG**                    |
| Service Type 2                  | **Service à surface web externe**          |
| Service Type 3                  | **Service Inter-COG**                      |
| COG Migration (parent/child)    | **Migration COG (pere/fils)**              |
| COG Parent-Child Link           | **Parentalite COG**                         |
| Connection Surface              | **Surface de connexion**                    |
| Web Catalog (Tracker)            | **Catalogue web (Tracker)**                 |
| Connection Limit (classic COG)  | **Limite de connexions (COG classique)**    |
| Embedded Web Server (COG)       | **Serveur web embarque (COG)**              |
| Lobby (Webway)                  | **Lobby (Webway)**                          |
| COG Favorites                   | **Favoris (COG)**                           |
| COG Friends                     | **Amis (COGs)**                             |
| Host Access Visa                | **Accord d'hôte**                           |


---

**Date de création :** 2026-01-27  
**Version :** 1.18 (Lobby, Favoris, Amis COGs, accord d'hôte ; surfaces au tracker, Lobbys prives, flow client-hôte)  
**Statut :** Document de référence normatif — GLOSSAIRE OFFICIEL

**Références croisées :**

- [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)
- [Miyukini Conceptual References - Types de Services et Espaces](./Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) : **Classification des Services (Type 1, 2, 3)**
- [Miyukini Conceptual References - Miyukini Central Hub Services](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) : **Service Fondamental — Hub COG**
- [Miyukini Web Portal - Document Fondateur](../services/MiyukiniWebPortal/Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) : **Service Fondamental — Hub Web**
- [Miyukini Conceptual References - Politique Residence Donnees Sensibles](./Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) : **Centralisation et résidence des données sensibles**
- [Miyukini Conceptual References - Miyukini Webway System](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) : **Couche de présence et découverte (MWS)**
- [Miyukini Conceptual References - Miyukini Webway System Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) : **Annexe MWS — normes, formats, protocole, matrice des statuts**
- [Miyukini Conceptual References - Miyukini Webway System Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) : **Annexe MWS — Outils, Kits d'Outils, Opérateurs MWS**
- [Miyukini Conceptual References - Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) : **Architecture relay de transport (tunnel étendu multi-tenant)**
- [Miyukini Conceptual References - Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) : **Protocole relay (messages, handshake, TLS)**
- [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- [Miyukini Conceptual References - Mandats et Équipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) : **Mandats de Permission et Équipes**
- [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- [Miyukini Conceptual References - Souveraineté Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md)
- [Miyukini Conceptual References - Objectif Final](./Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md)
- [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) : **Architecture de visite gouvernée**
- [Miyukini Conceptual References - Kernel Maintenance Observability Contract](./Miyukini%20Conceptual%20References%20-%20Kernel%20Maintenance%20Observability%20Contract.md) : **Capacités bas niveau de maintenance**

