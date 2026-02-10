# JayKonta — Analyse PR Concurrence Web

## Contexte

Ce document constitue l'**analyse PR (Product Review) concurrentielle** du service **JayKonta** face aux solutions de comptabilite, facturation et gestion financiere disponibles sur le marche web. Il couvre les segments **comptabilite entreprise** (point d'entree Account) et **budget personnel** (point d'entree Purse / JayBudget), face aux concurrents directs et indirects.

L'objectif est d'extraire de chaque concurrent :
- Les **fonctionnalites** cles et leur couverture
- Les **parcours utilisateurs** types
- Les **elements UI/UX** remarquables
- Les **avantages** concurrentiels
- Les **passerelles et adaptations COG** possibles pour JayKonta

## Portee / Scope

- **Applicable a :** Positionnement produit, roadmap fonctionnelle, decisions d'architecture UX
- **Audience :** Architectes, product owners, equipes design, contributeurs
- **Statut :** Document d'analyse concurrentielle PR

---

## 1. Methodologie

### 1.1 Approche

L'analyse suit une methodologie en trois phases :

1. **Extraction** : Identification des fonctionnalites depuis les pages produit officielles, documentations publiques et comparatifs
2. **Categorisation** : Classement par domaine fonctionnel aligne sur la grille de besoins JayKonta (MAC-01 a MAC-19 pour Account, PUR-01 a PUR-16 pour Purse)
3. **Transposition COG** : Pour chaque fonctionnalite concurrente, identification de l'adaptation dans le modele COG (Operateurs, Kits d'Outils, Cores, Mandats)

### 1.2 Grille d'analyse par concurrent

Chaque concurrent est analyse selon :

| Dimension | Description |
|-----------|-------------|
| **Segment** | Positionnement marche (SMB, Enterprise, Personal) |
| **Fonctionnalites** | Liste exhaustive des capacites |
| **Parcours utilisateur** | Flux types (devis → facture → paiement, etc.) |
| **Elements UI/UX** | Patterns d'interface remarquables |
| **Avantages** | Positionnement differentiel |
| **Passerelles COG** | Adaptation dans le modele Miyukini |
| **Ecarts vs JayKonta** | Fonctionnalites absentes ou approche incompatible COG |

### 1.3 Baseline JayKonta (rappel)

| Aspect | JayKonta Account | JayBudget Purse |
|--------|-----------------|-----------------|
| **Public** | Professionnels, associations, TPE/PME | Particuliers, foyers |
| **Perimetre** | Grand livre, devis, factures, paiements, rapports legaux | Mouvements, categories, budgets occasionnels, objectifs |
| **Besoins** | MAC-01 a MAC-19 | PUR-01 a PUR-16 |
| **Securite** | Niveau 2-3, residence centralisee | Niveau 2, residence selon politique |
| **Integrations** | JayFestival, JayRDV, futurs services | JayKoa (optionnel, rappels) |
| **7 Toolkits** | MiyuComptaLedger, MiyuComptaReports, MiyuDeclarations, MiyuBilling, MiyuInvoice, MiyuExpense, MiyuTreasury | Sous-ensemble selon contexte |
| **Gouvernance** | StrongFather, KindMother, MasterButler, WorrySentinel | Idem |

---

## 2. Concurrents Segment Entreprise — ERP et Comptabilite

### 2.1 Odoo Accounting + Invoicing

**Segment :** ERP modulaire SMB/Mid-market — Open source avec edition Enterprise

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Grand livre, ecritures journalieres, plan comptable personnalisable, exercices fiscaux |
| **Banque** | Synchronisation bancaire automatique (Open Banking, Yodlee, Ponto), import releves, rapprochement intelligent avec suggestions IA |
| **Facturation** | Creation factures, avoirs, factures fournisseurs, factures recurrentes, modeles personnalisables, facturation electronique |
| **Devis** | Devis → commande → facture brouillon, validation workflow |
| **Paiements** | Multi-passerelles (Stripe, PayPal, Mollie, etc.), portail client pour paiement en ligne |
| **Rapports** | Bilan, compte de resultat, flux de tresorerie, balance agee, analytique par axes |
| **Budget** | Gestion budgetaire avec comptabilite analytique, suivi ecarts |
| **Actifs** | Immobilisations, amortissements, revenus differes |
| **Multi** | Multi-societe, multi-devise, consolidation inter-societes |
| **Relances** | Suivi automatique des impayes, niveaux de relance configurables |
| **TVA** | Detection automatique, declarations, intra-communautaire |

#### Parcours utilisateurs types

1. **Cycle devis → facture** : Devis → acceptation client → commande vente → facture brouillon auto → validation → envoi email → paiement en ligne via portail client → rapprochement automatique
2. **Rapprochement bancaire** : Connexion banque → import automatique quotidien → suggestions de rapprochement → validation manuelle ou auto → ecritures comptables
3. **Facturation fournisseur** : Reception facture → saisie/OCR → validation workflow → lot de paiement → virement → rapprochement
4. **Relances** : Facture echue → detection automatique → email de relance niveau 1 → escalade niveau 2 → mise en contentieux

#### Elements UI/UX remarquables

- **Dashboard comptable** avec soldes des journaux bancaires et indicateurs cles en temps reel
- **Portail client** : vue liste factures, statut, paiement en ligne integre
- **Ecran de rapprochement** : suggestions cote-a-cote avec confiance IA, validation en un clic
- **Templates factures** : branding complet (logo, couleurs, mentions legales), preview en temps reel
- **Vue Kanban** des factures par statut (brouillon, envoyee, payee, en retard)
- **Analytique multi-axes** : tag/projet/departement sur chaque ecriture

#### Avantages concurrentiels

- **Couverture ERP complete** : comptabilite integree avec ventes, achats, inventaire, RH dans une seule suite
- **Automatisation bancaire** : rapprochement intelligent qui apprend des corrections
- **Multi-societe natif** : consolidation, prets inter-societes, regles fiscales par entite
- **Open source** (Community) avec migration possible vers Enterprise
- **Ecosysteme modules** : 30+ modules financiers disponibles

#### Passerelles et adaptations COG

| Fonctionnalite Odoo | Adaptation JayKonta COG |
|---------------------|------------------------|
| Portail client paiement | Acces visiteur via Visa (BorderGuard) avec scope lecture factures + paiement |
| Synchronisation bancaire | `tool.compta.bank.sync` sous WorrySentinel niveau 2+, audit KindMother |
| Multi-societe | Multi-COG avec contrats inter-COG explicites, pas de partage implicite |
| Rapprochement IA | `tool.compta.reconciliation.suggest` — suggestion sans decision, validation StrongFather |
| Workflow approbation | Mandats de Permission emis par StrongFather, roles via MasterButler |
| Factures recurrentes | `tool.billing.invoice.generate` recurrence avec Mandat permanent controle |

#### Ecarts vs JayKonta

- Odoo n'a pas de **souverainete environnement** : les donnees sont sur serveur centralise (SaaS) ou auto-heberge sans gouvernance COG
- Pas de **separation decision/execution** : un utilisateur admin peut tout faire sans mediation Core
- Le modele **multi-societe** est intra-instance, pas inter-environnements souverains

---

### 2.2 QuickBooks Online

**Segment :** Comptabilite SMB grand public — SaaS leader en Amerique du Nord

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Plan comptable, ecritures automatiques, categorisation par regles |
| **Banque** | Connexion bancaire automatique, import carte credit, categorisation IA |
| **Facturation** | Creation rapide, personnalisation modeles, envoi email/lien, statut en temps reel |
| **Devis** | Devis personnalises, acceptation en ligne, conversion en facture |
| **Paiements** | QuickBooks Payments (carte, ACH), suivi statut |
| **Depenses** | Suivi depenses avec connexion bancaire, scan justificatifs mobile, categories auto |
| **Rapports** | P&L, bilan, flux tresorerie, rapports personnalises, tableau de bord |
| **TVA/Taxes** | Calcul automatique taxes ventes, rapports TVA |
| **Paiement factures** | Bill Pay automatise pour payer les fournisseurs |
| **Projet** | Suivi rentabilite par projet, imputation temps et depenses |

