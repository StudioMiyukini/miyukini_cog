# MGE — Analyse et regroupement des points similaires

Identification des thèmes récurrents dans la référence MGE, avec propositions de consolidation.

---

## 1. Combat — Toucher, esquive, parade

| Sections | Points |
|----------|--------|
| **6** | Gestion parcours (ciblage) |
| **11.6** | Verrouillage de cible |
| **14.2** | Attack Rating vs Defense ; Block chance ; Dodge/Avoid |
| **16.2** | Flee ; modificateurs taille/race |

**Regroupement proposé :** Section **6 Combat** complétée par *Chance de toucher (AR/Def, Flee) ; parade ; verrouillage cible*. Les sections 14 et 16 gardent les détails spécifiques (formules Diablo, modificateurs RO).

---

## 2. Effets de statut — Buffs, debuffs, CC

| Sections | Points |
|----------|--------|
| **11.6** | Buffs/debuffs ; effets de statut ; crowd control |
| **14.2** | Stun, freeze, slow ; poison over time |
| **16.16** | Poison, aveuglement, silence ; stun, freeze, sleep ; curse, stone ; buffs |

**Regroupement proposé :** Créer un point unique en **6 Combat** ou **10** : *Effets de statut : buffs, debuffs, CC (stun, silence, slow, poison) ; durée ; immunités ; dispel*. Références croisées 11.6, 16.16.

---

## 3. Résistances, éléments, immunités

| Sections | Points |
|----------|--------|
| **14.2** | Résistances (feu, froid, éclair, poison) ; immunités |
| **14.5** | Immunités monstres |
| **16.2** | Éléments ; modificateurs taille ; modificateurs race |

**Regroupement proposé :** Point générique en **6 Combat** : *Résistances et immunités (éléments, physique) ; modificateurs (taille, race)*. 14 et 16 conservent les implémentations spécifiques.

---

## 4. Mort, résurrection, corps

| Sections | Points |
|----------|--------|
| **6** | Gestion mort ; respawn ; drops |
| **11.9** | Mécaniques de résurrection ; cimetière / point de réapparition |
| **12.3** | Corps (corpse) ; lootable ; décomposition |
| **14.6** | Corpse ; récupération du corps |
| **16.14** | Perte XP ; pas de drop ; résurrection |

**Regroupement proposé :** Étendre **6 Gestion mort** avec : *Corps au sol (optionnel) ; récupération ; résurrection (skill, objet, NPC) ; perte XP (optionnel) ; drop ou conservation équipement*. Sections 11, 12, 14, 16 gardent les variantes.

---

## 5. Montures et familiers

| Sections | Points |
|----------|--------|
| **11.3** | Montures ; familiers/pets |
| **13.5** | Montures au combat ; combat à cheval |
| **16.10** | Oeufs pet ; Peco Peco ; Faucon ; charrette |

**Regroupement proposé :** Point générique en **5 Joueur** ou **11.3** : *Montures (acquisition, vitesse, combat monté) ; familiers (capture, évolution, compétences)*. 13 et 16 gardent leurs variantes (chevaux, Peco, faucon, charrette).

---

## 6. Guildes et siège territorial

| Sections | Points |
|----------|--------|
| **11.1** | Guildes / clans |
| **12.9** | Guild wars ; siège de maisons |
| **13.3** | Zones de contrôle ; portes et murailles |
| **15.2** | Châteaux ; siège ; portes ; armes de siège |
| **16.11** | Guilde (emblème, compétences, stockage) |

**Regroupement proposé :** **11.1** comme base : *Guildes (membres, rangs, banque, emblème)*. Sous-thème séparé : *Siège territorial (châteaux, bases, portes, armes de siège)* → converger 12.9, 13.3, 15.2. 16.11 reste spécifique RO (Kafra, compétences guilde).

---

## 7. Navigation — Bateaux, téléportation

| Sections | Points |
|----------|--------|
| **12.8** | Bateaux ; combat naval ; ancrage |
| **15.9** | Bateaux ; continents ; PNJ téléportation |
| **11.2** | Monde persistant vs instancié |

