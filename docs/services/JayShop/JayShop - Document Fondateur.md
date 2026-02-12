# JayShop — Document fondateur

## Contexte

**JayShop** est le **service Miyukini dedie au commerce et a la vente** au sein de l'ecosysteme COG. Il couvre l'ensemble du cycle de vente : **onboarding vendeur**, **boutique en ligne**, **point de vente (PoS)** en caisse et **suivi des ventes**. JayShop s'appuie sur le **catalogue produits et les stocks de JayXpose** et transmet les donnees comptables a **JayKonta** pour le suivi financier.

**Un seul service COG**, avec **deux modes d'interaction** :

| Mode | Description |
|------|-------------|
| **Boutique en ligne** | Vitrine commerciale avec panier, commande et paiement en ligne. Accessible au client (visiteur ou authentifie) et a l'admin. |
| **Point de vente (PoS)** | Interface de caisse optimisee pour la saisie rapide de tickets en situation physique (marche, boutique, evenement). Reserve a l'admin. |

Les deux modes partagent les **memes Operateurs et Kits** du service COG ; seuls le perimetre fonctionnel, les interfaces et les Mandats different.

Ce document est le **document fondateur** du service : il en fixe la raison d'etre, la portee, les principes directeurs, les fonctionnalites structurantes et l'integration avec les autres services Jay. Il s'adresse aux equipes produit, technique et aux parties prenantes.

## Portee / Scope

- **Perimetre** : Definition du service JayShop — positionnement, fonctionnalites (onboarding, boutique en ligne, PoS, tickets, paiements, historique ventes), integration avec JayXpose et JayKonta, niveaux de securite.
- **Hors perimetre** : Specifications techniques detaillees (API, schemas), implementation des crates, logique de facturation et comptabilite (JayKonta), gestion du catalogue produits et du profil exposant (JayXpose).
- **References** : Glossaire Miyukini, [Document fondateur JayXpose](../JayXpose/JayXpose%20-%20Document%20Fondateur.md), [Document fondateur JayKonta](../JayKonta/JayKonta%20-%20Document%20Fondateur.md), [Interpolarite des services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md).

### Decisions structurantes (mini log)

| Id | Decision | Justification |
|----|----------|---------------|
| **DS-01** | Un seul service COG avec deux modes (boutique en ligne / PoS). | Meme socle Operateurs ; l'interface seule differe selon le contexte d'usage. |
| **DS-02** | Le catalogue produits est lu depuis JayXpose ; JayShop ne duplique pas les fiches produits. | Source de verite unique pour les produits, categories, stocks et visuels. |
| **DS-03** | L'admin peut CRUD produits et categories directement depuis JayShop (ecriture relayee vers JayXpose). | Ergonomie : l'admin ne devrait pas quitter JayShop pour gerer son catalogue pendant une session de vente. |
| **DS-04** | Les donnees de vente (tickets, lignes, paiements) sont detenues par JayShop ; les ecritures comptables sont transmises a JayKonta. | Responsabilites claires : JayShop = vente, JayKonta = comptabilite. |
| **DS-05** | Le mode PoS est reserve a l'admin ; les boutons et onglets PoS sont configurables. | Securite et personnalisation de l'interface de caisse. |
| **DS-06** | Les donnees de paiement sont classees niveau 3 (Critical). | Donnees financieres sensibles ; alignement politique securite. |
| **DS-07** | Les participations a des evenements/festivals ont un stock temporaire dedie. | Permet de suivre l'etat des stocks et les benefices par evenement sans impacter le stock global avant cloture. |
| **DS-08** | Integration automatique avec JayFestival lors de la validation d'une candidature exposant. | L'admin n'a pas a saisir manuellement les fiches evenement quand il participe a un festival via JayFestival. |
| **Dependance critique** | JayXpose (catalogue, stocks) et JayKonta (comptabilite) doivent exister pour que JayShop fonctionne pleinement. | — |
| **Dependance optionnelle** | JayFestival pour la synchronisation automatique des participations a des evenements. | — |

---

## 1. Raison d'etre

### 1.1 Proposition de valeur

**JayShop** permet a des **vendeurs** (artisans, commercants, exposants, petites entreprises, associations) de :

