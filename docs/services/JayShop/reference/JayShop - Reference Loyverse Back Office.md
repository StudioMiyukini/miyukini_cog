# JayShop — Reference Loyverse Back Office

## Contexte

Ce document catalogue et annote les **screenshots du back office Loyverse POS** captures le 11/02/2026. Loyverse est une reference d'inspiration pour la conception de JayShop, en particulier pour le **point de vente (PoS)**, la **gestion des articles**, les **parametres de caisse** et les **rapports de vente**.

Les screenshots sont stockes dans `reference/screenshot back office Loyverse/`.

## Portee / Scope

- **Perimetre** : Inventaire annote des ecrans Loyverse, identification des fonctionnalites a reprendre, adapter ou ignorer pour JayShop.
- **Hors perimetre** : Reproduction exacte de l'interface Loyverse (JayShop a sa propre identite UI).

---

## 1. Architecture du back office Loyverse

### 1.1 Navigation principale (sidebar)

La sidebar Loyverse comporte les sections suivantes (icones de haut en bas) :

| Icone | Section | Description |
|-------|---------|-------------|
| Profil | Compte utilisateur | Gestion du profil et du compte. |
| Graphique | **Reports** | Rapports de vente (summary, by item, category, employee, payment type, receipts, modifier, discounts, taxes). |
| Panier | **Items** | Gestion des articles (Item list, Categories, Modifiers, Discounts). |
| Chariot | **Inventory management** | Gestion avancee du stock (bons de commande, transferts, ajustements, comptages, production, historique). |
| Carte | **Customers** | Gestion des clients (creation, fiche client). |
| Personnes | **Employees** | Gestion des employes et droits d'acces. |
| Puzzle | **Extensions** | Apps et integrations tierces. |
| Engrenage | **Settings** | Parametres (Features, Billing, Payment types, Loyalty, Taxes, Receipt, Stores, POS devices). |
| Aide | **Help** | Centre d'aide. |

**Enseignement pour JayShop** : la navigation est claire et plate. Chaque section majeure est un item de la sidebar. JayShop reprend cette approche dans sa navigation admin.

---

## 2. Rapports (Reports)

### 2.1 Sales Summary

![Sales Summary](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20161736.png)
![Sales Summary alt](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162213.png)

**Ecran** : Tableau de bord des ventes sur une periode configurable.

**Composants observes** :
- Selecteur de periode (dates, fleches precedent/suivant)
- Filtres : « All day » (horaire), « All employees »
- **5 KPIs en haut** : Gross sales, Refunds, Discounts, Net sales, Gross profit — chacun avec montant et pourcentage de variation
- **Graphique** : barres ou lignes, granularite jour/semaine/mois
- **Bouton Export** en bas

**Enseignement pour JayShop** :
- Le tableau de bord JSH-A01 doit reprendre cette disposition : KPIs en bande horizontale + graphique + export.
- Ajouter les KPIs **Remboursements** et **Remises** (absents du spec actuel).
- Ajouter le filtre par employe/vendeur si multi-vendeurs en Phase 2.

### 2.2 Sous-rapports disponibles

![Reports menu](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162349.png)

**Rapports proposes par Loyverse** :

| Rapport | Description | Equivalent JayShop |
|---------|-------------|--------------------|
| Sales summary | Vue globale CA, remboursements, remises | JSH-A01 Tableau de bord |
| Sales by item | CA ventile par produit | A ajouter (JSH-54 enrichi) |
| Sales by category | CA ventile par categorie | A ajouter |
| Sales by employee | CA ventile par employe/vendeur | Phase 2 (multi-vendeurs) |
| Sales by payment type | CA ventile par mode de paiement | Presente dans cloture caisse, a detailler en rapport |
| Receipts | Liste des tickets avec filtres | JSH-A10 Historique |
| Sales by modifier | CA ventile par modificateur | Depend de l'ajout des modificateurs |
| Discounts | Detail des remises appliquees | A ajouter |
| Taxes | Detail des taxes collectees | A ajouter |

