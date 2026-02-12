# JayShop — Ecrans et UI

## Contexte

Ce document decrit la **cartographie des ecrans** du service **JayShop**, les composants principaux et les flux d'interaction. Il couvre l'interface admin (Central) et l'interface client (Portail), avec un focus sur le **mode PoS** et l'**ecran de paiement**.

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), [Parcours Utilisateur](./JayShop%20-%20Parcours%20Utilisateur.md), [Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md).

## Portee / Scope

- **Perimetre** : Inventaire des ecrans, composants, disposition, flux de navigation.
- **Hors perimetre** : Maquettes pixel-perfect, specifications CSS, implementation technique.

---

## 1. Cartographie des ecrans

### 1.1 Ecrans Admin (Central)

| Id | Ecran | Description | Besoins |
|----|-------|-------------|---------|
| JSH-A01 | Tableau de bord | Vue synthetique : CA, tickets, panier moyen, top produits, graphiques. | JSH-54 |
| JSH-A02 | Liste des produits | Catalogue complet, filtres, actions CRUD, marquage PoS. | JSH-10 a JSH-16 |
| JSH-A03 | Formulaire produit | Creation / edition d'un produit (nom, prix, categorie, visuel, disponibilite). | JSH-11, JSH-12 |
| JSH-A04 | Gestion des categories | Liste, creation, edition, suppression, reordonnancement. | JSH-14 |
| JSH-A05 | PoS — Ecran principal | Ticket en cours (gauche) + grille boutons par onglets (droite). | JSH-30 a JSH-36 |
| JSH-A06 | PoS — Ecran de paiement | Recap ticket, saisie montant, modes de paiement, rendu monnaie. | JSH-40 a JSH-47 |
| JSH-A07 | PoS — Configuration | Gestion des onglets, boutons, couleurs, taille, disposition. | JSH-05 |
| JSH-A08 | PoS — Ouverture de caisse | Saisie du fond de caisse initial. | JSH-60 |
| JSH-A09 | PoS — Cloture de caisse | Synthese session, saisie especes comptees, ecart, validation. | JSH-62 |
| JSH-A10 | Historique des tickets | Liste chronologique, filtres, acces au detail. | JSH-50 |
| JSH-A11 | Detail d'un ticket | Lignes, paiements, remise, recu, actions (rembourser, reimprimer). | JSH-51, JSH-52 |
| JSH-A12 | Commandes en ligne | Liste des commandes web, statuts, actions (preparer, expedier). | JSH-25 |
| JSH-A13 | Parametres boutique | Configuration generale (nom, slug, devise, CGV, modes paiement, fonctionnalites activables, recu). | JSH-02 a JSH-06 |
| JSH-A14 | Mouvements de caisse | Entrees/sorties manuelles d'especes. | JSH-61 |
| JSH-A15 | Ajustement de stock | Ajustement manuel du stock (reception, perte, casse). Relaye a JayXpose. | JSH-74 |
| JSH-A16 | Gestion des remises | Liste, creation, edition, suppression des remises pre-definies (nom, valeur, type). | JSH-90, JSH-91 |
| JSH-A17 | Fichier client | Liste des clients, creation/edition fiche client. Association avec MiyuContacts. | JSH-98, JSH-99 |
| JSH-A18 | Gestion des taxes | Liste, creation, edition des taux de taxe (nom, taux, incluse/ajoutee, application par defaut). | JSH-94 a JSH-97 |
| JSH-A19 | Rapports detailles | Sous-rapports : ventes par produit, par categorie, par mode de paiement, taxes, remises. | JSH-55 a JSH-59 |
| **JSH-A20** | **Liste des evenements** | **Liste des participations evenements/festivals avec statut, dates, CA, benefice. Actions : creer, editer, ouvrir.** | **JSH-110, JSH-111** |
| **JSH-A21** | **Formulaire evenement** | **Creation / edition d'une fiche evenement (nom, dates, lieu, stand, notes). Affiche le lien JayFestival si lie.** | **JSH-110** |
| **JSH-A22** | **Couts de participation** | **Liste des couts par evenement (stand, transport, logement, nourriture, autres). CRUD, total par categorie.** | **JSH-112 a JSH-114** |
| **JSH-A23** | **Stock temporaire** | **Allocation de stock temporaire depuis le stock global. Selection produits, quantites. Indicateurs restant.** | **JSH-115, JSH-116** |
| **JSH-A24** | **Dashboard evenement** | **Vue synthetique par evenement : CA, couts, stock restant, benefice brut, benefice net. Actions : cloturer.** | **JSH-118, JSH-119** |

### 1.2 Ecrans Client (Portail — Boutique en ligne)