**Regroupement proposé :** Point commun *Navigation longue distance (bateaux, téléports)* dans **3 Déplacement** ou nouvelle sous-section **Monde et déplacements**. 12, 15 gardent les détails (combat naval, continents).

---

## 8. Renforcement d’équipement

| Sections | Points |
|----------|--------|
| **12.5** | Durabilité ; réparation |
| **14.4** | Durabilité ; éthéré |
| **15.5** | Enchantement (+N) ; parchemins |
| **16.4** | Raffinement (+N) ; safe/risqué ; over-upgrade |

**Regroupement proposé :** Point générique *Renforcement équipement (durabilité, enchantement, raffinement ; risque d’échec)*. 12/14/15/16 décrivent les variantes (UO, Diablo, Lineage, RO).

---

## 9. Bosses et contenu instancié

| Sections | Points |
|----------|--------|
| **11.2** | Instances ; world bosses |
| **11.9** | Donjons ; raids |
| **12.9** | Champion spawns |
| **14.5** | Super uniques ; champion packs ; Treasure Class |
| **15.9** | Donjons |
| **16.5** | MVP ; mini-boss ; annonces |

**Regroupement proposé :** **11.2 / 11.9** comme base commune : *Instances ; donjons ; bosses (spawn, loot, annonces)*. 12, 14, 15, 16 ajoutent leurs variantes (champion spawns, super uniques, MVP, etc.).

---

## 10. Commerce et vente joueur

| Sections | Points |
|----------|--------|
| **8** | Commerce ; échange joueurs |
| **11.5** | Hôtel des ventes ; devises ; marché |
| **12.4** | Vendeurs joueur (PNJ) |
| **15.12** | Marchands NPC ; échange ; marché |
| **16.8** | Vending (boutique assise) ; charrette ; surcharge poids |

**Regroupement proposé :** **8** reste la base. Ajouter *Vente joueur (boutique, hôtel ventes, vendeurs PNJ)*. 16.8 conserve vending + charrette RO.

---

## 11. Inventaire et stockage

| Sections | Points |
|----------|--------|
| **5** | Inventaire (slots + poids) |
| **12.10** | Poids / encumbrance ; limite stack ; menus contextuels |
| **14.4** | Stash ; ceinture potions |
| **16.8** | Charrette ; surcharge |
| **16.11** | Stockage guilde |

**Regroupement proposé :** **5** = définition de base. Sous-point *Stockage persistant (stash, guilde, Kafra)*. 12, 14, 16 gardent les variantes.

---

## 12. Magie et compétences

| Sections | Points |
|----------|--------|
| **5** | Gain compétences |
| **6** | Action ; ressources (mana) |
| **11.4** | Niveau ; arbres talents |
| **12.7** | Cercles magie ; composants ; livres sorts |
| **14.3** | Hotkeys ; mana ; passives |
| **15.6** | Livres sorts ; mana ; sorts zone |
| **16.2** | Cast time ; after-cast delay |

**Regroupement proposé :** **6** = base (ressources, cooldowns). Ajouter *Cast time ; after-cast delay*. 12, 14, 15, 16 gardent cercles, hotkeys, livres, etc.

---

## 13. Karma, alignement, criminalité

| Sections | Points |
|----------|--------|
| **12.2** | Karma ; meurtrier ; full loot PvP |
| **15.3** | Alignement ; PK ; rédemption ; zones PK |

**Regroupement proposé :** Thème *Réputation et criminalité (karma, alignement, PK, rédemption)*. 12 (UO) et 15 (Lineage) gardent leurs implémentations.

---

## Synthèse des actions recommandées

| Action | Description |
|--------|-------------|
| **Renforcer sections 1–10** | Centraliser les concepts génériques (effets statut, résistances, mort/respawn, renforcement). |
| **Références croisées** | Utiliser *(voir section X)* ou *(cf. Y.Z)* pour éviter la duplication. |
| **Regrouper sous-thèmes** | « Siège territorial » (11 + 12 + 13 + 15) ; « Bosses » (11 + 14 + 15 + 16). |
| **Garder le détail spécifique** | Chaque section 11–16 conserve les variantes propres à chaque jeu. |

---

**Document** : Analyse points similaires MGE  
**Date** : 2026-02-18