- **Vendre en ligne** : boutique web avec panier, commande, selection du mode de paiement, confirmation.
- **Vendre en physique (PoS)** : interface de caisse avec boutons configurables, saisie rapide, selection des produits, ecran de paiement avec calcul du rendu monnaie et du reste a payer.
- **Gerer les produits et categories** : CRUD complet depuis JayShop (relaye vers JayXpose), sans quitter le contexte de vente.
- **Suivre l'historique des ventes** : tickets, detail des lignes, mode de paiement, montants, date et heure, statuts.
- **S'integrer au suivi comptable** : transmission automatique des ventes a JayKonta pour rapports, TVA, bilan.
- **Fonctionner hors-ligne** : le mode PoS supporte le fonctionnement offline avec synchronisation a la reconnexion (LOI-1, LOI-2).

### 1.2 Positionnement

| Mode | Description |
|------|-------------|
| **Boutique en ligne** | Surface web accessible au grand public (client non authentifie ou authentifie). L'admin gere les produits, les commandes et les parametres de la boutique. Le client parcourt le catalogue, ajoute au panier, passe commande et paie. |
| **Point de vente (PoS)** | Interface de caisse reservee a l'admin. Boutons produits et onglets configurables. Saisie rapide de ticket. Ecran de paiement avec recap, saisie du montant donne, choix du mode de paiement, calcul du rendu monnaie ou du reste a payer. Impression ou envoi du recu. |

### 1.3 Phrase fondatrice

> **JayShop est la surface de vente de l'ecosysteme Miyukini. Un catalogue, une boutique, une caisse — couples avec JayXpose et JayKonta, gouvernes par le COG.**

---

## 2. Fonctionnalites structurantes

### 2.1 Onboarding vendeur

| Fonctionnalite | Description |
|----------------|-------------|
| Activation du mode vente | L'admin active JayShop depuis Miyukini Central. Si un profil JayXpose existe, le catalogue est lie automatiquement. Sinon, creation guidee. |
| Configuration boutique | Nom de la boutique, slug URL, devise, modes de paiement acceptes, informations legales (CGV, mentions legales). |
| Configuration PoS | Definition des onglets (categories rapides) et des boutons produits. Configuration de l'interface de caisse (couleurs, taille des boutons, disposition). |
| Parametres fiscaux | Taux de TVA par defaut, arrondis, devise. |

### 2.2 Gestion des produits (via JayXpose)

| Fonctionnalite | Description |
|----------------|-------------|
| Lecture catalogue | JayShop lit le catalogue JayXpose en temps reel (produits, categories, visuels, prix, disponibilite). |
| CRUD depuis JayShop | L'admin peut creer, modifier, supprimer des produits et categories directement depuis JayShop. Les ecritures sont relayees a JayXpose (source de verite). |
| Synchronisation stocks | Chaque vente decremente le stock. Synchronisation bidirectionnelle avec JayXpose (push/pull, resolution de conflits). |
| Produits PoS | L'admin peut marquer des produits comme « favoris PoS » et les affecter a des onglets pour la saisie rapide en caisse. |
| Import / Export | Import CSV de produits en masse. Export CSV du catalogue. |
| Alertes de stock | Notifications quand le stock d'un produit passe sous un seuil configurable. Alerte en caisse si vente avec stock negatif. |

### 2.3 Remises pre-definies

| Fonctionnalite | Description |
|----------------|-------------|
| Entites remises | Les remises sont des **entites pre-configurees** (nom, valeur, type % ou montant fixe). L'admin les cree a l'avance (ex. « -10% fidelite », « Lot 3 pour 2 », « Remise salon »). |
| Application en caisse | En PoS ou en ligne, une remise pre-definie est appliquee en un clic sur une ligne ou sur le total du ticket. |
| Acces restreint | En contexte multi-vendeurs (Phase 2), certaines remises peuvent etre reservees a l'admin. |

### 2.4 Taxes et fiscalite

| Fonctionnalite | Description |
|----------------|-------------|
| Multi-taux de taxe | Plusieurs taux de taxe configurables (TVA 20%, TVA reduite 5.5%, TVA resto 10%, exonere 0%). Inspire du modele Loyverse. |
| Association par article | Chaque article peut avoir un ou plusieurs taux de taxe associes. Taux par defaut applicable automatiquement aux nouveaux articles. |
| Taxe incluse ou ajoutee | Choix par taxe : prix TTC (taxe incluse) ou prix HT (taxe ajoutee au total). |