| Id | Ecran | Description | Besoins |
|----|-------|-------------|---------|
| JSH-C01 | Accueil boutique | Banniere, nom, description, produits vedettes. | JSH-20 |
| JSH-C02 | Catalogue | Liste/grille de produits, filtres, recherche, tri. | JSH-20 |
| JSH-C03 | Fiche produit | Visuels, description, prix, disponibilite, ajout au panier. | JSH-21 |
| JSH-C04 | Panier | Lignes, quantites, sous-total, taxes, total. | JSH-22 |
| JSH-C05 | Commande | Informations client, recap, choix du mode de paiement. | JSH-23 |
| JSH-C06 | Confirmation | Numero de ticket, recap, message de confirmation. | JSH-24 |
| JSH-C07 | Mes commandes | Liste des commandes passees, statut. | JSH-26 |
| JSH-C08 | Detail commande | Lignes, montants, statut, historique. | JSH-26 |

---

## 2. Navigation

### 2.1 Navigation Admin (sidebar Central)

```
JayShop
├── Tableau de bord          [JSH-A01]
├── Point de vente (PoS)     [JSH-A05]
│   ├── Ouverture caisse     [JSH-A08]
│   ├── Ecran de caisse      [JSH-A05]
│   ├── Mouvements caisse    [JSH-A14]
│   └── Cloture caisse       [JSH-A09]
├── Produits                 [JSH-A02]
│   ├── Liste produits       [JSH-A02]
│   ├── Ajouter produit      [JSH-A03]
│   └── Categories           [JSH-A04]
├── Commandes en ligne       [JSH-A12]
├── Historique des ventes    [JSH-A10]
│   └── Detail ticket        [JSH-A11]
├── **Evenements**           [JSH-A20]   ← NOUVEAU
│   ├── Liste evenements     [JSH-A20]
│   ├── Nouvel evenement     [JSH-A21]
│   ├── Couts                [JSH-A22]
│   ├── Stock temporaire     [JSH-A23]
│   └── Dashboard evenement  [JSH-A24]
├── Clients                  [JSH-A17]
├── Remises                  [JSH-A16]
├── Rapports detailles       [JSH-A19]
│   ├── Par produit
│   ├── Par categorie
│   ├── Par mode de paiement
│   ├── Taxes collectees
│   ├── Remises appliquees
│   └── **Par evenement**    ← NOUVEAU
├── Configuration PoS        [JSH-A07]
└── Parametres               [JSH-A13]
    ├── General (nom, slug, devise, CGV)
    ├── Fonctionnalites (toggles)    [JSH-06]
    ├── Modes de paiement
    ├── Taxes                [JSH-A18]
    └── Recu
```

### 2.2 Navigation Client (Portail)

```
Boutique [Nom]
├── Accueil                  [JSH-C01]
├── Catalogue                [JSH-C02]
│   └── Fiche produit        [JSH-C03]
├── Panier                   [JSH-C04]
├── Commande                 [JSH-C05]
│   └── Confirmation         [JSH-C06]
└── Mes commandes            [JSH-C07]
    └── Detail commande      [JSH-C08]
```

---

## 3. Ecrans detailles

### 3.1 JSH-A05 — PoS : Ecran principal

**Layout :** Split horizontal (60/40 ou 50/50 selon ecran).

```
┌─────────────────────────────────────┬──────────────────────────────────────┐
│           TICKET EN COURS           │          GRILLE PRODUITS             │
│                                     │                                      │
│  #JSHOP-2026-00042                  │  [Tous] [Boissons] [Plats] [Dessert]│
│                                     │  ┌──────┐ ┌──────┐ ┌──────┐        │
│  Cafe latte          x2     6.00 €  │  │Cafe  │ │Cappuc│ │Latte │        │
│  Croissant beurre    x1     1.80 €  │  │ 3.00€│ │ 3.50€│ │ 3.00€│        │
│  Jus d'orange        x1     3.50 €  │  └──────┘ └──────┘ └──────┘        │
│                                     │  ┌──────┐ ┌──────┐ ┌──────┐        │
│                                     │  │The   │ │Chocol│ │Eau   │        │
│                                     │  │ 2.50€│ │ 3.80€│ │ 1.50€│        │
│                                     │  └──────┘ └──────┘ └──────┘        │
│                                     │  ┌──────┐ ┌──────┐ ┌──────┐        │
│                                     │  │Crssnt│ │Pain  │ │Tartin│        │
│                                     │  │ 1.80€│ │ 1.20€│ │ 2.50€│        │
│  ────────────────────────────────   │  └──────┘ └──────┘ └──────┘        │
│  Sous-total HT          9.42 €     │                                      │
│  TVA (20%)              1.88 €     │  ┌──────────────────────────────┐    │
│  ══════════════════════════════     │  │  🔍 Rechercher un produit... │    │
│  TOTAL                 11.30 €     │  └──────────────────────────────┘    │
│                                     │                                      │
│ [🏷️ Remise] [💾 Sauver] [💳 Paiement]│                                   │
└─────────────────────────────────────┴──────────────────────────────────────┘
```

**Composants :**