**Enseignement pour JayShop** : enrichir le tableau de bord avec des sous-vues (par produit, categorie, mode de paiement, taxes). Voir besoins JSH-54-bis a JSH-54-sept dans l'analyse des besoins enrichie.

### 2.3 Receipts (Tickets)

![Receipts](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162527.png)

**Composants observes** :
- **3 compteurs** en haut : All receipts, Sales, Refunds (avec icones distinctes)
- **Filtres** : periode, horaire, employe
- **Tableau** : Receipt no., Date, Employee, Customer, Type, Total
- **Bouton Export**
- **Recherche** (loupe)
- Etat vide avec message explicite (« No data to display »)

**Enseignement pour JayShop** : le JSH-A10 (Historique des tickets) doit reprendre les compteurs en haut (Total, Ventes, Remboursements) et la colonne Type (vente/remboursement).

---

## 3. Articles (Items)

### 3.1 Menu articles

![Items menu](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162400.png)

**Sous-sections** : Item list, Categories, Modifiers, Discounts.

**Enseignement pour JayShop** : les **Modifiers** et les **Discounts** sont des entites a part entiere (pas juste un champ sur une ligne de ticket). A integrer dans JayShop.

### 3.2 Liste des articles (Item list)

![Item list](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162602.png)

**Composants observes** :
- **Boutons** : + Add Item, Import, Export
- **Filtres** : Category (dropdown), Stock alert (dropdown)
- **Recherche** (loupe)
- **Colonnes** : Item name (tri), Category (dropdown inline), Price, Cost, Margin (%), In stock
- Articles avec **variantes** (fleche expandable, ex. « Badge bois », « Badges Studio Miyukini »)
- **Pagination** : Page X of Y, Rows per page

**Enseignements pour JayShop** :
- Ajouter **Import** (CSV) en complement de l'Export. Nouveau besoin JSH-17.
- Ajouter les colonnes **Cout** et **Marge** (utiles pour le tableau de bord profitabilite). Champs optionnels.
- Supporter les **variantes** de produit (taille, couleur, option). Nouveau besoin JSH-18.
- Filtre par **alerte de stock** (bas, epuise). Nouveau besoin JSH-19.

### 3.3 Creation d'article

![Create item top](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162609.png)
![Create item bottom - inventory, variants, taxes](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162614.png)
![Create item bottom - variants, taxes alt](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162618.png)

**Champs du formulaire de creation** :