#### Parcours utilisateurs types

1. **Expense tracking** : Connexion banque → import automatique transactions → categorisation IA → revision → rapport mensuel
2. **Invoice-to-cash** : Facture → envoi → client paie par carte/ACH → statut « paye » auto → ecriture comptable auto
3. **Receipt capture** : Photo justificatif mobile → OCR → association transaction → categorie depense → rapport
4. **Project billing** : Creer projet → imputer temps/depenses → generer facture depuis projet → envoi

#### Elements UI/UX remarquables

- **Dashboard simplifie** : graphiques depenses/revenus, solde bancaire, factures impayees en un coup d'oeil
- **Capture mobile** : snap justificatif → extraction IA → association automatique
- **Suivi statut factures** : timeline visuelle (creee → envoyee → vue → payee)
- **Categorisation intelligente** : regles qui apprennent du comportement utilisateur
- **Bill Pay** : paiement fournisseurs directement depuis l'application

#### Avantages concurrentiels

- **Simplicite d'utilisation** : UX orientee non-comptable, parcours guide
- **Ecosysteme integrations** : 750+ applications tierces connectees
- **Mobile-first** : application mobile complete pour facturation et justificatifs
- **Banque automatisee** : connexion + categorisation + rapprochement en arriere-plan
- **Marque etablie** : confiance et adoption massive

#### Passerelles et adaptations COG

| Fonctionnalite QuickBooks | Adaptation JayKonta COG |
|--------------------------|------------------------|
| Connexion bancaire auto | `tool.compta.bank.sync` avec audit trail KindMother |
| Receipt capture mobile | `tool.expense.receipt.capture` + `tool.expense.receipt.extract` (OCR) |
| Statut paiement auto | Evenement `payment.status` dans lifecycle facture JayKonta |
| Categorisation IA | `tool.compta.transaction.categorize` — regles fournies, pas d'IA autonome non gouvernee |
| Bill Pay | `tool.compta.payment.batch` sous Mandat StrongFather |

#### Ecarts vs JayKonta

- Modele **100% cloud** sans option offline-first ni souverainete locale
- Categorisation IA **non transparente** — pas de separation suggestion/decision comme StrongFather
- Pas de modele de **gouvernance** des permissions (admin/utilisateur basique seulement)

---

### 2.3 Xero

**Segment :** Comptabilite SMB — Leader en Australie/NZ, fort au UK et USA

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Banque** | Bank feeds automatiques, rapprochement avec suggestions, import releves |
| **Facturation** | Creation mobile, modeles, envoi, paiements en ligne, statut temps reel |
| **Devis** | Devis → acceptation → conversion facture sans ressaisie |
| **Paiements** | Suivi payables et receivables, lots de paiement, integration GoCardless/Stripe |
| **Rapports** | Bilan, P&L, balance agee, rapports personnalisables |
| **Achats** | Bons de commande, factures fournisseurs, approbation |
| **Projet** | Suivi cout par projet, facturation sur depenses/temps |
| **Multi** | Multi-devise, multi-organisation via Xero HQ |

#### Parcours utilisateurs types

1. **Quote-to-cash** : Devis → acceptation client en ligne → conversion facture auto → envoi → paiement en ligne → rapprochement
2. **Bank reconciliation** : Feed bancaire → suggestion matching → validation en un clic → ecriture finalisee
3. **Mobile invoicing** : App mobile → creer facture → envoyer → suivre statut → notification paiement

#### Elements UI/UX remarquables

- **Application mobile** complete pour facturation : creation, envoi, suivi en mobilite
- **Vue rapprochement** claire : transaction bancaire a gauche, suggestion a droite, validation centre
- **Flow devis → facture** : conversion en un clic avec reprise complete des donnees
- **Dashboard epure** : indicateurs essentiels sans surcharge visuelle

#### Avantages concurrentiels

- **Rapprochement bancaire** extremement fluide et rapide
- **Mobile invoicing** de reference dans l'industrie
- **Ecosysteme add-ons** : 1000+ applications connectees via API ouverte
- **Multi-devise** natif et robuste

#### Passerelles et adaptations COG

| Fonctionnalite Xero | Adaptation JayKonta COG |
|---------------------|------------------------|
| Devis → facture conversion | `quote.create` → `quote.to_invoice` via CK-OP-12 |
| Rapprochement suggestions | `tool.compta.reconciliation.suggest` + validation StrongFather |
| Mobile invoicing | JayKonta mobile via Miyukini Central (egui/eframe) ou PWA |
| Multi-org HQ | Multi-COG avec Webway et contrats inter-COG explicites |

#### Ecarts vs JayKonta

- Pas de **modele de souverainete** des donnees
- Multi-organisation ≠ multi-environnement souverain
- Pas de separation **decision (Core) / execution (Tool)**

---

### 2.4 Sage (Sage Accounting + Sage Intacct)

**Segment :** SMB (Sage Accounting) a Mid-market (Sage Intacct)

#### Sage Accounting — Fonctionnalites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Facturation** | Creation, envoi, modeles, relances automatiques |
| **Devis** | Devis → conversion facture |
| **Banque** | Rapprochement transactions, multi-devise |
| **Rapports** | Bilan, P&L, budget, previsionnel tresorerie |
| **Fournisseurs** | Factures fournisseurs, suivi paiements |
| **TVA** | Soumission TVA directe (UK Making Tax Digital) |

#### Sage Intacct — Fonctionnalites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Core Financials** | GL, AR, AP, gestion tresorerie |
| **Achats** | Bons de commande, approbation, suivi |
| **Multi-entite** | Consolidation inter-entites, reporting multi-niveaux |
| **Dimensions** | Comptabilite analytique multi-dimensions (departement, projet, localisation) |
| **Budget** | Budget detaille par entite et dimension |
| **Audit** | Trail complet, conformite SOC/SOX |

#### Parcours utilisateurs types (consolide)

1. **Sage Accounting** : Devis → facture → relance auto → paiement → rapprochement
2. **Sage Intacct** : Facture AP → workflow approbation → lot paiement → rapprochement → consolidation multi-entite

#### Elements UI/UX remarquables

- **Dashboards financiers** : Sage Intacct propose des tableaux de bord analytiques multi-dimensions
- **Previsionnel tresorerie** : Sage Accounting avec projection graphique du cash flow
- **Workflow relance** : niveaux configurables avec escalade automatique

#### Avantages concurrentiels

- **Sage Intacct** : reference mid-market pour multi-entite et audit
- **Sage Accounting** : simplicite SMB avec TVA integree (UK)
- **Conformite** : SOC 1/2, SOX readiness (Intacct)
- **Scalabilite** : migration Sage Accounting → Intacct pour croissance

#### Passerelles et adaptations COG

| Fonctionnalite Sage | Adaptation JayKonta COG |
|--------------------|------------------------|
| Multi-entite Intacct | Multi-COG avec contrats inter-COG, consolidation par Bridge |
| Relances automatiques | Notifications via MiyuNotify sous Mandat StrongFather |
| Audit SOC/SOX | KindMother audit trail + WorrySentinel niveaux 2-3 |
| Dimensions analytiques | Ventilation par categorie/projet dans MiyuComptaLedger (MAC-05) |
| Previsionnel | `tool.treasury.forecast.compute` dans MiyuTreasury |

---

### 2.5 FreshBooks

