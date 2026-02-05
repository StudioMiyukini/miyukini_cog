# Miyukini Survivor — Audit des mécaniques gameplay

## Contexte

Ce document est un **audit de la documentation gameplay** du service Miyukini Survivor. Il recense les **mécaniques floues**, **non abordées** ou **incomplètes** qui nécessitent un complément de documentation pour permettre l'équilibrage et l'implémentation.

**Documents audités :**
- [Miyukini Survivor - Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)
- [Miyukini Survivor - Document Fondateur](Miyukini%20Survivor%20-%20Document%20Fondateur.md)
- [Miyukini Survivor - Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md)

**Statut :** Les décisions de design listées ci-dessous ont été **intégrées** dans [Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md). Ce document sert de **traçabilité** des choix effectués.

## Portée / Scope

- **Périmètre :** Identification des manques et ambiguïtés sur les règles de jeu, les valeurs de référence et les flux (phases, level up, or, etc.).
- **Hors périmètre :** Implémentation technique, équilibrage numérique final.

---

## 1. Mécaniques mentionnées mais non précisées — RÉSOLU

### 1.1 Personnalisation du personnage

| Élément | Décision / valeur |
|--------|-------------------|
| **Fourchette du tirage aléatoire** | **1–5** par stat (référence). |

### 1.2 Joueur — Attaque de base (auto)

| Élément | Décision / valeur |
|--------|-------------------|
| **X (intervalle)** | **1 s** (arme de base). |
| **Y (zone en px)** | **6 px** (arme de base). |
| **Z (dégâts)** | **1–2** (arme de base). |

### 1.3 Armes de jet (clic)

| Élément | Décision / valeur |
|--------|-------------------|
| **X (portée px)** | **80 px** (arme de jet de base). |
| **Y (dégâts)** | **2–4** (arme de jet de base). |
| **Z (cooldown secondes)** | **2 s** (arme de jet de base). |
| **Pierce** | Certains objets ont le passif **« Transperce x fois »**. |

### 1.4 Sortilèges

| Élément | Décision / valeur |
|--------|-------------------|
| **Mana** | **Pool max = Intelligence × 2** ; **régénération = 1 mana/s** ; Intelligence = bonus % dégâts magiques. |
| **Équipement des sorts** | **Barre de sorts dédiée** ; **+1 slot de sort toutes les 10 points de Sagesse** ; sorts auto ou activés par raccourci ; **temps d'incantation** réduit par la Sagesse (ex. temps_base × (1 − Sagesse/100)). |
| **Résistance magique ennemis** | **Réduction flat** des dégâts magiques, **minimum 0**. |

### 1.5 Château

| Élément | Décision / valeur |
|--------|-------------------|
| **Armure** | **Absorption flat**, **minimum 0**. |

### 1.6 Ennemis

| Élément | Décision / valeur |
|--------|-------------------|
| **Champ de vision** | **30 px** de rayon. |
| **Ennemis à projectiles** | Types à **inclure en bêta** (liste à définir). |
| **Ennemis qui traquent le joueur** | À **inclure plus tard** (hors bêta). |
| **Répartition** | **Mini-boss** : 1–3 par vague, 10 % + nombre de vagues depuis dernier spawn (hors Boss). **Boss** : **1** par vague Boss. |

### 1.7 Lune rouge

| Élément | Décision / valeur |
|--------|-------------------|
| **Fréquence** | **10 %** de base + **nombre de vagues sans Lune rouge** (cumul), **max 20 vagues** sans Lune rouge. |
| **Plus difficile** | **Quantité × (1,5 + 0,1 × n)**, n = vagues depuis dernière Lune rouge. |
| **Récompenses** | **Or, XP, drop rate × (2 + 0,1 × n)** (même n). |

### 1.8 Phases

| Élément | Décision / valeur |
|--------|-------------------|
| **Fin phase Bataille** | **Tous les ennemis morts** **ou** **timer max 15 min** (IRL). |
| **Fin phase Préparation** | **Bouton « Lancer la vague »** **ou** **timer max 30 min** (IRL). |
| **Passage** | Fin Bataille → (optionnel) écran de résumé → Préparation ; fin Préparation → Bataille. |

### 1.9 Tours

| Élément | Décision / valeur |
|--------|-------------------|
| **Portée / champ de vision** | **80 px** (tour de base). |
| **Ciblage à égalité** | **Plus proche du Château**. |
| **Dégâts subis** | **PV base 100**, **armure base 0** ; à **0 PV** = **détruite**. |
| **Autres types** | Ralentissement, zone (AoE), multi-cible, etc. — à détailler en bêta. |

### 1.10 Troupes de soldats

| Élément | Décision / valeur |
|--------|-------------------|
| **Recrutement** | Fenêtre « Recruit », **phase Préparation**, **coût en or** ; **nombre max de troupes = Charisme** du joueur. |
| **PV des troupes** | **Figés au moment du recrutement** (basés sur PV max du joueur). |
| **Regroupement (2 s)** | Les troupes **peuvent encore attaquer** pendant les 2 s. |

### 1.11 Inventaire en fin de vague réussie

| Élément | Décision / valeur |
|--------|-------------------|
| **Surplus** | **Vente automatique du surplus** ; **ordre de priorité** : rareté, puis valeur. |

### 1.12 Équipement

