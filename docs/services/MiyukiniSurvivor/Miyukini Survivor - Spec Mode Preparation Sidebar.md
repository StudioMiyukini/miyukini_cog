# Miyukini Survivor — Spécification Mode Préparation (sidebar)

**Contexte :** Lord of the Castle. En phase Préparation, une sidebar droite (affichable/masquable) donne accès au Marchand, à l’Expert en identification, au mode Construction et au mode Recrutement. La zone de combat reste visible à gauche pour placer les bâtiments.

**Implémenté à ce jour :**
- Sidebar droite avec bouton « Mode préparation ▶ » (affichage) / « ◀ Masquer » (masquage).
- Quatre sections : Marchand, Expert, Construction, Recrutement (squelettes UI).
- Or + bouton « Lancer la vague » dans la sidebar.
- Zone de combat conservée à gauche.

---

## 1. Marchand

- **Potions et consommables** : vente illimitée.
- **Objets à équiper** : objets communs (connus).
- **Un objet non identifié** : plus cher, pari pour le joueur (peut être commun ou meilleur après identification).

*(À implémenter : catalogue, prix, achat, ajout à l’inventaire.)*

---

## 2. Expert en identification

- **Identification groupée** : identifier tous les objets non identifiés de l’inventaire en une fois.
- **Prix** : coût cumulé (ex. 100 or × nombre d’objets à identifier).

*(À implémenter : `identify_all_expert()` dans GameState, bouton + affichage du coût total.)*

---

## 3. Construction

### 3.1 Tours
- **Tour d’archer** : projectile sur une cible unique (la plus proche). Dégâts légers, cadence moyenne, portée moyenne.
- **Tour baliste** : projectile unique qui traverse tous les ennemis sur la ligne. Dégâts élevés, portée moyenne, cadence lente.
- **Tour catapulte** : cible l’ennemi avec le plus de PV à portée ; dégâts de zone à l’impact. Dégâts élevés, cadence lente, portée élevée.

### 3.2 Fortifications — Murs
- **Barricade** : obstacle, pas cher, peu de PV.
- **Palisade** : barricade améliorée (plus de PV, un peu plus cher).
- **Mur** : obstacle, coût moyen, bonnes PV.
- **Muraille** : très cher, très haute PV.
- **Porte** : traversable par le joueur et les troupes, moins de PV et moins cher que palissade.
- **Portail** : comme un mur, traversable par le joueur et les troupes.
- **Hearth** : portail avec plus de PV et plus cher.

### 3.3 Fortifications — Pièges
- **Douves** : zone carrée qui ralentit.
- **Pointes** : dégâts par seconde tant que l’ennemi est dessus.
- **Piège à loup** : immobilise X secondes.
- **Feu grégeois** : applique « en feu » (DoT jusqu’à la mort).

### 3.4 Bâtiments civils
- **Auberge** : (à définir)
- **Taverne** : mini-jeux pour or ou infos sur la vague suivante.
- **Forge** : accès à l’équipement.
- **Caserne** : types de troupes.
- **Arsenal** : améliorations pour les troupes.
- **Atelier** : améliorations pour les bâtiments.
- **Habitations** : augmentent le cap de troupes.

*(À implémenter : types de bâtiments, coûts, placement sur la zone de combat, logique en bataille.)*

---

## 4. Recrutement

- **Unité de base** : Paysans.
- **Cap** : dizaine de Charisme (ex. Cha 12 → cap 10 ou 20 selon règle choisie).

*(À implémenter : entité Troupe, cap basé sur Cha, recrutement payant, déploiement.)*

---

**Référence :** demande utilisateur (sidebar Mode préparation, marchand, expert, construction, recrutement).
