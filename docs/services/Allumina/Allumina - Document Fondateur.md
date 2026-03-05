# Allumina â€” Document Fondateur

## Contexte

**Allumina** est le **jeu Action RPG** de l'Ã©cosystÃ¨me Miyukini COG. InspirÃ© des rÃ©fÃ©rences du genre (Diablo 2, Path of Exile, Sacred), il est dÃ©veloppÃ© en **Rust** avec le **moteur de jeu Miyukini** (maison) et utilise le **MWS (Miyukini Webway System)** pour la partie multijoueur : dÃ©couverte des parties, connexion entre COGs via les **Lobbys**, sans dÃ©pendance critique Ã  l'exÃ©cution (LOI-1).

Ce document pose la vision et les principes fondateurs. Pour les concepts dÃ©taillÃ©s (monde, mÃ©caniques, persistance, Lobbys jeu), voir le [Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md).

## PortÃ©e / Scope

- **Applicable Ã  :** Vision produit, principes non nÃ©gociables, positionnement du service, intÃ©gration MWS.
- **Audience :** Parties prenantes, Ã©quipes produit, architecture.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> Un Action RPG souverain : jouable en solo ou en multijoueur via le Webway, avec les donnÃ©es de progression et les parties hÃ©bergÃ©es chez les joueurs ou sur des Lobbys exposÃ©s par des COGs, sans serveur central obligatoire.

| Principe | Description |
|----------|--------------|
| **SouverainetÃ© des donnÃ©es** | Progression, personnages et sauvegardes restent sous le contrÃ´le du joueur (COG local) ou de l'hÃ´te de partie (Lobby). |
| **Multijoueur via MWS** | DÃ©couverte des parties via le catalogue de Lobbys ; connexion client â†’ hÃ´te gouvernÃ©e par le MWS (Permis de circulation, accord d'hÃ´te). |
| **Pas de dÃ©pendance critique** | Le jeu fonctionne hors-ligne en solo ; le rÃ©seau est optionnel (LOI-1, LOI-2). |
| **Moteur Rust maison** | Binaire autonome, open source, sans runtime externe ; moteur Miyukini (voir [Moteur Jeux et Central Launcher](..//..//_index.md)). |

---

## 2. Type de Service

**Service Inter-COG (Type 3)** â€” jeu consommable depuis Miyukini Central ou en standalone :

- **Espace Miyukini Central :** lancement du jeu, liste des Lobbys Allumina (catalogue MWS), favoris, paramÃ¨tres.
- **Application jeu (moteur Miyukini) :** solo ou client multijoueur ; un COG peut aussi exposer un **Lobby Allumina** (hÃ©berger une partie).
- **Protocoles Inter-COG :** dÃ©couverte des Lobbys via les trackers MWS ; connexion aux parties via accord d'hÃ´te ; donnÃ©es de jeu (Ã©tats, actions) transitent entre client et hÃ´te selon le protocole mÃ©tier Allumina, au-dessus du transport MWS.

---

## 3. CapacitÃ©s clÃ©s

| CapacitÃ© | Description |
|----------|-------------|
| **Solo** | Campagne et progression entiÃ¨rement jouables hors-ligne ; sauvegarde locale (KindMother). |
| **Multijoueur (Lobbys)** | Un COG peut crÃ©er un **Lobby Allumina** (partie/serveur) ; les autres COGs dÃ©couvrent les Lobbys via le catalogue MWS (visible depuis le service Allumina) et rejoignent avec accord d'hÃ´te. |
| **DÃ©couverte** | Liste des parties disponibles = catalogue de Lobbys filtrÃ© par type de service Â« Allumina Â», exposÃ© par les trackers et affichÃ© dans le client jeu (ou Central). |
| **Progression** | Personnages, inventaire, quÃªtes : stockage local (solo) ou synchronisÃ© par l'hÃ´te (partie multijoueur), selon le design mÃ©tier. |

---

## 4. Stack technique (alignement LOI-1)

| Couche | Choix | Justification |
|--------|--------|----------------|
| **Moteur** | Moteur Miyukini (Rust, maison) | 2D (3D optionnel), binaire statique, pas de dÃ©pendance runtime critique. |
| **RÃ©seau mÃ©tier** | Protocole Allumina (Ã  dÃ©finir) sur transport MWS | RÃ©plication jeu (Ã©tat + backend MWS) ; optionnel. |
| **Persistance** | KindMother (local), ou Ã©tat hÃ´te (Lobby) | LOI-3 : Ã©tat local souverain. |
| **PrÃ©sence / Lobbys** | MWS (trackers, catalogue de Lobbys, accord d'hÃ´te) | DÃ©couverte et connexion sans serveur dÃ©diÃ© Allumina. |

---

## 5. DÃ©pendances

- **MWS :** prÃ©sence, dÃ©couverte, catalogue de Lobbys, transport. Allumina consomme ces capacitÃ©s ; il ne dÃ©finit pas le protocole de prÃ©sence.
- **Cores :** KindMother (sauvegardes locales), StrongFather (autorisation), WorrySentinel (sÃ©curitÃ©), Border Guard (frontiÃ¨res Inter-COG).
- **Outils (Strate 6) :** MiyuWebwayTracker, MiyuWebwayParticipant (annonces, Lobbys, dÃ©couverte) pour l'intÃ©gration dans Central et le client jeu.

---

## 6. Lois d'Autonomie

Allumina respecte en particulier :

- **LOI-1** â€” Aucune dÃ©pendance externe critique Ã  l'exÃ©cution : jeu jouable en solo hors-ligne ; moteur maison = crate Rust, binaire autonome.
- **LOI-2** â€” Le systÃ¨me accepte l'isolement : multijoueur optionnel ; jouable sans rÃ©seau.
- **LOI-3** â€” L'Ã©tat local est souverain : sauvegardes et progression locale maÃ®trisÃ©es par le COG.
- **LOI-6** â€” L'autonomie n'empÃªche pas la fÃ©dÃ©ration : parties multijoueur via MWS et Lobbys.

---

## 7. RÃ©sumÃ©

**Allumina** = Action RPG Miyukini : **moteur maison (Rust)**, **solo + multijoueur via MWS** (Lobbys = parties), **donnÃ©es souveraines**, gouvernance par les Cores, dÃ©couverte et transport par le Webway.

---

## 8. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Allumina - Document Conceptuel](./Concept/Allumina%20-%20Document%20Conceptuel.md) | Concepts jeu (monde, mÃ©caniques, Lobbys, persistance). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | PrÃ©sence, dÃ©couverte, transport, Lobbys. |
| [MWS - Lobbys, Favoris et Amis](../../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privÃ©s, accord d'hÃ´te. |

---

**Document** : Allumina â€” Document Fondateur  
**Version** : 1.0  
**Date** : 2026-02-17  
**Statut** : Document fondateur normatif

