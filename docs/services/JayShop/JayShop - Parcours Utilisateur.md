# JayShop — Parcours Utilisateur

## Contexte

Ce document decrit les **parcours utilisateur** du service **JayShop** pour les deux profils : **Admin** (vendeur/gerant) et **Client** (acheteur). Les parcours couvrent la boutique en ligne et le point de vente (PoS).

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), [Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md).

## Portee / Scope

- **Perimetre** : Parcours fonctionnels detailles pour chaque profil utilisateur, de l'onboarding a la vente.
- **Hors perimetre** : Maquettes UI detaillees (voir [Ecrans et UI](./JayShop%20-%20Ecrans%20et%20UI.md)), specifications techniques.

---

## 1. Profils utilisateur

| Profil | Description | Acces |
|--------|-------------|-------|
| **Admin** | Vendeur, gerant, exploitant. Proprietaire de la boutique et du PoS. | Central (PoS + gestion) + Portail (boutique admin). |
| **Client** | Acheteur, visiteur. Parcourt la boutique, passe commande, paie. | Portail (boutique en ligne). |

---

## 2. Parcours Admin

### 2.1 Onboarding

```
[Miyukini Central] → Activer JayShop
    → Si profil JayXpose existe : liaison automatique du catalogue
    → Sinon : creation guidee (nom, secteur, premiers produits)
    → Configuration boutique :
        → Nom de la boutique, slug URL
        → Devise, taux de TVA
        → Modes de paiement acceptes
        → CGV et mentions legales
    → Configuration PoS (optionnel) :
        → Creer les onglets (categories rapides)
        → Affecter les boutons produits aux onglets
        → Personnaliser couleurs et taille des boutons
    → Boutique prete / PoS pret
```

### 2.2 Gestion des produits

```
[JayShop > Produits]
    → Voir la liste des produits (depuis JayXpose)
    → Ajouter un produit :
        → Nom, description, prix, categorie, visuel(s)
        → Disponibilite (disponible / rupture / sur commande)
        → Optionnel : marquer comme « Produit PoS » → affecter a un onglet
        → Enregistrer (relaye a JayXpose)
    → Modifier un produit :
        → Clic sur le produit → formulaire pre-rempli
        → Modifier les champs → Enregistrer
    → Supprimer un produit :
        → Confirmation → Soft delete (relaye a JayXpose)
    → Gerer les categories :
        → Creer / Renommer / Supprimer / Reordonner
```

### 2.3 Session PoS — Vente en caisse

```
[JayShop > PoS] → Ouverture de caisse
    → Saisie du fond de caisse initial
    → Ecran principal PoS :
        ┌───────────────────────┬──────────────────────────┐
        │   TICKET EN COURS     │   GRILLE BOUTONS         │
        │                       │   [Onglet 1] [Onglet 2]  │
        │  Produit A    x2  10€│   [Prod1] [Prod2] [Prod3] │
        │  Produit B    x1   5€│   [Prod4] [Prod5] [Prod6] │
        │                       │   [Prod7] [Prod8] [Prod9] │
        │  Sous-total:    15.00€│                            │
        │  TVA:            3.00€│   [🔍 Recherche produit]   │
        │  TOTAL:         18.00€│                            │
        │                       │                            │
        │  [Remise] [Paiement]  │                            │
        └───────────────────────┴──────────────────────────┘
    → Clic bouton produit → +1 unite au ticket
    → Modifier quantite → clic sur la ligne → saisie quantite
    → Supprimer ligne → swipe ou bouton X
    → Appliquer remise → % ou montant fixe sur une ligne ou sur le total
    → Sauvegarder le ticket → reprendre plus tard
    → Bouton « Paiement » → Ecran de paiement (voir 2.4)
```

### 2.4 Ecran de paiement (PoS)

```
[Ecran de paiement]
    ┌──────────────────────────────────────────────┐
    │              RECAP DU TICKET                  │
    │                                               │
    │  Produit A         x2        10.00 €          │
    │  Produit B         x1         5.00 €          │
    │  Remise                      -1.50 €          │
    │  ─────────────────────────────────────        │
    │  Sous-total HT              13.50 €           │
    │  TVA (20%)                   2.70 €           │
    │  ═════════════════════════════════════        │
    │  TOTAL A PAYER              16.20 €           │
    │                                               │
    │  Reste a payer :            16.20 €           │
    │                                               │
    ├──────────────────────────────────────────────┤
    │  MONTANT DONNE PAR LE CLIENT :               │
    │  ┌────────────────────────────────┐           │
    │  │           20.00               │ €          │
    │  └────────────────────────────────┘           │
    │  [5€] [10€] [20€] [50€] [Exact]              │
    │                                               │
    │  Rendu monnaie :             3.80 €           │
    │                                               │
    ├──────────────────────────────────────────────┤
    │  MODE DE PAIEMENT :                           │
    │  [💵 Especes] [💳 Carte] [📄 Cheque]          │
    │  [🏦 Virement] [Autre]                        │
    │                                               │
    │           [✅ VALIDER LE PAIEMENT]             │
    └──────────────────────────────────────────────┘
```

**Flux detaille :**

```
1. Le ticket est affiche en recap (lignes, sous-total, taxes, total)
2. Le « Reste a payer » est initialise au total TTC
3. L'admin saisit le montant donne par le client
   → Si montant >= reste a payer : affichage « Rendu monnaie : X €»
   → Si montant < reste a payer : affichage « Reste a payer : X € »
4. L'admin clique sur le mode de paiement :
   a. Especes : le montant saisi est enregistre, rendu monnaie calcule
   b. Carte / Cheque / Virement : le montant saisi (ou le reste a payer) est enregistre
5. Si paiement mixte :
   → Apres le premier paiement partiel, le « Reste a payer » est recalcule
   → L'admin peut saisir un second montant et choisir un autre mode
   → Repeter jusqu'a « Reste a payer = 0 »
6. Bouton « Valider le paiement » (actif quand reste a payer <= 0)
   → Cloture du ticket
   → Decrementation du stock
   → Transmission a JayKonta
   → Generation du recu
   → Retour a l'ecran principal PoS
```

