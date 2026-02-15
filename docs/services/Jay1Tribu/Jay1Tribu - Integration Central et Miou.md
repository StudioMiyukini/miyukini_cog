# Jay1Tribu — Intégration Central et Miou

## Contexte

**Miyukini Central** est le hub de gestion des Services ; **Miou** est l'avatar/mascotte des COGs et s'appuie sur le contexte applicatif (présence amis, services ouverts, etc.) pour adapter ses bulles et suggestions. Ce document décrit le **contrat d'intégration** entre Jay1Tribu et Central / Miou : ce que Jay1Tribu expose, comment Central et Miou l'utilisent, et les règles de dégradation gracieuse.

## Portée / Scope

- **Applicable à :** Intégration du service Jay1Tribu dans Miyukini Central et consommation par Miou (bulles, notifications ami).
- **Audience :** Développeurs Central, équipes Miou, développeurs Jay1Tribu.
- **Statut :** Spécification d'intégration de référence.

---

## 1. Intégration avec Miyukini Central

### 1.1 Affichage et ouverture du service

| Point d'intégration | Comportement |
|---------------------|--------------|
| **Liste des services** | Jay1Tribu apparaît dans la liste des services (Salon / Bibliothèque) de Miyukini Central comme tout autre service. |
| **Ouverture** | L'utilisateur peut ouvrir Jay1Tribu depuis Central ; l'interface du service (tribus, salons, amis, messages) s'affiche dans le cadre prévu par Central. |
| **Pas de lecture de contenu** | Central n'accède pas au contenu des messages pour affichage ou analyse ; il fournit le cadre (navigation, onglets) et délègue l'affichage du contenu à Jay1Tribu. |

### 1.2 Actions depuis Central

