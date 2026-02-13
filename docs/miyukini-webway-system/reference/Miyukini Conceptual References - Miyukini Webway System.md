# Miyukini Conceptual References - Miyukini Webway System

> **Racine documentaire MWS :** La documentation officielle du MWS a pour racine **`docs/miyukini-webway-system`**. Pour le document fondateur, l'architecture (subordination aux Cores) et la consommation par les strates, voir [docs/miyukini-webway-system](../miyukini-webway-system/README.md).

## Contexte

Ce document définit le **Miyukini Webway System (MWS)** : la couche de **présence et de découverte** des environnements COG disposant d'un accès réseau. Le MWS permet aux COGs de se déclarer, de savoir qui est présent sur le maillage, et de faciliter l'initiation des visites gouvernées (Passeport, Permis de circulation, Visa de Connexion) sans transférer de données métier. Il inclut un système de sécurité fondé sur l'échange de listes de COGs avec statuts, et impose aux COGs Tracker un devoir de protection du réseau par des mécanismes passifs et actifs.

**Principe fondamental :**

> **Le Webway normalise la présence et facilite l'échange entre environnements ; il ne transporte pas la gouvernance ni les données — il permet de savoir où et comment initier une visite gouvernée.**

## Portée / Scope

- Définition du Miyukini Webway System (MWS) et de son rôle
- Acteurs : COG participant, COG Tracker
- **Annonces de présence** : services exposés, adresses (IP et ports) associées, déclaration d'hébergement de session (Host)
- **Norme de déclaration sécurisée** : schéma commun, signature et vérification pour les annonces de services, adresses et sessions hébergées (section 3.3)
- Système de sécurité : listes de COGs avec statuts, échange et analyse pour rejet de COGs ou connexions malveillantes
- Devoir des COGs Tracker : protection du réseau (systèmes passifs en 5.1 ; systèmes actifs en 5.2 — blocage, signalement, dégradation, alerte)
- Relation avec la Connexion Inter-COG (Passeport, Permis de circulation, Visa de Connexion, Bridge)
- Principes non négociables et compatibilité avec les Lois d'Autonomie

Ce document **ne couvre pas** :
- Le détail des **normes et standards** du MWS (formats, protocole, matrice des statuts, conformité Trackers) → voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (document conceptuel annexe)
- Les **Outils et Opérateurs** nécessaires au MWS (Strate 6 et 7) → voir [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) (document conceptuel annexe)
- Le détail des protocoles de visite gouvernée → voir [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- Les spécifications techniques détaillées des systèmes passifs/actifs (préconditions, postconditions, invariants) figurent dans des contrats dédiés ; le cadre conceptuel des systèmes actifs est défini en section 5.2.

---

## 1. Vue d'ensemble du Miyukini Webway System

### 1.1 Rôle du MWS

Le **Miyukini Webway System (MWS)** est la couche qui permet aux environnements COG ayant accès au réseau de :

| Capacité | Description |
|----------|-------------|
| **Se déclarer** | Annoncer sa présence (identité de COG, adresse de contact / Bridge) |
| **Découvrir** | Savoir quels COGs sont présents et où les joindre |
| **Faciliter l'échange** | Donner le point d'entrée pour initier une visite gouvernée (Passeport → Permis de circulation → Bridge → Visa de Connexion) |

**Le MWS ne sert pas à transférer des données métier.** Il est la transcription concrète des concepts de présence autour des Passeports et des Permis de circulation : il normalise *qui est là* et *où se présenter* pour demander un Permis de circulation (relay) ou un accord d'hôte / Visa de Connexion (COG hôte).

**Analogie (orientation)** : à la manière d'un réseau de type BitTorrent, les COGs peuvent s'annoncer et interroger des **Trackers** (COGs qui acceptent le rôle de point de rendez-vous pour la découverte) ; le transfert réel et la gouvernance restent dans le cadre de la visite gouvernée (Bridge, Visa de Connexion).

### 1.2 Principes cardinaux

> **Le maillage ne fait pas confiance — il transporte et expose des informations de présence.**
> **La gouvernance (Passeport, Permis de circulation, Visa de Connexion) reste souveraine ; le Webway ne gouverne pas.**

- **Optionnel** : les environnements sans réseau ou qui refusent la découverte restent souverains (LOI-2, LOI-6).
- **Aucun core partagé** : la présence ne donne aucun accès aux Cores ; elle indique où aller pour initier une visite.
- **Une seule gouvernance active** : c'est toujours le COG Hébergeur qui décide (Visa de Connexion / accord d'hôte, refus, révocation) ; Origin/relays décident du Permis de circulation.