### 2.5 Cloture de caisse

```
[JayShop > PoS] → Cloturer la caisse
    → Saisie du montant d'especes comptees dans le tiroir-caisse
    → Synthese de la session :
        ┌──────────────────────────────────────┐
        │       CLOTURE DE CAISSE              │
        │                                       │
        │  Ouverture :     09:00                │
        │  Cloture :       18:30                │
        │  Nombre de tickets :     42           │
        │                                       │
        │  Total ventes :       1 250.00 €      │
        │  Total remboursements :  -45.00 €     │
        │  Net :                1 205.00 €      │
        │                                       │
        │  Ventilation :                         │
        │    Especes :           620.00 €       │
        │    Carte bancaire :    530.00 €       │
        │    Cheque :             55.00 €       │
        │                                       │
        │  Fond de caisse initial : 100.00 €    │
        │  Especes attendues :      720.00 €    │
        │  Especes comptees :       718.50 €    │
        │  Ecart :                   -1.50 €    │
        │                                       │
        │  [Valider la cloture] [Imprimer]      │
        └──────────────────────────────────────┘
    → Valider → Session archivee
    → Transmission synthese a JayKonta
```

### 2.6 Historique des ventes

```
[JayShop > Historique]
    → Liste des tickets (chronologique, plus recents en premier)
    → Filtres : date, statut, source (PoS/Online), mode de paiement
    → Clic sur un ticket → Detail complet :
        → Lignes, quantites, prix, remises
        → Mode(s) de paiement, rendu monnaie
        → Horodatage, numero de ticket
    → Actions sur un ticket :
        → Reimprimer le recu
        → Rembourser (total ou partiel)
    → Export CSV / PDF
```

### 2.7 Tableau de bord

```
[JayShop > Tableau de bord]
    → Periode : Aujourd'hui / Semaine / Mois / Personnalise
    → Chiffre d'affaires
    → Nombre de tickets
    → Panier moyen
    → Top 10 produits vendus
    → Ventilation par mode de paiement (camembert)
    → Evolution CA (graphique en ligne)
```

---

## 3. Parcours Client (Boutique en ligne)

### 3.1 Decouverte

```
[Portail > Boutique JayShop du vendeur]
    → Page d'accueil boutique :
        → Banniere, nom de la boutique, description
        → Produits mis en avant (vedettes)
        → Lien vers le catalogue complet
```

### 3.2 Navigation catalogue

```
[Boutique > Catalogue]
    → Liste des produits (grille ou liste)
    → Filtres : categorie, prix (min-max), disponibilite
    → Recherche par mot-cle
    → Tri : prix croissant/decroissant, nom, nouveaute
    → Clic produit → Fiche produit :
        → Galerie visuels
        → Description
        → Prix et disponibilite
        → Bouton « Ajouter au panier »
        → Quantite souhaitee
```

### 3.3 Panier

```
[Boutique > Panier]
    → Liste des produits ajoutes
    → Modifier quantite (+ / -)
    → Supprimer un produit
    → Recap :
        → Sous-total
        → Taxes estimees
        → Total
    → Bouton « Commander »
```

### 3.4 Commande et paiement

```
[Boutique > Commande]
    → Informations client :
        → Nom, prenom
        → Email
        → Telephone (optionnel)
        → Adresse de livraison (si applicable)
    → Recap de la commande (lignes, total)
    → Selection du mode de paiement :
        → Carte bancaire (paiement en ligne)
        → Virement (instructions)
        → Paiement en magasin (si mode PoS active)
    → Validation → Confirmation :
        → Numero de ticket
        → Recapitulatif
        → Envoi du recu par email
        → Statut : « En attente de preparation »
```

### 3.5 Suivi de commande

```
[Boutique > Mes commandes] (si authentifie)
    → Liste des commandes passees
    → Statut : En attente / En preparation / Expediee / Terminee
    → Detail d'une commande :
        → Lignes, montants, mode de paiement
        → Historique des changements de statut
```

---

## 4. Matrice des capacites par profil

| Capacite | Admin | Client |
|----------|-------|--------|
| Activer / configurer JayShop | ✅ | ❌ |
| Gerer les produits (CRUD) | ✅ | ❌ |
| Configurer PoS (onglets, boutons) | ✅ | ❌ |
| Utiliser le PoS (caisse) | ✅ | ❌ |
| Ouvrir / cloturer la caisse | ✅ | ❌ |
| Voir l'historique des ventes | ✅ | ❌ |
| Rembourser un ticket | ✅ | ❌ |
| Voir le tableau de bord | ✅ | ❌ |
| Parcourir la boutique en ligne | ✅ | ✅ |
| Ajouter au panier | ✅ | ✅ |
| Passer commande | ❌ | ✅ |
| Choisir le mode de paiement | ✅ (PoS) | ✅ (en ligne) |
| Consulter ses commandes | ❌ | ✅ |
| Exporter l'historique | ✅ | ❌ |

---

## 5. References

- [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md)
- [JayShop - Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md)
- [JayShop - Ecrans et UI](./JayShop%20-%20Ecrans%20et%20UI.md)

---

**Document** : JayShop — Parcours Utilisateur
**Version** : 1.0
**Date** : 2026-02-11
**Statut** : Reference produit