| Élément | Décision / valeur |
|--------|-------------------|
| **Slot Dos** | **Sac** = capacité inventaire ; **Carquois** = dégâts flat et/ou vitesse de tir ; **Cape** = défense, vitesse, mana, cast speed. |
| **Monture** | Vitesse de déplacement, PV en plus, **plus de slots d'inventaire**. |

### 1.13 Potions

| Élément | Décision / valeur |
|--------|-------------------|
| **Utilisation** | **Cooldown** selon la potion ; **effet over time** ; **x utilisations par run** (stack par slot) ; **recharge en Préparation en payant** (or). |

---

## 2. Mécaniques absentes ou très peu détaillées — RÉSOLU

### 2.1 Level up

| Élément | Décision / valeur |
|--------|-------------------|
| **Moment** | **Pendant la vague** ; **choix différé en phase Préparation**. |
| **Choix** | **1 point de skill** par level à placer dans **n'importe quel arbre** (prérequis respectés), **en Préparation uniquement**. |

### 2.2 Or

| Élément | Décision / valeur |
|--------|-------------------|
| **Sources** | Drops ennemis, vente d'objets, quêtes, coffres. |
| **Dépenses** | Achat équipement (**marchand en Préparation**), construction tours, recrutement troupes, achat sortilèges, events (ex. recharge potions). |

### 2.3 Coffres

| Élément | Décision / valeur |
|--------|-------------------|
| **Apparition** | **Fin d'un boss** ou d'un **event**. |
| **Contenu** | Or, objets, XP. |
| **Ouverture** | **Crochetage** = Dextérité × (1 + Chance/100) = % d'ouverture ; **Chance** = % d'ouverture directe (sans crochetage). |

### 2.4 PNJ et Charisme

| Élément | Décision / valeur |
|--------|-------------------|
| **PNJ** | **Marchands**, **quêteurs** ; **fenêtre en phase Préparation**. |
| **Charisme** | **Prix** = prix / (1 + Charisme/100) ; **quêtes supplémentaires** et **meilleures récompenses**. |

### 2.5 Achievements et quêtes journalières

| Élément | Décision / valeur |
|--------|-------------------|
| **Achievements** | Récompenses : titres, cosmétiques, déblocages (usages courants du genre). |
| **Quêtes journalières** | Objectifs : kill X ennemis, X clics, jouer X min, tuer un boss, etc. ; récompenses : **bonus 1 h**. |

### 2.6 Vitesse de déplacement

| Élément | Décision / valeur |
|--------|-------------------|
| **Joueur** | **10 px/s** + **Agilité %**. |
| **Ennemis** | **Normal 8 px/s**, **Mini-boss 6 px/s**, **Boss 4 px/s**. |

---

## 3. Incohérences de numérotation (Gameplay et Mecaniques) — CORRIGÉ

Les corrections suivantes ont été appliquées dans [Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md) :

| Emplacement | Correction appliquée |
|-------------|----------------------|
| § Dimensions Château | **§ 2.3** → **§ 3.3** (sous « Le Château »). |
| Sections 6 | « Les phases » renumérotée en **§ 7** (et 7.0, 7.1, 7.2). |
| § Construction / Skills | **§ 6.1.2** → **§ 7.1.2** ; cohérence 7.1.x. |
| § Mode Normal | **§ 10.1** → **§ 11.1**. |
| § Synthèse / Références | **§ 12. Synthèse** → **§ 14. Synthèse** ; **§ 14. Références** → **§ 15. Références**. Section **§ 13. Or, coffres et PNJ** ajoutée. |

---

## 4. Synthèse des priorités

| Priorité | Thème | Statut |
|----------|--------|--------|
| **Haute** | Phases (fin Bataille / Préparation, passage) | Documenté |
| **Haute** | Level up (moment, choix) | Documenté |
| **Haute** | Or (sources, dépenses, où acheter) | Documenté |
| **Haute** | Inventaire fin de vague (règle du surplus) | Documenté |
| **Moyenne** | Mana et sorts (pool, regen, équipement) | Documenté |
| **Moyenne** | Tours (portée, dégâts subis, ciblage, types) | Documenté |
| **Moyenne** | Troupes (recrutement, PV, regroupement) | Documenté (nombre max = Charisme) |
| **Moyenne** | Ennemis (champ de vision, types tireurs/traqueurs, répartition) | Documenté (listes types en bêta / plus tard) |
| **Moyenne** | Lune rouge (X, modificateurs, récompenses) | Documenté |
| **Basse** | Coffres, PNJ, achievements, quêtes | Documenté |

---

## 5. Références

- [Miyukini Survivor - Gameplay et Mecaniques](Miyukini%20Survivor%20-%20Gameplay%20et%20Mecaniques.md)
- [Miyukini Survivor - Document Fondateur](Miyukini%20Survivor%20-%20Document%20Fondateur.md)
- [Miyukini Survivor - Ecrans et UI](Miyukini%20Survivor%20-%20Ecrans%20et%20UI.md)
- [Miyukini Survivor - Loot Prefixes Suffixes et Equipement](reference/Miyukini%20Survivor%20-%20Loot%20Prefixes%20Suffixes%20et%20Equipement.md)

---

**Document créé le :** 2026-02-04  
**Dernière mise à jour :** 2026-02-04  
**Statut :** Audit traité — décisions intégrées dans Gameplay et Mecaniques ; ce document conserve la traçabilité des choix.