**Segment :** Facturation et comptabilite pour freelances et petites entreprises

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Facturation** | Factures recurrentes, relances, penalites retard, carte en fichier |
| **Devis** | Propositions avec e-signature, conversion automatique en facture |
| **Depenses** | Connexion bancaire, import depenses, scan justificatifs |
| **Banque** | Rapprochement bancaire |
| **Rapports** | P&L, bilan, ecritures journalieres, rapports fiscaux |
| **Projet** | Suivi temps → facturation, rentabilite projet |
| **Paiements** | Paiements en ligne (carte, ACH, Apple Pay), lien de paiement |

#### Parcours utilisateurs types

1. **Proposal-to-cash** : Proposition/devis → e-signature client → conversion facture auto → paiement en ligne → reconciliation
2. **Time-to-invoice** : Suivi temps par projet → generer facture depuis temps → envoi → paiement
3. **Expense-to-billing** : Scan justificatif → categorie → marquer « refacturable » → inclure dans facture client

#### Elements UI/UX remarquables

- **Constructeur de propositions** : mise en page soignee, e-signature integree
- **Interface scan justificatifs** : photo → extraction → association transaction
- **Suivi temps** integre avec conversion directe en lignes de facture
- **Notifications paiement** : alerte temps reel quand le client consulte ou paie

#### Avantages concurrentiels

- **Flow proposition → facture** tres soigne et differenciant
- **E-signature** integree sans outil tiers
- **Suivi temps** natif avec facturation directe
- **Mobile complet** pour freelances en deplacement

#### Passerelles et adaptations COG

| Fonctionnalite FreshBooks | Adaptation JayKonta COG |
|--------------------------|------------------------|
| E-signature propositions | Futur : outil signature gouverne, validation StrongFather |
| Temps → facture | Integration JayRDV (prestations) → `invoice.emit` via contrat CK-INT |
| Depense refacturable | `tool.expense.claim.create` avec flag refacturable → `invoice.emit` |
| Notifications paiement | Evenement lifecycle facture via CaringNanny observation |

---

### 2.6 Zoho Books

**Segment :** Comptabilite SMB — Partie de la suite Zoho

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Banque** | Bank feeds, import releves, regles auto-categorisation |
| **Rapprochement** | Workflow rapprochement structure avec suggestions |
| **Depenses** | Suivi depenses, categories, depenses refacturables |
| **Inventaire** | Gestion stock integree a la comptabilite |
| **Facturation** | Factures, avoirs, recurrentes, modeles |
| **Integration** | Zoho Expense (notes de frais), Zoho CRM, Zoho Inventory |

#### Parcours utilisateurs types

1. **Bank reconciliation** : Feed bancaire → auto-categorisation par regles → rapprochement → validation
2. **Expense sync** : Zoho Expense → approbation → synchronisation vers Zoho Books → ecriture comptable
3. **Billable expenses** : Depense → marquer refacturable → inclure dans facture client

#### Elements UI/UX remarquables

- **Module bancaire** avec workflow de rapprochement etape par etape
- **Integration Zoho Expense** seamless : un clic pour synchroniser
- **Regles bancaires** : interface de creation de regles de categorisation automatique
- **Inventory tracking** lie aux ecritures (cout des marchandises)

#### Avantages concurrentiels

- **Integration Zoho suite** : CRM, Expense, Inventory, Projects dans un ecosysteme
- **Regles de rapprochement** configurables et intelligentes
- **Depenses refacturables** avec workflow clair
- **Prix agressif** avec plan gratuit

#### Passerelles et adaptations COG

| Fonctionnalite Zoho | Adaptation JayKonta COG |
|--------------------|------------------------|
| Expense approval workflow | StrongFather Mandat pour validation depenses |
| Sync inter-modules | Integration Services JayKonta (CK-INT) entre Operateurs |
| Inventory → compta | Futur : JayXpose (stock) → JayKonta via Bridge |
| Regles categorisation | `tool.compta.transaction.categorize` avec regles KindMother |

---

### 2.7 Wave

**Segment :** Comptabilite et facturation gratuite pour micro-entreprises

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Facturation** | Creation, personnalisation, recurrentes, relances automatiques |
| **Paiements** | Paiements en ligne (carte, ACH) integres |
| **Comptabilite** | Double entree, plan comptable, rapports |
| **Justificatifs** | Scan et stockage justificatifs |
| **Rapports** | P&L, bilan, balance agee, rapports taxes |

#### Parcours utilisateurs types

1. **Invoice-to-accounting** : Facture → envoi → paiement en ligne → ecriture comptable auto → rapport
2. **Recurring billing** : Configurer facture recurrente → envoi auto periodique → suivi statut → relances

#### Elements UI/UX remarquables

- **Templates factures** : drag-and-drop customisation avec preview
- **Vue client** avec historique factures complet
- **Connexion comptabilite-facturation** sans configuration

#### Avantages concurrentiels

- **Gratuit** pour comptabilite et facturation (monetisation sur paiements et payroll)
- **Simplicite extreme** : courbe d'apprentissage tres faible
- **Tout-en-un leger** : facturation + compta + justificatifs

#### Passerelles et adaptations COG

| Fonctionnalite Wave | Adaptation JayKonta COG |
|--------------------|------------------------|
| Sync facture → compta | Operateur Facturation → MiyuComptaLedger via BondingBrother |
| Factures recurrentes | `tool.billing.invoice.generate` avec Mandat recurrence |

---

## 3. Concurrents Segment France — Independants et TPE

### 3.1 Pennylane

**Segment :** Tout-en-un comptabilite + expert-comptable — France, cible independants et TPE/PME

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Facturation** | Devis, factures, avoirs, modeles, facturation electronique 2026, relances |
| **Comptabilite** | Synchronisation bancaire, ecritures auto, rapprochement, plan comptable |
| **TVA** | Detection auto, declarations, credit TVA |
| **Declarations** | URSSAF, liasse fiscale, bilan, compte de resultat |
| **Depenses** | Notes de frais, scan OCR, approbation workflow |
| **Tresorerie** | Dashboard tresorerie, previsionnel, alertes |
| **Collaboration** | Espace expert-comptable partage, droits granulaires, cloture assistee |
| **Multi-structures** | Micro, EURL, SASU, SAS — gestion multi-societes |

#### Parcours utilisateurs types

1. **Collaboration expert** : Comptabilite → partage securise avec cabinet → revision → cloture → liasse fiscale
2. **Facturation complete** : Devis → acceptation → facture → relance → encaissement → ecriture auto
3. **Declaration URSSAF** : CA saisi → calcul cotisations → preparation declaration → soumission teledeclaration
4. **Notes de frais** : Photo justificatif → OCR → categorisation → validation manager → export compta

#### Elements UI/UX remarquables

- **Espace collaboratif** expert/client : vue partagee avec droits granulaires
- **Dashboard tresorerie** avec projection et alertes
- **OCR intelligent** : extraction automatique montant, TVA, fournisseur
- **Timeline facture** : statut visuel de bout en bout

#### Avantages concurrentiels

- **Modele collaboratif** expert-comptable unique en France
- **Couverture legale francaise** native (URSSAF, TVA, liasse, facturation electronique 2026)
- **Tout-en-un** : pas besoin d'outils multiples
- **OCR + IA** pour categorisation automatique

#### Passerelles et adaptations COG

| Fonctionnalite Pennylane | Adaptation JayKonta COG |
|--------------------------|------------------------|
| Espace expert-comptable | Mandat « Expert » StrongFather + Bridge inter-COG / Facade Publique Gouvernee |
| URSSAF teledeclaration | `tool.compta.declaration.urssaf.submit` sous Mandat |
| Facturation electronique 2026 | `tool.invoice.electronic.submit` via BorderGuard conformite |
| Multi-structures | Multi-COG avec Mandats par structure |
| OCR justificatifs | `tool.expense.receipt.extract` — extraction sans decision |

---

