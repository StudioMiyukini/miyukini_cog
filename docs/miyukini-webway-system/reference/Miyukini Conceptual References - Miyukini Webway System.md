# Miyukini Conceptual References - Miyukini Webway System

> **Racine documentaire MWS :** La documentation officielle du MWS a pour racine **`docs/miyukini-webway-system`**. Pour le document fondateur, l'architecture (subordination aux Cores) et la consommation par les strates, voir [docs/miyukini-webway-system](..//README.md).

## Contexte

Ce document dÃ©finit le **Miyukini Webway System (MWS)** : la couche de **prÃ©sence et de dÃ©couverte** des environnements COG disposant d'un accÃ¨s rÃ©seau. Le MWS permet aux COGs de se dÃ©clarer, de savoir qui est prÃ©sent sur le maillage, et de faciliter l'initiation des visites gouvernÃ©es (Passeport, Permis de circulation, Visa de Connexion) sans transfÃ©rer de donnÃ©es mÃ©tier. Il inclut un systÃ¨me de sÃ©curitÃ© fondÃ© sur l'Ã©change de listes de COGs avec statuts, et impose aux COGs Tracker un devoir de protection du rÃ©seau par des mÃ©canismes passifs et actifs.

**Principe fondamental :**

> **Le Webway normalise la prÃ©sence et facilite l'Ã©change entre environnements ; il ne transporte pas la gouvernance ni les donnÃ©es â€” il permet de savoir oÃ¹ et comment initier une visite gouvernÃ©e.**

## PortÃ©e / Scope

- DÃ©finition du Miyukini Webway System (MWS) et de son rÃ´le
- Acteurs : COG participant, COG Tracker
- **Annonces de prÃ©sence** : services exposÃ©s, adresses (IP et ports) associÃ©es, dÃ©claration d'hÃ©bergement de session (Host)
- **Norme de dÃ©claration sÃ©curisÃ©e** : schÃ©ma commun, signature et vÃ©rification pour les annonces de services, adresses et sessions hÃ©bergÃ©es (section 3.3)
- SystÃ¨me de sÃ©curitÃ© : listes de COGs avec statuts, Ã©change et analyse pour rejet de COGs ou connexions malveillantes
- Devoir des COGs Tracker : protection du rÃ©seau (systÃ¨mes passifs en 5.1 ; systÃ¨mes actifs en 5.2 â€” blocage, signalement, dÃ©gradation, alerte)
- Relation avec la Connexion Inter-COG (Passeport, Permis de circulation, Visa de Connexion, Bridge)
- Principes non nÃ©gociables et compatibilitÃ© avec les Lois d'Autonomie

Ce document **ne couvre pas** :
- Le dÃ©tail des **normes et standards** du MWS (formats, protocole, matrice des statuts, conformitÃ© Trackers) â†’ voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (document conceptuel annexe)
- Les **Outils et OpÃ©rateurs** nÃ©cessaires au MWS (Strate 6 et 7) â†’ voir [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) (document conceptuel annexe)
- Le dÃ©tail des protocoles de visite gouvernÃ©e â†’ voir [Connexion Inter-COG](_index.md)
- Les spÃ©cifications techniques dÃ©taillÃ©es des systÃ¨mes passifs/actifs (prÃ©conditions, postconditions, invariants) figurent dans des contrats dÃ©diÃ©s ; le cadre conceptuel des systÃ¨mes actifs est dÃ©fini en section 5.2.

---

## 1. Vue d'ensemble du Miyukini Webway System

### 1.1 RÃ´le du MWS

Le **Miyukini Webway System (MWS)** est la couche qui permet aux environnements COG ayant accÃ¨s au rÃ©seau de :

| CapacitÃ© | Description |
|----------|-------------|
| **Se dÃ©clarer** | Annoncer sa prÃ©sence (identitÃ© de COG, adresse de contact / Bridge) |
| **DÃ©couvrir** | Savoir quels COGs sont prÃ©sents et oÃ¹ les joindre |
| **Faciliter l'Ã©change** | Donner le point d'entrÃ©e pour initier une visite gouvernÃ©e (Passeport â†’ Permis de circulation â†’ Bridge â†’ Visa de Connexion) |

