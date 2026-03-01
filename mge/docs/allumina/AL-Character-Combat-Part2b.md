<!-- @id: AL-Character-Combat-Part2b @do: reference @role: game-designer @layer: 3 @human: miyuk -->

# AL-Character-Combat-Part2b — Allumina : Systèmes Transversaux du Personnage

**Statut :** Référence canonique v1.0
**Date :** 2026-02-28
**Scope :** Progression, stats, équipement, mort — systèmes transversaux applicables à toutes les classes
**Monde :** Véranthas, An 247 AO

---

## Table des matières

1. [Système de progression hybride](#1-systeme-de-progression-hybride)
   - 1.1 [Progression par niveau](#11-progression-par-niveau)
   - 1.2 [Progression par skill usage](#12-progression-par-skill-usage)
   - 1.3 [Reset et respec](#13-reset-et-respec)
   - 1.4 [Système paragon post-60](#14-systeme-paragon-post-60)
2. [Stats du personnage](#2-stats-du-personnage)
   - 2.1 [Stats primaires — Les Quatre Piliers](#21-stats-primaires--les-quatre-piliers)
   - 2.2 [Stats dérivées](#22-stats-derivees)
   - 2.3 [Résistances — Les Cinq Voiles](#23-resistances--les-cinq-voiles)
   - 2.4 [Tableau stats de base par classe](#24-tableau-stats-de-base-par-classe)
3. [Équipement](#3-equipement)
   - 3.1 [Slots d'équipement](#31-slots-dequipement)
   - 3.2 [Qualités d'équipement](#32-qualites-dequipement)
   - 3.3 [Sets d'équipement](#33-sets-dequipement)
   - 3.4 [Prérequis d'équipement](#34-prerequis-dequipement)
4. [Mort et conséquences](#4-mort-et-consequences)
   - 4.1 [Règles de mort en zone normale](#41-regles-de-mort-en-zone-normale)
   - 4.2 [Règles de mort en zone de guerre](#42-regles-de-mort-en-zone-de-guerre)
   - 4.3 [Règles de mort en donjon instancié](#43-regles-de-mort-en-donjon-instancie)
   - 4.4 [Règles de mort en Stampede](#44-regles-de-mort-en-stampede)
   - 4.5 [Pénalité XP](#45-penalite-xp)
   - 4.6 [Mode Hardcore](#46-mode-hardcore)
   - 4.7 [Ghost et retour au corps](#47-ghost-et-retour-au-corps)
5. [Schémas TOML complets](#5-schemas-toml-complets)

---

## Contexte

Ce document définit les systèmes transversaux du personnage dans Allumina — les mécaniques qui s'appliquent à **toutes** les identités sociales (Citoyen, Homme Libre, Habitant, Banni) et à l'ensemble des 33 classes du jeu (9 classes Empire Pourpre, 9 classes Alliance de Rive, 9 classes Fédération Ervan, 6 classes Outlaws).

Ces systèmes sont les colonnes vertébrales du gameplay individuel. Ils doivent être cohérents avec le lore de Véranthas : la corruption de Garum a un impact sur les mécaniques de mort, la cosmologie des Quatre Piliers justifie la nomenclature des stats primaires, et les résistances élémentaires reflètent les cinq forces fondamentales du monde.

---

## 1. Système de progression hybride

### 1.1 Progression par niveau

#### Philosophie

Allumina adopte un système de progression **hybride** : l'expérience (XP) fait monter les niveaux et débloque des **points de stat** et des **points de skill**. Les skills eux-mêmes progressent séparément par l'usage. Cette dualité empêche qu'un joueur soit à la fois puissant en stats et maître de tous ses skills — le choix d'usage façonne le personnage autant que le niveau atteint.

Le niveau maximal est **60**. Au-delà, le système Paragon prend le relais.

#### Formule XP

```
xp_requis(n) = 500 * n ^ 1.8
```

Cette courbe est délibérément progressive : les niveaux 1-10 sont rapides (initiation), 11-30 sont modérés (campagne de faction), 31-45 sont lents (endgame lite, contenu saisonnier), 46-60 sont exigeants (endgame complet, nécessite du contenu de Convergence ou de Stampede).

#### Tableau de progression niveaux 1 à 60

| Niveau | XP requis | XP cumulée | Points stat | Points skill | Total stat cumulé | Total skill cumulé | Palier |
|--------|-----------|------------|-------------|--------------|-------------------|--------------------|--------|
| 1 | 0 | 0 | 5 | 2 | 5 | 2 | Départ classless |
| 2 | 707 | 707 | 3 | 1 | 8 | 3 | |
| 3 | 1 561 | 2 268 | 3 | 1 | 11 | 4 | |
| 4 | 2 520 | 4 788 | 3 | 1 | 14 | 5 | |
| 5 | 3 576 | 8 364 | 3 | 1 | 17 | 6 | |
| 6 | 4 720 | 13 084 | 3 | 1 | 20 | 7 | |
| 7 | 5 940 | 19 024 | 3 | 1 | 23 | 8 | |
| 8 | 7 231 | 26 255 | 3 | 1 | 26 | 9 | |
| 9 | 8 586 | 34 841 | 3 | 1 | 29 | 10 | |
| **10** | **10 000** | **44 841** | **4** | **2** | **33** | **12** | **Débloque spécialisation de faction** |
| 11 | 11 465 | 56 306 | 3 | 1 | 36 | 13 | |
| 12 | 12 977 | 69 283 | 3 | 1 | 39 | 14 | |
| 13 | 14 534 | 83 817 | 3 | 1 | 42 | 15 | |
| 14 | 16 133 | 99 950 | 3 | 1 | 45 | 16 | |
| 15 | 17 771 | 117 721 | 3 | 1 | 48 | 17 | |
| 16 | 19 448 | 137 169 | 3 | 1 | 51 | 18 | |
| 17 | 21 162 | 158 331 | 3 | 1 | 54 | 19 | |
| 18 | 22 910 | 181 241 | 3 | 1 | 57 | 20 | |
| 19 | 24 691 | 205 932 | 3 | 1 | 60 | 21 | |
| **20** | **26 504** | **232 436** | **4** | **2** | **64** | **23** | **Accès zones niveau 2, Convergence lite** |
| 21 | 28 348 | 260 784 | 3 | 1 | 67 | 24 | |
| 22 | 30 221 | 291 005 | 3 | 1 | 70 | 25 | |
| 23 | 32 122 | 323 127 | 3 | 1 | 73 | 26 | |
| 24 | 34 051 | 357 178 | 3 | 1 | 76 | 27 | |
| 25 | 36 006 | 393 184 | 3 | 1 | 79 | 28 | |
| 26 | 37 987 | 431 171 | 3 | 1 | 82 | 29 | |
| 27 | 39 993 | 471 164 | 3 | 1 | 85 | 30 | |
| 28 | 42 023 | 513 187 | 3 | 1 | 88 | 31 | |
| 29 | 44 076 | 557 263 | 3 | 1 | 91 | 32 | |
| **30** | **46 152** | **603 415** | **4** | **2** | **95** | **34** | **Convergence débloquée, RvR complet** |
| 31 | 48 250 | 651 665 | 3 | 1 | 98 | 35 | |
| 32 | 50 370 | 702 035 | 3 | 1 | 101 | 36 | |
| 33 | 52 510 | 754 545 | 3 | 1 | 104 | 37 | |
| 34 | 54 670 | 809 215 | 3 | 1 | 107 | 38 | |
| 35 | 56 850 | 866 065 | 3 | 1 | 110 | 39 | |
| 36 | 59 049 | 925 114 | 3 | 1 | 113 | 40 | |
| 37 | 61 267 | 986 381 | 3 | 1 | 116 | 41 | |
| 38 | 63 503 | 1 049 884 | 3 | 1 | 119 | 42 | |
| 39 | 65 757 | 1 115 641 | 3 | 1 | 122 | 43 | |
| 40 | 68 028 | 1 183 669 | 3 | 1 | 125 | 44 | |
| 41 | 70 317 | 1 253 986 | 3 | 1 | 128 | 45 | |
| 42 | 72 622 | 1 326 608 | 3 | 1 | 131 | 46 | |
| 43 | 74 943 | 1 401 551 | 3 | 1 | 134 | 47 | |
| 44 | 77 280 | 1 478 831 | 3 | 1 | 137 | 48 | |
| **45** | **79 633** | **1 558 464** | **4** | **2** | **141** | **50** | **Endgame lite — Raids 10 joueurs, donjons héroïques** |
| 46 | 82 001 | 1 640 465 | 3 | 1 | 144 | 51 | |
| 47 | 84 385 | 1 724 850 | 3 | 1 | 147 | 52 | |
| 48 | 86 784 | 1 811 634 | 3 | 1 | 150 | 53 | |
| 49 | 89 198 | 1 900 832 | 3 | 1 | 153 | 54 | |
| 50 | 91 627 | 1 992 459 | 3 | 1 | 156 | 55 | |
| 51 | 94 070 | 2 086 529 | 3 | 1 | 159 | 56 | |
| 52 | 96 527 | 2 183 056 | 3 | 1 | 162 | 57 | |
| 53 | 98 999 | 2 282 055 | 3 | 1 | 165 | 58 | |
| 54 | 101 484 | 2 383 539 | 3 | 1 | 168 | 59 | |
| 55 | 103 984 | 2 487 523 | 3 | 1 | 171 | 60 | |
| 56 | 106 497 | 2 594 020 | 3 | 1 | 174 | 61 | |
| 57 | 109 024 | 2 703 044 | 3 | 1 | 177 | 62 | |
| 58 | 111 564 | 2 814 608 | 3 | 1 | 180 | 63 | |
| 59 | 114 117 | 2 928 725 | 3 | 1 | 183 | 64 | |
| **60** | **116 683** | **3 045 408** | **5** | **3** | **188** | **67** | **Endgame complet — Voix de l'Érosion, Grande Convergence** |

**Note de design :** À niveau 1, le personnage reçoit 5 points de stat (distribution libre, pas d'attribution automatique) et 2 points de skill. Ces points de départ reflètent l'identité sociale choisie dans les 4 options classless.

#### Paliers importants — détail

**Niveau 10 — Débloque la spécialisation de faction**

C'est le premier vrai choix de classe. Le joueur qui a atteint le niveau 10 peut rejoindre une guilde de classe dans sa faction (ou rester "sans classe" comme Aventurier / Mercenaire / Outlaw prolongé). Ce choix affecte quels skills lui sont accessibles, quels prérequis d'équipement il peut satisfaire, et son titre social dans sa faction.

**Niveau 20 — Accès aux zones de niveau 2**

Les zones de campagne intermédiaires s'ouvrent. La "Convergence lite" (zone partagée non-RvR) permet les premières rencontres inter-factions. Les premiers donjons à 5 joueurs avec loot rare deviennent disponibles.

**Niveau 30 — Convergence**

La grande zone partagée — l'Île de la Convergence et ses alentours — s'ouvre. Les zones RvR complètes (Plateau de Velharris, Marches Brûlées) sont accessibles. Le système de réputation inter-factions démarre. Les Mercenaires peuvent désormais être recrutés par toutes les factions.

**Niveau 45 — Endgame lite**

Raids 10 joueurs actifs. Donjons héroïques avec mécaniques complexes. Les Voix de l'Érosion (lieutenants de Garum) commencent à apparaître en world boss. Premiers crafts de tier Légendaire accessibles via recettes de guilde.

**Niveau 60 — Endgame complet**

Raids 25 joueurs. Grande Convergence narrative (arcs saisonniers). Confrontations directes avec les Voix de l'Érosion dans leurs lieux de pouvoir. Crafts Artefact de Faction accessibles. Paragon démarre.

#### Bonus d'XP

| Source | Bonus XP | Conditions |
|--------|----------|------------|
| Groupe (2-4 joueurs) | +15% par joueur supplémentaire | XP partagée sur le groupe |
| Groupe (5 joueurs) | +50% total | Raid light — non applicable aux grands groupes |
| Zone de guerre active | +25% sur kills de monstres | Uniquement en zone RvR active |
| Zone de Garum active | +30% sur kills de créatures corrompues | Zone en Phase 2+ de Stampede |
| Quêtes de faction | +20% sur XP de la quête | Quêtes de rang 3+ |
| First kill de boss de zone | +500% de l'XP de base du boss | Une seule fois par personnage par boss |
| First kill de boss de donjon (mode normal) | +200% | Une seule fois par semaine |
| First kill de boss de donjon (mode héroïque) | +400% | Une seule fois par semaine |
| Bonus de faction (zone contrôlée) | +10% permanent | Dans une zone contrôlée par votre faction |
| Repos (offline) | Compte XP doublée jusqu'à 1 niveau | 8h hors connexion = 1 "niveau de repos" |

---

### 1.2 Progression par skill usage

#### Philosophie

Un skill ne progresse **que si utilisé en situation de combat réel**. "Combat réel" se définit techniquement : l'ennemi doit avoir son flag "in_combat" actif, doit avoir infligé ou reçu des dégâts dans les 10 dernières secondes, et doit être une créature hostile (PNJ ou joueur ennemi en PvP).

L'entraînement sur les mannequins des dojos ne génère aucun usage comptabilisé. Les skills utilisés sur des créatures déjà mortes ne comptent pas. L'anti-farm est intégré.

#### Types de skills et usages requis par rang

**Type 1 — Actif DPS (exemples : Frappe Brutale, Salve de Flèches, Boule de Garum Inversée)**

Ces skills sont utilisés très fréquemment en combat. Ils progressent donc avec un seuil d'usage plus élevé.

| Rang | Usages requis (ce rang) | Usages cumulés | Bonus principal | Feature débloquée |
|------|------------------------|----------------|-----------------|-------------------|
| 1 | — (départ) | 0 | Base | Effet de base |
| 2 | 250 | 250 | +5% dégâts | — |
| 3 | 280 | 530 | +5% dégâts | Portée +5% |
| 4 | 310 | 840 | +5% dégâts | — |
| 5 | 340 | 1 180 | +5% dégâts | Effet secondaire amélioré |
| 6 | 380 | 1 560 | +5% dégâts | — |
| 7 | 420 | 1 980 | +5% dégâts | Coût réduit 5% |
| 8 | 460 | 2 440 | +5% dégâts | — |
| 9 | 500 | 2 940 | +5% dégâts | AoE splash 10% |
| 10 | 550 | 3 490 | +5% dégâts | Débloque variante avancée |
| 11 | 600 | 4 090 | +4% dégâts | — |
| 12 | 650 | 4 740 | +4% dégâts | Coût réduit 10% |
| 13 | 700 | 5 440 | +4% dégâts | — |
| 14 | 750 | 6 190 | +4% dégâts | Proc passif amélioré |
| 15 | 800 | 6 990 | +4% dégâts | — |
| 16 | 860 | 7 850 | +3% dégâts | Portée +10% cumulé |
| 17 | 920 | 8 770 | +3% dégâts | — |
| 18 | 980 | 9 750 | +3% dégâts | Coût réduit 15% |
| 19 | 1 040 | 10 790 | +3% dégâts | — |
| 20 | 1 110 | 11 900 | +5% dégâts | Forme Maîtrisée débloquée |

**Bonus total rang 20 actif DPS :** +80% dégâts sur le skill, coût réduit 15%, portée +15%, forme Maîtrisée.

**Type 2 — Actif Utilitaire (exemples : Soin d'Urgence, Toile d'Immobilisation, Écran de Brume)**

Ces skills sont moins spammés que les DPS mais ont plus d'impact par usage. Ils progressent plus vite en usages absolus.

| Rang | Usages requis (ce rang) | Bonus principal | Feature débloquée |
|------|------------------------|-----------------|-------------------|
| 1 | — | Base | Effet de base |
| 2 | 120 | +6% efficacité | — |
| 3 | 135 | +6% efficacité | Durée +5% |
| 4 | 150 | +6% efficacité | — |
| 5 | 165 | +6% efficacité | Coût réduit 8% |
| 6 | 180 | +6% efficacité | — |
| 7 | 200 | +6% efficacité | Rayon +10% |
| 8 | 220 | +6% efficacité | — |
| 9 | 240 | +6% efficacité | Durée +10% cumulé |
| 10 | 260 | +6% efficacité | Effet secondaire débloqué |
| 11 | 280 | +5% efficacité | — |
| 12 | 300 | +5% efficacité | Coût réduit 15% |
| 13 | 320 | +5% efficacité | — |
| 14 | 340 | +4% efficacité | Portée +10% |
| 15 | 360 | +4% efficacité | Forme Maîtrisée débloquée |

Rang maximum : 15. Usages cumulés à rang 15 : 3 270.

**Type 3 — Passif (exemples : Endurance de Veine, Peau d'Écorce, Reflexes d'Ombre)**

Les passifs progressent à l'usage indirect — chaque fois que la condition déclenchante du passif se produit en combat, l'usage est comptabilisé.

| Rang | Déclenchements requis (ce rang) | Bonus principal |
|------|---------------------------------|-----------------|
| 1 | — | Base |
| 2 | 60 | +8% efficacité |
| 3 | 65 | +8% efficacité |
| 4 | 70 | +8% efficacité |
| 5 | 75 | +8% efficacité |
| 6 | 80 | +8% efficacité |
| 7 | 85 | +6% efficacité |
| 8 | 90 | +6% efficacité |
| 9 | 95 | +6% efficacité |
| 10 | 100 | +6% efficacité + Feature finale |

Rang maximum : 10. Déclenchements cumulés à rang 10 : 720.

**Type 4 — Aura (exemples : Aura de Commandement, Halo de Gaïa, Présence de l'Ombre)**

Les auras progressent au temps passé actives en combat.

| Rang | Secondes de combat en aura active (ce rang) | Bonus principal |
|------|---------------------------------------------|-----------------|
| 1 | — | Base |
| 2 | 600 s | +5% puissance d'aura |
| 3 | 700 s | +5% rayon |
| 4 | 800 s | +5% puissance |
| 5 | 900 s | +5% rayon |
| 6 | 1 000 s | +5% puissance + Coût réduit 10% |
| 7 | 1 200 s | +5% puissance |
| 8 | 1 400 s | +5% rayon |
| 9 | 1 600 s | +5% puissance |
| 10 | 1 800 s | +10% puissance + Forme Maîtrisée |

Rang maximum : 10. Secondes cumulées à rang 10 : 10 000 (environ 2h46 de combat actif).

#### Cap de compétences

Le nombre total de **points de skill** dépensés (points de niveau + points gagnés par l'usage en termes de "slots de skill disponibles") est limité. La formule :

```
cap_skill_actif(niveau) = floor(niveau / 2) * 20
```

Exemples :
- Niveau 10 : cap = 100 points
- Niveau 20 : cap = 200 points
- Niveau 30 : cap = 300 points
- Niveau 60 : cap = 600 points

Chaque skill actif coûte de 1 à 5 points de skill pour être mis en barre (coût selon la puissance du skill). Les passifs et auras coûtent 1 à 3 points. Ce cap force la spécialisation : un personnage ne peut pas maximiser 40 skills simultanément.

**Slots de skill en barre d'action :** 12 slots actifs, 6 slots passifs/aura permanents. Les skills connus mais non en barre ne progressent pas (pas d'usage possible).

#### Anti-farm — Diminishing Returns

Si le même skill est utilisé plus de **15 fois contre la même cible** dans un même combat, les usages supplémentaires ne sont comptabilisés qu'à 20% de leur valeur normale. Ce compteur se réinitialise à chaque changement de cible ou à la fin du combat.

Si le même skill est utilisé contre des cibles de niveau inférieur de plus de **10 niveaux**, les usages ne sont comptabilisés qu'à 10%.

Les usages en PvP (contre joueurs) sont comptabilisés à 150% de leur valeur normale — le PvP est la voie la plus rapide de progression des skills.

---

### 1.3 Reset et respec

#### Philosophie

Le respec est possible mais coûteux. Il est conçu pour permettre à un joueur qui a fait une erreur de build de se corriger, pas pour switcher de build à volonté chaque jour. La "Mémoire de build" atténue la friction pour les joueurs qui testent et alternent entre deux ou trois builds.

#### Coût de reset de stats (points de niveau)

| Points à rembourser | Coût en Or Stellaire | Coût en temps |
|--------------------|----------------------|---------------|
| 1-10 points | 500 po | Instantané |
| 11-30 points | 2 000 po | Instantané |
| 31-60 points | 8 000 po | 24h de cooldown après |
| 61-100 points | 25 000 po | 72h de cooldown + quête |
| 101+ points | Quête de respec profond | 1 semaine + quête + NPC spécial |

L'"Or Stellaire" est l'or standard du jeu (pièces d'or imprimées avec l'étoile de Sorath — terme lore).

#### Coût de reset de skill usage (rangs de skill)

Le reset des rangs de skill ne remboursent pas les usages passés — ils réinitialisent simplement le rang au niveau choisi, effaçant les features débloquées mais rendant les points de skill dépensés dans ce slot disponibles pour d'autres skills.

| Rangs à réinitialiser (par skill) | Coût en Or Stellaire |
|----------------------------------|----------------------|
| Rang 1-5 | 200 po par rang |
| Rang 6-10 | 500 po par rang |
| Rang 11-15 | 1 200 po par rang |
| Rang 16-20 | 3 000 po par rang |

La réinitialisation d'un skill de rang 20 coûte :
- (5 × 200) + (5 × 500) + (5 × 1 200) + (5 × 3 000) = 1 000 + 2 500 + 6 000 + 15 000 = **24 500 po**

#### Limite de respec par semaine

| Type de respec | Limite hebdomadaire |
|----------------|---------------------|
| Reset de stats léger (≤10 points) | Illimité |
| Reset de stats modéré (11-60 points) | 3 fois |
| Reset de stats profond (61-100 points) | 1 fois |
| Reset de stats total | Uniquement via quête (1/mois) |
| Reset de skill (1-3 rangs) | Illimité |
| Reset de skill (4-10 rangs) | 5 fois |
| Reset de skill total d'un skill | 2 fois |

#### Quête de respec profond

Accessible au niveau 20+. Elle replace le coût pécuniaire d'un reset massif par un engagement narratif. La quête varie selon la faction :

- **Empire Pourpre :** "La Refonte" — se présenter devant le Tribunal Martial de Velanthara, reconnaître ses erreurs devant 3 NPCs importants, accomplir une mission de rédemption (escorter un convoi en zone de guerre sans mourir)
- **Alliance de Rive :** "Le Nouveau Cap" — consulter l'Oracle de Caraveth, traverser trois zones de brume pour atteindre le Phare de l'Oubli, déposer un artefact personnel dans les eaux
- **Fédération Ervan :** "Le Rêve de la Racine" — entrer en transe dans un Nexus de rang 3+, naviguer dans le plan onirique de Gaïa, refaire symboliquement ses choix devant le Conseil des Voix
- **Outlaws :** "L'Ardoise" — payer en monnaie de contrebande, pas en or standard, et éliminer un ennemi désigné par la Confrérie comme prix de l'"oubli"

#### Mémoire de build

Le système sauvegarde automatiquement les **3 derniers états complets** du personnage (répartition de stats + skills actifs en barre). Switcher vers un build mémorisé nécessite :
- Si le build mémorisé a les mêmes skills connus et la même répartition de stats : **500 po + 30 secondes hors combat** (retrait de l'armure, méditation)
- Si le build mémorisé nécessite de modifier les rangs de skills : coût de reset normal appliqué sur les rangs différents uniquement

Les 3 slots de mémoire sont toujours disponibles. Un 4ème slot peut être débloqué via la Guilde des Aventuriers (rang Compagnon).

---

### 1.4 Système paragon post-60

#### Philosophie

Le Paragon d'Allumina est conçu pour donner aux joueurs endgame un sentiment de progression continue sans créer d'écart de puissance rédhibitoire. Il est **plafonné en puissance** mais **illimité en profondeur cosmétique et narrative**.

#### Paragon points par niveau paragon

Chaque niveau Paragon requiert :
```
xp_paragon(n) = 200 000 * n ^ 1.2
```

À chaque niveau Paragon, le joueur reçoit **1 point Paragon**.

#### Arbre Paragon — structure

L'arbre Paragon est organisé en trois branches. Le joueur peut investir librement entre les branches, mais chaque branche a un cap individuel de **50 points** pour un cap total de **150 points Paragon** efficaces en termes de puissance. Au-delà de 150 points, seule la branche Cosmétique est disponible (pas de plafond).

**Branche Acier (stats offensives secondaires)**

| Investissement | Bonus |
|----------------|-------|
| 1 point | +0,2% dégâts physiques |
| 1 point | +0,2% dégâts magiques |
| 1 point | +0,15% chance de critique |
| 1 point | +0,3% vitesse d'attaque |
| Cap à 50 points | +10% dégâts, +7,5% crit, +15% ASPD |

**Branche Écorce (stats défensives secondaires)**

| Investissement | Bonus |
|----------------|-------|
| 1 point | +0,3% vie maximale |
| 1 point | +0,2% défense physique |
| 1 point | +0,1% résistances élémentaires (toutes) |
| 1 point | +0,15% régénération de vie |
| Cap à 50 points | +15% vie, +10% défense, +5% résist, +7,5% regen |

**Branche Éther (stats de support/hybrides)**

| Investissement | Bonus |
|----------------|-------|
| 1 point | +0,25% mana maximale |
| 1 point | +0,15% réduction de coût de skill |
| 1 point | +0,2% vitesse de déplacement (cap à +5% via Paragon) |
| 1 point | +0,1% réduction de cooldown |
| Cap à 50 points | +12,5% mana, +7,5% réduction coût, +5% MS, +5% CDR |

**Branche Cosmos (au-delà du cap de puissance — cosmétique et narratif)**

Chaque 10 points Paragon au-delà de 150 débloque un choix parmi :
- Titre de Paragon unique (visible au-dessus du personnage) — "Éveillé de Sorath", "Gardien des Veines", "Bras de l'Équilibre"...
- Aura cosmétique de personnage (subtile, non agressive visuellement)
- Entrée dans le Hall of Fame de Paragon (classement public des personnages les plus avancés)
- Accès à des quêtes narratives "Paragon" — dialogues étendus avec les NPCs majeurs révélant des secrets de lore
- Décoration exclusive de housing (bannières, sculptures)

---

## 2. Stats du personnage

### 2.1 Stats primaires — Les Quatre Piliers

#### Nomenclature lore

En Véranthas, les quatre forces fondamentales qui définissent la puissance d'un être vivant sont théorisées par les érudits de la Fédération Ervan sous le nom des **Quatre Piliers de l'Âme**. Cette terminologie, issue des Mémoires Vertes, a été adoptée progressivement par toutes les factions — même l'Empire Pourpre utilise ces termes dans ses manuels militaires, en les rebaptisant pragmatiquement "les Quatre Qualités du Légionnaire".

Les noms officiels des stats primaires dans Allumina sont en langue proto-ervan (langue savante du monde) avec leur traduction commune :

---

#### FERRATH — La Force (stat primaire)

**Nom complet :** *Ferrath-Anth* — "L'Ancrage dans la Terre"

**Description lore :** Ferrath est la capacité d'un être à agir dans le monde physique. Les druides ervans enseignent que Ferrath est la résonnance du personnage avec la couche physique de Véranthas — plus un être est ancré, plus il peut frapper fort, porter lourd, et résister à l'impulsion de fuir.

| Métrique | Formule |
|----------|---------|
| Dégâts physiques | `base_dégâts_physiques * (1 + Ferrath * 0.003)` |
| Dégâts des armes lourdes (haches, masses) | `bonus supplémentaire : Ferrath * 0.002` |
| Capacité de charge | `100 + Ferrath * 5` (kg) |
| Prérequis équipements | Armures lourdes requièrent Ferrath ≥ seuil |

**Classes qui bénéficient le plus de Ferrath :**

Empire Pourpre : Légionnaire de Cramoisy, Centurion de Siège, Gladiateur de l'Arène
Alliance de Rive : Corsaire Abordeur, Colosse de Rade
Fédération Ervan : Gardien de Nexus, Berserker Sylvain

**Seuils de breakpoint Ferrath :**

| Seuil | Effet de palier |
|-------|----------------|
| 50 | Débloque les armures de plaque lourde |
| 100 | Bonus "Frappe Terrifiante" : chaque frappe physique a 5% de chance d'appliquer 1 stack de Découragement sur l'ennemi |
| 150 | Capacité de charge doublée, bonus de dégâts +5% passif permanent |
| 200 | Débloque la variante avancée de toutes les skills physiques passives |
| 250 | "Corps de Granit" : réduction passive de 3% des dégâts physiques reçus |

---

#### VELTHAR — L'Agilité (stat primaire)

**Nom complet :** *Velthar-Sorath* — "Le Souffle du Vent sous Sorath"

**Description lore :** Velthar est la vitesse de la pensée et du corps — la capacité de réagir avant que l'ennemi ne comprenne que l'attaque est venue. Les acrobates de l'Alliance de Rive et les assassins de la Confrérie de l'Ombre Libre l'élèvent comme la plus noble des qualités. L'Empire Pourpre le respecte chez ses éclaireurs mais le méprise comme vertu principale — "un légionnaire qui doit fuir est déjà mort".

| Métrique | Formule |
|----------|---------|
| Vitesse d'attaque | Voir breakpoints ASPD section 2.2 — Velthar divise le délai entre attaques |
| Esquive (dodge rating) | `Velthar * 0.15` (%) cap à 40% |
| Dégâts critiques (bonus) | `Velthar * 0.1` (% de bonus sur multiplicateur crit) |
| Portée de déplacement dash | `base_dash + Velthar * 0.05 m` |

**Classes qui bénéficient le plus de Velthar :**

Empire Pourpre : Éclaireur Impérial, Arcaniste de Mécanique (utilise Velthar pour le timing d'activation)
Alliance de Rive : Corsaire Abordeur, Flibustier des Brumes, Tireur de Haute-Mer
Fédération Ervan : Traqueur Sylvain, Danseur de Nexus (classe hybride)
Outlaws : Crocheur d'Ombre, Silhouette (classe furtivité)

**Seuils de breakpoint Velthar :**

| Seuil | Effet de palier |
|-------|----------------|
| 50 | Vitesse de déplacement +5% passive |
| 100 | Débloque les armures de cuir avancé — et double la fenêtre de parade (parry window) |
| 150 | Esquive "Active" : une fois par 30s, esquive automatique du prochain coup direct (non AoE) |
| 200 | Attaques critiques appliquent "Déséquilibre" sur l'ennemi pendant 2s (réduit sa vitesse d'attaque de 15%) |
| 250 | "Réflexes de l'Éclipse" : vitesse d'attaque maximale (breakpoint ASPD le plus rapide) débloquée |

---

#### GAÏATHAR — l'Intelligence (stat primaire)

**Nom complet :** *Gaïathar-Nexis* — "La Communion avec les Nexus"

**Description lore :** Gaïathar est le nom ervan de la capacité à canaliser les énergies des Nexus — ce que l'Empire appelle "puissance arcaniste" et l'Alliance nomme "talent mercantile de la persuasion magique". En termes de jeu, c'est la stat de toute forme de magie, de sort, de capacité mentale ou d'énergie non physique.

| Métrique | Formule |
|----------|---------|
| Puissance magique | `base_dégâts_magiques * (1 + Gaïathar * 0.004)` |
| Mana maximale | `base_mana_classe + Gaïathar * 8` |
| Résistances magiques | `Gaïathar * 0.05` (%) cap à 30% via stat seule |
| Efficacité des soins | `base_soin * (1 + Gaïathar * 0.003)` |
| Durée des effets magiques | `durée_base * (1 + Gaïathar * 0.002)` |

**Classes qui bénéficient le plus de Gaïathar :**

Empire Pourpre : Arcaniste de Mécanique, Inscripteur de Runes
Alliance de Rive : Enchanteur de Mer, Alchimiste Corsaire
Fédération Ervan : Druide de Nexus (classe cœur de la Fédération), Tisseur de Vent, Voix Verte (support)
Outlaws : Nécrolicient de Vorakis (classe héritière de la tradition maudite)

**Seuils de breakpoint Gaïathar :**

| Seuil | Effet de palier |
|-------|----------------|
| 50 | Débloque les bâtons et orbes de rang intermédiaire |
| 100 | Mana se régénère à 1% par seconde hors combat (bonus additionnel) |
| 150 | "Résonance de Nexus" : en zone de Nexus ou en Convergence, puissance magique +10% |
| 200 | Les sorts de contrôle de foule durent 20% plus longtemps |
| 250 | "Voix de Gaïa" : les soins appliqués par le personnage ont 20% de chances de retirer un stack de corruption Garum de la cible |

---

#### RHATHAR — La Constitution (stat primaire)

**Nom complet :** *Rhathar-Veines* — "La Solidité des Veines" (les veines du corps, pas les Veines Grises)

**Description lore :** Rhathar est la vitalité brute, la résistance à la douleur et à la maladie. Le nom choisi par la Fédération est intentionnellement ambigu : les "Veines" désignent à la fois les veines sanguines du corps (vitalité) et les Veines Grises du monde (résistance à la corruption). Un Rhathar élevé signifie que le personnage est difficile à corrompre — et difficile à tuer.

| Métrique | Formule |
|----------|---------|
| Points de vie maximaux | `base_vie_classe + Rhathar * 12` |
| Régénération de vie | `0.5% de la vie max par 5s hors combat + Rhathar * 0.02%` |
| Résistance physique | `Rhathar * 0.04` (%) cap à 20% via stat seule |
| Résistance à la corruption | `Rhathar * 0.06` (%) — réduit la durée des effets de Garum reçus |
| Durée des effets négatifs | `(1 - Rhathar * 0.002)` (multiplicateur, minimum 60% de la durée de base) |

**Classes qui bénéficient le plus de Rhathar :**

Empire Pourpre : Légionnaire de Cramoisy, Vétéran de Guerre (classe tank endgame)
Alliance de Rive : Colosse de Rade, Capitaine de Guerre Navale
Fédération Ervan : Gardien de Nexus, Berserker Sylvain
Outlaws : Pillard de Veine (classe tank survie des Outlaws)

**Seuils de breakpoint Rhathar :**

| Seuil | Effet de palier |
|-------|----------------|
| 50 | Régénération de vie active en combat léger (contre créatures de -5 niveaux) |
| 100 | "Endurance Primale" : la première fois sous 20% de vie par combat, absorbe un coup pour 0 dégâts (cooldown 5 min) |
| 150 | Résistance à la corruption portée à cap étendu (40% max au lieu de 30%) |
| 200 | "Constitution de Pierre" : les effets d'étourdissement ont leur durée réduite de 50% |
| 250 | "Veine Pure" : le personnage ne peut pas être transformé ou contrôlé par des sorts de corruption de Garum rang 1-3 |

---

### 2.2 Stats dérivées

#### Vie (Points de vie — PV)

```
PV_max = base_vie_classe + (Rhathar * 12) + (bonus_équipements)
```

Bases de vie par archétype à niveau 1 :
- Archétypes tank (Légionnaire, Gardien de Nexus, Colosse) : 220 PV de base
- Archétypes hybrides (Druide, Arcaniste, Capitaine) : 170 PV de base
- Archétypes DPS/support (Traqueur, Flibustier, Voix Verte) : 140 PV de base
- Archétypes glass cannon (Nécrolicient, Enchanteur de Mer) : 110 PV de base

#### Mana

```
Mana_max = base_mana_classe + (Gaïathar * 8) + (bonus_équipements)
```

Bases de mana par archétype à niveau 1 :
- Casters purs (Druide de Nexus, Arcaniste, Enchanteur) : 200 Mana de base
- Hybrides (Capitaine, Traqueur Sylvain, Crocheur) : 120 Mana de base
- Physiques avec magie (Inscripteur, Gardien) : 80 Mana de base
- Physiques purs (Légionnaire, Colosse, Berserker) : 40 Mana de base

#### Defense Rating (DR)

Réduction des dégâts physiques entrants.

```
réduction_physique(%) = DR / (DR + 800) * 100
```

Exemple : DR 200 → 200 / 1000 = 20% de réduction.
Cap effectif via DR : 40% (nécessite DR = 5 333).
Cap avec équipements et Rhathar combinés : 60%.

DR est composé de :
- Armure équipée (principale contribution)
- Rhathar * 0.04 (%)
- Bonus d'équipement

#### Attack Rating (AR) et Chance de toucher

```
chance_de_toucher(%) = 50 + (AR_attaquant - DR_esquive_défenseur) / 20
```

Cap de chance de toucher : 95%.
Plancher : 5% (même le joueur le plus faible peut toucher, même le meilleur peut rater).

AR est composé de :
- Velthar * 1.5 (contribution principale pour les personnages agiles)
- Ferrath * 0.5 (contribution physique brute)
- Bonus d'arme et d'équipement

#### Vitesse d'attaque (ASPD) — Breakpoints discrets

Comme Diablo II, les attaques se produisent à des ticks de serveur discrets (le serveur tourne à **20 ticks par seconde**, soit des ticks de 50ms). La vitesse d'attaque est donc exprimée en **nombre de ticks entre deux attaques**.

| Tier ASPD | Ticks entre attaques | Attaques/seconde | Velthar requis (approximatif) |
|-----------|---------------------|------------------|-------------------------------|
| 1 (lent) | 60 ticks | 0,33 APS | — (armes à deux mains lourdes) |
| 2 | 50 ticks | 0,40 APS | — |
| 3 | 40 ticks | 0,50 APS | Velthar 20 ou arme légère |
| 4 | 34 ticks | 0,59 APS | Velthar 40 |
| 5 | 28 ticks | 0,71 APS | Velthar 60 |
| 6 | 24 ticks | 0,83 APS | Velthar 80 |
| 7 | 20 ticks | 1,00 APS | Velthar 100 |
| 8 | 17 ticks | 1,18 APS | Velthar 120 |
| 9 | 14 ticks | 1,43 APS | Velthar 150 |
| 10 | 12 ticks | 1,67 APS | Velthar 180 |
| 11 | 10 ticks | 2,00 APS | Velthar 200 |
| 12 (cap) | 8 ticks | 2,50 APS | Velthar 250 + palier breakpoint |

Les armes ont leur propre ASPD de base qui peut monter ou descendre selon le type. Une épée courte démarre à Tier 5, une hache à deux mains démarre à Tier 1, une dague à Tier 8.

#### Vitesse de déplacement (Mouvement Speed — MS)

```
MS = base_MS_classe + Velthar * 0.1 + bonus_équipement
```

Base MS :
- Archétypes légers (Flibustier, Crocheur, Traqueur) : 115
- Archétypes hybrides : 105
- Archétypes lourds (Légionnaire, Colosse) : 95

Cap de vitesse de déplacement via stats : 140 (sur une base de 100).
Les bonus de sprint/compétences peuvent pousser temporairement jusqu'à 200.

#### Régénération de vie et de mana

| Condition | Régénération de vie | Régénération de mana |
|-----------|---------------------|----------------------|
| Hors combat (>15s sans dégâts) | 2% PV max par 5s + Rhathar bonus | 3% Mana max par 5s + Gaïathar bonus |
| En combat | 0% (sauf équipements dédiés) | 0.5% Mana max par 5s (base) |
| En combat (palier Rhathar 50) | Léger: 0.5% PV par 5s | — |
| Assis (animation) | x2 sur la régénération hors combat | x2 |
| Zone de Nexus | +1% PV et Mana par 5s supplémentaire | +2% Mana par 5s |

#### Chance de critique

```
crit_chance(%) = 2 + (Velthar * 0.05) + bonus_équipement
```

Base de départ : 2%.
Cap via stats et équipement en PvE : 60%.
Cap en PvP : 35% (cap réduit pour éviter les builds one-shot).

#### Dégâts critiques (multiplicateur)

```
multiplicateur_crit = 1.5 + (Velthar * 0.001) + bonus_équipement
```

Base : ×1.5 (soit +50% de dégâts sur un crit).
Maximum via Velthar (Velthar 250) : ×1.75.
Avec équipements : cap à ×2.5 en PvE, ×2.0 en PvP.

---

### 2.3 Résistances — Les Cinq Voiles

#### Nomenclature lore

Les cinq forces de résistance correspondent aux cinq types de dommages fondamentaux que les druides ervans ont identifiés dans les Mémoires Vertes. Chaque résistance est nommée d'après le "Voile" qu'elle représente — la couche de l'âme qui protège contre cette force.

---

**Voile de Pierre — Résistance Physique**

Protège contre les dégâts physiques (coups directs, projectiles, chutes).

```
réduction_physique = (résistance_physique%) appliquée après le Defense Rating
```

La résistance physique est complémentaire au DR — les deux s'appliquent successivement.

---

**Voile de Braise — Résistance au Feu (Brasier)**

Protège contre les dégâts de feu — sorts de mages, engins de guerre incendiaires de l'Empire Pourpre (huile bouillante, balistes enflammées), certains monstres de zones arides.

```
réduction_feu(%) = résistance_feu_brute capped
```

---

**Voile de Gel — Résistance au Givre (Voile de l'Hiver)**

Protège contre les dégâts de glace et les effets de ralentissement associés. Résistance au givre réduit à la fois les dégâts et la durée des effets de slow.

```
réduction_givre(%) = résistance_givre * 0.7  (dégâts)
réduction_durée_slow(%) = résistance_givre * 0.3  (effets)
```

---

**Voile de l'Étincelle — Résistance à la Foudre (Arc de Sorath)**

Protège contre les dégâts de foudre. La foudre a souvent des effets de chaîne (dommages en AoE rebond) — la résistance réduit à la fois les dégâts directs et les dégâts de chaîne.

```
réduction_foudre(%) = résistance_foudre * 0.8  (dégâts directs)
réduction_chaîne(%) = résistance_foudre * 0.4  (dommages de rebond seulement)
```

---

**Voile de Cendre — Résistance à la Corruption (Érosion de Garum)**

La résistance la plus importante pour le endgame. La Corruption de Garum ne tue pas directement — elle inflige des effets de dégradation : ralentissement progressif, réduction des dégâts infligés, transformation éventuelle en Eraillé si non soignée.

```
réduction_corruption(%) = résistance_corruption
durée_effets_corruption(%) = max(40%, 100% - résistance_corruption * 0.6)
```

La résistance à la Corruption est aussi appelée **Pureté de Veine** dans le jargon de jeu. C'est la stat la plus difficile à maxer et la plus recherchée en endgame.

---

#### Tableau des caps de résistance

| Résistance | Cap PvE | Cap PvP | Comment augmenter |
|------------|---------|---------|-------------------|
| Physique | 80% | 60% | Armures, Rhathar, certains buff de druide |
| Brasier | 75% | 50% | Équipements, potions, enchantements |
| Givre | 75% | 50% | Équipements, potions, enchantements |
| Foudre | 75% | 50% | Équipements, potions, enchantements |
| Corruption | 70% | 40% | Équipements de Gaïathar, reliques de Nexus, Rhathar palier 150 |

#### Pénalités de résistance par difficulté de zone

| Zone / Difficulté | Résistances physiques | Résistances élémentaires | Résistance corruption |
|-------------------|----------------------|--------------------------|-----------------------|
| Zone normale (niveaux 1-20) | 0% | 0% | 0% |
| Zone de Convergence (30+) | -10% | -15% | -20% |
| Zone de guerre active | -15% | -15% | -15% |
| Donjon normal | 0% | -10% | -10% |
| Donjon héroïque | -20% | -25% | -30% |
| Stampede Phase 3-4 | -25% | -20% | -40% |
| Voix de l'Érosion (boss endgame) | -30% | -35% | -50% |

Ces pénalités s'appliquent après le cap — elles peuvent rendre effective une résistance négative, augmentant les dégâts reçus au-delà des dégâts de base.

#### Immunités

Certaines créatures de Garum sont **immunisées** à la Corruption (ils sont la Corruption). Les boss de Stampede phase 3+ ont souvent une immunité ou une très haute résistance à un élément.

| Type de créature | Immunités |
|-----------------|-----------|
| Éraillés (corruption basique) | Corruption 100% |
| Gardiens de Nexus corrompus | Corruption 100%, Givre partiel (-50%) |
| Créatures de la Mer de Cendre | Corruption 100%, Givre 100% |
| Automates de guerre impériaux (PNJ alliés/ennemis) | Corruption 75%, Brasier partiellement |
| Créatures des Archipels de Brume | Foudre 100% |
| Voix de l'Érosion | Corruption 100% + une immunité élémentaire selon la Voix |

---

### 2.4 Tableau stats de base par classe

Le tableau suivant indique les stats à **niveau 1** (après distribution des 5 points de départ selon le profil recommandé) et le **gain par niveau** (arrondi). Ces valeurs sont les valeurs de départ avant équipement.

**Notation :** F=Ferrath / V=Velthar / G=Gaïathar / R=Rhathar / PV=vie base / Mana=mana base

#### Empire Pourpre — 9 Classes

| Classe | F | V | G | R | PV base | Mana base | Gain F/niv | Gain V/niv | Gain G/niv | Gain R/niv |
|--------|---|---|---|---|---------|-----------|-----------|-----------|-----------|-----------|
| Légionnaire de Cramoisy | 12 | 6 | 4 | 10 | 220 | 40 | 2.0 | 1.0 | 0.5 | 1.5 |
| Éclaireur Impérial | 6 | 14 | 5 | 7 | 155 | 80 | 1.0 | 2.5 | 0.5 | 1.0 |
| Arcaniste de Mécanique | 5 | 8 | 14 | 5 | 135 | 190 | 0.5 | 1.0 | 2.5 | 0.5 |
| Inscripteur de Runes | 4 | 6 | 16 | 6 | 125 | 200 | 0.5 | 1.0 | 3.0 | 0.5 |
| Vétéran de Guerre | 14 | 5 | 3 | 10 | 230 | 40 | 2.5 | 0.5 | 0.5 | 1.5 |
| Gladiateur de l'Arène | 12 | 10 | 4 | 6 | 180 | 50 | 2.0 | 2.0 | 0.5 | 0.5 |
| Médecin de Campagne | 5 | 7 | 12 | 8 | 160 | 160 | 0.5 | 1.0 | 2.0 | 1.0 |
| Centurion de Siège | 10 | 6 | 6 | 10 | 200 | 80 | 1.5 | 1.0 | 1.0 | 2.0 |
| Stratège Impérial | 7 | 8 | 10 | 7 | 165 | 140 | 1.0 | 1.5 | 2.0 | 0.5 |

#### Alliance de Rive — 9 Classes

| Classe | F | V | G | R | PV base | Mana base | Gain F/niv | Gain V/niv | Gain G/niv | Gain R/niv |
|--------|---|---|---|---|---------|-----------|-----------|-----------|-----------|-----------|
| Corsaire Abordeur | 10 | 12 | 4 | 6 | 175 | 60 | 1.5 | 2.0 | 0.5 | 1.0 |
| Flibustier des Brumes | 6 | 14 | 6 | 6 | 150 | 90 | 1.0 | 2.5 | 1.0 | 0.5 |
| Tireur de Haute-Mer | 5 | 14 | 7 | 6 | 145 | 100 | 0.5 | 2.5 | 1.0 | 1.0 |
| Enchanteur de Mer | 4 | 7 | 16 | 5 | 120 | 210 | 0.5 | 1.0 | 3.0 | 0.5 |
| Capitaine de Guerre Navale | 9 | 9 | 8 | 6 | 170 | 110 | 1.5 | 1.5 | 1.5 | 0.5 |
| Alchimiste Corsaire | 5 | 8 | 14 | 5 | 130 | 185 | 0.5 | 1.0 | 2.5 | 0.5 |
| Colosse de Rade | 13 | 5 | 3 | 11 | 225 | 40 | 2.5 | 0.5 | 0.5 | 1.5 |
| Marchande-Lame | 8 | 10 | 10 | 4 | 155 | 130 | 1.0 | 2.0 | 2.0 | 0 |
| Batteur de Brume | 6 | 12 | 8 | 6 | 155 | 110 | 1.0 | 2.0 | 1.5 | 0.5 |

#### Fédération Ervan — 9 Classes

| Classe | F | V | G | R | PV base | Mana base | Gain F/niv | Gain V/niv | Gain G/niv | Gain R/niv |
|--------|---|---|---|---|---------|-----------|-----------|-----------|-----------|-----------|
| Druide de Nexus | 4 | 6 | 17 | 5 | 125 | 215 | 0.5 | 1.0 | 3.0 | 0.5 |
| Gardien de Nexus | 10 | 6 | 8 | 8 | 195 | 100 | 1.5 | 1.0 | 1.5 | 2.0 |
| Traqueur Sylvain | 6 | 14 | 6 | 6 | 150 | 85 | 1.0 | 2.5 | 1.0 | 0.5 |
| Berserker Sylvain | 13 | 8 | 3 | 8 | 205 | 45 | 2.5 | 1.5 | 0.5 | 0.5 |
| Voix Verte | 5 | 7 | 14 | 6 | 140 | 180 | 0.5 | 1.0 | 2.5 | 1.0 |
| Tisseur de Vent | 4 | 9 | 14 | 5 | 130 | 190 | 0.5 | 1.5 | 2.5 | 0.5 |
| Danseur de Nexus | 6 | 12 | 10 | 4 | 150 | 145 | 1.0 | 2.0 | 2.0 | 0 |
| Mémoire Vivante | 4 | 5 | 16 | 7 | 140 | 195 | 0.5 | 0.5 | 2.5 | 1.5 |
| Gardien-Racine | 11 | 5 | 7 | 9 | 200 | 90 | 2.0 | 0.5 | 1.5 | 1.0 |

#### Outlaws (Confrérie de l'Ombre Libre) — 6 Classes

| Classe | F | V | G | R | PV base | Mana base | Gain F/niv | Gain V/niv | Gain G/niv | Gain R/niv |
|--------|---|---|---|---|---------|-----------|-----------|-----------|-----------|-----------|
| Crocheur d'Ombre | 5 | 15 | 7 | 5 | 145 | 95 | 0.5 | 3.0 | 1.0 | 0.5 |
| Pillard de Veine | 12 | 6 | 4 | 10 | 215 | 45 | 2.0 | 1.0 | 0.5 | 1.5 |
| Nécrolicient de Vorakis | 4 | 5 | 17 | 6 | 120 | 220 | 0.5 | 0.5 | 3.0 | 1.0 |
| Silhouette | 4 | 17 | 5 | 6 | 140 | 80 | 0.5 | 3.0 | 0.5 | 1.0 |
| Contrebandier Armé | 8 | 10 | 6 | 8 | 170 | 90 | 1.5 | 2.0 | 1.0 | 0.5 |
| Mercenaire Renégat | 10 | 10 | 4 | 8 | 180 | 60 | 2.0 | 2.0 | 0.5 | 0.5 |

---

## 3. Équipement

### 3.1 Slots d'équipement

Allumina utilise **12 slots d'équipement** + 2 slots d'anneaux. Le total est de **14 pièces d'équipement** portées simultanément.

| Slot | Nom affiché | Description | Types acceptés | Prérequis minimal |
|------|-------------|-------------|----------------|-------------------|
| 1 | Heaume | Protection de la tête | Casques, heaumes, chapeaux, capuches, couronnes | Niveau 1 |
| 2 | Plastron | Armure de torse | Cottes de maille, plastrons, robes, tuniques, vestes cuir | Niveau 1 |
| 3 | Jambières | Protection des jambes | Jambières de plaque, pantalons renforcés, bas de robe | Niveau 1 |
| 4 | Sollerets | Protection des pieds | Bottes de plaque, bottes de cuir, sandales, sabots enchantés | Niveau 1 |
| 5 | Gantlets | Protection des mains | Gantlets de plaque, brassards de cuir, gants de tissu | Niveau 1 |
| 6 | Ceinturon | Protection de la taille | Ceintures de plaque, sangles de cuir, écharpes de tissu | Niveau 1 |
| 7 | Main directrice | Arme principale | Épées 1M, haches 1M, masses, bâtons, arcs, arbalètes, dagues, épées 2M, haches 2M | Niveau 1 |
| 8 | Main de soutien | Arme ou protection secondaire | Boucliers (toutes tailles), pareurs, carquois, orbes, grimoires, dagues off-hand | Niveau 1 |
| 9 | Anneau gauche | Anneau | Tous anneaux | Niveau 5 |
| 10 | Anneau droit | Anneau | Tous anneaux (stack d'effets limité) | Niveau 5 |
| 11 | Pendentif | Amulette/relique | Amulettes, médaillons, reliques de Nexus, fragments de Veine | Niveau 5 |
| 12 | Manteau | Cape ou manteau | Capes légères, manteaux de voyage, manteaux d'armure | Niveau 10 |
| 13 | Monture | Monture équipée | Chevaux, destriers impériaux, cerfs ervans, krakens de mer (Alliance), montures Garum (Outlaws endgame) | Niveau 15 |
| 14 | Talisman | Slot bonus (débloqué) | Talismans de faction, artefacts de donjons | Niveau 30 |

**Note sur la main de soutien :**
- Un arc ou une arbalète en main directrice **verrouille** la main de soutien sur carquois uniquement
- Les armes à deux mains (épées 2M, haches 2M, bâtons à deux mains) **occupent les deux slots** (7 et 8)
- Une dague peut être tenue en main de soutien avec la plupart des armes 1M (dual wield)

---

### 3.2 Qualités d'équipement

| Qualité | Couleur (UI) | Nombre d'affixes | Niveau d'obtention | Méthodes principales |
|---------|-------------|------------------|--------------------|----------------------|
| **Commun** | Gris | 0-1 | 1-60 | Loot basique, marchands NPC |
| **Peu commun** | Vert | 1-2 | 5-60 | Loot de monstres standard, crafting débutant |
| **Rare** | Bleu | 2-3 | 15-60 | Donjons normaux, boss de zone, crafting intermédiaire |
| **Épique** | Violet | 3-4 | 30-60 | Donjons héroïques, boss Convergence, crafting avancé |
| **Légendaire** | Or | 4-5 | 45-60 | Raids, Voix de l'Érosion, crafting maître de guilde |
| **Artefact de Faction** | Rouge brillant + couleur faction | 5-6 + effet unique | 55-60 | Événements saisonniers, sièges victorieux, Grande Convergence |

**Règles d'affixes :**
- Les affixes sont toujours générés dans un range de valeur (min/max) selon le niveau de l'objet
- Un objet Épique ou supérieur peut avoir un **affix "scellé"** — non identifiable avant d'être utilisé par un Archiviste de guilde
- Les Artefacts de Faction ont toujours **un effet unique** non replicable sur d'autres qualités

---

### 3.3 Sets d'équipement

Trois exemples canoniques, un par faction principale.

---

#### SET 1 — "Armure du Légat de Cramoisy" (Empire Pourpre)

**Type :** Lourd (Plaque)
**Tier recommandé :** Niveau 45-55
**Obtention :** Donjons héroïques impériaux (Citadelle de Varenkor héroïque), drops de boss d'élite

Pièces : Heaume du Légat, Plastron du Légat, Jambières du Légat, Gantlets du Légat, Sollerets du Légat, Ceinturon du Légat

| Bonus | Condition |
|-------|-----------|
| +15% dégâts physiques | 2 pièces |
| +20% vie maximale, +10% Defense Rating | 4 pièces |
| Actif "Discipline de Cramoisy" : 1/min, cri de guerre qui augmente ASPD du groupe de +10% pendant 15s | 6 pièces |

---

#### SET 2 — "Habit de la Marée Libre" (Alliance de Rive)

**Type :** Cuir (Léger)
**Tier recommandé :** Niveau 45-55
**Obtention :** Commerce maritime de haut niveau, donjons navals (Archipel des Brumes), crafting Maître Cordier

Pièces : Chapeau de la Marée, Veste de la Marée, Pantalon de la Marée, Bottes de la Marée, Brassards de la Marée, Ceinturon de la Marée

| Bonus | Condition |
|-------|-----------|
| +12% Velthar, +8% esquive | 2 pièces |
| +15% dégâts avec armes de finesse (épées légères, dagues, arbalètes) | 4 pièces |
| Actif "Libertés des Eaux" : 1/2min, téléportation courte (10m) + 3s d'invulnérabilité aux dégâts de zone | 6 pièces |

---

#### SET 3 — "Vêtements du Tisserand de Nexus" (Fédération Ervan)

**Type :** Tissu (Très léger)
**Tier recommandé :** Niveau 45-55
**Obtention :** Récompenses du Conseil des Voix (rang 6), crafting Druide Maître, boss de Nexus endgame

Pièces : Tiare du Tisserand, Robe du Tisserand, Bas du Tisserand, Sandales du Tisserand, Gants du Tisserand, Écharpe du Tisserand

| Bonus | Condition |
|-------|-----------|
| +20% puissance magique, +15% mana maximale | 2 pièces |
| Les soins infligent aussi 30% de leur valeur en réduction de durée de corruption sur la cible soignée | 4 pièces |
| Actif "Voix de Gaïa Amplifiée" : passive permanente — 10% des dégâts magiques infligés soignent le porteur | 6 pièces |

---

### 3.4 Prérequis d'équipement

#### Prérequis de stat

| Tier d'armure | Ferrath minimum | Rhathar minimum | Notes |
|---------------|-----------------|-----------------|-------|
| Tissu rang 1 | — | — | Ouvert à tous |
| Cuir rang 1 | — | — | Ouvert à tous |
| Maille rang 1 | 20 | — | |
| Plaque légère rang 1 | 40 | 15 | |
| Plaque lourde rang 1 | Ferrath 50 | Rhathar 20 | Palier breakpoint F50 requis |
| Armure de campagne rang 5+ | 80 | 40 | |
| Armure héroïque rang 1 | 120 | 60 | |
| Armure de siège (endgame) | 160 | 80 | Uniquement plaques impériales |

| Tier d'arme | Ferrath min | Velthar min | Gaïathar min | Notes |
|-------------|-------------|-------------|--------------|-------|
| Arme 1M légère | — | 10 | — | Dagues, rapières |
| Arme 1M standard | 15 | — | — | Épées courtes, masses légères |
| Arme 1M lourde | 30 | — | — | Haches de guerre, masses lourdes |
| Arme 2M légère | 25 | 15 | — | Bâtons de combat, lances |
| Arme 2M lourde | 50 | — | — | Haches à deux mains, espadons |
| Arc | 10 | 20 | — | Force de tir proportionnelle au Velthar |
| Arbalète | 20 | 15 | — | |
| Bâton magique 1M | — | — | 20 | |
| Bâton magique 2M | — | — | 35 | |
| Orbe / Grimoire | — | — | 25 | |
| Arme de faction (rang 45+) | Selon classe | Selon classe | Selon classe | Varie |

#### Prérequis de classe et faction

Certains équipements sont réservés à des classes ou des factions spécifiques :

- **Équipements de faction** (qualité Artefact de Faction) : réservés aux membres actifs de la faction concernée. Un personnage ayant changé de faction ne peut plus équiper ses anciens artefacts.
- **Équipements de classe** (qualité Épique et supérieure avec l'étiquette "lié à la classe") : réservés à une classe ou une famille de classes. Exemple : "Bâton de la Voix Verte" réservé aux Druides de Nexus et Voix Vertes.
- **Équipements de set de faction** (comme les exemples ci-dessus) : réservés à la faction associée.

#### Prérequis de niveau

| Qualité | Niveau minimum pour équiper |
|---------|----------------------------|
| Commun | 1 |
| Peu commun | 5 |
| Rare | 15 |
| Épique | 30 |
| Légendaire | 45 |
| Artefact de Faction | 55 |

Les équipements de set n'ont pas de niveau minimum propre — c'est la qualité de chaque pièce qui détermine le minimum. Un set "Niveau 45-55" est composé de pièces entre Légendaire et Artefact de Faction.

---

## 4. Mort et conséquences

### 4.1 Règles de mort en zone normale

#### Définition de "zone normale"

Zone normale = tout espace de jeu **hors** zone de guerre active (RvR), hors instance de donjon, hors événement Stampede. Cela inclut les campagnes de faction 1-30, les zones exploratoires, les routes commerciales en temps de paix relative.

#### Ce qui se passe à la mort

1. **Animation de mort** (1,5 secondes) — le personnage s'effondre, les ennemis l'ignorent désormais
2. **Génération du corps** — le corps du personnage apparaît à l'emplacement exact de la mort
3. **Durée du corps** — **5 minutes** (300 secondes) après quoi le corps disparaît, et les objets non récupérés sont perdus définitivement
4. **Pillage du corps** — n'importe quel joueur (même allié) peut fouiller et prendre des objets du corps
5. **Objet protégé** — exactement **1 slot d'objet** peut être marqué "protégé" par le joueur (depuis l'interface d'inventaire, option "Objet chéri"). L'objet protégé ne drope jamais sur le corps
6. **Notification au tueur** — si la mort est causée par un autre joueur (PvP), le tueur reçoit une notification textuelle : *"Vous avez tué [Nom du personnage], [Classe], [Faction]. Ses biens reposent sur son corps."*

#### Ce qui drope exactement

- Tous les équipements portés (sauf l'objet protégé)
- Tout l'or transporté (pas l'or en banque)
- Tous les items d'inventaire (matériaux, potions, quêtes non-liées)
- Les items de quête "liés" ne dropent pas — ils restent dans l'inventaire

**Ce qui NE drope jamais (quelles que soient les circonstances) :**
- L'objet protégé (1 par personnage)
- Les items marqués "âme liée" par le jeu (certains dons de faction, items de tutoriel)
- Les items de quête narrative principale

---

### 4.2 Règles de mort en zone de guerre

#### Définition de "zone de guerre"

Zone de guerre = zone RvR active (Plateau de Velharris, Marches Brûlées, Détroit de la Mer Centrale), zone de siège, zone d'opération des Caravaniers en temps de guerre.

#### Règles spécifiques

Les règles de zone normale s'appliquent **plus** les éléments suivants :

- **Cargo de Caravanier intégralement pillable** — les cargaisons transportées par un Caravanier (profession civile) sont récupérables à 100% sur le corps en zone de guerre. En zone normale, seuls 60% du cargo sont récupérables (le reste est "perdu dans la confusion").
- **Durée du corps réduite à 3 minutes** (180 secondes) — la zone est active, la pression est maximale
- **Zone dangereuse autour du corps** — pendant 90 secondes après la mort, une icône dorée (visible sur la carte) indique la position du corps aux membres du groupe et aux ennemis
- **Pas de protection de faction** — même les gardes NPC de votre faction ignorent votre corps en zone de guerre active (ils sont occupés à combattre)
- **Bonus de loot pour le tueur** — en PvP de zone de guerre, le tueur reçoit +25% d'or sur les objets vendus depuis ce corps

---

### 4.3 Règles de mort en donjon instancié

#### Principes généraux

Les donjons instanciés ont une logique différente : la coopération de groupe est centrale, et le "plein loot" permanent dissuaderait la progression coopérative.

#### Règles

- **Corps récupérable dans la même instance** — tant que l'instance n'est pas terminée ou abandonnée
- **Drop réduit à 50% des items** — au lieu de dropper tous les items, seuls 50% (aléatoire) sont présents sur le corps. Les autres restent dans l'inventaire du personnage.
- **Résurrection par soigneur** — un soigneur du groupe peut ressusciter le personnage mort à son emplacement (consomme un "Cœur de Gaïa" — consommable rare de soin de groupe)
- **Résurrection au camp de base du donjon** — si aucun soigneur n'est disponible, le personnage mort respawn au camp de base de l'instance (entrée du donjon), doit courir à pied jusqu'à son groupe
- **Le corps disparaît si l'instance se termine** — les items sur le corps sont perdus si le groupe complète ou abandonne le donjon sans récupérer le corps

#### Mode Héroïque — règles spéciales

En mode héroïque (endgame), les règles sont plus sévères :
- Drop à 75% des items (plus punitif)
- Résurrection sur place uniquement si le boss actif n'est pas engagé
- Si le groupe essuyait une "mort de groupe" (TPK), l'instance se réinitialise au dernier checkpoint (pas retour au début)

---

### 4.4 Règles de mort en Stampede

#### Contexte

Les Stampedes sont des événements où **les conflits de faction sont officiellement suspendus**. La mort d'un joueur allié par un autre joueur allié serait catastrophique pour la coopération. Le système adapte donc les règles.

#### Règles spécifiques aux phases de Stampede

- **Résurrection sur place par soigneur autorisée** — sans limite de charges (pas de consommable requis), car la priorité est de maintenir le front
- **Pénalité XP maintenue** — la mort a un coût même en Stampede (retour à 0% du niveau courant — voir 4.5)
- **Pas de drop d'items** — zéro item ne tombe sur le corps en Stampede. Le corps n'est pas pillable.
- **Corps visible uniquement par le groupe du joueur** — pas d'icône publique sur la carte
- **Corps dure 10 minutes** — plus longtemps qu'en zone normale (le combat est chaotique)

**Justification narrative :** En Stampede, les factions sont liées par le Pacte de la Percée — un accord tacite reconnu par toutes les traditions de Véranthas, y compris les Outlaws. Piller un camarade lors d'un Stampede est considéré un acte de trahison absolue, passible d'ostracisme social dans toutes les factions simultanément (le système de réputation en prend acte).

---

### 4.5 Pénalité XP

#### Règle fondamentale

La mort entraîne une **perte de tout le progrès d'XP accumulé dans le niveau courant**, retour à 0%.

```
xp_après_mort = xp_au_début_du_niveau_courant
```

**Exemple concret :**
- Niveau 25 (à 67% de progression vers le niveau 26)
- Mort → retour à niveau 25 à 0%
- Le joueur doit refaire 67% du niveau 25 pour retrouver sa position précédente
- Le niveau 25 **n'est pas perdu** — on ne régresse jamais en dessous du niveau atteint

**Le niveau lui-même n'est jamais perdu.** Un joueur niveau 25 ne peut pas passer à niveau 24 suite à des morts répétées. La pénalité est uniquement sur l'XP intra-niveau.

#### Pénalité XP par zone

| Zone | Pénalité XP |
|------|-------------|
| Zone normale (tous niveaux) | Retour à 0% du niveau courant |
| Zone de guerre | Retour à 0% du niveau courant |
| Donjon normal | Retour à 0% du niveau courant |
| Donjon héroïque | Retour à 0% du niveau courant |
| Stampede | Retour à 0% du niveau courant |
| Mode Hardcore | Mort permanente (voir 4.6) |
| Paragon (post-60) | Retour à 0% du niveau Paragon courant |

**Cas particulier — mort PvP injuste :** Si un joueur meurt du fait d'un bug documenté (pathfinding exploité, sort hors-portée ayant traversé une géométrie) et que cela est reporté et confirmé par le système anti-triche, la pénalité XP est remboursée dans les 24h.

#### XP de compensation

Après une mort, le personnage reçoit un "Buff de Résilience" pendant 30 minutes : +15% d'XP gagnée. Cela ne compense pas intégralement la perte (le buff se désactive dès que la position XP pré-mort est retrouvée), mais réduit le grind de rattrapage.

---

### 4.6 Mode Hardcore

#### Définition

Le mode Hardcore est une **option de création de personnage irréversible**. Le personnage créé en mode Hardcore :

- Vit sur un **serveur séparé** dédié (règles identiques au serveur normal sauf la mort)
- Meurt **définitivement** à la première mort (quel que soit le contexte — accident, PvP, lag, donjon)
- Ne peut pas être transféré vers un serveur normal
- Ses items ne peuvent pas être transférés vers un personnage normal

#### Hall of Fame Hardcore

Chaque personnage Hardcore mort est automatiquement archivé dans le **Hall of Fame Hardcore** :
- Nom du personnage, classe, faction
- Niveau atteint
- Date et cause de mort (type de mob/joueur)
- Top-5 des kills PvP de ce personnage
- Top-5 des bosses vaincus
- Durée totale de jeu (heures)

Les personnages qui ont atteint le niveau 60 avant de mourir reçoivent une mention spéciale permanente, leur nom gravé dans un monument virtuel dans chaque capitale.

#### Règles spécifiques au serveur Hardcore

- Les groupes sont possibles et encouragés
- Le PvP existe avec les mêmes règles que le serveur normal (zones de guerre, full-loot)
- **Une mort = fin du personnage**, même en donjon instancié, même en Stampede
- Le seul cas de "sauvetage" : ressuscité par un soigneur allié **avant** l'animation de mort complète (fenêtre de 1,5 seconde). C'est la seule résurrection possible en Hardcore.
- Le marché du serveur Hardcore est **isolé** du serveur normal

#### Ladder Hardcore saisonnier

À chaque saison (environ 3 mois), le ladder Hardcore est remis à zéro. Les personnages qui survivent à une saison complète reçoivent un cosmétique permanent sur leurs futurs personnages (normal ou Hardcore) — une "Marque de Survivant" visible sur l'avatar.

---

### 4.7 Ghost et retour au corps

#### Spawn après mort

À la mort, le joueur apparaît (en tant que personnage "en vie mais loin") au **dernier waypoint activé**. Les waypoints sont des structures physiques dans le monde (obélisques, portails de pierre, flambeaux de faction) que le joueur doit toucher physiquement pour les activer.

**Spawn au waypoint, pas en ghost.** Il n'y a pas de mode "fantôme invulnérable" — le personnage respawne avec **25% de ses PV maximum** et peut immédiatement mourir à nouveau si la zone autour du waypoint est dangereuse.

#### Course de retour au corps

Le joueur court à vitesse normale (MS de base, sans buff) depuis le waypoint jusqu'à l'emplacement de son corps. La carte affiche :
- Une icône indiquant la position du corps
- Un timer montrant la durée restante avant disparition du corps
- Une estimation de temps de trajet (en secondes)

**Risques :**
- Le joueur peut mourir à nouveau en courant vers son corps. Une nouvelle mort génère un nouveau corps à cet endroit, et le premier corps continue son décompte séparément.
- Si deux corps coexistent, les pénalités XP ne se cumulent pas — seule la dernière mort est comptabilisée pour l'XP.

**Récupération du corps :**
- Le joueur doit **physically reach** son corps et cliquer dessus pour récupérer ses objets
- Si le corps est pillé avant son arrivée : les objets pris par d'autres joueurs sont perdus définitivement
- Si personne n'a pillé le corps : le joueur récupère intégralement tout ce qui reste

#### Résurrection par soigneur allié

En zone normale et en donjon, un soigneur peut ressusciter un joueur **sur place** (à l'emplacement du corps, pas au waypoint). Cette résurrection :
- Requiert que le soigneur soit adjacent au corps (dans 3 mètres)
- Prend 3 secondes de cast interrompable
- Restaure le personnage à 50% PV et 30% Mana
- Ne rend pas les objets pillés (le corps est récupéré tel quel)
- Ne peut pas être effectuée si le corps a déjà disparu

---

## 5. Schémas TOML complets

Les schémas suivants définissent la structure de données attendue par le moteur MGE (Rust, ECS archétype, data-driven TOML). Ils sont normatifs — toute déviation doit être documentée et approuvée.

```toml
# AL-Character-Combat-Part2b.toml
# Schéma de référence — Systèmes transversaux du personnage
# Monde : Véranthas, An 247 AO

# ====================================================================
# SECTION 1 — PROGRESSION
# ====================================================================

[progression]
version = "1.0"
world = "veranthas"

[progression.xp_curve]
formula = "500 * level ^ 1.8"
level_cap = 60
stat_points_base_per_level = 3
skill_points_base_per_level = 1

[progression.xp_curve.paliers]
level_10 = { stat_bonus = 1, skill_bonus = 1, unlock = "specialisation_faction" }
level_20 = { stat_bonus = 1, skill_bonus = 1, unlock = "zones_niveau_2" }
level_30 = { stat_bonus = 1, skill_bonus = 1, unlock = "convergence_complete" }
level_45 = { stat_bonus = 1, skill_bonus = 1, unlock = "endgame_lite" }
level_60 = { stat_bonus = 2, skill_bonus = 2, unlock = "endgame_complet" }

[progression.xp_bonus]
group_per_player = 0.15
group_max_bonus = 0.50
war_zone_active = 0.25
garum_zone_active = 0.30
faction_quest_high_rank = 0.20
first_kill_zone_boss = 5.00
first_kill_dungeon_normal_weekly = 2.00
first_kill_dungeon_heroic_weekly = 4.00
controlled_faction_zone = 0.10
rest_multiplier = 2.0
rest_offline_hours_per_level = 8.0

# ====================================================================
# SECTION 2 — SKILL USAGE
# ====================================================================

[progression.skill_usage]
only_real_combat = true
combat_flag_window_seconds = 10
minimum_enemy_level_delta = -10
pvp_usage_multiplier = 1.50
same_target_spam_threshold = 15
same_target_spam_multiplier = 0.20
underleveled_target_multiplier = 0.10

[progression.skill_usage.active_dps]
uses_per_rank = [0, 250, 280, 310, 340, 380, 420, 460, 500, 550, 600, 650, 700, 750, 800, 860, 920, 980, 1040, 1110]
max_rank = 20
damage_bonus_per_rank = [0.0, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.04, 0.04, 0.04, 0.04, 0.04, 0.03, 0.03, 0.03, 0.05]
unlock_ranks = { 5 = "secondary_effect", 10 = "advanced_variant", 20 = "mastered_form" }
cost_reduction_ranks = { 7 = 0.05, 12 = 0.10, 18 = 0.15 }

[progression.skill_usage.active_utility]
uses_per_rank = [0, 120, 135, 150, 165, 180, 200, 220, 240, 260, 280, 300, 320, 340, 360]
max_rank = 15
efficiency_bonus_per_rank = [0.0, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.06, 0.05, 0.05, 0.05, 0.04, 0.04]
unlock_ranks = { 10 = "secondary_effect", 15 = "mastered_form" }

[progression.skill_usage.passive]
triggers_per_rank = [0, 60, 65, 70, 75, 80, 85, 90, 95, 100]
max_rank = 10
efficiency_bonus_per_rank = [0.0, 0.08, 0.08, 0.08, 0.08, 0.08, 0.08, 0.06, 0.06, 0.06]
unlock_ranks = { 10 = "final_feature" }

[progression.skill_usage.aura]
seconds_active_per_rank = [0, 600, 700, 800, 900, 1000, 1200, 1400, 1600, 1800]
max_rank = 10
power_bonus_per_rank = [0.0, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.10]
cost_reduction_ranks = { 6 = 0.10 }

[progression.skill_cap]
formula = "floor(level / 2) * 20"
action_bar_active_slots = 12
passive_aura_slots = 6
extra_slot_unlock = { source = "adventurers_guild_companion", slots = 1 }

# ====================================================================
# SECTION 3 — RESPEC
# ====================================================================

[progression.respec]
[progression.respec.stat_reset]
tiers = [
  { max_points = 10, cost_gold = 500, cooldown_hours = 0 },
  { max_points = 30, cost_gold = 2000, cooldown_hours = 0 },
  { max_points = 60, cost_gold = 8000, cooldown_hours = 24 },
  { max_points = 100, cost_gold = 25000, cooldown_hours = 72, requires_quest = true },
  { max_points = 999, cost_gold = 0, cooldown_hours = 168, requires_quest = "respec_profond" },
]

[progression.respec.weekly_limits]
stat_light = 999
stat_moderate = 3
stat_deep = 1
stat_total = 0
skill_light = 999
skill_moderate = 5
skill_total_single = 2

[progression.respec.skill_reset_cost_per_rank]
rank_1_5 = 200
rank_6_10 = 500
rank_11_15 = 1200
rank_16_20 = 3000

[progression.respec.build_memory]
slots = 3
extra_slot_unlock = { source = "adventurers_guild_companion" }
switch_cost_gold = 500
switch_cast_time_seconds = 30
switch_requires_out_of_combat = true

# ====================================================================
# SECTION 4 — PARAGON
# ====================================================================

[progression.paragon]
xp_formula = "200000 * level ^ 1.2"
power_cap_points = 150

[progression.paragon.branch_acier]
cap = 50
bonuses_per_point = { phys_damage = 0.002, magic_damage = 0.002, crit_chance = 0.0015, aspd = 0.003 }

[progression.paragon.branch_ecorce]
cap = 50
bonuses_per_point = { max_hp = 0.003, phys_defense = 0.002, all_resist = 0.001, hp_regen = 0.0015 }

[progression.paragon.branch_ether]
cap = 50
bonuses_per_point = { max_mana = 0.0025, skill_cost_reduction = 0.0015, move_speed = 0.002, cooldown_reduction = 0.001 }

[progression.paragon.branch_cosmos]
cap = 999
points_per_unlock = 10
unlocks = ["title", "aura_cosmetic", "hall_of_fame", "lore_quest", "housing_decoration"]

# ====================================================================
# SECTION 5 — STATS PRIMAIRES
# ====================================================================

[stats.primary]
names = ["ferrath", "velthar", "gaiathar", "rhathar"]
lore_names = ["Ferrath-Anth", "Velthar-Sorath", "Gaïathar-Nexis", "Rhathar-Veines"]

[stats.primary.ferrath]
phys_damage_multiplier = 0.003
heavy_weapon_bonus = 0.002
carry_capacity_base = 100
carry_capacity_per_point = 5
breakpoints = { 50 = "heavy_plate_unlock", 100 = "terrifying_strike", 150 = "carry_double_bonus", 200 = "phys_passive_unlock", 250 = "granite_body" }

[stats.primary.velthar]
aspd_reduction_factor = 1.0
dodge_rating_per_point = 0.15
dodge_cap = 40.0
crit_bonus_per_point = 0.1
dash_range_bonus = 0.05
breakpoints = { 50 = "move_speed_5", 100 = "advanced_leather_parry", 150 = "auto_dodge_30s", 200 = "crit_imbalance", 250 = "eclipse_reflexes" }

[stats.primary.gaiathar]
magic_damage_multiplier = 0.004
mana_per_point = 8
magic_resist_per_point = 0.05
magic_resist_cap_from_stat = 30.0
heal_power_multiplier = 0.003
effect_duration_multiplier = 0.002
breakpoints = { 50 = "intermediate_staves", 100 = "mana_regen_bonus", 150 = "nexus_resonance", 200 = "cc_duration_plus", 250 = "voice_of_gaia" }

[stats.primary.rhathar]
hp_per_point = 12
hp_regen_bonus_per_point = 0.02
phys_resist_per_point = 0.04
corruption_resist_per_point = 0.06
debuff_duration_reduction_per_point = 0.002
debuff_duration_minimum = 0.60
breakpoints = { 50 = "combat_light_regen", 100 = "primal_endurance", 150 = "corruption_resist_extended_cap", 200 = "stun_duration_half", 250 = "pure_vein" }

# ====================================================================
# SECTION 6 — STATS DÉRIVÉES
# ====================================================================

[stats.derived]

[stats.derived.hp]
formula = "base_hp_class + (rhathar * 12) + equipment_bonus"

[stats.derived.mana]
formula = "base_mana_class + (gaiathar * 8) + equipment_bonus"

[stats.derived.defense_rating]
formula = "equipment_armor + (rhathar * 0.04_percent) + bonuses"
reduction_formula = "dr / (dr + 800) * 100"
cap_from_dr_alone = 40.0
combined_cap = 60.0

[stats.derived.attack_rating]
formula = "(velthar * 1.5) + (ferrath * 0.5) + weapon_bonus + equipment_bonus"
hit_chance_formula = "50 + (ar_attacker - dodge_defender) / 20"
hit_chance_cap = 95.0
hit_chance_floor = 5.0

[stats.derived.aspd]
server_tick_rate = 20
breakpoints_ticks = [60, 50, 40, 34, 28, 24, 20, 17, 14, 12, 10, 8]
breakpoints_aps = [0.33, 0.40, 0.50, 0.59, 0.71, 0.83, 1.00, 1.18, 1.43, 1.67, 2.00, 2.50]

[stats.derived.move_speed]
formula = "base_ms_class + (velthar * 0.1) + equipment_bonus"
base_ms_light = 115
base_ms_hybrid = 105
base_ms_heavy = 95
cap_from_stats = 140
cap_with_abilities = 200

[stats.derived.regeneration]
hp_out_of_combat_percent_per_5s = 2.0
hp_out_of_combat_timer_seconds = 15
mana_out_of_combat_percent_per_5s = 3.0
mana_in_combat_percent_per_5s = 0.5
sitting_multiplier = 2.0
nexus_zone_hp_bonus_per_5s = 1.0
nexus_zone_mana_bonus_per_5s = 2.0

[stats.derived.crit]
base_chance_percent = 2.0
velthar_per_point = 0.05
cap_pve = 60.0
cap_pvp = 35.0
base_multiplier = 1.5
velthar_multiplier_per_point = 0.001
multiplier_cap_pve = 2.5
multiplier_cap_pvp = 2.0

# ====================================================================
# SECTION 7 — RÉSISTANCES
# ====================================================================

[stats.resistances]
names = ["physical", "fire", "frost", "lightning", "corruption"]
lore_names = ["Voile de Pierre", "Voile de Braise", "Voile de Gel", "Voile de l'Étincelle", "Voile de Cendre"]

[stats.resistances.caps]
physical_pve = 80
physical_pvp = 60
elemental_pve = 75
elemental_pvp = 50
corruption_pve = 70
corruption_pvp = 40

[stats.resistances.frost]
damage_reduction_factor = 0.70
slow_duration_reduction_factor = 0.30

[stats.resistances.lightning]
direct_reduction_factor = 0.80
chain_reduction_factor = 0.40

[stats.resistances.corruption]
damage_reduction_factor = 1.00
duration_formula = "max(0.40, 1.0 - resist * 0.006)"

[stats.resistances.penalties_by_zone]
normal = { physical = 0, elemental = 0, corruption = 0 }
convergence_30_plus = { physical = -10, elemental = -15, corruption = -20 }
war_zone_active = { physical = -15, elemental = -15, corruption = -15 }
dungeon_normal = { physical = 0, elemental = -10, corruption = -10 }
dungeon_heroic = { physical = -20, elemental = -25, corruption = -30 }
stampede_phase_3_4 = { physical = -25, elemental = -20, corruption = -40 }
voice_of_erosion_boss = { physical = -30, elemental = -35, corruption = -50 }

# ====================================================================
# SECTION 8 — MORT ET CONSÉQUENCES
# ====================================================================

[death]
version = "1.0"

[death.normal_zone]
items_dropped = "all_except_protected"
protected_slots = 1
body_duration_seconds = 300
pillage_allowed_by = "anyone"
xp_penalty = "reset_current_level_progress"
notification_killer = true
revival_buff_duration_minutes = 30
revival_buff_xp_bonus = 0.15
spawn_location = "last_waypoint"
spawn_hp_percent = 0.25

[death.war_zone]
items_dropped = "all_except_protected_including_cargo"
cargo_pillage_percent = 1.00
body_duration_seconds = 180
body_icon_visible_on_map_seconds = 90
xp_penalty = "reset_current_level_progress"
spawn_location = "last_waypoint"
loot_killer_gold_bonus = 0.25

[death.dungeon_instanced]
items_dropped_chance = 0.50
body_recoverable = true
body_disappears_on_instance_end = true
resurrection_by_healer = true
resurrection_consumes = "coeur_de_gaia"
resurrection_hp_percent = 0.50
resurrection_mana_percent = 0.30
resurrection_cast_time_seconds = 3.0
resurrection_range_meters = 3.0
spawn_location = "dungeon_camp_base"
xp_penalty = "reset_current_level_progress"

[death.dungeon_heroic]
items_dropped_chance = 0.75
body_recoverable = true
resurrection_during_boss_engagement = false
tpk_behavior = "reset_to_last_checkpoint"

[death.stampede]
items_dropped = "none"
body_duration_seconds = 600
body_visible_to = "group_only"
resurrection_by_healer = true
resurrection_requires_consumable = false
resurrection_unlimited = true
xp_penalty = "reset_current_level_progress"
pillage_reputation_penalty = "all_factions_max"

[death.hardcore]
permanent = true
server = "hardcore"
inter_server_item_transfer = false
hall_of_fame = true
revival_window_seconds = 1.5
revival_only_by_healer_in_window = true
season_duration_days = 90
season_survivor_reward = "mark_of_survivor_cosmetic"

[death.xp_penalty]
rule = "reset_to_level_start"
level_regression = false
paragon_rule = "reset_to_paragon_level_start"
unfair_death_refund = true
unfair_refund_delay_hours = 24

[death.ghost]
mode = "alive_at_waypoint"
invulnerability = false
movement_speed = "base_ms"
body_map_icon = true
body_map_icon_visible_to = ["owner", "group"]
return_action = "walk_to_body_and_interact"
items_lost_if_not_recovered = true

[death.resurrection]
cast_time_seconds = 3.0
required_proximity_meters = 3.0
hp_restored_percent = 0.50
mana_restored_percent = 0.30
not_possible_if_body_gone = true
hardcore_only_in_revival_window = true

# ====================================================================
# SECTION 9 — ÉQUIPEMENT
# ====================================================================

[equipment]
version = "1.0"
total_slots = 14

[equipment.slots]
heaume = { index = 1, unlock_level = 1, accepts = ["helm", "hat", "hood", "crown"] }
plastron = { index = 2, unlock_level = 1, accepts = ["chestplate", "robe", "jacket", "mail"] }
jambières = { index = 3, unlock_level = 1, accepts = ["leggings", "pants", "robe_bottom"] }
sollerets = { index = 4, unlock_level = 1, accepts = ["boots", "sandals", "sabatons"] }
gantlets = { index = 5, unlock_level = 1, accepts = ["gauntlets", "gloves", "bracers"] }
ceinturon = { index = 6, unlock_level = 1, accepts = ["belt", "sash", "band"] }
main_directrice = { index = 7, unlock_level = 1, accepts = ["sword_1h", "axe_1h", "mace", "staff", "bow", "crossbow", "dagger", "sword_2h", "axe_2h", "staff_2h"] }
main_soutien = { index = 8, unlock_level = 1, accepts = ["shield", "parry", "quiver", "orb", "grimoire", "dagger_offhand"] }
anneau_gauche = { index = 9, unlock_level = 5, accepts = ["ring"] }
anneau_droit = { index = 10, unlock_level = 5, accepts = ["ring"], effect_stack_limit = true }
pendentif = { index = 11, unlock_level = 5, accepts = ["amulet", "medallion", "nexus_relic", "vein_fragment"] }
manteau = { index = 12, unlock_level = 10, accepts = ["cape", "cloak", "coat"] }
monture = { index = 13, unlock_level = 15, accepts = ["horse", "destrier", "ervan_deer", "sea_kraken", "garum_mount"] }
talisman = { index = 14, unlock_level = 30, accepts = ["faction_talisman", "dungeon_artifact"] }

[equipment.two_handed_lock]
two_handed_occupies_slots = [7, 8]
bow_locks_secondary_to = ["quiver"]

[equipment.quality_tiers]
commun = { color = "#808080", affixes_min = 0, affixes_max = 1, level_min = 1, sources = ["npc_vendor", "basic_loot"] }
peu_commun = { color = "#00AA00", affixes_min = 1, affixes_max = 2, level_min = 5, sources = ["mob_loot", "beginner_crafting"] }
rare = { color = "#0000FF", affixes_min = 2, affixes_max = 3, level_min = 15, sources = ["normal_dungeons", "zone_bosses", "intermediate_crafting"] }
epique = { color = "#800080", affixes_min = 3, affixes_max = 4, level_min = 30, sources = ["heroic_dungeons", "convergence_bosses", "advanced_crafting"] }
legendaire = { color = "#FFD700", affixes_min = 4, affixes_max = 5, level_min = 45, sources = ["raids", "voice_of_erosion", "master_crafting"] }
artefact_faction = { color = "#FF0000", affixes_min = 5, affixes_max = 6, level_min = 55, has_unique_effect = true, sources = ["seasonal_events", "victorious_sieges", "grande_convergence"] }

[equipment.level_requirements]
commun = 1
peu_commun = 5
rare = 15
epique = 30
legendaire = 45
artefact_faction = 55

[equipment.stat_requirements]
[equipment.stat_requirements.armor_tiers]
tissu_1 = { ferrath = 0, rhathar = 0 }
cuir_1 = { ferrath = 0, rhathar = 0 }
maille_1 = { ferrath = 20, rhathar = 0 }
plaque_legere_1 = { ferrath = 40, rhathar = 15 }
plaque_lourde_1 = { ferrath = 50, rhathar = 20 }
armure_campagne_5 = { ferrath = 80, rhathar = 40 }
armure_heroique_1 = { ferrath = 120, rhathar = 60 }
armure_siege = { ferrath = 160, rhathar = 80 }

[equipment.stat_requirements.weapon_tiers]
arme_1m_legere = { velthar = 10 }
arme_1m_standard = { ferrath = 15 }
arme_1m_lourde = { ferrath = 30 }
arme_2m_legere = { ferrath = 25, velthar = 15 }
arme_2m_lourde = { ferrath = 50 }
arc = { velthar = 20, ferrath = 10 }
arbalète = { ferrath = 20, velthar = 15 }
baton_magique_1m = { gaiathar = 20 }
baton_magique_2m = { gaiathar = 35 }
orbe_grimoire = { gaiathar = 25 }
```

---

## Notes de design — Équilibre et cohérence

### Sur la progression hybride

Le système niveau + skill usage crée naturellement **deux vitesses de progression**. Un joueur peut atteindre le niveau 60 avec des skills à peine au rang 5, ou atteindre le rang 20 en plusieurs skills tout en restant au niveau 30. C'est intentionnel : le "vétéran de contenu" est différent du "vétéran de puissance". Le cap de skill empêche l'accumulation infinie.

### Sur les stats primaires

Le choix des noms proto-ervan (Ferrath, Velthar, Gaïathar, Rhathar) ancre les mécaniques dans le lore de Véranthas. Le joueur qui lit les Mémoires Vertes dans le jeu retrouve ces termes — il comprend que ses stats ne sont pas des valeurs abstraites mais des concepts que le monde lui-même reconnaît.

### Sur la mort full-loot

La mort est le système de régulation économique principal. Sans full-loot, les items s'accumulent, l'économie s'effondre. Avec un seul slot protégé, le joueur a un espace émotionnel (il ne perd jamais son "item chéri") tout en subissant une vraie pénalité. C'est l'équilibre UO qu'Allumina cherche à retrouver.

### Sur le Stampede comme espace de trêve

Supprimer le drop pendant les Stampedes n'est pas un cadeau fait au joueur — c'est une décision narrative forte. Le Stampede est le seul moment où Véranthas rappelle à ses habitants qu'ils partagent un monde commun. Les mécaniques doivent servir cette vérité lore.

---

*Document rédigé pour Allumina, An 247 AO. Référence canonique v1.0. Toute modification majeure des systèmes ici décrits requiert validation par le lead game designer et annotation dans le registre de décisions architecturales.*
