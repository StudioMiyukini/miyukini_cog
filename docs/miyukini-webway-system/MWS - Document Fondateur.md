# MWS — Document Fondateur

## Contexte

Le **Miyukini Webway System (MWS)** est la couche de **présence, de découverte et de transport** des environnements COG disposant d'un accès réseau. Il permet aux COGs de se déclarer, de découvrir qui est présent sur le maillage, d'exposer des surfaces de connexion (Lobbys), et de faciliter l'initiation des visites gouvernées (Passeport, Visa) sans transférer les données métier.

**Positionnement exceptionnel :**

- Le MWS est **trop vaste pour être contenu dans une seule strate** de la pyramide Miyukini.
- En raison de ce **caractère exceptionnel**, sa **racine documentaire** est établie dans **`docs/miyukini-webway-system`**.
- Le MWS est considéré comme un **système complet** dans l'environnement Miyukini, **uniquement subordonné aux Cores**.
- Il est **consommé par toutes les strates**, à différents endroits (Outils, Opérateurs, Services, Cores), selon des besoins distincts.

**Principe fondamental :**

> **Le Webway normalise la présence et facilite l'échange entre environnements ; il ne transporte pas la gouvernance ni les données métier — il permet de savoir où et comment initier une visite gouvernée.**

## Portée / Scope

- **Définition** du MWS comme système complet et subordonné aux Cores uniquement.
- **Racine documentaire** : toute la documentation MWS part d'ici ; les documents détaillés (normes, protocoles, relay, trackers) sont référencés depuis cette racine.
- **Acteurs** : Origin, relays, trackers, COG participant, COG Tracker ; rôles et responsabilités.
- **Consommation par les strates** : comment chaque strate (Cores, Outils, Opérateurs, Services) consomme le MWS.
- **Principes non négociables** et compatibilité avec les Lois d'Autonomie (LOI-2, LOI-6, LOI-7, LOI-8).

Ce document **ne remplace pas** les spécifications détaillées (normes, formats binaires, contrats passifs/actifs) ; il en est le **point d'entrée** et en fixe le **cadre fondateur**.

---

## 1. Le MWS comme système complet

### 1.1 Un système, pas une strate

Dans l'architecture Miyukini, les **strates** (0 à 9) organisent le Kernel, les Cores, les Outils, les Opérateurs, etc. Le MWS **ne s'identifie pas à une strate unique** : il couvre des capacités qui concernent plusieurs strates (découverte, transport, conformité, catalogue, Lobbys) et dont la cohérence exige une **vision système**.

| Caractéristique | Description |
|-----------------|-------------|
| **Système complet** | Le MWS a un périmètre fonctionnel clair : présence, découverte, transport, vérification de conformité, catalogue et Lobbys, visas de circulation et d'accès. |
| **Subordination unique** | Le MWS est **uniquement subordonné aux Cores**. Il n'est pas subordonné à une strate Outils ou Opérateurs ; les décisions de gouvernance qui le concernent (ex. politique de présence, conformité) relèvent des Cores. |
| **Consommation multi-strates** | **Toutes les strates** peuvent consommer le MWS : Cores (conformité, attestation), Outils (MiyuWebwayTracker, MiyuWebwayParticipant), Opérateurs et Services (annonces, Lobbys, découverte). |

### 1.2 Racine documentaire

La **racine documentaire** du MWS est **`docs/miyukini-webway-system`**. Toute la documentation du système (fondateur, architecture, consommation par les strates, index des références) **commence ici**. Les documents existants dans `docs/reference/`, `docs/tools/`, `docs/setup/` restent les **références détaillées** ; ils sont **liés** depuis cette racine et ne dupliquent pas le positionnement fondateur.

---

## 2. Subordination aux Cores

Le MWS ne remplace pas la gouvernance des Cores ; il la **sert**.

