# MiyuClicker — Bâtiments, maçons et construction

## Contexte

Ce document décrit le **système de bâtiments**, la **Guilde des Maçons** (transformation pop → maçon), les **coûts de clic**, les **caps de stock** et l’**UI par cartes** (logo, niveau, nom, description, coût, affectation gens/maçons avec boutons +/−).

## Portée / Scope

- **Périmètre :** Bâtiments (Maison, Caserne, Grenier, Dépôt, Entrepôt), Guilde des Maçons, maçons (1 pop → 1 maçon), construction (1 pt/jour par maçon alloué), état de départ, clics, UI cartes.
- **Référence code :** `state` (niveaux bâtiments, maçons, allocation maçons, caps), `idlesim` (clics, tick construction), `app` (barre Maçons, cartes bâtiment).

---

## 1. État de départ (nouvelle partie)

| Donnée | Valeur |
|--------|--------|
| Bâtiments | Niveau 1 + **3 maisons** |
| Population | **10** |
| Nourriture | **20** |
| Maisons | 3 |
| Caserne | niveau 1 |
| Grenier | niveau 1 |
| Dépôt | niveau 1 |
| Entrepôt | niveau 1 |
| Maçons | 0 |

---

## 2. Guilde des Maçons et maçons

- **Guilde des Maçons** : bâtiment / action permettant de transformer **1 pop (gens) en 1 maçon**.
- **Maçons** : compteur affiché dans la barre **après les Soldats**.
- Chaque maçon alloué à un bâtiment en construction apporte **1 pt de construction / jour** à ce bâtiment.
- Allocation des maçons : par type de bâtiment (Maison, Caserne, Grenier, Dépôt, Entrepôt) ; boutons +/− sur chaque carte.

---

## 3. Bâtiments et construction

### 3.1 Maison

| Propriété | Valeur |
|-----------|--------|
| Effet | **+4 cap population** par maison |
| Coût construction | 30 bois, 20 pierre, 5 fer |
| Points de construction | **30 pts** (+1 % par maison existante) |
| Formule pts | `30 * (1 + 0.01 * nb_maisons)` |

### 3.2 Caserne

| Propriété | Valeur |
|-----------|--------|
| Effet | **+10 cap soldats** par niveau |
| Coût construction | 50 bois, 100 pierre, 20 fer |
| Points de construction | **100 pts** (+5 % par niveau de caserne) |
| Formule pts | `100 * (1 + 0.05 * niveau_caserne)` |

### 3.3 Grenier

| Propriété | Valeur |
|-----------|--------|
| Effet | **+100 cap nourriture** par niveau |
| Coût construction | 50 bois, 50 pierre, 10 fer |
| Points de construction | **50 pts** (+10 % par niveau du grenier) |
| Formule pts | `50 * (1 + 0.10 * niveau_grenier)` |

### 3.4 Dépôt

| Propriété | Valeur |
|-----------|--------|
| Effet | **+100 cap stock matières premières** (bois + pierre + fer) par niveau |
| Coût construction | 30 bois, 30 pierre, 10 fer |
| Points de construction | **50 pts** (+5 % par niveau du dépôt) |
| Formule pts | `50 * (1 + 0.05 * niveau_depot)` |

### 3.5 Entrepôt

| Propriété | Valeur |
|-----------|--------|
| Effet | **+50 cap produits manufacturés** (outils + armes) par niveau |
| Coût construction | 40 bois, 40 pierre, 20 fer |
| Points de construction | **100 pts** (+5 % par niveau de l’entrepôt) |
| Formule pts | `100 * (1 + 0.05 * niveau_entrepot)` |

### 3.6 Tick construction

- Chaque **maçon** alloué à un type de bâtiment ajoute **1 pt de construction / jour** à ce bâtiment.
- Quand les points de construction atteignent le seuil requis, le niveau (ou la quantité) du bâtiment augmente, les ressources sont consommées, et la barre de construction repart à 0 pour le prochain niveau.

---

## 4. Coûts des clics

| Clic | Effet |
|------|--------|
| **Village** | +1 pop, **−30 nourriture** |
| **Champs** | +1 nourriture |
| **Château** | +1 soldat, **−50 nourriture**, **−5 armes** |
| **Ressource (bois, pierre, fer, outils)** | +1 de la ressource cliquée |

---

## 5. Caps (résumé)

| Cap | Formule |
|-----|---------|
| Cap population | `maisons * 4` |
| Cap soldats | `niveau_caserne * 10` |
| Cap nourriture | `niveau_grenier * 100` |
| Cap matières premières (bois+pierre+fer) | `niveau_depot * 100` |
| Cap produits manufacturés (outils+armes) | `niveau_entrepot * 50` |

---

## 6. UI — Cartes par bâtiment

Remplacement des **sliders** d’affectation par des **cartes par bâtiment** :

Pour chaque bâtiment (Maison, Caserne, Grenier, Dépôt, Entrepôt, Guilde des Maçons) :

- **Logo** (icône ou placeholder)
- **Niveau** (ou quantité pour maisons)
- **Nom**
- **Description**
- **Coût** (prochaine construction : bois, pierre, fer)
- En dessous, **aligné verticalement** :
  - Ligne 1 : **Gens alloués** (nombre) + bouton **+** et **−**
  - Ligne 2 : **Maçons alloués** (nombre) + bouton **+** et **−**

Pour la **Guilde des Maçons** : pas de niveau ni de construction ; action « 1 pop → 1 maçon » (bouton ou coût affiché).

---

## 7. Références

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyuClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Guide Implementation MVP](MiyuClicker%20-%20Guide%20Implementation%20MVP.md)
- Code : `state.rs` (bâtiments, maçons, caps), `idlesim.rs` (clics, construction), `app.rs` (ui_bar Maçons, cartes).