| Composant | Description |
|-----------|-------------|
| Zone ticket | Liste des lignes (produit, quantite, prix). Clic sur une ligne pour modifier quantite ou supprimer. |
| Totaux | Sous-total HT, TVA, Total TTC. Mis a jour en temps reel. |
| Barre d'actions | Remise (ouvre un dialogue), Sauver (ticket ouvert), Paiement (ouvre JSH-A06). |
| Barre d'onglets | Onglets configurables. Clic = filtre la grille de boutons. |
| Grille de boutons | Boutons produits. Taille configurable (1x1, 2x1). Couleur configurable. Clic = +1 au ticket. |
| Recherche | Champ texte pour chercher un produit par nom. Resultats en liste deroulante. |

### 3.2 JSH-A06 — PoS : Ecran de paiement

**Layout :** Plein ecran (modale ou page).

```
┌────────────────────────────────────────────────────────────────┐
│                     PAIEMENT                                    │
│                                                                 │
│  ┌─────────────────────────────────────────┐                   │
│  │  RECAP                                   │                   │
│  │  Cafe latte          x2        6.00 €   │                   │
│  │  Croissant beurre    x1        1.80 €   │                   │
│  │  Jus d'orange        x1        3.50 €   │                   │
│  │  ──────────────────────────────────────  │                   │
│  │  Sous-total HT              9.42 €      │                   │
│  │  TVA (20%)                  1.88 €      │                   │
│  │  ══════════════════════════════════════  │                   │
│  │  TOTAL A PAYER             11.30 €      │                   │
│  └─────────────────────────────────────────┘                   │
│                                                                 │
│  ┌─────────────────────────────────────────┐                   │
│  │  RESTE A PAYER :          11.30 €       │                   │
│  └─────────────────────────────────────────┘                   │
│                                                                 │
│  MONTANT DONNE :                                                │
│  ┌──────────────────────┐                                       │
│  │       20.00           │  €                                   │
│  └──────────────────────┘                                       │
│                                                                 │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌───────┐                   │
│  │ 5 € │ │10 € │ │20 € │ │50 € │ │ Exact │                   │
│  └─────┘ └─────┘ └─────┘ └─────┘ └───────┘                   │
│                                                                 │
│  ┌───────────┐  ┌───────────┐  ┌────────────┐                 │
│  │  7  8  9  │  │ RENDU     │  │            │                  │
│  │  4  5  6  │  │ MONNAIE : │  │  8.70 €   │                  │
│  │  1  2  3  │  │           │  │            │                  │
│  │  0  .  ⌫  │  └───────────┘  └────────────┘                 │
│  └───────────┘                                                  │
│                                                                 │
│  MODE DE PAIEMENT :                                             │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │
│  │ 💵       │ │ 💳       │ │ 📄       │ │ 🏦       │          │
│  │ Especes  │ │ Carte    │ │ Cheque   │ │ Virement │          │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │
│                                                                 │
│  Paiements enregistres :                                        │
│  (vide — aucun paiement partiel pour l'instant)                │
│                                                                 │
│  ┌────────────────────────────────────┐                         │
│  │     ✅ VALIDER LE PAIEMENT         │ (grise si reste > 0)   │
│  └────────────────────────────────────┘                         │
│                                                                 │
│  [← Retour au ticket]                                           │
└────────────────────────────────────────────────────────────────┘
```

**Composants :**

| Composant | Description |
|-----------|-------------|
| Recap ticket | Lignes, sous-total, TVA, total. Lecture seule. |
| Reste a payer | Montant restant. Decroit a chaque paiement partiel. Mis a jour en temps reel. |
| Champ montant donne | Saisie libre du montant remis par le client. Peut etre rempli via les boutons rapides ou le pave numerique. |
| Boutons montant rapide | 5 EUR, 10 EUR, 20 EUR, 50 EUR, Exact (= reste a payer). Remplissent le champ montant. |
| Pave numerique | Pave virtuel (0-9, point, effacer). Pour saisie tactile (tablette, ecran tactile). |
| Calcul rendu monnaie | Affichage dynamique. Si montant donne >= reste a payer → « Rendu monnaie : X EUR ». Si montant < reste → « Reste a payer : X EUR ». |
| Boutons mode de paiement | Un bouton par mode configure. Clic = enregistre le paiement avec le montant saisi et le mode. |
| Liste paiements partiels | Si paiement mixte : affiche chaque paiement deja enregistre (mode, montant). |
| Bouton valider | Actif quand reste a payer <= 0. Cloture le ticket. |
| Retour | Retour au ticket en cours (JSH-A05) sans perdre les donnees. |

**Flux d'interaction :**