---

## 2. Acteurs du Webway

### 2.1 COG participant (Webway Participant)

**Définition :** tout COG qui choisit de participer au maillage MWS (accès réseau et déclaration activée).

**Rôle :**
- Se déclarer auprès d'un ou plusieurs COGs Tracker (ou au maillage) selon le protocole MWS
- Exposer les informations minimales de présence (identité COG, adresse du Bridge / point de contact)
- Consulter la présence d'autres COGs pour initier des visites gouvernées
- Participer au système de sécurité en échangeant et en tenant à jour une **liste de COGs avec statuts** (voir section 3)

**Responsabilités :**
- Ne pas exposer de données métier ni de gouvernance via le Webway
- Respecter les règles de sécurité du maillage (listes de statuts, rejet de connexions malveillantes)

### 2.2 COG Tracker (Webway Tracker)

**Définition :** COG dont l'administrateur a choisi d'endosser le rôle de **Tracker** : exposer volontairement une adresse (IP ou nom de domaine) pour participer au maillage et servir de point de rendez-vous pour la découverte.

**Port officiel :** les COGs Tracker MWS exposent leur endpoint sur le **port 21000**. Les COGs participants se connectent aux Trackers sur ce port par défaut. Voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (section 2.7.4).

**Rôle :**
- Tenir un rôle de **point de rendez-vous** pour la découverte (enregistrement des annonces de présence, réponse aux requêtes de découverte)
- **Protéger le réseau** par des mécanismes **passifs** et **actifs** (voir section 4)
- Ne pas détenir de données métier ni gouverner les accès ; la délivrance des Visa de Connexion / accord d'hôte reste du ressort de chaque COG Hébergeur ; la délivrance du Permis de circulation reste du ressort d'Origin/relays

**Devoir fondamental :**

> **Les COGs Tracker ont le devoir de protéger le réseau par des systèmes passifs et actifs.**

Ils ne sont pas de simples annuaires : ils contribuent à la santé et à la sûreté du maillage (détection, signalement, filtrage, selon les mécanismes définis).

---

## 3. Annonces de présence : services, adresses et sessions hébergées

### 3.1 Communication des services et adresses (IP / ports)

Les COGs participants peuvent **communiquer au réseau** les **services** qu'ils exposent et les **adresses** associées : **IP** (ou nom de domaine) et **ports**. Cela permet à d'autres COGs de savoir où et comment initier une visite gouvernée vers un service donné (Bridge, endpoint).

**Règle :** ces informations relèvent de la **présence et de la découverte** ; elles ne contiennent pas de données métier ni de gouvernance. Le détail des champs (identifiant de service, protocole, port, etc.) sera défini dans le protocole MWS et la norme de déclaration sécurisée (voir 3.3).

### 3.2 Déclaration d'hébergement de session (Host)

Dans le cadre d'un **COG Hébergeur** qui propose une **session** (ex. partie de jeu, salle de collaboration, service temporaire), il est nécessaire qu'il **déclare au réseau** :

> **« J'héberge une session de tel service et j'attends qu'on se connecte à moi. »**

Cette déclaration permet aux autres COGs (ou aux Utilisateurs Visiteurs via leur COG Origine) de **découvrir** qu'une session est ouverte et d'où s'y connecter (adresse et port du Host), puis d'initier la visite gouvernée (Passeport → Permis de circulation → Bridge → Visa de Connexion).

