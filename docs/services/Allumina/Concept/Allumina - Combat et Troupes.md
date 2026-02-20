# Allumina — Combat et Troupes

## Contexte

Ce document décrit le système de **troupes** et de **combat à échelle variable** d'Allumina. Il couvre les échelles (groupe à armée), les voies (aventurier, mercenaire, armée nation, caravanier, nécromancie), le recrutement, les ordres tactiques et la gestion de la difficulté.

## Portée / Scope

- **Applicable à :** Game design du combat, troupes, IA, ordres tactiques.
- **Audience :** Game design, implémentation Bevy.
- **Statut :** Document conceptuel normatif.

---

## 1. Cap et double limite

### 1.1 Charisme

Le **Charisme** est une caractéristique (voir [Caractéristiques, Aptitudes et Compétences](./Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md)). Il monte comme les autres, via le gain de points de compétences associés.

**Le cap de troupe est plafonné par le Charisme.** Plus le Charisme est élevé, plus le joueur peut avoir de suivants.

### 1.2 Statut social

Une seconde limite s'applique : **le statut** (esclave affranchi, étranger, citoyen, etc.). Quasi personne ne veut suivre un esclave fraîchement affranchi. Au début de partie, le cap effectif est donc très bas (solo ou quasi), même si le Charisme théorique permettrait plus.

---

## 2. Échelles de troupe

| Échelle | Cap | Description |
|---------|-----|-------------|
| **Groupe** | Charisme × 1 | Petite équipe, style Diablo 2. |
| **Compagnie** | Charisme × 3 | Équipe élargie. |
| **Troupe** | Charisme × 5 | Unité tactique. |
| **Régiment / Centurie** | Charisme × 10 | Qualité dépend du rang. |
| **Bataillon / Cohorte** | Plusieurs joueurs « régiment » | Agrégation multi-joueurs. |
| **Armée** | Ensemble des bataillons d'un front | Échelle stratégique. |

---

## 3. Voies de troupe

### 3.1 Aventurier

- **Échelle** : Groupe maximum.
- **Particularité** : Chaque membre est équipable et évolue comme un personnage. Très personnalisable mais peu nombreux.
- **Coût** : Équipement à fournir soi-même.

### 3.2 Mercenaire

- **Échelle** : Groupe à Troupe ; qualité variable.
- **Particularité** : Assignation des PNJ par type de combattant ; évolution fixe (inspiration Mount & Blade) ; plusieurs types de combattants.
- **Recrutement** : Mercenaires recrutables dans le monde.
- **Coût** : Paiement périodique (tous les X temps).

### 3.3 Armée d'une nation

- **Échelle** : Solo à Régiment.
- **Particularité** : Troupes = PNJ alloués par la nation. Qualité et quantité dépendent du **rang** du joueur.
- **Recrutement** : Par progression de rang dans l'armée.

### 3.4 Caravanier

- **Échelle** : Groupe à Compagnie.
- **Particularité** : Similaire aux mercenaires (escorte de caravane).

### 3.5 Nécromancie

- **Échelle** : Solo (apprenti) à Régiment (archi-liche), selon maîtrise en nécromancie.
- **Particularité** : École de magie accessible à tous. Les morts-vivants sont des **invocations** = suivants **temporaires**.
- **Pool séparé** : Les morts-vivants ne comptent **pas** dans le cap Charisme.

---

## 4. Recrutement

Tous les modes sont possibles selon la voie :

- PNJ trouvés dans le monde
- Mercenaires (payants)
- Levées locales
- Progression du rang social / militaire
- Allocations par la nation (voie armée)
- Invocations (nécromancie)

---

## 5. Ordres tactiques

Le niveau de contrôle dépend de l'échelle :

| Échelle | Mode de contrôle | Description |
|---------|------------------|-------------|
| **Groupe** | Suivi automatique | Pas de formation à proprement parler ; suit le joueur (style mercenaires Diablo 2). |
| **Compagnie+** | Ordres mid-battle | Ordres à des sous-groupes par type : « les guerriers corps à corps », « les archers », « les mages », « les guérisseurs » (inspiration Mount & Blade). |
| **Grande échelle** | Ordres balise | Rejoindre point A, mode garde, mode agressif. Vue large type RTS. Le joueur peut prendre sa garde rapprochée et se jeter dans la mêlée (Dynasty Warriors) ; les troupes restent autonomes. |

### 5.1 Sous-groupes (Compagnie+)

Le joueur donne des ordres à des **sous-groupes organisés par types** (guerriers, archers, mages, guérisseurs, etc.).

---

## 6. Gestion de la difficulté

En mode Compagnie+, le joueur contrôle des sous-groupes par type. L'IA des alliés gère le combat au sein de chaque sous-groupe. Le joueur peut se concentrer sur son avatar et les ordres de haut niveau.

---

## 7. Guerres entre nations

Le joueur peut être :
- **Au cœur de la bataille** (combat à la Dynasty Warriors),
- **En mode officier / stratège**,
- Ou **les deux** selon l'échelle à laquelle il opère.

---

## 8. Référence prototype

Le service **MiyukiniSurvivor** contient un début d'IA et de boucle de gameplay (recrutement, combat). Il sert de **référence** pour les mécaniques. Le code Allumina sera **réécrit en Bevy** (pas de réutilisation directe).

---

## 9. Références

| Document | Rôle |
|----------|------|
| [Allumina - Vision Gameplay et Ambition](./Allumina%20-%20Vision%20Gameplay%20et%20Ambition.md) | Vision générale, progression sociale. |
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Charisme, aptitudes de combat. |

---

**Document** : Allumina — Combat et Troupes  
**Version** : 1.0  
**Date** : 2026-02-17