**Le MWS ne sert pas Ã  transfÃ©rer des donnÃ©es mÃ©tier.** Il est la transcription concrÃ¨te des concepts de prÃ©sence autour des Passeports et des Permis de circulation : il normalise *qui est lÃ * et *oÃ¹ se prÃ©senter* pour demander un Permis de circulation (relay) ou un accord d'hÃ´te / Visa de Connexion (COG hÃ´te).

**Analogie (orientation)** : Ã  la maniÃ¨re d'un rÃ©seau de type BitTorrent, les COGs peuvent s'annoncer et interroger des **Trackers** (COGs qui acceptent le rÃ´le de point de rendez-vous pour la dÃ©couverte) ; le transfert rÃ©el et la gouvernance restent dans le cadre de la visite gouvernÃ©e (Bridge, Visa de Connexion).

### 1.2 Principes cardinaux

> **Le maillage ne fait pas confiance â€” il transporte et expose des informations de prÃ©sence.**
> **La gouvernance (Passeport, Permis de circulation, Visa de Connexion) reste souveraine ; le Webway ne gouverne pas.**

- **Optionnel** : les environnements sans rÃ©seau ou qui refusent la dÃ©couverte restent souverains (LOI-2, LOI-6).
- **Aucun core partagÃ©** : la prÃ©sence ne donne aucun accÃ¨s aux Cores ; elle indique oÃ¹ aller pour initier une visite.
- **Une seule gouvernance active** : c'est toujours le COG HÃ©bergeur qui dÃ©cide (Visa de Connexion / accord d'hÃ´te, refus, rÃ©vocation) ; Origin/relays dÃ©cident du Permis de circulation.

---

## 2. Acteurs du Webway

### 2.1 COG participant (Webway Participant)

**DÃ©finition :** tout COG qui choisit de participer au maillage MWS (accÃ¨s rÃ©seau et dÃ©claration activÃ©e).

**RÃ´le :**
- Se dÃ©clarer auprÃ¨s d'un ou plusieurs COGs Tracker (ou au maillage) selon le protocole MWS
- Exposer les informations minimales de prÃ©sence (identitÃ© COG, adresse du Bridge / point de contact)
- Consulter la prÃ©sence d'autres COGs pour initier des visites gouvernÃ©es
- Participer au systÃ¨me de sÃ©curitÃ© en Ã©changeant et en tenant Ã  jour une **liste de COGs avec statuts** (voir section 3)

**ResponsabilitÃ©s :**
- Ne pas exposer de donnÃ©es mÃ©tier ni de gouvernance via le Webway
- Respecter les rÃ¨gles de sÃ©curitÃ© du maillage (listes de statuts, rejet de connexions malveillantes)

### 2.2 COG Tracker (Webway Tracker)

**DÃ©finition :** COG dont l'administrateur a choisi d'endosser le rÃ´le de **Tracker** : exposer volontairement une adresse (IP ou nom de domaine) pour participer au maillage et servir de point de rendez-vous pour la dÃ©couverte.

**Port officiel :** les COGs Tracker MWS exposent leur endpoint sur le **port 21000**. Les COGs participants se connectent aux Trackers sur ce port par dÃ©faut. Voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (section 2.7.4).

**RÃ´le :**
- Tenir un rÃ´le de **point de rendez-vous** pour la dÃ©couverte (enregistrement des annonces de prÃ©sence, rÃ©ponse aux requÃªtes de dÃ©couverte)
- **ProtÃ©ger le rÃ©seau** par des mÃ©canismes **passifs** et **actifs** (voir section 4)
- Ne pas dÃ©tenir de donnÃ©es mÃ©tier ni gouverner les accÃ¨s ; la dÃ©livrance des Visa de Connexion / accord d'hÃ´te reste du ressort de chaque COG HÃ©bergeur ; la dÃ©livrance du Permis de circulation reste du ressort d'Origin/relays

**Devoir fondamental :**

> **Les COGs Tracker ont le devoir de protÃ©ger le rÃ©seau par des systÃ¨mes passifs et actifs.**

Ils ne sont pas de simples annuaires : ils contribuent Ã  la santÃ© et Ã  la sÃ»retÃ© du maillage (dÃ©tection, signalement, filtrage, selon les mÃ©canismes dÃ©finis).

---

## 3. Annonces de prÃ©sence : services, adresses et sessions hÃ©bergÃ©es

### 3.1 Communication des services et adresses (IP / ports)