### 2.5 Fichier client (optionnel)

| Fonctionnalite | Description |
|----------------|-------------|
| Fiche client | Nom, email, telephone, adresse. S'appuie sur MiyuContacts. |
| Association ticket | Un ticket peut etre associe a un client (pour historique d'achats, envoi recu par email, fidelite). |
| Programme de fidelite | Phase 2 : pourcentage du montant d'achat credite en points de fidelite sur le compte client. |

### 2.6 Fonctionnalites activables (toggles)

| Fonctionnalite | Description |
|----------------|-------------|
| Activation modulaire | Les fonctionnalites avancees (sessions de caisse, tickets ouverts, alertes stock, fichier client, remises) sont **activables par toggle** dans les parametres. Cela simplifie l'interface pour les usages simples. |

### 2.7 Boutique en ligne

| Fonctionnalite | Description |
|----------------|-------------|
| Catalogue public | Page liste des produits avec filtres (categorie, prix, disponibilite), recherche, tri. |
| Fiche produit | Visuels, description, prix, disponibilite, bouton « Ajouter au panier ». |
| Panier | Ajout, modification des quantites, suppression. Recap avec sous-total, taxes, total. |
| Commande | Informations client (nom, email, adresse si livraison), selection du mode de paiement, validation. |
| Confirmation | Recap de la commande, numero de ticket, recu par email. |
| Suivi commande | L'admin voit les commandes entrantes, peut les marquer comme preparees, expediees, terminees. Le client peut consulter le statut. |

### 2.8 Point de vente (PoS) — Interface de caisse

| Fonctionnalite | Description |
|----------------|-------------|
| Ecran principal PoS | Zone gauche : ticket en cours (lignes produit, quantites, prix unitaire, sous-total par ligne). Zone droite : grille de boutons produits organises par onglets configurables. |
| Onglets configurables | L'admin cree des onglets (ex. « Boissons », « Plats », « Desserts », « Favoris »). Chaque onglet contient une grille de boutons produits. |
| Boutons produits | Chaque bouton affiche le nom du produit et optionnellement le prix. Personnalisable : **couleur + forme** (carre, rond, badge, hexagone) ou **image** du produit (visuel JayXpose). Clic = ajout d'une unite au ticket. Inspire de la « Representation on POS » de Loyverse. |
| Modification ticket | Modifier la quantite d'une ligne, supprimer une ligne, appliquer une remise (pourcentage ou montant). |
| Tickets ouverts | Sauvegarder un ticket en cours, ouvrir un nouveau, reprendre un ticket sauvegarde. |
| Recherche produit | Barre de recherche et/ou scan code-barres pour ajouter un produit non present dans les boutons rapides. |
| Ecran de paiement | Declenche par le bouton « Paiement ». Affiche un recap : liste des lignes, sous-total, taxes, total a payer. |
| Selection mode de paiement | Boutons pour chaque mode de paiement configure (Especes, Carte bancaire, Cheque, Virement, Autre). |
| Calcul rendu monnaie | Pour le paiement en especes : champ de saisie du montant donne par le client. Affichage instantane du rendu monnaie (montant donne - total) ou du reste a payer (si montant insuffisant). |
| Paiement mixte | Le client peut payer en plusieurs fois ou avec differents modes. Le reste a payer est mis a jour a chaque paiement partiel. |
| Validation paiement | Une fois le total atteint ou depasse, le paiement est valide. Le ticket est cloture. Le stock est decremente. |
| Recu | Impression thermique et/ou envoi par email. Contient : numero de ticket, date/heure, lignes, total, mode(s) de paiement, rendu monnaie. |

### 2.9 Gestion des evenements et festivals (Integration JayFestival)

JayShop permet de gerer les **participations a des evenements ou festivals** avec suivi des couts, des stocks temporaires et des benefices par evenement.

| Fonctionnalite | Description |
|----------------|-------------|
| **Fiche evenement** | Creation manuelle d'une fiche evenement avec : nom, dates, lieu, organisateur, edition JayFestival liee (optionnel). Ou **creation automatique** lors de la validation d'une candidature exposant par l'organisateur JayFestival. |
| **Couts de participation** | Saisie des couts lies a l'evenement : **prix du stand**, **nourriture**, **logement**, **transport**, **autres frais** (badges, electricite, etc.). Chaque cout a un libelle, un montant, une categorie et une date. |
| **Stock temporaire** | Stock **pre-alloue a l'evenement** : l'admin definit les produits et quantites qu'il emmene sur l'evenement. Ce stock temporaire est **deduit du stock global** (JayXpose) et alloue a l'evenement. A la cloture, le stock non vendu est reintegre au stock global. |
| **Ventes sur evenement** | Les tickets PoS peuvent etre **associes a un evenement**. Chaque vente decremente le stock temporaire de l'evenement (et non le stock global directement). |
| **Suivi temps reel** | Dashboard par evenement : stock restant, CA realise, couts saisis, **benefice brut** (CA - couts), **benefice net** (CA - couts - cout des marchandises vendues). |
| **Cloture d'evenement** | A la cloture : synthese finale, reintegration du stock non vendu au stock global, transmission comptable a JayKonta avec ventilation par evenement. |
| **Liaison JayFestival** | Si l'exposant utilise JayFestival, la fiche evenement peut etre **creee automatiquement** lorsque l'organisateur valide sa candidature. Les informations (nom, dates, lieu, stand attribue) sont synchronisees. L'admin peut enrichir avec les couts et le stock temporaire. |
| **Multi-evenements** | Un admin peut gerer plusieurs evenements en parallele avec des fiches, stocks et suivis independants. Alignement avec le modele cross-evenements de JayFestival. |

### 2.10 Historique des ventes et tickets

| Fonctionnalite | Description |
|----------------|-------------|
| Liste des tickets | Vue chronologique de tous les tickets (date, heure, montant, statut, mode de paiement). Filtres par date, statut, mode de paiement. |
| Detail d'un ticket | Lignes produits, quantites, prix, remises, sous-total, taxes, total, mode(s) de paiement, rendu monnaie, horodatage. |
| Remboursement | Remboursement total ou partiel d'un ticket. Generation d'un ticket de remboursement lie au ticket original. Reajustement du stock. |
| Export | Export CSV ou PDF de l'historique des ventes (periodes, filtres). |
| Tableau de bord | Synthese : chiffre d'affaires du jour/semaine/mois, nombre de tickets, panier moyen, top produits vendus. KPIs inspires de Loyverse : Gross sales, Refunds, Discounts, Net sales, Gross profit en bande horizontale. |
| Sous-rapports | Ventes par produit, par categorie, par mode de paiement, par taxes. Inspire des 9 rapports Loyverse. |

### 2.10 Gestion de caisse (PoS)

| Fonctionnalite | Description |
|----------------|-------------|
| Ouverture de caisse | Saisie du fond de caisse initial (montant en especes au demarrage). |
| Mouvements de caisse | Entrees et sorties manuelles d'especes (depot, retrait), avec motif. |
| Cloture de caisse | Synthese de la session : total des ventes, ventilation par mode de paiement, especes attendues vs comptees, ecart. |
| Periodes de travail (shifts) | Suivi des sessions de caisse par utilisateur (debut, fin, total). |

---

## 3. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Gouvernance COG** | Le service fonctionne sous gouvernance COG : StrongFather (decisions), KindMother (persistance), Master Butler (capacites/permissions), WorrySentinel (securite), Ever Buddy (cycle de vie). |
| **Source de verite unique** | Le catalogue produits et les stocks resident dans JayXpose. JayShop lit et ecrit via les interfaces contractuelles. Les donnees de vente (tickets, paiements) resident dans JayShop. La comptabilite reside dans JayKonta. |
| **Reutilisabilite** | S'appuyer sur les Kits d'outils Miyukini existants (MiyuInvoice, MiyuContacts, MiyuPayment) et les Operateurs JayXpose et JayKonta. |
| **Interpolarite** | Concu pour s'integrer dans l'ecosysteme Jay. Les couplages avec JayXpose et JayKonta sont explicites et gouvernes (Mandats de Permission, niveaux de securite). |
| **Hors-ligne d'abord** | Le mode PoS doit fonctionner sans connexion internet. Les ventes sont enregistrees localement et synchronisees a la reconnexion (LOI-1, LOI-2, LOI-3). |
| **Ergonomie de caisse** | L'interface PoS est optimisee pour la vitesse : grands boutons, onglets configurables, saisie minimale, calcul de monnaie instantane. Inspire de Loyverse POS. |
| **Confidentialite** | Les donnees de paiement (montants, moyens de paiement) ne sont jamais exposees publiquement. Le partage vers JayKonta est gouverne. |

---

## 4. Integration et interpolarite

### 4.1 JayShop et JayXpose

| Fonctionnalite | Description |
|----------------|-------------|
| Lecture catalogue | JayShop lit les produits, categories, visuels, prix et disponibilite depuis JayXpose. |
| Ecriture catalogue | L'admin peut CRUD les produits et categories depuis JayShop ; les ecritures sont relayees a JayXpose via l'interface contractuelle IFS-06. |
| Synchronisation stocks | A chaque vente, JayShop decremente le stock et notifie JayXpose. En cas de modification de stock cote JayXpose, JayShop est notifie (push/pull). |
| Resolution de conflits | Politique configurable : `prefer_pos`, `prefer_local`, `manual_review` (alignement avec le protocole de sync existant JayXpose/PoS). |

### 4.2 JayShop et JayKonta

| Fonctionnalite | Description |
|----------------|-------------|
| Transmission des ventes | A chaque cloture de ticket, JayShop transmet un resume comptable a JayKonta (montant HT, TVA, TTC, mode de paiement, date). |
| Remboursements | Les remboursements sont egalement transmis a JayKonta (ecriture inverse). |
| Rapprochement | JayKonta peut rapprocher les ventes JayShop avec les encaissements bancaires. |
| Clotures de caisse | Les syntheses de cloture de caisse sont transmises a JayKonta pour le suivi des especes. |

### 4.3 JayShop et JayFestival (Gestion des evenements)

| Fonctionnalite | Description |
|----------------|-------------|
| **Creation automatique de fiche evenement** | Lorsque l'organisateur JayFestival **valide la candidature** d'un exposant, une fiche evenement est automatiquement creee dans JayShop avec les informations de l'edition (nom, dates, lieu, stand attribue). |
| **Synchronisation des informations** | Les mises a jour cote JayFestival (changement de dates, de stand, annulation) sont synchronisees vers JayShop. |
| **Stock temporaire** | L'admin JayShop peut definir un stock temporaire pour l'evenement. Ce stock est **deduit du stock global** (JayXpose) et alloue a l'evenement. Les ventes PoS associees a l'evenement decrementent ce stock temporaire. |
| **Couts de participation** | L'admin saisit les couts lies a l'evenement (stand, transport, logement, nourriture). Ces couts ne proviennent pas de JayFestival (qui gere la facturation cote organisateur) mais du suivi interne du vendeur. |
| **Suivi des benefices** | Dashboard evenement avec CA, couts, stock restant, benefice net. Permet de mesurer la rentabilite de chaque participation. |
| **Cloture et reintegration stock** | A la cloture de l'evenement, le stock non vendu est reintegre au stock global. La synthese est transmise a JayKonta. |

### 4.4 JayShop et les autres services Jay

| Service | Integration |
|---------|-------------|
| **JayXpose** | Catalogue, stocks, profil entreprise, visuels. |
| **JayKonta** | Comptabilite, facturation, rapprochement, TVA. |
| **JayFestival** | Gestion des participations a des evenements/festivals : creation automatique de fiche evenement a la validation de candidature, synchronisation des informations. |
| **JayKoa** | Synchronisation agenda pour evenements de vente. |
| **Miyukini Central** | Hebergement de JayShop dans le shell Central (trait `ServiceUi`). |
| **Miyukini Web Portal** | Surface web de la boutique en ligne (portail client). |

### 4.4 Reference interpolarite

Voir [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) pour le principe global et les couplages entre services Jay.

---

## 5. Modele de donnees (orientation)

### 5.1 Ticket de vente

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique du ticket. |
| ticket_number | TEXT | Numero sequentiel affiche (ex. `JSHOP-2026-00042`). |
| seller_id | UUID (FK) | Vendeur / admin qui a saisi le ticket. |
| source | TEXT | `pos` ou `online`. |
| status | TEXT | `draft` / `paid` / `refunded` / `cancelled`. |
| subtotal | NUMERIC | Sous-total HT (en centimes). |
| tax_total | NUMERIC | Total TVA. |
| total | NUMERIC | Total TTC. |
| currency | TEXT | Devise (defaut EUR). |
| created_at | TIMESTAMPTZ | Date/heure de creation. |
| closed_at | TIMESTAMPTZ | Date/heure de cloture (paiement valide). |

### 5.2 Ligne de ticket

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la ligne. |
| ticket_id | UUID (FK) | Ticket parent. |
| product_id | UUID (FK) | Produit (reference JayXpose). |
| product_name | TEXT | Nom du produit (snapshot au moment de la vente). |
| quantity | INTEGER | Quantite vendue. |
| unit_price | NUMERIC | Prix unitaire (en centimes, snapshot). |
| discount | NUMERIC | Remise appliquee (en centimes ou pourcentage). |
| discount_type | TEXT | `percent` ou `amount`. |
| tax_rate | NUMERIC | Taux de TVA applique. |
| line_total | NUMERIC | Total de la ligne TTC. |

### 5.3 Paiement

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique du paiement. |
| ticket_id | UUID (FK) | Ticket associe. |
| method | TEXT | `cash` / `card` / `check` / `transfer` / `other`. |
| amount | NUMERIC | Montant paye (en centimes). |
| given_amount | NUMERIC | Montant donne par le client (especes). |
| change_amount | NUMERIC | Rendu monnaie (especes). |
| created_at | TIMESTAMPTZ | Date/heure du paiement. |

### 5.4 Session de caisse

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la session. |
| seller_id | UUID (FK) | Utilisateur admin. |
| opened_at | TIMESTAMPTZ | Date/heure d'ouverture. |
| closed_at | TIMESTAMPTZ | Date/heure de cloture. |
| opening_cash | NUMERIC | Fond de caisse initial (en centimes). |
| closing_cash_expected | NUMERIC | Especes attendues a la cloture. |
| closing_cash_counted | NUMERIC | Especes comptees a la cloture. |
| cash_difference | NUMERIC | Ecart (comptees - attendues). |
| total_sales | NUMERIC | Total des ventes de la session. |
| total_refunds | NUMERIC | Total des remboursements. |

### 5.5 Configuration PoS

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la configuration. |
| seller_id | UUID (FK) | Admin proprietaire. |
| tabs | JSON | Liste des onglets (nom, ordre, icone, couleur). |
| buttons | JSON | Boutons par onglet (product_id, label, couleur, position, taille). |
| payment_methods | JSON | Modes de paiement actifs et leur ordre d'affichage. |
| receipt_settings | JSON | Parametres de recu (logo, texte haut/bas, format). |

### 5.6 Fiche evenement

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de la participation. |
| seller_id | UUID (FK) | Admin proprietaire. |
| name | TEXT | Nom de l'evenement (ex. « Salon du Livre 2026 »). |
| start_date | DATE | Date de debut de l'evenement. |
| end_date | DATE | Date de fin de l'evenement. |
| location | TEXT | Lieu de l'evenement. |
| stand_info | TEXT | Informations sur le stand attribue (numero, zone, taille). |
| status | TEXT | `draft` / `confirmed` / `ongoing` / `closed` / `cancelled`. |
| jayfestival_edition_id | UUID (FK, optionnel) | Liaison avec une edition JayFestival. |
| jayfestival_candidature_id | UUID (FK, optionnel) | Liaison avec la candidature JayFestival. |
| created_at | TIMESTAMPTZ | Date de creation. |
| closed_at | TIMESTAMPTZ | Date de cloture. |
| notes | TEXT | Notes internes. |

### 5.7 Couts de participation

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique du cout. |
| event_id | UUID (FK) | Fiche evenement parent. |
| category | TEXT | `stand` / `transport` / `lodging` / `food` / `other`. |
| label | TEXT | Libelle (ex. « Billet train A/R », « Hotel 3 nuits »). |
| amount | NUMERIC | Montant en centimes. |
| currency | TEXT | Devise (defaut EUR). |
| date | DATE | Date du cout (pour ventilation). |
| receipt_url | TEXT | URL justificatif (optionnel). |
| created_at | TIMESTAMPTZ | Date de creation. |

### 5.8 Stock temporaire evenement

| Champ | Type | Description |
|-------|------|-------------|
| id | UUID | Identifiant unique de l'allocation. |
| event_id | UUID (FK) | Fiche evenement parent. |
| product_id | UUID (FK) | Produit (reference JayXpose). |
| allocated_qty | INTEGER | Quantite allouee pour l'evenement. |
| sold_qty | INTEGER | Quantite vendue sur l'evenement. |
| remaining_qty | INTEGER | Quantite restante (= allocated - sold). |
| returned_qty | INTEGER | Quantite reintegree au stock global a la cloture. |
| created_at | TIMESTAMPTZ | Date de creation. |
| updated_at | TIMESTAMPTZ | Date de mise a jour. |

---

## 6. Types de service

| Espace | Description |
|--------|-------------|
| **Central** | Interface admin : gestion produits, PoS, historique, parametres, tableau de bord. Service interne COG (Type 1) pour la partie admin. |
| **Portail** | Boutique en ligne accessible au public. Service a surface web externe (Type 2) pour la partie client. |

**Regle** : JayShop est un **Service de Type 2** (surface web externe pour la boutique) avec une composante **Type 1** (PoS et gestion reserves a l'admin dans Central).

---

## 7. Niveaux de securite (orientation)

| Categorie de donnees | Niveau | Justification |
|----------------------|--------|---------------|
| Boutique en ligne (catalogue public) | **Public (0)** a **Standard (1)** | Contenu destine a etre visible par le client. |
| Informations commande client (nom, email) | **Sensitive (2)** | Donnees personnelles. |
| Tickets et lignes de vente | **Standard (1)** a **Sensitive (2)** | Donnees commerciales. |
| Paiements (montants, modes, transactions) | **Critical (3)** | Donnees financieres sensibles. |
| Configuration PoS (boutons, onglets) | **Standard (1)** | Configuration metier non sensible. |
| Sessions de caisse (fonds, ecarts) | **Sensitive (2)** a **Critical (3)** | Donnees financieres. |
| Donnees de stock (via JayXpose) | **Standard (1)** a **Sensitive (2)** | Selon strategie commerciale. |

---

## 8. Prochaines etapes (orientation)

1. **Fonder** : Valider ce document fondateur et le diffuser.
2. **Specifier** : Documenter les Operateurs et Kits JayShop (ticket, paiement, session caisse, config PoS).
3. **Integration** : Formaliser les contrats de synchronisation avec JayXpose (catalogue, stocks) et JayKonta (comptabilite).
4. **Securite** : Formaliser les niveaux de securite et la politique de confidentialite inter-services.
5. **Implementation** : Developper les Operateurs et Kits en s'appuyant sur les Cores.
6. **Phase 2** : Integration JayFestival (PoS evenementiel), programme de fidelite, multi-vendeurs.

---

## 9. References

| Document | Role |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie (Operateur, Mandat, COG, Niveaux de securite). |
| [JayXpose - Document Fondateur](../JayXpose/JayXpose%20-%20Document%20Fondateur.md) | Service fournisseur du catalogue produits et des stocks. |
| [JayXpose - Catalogue Produits](../JayXpose/JayXpose%20-%20Catalogue%20Produits.md) | Specification du module catalogue consomme par JayShop. |
| [JayKonta - Document Fondateur](../JayKonta/JayKonta%20-%20Document%20Fondateur.md) | Service consommateur des ecritures comptables. |
| [JayKonta - Integration Services](../JayKonta/reference/JayKonta%20-%20Integration%20Services.md) | Schemas d'integration comptable. |
| [JayXpose - Interfaces Inter-Services](../JayXpose/JayXpose%20-%20Interfaces%20Inter-Services.md) | Contrats inter-services existants (IFS-04 PoS). |
| [Miyukini Sales - Document Fondateur](../MiyukiniSales/Miyukini%20Sales%20-%20Document%20Fondateur.md) | Service de ventes B2B/B2C (socle Operateurs partages). |
| [Miyukini Conceptual References - Interpolarite Services Jay](../../reference/Miyukini%20Conceptual%20References%20-%20Interpolarite%20Services%20Jay.md) | Principe d'interpolarite et couplage entre services Jay. |
| [JayShop - Reference Loyverse Back Office](./reference/JayShop%20-%20Reference%20Loyverse%20Back%20Office.md) | Analyse concurrentielle et screenshots annotes du back office Loyverse POS. |
| [JayFestival - Document Fondateur](../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service fournisseur des informations evenements (editions, candidatures). |

---

**Document** : JayShop — Document fondateur
**Version** : 1.2
**Date** : 2026-02-11
**Statut** : Document de reference — enrichi avec gestion des evenements/festivals et integration JayFestival.