```
Etat initial :
  reste_a_payer = total_ttc
  montant_donne = 0

Boucle :
  1. L'admin saisit un montant (champ ou bouton rapide)
  2. Le rendu monnaie/reste a payer est calcule dynamiquement
  3. L'admin clique sur un mode de paiement
  4. → Un paiement partiel est enregistre :
       montant_paye = min(montant_donne, reste_a_payer)
       reste_a_payer -= montant_paye
       Si mode = especes et montant_donne > montant_paye :
         rendu_monnaie = montant_donne - montant_paye
  5. Si reste_a_payer > 0 → retour a l'etape 1 (paiement mixte)
  6. Si reste_a_payer <= 0 → bouton « Valider » actif
  7. Clic « Valider » → cloture ticket
```

### 3.3 JSH-A07 — Configuration PoS

**Layout :** Page de configuration en deux zones.

```
┌────────────────────────────────────────────────────────────────┐
│              CONFIGURATION DU POINT DE VENTE                   │
│                                                                 │
│  ONGLETS                                                        │
│  ┌──────────┬──────────┬──────────┬──────────┬───────┐         │
│  │ Boissons │ Plats    │ Desserts │ Favoris  │  [+]  │         │
│  └──────────┴──────────┴──────────┴──────────┴───────┘         │
│  (glisser-deposer pour reordonner)                              │
│                                                                 │
│  BOUTONS DE L'ONGLET « Boissons »                               │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐                          │
│  │Cafe  │ │Cappuc│ │Latte │ │The   │                           │
│  │ 🟤   │ │ 🟤   │ │ 🟤   │ │ 🟢   │                          │
│  └──────┘ └──────┘ └──────┘ └──────┘                           │
│  (glisser-deposer pour reordonner, clic droit pour editer)     │
│                                                                 │
│  [+ Ajouter un bouton]                                          │
│                                                                 │
│  PROPRIETES DU BOUTON                                           │
│  ┌──────────────────────────────────────┐                      │
│  │ Produit : [Cafe latte         ▼]     │                      │
│  │ Label :   [Cafe                 ]     │                      │
│  │                                       │                      │
│  │ Representation :                      │                      │
│  │ (●) Couleur + Forme  ( ) Image       │                      │
│  │                                       │                      │
│  │ Couleur : [🟤][🔴][🩷][🟠][🟡][🟢][🔵][🟣]│                      │
│  │ Forme :   [■][●][✿][⬡]              │                      │
│  │                                       │                      │
│  │ OU Image : [📷 Visuel JayXpose]       │                      │
│  │                                       │                      │
│  │ Taille :  [1x1 ▼]                    │                      │
│  │                                       │                      │
│  │ [Enregistrer] [Supprimer]             │                      │
│  └──────────────────────────────────────┘                      │
│                                                                 │
│  MODES DE PAIEMENT ACTIFS                                       │
│  [✅ Especes] [✅ Carte] [✅ Cheque] [☐ Virement] [☐ Autre]    │
│                                                                 │
│  PARAMETRES RECU                                                │
│  Logo : [Choisir un fichier]                                    │
│  Texte en-tete : [Merci de votre visite !        ]              │
│  Texte pied :    [A bientot chez [NomBoutique]   ]              │
│                                                                 │
│  [💾 Sauvegarder la configuration]                              │
└────────────────────────────────────────────────────────────────┘
```

### 3.4 JSH-A09 — Cloture de caisse

**Layout :** Page modale.

```
┌────────────────────────────────────────────────────────────────┐
│                   CLOTURE DE CAISSE                             │
│                                                                 │
│  Session : 11/02/2026 09:00 — 18:30                            │
│  Vendeur : Admin                                                │
│                                                                 │
│  ┌───────────────────────────────────────────────┐             │
│  │  SYNTHESE DES VENTES                           │             │
│  │                                                │             │
│  │  Nombre de tickets :              42           │             │
│  │  Nombre de remboursements :        2           │             │
│  │                                                │             │
│  │  Total brut des ventes :    1 250.00 €         │             │
│  │  Total remboursements :       -45.00 €         │             │
│  │  Chiffre d'affaires net :   1 205.00 €         │             │
│  │                                                │             │
│  │  VENTILATION PAR MODE DE PAIEMENT              │             │
│  │  Especes :                    620.00 €         │             │
│  │  Carte bancaire :             530.00 €         │             │
│  │  Cheque :                      55.00 €         │             │
│  └───────────────────────────────────────────────┘             │
│                                                                 │
│  ┌───────────────────────────────────────────────┐             │
│  │  CONTROLE ESPECES                              │             │
│  │                                                │             │
│  │  Fond de caisse initial :       100.00 €       │             │
│  │  Mouvements manuels :          +20.00 €        │             │
│  │  Especes encaissees :          620.00 €        │             │
│  │  Rendus monnaie :              -78.50 €        │             │
│  │  ─────────────────────────────────────         │             │
│  │  Especes attendues :           661.50 €        │             │
│  │                                                │             │
│  │  Especes comptees : [         661.50  ] €      │             │
│  │                                                │             │
│  │  Ecart :                         0.00 €        │             │
│  └───────────────────────────────────────────────┘             │
│                                                                 │
│  Note / commentaire : [                            ]            │
│                                                                 │
│  [✅ Valider la cloture]  [🖨️ Imprimer la synthese]            │
└────────────────────────────────────────────────────────────────┘
```