Les COGs participants peuvent **communiquer au rÃ©seau** les **services** qu'ils exposent et les **adresses** associÃ©es : **IP** (ou nom de domaine) et **ports**. Cela permet Ã  d'autres COGs de savoir oÃ¹ et comment initier une visite gouvernÃ©e vers un service donnÃ© (Bridge, endpoint).

**RÃ¨gle :** ces informations relÃ¨vent de la **prÃ©sence et de la dÃ©couverte** ; elles ne contiennent pas de donnÃ©es mÃ©tier ni de gouvernance. Le dÃ©tail des champs (identifiant de service, protocole, port, etc.) sera dÃ©fini dans le protocole MWS et la norme de dÃ©claration sÃ©curisÃ©e (voir 3.3).

### 3.2 DÃ©claration d'hÃ©bergement de session (Host)

Dans le cadre d'un **COG HÃ©bergeur** qui propose une **session** (ex. partie de jeu, salle de collaboration, service temporaire), il est nÃ©cessaire qu'il **dÃ©clare au rÃ©seau** :

> **Â« J'hÃ©berge une session de tel service et j'attends qu'on se connecte Ã  moi. Â»**

Cette dÃ©claration permet aux autres COGs (ou aux Utilisateurs Visiteurs via leur COG Origine) de **dÃ©couvrir** qu'une session est ouverte et d'oÃ¹ s'y connecter (adresse et port du Host), puis d'initier la visite gouvernÃ©e (Passeport â†’ Permis de circulation â†’ Bridge â†’ Visa de Connexion).

**Contenu minimal (orientation) :**
- Identifiant du service (ou type de session)
- Identifiant du COG HÃ©bergeur
- Adresse de connexion (IP ou nom de domaine, port)
- Ã‰ventuellement : capacitÃ© restante, niveau de sÃ©curitÃ© proposÃ©, selon protocole

**RÃ¨gle :** la dÃ©claration d'hÃ©bergement de session **ne donne aucun droit d'accÃ¨s** ; elle indique seulement oÃ¹ se prÃ©senter pour demander un Permis de circulation (relay) ou un Visa de Connexion / accord d'hÃ´te (COG HÃ©bergeur). L'accÃ¨s reste gouvernÃ© par le COG HÃ©bergeur (Douane, Visa de Connexion, rÃ©vocation).

### 3.3 Norme de dÃ©claration sÃ©curisÃ©e

Pour les annonces de **services**, d'**adresses** (IP/ports) et de **sessions hÃ©bergÃ©es**, une **norme de dÃ©claration sÃ©curisÃ©e** est dÃ©finie et doit Ãªtre **appliquÃ©e** par tous les COGs participants qui annoncent sur le Webway. Elle vise Ã  :

- **Authentifier** l'origine des dÃ©clarations (COG attestÃ©, non usurpation)
- **IntÃ©gritÃ©** : garantir que les dÃ©clarations n'ont pas Ã©tÃ© altÃ©rÃ©es en transit
- **Format unifiÃ©** : permettre l'interopÃ©rabilitÃ© et la vÃ©rification par les Trackers et les participants
- **Limiter les abus** : dÃ©clarations conformes, sans exposition de donnÃ©es sensibles ni de gouvernance