| Section | Champs | Observations |
|---------|--------|-------------|
| **Infos principales** | Name, Category (dropdown), Description | Similaire au modele JayXpose. |
| **Disponibilite** | Checkbox « The item is available for sale » | Equivalent au champ `availability` de JayXpose. |
| **Vente par** | Each / Weight-Volume (radio) | **Nouveau pour JayShop** : vente a l'unite ou au poids/volume. |
| **Prix** | Price, Cost | Cost = prix d'achat (calcul marge). Champ optionnel pour JayShop. |
| **Identifiants** | SKU (auto-genere), Barcode | SKU deja present (mapping JayXpose). Barcode a ajouter au modele. |
| **Inventory** | Composite item (toggle), Track stock (toggle) | Composite item = kit/pack. Track stock = suivi de stock actif. |
| **Variants** | Add Variants → Create options (nom d'option + valeurs) | Gestion de variantes (taille, couleur, etc.). |
| **Taxes** | Liste de taxes avec toggles par article | Association taxes ↔ articles. |

**Enseignements pour JayShop** :
- **Vente au poids/volume** : utile pour marche alimentaire. A ajouter en Phase 2 (JSH-18b).
- **Cout (prix d'achat)** : permet le calcul de la marge. Champ optionnel a relayer vers JayXpose.
- **Articles composites (kits)** : un produit compose de plusieurs sous-produits. Phase 2 (JSH-18c).
- **Association taxes par article** : plus flexible que le taux unique actuel. A enrichir dans JSH-04.

### 3.4 Variantes — Dialogue d'options

![Create options](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162623.png)

**Composants observes** :
- Nom de l'option (ex. « Taille », « Couleur »)
- Valeurs de l'option (saisie libre, ajout par Entree)
- Bouton « + Add Option » pour ajouter d'autres axes de variation
- Chaque combinaison de valeurs genere une variante avec son propre stock/prix

**Enseignement pour JayShop** : les variantes sont gerees comme des « axes d'options » combinatoires. A refleter dans le modele de donnees JayXpose (extension catalogue).

### 3.5 Representation sur le PoS

![Representation on POS](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162636.png)

**Composants observes** :
- **Deux modes** : « Color and shape » ou « Image »
- **8 couleurs** : gris, rouge, rose, orange, jaune-vert, vert, bleu, violet
- **4 formes** : carre (defaut), rond, etoile/soleil, hexagone
- **Upload d'image** : photo du produit comme visuel du bouton

**Enseignement pour JayShop** : enrichir la configuration des boutons PoS (JSH-A07) avec :
- Choix de **forme** (pas seulement couleur) → carre, rond, badge, hexagone
- Option **image** en alternative a couleur+forme (utilise le visuel produit de JayXpose)
- Cela ameliore la reconnaissance visuelle rapide en caisse

### 3.6 Categories

![Categories list](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162547.png)
![Create category](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162551.png)

**Composants observes** :
- Liste : nom, pastille de couleur, nombre d'articles
- Creation : champ Name + palette de 8 couleurs (meme palette que les boutons PoS)
- Pagination en bas

**Enseignement pour JayShop** : les categories ont une **couleur** associee (pas seulement un nom). Cette couleur peut etre utilisee pour :
- Colorer les onglets PoS
- Colorer les boutons des produits de cette categorie par defaut
- Faciliter l'identification visuelle

### 3.7 Remises (Discounts)

![Discounts list](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162729.png)

**Composants observes** :
- Liste : Name, Value (montant fixe ou %), Restricted access (Yes/No)
- Les remises sont des **entites pre-definies** (pas juste une saisie libre au moment du ticket)
- Exemples : « -50% mochi » (£2.50), « Badge » (£1.00), « Badges 5x » (50%), « Cartes A5 3x » (£2.00)
- **Restricted access** : seuls certains employes peuvent appliquer certaines remises

**Enseignement pour JayShop** :
- Les remises doivent etre des **entites pre-configurees** (nom, valeur, type %, type montant). Nouveau besoin JSH-90.
- Ajouter un controle d'acces sur les remises (Phase 2, multi-vendeurs).
- Les remises pre-definies accelerent la saisie en caisse (pas besoin de saisir manuellement).

---

## 4. Gestion des stocks (Inventory)

![Advanced inventory](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162414.png)

**Fonctionnalites Loyverse** :

| Fonctionnalite | Description | Pertinence JayShop |
|----------------|-------------|--------------------|
| Purchase orders | Bons de commande fournisseur | Phase 2+ (hors perimetre initial, gere par JayXpose) |
| Transfer orders | Transferts entre magasins | Hors perimetre (mono-magasin) |
| Stock adjustments | Ajustements manuels (reception, perte, casse) | Pertinent — relaye vers JayXpose |
| Inventory counts | Inventaire physique (scan ou manuel) | Phase 2 |
| Production | Stock produit a partir d'ingredients | Phase 2 (articles composites) |
| Inventory history | Historique de tous les mouvements | Pertinent — journal de sync JayXpose |

**Enseignement pour JayShop** : la gestion de stock avancee reste dans JayXpose (source de verite). JayShop peut offrir un ecran simplifie d'**ajustement de stock** (reception, perte, casse) relaye a JayXpose. Nouveau besoin JSH-74.

---

## 5. Clients (Customers)

![Create customer](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162426.png)

**Champs du formulaire client** :
- Avatar
- Name
- Email
- Phone
- Address, Town/City, County, Postcode, Country

**Enseignement pour JayShop** :
- Un **fichier client** basique est utile meme en PoS (fidelite, historique d'achats, envoi de recu par email).
- JayShop peut s'appuyer sur **MiyuContacts** pour la gestion des clients.
- Ajouter un besoin JSH-91 : association client ↔ ticket (optionnel, pour fidelite et historique).

---

## 6. Parametres (Settings)

### 6.1 Features (fonctionnalites activables)

![Settings Features](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162746.png)

**Fonctionnalites activables par toggle** :

| Fonctionnalite | Description | Pertinence JayShop |
|----------------|-------------|--------------------|
| **Shifts** | Suivi des periodes de travail (tiroir-caisse) | Oui — JSH-63 (sessions de caisse) |
| **Time clock** | Pointage employes | Phase 2 (multi-employes) |
| **Open tickets** | Sauvegarder/editer des tickets avant paiement | Oui — JSH-36 (tickets ouverts) |
| **Kitchen printers** | Impression vers cuisine | Phase 2 (restauration, JayFaim) |
| **Customer displays** | Affichage client (second ecran) | Phase 2 |
| **Dining options** | Dine in / Takeout / Delivery | Phase 2 (restauration) |
| **Low stock notifications** | Alertes par email quand stock bas | Pertinent — nouveau besoin JSH-75 |
| **Negative stock alerts** | Avertir si vente malgre stock negatif | Pertinent — nouveau besoin JSH-76 |
| **Weight embedded barcodes** | Scan codes-barres avec poids integre | Phase 2 |

**Enseignement pour JayShop** :
- Les fonctionnalites avancees sont **activables** (pas toujours visibles). Cela simplifie l'interface pour les vendeurs simples.
- Ajouter un ecran **Parametres > Fonctionnalites** avec des toggles. Nouveau besoin JSH-06.
- Les alertes de stock et l'affichage client sont des enrichissements de valeur.

### 6.2 Taxes

![Settings Taxes](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162755.png)
![Create tax](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162759.png)
![Taxes on item](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162701.png)

**Composants observes** :
- Liste des taxes : Name, « Apply to new items » (Yes/No), Tax rate (%)
- Creation : Name, Tax rate %, Type (« Included in the price » / « Added to the price »), Apply to items (selection)
- Sur un article : liste de toutes les taxes avec toggles individuels

**Taxes observees (usage reel)** :
- commission Artiste (25%), Commission Edite (35%), Commission Membre (15%), Commission Permanent (12%)
- Tva (20%), TVA ali. (5.5%), TVA free (0%), TVA Resto (10%)

**Enseignement pour JayShop** :
- Le systeme de taxes est **multi-taux** et **par article** (pas un taux global unique).
- Les « commissions » sont aussi modelisees comme des taxes (usage creatif de Loyverse).
- JayShop doit supporter : **taxes incluses vs ajoutees**, **multi-taxes par article**, **application par defaut aux nouveaux articles**.
- Enrichir JSH-04 (configuration fiscale) avec un modele de taxes a part entiere.

### 6.3 Types de paiement

![Payment types](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162813.png)
![Create payment type](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162823.png)

**Composants observes** :
- Liste : Cash, CB, SumUp (reordonnables par drag-and-drop)
- Creation : dropdown avec types pre-definis (Card, Check, Other, SumUp, Teya, Zettle)
- Chaque type de paiement = un bouton distinct sur l'ecran de paiement

**Enseignement pour JayShop** :
- Les modes de paiement sont **configurables** et **reordonnables** (pas une liste fixe).
- Supporter les **integrations terminaux de paiement** (SumUp, Zettle, etc.) en Phase 2.
- Le mode de paiement par defaut est Cash (especes). Aligne avec la spec actuelle.

### 6.4 Fidelite (Loyalty)

![Loyalty settings](./screenshot%20back%20office%20Loyverse/Capture%20d'%C3%A9cran%202026-02-11%20162849.png)

**Composants observes** :
- Configuration simple : « Points earning percentage » (% du montant d'achat credite en points)
- Associe a la fiche client

**Enseignement pour JayShop** :
- Programme de fidelite simple (% du CA en points). Phase 2 (deja prevu).
- Necessite un fichier client (voir JSH-91).

---

## 7. Synthese des enrichissements identifies

### 7.1 Fonctionnalites a ajouter au MVP ou P1

| Id | Fonctionnalite | Source Loyverse | Impact JayShop |
|----|----------------|-----------------|----------------|
| JSH-06 | Ecran Parametres > Fonctionnalites (toggles) | Settings > Features | JSH-A13 enrichi |
| JSH-17 | Import CSV d'articles | Item list > Import | JSH-A02 enrichi |
| JSH-74 | Ajustement de stock simplifie depuis JayShop | Stock adjustments | Nouveau ecran JSH-A15 |
| JSH-75 | Alertes de stock bas | Low stock notifications | Notification / badge |
| JSH-76 | Alerte vente avec stock negatif | Negative stock alerts | Avertissement PoS |
| JSH-90 | Remises pre-definies (entites) | Discounts | Nouveau ecran / modele |
| JSH-91 | Association client ↔ ticket | Create customer | Enrichissement PoS |
| — | Couleur sur les categories | Categories > color | Config PoS enrichie |
| — | Forme + image sur les boutons PoS | Representation on POS | JSH-A07 enrichi |
| — | Multi-taxes par article (incluse/ajoutee) | Taxes | JSH-04 enrichi |

### 7.2 Fonctionnalites Phase 2+

| Fonctionnalite | Source Loyverse | Note |
|----------------|-----------------|------|
| Variantes de produit (taille, couleur) | Variants | Extension modele JayXpose |
| Articles composites / kits | Composite item | Extension modele JayXpose |
| Vente au poids/volume | Sold by Weight/Volume | Marche alimentaire |
| Integration terminaux de paiement (SumUp, Zettle) | Payment types | Phase 2 |
| Programme de fidelite | Loyalty | Phase 2 |
| Multi-employes et pointage | Employees, Time clock | Phase 2 |
| Affichage client (second ecran) | Customer displays | Phase 2 |
| Impressions cuisine | Kitchen printers | Phase 2 (JayFaim) |
| Bons de commande fournisseur | Purchase orders | JayXpose / Phase 3 |
| Inventaire physique (scan) | Inventory counts | Phase 2 |

---

## 8. Correspondance ecrans Loyverse ↔ JayShop

| Ecran Loyverse | Ecran JayShop | Notes |
|----------------|---------------|-------|
| Sales summary | JSH-A01 Tableau de bord | Enrichir avec KPIs remboursements, remises, profit |
| Receipts | JSH-A10 Historique des tickets | Ajouter compteurs en haut |
| Item list | JSH-A02 Liste des produits | Ajouter colonnes Cout, Marge, Import |
| Create item | JSH-A03 Formulaire produit | Ajouter vente au poids, barcode, variantes (Phase 2) |
| Categories | JSH-A04 Gestion des categories | Ajouter couleur par categorie |
| Discounts | *Nouveau* JSH-A16 Gestion des remises | Entites pre-definies |
| Customers | Via MiyuContacts + JSH-A17 | Fiche client associee au ticket |
| Settings > Features | JSH-A13 Parametres (enrichi) | Toggles fonctionnalites |
| Settings > Payment types | JSH-A13 > Modes de paiement | Deja prevu |
| Settings > Taxes | JSH-A13 > Taxes (enrichi) | Multi-taux, incluse/ajoutee |
| Settings > Receipt | JSH-A07 > Parametres recu | Deja prevu |
| Settings > Loyalty | Phase 2 | Programme de fidelite |
| Inventory management | Via JayXpose + JSH-A15 | Ajustement simplifie |
| Representation on POS | JSH-A07 Configuration PoS | Ajouter forme et mode image |

---

## 9. References

- [JayShop - Document Fondateur](../JayShop%20-%20Document%20Fondateur.md)
- [JayShop - Analyse des besoins](../JayShop%20-%20Analyse%20des%20besoins.md)
- [JayShop - Ecrans et UI](../JayShop%20-%20Ecrans%20et%20UI.md)
- Screenshots : `reference/screenshot back office Loyverse/`

---

**Document** : JayShop — Reference Loyverse Back Office
**Version** : 1.0
**Date** : 2026-02-11
**Statut** : Reference produit — Analyse concurrentielle