| Principe | Description |
|----------|-------------|
| **Cores décident** | Les politiques de présence, de conformité, d'attestation et de sécurité du Webway sont sous le contrôle des Cores (WorrySentinel, Border Guard, StrongFather, etc.). |
| **MWS exécute** | Le MWS fournit les mécanismes (relays, trackers, protocoles, catalogue, Lobbys) ; les Cores décident qui peut circuler, qui est conforme, qui reçoit un Visa. |
| **Aucune gouvernance métier dans le MWS** | Le Webway ne délivre pas les Visas d'accès aux ressources métier des COGs ; il permet la circulation (Visa de circulation) et la découverte. L'accès aux services exposés (Visa d'accès hôte) reste du ressort du COG Hébergeur. |

---

## 3. Acteurs du MWS

| Acteur | Rôle | Subordination |
|--------|------|---------------|
| **Origin** | Point d'origine du MWS ; fonctions relay + tracker ; source de vérité (Registre, versions, conformité). | Cores / écosystème |
| **Relays** | Duplications d'Origin ; vérification de conformité ; distribution des versions ; délivrance du Visa de circulation. | Origin |
| **Trackers** | Douaniers du réseau ; contrôle d'identité et de Visa ; pools par version ; Lobbys et catalogue ; whitelists/blacklists/quarantaines. | Origin / critères relay |
| **COG participant** | Se déclare, expose ses surfaces (Lobbys), découvre les autres COGs, consomme les services via Visa d'accès hôte. | Cores du COG |
| **COG Tracker** | COG qui endosse le rôle Tracker ; port 21000 ; catalogue et Lobbys (port 80) ; protection du réseau (passif/actif). | Cores du COG + contrats MWS |

---

## 4. Consommation du MWS par les strates

Le MWS est **consommé à différentes parties** par **toutes les strates** :

| Strate / Niveau | Consommation du MWS |
|-----------------|---------------------|
| **Cores** | Attestation d'environnement, conformité, clé Cores, politique de présence ; dialogue avec Origin/relays pour Visa de circulation. |
| **Outils (Strate 6)** | MiyuWebwayTracker, MiyuWebwayParticipant : annonces, découverte, Lobbys, catalogue, contrats passifs/actifs. |
| **Opérateurs / Services (Strate 7+)** | Annonces de services, déclaration de surfaces, exposition de Lobbys, consommation des Lobbys d'autres COGs, favoris, amis. |
| **Interfaces / BondingBrother (Strate 5)** | Pont vers le réseau ; exposition des intentions de visite et réception des réponses de découverte. |

Le détail par strate est décrit dans [MWS - Consommation par les Strates](./strates/MWS%20-%20Consommation%20par%20les%20Strates.md).

---

## 5. Principes cardinaux

- **Le maillage ne fait pas confiance** — il transporte et expose des informations de présence et des chemins.
- **La gouvernance (Passeport, Visa) reste souveraine** ; le Webway ne gouverne pas les accès métier.
- **Optionnel** : les environnements sans réseau ou qui refusent la découverte restent souverains (LOI-2, LOI-6).
- **Aucun core partagé** : la présence ne donne aucun accès aux Cores ; elle indique où aller pour initier une visite.
- **Une seule gouvernance active par ressource** : le COG Hébergeur décide (Visa d'accès, refus, révocation) ; Origin/relays décident du Visa de circulation.

---

## 6. Références détaillées

Les documents suivants (hors racine MWS) constituent la **documentation précise** du système ; ils sont accessibles depuis l’[Index des références MWS](./reference/_index.md) :

- Références conceptuelles (MWS, Normes et Standards, Outils et Opérateurs, Relay, Protocol)
- Outils : MiyuWebwayTracker, MiyuWebwayParticipant (Documentation Fondatrice, contrats, implémentation)
- Setup : Relay Deployment, Oracle Cloud Instance

---

**Version :** 1.0  
**Statut :** Document fondateur — racine documentaire MWS
