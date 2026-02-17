# Allumina — Document Fondateur

## Contexte

**Allumina** est le **jeu Action RPG** de l'écosystème Miyukini COG. Inspiré des références du genre (Diablo 2, Path of Exile, Sacred), il est développé en **Rust** avec le moteur **Bevy** et utilise le **MWS (Miyukini Webway System)** pour la partie multijoueur : découverte des parties, connexion entre COGs via les **Lobbys**, sans dépendance critique à l'exécution (LOI-1).

Ce document pose la vision et les principes fondateurs. Pour les concepts détaillés (monde, mécaniques, persistance, Lobbys jeu), voir le [Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md).

## Portée / Scope

- **Applicable à :** Vision produit, principes non négociables, positionnement du service, intégration MWS.
- **Audience :** Parties prenantes, équipes produit, architecture.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> Un Action RPG souverain : jouable en solo ou en multijoueur via le Webway, avec les données de progression et les parties hébergées chez les joueurs ou sur des Lobbys exposés par des COGs, sans serveur central obligatoire.

| Principe | Description |
|----------|--------------|
| **Souveraineté des données** | Progression, personnages et sauvegardes restent sous le contrôle du joueur (COG local) ou de l'hôte de partie (Lobby). |
| **Multijoueur via MWS** | Découverte des parties via le catalogue de Lobbys ; connexion client → hôte gouvernée par le MWS (Permis de circulation, accord d'hôte). |
| **Pas de dépendance critique** | Le jeu fonctionne hors-ligne en solo ; le réseau est optionnel (LOI-1, LOI-2). |
| **Moteur Rust/Bevy** | Binaire autonome, open source, sans runtime externe ni launcher obligatoire. |

---

## 2. Type de Service

**Service Inter-COG (Type 3)** — jeu consommable depuis Miyukini Central ou en standalone :

- **Espace Miyukini Central :** lancement du jeu, liste des Lobbys Allumina (catalogue MWS), favoris, paramètres.
- **Application jeu (Bevy) :** solo ou client multijoueur ; un COG peut aussi exposer un **Lobby Allumina** (héberger une partie).
- **Protocoles Inter-COG :** découverte des Lobbys via les trackers MWS ; connexion aux parties via accord d'hôte ; données de jeu (états, actions) transitent entre client et hôte selon le protocole métier Allumina, au-dessus du transport MWS.

---

## 3. Capacités clés

| Capacité | Description |
|----------|-------------|
| **Solo** | Campagne et progression entièrement jouables hors-ligne ; sauvegarde locale (KindMother). |
| **Multijoueur (Lobbys)** | Un COG peut créer un **Lobby Allumina** (partie/serveur) ; les autres COGs découvrent les Lobbys via le catalogue MWS (visible depuis le service Allumina) et rejoignent avec accord d'hôte. |
| **Découverte** | Liste des parties disponibles = catalogue de Lobbys filtré par type de service « Allumina », exposé par les trackers et affiché dans le client jeu (ou Central). |
| **Progression** | Personnages, inventaire, quêtes : stockage local (solo) ou synchronisé par l'hôte (partie multijoueur), selon le design métier. |

---

## 4. Stack technique (alignement LOI-1)

| Couche | Choix | Justification |
|--------|--------|----------------|
| **Moteur** | Bevy (Rust) | 2D/3D, ECS, binaire statique, pas de dépendance runtime critique. |
| **Réseau métier** | Protocole Allumina (à définir) sur transport MWS | Réplication jeu (ex. bevy_replicon + backend MWS) ; optionnel. |
| **Persistance** | KindMother (local), ou état hôte (Lobby) | LOI-3 : état local souverain. |
| **Présence / Lobbys** | MWS (trackers, catalogue de Lobbys, accord d'hôte) | Découverte et connexion sans serveur dédié Allumina. |

---

## 5. Dépendances

- **MWS :** présence, découverte, catalogue de Lobbys, transport. Allumina consomme ces capacités ; il ne définit pas le protocole de présence.
- **Cores :** KindMother (sauvegardes locales), StrongFather (autorisation), WorrySentinel (sécurité), Border Guard (frontières Inter-COG).
- **Outils (Strate 6) :** MiyuWebwayTracker, MiyuWebwayParticipant (annonces, Lobbys, découverte) pour l'intégration dans Central et le client jeu.

---

## 6. Lois d'Autonomie

Allumina respecte en particulier :

- **LOI-1** — Aucune dépendance externe critique à l'exécution : jeu jouable en solo hors-ligne ; Bevy = crate Rust, binaire autonome.
- **LOI-2** — Le système accepte l'isolement : multijoueur optionnel ; jouable sans réseau.
- **LOI-3** — L'état local est souverain : sauvegardes et progression locale maîtrisées par le COG.
- **LOI-6** — L'autonomie n'empêche pas la fédération : parties multijoueur via MWS et Lobbys.

---

## 7. Résumé

**Allumina** = Action RPG Miyukini : **Bevy (Rust)**, **solo + multijoueur via MWS** (Lobbys = parties), **données souveraines**, gouvernance par les Cores, découverte et transport par le Webway.

---

## 8. Références

| Document | Rôle |
|----------|------|
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Concepts jeu (monde, mécaniques, Lobbys, persistance). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, transport, Lobbys. |
| [MWS - Lobbys, Favoris et Amis](../../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privés, accord d'hôte. |

---

**Document** : Allumina — Document Fondateur  
**Version** : 1.0  
**Date** : 2026-02-17  
**Statut** : Document fondateur normatif