### 3.2 Indy (ex-Georges)

**Segment :** Comptabilite autonome pour independants — France

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Synchro bancaire, categorisation assistee, ecritures |
| **Facturation** | Factures, devis, relances, modeles |
| **Declarations** | URSSAF, TVA, estimateur cotisations |
| **Rapports** | Livre des recettes, registre des achats, bilan simplifie |
| **Multi** | Micro-entreprise, EURL, SASU, SAS |

#### Parcours utilisateurs types

1. **Compta autonome** : Synchro banque → categorisation assistee → livre des recettes auto → declaration URSSAF
2. **Facturation simple** : Creer facture → envoyer → relance auto → encaissement → compta

#### Elements UI/UX remarquables

- **Interface simplifiee** : concu pour non-comptables, langage clair
- **Categorisation assistee** : suggestions avec explication du choix
- **Estimateur cotisations** : projection en temps reel selon CA

#### Avantages concurrentiels

- **Autonomie complete** sans expert-comptable
- **Prix competitif** pour independants
- **Focus micro/EURL** avec parcours dedies par regime
- **Simplicite** d'utilisation maximale

#### Passerelles et adaptations COG

| Fonctionnalite Indy | Adaptation JayKonta COG |
|---------------------|------------------------|
| Compta autonome | Service « Compta seule » : Equipe Operateurs sans expert |
| Estimateur cotisations | `tool.compta.declaration.estimate.cotisations` |
| Multi-regimes | `tool.company.structure.resolve` pour contexte regime |

---

### 3.3 Henrri

**Segment :** Facturation gratuite pour TPE/PME — France

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Facturation** | Devis, factures, avoirs, relances, modeles conformes |
| **Clients** | Fichier client, historique, encours |
| **Rapports** | Tableau de bord CA, statistiques ventes |
| **Export** | Export comptable standard (FEC), integration experts |
| **Legal** | Mentions obligatoires auto, numerotation conforme |

#### Parcours utilisateurs types

1. **Devis → facture** : Creer devis conforme → envoi → acceptation → facture auto → envoi → relance
2. **Export expert** : Facturation du mois → export FEC → envoi expert-comptable

#### Elements UI/UX remarquables

- **Conformite legale** automatique : mentions, numerotation, loi anti-fraude
- **Interface epuree** : focus sur la facturation, pas de complexite inutile

#### Avantages concurrentiels

- **Gratuit** et complet pour la facturation
- **Conformite francaise** native (mentions, FEC, anti-fraude)
- **Export FEC** pour collaboration expert

---

## 4. Concurrents Segment ERP Open Source

### 4.1 Dolibarr

**Segment :** ERP/CRM open source — Communaute active, auto-hebergeable

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Double entree, plan comptable, ecritures, banque |
| **Facturation** | Propositions commerciales, factures, avoirs, paiements en ligne |
| **Paiements** | Gestion paiements, integration passerelles |
| **Depenses** | Notes de frais, validation |
| **Agenda** | Calendrier, export iCal |
| **Stock** | Gestion stock, inventaire |
| **RH** | Gestion conges, absences |
| **Multi** | Modules activables a la carte |

#### Parcours utilisateurs types

1. **Proposition → facture → paiement** : Proposition commerciale → acceptation → facture → paiement → comptabilite
2. **Notes de frais** : Saisie → validation → ecriture comptable
3. **Agenda + facturation** : Rendez-vous → prestation → facturation

#### Elements UI/UX remarquables

- **Architecture modulaire** : activation/desactivation des modules a la carte
- **Interface classique** : navigation par menus hierarchiques
- **Widgets dashboard** personnalisables

#### Avantages concurrentiels

- **Open source** : liberte totale, auto-hebergement
- **Modulaire** : payer uniquement ce qu'on utilise
- **Communaute** : plugins et modules communautaires
- **France-friendly** : plan comptable francais, FEC, mentions legales

#### Passerelles et adaptations COG

| Fonctionnalite Dolibarr | Adaptation JayKonta COG |
|-------------------------|------------------------|
| Modulaire a la carte | Kits d'Outils activables par Mandat dans COG |
| Export calendrier | JayKoa integration pour rappels echeances |
| Auto-hebergement | Souverainete COG locale — modele naturellement aligne |
| Paiements en ligne | Passerelle paiement via BorderGuard + WorrySentinel |

---

### 4.2 ERPNext

**Segment :** ERP open source moderne — Python/Frappe

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Grand livre, ecritures, AR/AP, rapprochement |
| **Facturation** | Sales Invoice avec statut et posting, factures achats |
| **Paiements** | Payment Entry distinct de la facture, rapprochement |
| **Ecritures** | Journal Entry pour transactions hors ventes/achats |
| **Rapports** | P&L, bilan, balance agee, rapports personnalises |

#### Parcours utilisateurs types

1. **Sales Invoice → Payment** : Facture vente → posting → Payment Entry → statut paye → rapprochement
2. **Journal Entry** : Ecriture manuelle → impact grand livre → reporting

#### Elements UI/UX remarquables

- **Separation claire** facture vs paiement (Payment Entry dedié)
- **Formulaires structures** avec champs obligatoires et validation
- **Open source moderne** : interface web responsive

#### Avantages concurrentiels

- **Separation facture/paiement** : tracabilite claire
- **Open source** avec communaute active
- **Extensible** via Frappe framework

#### Passerelles et adaptations COG

| Fonctionnalite ERPNext | Adaptation JayKonta COG |
|-----------------------|------------------------|
| Payment Entry distinct | `payment.record` separe de `invoice.emit` — modele naturellement aligne |
| Journal Entry | `budget.movements.record` pour ecritures manuelles |
| Posting workflow | KindMother WriteIntent pour persistance gouvernee |

---

### 4.3 Akaunting

**Segment :** Comptabilite open source SMB — Auto-hebergeable

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Double entree, plan comptable |
| **Banque** | Bank feeds, comptes bancaires multiples |
| **Facturation** | Factures, recurrentes, avoirs |
| **Paiements** | Paiements, recurrents |
| **Portail client** | Vue factures et paiement en ligne |
| **Multi** | Multi-societe, multi-devise |
| **Rapports** | P&L, bilan, reporting |
| **Widgets** | Dashboard avec widgets configurables |

#### Elements UI/UX remarquables

- **Portail client** integre pour consultation et paiement
- **Dashboard widgets** drag-and-drop
- **Multi-societe** natif avec switch rapide

#### Avantages concurrentiels

- **Open source + auto-heberge** : souverainete des donnees
- **Portail client** inclus sans module additionnel
- **Multi-societe** natif gratuit

#### Passerelles et adaptations COG

| Fonctionnalite Akaunting | Adaptation JayKonta COG |
|-------------------------|------------------------|
| Portail client | Acces visiteur Visa pour consultation factures |
| Auto-hebergement | Souverainete COG locale aligne |
| Multi-societe | Multi-COG ou multi-contexte Account |

---

### 4.4 Tryton

**Segment :** ERP open source — Architecture stricte et modulaire

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | Module financial accounting, ecritures equilibrees obligatoires |
| **Analytique** | Module analytic accounting, axes analytiques |
| **Banque** | Module banking, import releves |
| **Integrite** | Contraintes strictes : moves equilibres, pas d'ecriture desequilibree |

#### Elements UI/UX remarquables

- **Client desktop** : interface structuree par modules
- **Rigueur comptable** : impossible de poster une ecriture desequilibree

#### Avantages concurrentiels

- **Integrite comptable** maximale — pas de compromis
- **Architecture modulaire** stricte et propre
- **Open source** pur

#### Passerelles et adaptations COG

| Fonctionnalite Tryton | Adaptation JayKonta COG |
|----------------------|------------------------|
| Ecritures equilibrees | KindMother integrity rules — WriteIntent refuse si desequilibre |
| Architecture modulaire | Pyramide Miyukini naturellement modulaire par strates |

