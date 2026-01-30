# Miyukini Conceptual References - Miyukini Webway System

## Contexte

Ce document définit le **Miyukini Webway System (MWS)** : la couche de **présence et de découverte** des environnements COG disposant d'un accès réseau. Le MWS permet aux COGs de se déclarer, de savoir qui est présent sur le maillage, et de faciliter l'initiation des visites gouvernées (Passeport, Visa) sans transférer de données métier. Il inclut un système de sécurité fondé sur l'échange de listes de COGs avec statuts, et impose aux COGs Tracker un devoir de protection du réseau par des mécanismes passifs et actifs.

**Principe fondamental :**

> **Le Webway normalise la présence et facilite l'échange entre environnements ; il ne transporte pas la gouvernance ni les données — il permet de savoir où et comment initier une visite gouvernée.**

## Portée / Scope

- Définition du Miyukini Webway System (MWS) et de son rôle
- Acteurs : COG participant, COG Tracker
- **Annonces de présence** : services exposés, adresses (IP et ports) associées, déclaration d'hébergement de session (Host)
- **Norme de déclaration sécurisée** : à créer et appliquer pour les annonces de services, adresses et sessions hébergées
- Système de sécurité : listes de COGs avec statuts, échange et analyse pour rejet de COGs ou connexions malveillantes
- Devoir des COGs Tracker : protection du réseau (systèmes passifs et actifs — à créer)
- Relation avec la Connexion Inter-COG (Passeport, Visa, Bridge)
- Principes non négociables et compatibilité avec les Lois d'Autonomie

