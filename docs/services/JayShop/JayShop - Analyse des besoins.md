# JayShop — Analyse des besoins

## Contexte

Ce document recense les **besoins fonctionnels et non-fonctionnels** du service **JayShop**. Il sert de reference pour la conception des Operateurs, Kits d'outils et ecrans du service.

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), Glossaire Miyukini.

## Portee / Scope

- **Perimetre** : Besoins fonctionnels (JSH-01 a JSH-xx) et non-fonctionnels (NFR-JSH-xx) couvrant l'onboarding, la boutique en ligne, le PoS, le paiement, l'historique et les integrations.
- **Hors perimetre** : Implementation, API, schemas de base de donnees (references dans d'autres documents).

---

## 1. Besoins fonctionnels

### 1.1 Onboarding et configuration

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-01 | Activation du service | P0 | L'admin active JayShop depuis Miyukini Central. Liaison automatique avec le profil JayXpose existant. |
| JSH-02 | Configuration boutique | P0 | Nom, slug URL, devise, CGV, mentions legales. |
| JSH-03 | Configuration modes de paiement | P0 | L'admin definit les modes de paiement acceptes (especes, CB, cheque, virement, autre) et leur ordre. |
| JSH-04 | Configuration fiscale | P0 | Taux de TVA par defaut, possibilite de definir des taux par categorie de produit. |
| JSH-05 | Configuration PoS | P1 | Definition des onglets et boutons produits pour l'interface de caisse. |
| JSH-06 | Fonctionnalites activables (toggles) | P1 | Ecran de parametres avec toggles pour activer/desactiver les fonctionnalites avancees (sessions de caisse, tickets ouverts, alertes stock, fichier client, remises). Inspire de Loyverse Settings > Features. |

### 1.2 Gestion des produits (CRUD via JayXpose)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-10 | Lire le catalogue | P0 | JayShop affiche les produits, categories, visuels et prix depuis JayXpose. |
| JSH-11 | Creer un produit | P1 | L'admin cree un produit depuis JayShop (nom, prix, categorie, visuel). Ecriture relayee a JayXpose. |
| JSH-12 | Modifier un produit | P1 | L'admin modifie un produit (prix, description, disponibilite, categorie). Ecriture relayee a JayXpose. |
| JSH-13 | Supprimer un produit | P1 | L'admin archive un produit. Soft delete relaye a JayXpose. |
| JSH-14 | Gerer les categories | P1 | L'admin cree, renomme, supprime, reordonne les categories depuis JayShop. |
| JSH-15 | Synchroniser les stocks | P0 | Decrementation du stock a chaque vente. Synchronisation bidirectionnelle avec JayXpose. |
| JSH-16 | Marquer produits PoS | P1 | L'admin assigne des produits a des onglets et boutons PoS. |
| JSH-17 | Import CSV d'articles | P1 | Import en masse de produits via fichier CSV (nom, prix, categorie, SKU, barcode). Complement de l'export existant. Inspire de Loyverse Item list > Import. |
| JSH-18 | Variantes de produit | P2 | Un article peut avoir des variantes (taille, couleur, option) avec prix et stock propres. Chaque combinaison d'options genere une variante. Extension du modele JayXpose. |
| JSH-19 | Filtrage par alerte de stock | P1 | Filtre dans la liste des produits : stock bas, epuise, tous. Inspire de Loyverse Item list > Stock alert. |

### 1.3 Boutique en ligne

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-20 | Catalogue public | P0 | Page catalogue avec liste de produits, filtres par categorie, recherche, tri par prix/nom. |
| JSH-21 | Fiche produit | P0 | Page de detail : visuels, description, prix, disponibilite, bouton « Ajouter au panier ». |
| JSH-22 | Panier | P0 | Ajout, modification quantite, suppression. Affichage sous-total, taxes, total. |
| JSH-23 | Commande | P0 | Saisie des informations client (nom, email, adresse optionnelle), selection mode de paiement. |
| JSH-24 | Confirmation commande | P0 | Recapitulatif, numero de ticket, envoi du recu par email. |
| JSH-25 | Suivi commande (admin) | P1 | L'admin voit les commandes entrantes et peut les marquer comme preparees/expediees/terminees. |
| JSH-26 | Suivi commande (client) | P2 | Le client peut consulter le statut de sa commande via un lien ou en etant authentifie. |

### 1.4 Point de vente (PoS)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-30 | Ecran principal PoS | P0 | Zone ticket (gauche) + grille boutons produits par onglets (droite). |
| JSH-31 | Onglets configurables | P0 | L'admin cree/modifie/supprime/reordonne les onglets. Chaque onglet regroupe des boutons produits. |
| JSH-32 | Boutons produits | P0 | Bouton = produit. Affiche nom, prix optionnel, couleur/icone. Clic = +1 au ticket. |
| JSH-33 | Modification ligne ticket | P0 | Modifier quantite, supprimer ligne, appliquer remise (% ou montant). |
| JSH-34 | Recherche produit | P1 | Barre de recherche dans la grille PoS pour trouver un produit non affiche dans les boutons. |
| JSH-35 | Scan code-barres | P2 | Ajout de produit par scan code-barres (lecteur USB/Bluetooth). |
| JSH-36 | Tickets ouverts | P1 | Sauvegarder un ticket en cours, ouvrir un nouveau, reprendre un ticket sauvegarde. Fonctionnalite activable par toggle. |
| JSH-37 | Representation bouton PoS | P1 | Chaque bouton peut etre personnalise : couleur + forme (carre, rond, badge, hexagone) OU image du produit (visuel JayXpose). Inspire de Loyverse « Representation on POS ». |
| JSH-38 | Couleur par categorie | P1 | Chaque categorie a une couleur associee (palette de 8 couleurs). Utilisee par defaut pour colorer les onglets et boutons PoS. |

### 1.5 Ecran de paiement

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-40 | Acces paiement | P0 | Bouton « Paiement » sur le ticket en cours. Ouvre l'ecran de paiement. |
| JSH-41 | Recap paiement | P0 | Affichage : liste des lignes, sous-total, taxes, total a payer. |
| JSH-42 | Selection mode de paiement | P0 | Boutons pour chaque mode configure (Especes, CB, Cheque, Virement, Autre). |
| JSH-43 | Saisie montant donne | P0 | Champ de saisie du montant remis par le client (mode especes). Pave numerique integre. |
| JSH-44 | Calcul rendu monnaie | P0 | Affichage dynamique : rendu monnaie = montant donne - total. Si montant donne < total : reste a payer affiche. |
| JSH-45 | Paiement mixte | P1 | Le client peut fractionner le paiement sur plusieurs modes. Reste a payer mis a jour apres chaque paiement partiel. |
| JSH-46 | Validation paiement | P0 | Lorsque le total est atteint ou depasse : validation, cloture du ticket, decrementation du stock, generation du recu. |
| JSH-47 | Boutons montant rapide | P2 | Boutons pre-definis pour les montants courants (5 EUR, 10 EUR, 20 EUR, 50 EUR) en mode especes. |
| JSH-48 | Remises pre-definies en caisse | P1 | En PoS, l'admin peut appliquer une remise pre-configuree (entite remise) en un clic sur une ligne ou sur le total. |

### 1.6 Historique et suivi

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-50 | Liste des tickets | P0 | Vue chronologique avec filtres (date, statut, source PoS/online, mode de paiement). |
| JSH-51 | Detail ticket | P0 | Lignes, quantites, prix, remises, taxes, total, mode(s) de paiement, rendu monnaie, horodatage. |
| JSH-52 | Remboursement | P1 | Remboursement total ou partiel. Generation d'un ticket de remboursement lie. Reajustement stock. |
| JSH-53 | Export | P1 | Export CSV ou PDF de l'historique des ventes. |
| JSH-54 | Tableau de bord | P1 | CA jour/semaine/mois, nombre de tickets, panier moyen, top produits, ventilation par mode de paiement. KPIs en bande : Ventes brutes, Remboursements, Remises, Ventes nettes, Profit brut. |
| JSH-55 | Rapport ventes par produit | P1 | Sous-rapport : CA ventile par produit sur une periode. Inspire de Loyverse « Sales by item ». |
| JSH-56 | Rapport ventes par categorie | P1 | Sous-rapport : CA ventile par categorie. Inspire de Loyverse « Sales by category ». |
| JSH-57 | Rapport ventes par mode de paiement | P1 | Sous-rapport : CA ventile par mode de paiement (especes, CB, cheque, etc.). |
| JSH-58 | Rapport taxes collectees | P2 | Sous-rapport : detail des taxes collectees par taux et par periode. |
| JSH-59 | Rapport remises appliquees | P2 | Sous-rapport : detail des remises appliquees (montant, frequence, par remise). |

### 1.7 Gestion de caisse (PoS)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-60 | Ouverture de caisse | P0 | Saisie du fond de caisse initial. |
| JSH-61 | Mouvements manuels | P1 | Entrees/sorties manuelles d'especes avec motif. |
| JSH-62 | Cloture de caisse | P0 | Synthese : total ventes, ventilation par mode, especes attendues vs comptees, ecart. |
| JSH-63 | Periodes de travail | P2 | Suivi des sessions par utilisateur (debut, fin, totaux). |

### 1.8 Integration JayXpose

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-70 | Liaison catalogue | P0 | JayShop lit le catalogue JayXpose (produits, categories, visuels, prix, disponibilite). |
| JSH-71 | Ecriture relayee | P1 | CRUD produits/categories depuis JayShop relaye a JayXpose. |
| JSH-72 | Sync stocks bidirectionnelle | P0 | Push (vente → decrementation) et Pull (modification stock JayXpose → mise a jour JayShop). |
| JSH-73 | Resolution conflits stocks | P1 | Politique configurable : `prefer_pos`, `prefer_local`, `manual_review`. |
| JSH-74 | Ajustement de stock simplifie | P1 | L'admin peut ajuster manuellement le stock depuis JayShop (reception, perte, casse). Relaye a JayXpose. Inspire de Loyverse « Stock adjustments ». |
| JSH-75 | Alertes de stock bas | P1 | Notification quand le stock d'un produit passe sous un seuil configurable. Email et/ou badge dans l'interface. Inspire de Loyverse « Low stock notifications ». |
| JSH-76 | Alerte stock negatif | P1 | Avertissement en caisse si le vendeur tente de vendre un produit dont le stock est a 0 ou negatif. Configurable (bloquer ou avertir). Inspire de Loyverse « Negative stock alerts ». |

### 1.9 Integration JayKonta

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-80 | Transmission ventes | P0 | A chaque cloture de ticket, envoi du resume comptable a JayKonta (montant HT, TVA, TTC, mode, date). |
| JSH-81 | Transmission remboursements | P1 | Les remboursements sont transmis comme ecritures inverses. |
| JSH-82 | Cloture de caisse | P1 | Syntheses de cloture transmises a JayKonta. |
| JSH-83 | Rapprochement | P2 | JayKonta peut rapprocher les ventes avec les encaissements bancaires. |

### 1.10 Remises pre-definies (inspire de Loyverse Discounts)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-90 | Creer une remise | P1 | L'admin cree une remise pre-definie : nom (ex. « -10% fidelite »), valeur (montant fixe ou %), type (montant ou pourcentage). |
| JSH-91 | Liste des remises | P1 | Vue liste des remises configurees avec nom, valeur, type, acces restreint (oui/non). |
| JSH-92 | Appliquer une remise en caisse | P1 | En PoS, l'admin selectionne une remise pre-definie et l'applique sur une ligne ou sur le total du ticket. |
| JSH-93 | Acces restreint aux remises | P2 | En contexte multi-vendeurs, certaines remises sont reservees a l'admin. |

### 1.11 Taxes et fiscalite enrichies (inspire de Loyverse Taxes)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-94 | Multi-taux de taxe | P0 | L'admin configure plusieurs taux de taxe (ex. TVA 20%, TVA reduite 5.5%, TVA resto 10%, exonere 0%). |
| JSH-95 | Taxe incluse ou ajoutee | P0 | Choix par taux : « Incluse dans le prix » (TTC) ou « Ajoutee au prix » (HT + taxe). |
| JSH-96 | Association taxe ↔ article | P1 | Chaque article peut avoir un ou plusieurs taux de taxe associes (toggles). |
| JSH-97 | Application par defaut | P1 | Option « Apply to new items » : un taux de taxe peut etre automatiquement applique aux nouveaux articles. |

### 1.12 Fichier client (inspire de Loyverse Customers)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-98 | Creer un client | P1 | L'admin cree une fiche client : nom, email, telephone, adresse. S'appuie sur MiyuContacts. |
| JSH-99 | Associer client ↔ ticket | P1 | En PoS ou en ligne, un ticket peut etre associe a un client (pour historique, envoi recu par email). |
| JSH-100 | Historique d'achats client | P2 | Consulter l'historique des achats d'un client (tickets, montants, produits). |
| JSH-101 | Programme de fidelite | P2 | Pourcentage du montant d'achat credite en points sur le compte client. Inspire de Loyverse Loyalty. |

### 1.13 Gestion des evenements et festivals

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-110 | Creer une fiche evenement | P1 | L'admin cree manuellement une fiche evenement : nom, dates (debut/fin), lieu, informations stand, notes. |
| JSH-111 | Liste des evenements | P1 | Vue liste des evenements avec statut (brouillon, confirme, en cours, cloture, annule), dates, CA realise. |
| JSH-112 | Saisir les couts de participation | P1 | L'admin saisit les couts lies a l'evenement par categorie : **stand**, **transport**, **logement**, **nourriture**, **autres**. Chaque cout a un libelle, un montant, une date et un justificatif optionnel. |
| JSH-113 | Liste des couts par evenement | P1 | Vue liste des couts saisis avec total par categorie et total general. |
| JSH-114 | Modifier/supprimer un cout | P1 | L'admin peut modifier ou supprimer un cout saisi. |
| JSH-115 | Allouer un stock temporaire | P1 | L'admin selectionne des produits et definit les quantites a emporter sur l'evenement. Ce stock temporaire est **deduit du stock global** (JayXpose) et reserve pour l'evenement. |
| JSH-116 | Vue stock temporaire par evenement | P1 | Liste des produits alloues avec : quantite allouee, quantite vendue, quantite restante. Indicateurs visuels (rupture, stock bas). |
| JSH-117 | Associer ticket ↔ evenement | P1 | En PoS, l'admin peut **selectionner l'evenement actif**. Tous les tickets crees decrementent le stock temporaire de cet evenement (et non le stock global). |
| JSH-118 | Dashboard evenement | P1 | Tableau de bord par evenement : CA realise, total des couts, stock restant, **benefice brut** (CA - couts), **benefice net** (CA - couts - cout des marchandises vendues). |
| JSH-119 | Cloture d'evenement | P1 | Cloture de l'evenement : synthese finale, calcul du benefice. Le stock non vendu est **reintegre au stock global** (JayXpose). |
| JSH-120 | Transmission comptable par evenement | P1 | A la cloture, transmission a JayKonta avec ventilation par evenement : CA, couts, benefice. |

### 1.14 Integration JayFestival (evenements)

| Id | Besoin | Priorite | Description |
|----|--------|----------|-------------|
| JSH-130 | Creation automatique de fiche evenement | P1 | Lorsque l'organisateur JayFestival **valide la candidature** d'un exposant, une fiche evenement est **automatiquement creee** dans JayShop avec les informations de l'edition (nom, dates, lieu, stand attribue). |
| JSH-131 | Synchronisation des informations | P1 | Les mises a jour cote JayFestival (changement de dates, de stand, annulation de participation) sont **synchronisees** vers JayShop. La fiche evenement est mise a jour automatiquement. |
| JSH-132 | Notification de validation | P1 | L'admin JayShop recoit une notification lorsqu'une candidature est validee et qu'une fiche evenement est creee. |
| JSH-133 | Lien vers JayFestival | P2 | Depuis la fiche evenement JayShop, l'admin peut acceder a son dashboard exposant JayFestival (documents, factures organisateur, planning). |
| JSH-134 | Annulation depuis JayFestival | P1 | Si la participation est annulee cote JayFestival, la fiche evenement JayShop passe en statut « annule » et le stock temporaire est reintegre automatiquement. |

---

## 2. Besoins non-fonctionnels

| Id | Besoin | Description |
|----|--------|-------------|
| NFR-JSH-01 | Performance PoS | L'ajout d'un produit au ticket doit etre instantane (< 100ms). Le calcul du rendu monnaie doit etre instantane. |
| NFR-JSH-02 | Fonctionnement hors-ligne | Le mode PoS doit fonctionner sans connexion internet. Synchronisation a la reconnexion. (LOI-1, LOI-2) |
| NFR-JSH-03 | Securite paiements | Les donnees de paiement sont classees niveau 3 (Critical). Chiffrement en transit et au repos. |
| NFR-JSH-04 | Scalabilite | Support de 500 produits par vendeur (alignement JayXpose). Support de 10 000 tickets par an par vendeur. |
| NFR-JSH-05 | Accessibilite | Conformite WCAG 2.1 AA pour la boutique en ligne. Contraste suffisant pour le mode PoS. |
| NFR-JSH-06 | Responsive | La boutique en ligne est adaptee mobile/tablette/desktop. Le PoS est optimise tablette et desktop. |
| NFR-JSH-07 | Impression | Support des imprimantes thermiques (ESC/POS) via connexion Ethernet ou Bluetooth pour les recus. |
| NFR-JSH-08 | Coherence donnees | Toute vente doit decrementer le stock de facon atomique. Pas de survente si stock = 0. |
| NFR-JSH-09 | Audit | Toute creation, modification ou suppression de ticket est tracee (horodatage, utilisateur, action). |
| NFR-JSH-10 | Souverainete des donnees | Les donnees de vente resident dans le COG local. L'etat local est souverain (LOI-3). |

---

## 3. Matrice de priorites

| Priorite | Description | Besoins |
|----------|-------------|---------|
| **P0** | Indispensable au MVP | JSH-01 a JSH-04, JSH-10, JSH-15, JSH-20 a JSH-24, JSH-30 a JSH-33, JSH-40 a JSH-44, JSH-46, JSH-50, JSH-51, JSH-60, JSH-62, JSH-70, JSH-72, JSH-80, JSH-94, JSH-95 |
| **P1** | Important, post-MVP proche | JSH-05, JSH-06, JSH-11 a JSH-14, JSH-16 a JSH-17, JSH-19, JSH-25, JSH-34, JSH-36 a JSH-38, JSH-45, JSH-48, JSH-52 a JSH-57, JSH-61, JSH-71, JSH-73 a JSH-76, JSH-81, JSH-82, JSH-90 a JSH-92, JSH-96 a JSH-99, **JSH-110 a JSH-120** (evenements), **JSH-130 a JSH-132, JSH-134** (integration JayFestival) |
| **P2** | Phase 2 | JSH-18, JSH-26, JSH-35, JSH-47, JSH-58, JSH-59, JSH-63, JSH-83, JSH-93, JSH-100, JSH-101, **JSH-133** (lien JayFestival) |

---

## 4. References

- [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md)
- [JayXpose - Catalogue Produits](../JayXpose/JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Interfaces Inter-Services](../JayXpose/JayXpose%20-%20Interfaces%20Inter-Services.md)
- [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md)
- [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md)

---

**Document** : JayShop — Analyse des besoins
**Version** : 1.2
**Date** : 2026-02-11
**Statut** : Reference produit — enrichi avec gestion des evenements/festivals et integration JayFestival