---

## 5. Concurrents Segment ERP Enterprise

### 5.1 SAP Business One

**Segment :** ERP SMB — Branche SMB de SAP

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Comptabilite** | GL, AR, AP, ecritures journalieres |
| **Budget** | Gestion budgetaire et controlling |
| **Actifs** | Immobilisations |
| **Banque** | Rapprochement bancaire |
| **Rapports** | Reporting financier et analytique |
| **Integration** | Ventes → comptabilite, achats → comptabilite |

#### Passerelles COG

- Integration ventes → finance aligne sur JayFestival/JayRDV → JayKonta (CK-INT)
- ERP-wide workflows traduisibles en Equipes d'Operateurs sous Contrat

---

### 5.2 Microsoft Dynamics 365 Finance / Business Central

**Segment :** Enterprise (Finance) / SMB (Business Central)

#### Fonctionnalites extraites

| Domaine | Finance | Business Central |
|---------|---------|-----------------|
| **Comptabilite** | GL, financial close, consolidation | GL, ecritures, posting |
| **Budget** | Budget proposals, previsionnel | Budget basique |
| **Facturation** | Quote-to-cash pipeline | Sales invoices, purchase invoices |
| **Banque** | Cash management | Import releves, rapprochement |
| **Multi** | Multi-entite, multi-devise | Multi-societe |
| **Taxes** | Tax management avance | TVA basique |

#### Parcours utilisateurs types

1. **Finance** : Budget planning → previsionnel → close financier → reporting consolidation
2. **Business Central** : Devis vente → facture vente → posting → grand livre → rapprochement bancaire

#### Elements UI/UX remarquables

- **Dynamics 365 Finance** : dashboards analytiques, previsionnel IA
- **Business Central** : formulaires structures devis → facture → posting

#### Avantages concurrentiels

- **Scalabilite enterprise** : du SMB au grand groupe
- **Integration Microsoft** : Excel, Power BI, Teams
- **Close automation** : processus de cloture structure

#### Passerelles COG

| Fonctionnalite Dynamics | Adaptation JayKonta COG |
|------------------------|------------------------|
| Quote-to-cash | `quote.create` → `invoice.emit` → `payment.record` |
| Close process | Politique freeze KindMother + audit trail |
| Posting | WriteIntent KindMother pour toute persistance |

---

## 6. Concurrents Segment Budget Personnel

### 6.1 YNAB (You Need A Budget)

**Segment :** Budget personnel par methode enveloppes — Reference mondiale

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Budget** | Categories avec enveloppes, assignation de chaque euro a une categorie |
| **Objectifs** | Targets par categorie : epargne, depense mensuelle, dette |
| **Sync** | Synchronisation bancaire, saisie manuelle, multi-appareils |
| **Shared** | Budgets partages (couple, famille) |
| **Templates** | Modeles de categories pre-configures |
| **Reporting** | Rapports depenses, revenus, tendances |
| **Rollover** | Report des non-depenses au mois suivant |

#### Parcours utilisateurs types

1. **Setup** : Creer categories → definir targets → assigner revenus → suivre
2. **Quotidien** : Transaction → categoriser → verifier budget restant → ajuster si depassement
3. **Partage** : Inviter partenaire → budget commun → visibilite partagee

#### Elements UI/UX remarquables

- **Barres de progression** par categorie/objectif — feedback visuel immediat
- **Assignation** : chaque euro recu doit etre assigne a une categorie (methode Zero-Based)
- **Templates categories** : configurations pre-faites par profil de vie
- **Mobile** : application complete avec saisie rapide

#### Avantages concurrentiels

- **Methode pedagogique** : YNAB enseigne la gestion budgetaire
- **Zero-Based Budgeting** : approche unique et disciplinee
- **Communaute** active et contenu educatif
- **Rollover intelligent** : flexibilite entre mois

#### Passerelles et adaptations COG

| Fonctionnalite YNAB | Adaptation JayKonta COG |
|--------------------|------------------------|
| Categories budgetaires | PUR-05 (categories) et PUR-08 (budgets occasionnels) |
| Targets/objectifs | PUR-11 (objectifs) et PUR-12 (progression) |
| Budget partage | Mandats de Permission StrongFather pour acces partage au Purse |
| Rollover | Logique report MiyuComptaLedger par periode |
| Templates | Categories pre-configurees par contexte dans KindMother |

---

### 6.2 Monarch Money

**Segment :** Finance personnelle tout-en-un — Agregation + budget

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Agregation** | Connexion multi-comptes bancaires, vue patrimoine nette |
| **Budget** | Budget flexible ou par categories, rollover |
| **Previsionnel** | Projection cash flow, scenarios |
| **Abonnements** | Detection automatique abonnements recurrents |
| **Objectifs** | Suivi objectifs d'epargne |
| **Rapports** | Graphiques depenses, revenus, tendances, patrimoine |

#### Parcours utilisateurs types

1. **Setup** : Connecter comptes → vue patrimoine net instantanee → configurer budget
2. **Quotidien** : Transactions auto-categorisees → progression budget → alertes depassement
3. **Abonnements** : Detection automatique → revue → annulation/ajustement

#### Elements UI/UX remarquables

- **Dashboard patrimoine net** : vue consolidee de tous les comptes
- **Barres progression budget** avec couleurs intuitive
- **Detection abonnements** : liste automatique avec cout mensuel/annuel
- **Graphiques tendances** : evolution temporelle claire

#### Avantages concurrentiels

- **Vue holistique** des finances (agregation complete)
- **Flexibilite budget** : pas impose une methode unique
- **Detection abonnements** automatique et pratique
- **Interface moderne** et soignee

#### Passerelles et adaptations COG

| Fonctionnalite Monarch | Adaptation JayKonta COG |
|-----------------------|------------------------|
| Agregation multi-comptes | Multi-source read avec permissions strictes WorrySentinel |
| Detection abonnements | Pattern detection dans MiyuComptaLedger (recurrence) |
| Vue patrimoine | PUR-06 (solde et synthese) etendu multi-contexte |

---

### 6.3 PocketGuard

**Segment :** Budget personnel simplifie — « Combien puis-je depenser ? »

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Budget** | Budget mensuel et annuel personnalise, categories personnalisables |
| **Progression** | Suivi en temps reel, notifications |
| **Recurrences** | Planning depenses recurrentes |
| **Scenarios** | Test de scenarios budgetaires |
| **Connexion** | Comptes bancaires et cartes |

#### Elements UI/UX remarquables

- **« In My Pocket »** : indicateur central de combien il reste a depenser
- **Categories visuelles** avec codes couleur
- **Notifications** proactives en cas de depassement

#### Avantages concurrentiels

- **Simplicite extreme** : reponse a une seule question
- **Budget personnalisable** en profondeur
- **Scenarios** pour planifier

---

### 6.4 Quicken Simplifi

**Segment :** Finance personnelle — Heritage Quicken, version cloud moderne

#### Fonctionnalites extraites

| Domaine | Fonctionnalites |
|---------|-----------------|
| **Spending Plan** | Plan de depenses vs revenus, suivi temps reel |
| **Previsionnel** | Projection flux de tresorerie |
| **Objectifs** | Objectifs d'epargne avec suivi |
| **Rapports** | Insights et rapports personnalises |

#### Elements UI/UX remarquables

- **Spending Plan** visuel avec barre de progression
- **Cash flow projection** : graphique temporel des flux futurs

#### Passerelles COG

- Projection tresorerie aligne avec `tool.treasury.forecast.compute` pour Purse
- Spending Plan similaire aux alertes budget PUR-15

---

## 7. Tableaux Comparatifs par Domaine Fonctionnel

### 7.1 Comptabilite Coeur (GL, Journal, Ecritures)