**Contenu minimal (orientation) :**
- Identifiant du service (ou type de session)
- Identifiant du COG Hébergeur
- Adresse de connexion (IP ou nom de domaine, port)
- Éventuellement : capacité restante, niveau de sécurité proposé, selon protocole

**Règle :** la déclaration d'hébergement de session **ne donne aucun droit d'accès** ; elle indique seulement où se présenter pour demander un Permis de circulation (relay) ou un Visa de Connexion / accord d'hôte (COG Hébergeur). L'accès reste gouverné par le COG Hébergeur (Douane, Visa de Connexion, révocation).

### 3.3 Norme de déclaration sécurisée

Pour les annonces de **services**, d'**adresses** (IP/ports) et de **sessions hébergées**, une **norme de déclaration sécurisée** est définie et doit être **appliquée** par tous les COGs participants qui annoncent sur le Webway. Elle vise à :

- **Authentifier** l'origine des déclarations (COG attesté, non usurpation)
- **Intégrité** : garantir que les déclarations n'ont pas été altérées en transit
- **Format unifié** : permettre l'interopérabilité et la vérification par les Trackers et les participants
- **Limiter les abus** : déclarations conformes, sans exposition de données sensibles ni de gouvernance

Le détail des formats de messages (annonce de présence, services, session hébergée, requête de découverte, liste de statuts) est défini dans [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (sections 1 et 2). Ce qui suit en fixe le **cadre conceptuel** : schéma commun, signature et vérification.

#### 3.3.1 Schéma commun des déclarations

Toute déclaration MWS (présence, service, session hébergée) conforme à la norme respecte une **structure commune** :

| Élément | Règle |
|--------|--------|
| **Version** | Champ obligatoire `version` (ex. `mws_declaration_v1`) indiquant la version de la norme utilisée. |
| **Type** | Champ obligatoire `type` identifiant le message (`presence_announcement`, `service_announcement`, `host_session_declaration`, etc.). |
| **Identité** | Champ obligatoire identifiant le COG émetteur (`cog_id` ou équivalent attesté). |
| **Horodatage** | Champ obligatoire `issued_at` (ISO 8601) pour la traçabilité et la limitation des rejeux. |
| **Intégrité** | Champ obligatoire `integrity` contenant le mécanisme de vérification (signature ou MAC — voir 3.3.2). |
| **Corps métier** | Champs spécifiques au type (adresses, services, `session_id`, etc.) selon le schéma défini dans Normes et Standards. |

**Sérialisation canonique :** pour que la signature soit reproductible, la déclaration doit être **sérialisée de manière déterministe** (ordre des champs fixe, encodage unique, ex. JSON canonique ou CBOR) avant calcul de la signature. Seul le **contenu signé** (corps de la déclaration sans le champ `integrity`) est inclus dans l’entrée de la fonction de signature.

**Champs interdits :** données utilisateur, secrets, contenu métier, informations permettant d’usurper une gouvernance. La norme restreint les champs autorisés à la présence et à la découverte.

#### 3.3.2 Signature (authentification et intégrité)

- **Responsable de la signature :** le **COG émetteur** (participant ou Hébergeur) signe ses propres déclarations avec une clé ou un secret **associé à son identité** (ex. clé dérivée de l’identité COG, certificat, mécanisme attesté).
- **Périmètre signé :** tout le contenu de la déclaration **à l’exclusion du champ `integrity`** (version, type, cog_id, champs métier, issued_at, etc.), après sérialisation canonique.
- **Mécanisme :** la norme impose un **mécanisme d’intégrité** (signature numérique ou MAC) dont le résultat est placé dans `integrity.value` (ex. encodage base64). Le champ `integrity` peut inclure `method` (ex. `signature`, `mac`), `algorithm` et `key_id` pour permettre au récepteur de choisir la clé ou l’algorithme de vérification.
- **Objectifs :** attester que l’émetteur est bien le COG annoncé et que le message n’a pas été modifié en transit.

#### 3.3.3 Vérification par le récepteur (Tracker ou participant)

Le récepteur (COG Tracker ou autre COG participant) **vérifie** chaque déclaration avant de l’accepter, de la relayer ou de l’exploiter :

1. **Conformité du schéma** : présence et format des champs obligatoires (version, type, cog_id, issued_at, integrity), types et contraintes (ports non exclus, plages de valeurs). Les schémas détaillés sont dans [Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (section 2).
2. **Vérification d’intégrité** : reconstruire le contenu signé en sérialisation canonique, puis vérifier la signature ou le MAC à l’aide de la clé ou du secret associé au `cog_id` (registre local, infrastructure à clés publiques, ou mécanisme attesté selon déploiement).
3. **Cohérence identité–signature** : s’assurer que la clé utilisée pour la vérification est bien liée au COG annoncé dans la déclaration (non usurpation).
4. **Optionnel — limitation des rejeux** : vérifier que `issued_at` est dans une fenêtre temporelle acceptée (ex. pas trop ancien) selon politique locale.

En cas d’échec (schéma invalide, signature invalide, identité incohérente), le récepteur **rejette** la déclaration et peut, selon les contrats MWS (systèmes passifs et actifs), **signaler** ou **dégrader** l’émetteur (listes de statuts). Les COGs Tracker peuvent exiger la conformité à cette norme pour accepter ou relayer les annonces.

---

## 4. Système de sécurité du Webway

### 4.1 Liste de COGs avec statuts (Webway COG List)

Chaque COG participant (et en particulier chaque COG Tracker) maintient une **liste de COGs** avec un **statut** associé à chaque entrée. Cette liste permet d'analyser et, le cas échéant, de rejeter un COG ou une connexion considérée comme malveillante ou non fiable.

**Contenu minimal d'une entrée (orientation) :**

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant du COG (ex. LSI ou équivalent attesté) |
| `status` | Statut (voir 4.2) |
| `source` | Origine de l'information (quel COG / Tracker a fourni ou mis à jour le statut) |
| `updated_at` | Dernière mise à jour du statut (trace only) |
| Données optionnelles | Adresse de contact, version Core, selon politique locale |

### 4.2 Statuts de COG (orientation)

Les statuts permettent d'exprimer le niveau de confiance ou de défiance à l'égard d'un COG dans le cadre du Webway (présence et découverte), **sans préjuger** du Visa de Connexion ou accord d'hôte qui sera accordé ou refusé par un COG Hébergeur lors d'une visite.

| Statut | Signification | Usage typique |
|--------|---------------|---------------|
| **Trusted** | COG considéré comme fiable pour la présence / découverte | Annonces acceptées, relayées |
| **Neutral** | Aucun signal positif ou négatif | Traité par défaut selon politique locale |
| **Under review** | En cours d'analyse (comportement suspect, signalement) | Limitation ou surveillance des annonces/connexions |
| **Distrusted** | COG considéré comme non fiable (pas nécessairement malveillant) | Annonces ou connexions dégradées / filtrées |
| **Rejected** | COG ou connexion rejetée (malveillant ou politique locale) | Refus d'annonce, blocage de connexion Webway |

Les valeurs exactes et la sémantique opérationnelle peuvent être précisées dans un protocole ou un contrat MWS dédié.

### 4.3 Échange de listes entre COGs

Les COGs participants **se transfèrent** (selon le protocole MWS) des **listes ou des mises à jour de statuts** de COGs, afin de :

- **Analyser** : agréger des signaux pour décider du statut à attribuer à un COG (comportement, cohérence, signalements)
- **Rejeter** : refuser d'accepter ou de relayer les annonces d'un COG, ou de traiter des connexions provenant de celui-ci, lorsqu'il est marqué Rejected ou Distrusted

**Règles :**
- L'échange de listes ne contient **pas de données métier** ni de secrets de gouvernance ; uniquement des identifiants COG et des statuts (et métadonnées de traçabilité).
- Chaque COG reste **souverain** : il peut ignorer un statut fourni par un autre et appliquer sa propre politique (analyser, rejeter, ou accepter).
- Les COGs Tracker, du fait de leur devoir de protection du réseau, ont un rôle central dans l'agrégation et la diffusion de ces signaux (voir section 5).

---

## 5. Devoir des COGs Tracker : protection du réseau

Les COGs Tracker ont le **devoir de protéger le réseau** par des systèmes **passifs** et **actifs**. Les spécifications détaillées sont formalisées dans les contrats dédiés : [MiyuWebwayTracker - Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) et [MiyuWebwayTracker - Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md). Ce document en fixe le cadre conceptuel.

### 5.1 Systèmes passifs

**Définition (orientation) :** mécanismes qui **observent, enregistrent et signalent** sans modifier le comportement des connexions ou des annonces de manière proactive.

**Exemples de directions (non exhaustifs) :**
- Observation et journalisation des annonces et des requêtes de découverte (traçabilité, détection d'anomalies a posteriori)
- Mise à jour et partage des listes de COGs avec statuts (Trusted, Neutral, Under review, Distrusted, Rejected)
- Signalement vers d'autres Trackers ou COGs (réputation, alertes)
- Vérification de cohérence des annonces (identité, adresse, format) sans bloquer a priori

**Principe :** le passif **informe** et **alimente** la décision ; il ne coupe pas ni ne modifie le flux par lui-même (la décision de rejet reste locale ou déléguée selon contrat).

### 5.2 Systèmes actifs

**Définition (orientation) :** mécanismes qui **agissent sur les flux** du Webway (annonces, requêtes, connexions) pour **filtrer, dégrader ou bloquer** en fonction des listes de statuts et des politiques.

**Exemples de directions (non exhaustifs) :**
- Refus de relayer ou d'enregistrer les annonces des COGs en statut Rejected (ou Distrusted, selon politique)
- Refus ou limitation des requêtes de découverte provenant de COGs ou d'adresses marquées comme malveillantes
- Throttling ou dégradation des réponses pour les COGs Under review
- Blacklist locale ou partagée (selon protocole) pour adresses ou COGs Rejected

**Principe :** l'actif **protège** le maillage en appliquant des décisions (rejet, dégradation) conformes au devoir de protection des Trackers et aux contrats MWS.

### 5.3 Synthèse

| Type | Rôle | Statut |
|------|------|--------|
| **Passif** | Validation, observation/filtrage, journalisation ; alimenter les listes de statuts et le signalement | Défini en 5.1 ; contrat dédié pour spécifications détaillées |
| **Actif** | Filtrer, dégrader, rejeter (annonces/connexions) | Défini en 5.2 ; contrat dédié : [Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) |

Les spécifications détaillées (protocoles, formats, responsabilités précises des Trackers) sont ou seront définies dans des documents de contrat ou de protocole MWS dédiés (voir références en 5.1 et 5.2).

---

## 6. Relation avec la Connexion Inter-COG

Le MWS **ne remplace pas** la visite gouvernée ; il la **précède** et la **rend possible** en environnement connecté.

| Étape | Couche | Rôle |
|-------|--------|------|
| 1 | **MWS** | Découverte : savoir quels COGs sont présents et où contacter le Bridge du COG Hébergeur |
| 2 | **Connexion Inter-COG** | Pré-validation locale (COG Origine), émission du Passeport Utilisateur |
| 3 | **Connexion Inter-COG** | Présentation au Bridge (Passeport + Demande de Visite) |
| 4 | **Connexion Inter-COG** | Douane du Host COG, émission du Visa de Connexion / accord d'hôte, session gouvernée |

Sans le Webway, un COG peut toujours recevoir une visite si son adresse est connue par d'autres moyens (config manuelle, autre mécanisme). Le MWS **normalise la présence** et **facilite** l'échange entre services d'environnements différents (ex. lobby de jeu en ligne pour rejoindre une partie).

**Référence :** [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)

---

## 7. Principes non négociables

| Principe | Application au MWS |
|----------|---------------------|
| **Le maillage ne fait pas confiance** | Il transporte/expose des informations de présence ; aucune décision d'accès (Permis de circulation, Visa de Connexion) n'est prise par le Webway pour le métier ; le Webway délivre le Permis de circulation (relay) et contrôle l'accès au maillage (contrôle tracker) |
| **Aucun core partagé** | La présence ne donne aucun accès aux Cores ; elle indique où initier une visite |
| **Une seule gouvernance active** | Le COG Hébergeur reste l'autorité pour Visa de Connexion / accord d'hôte, refus, révocation ; Origin/relays pour Permis de circulation |
| **Optionnel** | Environnements offline ou refusant la découverte : pas de dépendance critique (LOI-1, LOI-2) |
| **Fédération** | LOI-6 : l'autonomie n'empêche pas la fédération ; le MWS est un moyen de fédération sans transfert de données métier |
| **Protection du réseau** | Les COGs Tracker ont le devoir de protéger le maillage par des mécanismes passifs (5.1) et actifs (5.2 : blocage, signalement, dégradation, alerte) |

---

## 8. Positionnement dans l'architecture

- **Border Guard** : peut définir les règles de qui est autorisé à s'annoncer ou à interroger le maillage (politique locale) et d'utilisation des listes de statuts.
- **Bridge inter-COG** : une fois l'adresse connue via le MWS, le Bridge reste le canal diplomatique ; le MWS ne remplace pas le Bridge.
- **WorrySentinel** : peut être sollicité pour surveiller les signaux issus du Webway (statuts, alertes) dans le cadre de la gouvernance locale.

Le MWS est une **couche de découverte et de présence** sous le contrôle des Cores existants ; il n'introduit pas de nouveau Core métier.

---

## 9. Évolutions futures

- [ ] Formaliser le **protocole MWS** (formats d'annonce, de requête, de liste de statuts)
- [x] **Norme de déclaration sécurisée** formalisée (schéma, signature, vérification) pour les annonces de services, adresses (IP/ports) et sessions hébergées — section 3.3 ; cadre détaillé dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [x] Définir le **cadre conceptuel des systèmes passifs** des COGs Tracker (section 5.1 ; contrat dédié : [Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md))
- [x] Définir les **contrats des systèmes actifs** des COGs Tracker — [Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md)
- [ ] Spécifier la **matrice des statuts** et les règles d'échange entre COGs
- [ ] Intégrer le MWS dans la section « Évolutions futures » de la Connexion Inter-COG comme couche de présence

---

## Références croisées

- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) — annexe conceptuelle (normes, formats, protocole, matrice des statuts)
- [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) — annexe conceptuelle (Outils, Kits d'Outils, Opérateurs MWS)
- [Miyukini Webway Relay](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) — architecture du relay de transport (tunnel étendu multi-tenant)
- [Miyukini Webway Relay Protocol](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay%20Protocol.md) — protocole relay (messages, handshake, TLS)
- [Miyukini - Webway Relay Deployment Guide](../setup/Miyukini%20-%20Webway%20Relay%20Deployment%20Guide.md) — guide de déploiement du relay (VM, TLS, systemd, tests)
- [MiyuWebwayTracker - Passive Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Passive%20Systems%20Contract.md) — contrat des systèmes passifs (validation, journalisation, signalement)
- [MiyuWebwayTracker - Active Systems Contract](../tools/MiyuWebwayTracker/contracts/security/MiyuWebwayTracker%20-%20Active%20Systems%20Contract.md) — contrat des systèmes actifs (blocage, signalement, dégradation, alerte)
- [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- [Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)
- [Souverainete Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Doctrine Securite Fondamentale](./Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Passeport Utilisateur, Visa de Connexion, Bridge inter-COG, COG Hébergeur, COG Origine)

---

*Document créé le 30/01/2026*  
*Classification : Reference conceptuelle — Miyukini Webway System (MWS)*