### 3.5 JSH-A18 — Gestion des taxes

**Layout :** Page de parametres (section Taxes). Inspire de Loyverse Settings > Taxes.

```
┌────────────────────────────────────────────────────────────────┐
│                      TAXES                                      │
│                                                                 │
│  [+ Ajouter un taux de taxe]                                    │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ ☐  Nom                  Appliquer aux    Taux           │  │
│  │                         nvx articles                     │  │
│  │ ☐  TVA                  Oui              20%            │  │
│  │ ☐  TVA reduite          Non              5.5%           │  │
│  │ ☐  TVA restauration     Non              10%            │  │
│  │ ☐  Exonere              Non              0%             │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ─── CREATION / EDITION ───                                     │
│  ┌──────────────────────────────────────┐                      │
│  │ Nom :        [TVA reduite         ]   │                      │
│  │ Taux :       [5.5               ] %   │                      │
│  │ Type :       [Incluse dans le prix ▼] │                      │
│  │              (Incluse / Ajoutee)       │                      │
│  │ Appliquer aux nouveaux articles : [☐] │                      │
│  │ Appliquer a : [Selectionner articles] │                      │
│  │                                       │                      │
│  │ [Annuler] [Enregistrer]               │                      │
│  └──────────────────────────────────────┘                      │
└────────────────────────────────────────────────────────────────┘
```

### 3.6 JSH-A16 — Gestion des remises

**Layout :** Page dediee. Inspire de Loyverse Items > Discounts.

```
┌────────────────────────────────────────────────────────────────┐
│                      REMISES                                    │
│                                                                 │
│  [+ Ajouter une remise]                                         │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ ☐  Nom                  Valeur          Acces restreint  │  │
│  │                                                           │  │
│  │ ☐  🏷️ -10% fidelite     10%             Non              │  │
│  │ ☐  🏷️ Lot 3 pour 2      33%             Non              │  │
│  │ ☐  🏷️ Remise salon      5.00 €          Non              │  │
│  │ ☐  🏷️ VIP               20%             Oui              │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                 │
│  ─── CREATION / EDITION ───                                     │
│  ┌──────────────────────────────────────┐                      │
│  │ Nom :        [-10% fidelite       ]   │                      │
│  │ Type :       (●) Pourcentage ( ) Montant│                    │
│  │ Valeur :     [10                ] %   │                      │
│  │ Acces restreint : [☐] (admin seul)    │                      │
│  │                                       │                      │
│  │ [Annuler] [Enregistrer]               │                      │
│  └──────────────────────────────────────┘                      │
└────────────────────────────────────────────────────────────────┘
```

### 3.7 JSH-A13 — Parametres (enrichi avec toggles fonctionnalites)

**Layout :** Page de parametres avec sidebar de sections. Inspire de Loyverse Settings.

```
┌────────────────────────────────────────────────────────────────┐
│                      PARAMETRES                                 │
│                                                                 │
│  ┌──────────────┐  ┌───────────────────────────────────────┐   │
│  │ Parametres    │  │  FONCTIONNALITES                      │   │
│  │               │  │                                       │   │
│  │ ● General     │  │  Sessions de caisse (shifts)    [ON ] │   │
│  │   Fonctionnal.│  │  Suivi des periodes de travail  ...   │   │
│  │   Modes paiem.│  │                                       │   │
│  │   Fidelite    │  │  Tickets ouverts                [ON ] │   │
│  │   Taxes       │  │  Sauvegarder/reprendre tickets  ...   │   │
│  │   Recu        │  │                                       │   │
│  │               │  │  Alertes stock bas              [OFF] │   │
│  │ Magasin       │  │  Email quand stock bas          ...   │   │
│  │   Boutique    │  │                                       │   │
│  │   PoS devices │  │  Alertes stock negatif          [ON ] │   │
│  │               │  │  Avertir si vente stock <= 0    ...   │   │
│  │               │  │                                       │   │
│  │               │  │  Fichier client                 [ON ] │   │
│  │               │  │  Association client-ticket      ...   │   │
│  │               │  │                                       │   │
│  │               │  │  Remises pre-definies           [ON ] │   │
│  │               │  │  Remises configurees en caisse  ...   │   │
│  │               │  │                                       │   │
│  │               │  │             [Annuler] [Enregistrer]   │   │
│  └──────────────┘  └───────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────┘
```

### 3.8 JSH-A20 — Liste des evenements

