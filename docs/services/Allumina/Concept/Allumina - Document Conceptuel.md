# Allumina — Document Conceptuel

## Contexte

**Allumina** est un **jeu Action RPG** de l'écosystème Miyukini COG. Il vise une expérience de type Diablo 2, Path of Exile ou Sacred : vue isométrique (ou 2D top-down), combat en temps réel, loot, progression de personnage, monde persistant ou instancié, et **multijoueur optionnel** via le MWS (Miyukini Webway System).

Ce document est le **document conceptuel** du jeu : il en fixe la vision créative, le genre, le positionnement solo/multijoueur, l'usage des **Lobbys MWS** comme parties/serveurs de jeu, et les principes de persistance. Il ne décrit pas l'implémentation technique ni les écrans détaillés.

## Portée / Scope

- **Applicable à :** Vision jeu, genre, monde, mécaniques de haut niveau, modèle solo/multijoueur, Lobbys, persistance.
- **Audience :** Game design, équipes produit, architecture.
- **Statut :** Document conceptuel normatif — référence pour la conception du jeu.

### Hors périmètre

- Détail des écrans et UI (à traiter dans un document dédié).
- Spécification du protocole réseau métier Allumina (états, réplication).
- Choix d'assets, level design, narration (à préciser ultérieurement).

---

## 1. Genre et références

| Aspect | Description |
|--------|--------------|
| **Genre** | Action RPG (ARPG) : combat en temps réel, quêtes, loot, progression de personnage. |
| **Vue** | 2D isométrique ou top-down (à valider) ; rendu Bevy 2D. |
| **Références** | Diablo 2, Path of Exile, Sacred — inspiration pour le feel du combat, la progression et le loot. |

Les références servent de boussole pour le game feel et les attentes joueur ; Allumina reste un produit distinct, souverain et aligné sur les Lois d'Autonomie (solo jouable, multijoueur via MWS).

---

## 2. Monde et cadre (esquisse)

Le monde et le cadre narratif seront détaillés dans des documents dédiés (lore, zones, factions). Ce document retient uniquement :

- **Monde persistant (solo)** : le joueur progresse dans un monde dont l'état est sauvegardé localement (KindMother).
- **Parties multijoueur (Lobbys)** : une partie = une instance de jeu hébergée par un COG ; les joueurs rejoignent via le catalogue de Lobbys MWS (liste des « serveurs » Allumina).

La frontière entre monde persistant partagé (type MMO light) et parties instanciées (type donjon/partie) sera précisée au game design ; le MWS permet les deux (un Lobby = une surface de connexion exposée par un COG).

---

## 3. Solo et multijoueur

### 3.1 Solo

- **Campagne / aventure** : jouable entièrement hors-ligne.
- **Sauvegarde** : locale sur le COG du joueur (KindMother) ; pas de serveur obligatoire.
- **Progression** : personnage(s), inventaire, quêtes, état du monde gérés en local.

### 3.2 Multijoueur via MWS

Le multijoueur s'appuie sur le **MWS** et le modèle **Lobby** :

| Concept | Description |
|---------|-------------|
| **Lobby Allumina** | Un COG **hôte** expose une **partie** Allumina comme **Lobby** dans le catalogue de Lobbys tenu par les trackers. Le Lobby représente une surface de connexion (service « Allumina », port(s), nom de partie, visibilité publique/privée). |
| **Découverte** | Le client Allumina (ou Central) interroge le **catalogue de Lobbys** (visible depuis les services COG, pas depuis le portail web des trackers). Les Lobbys de type « Allumina » sont listés (nom, hôte, nombre de joueurs, visibilité, etc.). |
| **Connexion** | Le joueur choisit un Lobby → demande de connexion au tracker/hôte → **Permis de circulation** (MWS) puis **accord d'hôte** délivré par le COG hôte → connexion au flux de jeu (protocole métier Allumina). |
| **Hébergement** | Tout COG peut créer un Lobby Allumina (devenir hôte d'une partie) ; l'état de la partie (monde, entités, joueurs connectés) est géré par l'hôte. |

En résumé : **Lobbys MWS = parties/serveurs Allumina** ; découverte via le catalogue, connexion gouvernée par le MWS (accord d'hôte), données métier jeu (réplication, états) au-dessus du transport.

---

## 4. Persistance

| Contexte | Responsable | Règle conceptuelle |
|----------|-------------|---------------------|
| **Solo** | COG du joueur (KindMother) | Sauvegarde locale souveraine ; pas de cloud obligatoire. |
| **Partie multijoueur (Lobby)** | COG hôte | L'hôte détient l'état de la partie ; les clients reçoivent les mises à jour (réplication). La persistance entre sessions de partie (sauvegarde côté hôte) sera précisée au design. |
| **Progression personnage en multijoueur** | À définir | Soit progression locale (personnage exporté/importé), soit progression gérée par l'hôte ; à trancher au game design. |

Principe retenu : **LOI-3** — l'état local est souverain ; pas de base de données centrale Allumina obligatoire.

---

## 5. Concepts de jeu (à détailler)

Les concepts suivants sont développés dans des documents dédiés ou restent à préciser :

- **Personnage** : caractéristiques, aptitudes de combat, compétences, équipement, inventaire — voir [Allumina - Caractéristiques, Aptitudes et Compétences](./Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md).
- **Combat** : types de dégâts, résistances, compétences en temps réel (à détailler).
- **Loot et objets** : rareté, génération, stockage (à détailler).
- **Quêtes et progression** : objectifs, récompenses, état de quête (solo vs partie) (à détailler).
- **Monde** : zones, cartes, instances (solo vs Lobby) (à détailler).
- **Économie** : si applicable (marchands, échange entre joueurs dans une partie) (à détailler).

Ce document conceptuel pose que ces briques existent et sont cohérentes avec le modèle solo + Lobbys MWS.

---

## 6. Intégration Central (optionnel)

Allumina peut être **lancé depuis Miyukini Central** (comme un service de la strate 7) : raccourci lancement, liste des Lobbys Allumina (catalogue MWS), favoris, paramètres. Le client jeu peut aussi être utilisé en **standalone** (sans Central), en s'appuyant sur les mêmes outils MWS (MiyuWebwayTracker, MiyuWebwayParticipant) pour la découverte des Lobbys si le joueur est connecté au Webway.

---

## 7. Résumé

- **Allumina** = Action RPG (Bevy, Rust), inspiré Diablo 2 / PoE / Sacred.
- **Solo** : campagne hors-ligne, sauvegarde locale (KindMother).
- **Multijoueur** : parties = **Lobbys MWS** ; découverte via le catalogue de Lobbys, connexion via accord d'hôte ; pas de serveur central Allumina.
- **Persistance** : état local souverain (solo) ; état de partie côté hôte (Lobby).
- **Central** : lancement et liste des Lobbys possibles ; jeu aussi utilisable en standalone.

---

## 8. Références

| Document | Rôle |
|----------|------|
| [Allumina - Document Fondateur](../Allumina%20-%20Document%20Fondateur.md) | Vision service, MWS, Lois d'Autonomie. |
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Caractéristiques, aptitudes de combat, compétences, plafonds. |
| [MWS - Document Fondateur](../../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, Lobbys. |
| [MWS - Lobbys, Favoris et Amis](../../../miyukini-webway-system/lobbys/MWS%20-%20Lobbys%20Favoris%20et%20Amis.md) | Lobbys publics/privés, surfaces, accord d'hôte. |

---

**Document** : Allumina — Document Conceptuel  
**Version** : 1.0  
**Date** : 2026-02-17  
**Statut** : Document conceptuel normatif
