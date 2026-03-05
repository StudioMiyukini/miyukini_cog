# MiyuClicker â€” BÃ¢timents, maÃ§ons et construction

## Contexte

Ce document dÃ©crit le **systÃ¨me de bÃ¢timents**, la **Guilde des MaÃ§ons** (transformation pop â†’ maÃ§on), les **coÃ»ts de clic**, les **caps de stock** et lâ€™**UI par cartes** (logo, niveau, nom, description, coÃ»t, affectation gens/maÃ§ons avec boutons +/âˆ’).

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre :** BÃ¢timents (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t), Guilde des MaÃ§ons, maÃ§ons (1 pop â†’ 1 maÃ§on), construction (1 pt/jour par maÃ§on allouÃ©), Ã©tat de dÃ©part, clics, UI cartes.
- **RÃ©fÃ©rence code :** `state` (niveaux bÃ¢timents, maÃ§ons, allocation maÃ§ons, caps), `idlesim` (clics, tick construction), `app` (barre MaÃ§ons, cartes bÃ¢timent).

---

## 1. Ã‰tat de dÃ©part (nouvelle partie)

| DonnÃ©e | Valeur |
|--------|--------|
| BÃ¢timents | Niveau 1 + **3 maisons** |
| Population | **10** |
| Nourriture | **20** |
| Maisons | 3 |
| Caserne | niveau 1 |
| Grenier | niveau 1 |
| DÃ©pÃ´t | niveau 1 |
| EntrepÃ´t | niveau 1 |
| MaÃ§ons | 0 |

---

## 2. Guilde des MaÃ§ons et maÃ§ons

- **Guilde des MaÃ§ons** : bÃ¢timent / action permettant de transformer **1 pop (gens) en 1 maÃ§on**.
- **MaÃ§ons** : compteur affichÃ© dans la barre **aprÃ¨s les Soldats**.
- Chaque maÃ§on allouÃ© Ã  un bÃ¢timent en construction apporte **1 pt de construction / jour** Ã  ce bÃ¢timent.
- Allocation des maÃ§ons : par type de bÃ¢timent (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t) ; boutons +/âˆ’ sur chaque carte.

---

## 3. BÃ¢timents et construction

### 3.1 Maison

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Effet | **+4 cap population** par maison |
| CoÃ»t construction | 30 bois, 20 pierre, 5 fer |
| Points de construction | **30 pts** (+1 % par maison existante) |
| Formule pts | `30 * (1 + 0.01 * nb_maisons)` |

### 3.2 Caserne

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Effet | **+10 cap soldats** par niveau |
| CoÃ»t construction | 50 bois, 100 pierre, 20 fer |
| Points de construction | **100 pts** (+5 % par niveau de caserne) |
| Formule pts | `100 * (1 + 0.05 * niveau_caserne)` |

### 3.3 Grenier

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Effet | **+100 cap nourriture** par niveau |
| CoÃ»t construction | 50 bois, 50 pierre, 10 fer |
| Points de construction | **50 pts** (+10 % par niveau du grenier) |
| Formule pts | `50 * (1 + 0.10 * niveau_grenier)` |

### 3.4 DÃ©pÃ´t

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Effet | **+100 cap stock matiÃ¨res premiÃ¨res** (bois + pierre + fer) par niveau |
| CoÃ»t construction | 30 bois, 30 pierre, 10 fer |
| Points de construction | **50 pts** (+5 % par niveau du dÃ©pÃ´t) |
| Formule pts | `50 * (1 + 0.05 * niveau_depot)` |

### 3.5 EntrepÃ´t

| PropriÃ©tÃ© | Valeur |
|-----------|--------|
| Effet | **+50 cap produits manufacturÃ©s** (outils + armes) par niveau |
| CoÃ»t construction | 40 bois, 40 pierre, 20 fer |
| Points de construction | **100 pts** (+5 % par niveau de lâ€™entrepÃ´t) |
| Formule pts | `100 * (1 + 0.05 * niveau_entrepot)` |

### 3.6 Tick construction

- Chaque **maÃ§on** allouÃ© Ã  un type de bÃ¢timent ajoute **1 pt de construction / jour** Ã  ce bÃ¢timent.
- Quand les points de construction atteignent le seuil requis, le niveau (ou la quantitÃ©) du bÃ¢timent augmente, les ressources sont consommÃ©es, et la barre de construction repart Ã  0 pour le prochain niveau.

---

## 4. CoÃ»ts des clics

| Clic | Effet |
|------|--------|
| **Village** | +1 pop, **âˆ’30 nourriture** |
| **Champs** | +1 nourriture |
| **ChÃ¢teau** | +1 soldat, **âˆ’50 nourriture**, **âˆ’5 armes** |
| **Ressource (bois, pierre, fer, outils)** | +1 de la ressource cliquÃ©e |

---

## 5. Caps (rÃ©sumÃ©)

| Cap | Formule |
|-----|---------|
| Cap population | `maisons * 4` |
| Cap soldats | `niveau_caserne * 10` |
| Cap nourriture | `niveau_grenier * 100` |
| Cap matiÃ¨res premiÃ¨res (bois+pierre+fer) | `niveau_depot * 100` |
| Cap produits manufacturÃ©s (outils+armes) | `niveau_entrepot * 50` |

---

## 6. UI â€” Cartes par bÃ¢timent

Remplacement des **sliders** dâ€™affectation par des **cartes par bÃ¢timent** :

Pour chaque bÃ¢timent (Maison, Caserne, Grenier, DÃ©pÃ´t, EntrepÃ´t, Guilde des MaÃ§ons) :

- **Logo** (icÃ´ne ou placeholder)
- **Niveau** (ou quantitÃ© pour maisons)
- **Nom**
- **Description**
- **CoÃ»t** (prochaine construction : bois, pierre, fer)
- En dessous, **alignÃ© verticalement** :
  - Ligne 1 : **Gens allouÃ©s** (nombre) + bouton **+** et **âˆ’**
  - Ligne 2 : **MaÃ§ons allouÃ©s** (nombre) + bouton **+** et **âˆ’**

Pour la **Guilde des MaÃ§ons** : pas de niveau ni de construction ; action Â« 1 pop â†’ 1 maÃ§on Â» (bouton ou coÃ»t affichÃ©).

---

## 7. RÃ©fÃ©rences

- [MiyuClicker - MVP Ecrans et Mecaniques](MiyukiniClicker%20-%20MVP%20Ecrans%20et%20Mecaniques.md)
- [MiyuClicker - Guide Implementation MVP](MiyukiniClicker%20-%20Guide%20Implementation%20MVP.md)
- Code : `state.rs` (bÃ¢timents, maÃ§ons, caps), `idlesim.rs` (clics, construction), `app.rs` (ui_bar MaÃ§ons, cartes).