**Layout :** Liste des participations evenements/festivals avec indicateurs de performance.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                           EVENEMENTS                     [+ Nouvel evenement]  │
│                                                                                │
│  Filtres: [Tous ▼]  [2026 ▼]                                                   │
│                                                                                │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │ Statut │ Evenement                │ Dates           │ CA      │ Benefice │  │
│  ├────────┼──────────────────────────┼─────────────────┼─────────┼──────────┤  │
│  │ 🟢 En  │ Salon du Livre 2026      │ 15-17 mars 2026 │ 1 250 € │  +420 €  │  │
│  │  cours │ 📌 JayFestival           │ Paris Expo      │         │          │  │
│  ├────────┼──────────────────────────┼─────────────────┼─────────┼──────────┤  │
│  │ 🔵 Con │ Marche de Noel           │ 01-24 dec 2025  │ 3 800 € │ +1 200 € │  │
│  │  firme │                          │ Place centrale  │         │          │  │
│  ├────────┼──────────────────────────┼─────────────────┼─────────┼──────────┤  │
│  │ ⚫ Clo │ Foire de Printemps       │ 10-12 avr 2025  │ 2 100 € │  +650 €  │  │
│  │  ture  │ 📌 JayFestival           │ Parc des expos  │         │          │  │
│  └────────┴──────────────────────────┴─────────────────┴─────────┴──────────┘  │
│                                                                                │
│  📌 = Lie a JayFestival (synchronisation automatique)                          │
│                                                                                │
│  Legende : 🟢 En cours  🔵 Confirme  🟡 Brouillon  🔴 Annule  ⚫ Cloture        │
└────────────────────────────────────────────────────────────────────────────────┘
```

### 3.9 JSH-A21 — Formulaire evenement

**Layout :** Formulaire de creation/edition d'une fiche evenement.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│                           NOUVEL EVENEMENT                                     │
│                                                                                │
│  ┌────────────────────────────────────────────────────────────────────────┐   │
│  │  Nom de l'evenement *                                                  │   │
│  │  [Salon du Livre 2026                                            ]     │   │
│  │                                                                        │   │
│  │  Dates *                                                               │   │
│  │  [15/03/2026] au [17/03/2026]                                         │   │
│  │                                                                        │   │
│  │  Lieu                                                                  │   │
│  │  [Paris Expo - Porte de Versailles                               ]     │   │
│  │                                                                        │   │
│  │  Informations stand                                                    │   │
│  │  [Stand A42 - Zone Litterature jeunesse - 9m2                    ]     │   │
│  │                                                                        │   │
│  │  Notes internes                                                        │   │
│  │  ┌──────────────────────────────────────────────────────────────┐     │   │
│  │  │ Emporter les nouveautes 2026 et les bestsellers 2025.        │     │   │
│  │  │ Contact organisateur : Marie (06 12 34 56 78)                │     │   │
│  │  └──────────────────────────────────────────────────────────────┘     │   │
│  │                                                                        │   │
│  │  ┌────────────────────────────────────────────────────────────┐       │   │
│  │  │ 📌 Lie a JayFestival                                        │       │   │
│  │  │ Edition : Salon du Livre 2026                               │       │   │
│  │  │ Candidature : Validee le 11/02/2026                         │       │   │
│  │  │ [Voir dans JayFestival →]                                   │       │   │
│  │  └────────────────────────────────────────────────────────────┘       │   │
│  │                                                                        │   │
│  │                                    [Annuler] [Enregistrer]             │   │
│  └────────────────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────────────────┘
```

### 3.10 JSH-A22 — Couts de participation

