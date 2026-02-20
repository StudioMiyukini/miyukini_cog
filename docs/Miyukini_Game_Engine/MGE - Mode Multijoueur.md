# MGE — Mode Multijoueur

Modes standalone, host authoritative, client replica, snapshot/delta, compatibilité MWS.

## Contexte

Le MGE supporte le multijoueur via un modèle host authoritative : un COG héberge la partie (Lobby), les autres COGs rejoignent en tant que clients. Le transport s'appuie sur le MWS (Miyukini Webway System). Le lockstep n'est pas obligatoire.

## Portée / Scope

- **Applicable à :** Conception du plugin network, intégration MWS, jeux multijoueur (Allumina, etc.).
- **Audience :** Développeurs moteur, développeurs de jeux.
- **Statut :** Spécification normative.

---

## 1. Modes de fonctionnement

### 1.1 Standalone (solo offline)

- Un seul COG ; aucun réseau.
- Simulation locale complète ; sauvegarde via KindMother.
- Conforme LOI-1, LOI-2 : aucune dépendance externe critique.

### 1.2 Host authoritative

- Un COG **héberge** la partie : il est l'autorité sur l'état du monde.
- Les clients envoient des **inputs** (actions joueur) ; l'hôte simule et diffuse l'état.
- L'hôte décide des résultats (combat, loot, collisions).
- Réplication : snapshot ou delta vers les clients.

### 1.3 Client replica

- Un COG **rejoint** une partie hébergée par un autre COG.
- Le client reçoit des snapshots ou deltas et **reproduit** l'état localement.
- Les inputs du joueur sont envoyés à l'hôte ; pas d'exécution locale des actions métier (sauf prédiction optionnelle).
- Rendu local : le client affiche l'état reçu.

---

## 2. Snapshot / delta

### 2.1 Snapshot

- État complet du monde à un tick donné (ou sous-ensemble pertinent).
- Envoi périodique (ex. toutes les 10 frames) pour resynchronisation.
- Coût : volume plus élevé ; utile pour correction de drift.

### 2.2 Delta

- Seules les **modifications** depuis le dernier état connu sont envoyées.
- Entités modifiées, composants changés, entités créées/supprimées.
- Coût : moindre bande passante ; plus complexe à implémenter.

### 2.3 Stratégie hybride

- Snapshots épisodiques (ex. toutes les 60 frames) + deltas entre eux.
- Réduit la bande passante tout en limitant l'accumulation d'erreurs.

---

## 3. Pas de lockstep obligatoire

- **Lockstep** : tous les clients exécutent exactement les mêmes ticks ; synchronisation par input.
- Le MGE **ne impose pas** le lockstep : l'hôte peut avancer à son rythme et envoyer l'état.
- Lockstep possible si le jeu le requiert (ex. RTS strict) ; implémentable côté jeu.
- Par défaut : host authoritative avec réplication asynchrone.

---

## 4. Compatible MWS

### 4.1 Lobbys

- Un **Lobby** est une surface de connexion exposée par un COG via le MWS.
- Pour un jeu (ex. Allumina), le Lobby = une partie/session hébergée.
- Découverte : les trackers MWS exposent le catalogue de Lobbys (type de service, joueurs, état).

### 4.2 Permis de circulation

- Le COG doit posséder un **Permis de circulation** (accord relay) pour circuler sur le Webway.
- Délivré par un relay après vérification de conformité (Passeport COG, clé Cores, blocs MIP).
- Le jeu ne gère pas le Permis ; c'est le COG (via les Cores) qui le possède.

### 4.3 Accord d'hôte

- Pour rejoindre un Lobby, le client demande un **accord d'hôte** au COG Hébergeur.
- L'hôte décide d'accepter ou refuser (selon règles métier : capacité, whitelist, etc.).
- Une fois l'accord obtenu, le transport des données de jeu (snapshots, inputs) peut commencer.

### 4.4 Transport

- Le MWS fournit le **tunnel étendu** (connexion sécurisée entre COGs).
- Le protocole métier du jeu (snapshot, delta, inputs) est transporté **au-dessus** du tunnel MWS.
- Le MGE ne définit pas le format binaire du protocole métier ; chaque jeu peut avoir le sien.

---

## 5. Flux host authoritative

```
┌─────────────┐                    ┌─────────────┐
│   Client    │                    │    Host     │
│  (COG A)    │                    │  (COG B)    │
└──────┬──────┘                    └──────┬──────┘
       │                                  │
       │  Accord d'hôte (MWS)             │
       │─────────────────────────────────►
       │◄─────────────────────────────────
       │                                  │
       │  Input (action joueur)            │
       │─────────────────────────────────►│
       │                                  │  Simulation
       │                                  │  (tick)
       │  Snapshot / Delta (état)         │
       │◄─────────────────────────────────
       │  Rendu local                     │
       │                                  │
```

### 5.1 Côté hôte

- Reçoit les inputs des clients.
- Simule la partie (Engine, World, Scheduler).
- Envoie l'état (snapshot/delta) aux clients connectés.
- Gère les entrées/sorties de joueurs (join/leave).

### 5.2 Côté client

- Envoie les inputs du joueur local à l'hôte.
- Reçoit l'état et met à jour le World local (replica).
- Rend l'état reçu ; pas d'autorité sur la simulation.
- Option : prédiction client (inputs appliqués localement avant confirmation) — non déterministe, à gérer avec correction.

---

## 6. Intégration plugin network

- Le plugin **mge-plugin-network** fournit :
  - Connexion au Lobby (via MWS, accord d'hôte).
  - Envoi d'inputs.
  - Réception de snapshots/deltas.
  - Application des deltas au World (mode client).

- Le jeu (ex. Allumina) définit :
  - Quels composants sont répliqués.
  - Le format de sérialisation.
  - Les règles de prédiction (si applicable).

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MWS - Document Fondateur](../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | MWS : présence, découverte, transport. |
| [MWS - Lobbys, Favoris et Amis](../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys, accord d'hôte. |
| [MGE - Intégration COG](./MGE%20-%20Integration%20COG.md) | CogService, lancement, isolation. |
| [Allumina - Document Fondateur](../services/Allumina/Allumina%20-%20Document%20Fondateur.md) | Cas d'usage : Lobbys Allumina. |

---

**Document** : MGE — Mode Multijoueur  
**Version** : 1.0  
**Date** : 2026-02-19  
**Statut** : Spécification normative