Ce document **ne couvre pas** :
- Le détail des **normes et standards** du MWS (formats, protocole, matrice des statuts, conformité Trackers) → voir [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) (document conceptuel annexe)
- Les **Outils et Opérateurs** nécessaires au MWS (Strate 6 et 7) → voir [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) (document conceptuel annexe)
- Le détail des protocoles de visite gouvernée → voir [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- Les spécifications techniques des systèmes passifs/actifs des Trackers (à définir dans des contrats dédiés)

---

## 1. Vue d'ensemble du Miyukini Webway System

### 1.1 Rôle du MWS

Le **Miyukini Webway System (MWS)** est la couche qui permet aux environnements COG ayant accès au réseau de :

| Capacité | Description |
|----------|-------------|
| **Se déclarer** | Annoncer sa présence (identité de COG, adresse de contact / Bridge) |
| **Découvrir** | Savoir quels COGs sont présents et où les joindre |
| **Faciliter l'échange** | Donner le point d'entrée pour initier une visite gouvernée (Passeport → Bridge → Visa) |

**Le MWS ne sert pas à transférer des données métier.** Il est la transcription concrète des concepts de présence autour des Passeports et des Visas : il normalise *qui est là* et *où se présenter* pour demander un Visa.

**Analogie (orientation)** : à la manière d'un réseau de type BitTorrent, les COGs peuvent s'annoncer et interroger des **Trackers** (COGs qui acceptent le rôle de point de rendez-vous pour la découverte) ; le transfert réel et la gouvernance restent dans le cadre de la visite gouvernée (Bridge, Visa).

### 1.2 Principes cardinaux

> **Le maillage ne fait pas confiance — il transporte et expose des informations de présence.**
> **La gouvernance (Passeport, Visa) reste souveraine ; le Webway ne gouverne pas.**

- **Optionnel** : les environnements sans réseau ou qui refusent la découverte restent souverains (LOI-2, LOI-6).
- **Aucun core partagé** : la présence ne donne aucun accès aux Cores ; elle indique où aller pour initier une visite.
- **Une seule gouvernance active** : c'est toujours le COG Hébergeur qui décide (Visa, refus, révocation).

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
- Ne pas détenir de données métier ni gouverner les accès ; la délivrance des Visas reste du ressort de chaque COG Hébergeur

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

Cette déclaration permet aux autres COGs (ou aux Utilisateurs Visiteurs via leur COG Origine) de **découvrir** qu'une session est ouverte et d'où s'y connecter (adresse et port du Host), puis d'initier la visite gouvernée (Passeport → Bridge → Visa).

**Contenu minimal (orientation) :**
- Identifiant du service (ou type de session)
- Identifiant du COG Hébergeur
- Adresse de connexion (IP ou nom de domaine, port)
- Éventuellement : capacité restante, niveau de sécurité proposé, selon protocole

**Règle :** la déclaration d'hébergement de session **ne donne aucun droit d'accès** ; elle indique seulement où se présenter pour demander un Visa. L'accès reste gouverné par le COG Hébergeur (Douane, Visa, révocation).

### 3.3 Norme de déclaration sécurisée (à créer et appliquer)

Pour les annonces de **services**, d'**adresses** (IP/ports) et de **sessions hébergées**, une **norme de déclaration sécurisée** doit être **créée et appliquée**. Elle vise à :

- **Authentifier** l'origine des déclarations (COG attesté, non usurpation)
- **Intégrité** : garantir que les déclarations n'ont pas été altérées en transit
- **Format unifié** : permettre l'interopérabilité et la vérification par les Trackers et les participants
- **Limiter les abus** : déclarations conformes, sans exposition de données sensibles ni de gouvernance

**Statut :** cette norme est **à créer** dans un protocole ou un contrat MWS dédié, puis **à appliquer** par tous les COGs participants qui annoncent des services ou des sessions hébergées. Les COGs Tracker peuvent exiger la conformité à cette norme pour accepter ou relayer les annonces (systèmes passifs et actifs).

---

## 4. Système de sécurité du Webway

### 4.1 Liste de COGs avec statuts (Webway COG List)

Chaque COG participant (et en particulier chaque COG Tracker) maintient une **liste de COGs** avec un **statut** associé à chaque entrée. Cette liste permet d'analyser et, le cas échéant, de rejeter un COG ou une connexion considérée comme malveillante ou non fiable.

**Contenu minimal d'une entrée (orientation) :**

| Champ | Description |
|-------|-------------|
| `cog_id` | Identifiant du COG (ex. LSI ou équivalent attesté) |
| `status` | Statut (voir 3.2) |
| `source` | Origine de l'information (quel COG / Tracker a fourni ou mis à jour le statut) |
| `updated_at` | Dernière mise à jour du statut (trace only) |
| Données optionnelles | Adresse de contact, version Core, selon politique locale |

### 4.2 Statuts de COG (orientation)

Les statuts permettent d'exprimer le niveau de confiance ou de défiance à l'égard d'un COG dans le cadre du Webway (présence et découverte), **sans préjuger** du Visa qui sera accordé ou refusé par un COG Hébergeur lors d'une visite.

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

Les COGs Tracker ont le **devoir de protéger le réseau** par des systèmes **passifs** et **actifs**. Les mécanismes détaillés sont **à créer** dans des contrats ou protocoles dédiés ; ce document en fixe le cadre conceptuel.

### 5.1 Systèmes passifs (à créer)

**Définition (orientation) :** mécanismes qui **observent, enregistrent et signalent** sans modifier le comportement des connexions ou des annonces de manière proactive.

**Exemples de directions (non exhaustifs) :**
- Observation et journalisation des annonces et des requêtes de découverte (traçabilité, détection d'anomalies a posteriori)
- Mise à jour et partage des listes de COGs avec statuts (Trusted, Neutral, Under review, Distrusted, Rejected)
- Signalement vers d'autres Trackers ou COGs (réputation, alertes)
- Vérification de cohérence des annonces (identité, adresse, format) sans bloquer a priori

**Principe :** le passif **informe** et **alimente** la décision ; il ne coupe pas ni ne modifie le flux par lui-même (la décision de rejet reste locale ou déléguée selon contrat).

### 5.2 Systèmes actifs (à créer)

**Définition (orientation) :** mécanismes qui **agissent sur les flux** du Webway (annonces, requêtes, connexions) pour **filtrer, dégrader ou bloquer** en fonction des listes de statuts et des politiques.

**Exemples de directions (non exhaustifs) :**
- Refus de relayer ou d'enregistrer les annonces des COGs en statut Rejected (ou Distrusted, selon politique)
- Refus ou limitation des requêtes de découverte provenant de COGs ou d'adresses marquées comme malveillantes
- Throttling ou dégradation des réponses pour les COGs Under review
- Blacklist locale ou partagée (selon protocole) pour adresses ou COGs Rejected

**Principe :** l'actif **protège** le maillage en appliquant des décisions (rejet, dégradation) conformes au devoir de protection des Trackers et aux contrats MWS.

### 4.3 Synthèse

| Type | Rôle | Statut |
|------|------|--------|
| **Passif** | Observer, signaler, alimenter les listes de statuts | Mécanismes à créer (contrats dédiés) |
| **Actif** | Filtrer, dégrader, rejeter (annonces/connexions) | Mécanismes à créer (contrats dédiés) |

Les spécifications détaillées (protocoles, formats, responsabilités précises des Trackers) seront définies dans des documents de contrat ou de protocole MWS dédiés.

---

## 6. Relation avec la Connexion Inter-COG

Le MWS **ne remplace pas** la visite gouvernée ; il la **précède** et la **rend possible** en environnement connecté.

| Étape | Couche | Rôle |
|-------|--------|------|
| 1 | **MWS** | Découverte : savoir quels COGs sont présents et où contacter le Bridge du COG Hébergeur |
| 2 | **Connexion Inter-COG** | Pré-validation locale (COG Origine), émission du Passeport Utilisateur |
| 3 | **Connexion Inter-COG** | Présentation au Bridge (Passeport + Demande de Visite) |
| 4 | **Connexion Inter-COG** | Douane du Host COG, émission du Visa, session gouvernée |

Sans le Webway, un COG peut toujours recevoir une visite si son adresse est connue par d'autres moyens (config manuelle, autre mécanisme). Le MWS **normalise la présence** et **facilite** l'échange entre services d'environnements différents (ex. lobby de jeu en ligne pour rejoindre une partie).

**Référence :** [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)

---

## 7. Principes non négociables

| Principe | Application au MWS |
|----------|---------------------|
| **Le maillage ne fait pas confiance** | Il transporte/expose des informations de présence ; aucune décision d'accès (Visa) n'est prise par le Webway |
| **Aucun core partagé** | La présence ne donne aucun accès aux Cores ; elle indique où initier une visite |
| **Une seule gouvernance active** | Le COG Hébergeur reste l'autorité pour Visa, refus, révocation |
| **Optionnel** | Environnements offline ou refusant la découverte : pas de dépendance critique (LOI-1, LOI-2) |
| **Fédération** | LOI-6 : l'autonomie n'empêche pas la fédération ; le MWS est un moyen de fédération sans transfert de données métier |
| **Protection du réseau** | Les COGs Tracker ont le devoir de protéger le maillage par des mécanismes passifs et actifs (à créer) |

---

## 8. Positionnement dans l'architecture

- **Border Guard** : peut définir les règles de qui est autorisé à s'annoncer ou à interroger le maillage (politique locale) et d'utilisation des listes de statuts.
- **Bridge inter-COG** : une fois l'adresse connue via le MWS, le Bridge reste le canal diplomatique ; le MWS ne remplace pas le Bridge.
- **WorrySentinel** : peut être sollicité pour surveiller les signaux issus du Webway (statuts, alertes) dans le cadre de la gouvernance locale.

Le MWS est une **couche de découverte et de présence** sous le contrôle des Cores existants ; il n'introduit pas de nouveau Core métier.

---

## 9. Évolutions futures

- [ ] Formaliser le **protocole MWS** (formats d'annonce, de requête, de liste de statuts)
- [ ] **Créer et appliquer la norme de déclaration sécurisée** pour les annonces de services, adresses (IP/ports) et sessions hébergées (voir section 3.3) — cadre détaillé dans [MWS Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md)
- [ ] Définir les **contrats des systèmes passifs** des COGs Tracker
- [ ] Définir les **contrats des systèmes actifs** des COGs Tracker
- [ ] Spécifier la **matrice des statuts** et les règles d'échange entre COGs
- [ ] Intégrer le MWS dans la section « Évolutions futures » de la Connexion Inter-COG comme couche de présence

---

## Références croisées

- [Miyukini Webway System - Normes et Standards](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Normes%20et%20Standards.md) — annexe conceptuelle (normes, formats, protocole, matrice des statuts)
- [Miyukini Webway System - Outils et Operateurs](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System%20Outils%20et%20Operateurs.md) — annexe conceptuelle (Outils, Kits d'Outils, Opérateurs MWS)
- [Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)
- [Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)
- [Souverainete Environnement](./Miyukini%20Conceptual%20References%20-%20Souverainete%20Environnement.md)
- [Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- [Doctrine Securite Fondamentale](./Miyukini%20Conceptual%20References%20-%20Doctrine%20Securite%20Fondamentale.md)
- [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Passeport Utilisateur, Visa de Connexion, Bridge inter-COG, COG Hébergeur, COG Origine)

---

*Document créé le 30/01/2026*  
*Classification : Reference conceptuelle — Miyukini Webway System (MWS)*