**Layout :** Liste des couts par evenement avec totaux par categorie.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│  COUTS DE PARTICIPATION — Salon du Livre 2026              [+ Ajouter un cout] │
│                                                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Categorie  │ Libelle                      │ Date       │ Montant        │  │
│  ├────────────┼──────────────────────────────┼────────────┼────────────────┤  │
│  │ 🏪 Stand   │ Location stand 9m2           │ 01/02/2026 │       450.00 € │  │
│  │ 🏪 Stand   │ Electricite                  │ 01/02/2026 │        50.00 € │  │
│  ├────────────┼──────────────────────────────┼────────────┼────────────────┤  │
│  │ 🚗 Transport│ Billet train A/R Paris      │ 15/03/2026 │       120.00 € │  │
│  │ 🚗 Transport│ Location fourgon 3 jours    │ 14/03/2026 │       180.00 € │  │
│  ├────────────┼──────────────────────────────┼────────────┼────────────────┤  │
│  │ 🏨 Logement│ Hotel 3 nuits (14-16 mars)   │ 14/03/2026 │       285.00 € │  │
│  ├────────────┼──────────────────────────────┼────────────┼────────────────┤  │
│  │ 🍽️ Repas   │ Repas x3 jours x2 pers.      │ 15/03/2026 │       150.00 € │  │
│  ├────────────┼──────────────────────────────┼────────────┼────────────────┤  │
│  │ 📦 Autre   │ Badges exposant              │ 10/03/2026 │        25.00 € │  │
│  └────────────┴──────────────────────────────┴────────────┴────────────────┘  │
│                                                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │  TOTAL PAR CATEGORIE                                                    │  │
│  │  🏪 Stand : 500.00 €  │  🚗 Transport : 300.00 €  │  🏨 Logement: 285 € │  │
│  │  🍽️ Repas : 150.00 €  │  📦 Autre : 25.00 €                             │  │
│  │                                                                         │  │
│  │  TOTAL COUTS : 1 260.00 €                                               │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────────┘
```

### 3.11 JSH-A23 — Stock temporaire

**Layout :** Allocation de stock temporaire depuis le stock global pour l'evenement.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│  STOCK TEMPORAIRE — Salon du Livre 2026           [+ Ajouter des produits]     │
│                                                                                │
│  ⚠️ Le stock temporaire est deduit du stock global. A la cloture de            │
│     l'evenement, le stock non vendu sera reintegre automatiquement.            │
│                                                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ Produit                    │ Alloue │ Vendu │ Restant │ Stock global    │  │
│  ├────────────────────────────┼────────┼───────┼─────────┼─────────────────┤  │
│  │ 📚 Roman "L'Odyssee"       │   50   │  32   │   18    │ 150 (dont 50    │  │
│  │                            │        │       │   🟢    │ alloues ici)    │  │
│  ├────────────────────────────┼────────┼───────┼─────────┼─────────────────┤  │
│  │ 📚 Album "Le Petit Prince" │   30   │  28   │    2    │ 80 (dont 30     │  │
│  │                            │        │       │   🟠    │ alloues ici)    │  │
│  ├────────────────────────────┼────────┼───────┼─────────┼─────────────────┤  │
│  │ 📚 BD "Asterix XLII"       │   20   │  20   │    0    │ 45 (dont 20     │  │
│  │                            │        │       │   🔴    │ alloues ici)    │  │
│  ├────────────────────────────┼────────┼───────┼─────────┼─────────────────┤  │
│  │ 📚 Nouveautes 2026 (lot)   │   40   │  15   │   25    │ 100 (dont 40    │  │
│  │                            │        │       │   🟢    │ alloues ici)    │  │
│  └────────────────────────────┴────────┴───────┴─────────┴─────────────────┘  │
│                                                                                │
│  Legende : 🟢 Stock OK  🟠 Stock bas (< 5)  🔴 Rupture                          │
│                                                                                │
│  TOTAL : 140 produits alloues / 95 vendus / 45 restants                        │
│                                                                                │
│                                        [Modifier les quantites] [Enregistrer]  │
└────────────────────────────────────────────────────────────────────────────────┘
```

### 3.12 JSH-A24 — Dashboard evenement

**Layout :** Vue synthetique de la performance d'un evenement.