| Concurrent | GL | Journal | Multi-devise | Multi-entite | Analytique |
|-----------|-----|---------|-------------|-------------|------------|
| **Odoo** | Oui | Oui | Oui | Oui | Multi-axes |
| **QuickBooks** | Oui | Oui | Oui (plans+) | Non natif | Categories |
| **Xero** | Oui | Oui | Oui | Via Xero HQ | Tracking |
| **Sage Intacct** | Oui | Oui | Oui | Oui (fort) | Dimensions |
| **FreshBooks** | Oui | Oui | Oui | Non | Projet |
| **Zoho Books** | Oui | Oui | Oui | Non | Non |
| **ERPNext** | Oui | Oui | Oui | Oui | Cost Centers |
| **Dolibarr** | Oui | Oui | Partiel | Non natif | Non |
| **Akaunting** | Oui | Oui | Oui | Oui | Non |
| **Tryton** | Oui | Oui (strict) | Oui | Oui | Oui |
| **JayKonta** | Oui (MAC-04/06) | Oui (MiyuComptaLedger) | Futur | Multi-COG | MAC-05 (ventilation) |

### 7.2 Cycle Devis → Facture → Paiement

| Concurrent | Devis | Conversion auto | Facture | Recurrence | Relances | Paiement ligne | Portail client |
|-----------|-------|----------------|---------|-----------|----------|---------------|---------------|
| **Odoo** | Oui | Oui | Oui | Oui | Oui (niveaux) | Oui | Oui |
| **QuickBooks** | Oui | Oui | Oui | Oui | Oui | Oui | Non |
| **Xero** | Oui | Oui (1 clic) | Oui | Oui | Oui | Oui | Non |
| **Sage** | Oui | Oui | Oui | Oui | Oui (auto) | Oui | Non |
| **FreshBooks** | Oui (e-sign) | Oui | Oui | Oui | Oui | Oui | Non |
| **Zoho Books** | Oui | Oui | Oui | Oui | Oui | Oui | Non |
| **Wave** | Non | Non | Oui | Oui | Oui | Oui | Non |
| **Pennylane** | Oui | Oui | Oui | Oui | Oui | Partiel | Non |
| **Indy** | Oui | Oui | Oui | Non | Oui | Non | Non |
| **Henrri** | Oui | Oui | Oui | Non | Oui | Non | Non |
| **ERPNext** | Oui | Oui | Oui | Oui | Partiel | Non natif | Non |
| **Dolibarr** | Oui | Oui | Oui | Non | Partiel | Oui | Non |
| **Akaunting** | Oui | Partiel | Oui | Oui | Partiel | Oui | Oui |
| **JayKonta** | MAC-07/08 | MAC-09 (CK-OP-12) | MAC-11 | Via MiyuBilling | MAC-12 | Futur (Visa) | Futur (Visa) |

### 7.3 Banque et Rapprochement

| Concurrent | Sync bancaire | Import releves | Rapprochement auto | Regles categorisation |
|-----------|--------------|---------------|-------------------|-----------------------|
| **Odoo** | Oui (Open Banking) | Oui | Oui (IA) | Oui |
| **QuickBooks** | Oui | Oui | Oui (IA) | Oui (learning) |
| **Xero** | Oui (feeds) | Oui | Oui (suggestions) | Oui |
| **Sage** | Oui | Oui | Oui | Partiel |
| **Zoho Books** | Oui | Oui | Oui | Oui (rules) |
| **FreshBooks** | Oui | Oui | Oui | Partiel |
| **Pennylane** | Oui | Oui | Oui | Oui (IA) |
| **Indy** | Oui | Non | Oui (assisté) | Oui |
| **ERPNext** | Non natif | Oui | Oui | Non |
| **Dolibarr** | Non natif | Oui | Non | Non |
| **Akaunting** | Oui | Oui | Partiel | Non |
| **JayKonta** | Futur (tool.compta.bank.sync) | Futur | Futur (suggest+validate) | Futur (rules KindMother) |

### 7.4 Depenses et Justificatifs

| Concurrent | Scan OCR | Categorisation | Depenses refacturables | Notes de frais | IK |
|-----------|---------|---------------|----------------------|---------------|-----|
| **Odoo** | Oui | Oui | Oui | Oui | Non natif |
| **QuickBooks** | Oui (mobile) | Oui (IA) | Oui | Partiel | Non |
| **Xero** | Via add-ons | Oui | Oui | Via add-ons | Non |
| **FreshBooks** | Oui | Oui | Oui | Partiel | Non |
| **Zoho Books** | Via Zoho Expense | Oui | Oui | Oui (integration) | Non |
| **Pennylane** | Oui (IA) | Oui | Non | Oui | Non |
| **Indy** | Non | Oui | Non | Non | Non |
| **JayKonta** | MiyuExpense (receipt.extract) | MiyuExpense | Futur (flag refacturable) | MiyuExpense (PUR-13) | MiyuExpense (mileage) |

### 7.5 Reporting

| Concurrent | P&L | Bilan | Cash Flow | Balance agee | Budget | Export PDF/CSV |
|-----------|-----|-------|-----------|-------------|--------|---------------|
| **Odoo** | Oui | Oui | Oui | Oui | Oui | Oui |
| **QuickBooks** | Oui | Oui | Oui | Oui | Oui (plans+) | Oui |
| **Xero** | Oui | Oui | Oui | Oui | Oui | Oui |
| **Sage Intacct** | Oui | Oui | Oui | Oui | Oui (fort) | Oui |
| **FreshBooks** | Oui | Oui | Non natif | Oui | Non | Oui |
| **Zoho Books** | Oui | Oui | Oui | Oui | Oui | Oui |
| **Pennylane** | Oui | Oui | Oui | Non | Oui | Oui |
| **ERPNext** | Oui | Oui | Oui | Oui | Oui | Oui |
| **Dolibarr** | Partiel | Partiel | Non | Non | Non | Oui |
| **Akaunting** | Oui | Oui | Oui | Non | Non | Oui |
| **JayKonta** | MAC-16 (MiyuComptaReports) | MAC-16 | Futur (MiyuTreasury) | Futur | MAC-18/19 | MAC-17 (PDF/CSV) |

### 7.6 Budget Personnel (vs JayBudget Purse)

| Concurrent | Categories | Objectifs | Rollover | Alertes | Partage | Mobile | Previsionnel |
|-----------|-----------|----------|---------|---------|---------|--------|-------------|
| **YNAB** | Oui (enveloppes) | Oui (targets) | Oui | Oui | Oui | Oui | Non |
| **Monarch** | Oui | Oui | Oui | Oui | Non | Oui | Oui |
| **PocketGuard** | Oui | Non | Non | Oui | Partiel | Oui | Scenarios |
| **Quicken Simplifi** | Oui | Oui | Non | Oui | Non | Oui | Oui |
| **JayBudget** | PUR-05 | PUR-11/12 | Futur | PUR-15 | Futur (Mandat) | Futur | Futur (MiyuTreasury) |

---

## 8. Parcours Utilisateurs Compares — Best Practices

### 8.1 Parcours Onboarding

| Etape | Best Practice Marche | Adaptation JayKonta |
|-------|---------------------|---------------------|
| **1. Inscription** | Email + verification (QuickBooks, FreshBooks) ou SSO (Xero, Zoho) | Creation compte via MiyuAuth (CK-SVC-01, CK-TK-01) + contexte Purse ou Account |
| **2. Profil entreprise** | Assistant guide avec champs cles : nom, SIRET, regime fiscal (Pennylane, Indy) | `tool.company.structure.resolve` + `tool.company.structure.register` sous StrongFather |
| **3. Connexion banque** | Sync automatique immediate (QuickBooks, Pennylane) | `tool.compta.bank.sync` avec WorrySentinel niveau 2+ et consentement explicite |
| **4. Import donnees** | Import depuis ancien logiciel (Xero, Odoo) | Import CSV/FEC gouverne par KindMother avec audit |
| **5. Premiere facture** | Assistant creation avec preview (FreshBooks, Wave) | `invoice.emit` avec modele pre-configure |