Le dÃ©tail des formats de messages (annonce de prÃ©sence, services, session hÃ©bergÃ©e, requÃªte de dÃ©couverte, liste de statuts) est dÃ©fini dans [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (sections 1 et 2). Ce qui suit en fixe le **cadre conceptuel** : schÃ©ma commun, signature et vÃ©rification.

#### 3.3.1 SchÃ©ma commun des dÃ©clarations

Toute dÃ©claration MWS (prÃ©sence, service, session hÃ©bergÃ©e) conforme Ã  la norme respecte une **structure commune** :

| Ã‰lÃ©ment | RÃ¨gle |
|--------|--------|
| **Version** | Champ obligatoire `version` (ex. `mws_declaration_v1`) indiquant la version de la norme utilisÃ©e. |
| **Type** | Champ obligatoire `type` identifiant le message (`presence_announcement`, `service_announcement`, `host_session_declaration`, etc.). |
| **IdentitÃ©** | Champ obligatoire identifiant le COG Ã©metteur (`cog_id` ou Ã©quivalent attestÃ©). |
| **Horodatage** | Champ obligatoire `issued_at` (ISO 8601) pour la traÃ§abilitÃ© et la limitation des rejeux. |
| **IntÃ©gritÃ©** | Champ obligatoire `integrity` contenant le mÃ©canisme de vÃ©rification (signature ou MAC â€” voir 3.3.2). |
| **Corps mÃ©tier** | Champs spÃ©cifiques au type (adresses, services, `session_id`, etc.) selon le schÃ©ma dÃ©fini dans Normes et Standards. |

**SÃ©rialisation canonique :** pour que la signature soit reproductible, la dÃ©claration doit Ãªtre **sÃ©rialisÃ©e de maniÃ¨re dÃ©terministe** (ordre des champs fixe, encodage unique, ex. JSON canonique ou CBOR) avant calcul de la signature. Seul le **contenu signÃ©** (corps de la dÃ©claration sans le champ `integrity`) est inclus dans lâ€™entrÃ©e de la fonction de signature.

**Champs interdits :** donnÃ©es utilisateur, secrets, contenu mÃ©tier, informations permettant dâ€™usurper une gouvernance. La norme restreint les champs autorisÃ©s Ã  la prÃ©sence et Ã  la dÃ©couverte.

#### 3.3.2 Signature (authentification et intÃ©gritÃ©)

- **Responsable de la signature :** le **COG Ã©metteur** (participant ou HÃ©bergeur) signe ses propres dÃ©clarations avec une clÃ© ou un secret **associÃ© Ã  son identitÃ©** (ex. clÃ© dÃ©rivÃ©e de lâ€™identitÃ© COG, certificat, mÃ©canisme attestÃ©).
- **PÃ©rimÃ¨tre signÃ© :** tout le contenu de la dÃ©claration **Ã  lâ€™exclusion du champ `integrity`** (version, type, cog_id, champs mÃ©tier, issued_at, etc.), aprÃ¨s sÃ©rialisation canonique.
- **MÃ©canisme :** la norme impose un **mÃ©canisme dâ€™intÃ©gritÃ©** (signature numÃ©rique ou MAC) dont le rÃ©sultat est placÃ© dans `integrity.value` (ex. encodage base64). Le champ `integrity` peut inclure `method` (ex. `signature`, `mac`), `algorithm` et `key_id` pour permettre au rÃ©cepteur de choisir la clÃ© ou lâ€™algorithme de vÃ©rification.
- **Objectifs :** attester que lâ€™Ã©metteur est bien le COG annoncÃ© et que le message nâ€™a pas Ã©tÃ© modifiÃ© en transit.

#### 3.3.3 VÃ©rification par le rÃ©cepteur (Tracker ou participant)

Le rÃ©cepteur (COG Tracker ou autre COG participant) **vÃ©rifie** chaque dÃ©claration avant de lâ€™accepter, de la relayer ou de lâ€™exploiter :

1. **ConformitÃ© du schÃ©ma** : prÃ©sence et format des champs obligatoires (version, type, cog_id, issued_at, integrity), types et contraintes (ports non exclus, plages de valeurs). Les schÃ©mas dÃ©taillÃ©s sont dans [Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (section 2).
2. **VÃ©rification dâ€™intÃ©gritÃ©** : reconstruire le contenu signÃ© en sÃ©rialisation canonique, puis vÃ©rifier la signature ou le MAC Ã  lâ€™aide de la clÃ© ou du secret associÃ© au `cog_id` (registre local, infrastructure Ã  clÃ©s publiques, ou mÃ©canisme attestÃ© selon dÃ©ploiement).
3. **CohÃ©rence identitÃ©â€“signature** : sâ€™assurer que la clÃ© utilisÃ©e pour la vÃ©rification est bien liÃ©e au COG annoncÃ© dans la dÃ©claration (non usurpation).
4. **Optionnel â€” limitation des rejeux** : vÃ©rifier que `issued_at` est dans une fenÃªtre temporelle acceptÃ©e (ex. pas trop ancien) selon politique locale.

En cas dâ€™Ã©chec (schÃ©ma invalide, signature invalide, identitÃ© incohÃ©rente), le rÃ©cepteur **rejette** la dÃ©claration et peut, selon les contrats MWS (systÃ¨mes passifs et actifs), **signaler** ou **dÃ©grader** lâ€™Ã©metteur (listes de statuts). Les COGs Tracker peuvent exiger la conformitÃ© Ã  cette norme pour accepter ou relayer les annonces.

---

## 4. SystÃ¨me de sÃ©curitÃ© du Webway

### 4.1 Liste de COGs avec statuts (Webway COG List)

Chaque COG participant (et en particulier chaque COG Tracker) maintient une **liste de COGs** avec un **statut** associÃ© Ã  chaque entrÃ©e. Cette liste permet d'analyser et, le cas Ã©chÃ©ant, de rejeter un COG ou une connexion considÃ©rÃ©e comme malveillante ou non fiable.

**Contenu minimal d'une entrÃ©e (orientation) :**

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant du COG (ex. LSI ou Ã©quivalent attestÃ©) |
| `status` | Statut (voir 4.2) |
| `source` | Origine de l'information (quel COG / Tracker a fourni ou mis Ã  jour le statut) |
| `updated_at` | DerniÃ¨re mise Ã  jour du statut (trace only) |
| DonnÃ©es optionnelles | Adresse de contact, version Core, selon politique locale |

### 4.2 Statuts de COG (orientation)

Les statuts permettent d'exprimer le niveau de confiance ou de dÃ©fiance Ã  l'Ã©gard d'un COG dans le cadre du Webway (prÃ©sence et dÃ©couverte), **sans prÃ©juger** du Visa de Connexion ou accord d'hÃ´te qui sera accordÃ© ou refusÃ© par un COG HÃ©bergeur lors d'une visite.

| Statut | Signification | Usage typique |
|--------|---------------|---------------|
| **Trusted** | COG considÃ©rÃ© comme fiable pour la prÃ©sence / dÃ©couverte | Annonces acceptÃ©es, relayÃ©es |
| **Neutral** | Aucun signal positif ou nÃ©gatif | TraitÃ© par dÃ©faut selon politique locale |
| **Under review** | En cours d'analyse (comportement suspect, signalement) | Limitation ou surveillance des annonces/connexions |
| **Distrusted** | COG considÃ©rÃ© comme non fiable (pas nÃ©cessairement malveillant) | Annonces ou connexions dÃ©gradÃ©es / filtrÃ©es |
| **Rejected** | COG ou connexion rejetÃ©e (malveillant ou politique locale) | Refus d'annonce, blocage de connexion Webway |

Les valeurs exactes et la sÃ©mantique opÃ©rationnelle peuvent Ãªtre prÃ©cisÃ©es dans un protocole ou un contrat MWS dÃ©diÃ©.

### 4.3 Ã‰change de listes entre COGs

Les COGs participants **se transfÃ¨rent** (selon le protocole MWS) des **listes ou des mises Ã  jour de statuts** de COGs, afin de :

- **Analyser** : agrÃ©ger des signaux pour dÃ©cider du statut Ã  attribuer Ã  un COG (comportement, cohÃ©rence, signalements)
- **Rejeter** : refuser d'accepter ou de relayer les annonces d'un COG, ou de traiter des connexions provenant de celui-ci, lorsqu'il est marquÃ© Rejected ou Distrusted

**RÃ¨gles :**
- L'Ã©change de listes ne contient **pas de donnÃ©es mÃ©tier** ni de secrets de gouvernance ; uniquement des identifiants COG et des statuts (et mÃ©tadonnÃ©es de traÃ§abilitÃ©).
- Chaque COG reste **souverain** : il peut ignorer un statut fourni par un autre et appliquer sa propre politique (analyser, rejeter, ou accepter).
- Les COGs Tracker, du fait de leur devoir de protection du rÃ©seau, ont un rÃ´le central dans l'agrÃ©gation et la diffusion de ces signaux (voir section 5).

---

## 5. Devoir des COGs Tracker : protection du rÃ©seau

Les COGs Tracker ont le **devoir de protÃ©ger le rÃ©seau** par des systÃ¨mes **passifs** et **actifs**. Les spÃ©cifications dÃ©taillÃ©es sont formalisÃ©es dans les contrats dÃ©diÃ©s : [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md). Ce document en fixe le cadre conceptuel.

### 5.1 SystÃ¨mes passifs

**DÃ©finition (orientation) :** mÃ©canismes qui **observent, enregistrent et signalent** sans modifier le comportement des connexions ou des annonces de maniÃ¨re proactive.

**Exemples de directions (non exhaustifs) :**
- Observation et journalisation des annonces et des requÃªtes de dÃ©couverte (traÃ§abilitÃ©, dÃ©tection d'anomalies a posteriori)
- Mise Ã  jour et partage des listes de COGs avec statuts (Trusted, Neutral, Under review, Distrusted, Rejected)
- Signalement vers d'autres Trackers ou COGs (rÃ©putation, alertes)
- VÃ©rification de cohÃ©rence des annonces (identitÃ©, adresse, format) sans bloquer a priori

**Principe :** le passif **informe** et **alimente** la dÃ©cision ; il ne coupe pas ni ne modifie le flux par lui-mÃªme (la dÃ©cision de rejet reste locale ou dÃ©lÃ©guÃ©e selon contrat).

### 5.2 SystÃ¨mes actifs

**DÃ©finition (orientation) :** mÃ©canismes qui **agissent sur les flux** du Webway (annonces, requÃªtes, connexions) pour **filtrer, dÃ©grader ou bloquer** en fonction des listes de statuts et des politiques.

**Exemples de directions (non exhaustifs) :**
- Refus de relayer ou d'enregistrer les annonces des COGs en statut Rejected (ou Distrusted, selon politique)
- Refus ou limitation des requÃªtes de dÃ©couverte provenant de COGs ou d'adresses marquÃ©es comme malveillantes
- Throttling ou dÃ©gradation des rÃ©ponses pour les COGs Under review
- Blacklist locale ou partagÃ©e (selon protocole) pour adresses ou COGs Rejected

**Principe :** l'actif **protÃ¨ge** le maillage en appliquant des dÃ©cisions (rejet, dÃ©gradation) conformes au devoir de protection des Trackers et aux contrats MWS.

### 5.3 SynthÃ¨se

| Type | RÃ´le | Statut |
|------|------|--------|
| **Passif** | Validation, observation/filtrage, journalisation ; alimenter les listes de statuts et le signalement | DÃ©fini en 5.1 ; contrat dÃ©diÃ© pour spÃ©cifications dÃ©taillÃ©es |
| **Actif** | Filtrer, dÃ©grader, rejeter (annonces/connexions) | DÃ©fini en 5.2 ; contrat dÃ©diÃ© : [Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) |

Les spÃ©cifications dÃ©taillÃ©es (protocoles, formats, responsabilitÃ©s prÃ©cises des Trackers) sont ou seront dÃ©finies dans des documents de contrat ou de protocole MWS dÃ©diÃ©s (voir rÃ©fÃ©rences en 5.1 et 5.2).

---

## 6. Relation avec la Connexion Inter-COG

Le MWS **ne remplace pas** la visite gouvernÃ©e ; il la **prÃ©cÃ¨de** et la **rend possible** en environnement connectÃ©.

| Ã‰tape | Couche | RÃ´le |
|-------|--------|------|
| 1 | **MWS** | DÃ©couverte : savoir quels COGs sont prÃ©sents et oÃ¹ contacter le Bridge du COG HÃ©bergeur |
| 2 | **Connexion Inter-COG** | PrÃ©-validation locale (COG Origine), Ã©mission du Passeport Utilisateur |
| 3 | **Connexion Inter-COG** | PrÃ©sentation au Bridge (Passeport + Demande de Visite) |
| 4 | **Connexion Inter-COG** | Douane du Host COG, Ã©mission du Visa de Connexion / accord d'hÃ´te, session gouvernÃ©e |

Sans le Webway, un COG peut toujours recevoir une visite si son adresse est connue par d'autres moyens (config manuelle, autre mÃ©canisme). Le MWS **normalise la prÃ©sence** et **facilite** l'Ã©change entre services d'environnements diffÃ©rents (ex. lobby de jeu en ligne pour rejoindre une partie).

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Connexion Inter-COG](_index.md)

---

## 7. Principes non nÃ©gociables

| Principe | Application au MWS |
|----------|---------------------|
| **Le maillage ne fait pas confiance** | Il transporte/expose des informations de prÃ©sence ; aucune dÃ©cision d'accÃ¨s (Permis de circulation, Visa de Connexion) n'est prise par le Webway pour le mÃ©tier ; le Webway dÃ©livre le Permis de circulation (relay) et contrÃ´le l'accÃ¨s au maillage (contrÃ´le tracker) |
| **Aucun core partagÃ©** | La prÃ©sence ne donne aucun accÃ¨s aux Cores ; elle indique oÃ¹ initier une visite |
| **Une seule gouvernance active** | Le COG HÃ©bergeur reste l'autoritÃ© pour Visa de Connexion / accord d'hÃ´te, refus, rÃ©vocation ; Origin/relays pour Permis de circulation |
| **Optionnel** | Environnements offline ou refusant la dÃ©couverte : pas de dÃ©pendance critique (LOI-1, LOI-2) |
| **FÃ©dÃ©ration** | LOI-6 : l'autonomie n'empÃªche pas la fÃ©dÃ©ration ; le MWS est un moyen de fÃ©dÃ©ration sans transfert de donnÃ©es mÃ©tier |
| **Protection du rÃ©seau** | Les COGs Tracker ont le devoir de protÃ©ger le maillage par des mÃ©canismes passifs (5.1) et actifs (5.2 : blocage, signalement, dÃ©gradation, alerte) |

---

## 8. Positionnement dans l'architecture

- **Border Guard** : peut dÃ©finir les rÃ¨gles de qui est autorisÃ© Ã  s'annoncer ou Ã  interroger le maillage (politique locale) et d'utilisation des listes de statuts.
- **Bridge inter-COG** : une fois l'adresse connue via le MWS, le Bridge reste le canal diplomatique ; le MWS ne remplace pas le Bridge.
- **WorrySentinel** : peut Ãªtre sollicitÃ© pour surveiller les signaux issus du Webway (statuts, alertes) dans le cadre de la gouvernance locale.

Le MWS est une **couche de dÃ©couverte et de prÃ©sence** sous le contrÃ´le des Cores existants ; il n'introduit pas de nouveau Core mÃ©tier.

---

## 9. Ã‰volutions futures

- [ ] Formaliser le **protocole MWS** (formats d'annonce, de requÃªte, de liste de statuts)
- [x] **Norme de dÃ©claration sÃ©curisÃ©e** formalisÃ©e (schÃ©ma, signature, vÃ©rification) pour les annonces de services, adresses (IP/ports) et sessions hÃ©bergÃ©es â€” section 3.3 ; cadre dÃ©taillÃ© dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [x] DÃ©finir le **cadre conceptuel des systÃ¨mes passifs** des COGs Tracker (section 5.1 ; contrat dÃ©diÃ© : [Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md))
- [x] DÃ©finir les **contrats des systÃ¨mes actifs** des COGs Tracker â€” [Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)
- [ ] SpÃ©cifier la **matrice des statuts** et les rÃ¨gles d'Ã©change entre COGs
- [ ] IntÃ©grer le MWS dans la section Â« Ã‰volutions futures Â» de la Connexion Inter-COG comme couche de prÃ©sence

---

## RÃ©fÃ©rences croisÃ©es

- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) â€” annexe conceptuelle (normes, formats, protocole, matrice des statuts)
- [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) â€” annexe conceptuelle (Outils, Kits d'Outils, OpÃ©rateurs MWS)
- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) â€” architecture du relay de transport (tunnel Ã©tendu multi-tenant)
- [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) â€” protocole relay (messages, handshake, TLS)
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) â€” guide de dÃ©ploiement du relay (VM, TLS, systemd, tests)
- [MiyuWebwayTracker - Passive Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) â€” contrat des systÃ¨mes passifs (validation, journalisation, signalement)
- [MiyuWebwayTracker - Active Systems Contract](..//..//tools//MiyuWebwayTracker//contracts//security//MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) â€” contrat des systÃ¨mes actifs (blocage, signalement, dÃ©gradation, alerte)
- [Connexion Inter-COG](_index.md)
- [Definition COG](_index.md)
- [Souverainete Environnement](_index.md)
- [Lois Autonomie Systeme](_index.md)
- [Doctrine Securite Fondamentale](_index.md)
- [Glossaire](_index.md) (Passeport Utilisateur, Visa de Connexion, Bridge inter-COG, COG HÃ©bergeur, COG Origine)

---

*Document crÃ©Ã© le 30/01/2026*  
*Classification : Reference conceptuelle â€” Miyukini Webway System (MWS)*