```
┌────────────────────────────────────────────────────────────────────────────────┐
│  DASHBOARD EVENEMENT — Salon du Livre 2026                    [Cloturer →]     │
│  📅 15-17 mars 2026 • 📍 Paris Expo • 🟢 En cours                               │
│                                                                                │
│  ┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐      │
│  │ CA REALISE  │ TOTAL COUTS │ COUT MARCH. │ BENEF. BRUT │ BENEF. NET  │      │
│  │  1 850 €    │  1 260 €    │   620 €     │   +590 €    │   -30 €     │      │
│  │ 42 tickets  │ 7 postes    │ 95 vendus   │ CA - Couts  │ - Cout march│      │
│  └─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘      │
│                                                                                │
│  ┌─────────────────────────────────────┬──────────────────────────────────┐   │
│  │  REPARTITION VENTES                 │  STOCK TEMPORAIRE                │   │
│  │  ┌──────────────────────────────┐   │  ┌────────────────────────────┐  │   │
│  │  │ Roman "L'Odyssee"     640 €  │   │  │ Alloue : 140               │  │   │
│  │  │ Album "Le Petit"      560 €  │   │  │ Vendu : 95                 │  │   │
│  │  │ BD "Asterix"          400 €  │   │  │ Restant : 45               │  │   │
│  │  │ Nouveautes 2026       250 €  │   │  │ ████████████████░░░░░ 68%  │  │   │
│  │  └──────────────────────────────┘   │  └────────────────────────────┘  │   │
│  └─────────────────────────────────────┴──────────────────────────────────┘   │
│                                                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │  DETAIL DES COUTS                                                       │  │
│  │  🏪 Stand : 500 €  │  🚗 Transport : 300 €  │  🏨 Logement : 285 €       │  │
│  │  🍽️ Repas : 150 €  │  📦 Autre : 25 €                                    │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│                                                                                │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │ 📌 Lie a JayFestival • Edition : Salon du Livre 2026                    │  │
│  │ [Voir Couts →] [Voir Stock →] [Voir Tickets →] [Voir dans JayFestival →]│  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Composants partages

| Composant | Utilise dans | Description |
|-----------|-------------|-------------|
| `ProductCard` | JSH-A02, JSH-C02, JSH-C03 | Carte produit (image, nom, prix, disponibilite). |
| `CartLine` | JSH-A05, JSH-C04 | Ligne de panier/ticket (produit, quantite, prix, sous-total). |
| `PaymentButton` | JSH-A06, JSH-C05 | Bouton de mode de paiement (icone, label). |
| `NumericPad` | JSH-A06 | Pave numerique tactile (0-9, point, effacer). |
| `MoneyDisplay` | JSH-A05, JSH-A06, JSH-A09 | Affichage d'un montant formate (devise, decimales). |
| `TicketRecap` | JSH-A06, JSH-A11, JSH-C06 | Recapitulatif d'un ticket (lignes, totaux, paiements). |
| `PosButton` | JSH-A05, JSH-A07 | Bouton produit configurable (nom, prix, couleur, taille). |
| `TabBar` | JSH-A05, JSH-A07 | Barre d'onglets configurables. |
| `StatusBadge` | JSH-A10, JSH-A12, JSH-C07 | Badge de statut (draft, paid, refunded, cancelled, preparing, shipped). |
| `DateRangePicker` | JSH-A01, JSH-A10, JSH-A19 | Selecteur de periode (jour, semaine, mois, personnalise). |
| `DiscountBadge` | JSH-A05, JSH-A16 | Badge de remise (nom, valeur). |
| `TaxToggle` | JSH-A03, JSH-A18 | Toggle d'association taxe ↔ article. |
| `ShapeColorPicker` | JSH-A07 | Selecteur couleur (palette 8) + forme (carre, rond, badge, hexagone). Inspire de Loyverse « Representation on POS ». |
| `FeatureToggle` | JSH-A13 | Toggle avec label et description pour activer/desactiver une fonctionnalite. |
| `KpiCard` | JSH-A01, JSH-A19, JSH-A24 | Carte KPI (valeur, label, variation %). Inspire de Loyverse Sales Summary. |
| `CustomerCard` | JSH-A17 | Carte client (avatar, nom, email, telephone). |
| `EventCard` | JSH-A20 | Carte evenement (nom, dates, lieu, statut, CA, benefice). |
| `EventStatusBadge` | JSH-A20, JSH-A21, JSH-A24 | Badge de statut evenement (draft, confirmed, ongoing, closed, cancelled). |
| `CostCategoryBadge` | JSH-A22 | Badge de categorie de cout (stand, transport, lodging, food, other). |
| `StockAllocationRow` | JSH-A23 | Ligne d'allocation stock (produit, alloue, vendu, restant, indicateur). |
| `JayFestivalLink` | JSH-A21, JSH-A24 | Encart de liaison JayFestival (edition, candidature, lien externe). |
| `ProfitDisplay` | JSH-A20, JSH-A24 | Affichage benefice avec coloration (+vert, -rouge). |

---

## 5. Specifications d'interaction

### 5.1 Rendu monnaie — Logique temps reel

```
A chaque modification du champ « montant donne » :
  Si montant_donne >= reste_a_payer :
    Afficher « Rendu monnaie : {montant_donne - reste_a_payer} € »
    Colorer en vert
  Sinon :
    Afficher « Reste a payer : {reste_a_payer - montant_donne} € »
    Colorer en orange
```

### 5.2 Bouton « Valider le paiement » — Conditions d'activation

```
Actif si et seulement si :
  somme(paiements_partiels.montant) >= total_ttc
  OU
  (montant_donne >= reste_a_payer ET un mode de paiement est selectionne)
```

### 5.3 Ajout produit au ticket (PoS) — Feedback

```
Au clic sur un bouton produit :
  Si le produit est deja dans le ticket → incrementer la quantite
  Sinon → ajouter une nouvelle ligne
  Animation : flash bref sur la ligne modifiee
  Son : bip discret (configurable)
  Mise a jour instantanee des totaux
```

---

## 6. Responsive et supports

| Support | Ecrans concernes | Adaptation |
|---------|------------------|------------|
| **Desktop** | Tous | Layout complet, split horizontal pour le PoS. |
| **Tablette** | JSH-A05, JSH-A06 | Layout PoS optimise tactile. Boutons plus grands. Pave numerique integre. |
| **Mobile** | JSH-C01 a JSH-C08 | Boutique en ligne adaptee mobile. PoS non recommande sur mobile (ecran trop petit). |

---

## 7. References

- [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md)
- [JayShop - Parcours Utilisateur](./JayShop%20-%20Parcours%20Utilisateur.md)
- [JayShop - Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md)
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)

---

**Document** : JayShop — Ecrans et UI
**Version** : 1.2
**Date** : 2026-02-11
**Statut** : Reference produit — enrichi avec ecrans gestion evenements/festivals (JSH-A20 a JSH-A24)