### 8.2 Parcours Facturation Quotidien

| Etape | Best Practice Marche | Adaptation JayKonta |
|-------|---------------------|---------------------|
| **1. Creer devis** | Modeles avec auto-completion client (Odoo, Xero) | `quote.create` avec references client KindMother |
| **2. Envoyer** | Email direct avec preview integre (FreshBooks) | Envoi gouverne avec log BondingBrother |
| **3. Suivi** | Timeline statut : cree → vu → accepte (QuickBooks) | Evenements lifecycle JayKonta |
| **4. Convertir** | 1 clic devis → facture (Xero) | `quote.to_invoice` via CK-OP-12 |
| **5. Encaisser** | Paiement en ligne + reconciliation auto (Odoo) | `payment.record` + rapprochement gouverne |
| **6. Relancer** | Niveaux de relance configurables (Odoo, Sage) | Notifications MiyuNotify sous Mandat |

### 8.3 Parcours Budget Personnel

| Etape | Best Practice Marche | Adaptation JayBudget |
|-------|---------------------|---------------------|
| **1. Creer budget** | Categories pre-configurees (YNAB templates) | PUR-05 categories + PUR-08 budgets occasionnels |
| **2. Saisir depense** | Saisie rapide en < 5s (PocketGuard) | PUR-04 avec NFR-PUR-05 (< 2s) et NFR-PUR-06 (3 actions max) |
| **3. Suivre** | Barres progression colorees (YNAB, Monarch) | PUR-06 solde + PUR-12 progression objectifs |
| **4. Alerter** | Notification au seuil (PocketGuard) | PUR-15 alertes configurables |
| **5. Ajuster** | Rollover et transfert entre categories (YNAB) | Logique report dans MiyuComptaLedger |

---

## 9. Elements UI/UX — Best Practices du Marche

### 9.1 Patterns d'Interface Recurrents

| Pattern | Utilise par | Description | Priorite JayKonta |
|---------|-----------|-------------|-------------------|
| **Dashboard synthetique** | Tous | Indicateurs cles (CA, depenses, solde, factures impayees) en une vue | P0 — MAC-15 |
| **Timeline facture** | QuickBooks, FreshBooks | Statut visuel de bout en bout du cycle de vie facture | P1 — lifecycle visuel |
| **Barres progression** | YNAB, Monarch, PocketGuard | Progression categorie/objectif avec couleur intuitive | P0 — PUR-12 |
| **Rapprochement split-view** | Xero, Odoo | Transaction a gauche, suggestion a droite, validation centre | P2 — rapprochement |
| **Scan justificatif** | QuickBooks, Pennylane | Photo → OCR → association en < 10s | P1 — MiyuExpense |
| **Portail client** | Odoo, Akaunting | Vue externe read-only des factures avec paiement en ligne | P2 — acces Visa |
| **Kanban factures** | Odoo | Colonnes par statut (brouillon, envoyee, payee, retard) | P1 — vue Account |
| **Toggle rapide** | Akaunting, QuickBooks | Switch entreprise/contexte en 1 clic | P1 — switch Purse/Account |

### 9.2 Principes UX Extraits

| Principe | Justification | Exemple concurrent |
|----------|--------------|-------------------|
| **Zero-config pour debut** | Reduire friction onboarding | Wave : gratuit + fonctionnel des l'inscription |
| **Progressive disclosure** | Ne montrer que le necessaire | Indy : interface simplifiee, complexite cachee |
| **Feedback immediat** | Toute action = retour visuel | YNAB : barre progresse immediatement apres saisie |
| **Mobile-first saisie** | Les depenses se font en mobilite | QuickBooks : snap receipt + categorise en 5s |
| **Automation visible** | Montrer ce qui est automatise et pourquoi | Pennylane : explication de la categorisation IA |
| **Export accessible** | PDF/CSV toujours a 1 clic max | Tous : export omni-present |

---

## 10. Avantages Differenciants JayKonta vs Concurrence

### 10.1 Avantages architecturaux uniques

| Avantage JayKonta/COG | Description | Aucun concurrent ne propose |
|-----------------------|-------------|---------------------------|
| **Souverainete des donnees** | Donnees financieres residant dans un COG souverain, pas sur un cloud tiers | Seuls les open-source auto-heberges s'en approchent, sans gouvernance |
| **Separation decision/execution** | StrongFather decide, les Outils executent — jamais l'inverse | Aucun concurrent n'a de Core decideur distinct |
| **Gouvernance multi-niveau** | WorrySentinel (securite) + MasterButler (permissions) + StrongFather (decisions) | Les concurrents ont des roles basiques (admin/user) |
| **Audit structurel** | KindMother WriteIntent = tout est trace par design, pas par option | Les concurrents ajoutent l'audit en surcouche |
| **Multi-echelle unifie** | Budget perso (Purse) et comptabilite entreprise (Account) dans le meme service | Aucun concurrent ne couvre les deux segments |
| **Integration gouvernee** | JayFestival, JayRDV consomment JayKonta via contrats — pas d'API sauvage | Les ERP ont des modules couples, pas des contrats gouvernes |
| **Offline-first** | Fonctionne sans connexion internet | Quasi aucun concurrent web ne le propose |
| **Federation inter-COG** | Collaboration expert-comptable via Bridge, pas via partage cloud | Pennylane s'en approche mais reste centralise |

### 10.2 Avantages fonctionnels

| Avantage | Description |
|----------|-------------|
| **Deux marques, un service** | JayBudget pour le perso, JayKonta pour l'entreprise — meme base, UX adaptee |
| **7 Toolkits specialises** | MiyuComptaLedger, MiyuComptaReports, MiyuDeclarations, MiyuBilling, MiyuInvoice, MiyuExpense, MiyuTreasury |
| **Integration native** | JayFestival (evenements), JayRDV (rendez-vous), JayKoa (calendrier) — ecosysteme coherent |
| **Mandats granulaires** | Permissions au niveau de chaque action, pas des roles globaux |
| **Residence configurable** | Purse : local ou COG ; Account : centralise obligatoire — choix explicite |

### 10.3 Risques et faiblesses identifiees

| Risque | Description | Mitigation |
|--------|-------------|------------|
| **Maturite** | JayKonta est en conception, les concurrents sont en production | Focus MVP sur P0 (MAC-01 a MAC-06, MAC-11 a MAC-13) |
| **Ecosysteme integrations** | QuickBooks a 750+ apps, Xero 1000+ | Construire d'abord l'ecosysteme interne (Jay services) |
| **Sync bancaire** | Open Banking complexe, les concurrents ont des annees d'experience | Prioriser import CSV/FEC avant sync temps reel |
| **Mobile** | Pas d'app mobile native au lancement | PWA via MiyuWeb ou Miyukini Central mobile |
| **IA/OCR** | Les concurrents investissent massivement dans l'IA | OCR via outil gouverne, pas d'IA autonome non gouvernee |
| **Complexite perçue** | Le modele COG (Cores, Mandats, Strates) peut intimider | UX Purse ultra-simplifiee, complexite cachee pour l'utilisateur |

---

## 11. Passerelles et Adaptations COG — Synthese

### 11.1 Matrice Fonctionnalite → Adaptation COG