- **Ouvrir Jay1Tribu** : action standard « Ouvrir un service » (ex. bouton ou lien « Voir Jay1Tribu » dans une bulle Miou).
- **Détection d'onglet actif** : si l'utilisateur a déjà l'onglet ou l'écran Jay1Tribu ouvert (conversation en cours), Central peut le signaler à Miou pour éviter des notifications redondantes (ex. « Un ami est en ligne » alors que l'utilisateur est déjà dans Jay1Tribu).

---

## 2. Capacités exposées à Miou (contrat)

Miou a besoin de **métadonnées** pour adapter ses bulles (accueil, rappels, notifications ami). Jay1Tribu expose les capacités suivantes **sans jamais exposer le contenu des messages**.

### 2.1 Liste d'amis (optionnel)

| Capacité | Signature conceptuelle | Description |
|----------|------------------------|-------------|
| **get_friends_list** | `get_friends_list(profile_id) -> Vec<Friend>` | Retourne la liste d'amis avec métadonnées (identifiant, pseudo si résolu). Pas le contenu des conversations. |

**Usage par Miou :** Connaître le cercle d'amis pour des bulles du type « Pense à reprendre contact » ou « Ton réseau grandit » (gamification). La résolution des pseudos est du ressort de Jay1Tribu (ou du service de contacts).

### 2.2 Amis en ligne / présence

| Capacité | Signature conceptuelle | Description |
|----------|------------------------|-------------|
| **get_online_friends** | `get_online_friends(profile_id) -> Vec<Friend>` ou équivalent | Retourne les amis actuellement en ligne (présence fournie par le MWS). |

**Usage par Miou :** Déclencher une bulle de type « Un ami est en ligne » ou « Un ami t'attend peut-être » avec des actions « Ouvrir Jay1Tribu » / « Plus tard ». Voir [Bot - Catalogue Complet des Triggers](../MiyukiniCentral/Miou/Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md) (E-31, notification ami).

### 2.3 Événements (optionnel)

| Événement | Description | Usage possible par Miou |
|-----------|-------------|--------------------------|
| **friend_came_online** | Un ami de la liste vient de passer en ligne. | Déclencher la bulle « Un ami est en ligne » (si l'utilisateur n'est pas déjà dans Jay1Tribu). |
| **conversation_opened** / **conversation_closed** | Ouverture / fermeture d'une conversation. | MiyukiniWatch peut enregistrer des métriques (A-01, A-03) ; Miou peut éviter une notification ami redondante si conversation déjà ouverte. |

**Règle :** Miou et MiyukiniWatch ne reçoivent jamais le contenu des messages ; uniquement des métadonnées (présence, ouverture/fermeture de conversation, identifiants techniques).

---

## 3. Règles d'intégration

| # | Règle | Description |
|---|-------|-------------|
| **INT-01** | Central n'accède pas au contenu | Miyukini Central n'utilise pas le contenu des messages pour affichage ou analyse. |
| **INT-02** | MiyukiniWatch ne lit pas le contenu | Les métriques (conversation ouverte/fermée, ami contacté) sont des métadonnées uniquement. |
| **INT-03** | Résolution des pseudos par Jay1Tribu | L'identifiant technique (`friend_cog_id`) est résolu en pseudo lisible par le service de contacts (Jay1Tribu), pas par MiyukiniWatch ni par Miou directement. |
| **INT-04** | Contrat versionné | Le contrat (get_friends_list, get_online_friends, événements) est versionné avec l'environnement COG (LOI-7). |
| **INT-05** | Dégradation gracieuse | Si Jay1Tribu est indisponible (non installé, non démarré, erreur), Central et Miou continuent de fonctionner : pas de crash. Comportement attendu : liste d'amis vide ou absente, pas de notification ami, pas d'erreur bloquante. |

---

## 4. Dégradation gracieuse (détail)

| Situation | Comportement attendu |
|-----------|----------------------|
| **Jay1Tribu non installé** | Le service n'apparaît pas ou apparaît comme indisponible ; Miou n'a pas de donnée ami ; pas de crash. |
| **Jay1Tribu indisponible (timeout, erreur)** | `get_online_friends` / `get_friends_list` renvoient une liste vide ou une erreur gérée ; Miou utilise des valeurs par défaut (ex. `ami_connecte_recemment = None`, `amis = []`). |
| **Utilisateur déjà dans Jay1Tribu (conversation ouverte)** | Miou peut ne pas afficher la notification « Un ami est en ligne » pour éviter redondance (trigger X-06). |

---

## 5. Références croisées

| Document | Section pertinente |
|----------|--------------------|
| [Miou - Guide Implementation Complet](../MiyukiniCentral/Miou/Miou%20-%20Guide%20Implementation%20Complet.md) | Tableau des services (Jay1Tribu : get_online_friends, get_friends_list) ; dégradation si Jay1Tribu indisponible. |
| [Bot - Integration et Flux de Donnees](../MiyukiniCentral/Miou/Bot/Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) | Contexte applicatif Jay1Tribu ; chargement ami en ligne ; comportement si Jay1Tribu indisponible. |
| [MiyukiniWatch - Specification Fonctionnelle Metriques](../MiyukiniWatch/MiyukiniWatch%20-%20Specification%20Fonctionnelle%20Metriques%20et%20Collecte.md) | Métriques conversation_opened, conversation_closed ; ami contacté ; pas de lecture du contenu. |
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | INT-01 à INT-05. |

---

## 6. Résumé

- **Central** : affiche et ouvre Jay1Tribu ; ne lit pas le contenu des messages.
- **Miou** : consomme `get_online_friends` et éventuellement `get_friends_list` pour bulles et notifications ami ; résolution des pseudos par Jay1Tribu.
- **MiyukiniWatch** : enregistre des métadonnées (conversation ouverte/fermée, ami contacté) ; ne lit jamais le contenu.
- **Dégradation** : si Jay1Tribu est indisponible, pas de crash ; listes vides, pas de notification ami.

---

**Document** : Jay1Tribu — Intégration Central et Miou  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Spécification d'intégration de référence