| Fonctionnalite Marche | Operateur/Kit JayKonta | Core(s) implique(s) | Adaptation COG |
|----------------------|----------------------|--------------------|-----------------------|
| Portail client / paiement en ligne | MiyuInvoice + BorderGuard | WorrySentinel, StrongFather | Acces visiteur via Visa, scope lecture facture + paiement |
| Synchronisation bancaire | MiyuComptaLedger | WorrySentinel (niveau 2+), KindMother | Audit complet, consentement explicite, residence donnees |
| Rapprochement IA/suggestions | MiyuComptaLedger | StrongFather (validation) | Suggestion sans decision auto — humain valide via TAMR si necessaire |
| Multi-societe / multi-entite | Multi-COG | BorderGuard, StrongFather | Chaque entite = COG souverain, liaison par contrats inter-COG |
| Collaboration expert-comptable | Bridge inter-COG | StrongFather (Mandat Expert), BorderGuard | Facade Publique Gouvernee, export securise, pas d'execution dans le COG |
| Notes de frais OCR | MiyuExpense | KindMother (persistance), StrongFather (validation) | OCR extraction seule, pas de decision — validation humaine |
| Facturation electronique 2026 | MiyuInvoice | BorderGuard (conformite), EverBuddy (versions) | Soumission plateforme agreee via outil gouverne |
| Declarations URSSAF/TVA | MiyuDeclarations | StrongFather (autorisation soumission) | Preparation → validation explicite → soumission gouvernee |
| Budget categories et objectifs | Purse (MiyuComptaLedger) | KindMother (donnees), StrongFather (seuils) | Categories et objectifs persistes dans COG souverain |
| Previsionnel tresorerie | MiyuTreasury | KindMother (donnees source) | Calcul projection sans decision, affichage informatif |
| Factures recurrentes | MiyuBilling | StrongFather (Mandat recurrence) | Mandat permanent avec scope et duree controles |
| Workflow approbation | MasterButler + StrongFather | StrongFather (Mandat), MasterButler (permissions) | Mandats de Permission par role et action |

### 11.2 Regles d'adaptation non-negociables

| Regle | Description |
|-------|-------------|
| **COG-ADAPT-01** | Tout acces externe (portail client, expert, API) passe par Visa et BorderGuard — jamais d'acces direct |
| **COG-ADAPT-02** | Tout write intent (ecriture facture, mouvement, declaration) passe par KindMother avec audit |
| **COG-ADAPT-03** | Toute decision (validation, envoi, export sensible) est autorisee par StrongFather via Mandat |
| **COG-ADAPT-04** | Les donnees financieres (niveau 2+) respectent la residence definie par le contrat de service |
| **COG-ADAPT-05** | Aucune IA autonome ne prend de decision — suggestion uniquement, validation humaine (TAMR) |
| **COG-ADAPT-06** | La complexite COG est cachee a l'utilisateur final — l'UX reste simple |
| **COG-ADAPT-07** | Le multi-entite est multi-COG avec contrats explicites, pas de partage implicite de donnees |

---

## 12. Recommandations pour la Roadmap JayKonta

### 12.1 Quick Wins — Fonctionnalites communes a tous les concurrents

| Priorite | Fonctionnalite | Justification |
|----------|---------------|---------------|
| **P0** | Dashboard financier synthetique | Present chez 100% des concurrents — attendu minimum |
| **P0** | Cycle devis → facture → paiement | Coeur de toute solution comptable |
| **P0** | Export PDF/CSV | Basique, universel, requis pour expert-comptable |
| **P0** | Saisie mouvements rapide (Purse) | < 2s, 3 actions max — standard YNAB/PocketGuard |
| **P1** | Relances impayes automatiques | Fort differenciateur chez Odoo, Sage, FreshBooks |
| **P1** | Scan justificatifs OCR | Mobile-first, QuickBooks et Pennylane en reference |
| **P1** | Templates factures brandees | Standard industriel, FreshBooks et Wave en reference |

### 12.2 Differenciateurs a construire

| Priorite | Differenciateur | Valeur ajoutee |
|----------|----------------|----------------|
| **P0** | Double entree Purse/Account | Unique sur le marche — perso et pro unifies |
| **P1** | Integration JayFestival/JayRDV | Ecosysteme interne coherent |
| **P1** | Offline-first avec sync | Souverainete locale impossible chez les SaaS |
| **P2** | Portail client Visa | Acces gouverne sans compte COG |
| **P2** | Collaboration expert via Bridge | Alternative a Pennylane avec souverainete |
| **P3** | Federation inter-COG | Consolidation multi-entites souveraines |

### 12.3 Ecarts critiques a combler

| Ecart | Concurrents de reference | Plan |
|-------|------------------------|------|
| **Sync bancaire** | Odoo, QuickBooks, Pennylane | Phase 2 : import CSV/FEC d'abord, Open Banking ensuite |
| **Mobile natif** | QuickBooks, Xero, FreshBooks | PWA via MiyuWeb (MVP), Miyukini Central mobile (futur) |
| **IA categorisation** | QuickBooks, Pennylane | Suggestions gouvernees (tool.compta.transaction.categorize) |
| **Facturation electronique** | Pennylane, Sage | Phase 3 : conformite 2026 avec tool.invoice.electronic.submit |

---

## 13. Conclusion

L'analyse de **19 concurrents** couvrant les segments ERP (Odoo, Sage Intacct, SAP, Dynamics), SMB (QuickBooks, Xero, FreshBooks, Zoho Books, Wave), France (Pennylane, Indy, Henrri), Open Source (Dolibarr, ERPNext, Akaunting, Tryton) et Budget Personnel (YNAB, Monarch, PocketGuard, Quicken Simplifi) revele :

1. **Le marche est mature** : les fonctionnalites de base (facturation, GL, rapprochement, reporting) sont un prerequis, pas un differenciateur
2. **L'automatisation domine** : sync bancaire, categorisation IA, relances automatiques sont attendues
3. **Le mobile est indispensable** : facturation mobile et scan justificatifs sont des standards
4. **Le multi-echelle est un vide** : aucun concurrent ne couvre budget personnel + comptabilite entreprise dans un meme service
5. **La souverainete est absente** : aucun concurrent web n'offre de gouvernance des donnees comparable au modele COG

**JayKonta se differencie par** :
- La **double entree** Purse/Account (unique sur le marche)
- La **souverainete COG** avec gouvernance multi-niveau
- L'**integration ecosysteme** native (JayFestival, JayRDV, JayKoa)
- Le modele **offline-first** avec sync
- La **federation inter-COG** pour collaboration expert et multi-entite

La priorite est de livrer un **MVP solide** (P0 : dashboard, cycle devis→facture→paiement, export, saisie Purse) avant de construire les differenciateurs (P1-P3).

---

## References

| Document | Role |
|----------|------|
| [JayKonta - Document Fondateur](./JayKonta%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, securite |
| [Account - Analyse des besoins](./publics/Account/Account%20-%20Analyse%20des%20besoins.md) | Besoins MAC-01 a MAC-19 |
| [Purse - Analyse des besoins](./publics/Purse/Purse%20-%20Analyse%20des%20besoins.md) | Besoins PUR-01 a PUR-16 |
| [JayKonta - Integration Services](./reference/JayKonta%20-%20Integration%20Services.md) | Integration JayFestival, JayRDV |
| [JayKonta - Niveaux Securite](./reference/JayKonta%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Classification donnees et mesures |
| [Equivalents Comptabilite Independants](../../reference/equivalents/Miyukini%20Conceptual%20References%20-%20Equivalents%20Comptabilite%20Independants.md) | Cartographie detaillee Toolkits/Tools compta |
| [Miyukini Architecture Skill](.claude/skills/miyukini-architecture/) | Architecture pyramidale et Cores |
| [Miyukini Glossary Skill](.claude/skills/miyukini-glossary/) | Terminologie officielle |

---

**Document** : JayKonta — Analyse PR Concurrence Web
**Version** : 2.0
**Date** : 2026-02-07
**Statut** : Document d'analyse PR — reference concurrentielle pour le service JayKonta
