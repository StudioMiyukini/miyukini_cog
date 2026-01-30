# Miyukini Conceptual References — Équivalents Comptabilité Indépendants

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités des **logiciels de comptabilité pour indépendants** (auto-entrepreneurs, freelances, TPE) tels que **Indy**, **Pennylane**, **Abby**, **Tiime**, **Shine** et solutions génériques. Il vise à permettre la création d'**outils**, **opérateurs** et **services** Miyukini pour proposer des **services comptables spécialisés** :

- **Facturation** (devis, factures, acomptes, relances, facturation électronique B2B 2026)
- **Comptabilité** (synchronisation bancaire, écritures, rapprochement, TVA, livre des recettes, bilan, liasse fiscale)
- **Déclarations** (URSSAF, TVA, échéances fiscales et sociales)
- **Notes de frais et indemnités** (justificatifs, OCR, indemnités kilométriques, validation)
- **Trésorerie** (tableau de bord, prévisionnel, créances/dettes, alertes)
- **Multi-structures** (micro-entreprise, EURL, SARL, SASU, SAS)
- **Collaboration expert-comptable** (partage sécurisé, clôture, export)

Il **s'appuie sur** la documentation conceptuelle existante : [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md).

---

## Fondements conceptuels (alignement documentation existante)

Ce document applique les **définitions canoniques** et **règles** des références listées ci-dessus. Les équivalents Comptabilité Indépendants respectent en particulier :

### Outils (Tools) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.
- **Règle :** *« Un Outil fait, mais ne décide jamais. »* Les Tools compta (ex. `tool.compta.bank.sync`, `tool.compta.declaration.urssaf.submit`) exécutent des actions ; la décision (autoriser une déclaration, valider une note de frais, etc.) appartient à **StrongFather**.

### Kits d'Outils (Toolkits) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience et cohérence.
- **Règle :** *« Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants. »* Les Toolkits compta (`toolkit.compta.ledger`, `toolkit.compta.declarations`, etc.) agrègent des Tools existants sans logique métier propre.

### Opérateurs (Operators) — [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)

- **Définition :** Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.
- Les Opérateurs compta (Facturation, Comptabilité, Déclarations, Notes de frais, Trésorerie) sont des **Opérateurs de Domaine** ou **d'Interface** (Strate 7) ; ils n'ont pas d'autorité propre et passent par la gouvernance pour toute action.

### Service vs Opérateur — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- **Service** = capacité perçue par l'utilisateur. **Opérateur** = unité d'exécution gouvernée.
- **Règle :** *« Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs. »* Les services « compta seule » et « compta + expert » sont donc livrés par un ou plusieurs Opérateurs sous **Contrat d'équipe** et **Mandat de Permission**.

### Collaboration entre Opérateurs — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- *« Aucun Opérateur ne parle librement à un autre. »* Toute communication Facturation ↔ Comptabilité, Comptabilité ↔ Déclarations, ou avec un expert-comptable externe passe par **BondingBrother**, est définie dans le **Contrat d'équipe** et autorisée par un **Mandat de Permission** émis par StrongFather.
- *« Le contrat est validé UNE FOIS, pas à chaque appel. »* Le Contrat d'équipe est statique (conception) ; le Mandat de Permission encadre l'exécution.

### Pyramide et COG — [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)

- **Strate 6** : Tools & Toolkits compta (capacités gouvernées).
- **Strate 7** : Opérateurs compta (Facturation, Comptabilité, Déclarations, Notes de frais, Trésorerie).
- **COG** : environnement de gouvernance orchestré par des Cores ; Miyukini n'est pas un OS, c'est le « cog » qui fait fonctionner les systèmes ensemble.

### Données et écriture — [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

- **KindMother** : autorité sur toutes les données (factures, écritures, justificatifs, déclarations, paramètres société, barèmes). Toute écriture passe par **WriteIntent** sous autorité KindMother.
- **StrongFather** : décision ALLOW/DENY (validation note de frais, envoi déclaration, export sensible, partage expert-comptable). N'exécute jamais.

---

## Portée / Scope

**Ce document définit :**

- La cartographie détaillée **logiciel comptabilité indépendants** → Outils, Opérateurs, Services Miyukini
- Les **Kits d'outils (Toolkits)** et **Outils (Tools)** à créer ou à réutiliser, conformes aux définitions de [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- Les **Opérateurs** (Domaine, Interface) à déployer, conformes à [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- Les **Services** perçus par l'utilisateur et les **Équipes d'Opérateurs** / **Contrats d'équipe** / **Mandats de Permission** pour « compta seule » vs « compta + expert », conformes à [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)
- Les **Cores** impliqués et les flux de gouvernance (Glossaire)
- La relation avec **MiyuBilling** (facturation SaaS) et les patterns multi-structures

**Hors scope :**

- L'implémentation technique détaillée (code, schémas DB)
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools)
- La stratégie commerciale ou marketing des services
- Les règles fiscales ou sociales spécifiques par pays (le référentiel reste conceptuel)

**Statut :** Document de référence normatif — source de vérité pour la conception des services Comptabilité Indépendants Miyukini.

---

## 1. Périmètre cible et objectifs

### 1.1 Équivalents logiciels ciblés

| Équivalent | Rôle | Objectif Miyukini |
|------------|------|--------------------|
| **Indy** | Comptabilité autonome indépendants, déclarations, facturation | Opérateurs Facturation, Comptabilité, Déclarations + Toolkits compta.ledger, compta.declarations, invoice.standalone |
| **Pennylane** | Tout-en-un compta + expert-comptable, trésorerie, multi-structures | Même modèle COG ; Équipe d'Opérateurs + collaboration expert (Bridge / Mandat) |
| **Abby** | Facturation gratuite, déclarations URSSAF/TVA, compta | Toolkits invoice.standalone, compta.declarations ; réutilisation MiyuBilling si facturation SaaS croisée |
| **Tiime / Shine / Qonto** | Compte pro + facturation + compta + intégrations | Opérateurs Facturation, Comptabilité, Trésorerie ; intégrations = BondingBrother, API gouvernée |
| **Logiciel comptabilité indépendant générique** | Facturation, écritures, déclarations, notes de frais, rapports | Toolkits atomiques + Opérateurs gouvernés |

### 1.2 Services utilisateur visés

| Service | Description | Opérateurs / Tools principaux |
|--------|-------------|-------------------------------|
| **Comptabilité autonome (indépendant)** | Facturation, écritures, déclarations URSSAF/TVA, livre des recettes, rappels échéances | Opérateurs Facturation, Comptabilité, Déclarations ; Toolkits invoice.standalone, compta.ledger, compta.declarations |
| **Notes de frais et indemnités** | Saisie justificatifs, OCR, indemnités kilométriques, validation | Opérateur Notes de frais, Toolkit expense.claims |
| **Trésorerie et prévisionnel** | Tableau de bord, prévisionnel, créances/dettes, alertes | Opérateur Trésorerie, Toolkit treasury.forecast |
| **Compta + expert-comptable** | Partage sécurisé des données, clôture, export, collaboration | Équipe d'Opérateurs + Mandat ; BondingBrother / Bridge inter-COG pour expert externe |
| **Multi-structures** | Plusieurs sociétés (micro, EURL, SASU, etc.) dans un même environnement | KindMother (périmètre structure) + tool.company.structure.resolve ; Mandats par structure |

---

## 2. Cartographie Facturation (indépendants) → Miyukini COG

### 2.1 Fonctionnalités facturation et équivalents

| Fonctionnalité marché | Équivalent Miyukini | Type | Détail |
|------------------------|----------------------|------|--------|
| **Devis** | Tools devis | Tools | `tool.invoice.quote.create`, `tool.invoice.quote.update`, `tool.invoice.quote.to_invoice` ; persistance = KindMother. |
| **Factures (création, modèles)** | Tools facture + MiyuBilling | Tools / Toolkit | `tool.billing.invoice.generate` (MiyuBilling) ou `tool.invoice.create` (facturation métier indépendant) ; modèles = données KindMother. |
| **Factures d'acompte** | Tool facture | Tool | `tool.invoice.create` avec type « acompte » ; règles = StrongFather. |
| **Relances impayés** | Tool relance | Tool | `tool.invoice.reminder.send` ; autorisation = StrongFather ; contenu fourni dans le flux. |
| **Facturation électronique B2B (2026)** | Tool + conformité | Tool / Core | `tool.invoice.electronic.submit` (plateforme agréée) ; règles conformité = Ever Buddy, Border Guard. |
| **Envoi facture (email, lien)** | Tools envoi | Tools | `tool.invoice.send`, `tool.invoice.payment.link.generate` ; décision envoi = StrongFather. |
| **Gestion clients (facturation)** | Données KindMother + Tools | Core + Tools | `tool.crm.customer.*` si partagé avec CRM ; ou `tool.invoice.customer.resolve`, `tool.invoice.customer.list` dans toolkit invoice.standalone. |

### 2.2 Outils facturation indépendants (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.invoice.quote.create` | Crée un devis à partir de données fournies | 1–2 |
| `tool.invoice.quote.update` | Met à jour un devis existant | 1–2 |
| `tool.invoice.quote.to_invoice` | Convertit un devis en facture (exécution ; décision = StrongFather) | 2 |
| `tool.invoice.create` | Crée une facture (métier indépendant, hors abo SaaS) à partir de données fournies | 1–2 |
| `tool.invoice.send` | Envoie une facture par canal fourni (email, etc.) | 1–2 |
| `tool.invoice.electronic.submit` | Soumet à la facturation électronique (plateforme agréée 2026) | 2 |
| `tool.invoice.reminder.send` | Envoie une relance (exécution ; règles = StrongFather) | 1–2 |
| `tool.invoice.payment.link.generate` | Génère un lien de paiement pour une facture | 1–2 |
| `tool.invoice.customer.resolve` | Résout un client (facturation) par identifiant | 0–1 |
| `tool.invoice.customer.list` | Liste les clients (filtres fournis) pour facturation | 0–1 |

*Recoupement avec MiyuBilling :* `tool.billing.invoice.generate`, `tool.billing.invoice.list`, `tool.billing.payment.record` restent le cœur facturation SaaS (abonnements, factures récurrentes). Les Tools ci-dessus couvrent la **facturation métier indépendant** (devis, factures ponctuelles, relances, électronique B2B). Un même environnement peut déclarer les deux ; le périmètre (tenant / structure) est résolu par KindMother et Master Butler.

---

## 3. Cartographie Comptabilité (écritures, banque, rapports) → Miyukini COG

### 3.1 Fonctionnalités comptabilité et équivalents

| Fonctionnalité marché | Équivalent Miyukini | Type | Détail |
|------------------------|----------------------|------|--------|
| **Synchronisation bancaire** | Tool synchro + gouvernance | Tool | `tool.compta.bank.sync` (déclenche ou enregistre synchro API/EBICS/agrégateur) ; données brutes = KindMother ; reconnexion DSP2 = politique Ever Buddy. |
| **Catégorisation des écritures** | Tool catégorisation | Tool | `tool.compta.transaction.categorize` ; règles = StrongFather / KindMother (règles fournies dans le flux). |
| **Détection TVA** | Tool TVA | Tool | `tool.compta.transaction.vat.resolve` ; taux 5,5 % / 10 % / 20 % = données ou règles. |
| **Livre des recettes** | Tool rapport | Tool | `tool.compta.report.livre_recettes.generate` ; données = KindMother. |
| **Rapprochement bancaire** | Tools rapprochement | Tools | `tool.compta.reconciliation.suggest`, `tool.compta.reconciliation.record` ; validation = StrongFather. |
| **Bilan / Compte de résultat** | Tool rapport | Tool | `tool.compta.report.balance.generate` ; lecture seule, données = KindMother. |
| **Liasse fiscale** | Tool rapport | Tool | `tool.compta.report.liasse.generate` ; export selon régime (réel normal / simplifié) ; décision export = StrongFather. |
| **Multi-structures** | KindMother (périmètre) + Tool | Core + Tool | `tool.company.structure.resolve`, `tool.company.structure.register` ; chaque structure = périmètre de données. |

### 3.2 Opérateur Comptabilité (MiyuCompta)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère les écritures, la synchronisation bancaire, le rapprochement, la TVA et les rapports comptables (livre des recettes, bilan, liasse) pour le compte de l'utilisateur. |
| **Service perçu** | « Comptabilité / tenue des livres » |
| **Tools utilisés** | `tool.compta.bank.sync`, `tool.compta.transaction.categorize`, `tool.compta.transaction.vat.resolve`, `tool.compta.reconciliation.suggest`, `tool.compta.reconciliation.record`, `tool.compta.report.livre_recettes.generate`, `tool.compta.report.balance.generate`, `tool.compta.report.liasse.generate`, `tool.compta.export.ledger` ; contexte structure = `tool.company.structure.resolve`. |
| **Données** | KindMother (écritures, transactions bancaires, règles de catégorisation, paramètres TVA, rapports générés). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; écriture = WriteIntent vers KindMother. |

### 3.3 Outils comptabilité (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.compta.bank.sync` | Déclenche ou enregistre une synchronisation bancaire (API/EBICS/agrégateur) | 2 |
| `tool.compta.transaction.categorize` | Catégorise une écriture (exécution ; règles fournies) | 1–2 |
| `tool.compta.transaction.vat.resolve` | Rattache un taux TVA à une écriture | 0–1 |
| `tool.compta.reconciliation.suggest` | Propose des rapprochements (sans décider) | 0–1 |
| `tool.compta.reconciliation.record` | Enregistre un rapprochement validé ; autorisation = StrongFather | 2 |
| `tool.compta.report.livre_recettes.generate` | Génère le livre des recettes | 1–2 |
| `tool.compta.report.balance.generate` | Génère bilan / compte de résultat | 1–2 |
| `tool.compta.report.liasse.generate` | Génère la liasse fiscale (export) | 2 |
| `tool.compta.report.cashflow.generate` | Génère un rapport flux de trésorerie / prévisionnel | 1–2 |
| `tool.compta.export.ledger` | Export des écritures (format fourni) ; autorisation = StrongFather | 2 |
| `tool.company.structure.resolve` | Résout la structure juridique courante (micro, EURL, etc.) pour le contexte | 0–1 |
| `tool.company.structure.register` | Enregistre une structure (WriteIntent KindMother) | 2 |
| `tool.company.siret.resolve` | Récupère les informations depuis SIRET/INSEE (lecture seule) | 0–1 |

---

## 4. Cartographie Déclarations (URSSAF, TVA, échéances) → Miyukini COG

### 4.1 Fonctionnalités déclarations et équivalents

| Fonctionnalité marché | Équivalent Miyukini | Type | Détail |
|------------------------|----------------------|------|--------|
| **Déclaration URSSAF (CA)** | Tools déclaration | Tools | `tool.compta.declaration.urssaf.prepare`, `tool.compta.declaration.urssaf.submit` ; télédéclaration = exécution ; décision envoi = StrongFather. |
| **Règlement cotisations** | Hors scope Tool direct | — | Le paiement relève du flux bancaire ou d'un Opérateur Paiement ; le Tool prépare/soumet la déclaration. |
| **Déclaration TVA** | Tools déclaration | Tools | `tool.compta.declaration.tva.prepare`, `tool.compta.declaration.tva.submit` ; données = KindMother. |
| **Historique déclarations** | KindMother + Tool liste | Core + Tool | Données = KindMother ; `tool.compta.declaration.list` (filtres fournis). |
| **Échéances fiscales et sociales** | Tool échéances | Tool | `tool.compta.declaration.deadline.list` ; données fournies (calendrier, structure). |
| **Estimateur cotisations (micro)** | Tool calcul | Tool | `tool.compta.declaration.estimate.cotisations` ; calcul indicatif à partir de CA fourni ; pas de décision métier. |

### 4.2 Opérateur Déclarations (MiyuDeclarations)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Prépare et soumet les déclarations fiscales et sociales (URSSAF, TVA), liste les échéances et l'historique des déclarations. |
| **Service perçu** | « Déclarations URSSAF / TVA / échéances » |
| **Tools utilisés** | `tool.compta.declaration.urssaf.prepare`, `tool.compta.declaration.urssaf.submit`, `tool.compta.declaration.tva.prepare`, `tool.compta.declaration.tva.submit`, `tool.compta.declaration.deadline.list`, `tool.compta.declaration.list`, `tool.compta.declaration.estimate.cotisations`. |
| **Données** | KindMother (données déclarations, historique, calendrier échéances). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; soumission = décision StrongFather. |

### 4.3 Outils déclarations (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.compta.declaration.urssaf.prepare` | Prépare les données de déclaration URSSAF (CA, etc.) | 1–2 |
| `tool.compta.declaration.urssaf.submit` | Soumet la déclaration URSSAF (télédéclaration) ; autorisation = StrongFather | 2 |
| `tool.compta.declaration.tva.prepare` | Prépare la déclaration TVA | 1–2 |
| `tool.compta.declaration.tva.submit` | Soumet la déclaration TVA ; autorisation = StrongFather | 2 |
| `tool.compta.declaration.deadline.list` | Liste les échéances fiscales et sociales (données fournies) | 0–1 |
| `tool.compta.declaration.list` | Liste l'historique des déclarations (filtres fournis) | 0–1 |
| `tool.compta.declaration.estimate.cotisations` | Calcule une estimation des cotisations (micro) à partir de CA fourni | 0–1 |

---

## 5. Cartographie Notes de frais et indemnités → Miyukini COG

### 5.1 Fonctionnalités notes de frais et équivalents

| Fonctionnalité marché | Équivalent Miyukini | Type | Détail |
|------------------------|----------------------|------|--------|
| **Saisie justificatif (photo / scan)** | Tool capture | Tool | `tool.expense.receipt.capture` ; persistance = KindMother. |
| **OCR justificatif** | Tool extraction | Tool | `tool.expense.receipt.extract` (OCR) ; retourne montant, date, TVA, fournisseur ; exécution seule. |
| **Note de frais (création, lignes)** | Tools note de frais | Tools | `tool.expense.claim.create`, `tool.expense.claim.update`, `tool.expense.claim.list` ; données = KindMother. |
| **Validation note de frais** | Tool validation | Tool | `tool.expense.claim.validate` ; décision = StrongFather. |
| **Indemnités kilométriques** | Tools kilométriques | Tools | `tool.expense.mileage.calculate` (barème fourni), `tool.expense.mileage.export` (PDF/CSV) ; trajets = données ou intégration agenda (périmètre Opérateur). |
| **Export vers compta** | Tool export | Tool | `tool.expense.claim.export` ou flux BondingBrother vers Opérateur Comptabilité ; autorisation = StrongFather. |

### 5.2 Opérateur Notes de frais (MiyuExpense)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère la saisie des justificatifs, l'extraction OCR, les notes de frais et les indemnités kilométriques ; prépare l'export vers la comptabilité. |
| **Service perçu** | « Notes de frais et indemnités » |
| **Tools utilisés** | `tool.expense.receipt.capture`, `tool.expense.receipt.extract`, `tool.expense.claim.create`, `tool.expense.claim.update`, `tool.expense.claim.list`, `tool.expense.claim.validate`, `tool.expense.mileage.calculate`, `tool.expense.mileage.export`, `tool.expense.claim.export`. |
| **Données** | KindMother (justificatifs, notes de frais, barème kilométrique, validations). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; validation = StrongFather. |

### 5.3 Outils notes de frais (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.expense.receipt.capture` | Enregistre un justificatif (photo/scan) ; WriteIntent KindMother | 1–2 |
| `tool.expense.receipt.extract` | Extrait les données d'un justificatif par OCR (exécution seule) | 0–1 |
| `tool.expense.claim.create` | Crée une note de frais à partir de données fournies | 1–2 |
| `tool.expense.claim.update` | Met à jour une note de frais | 1–2 |
| `tool.expense.claim.list` | Liste les notes de frais (filtres fournis) | 0–1 |
| `tool.expense.claim.validate` | Valide une note de frais (workflow ; décision = StrongFather) | 2 |
| `tool.expense.mileage.calculate` | Calcule les indemnités kilométriques selon barème fourni | 0–1 |
| `tool.expense.mileage.export` | Export PDF/CSV des indemnités pour administration | 1 |
| `tool.expense.claim.export` | Export des notes de frais vers compta (format fourni) ; autorisation = StrongFather | 2 |

---

## 6. Cartographie Trésorerie et prévisionnel → Miyukini COG

### 6.1 Fonctionnalités trésorerie et équivalents

| Fonctionnalité marché | Équivalent Miyukini | Type | Détail |
|------------------------|----------------------|------|--------|
| **Tableau de bord trésorerie** | Tool agrégation | Tool | `tool.treasury.dashboard.aggregate` ; indicateurs (solde, créances, dettes) = agrégation des données KindMother. |
| **Prévisionnel** | Tool calcul | Tool | `tool.treasury.forecast.compute` ; scénarios à partir de données fournies ; exécution seule. |
| **Alertes échéances / seuils** | Tool vérification | Tool | `tool.treasury.alert.check` ; exécution ; règles = StrongFather ; notifications = périmètre Opérateur ou Ever Buddy. |

### 6.2 Opérateur Trésorerie (MiyuTreasury)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Agrège les indicateurs de trésorerie, calcule les prévisionnels et vérifie les alertes (seuils, échéances). |
| **Service perçu** | « Trésorerie et prévisionnel » |
| **Tools utilisés** | `tool.treasury.dashboard.aggregate`, `tool.treasury.forecast.compute`, `tool.treasury.alert.check` ; données sous-jacentes = KindMother (écritures, factures, échéances). |
| **Données** | KindMother (lecture) ; pas d'écriture métier propre, sauf paramètres alertes si définis. |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. |

### 6.3 Outils trésorerie (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.treasury.dashboard.aggregate` | Agrège les indicateurs pour le tableau de bord trésorerie | 1–2 |
| `tool.treasury.forecast.compute` | Calcule un prévisionnel à partir de données fournies | 1–2 |
| `tool.treasury.alert.check` | Vérifie les seuils et échéances (exécution ; règles = StrongFather) | 1–2 |

---

## 7. Synthèse — Toolkits et Opérateurs catalogue

### 7.1 Toolkits

| ToolkitId | Domaine | Rôle |
|-----------|---------|------|
| `toolkit.invoice.standalone` | invoice / billing | Devis, factures métier indépendant, relances, facturation électronique B2B |
| `toolkit.compta.ledger` | compta | Écritures, banque, rapprochement, TVA, structure |
| `toolkit.compta.declarations` | compta | URSSAF, TVA, échéances, historique déclarations |
| `toolkit.compta.reports` | compta | Livre des recettes, bilan, liasse, flux de trésorerie, export |
| `toolkit.expense.claims` | expense | Notes de frais, justificatifs, indemnités kilométriques |
| `toolkit.treasury.forecast` | treasury | Prévisionnel, tableau de bord, alertes |

### 7.2 Opérateurs

| Opérateur | Type | Service perçu | Tools principaux |
|-----------|------|----------------|------------------|
| **Facturation (indépendants)** | Domaine / Interface | Devis, factures, relances, facturation électronique | toolkit.invoice.standalone ; évent. tool.billing.invoice.* (MiyuBilling) |
| **MiyuCompta** | Domaine | Comptabilité / tenue des livres | toolkit.compta.ledger, toolkit.compta.reports |
| **MiyuDeclarations** | Domaine | Déclarations URSSAF / TVA / échéances | toolkit.compta.declarations |
| **MiyuExpense** | Domaine | Notes de frais et indemnités | toolkit.expense.claims |
| **MiyuTreasury** | Domaine | Trésorerie et prévisionnel | toolkit.treasury.forecast |

---

## 8. Services utilisateur : « compta seule » vs « compta + expert »

### 8.1 Service « Compta seule » (autonome)

L'utilisateur gère sa comptabilité sans expert-comptable. Le service est porté par une **Équipe d'Opérateurs** (Facturation, Comptabilité, Déclarations, Notes de frais, Trésorerie) sous un **Contrat d'équipe** et un **Mandat de Permission** uniques.

| Élément | Description |
|---------|-------------|
| **Opérateurs membres** | Facturation, MiyuCompta, MiyuDeclarations, MiyuExpense, MiyuTreasury |
| **Flux autorisés** | Facturation → Comptabilité (écritures factures) ; Notes de frais → Comptabilité (export) ; Comptabilité ↔ Déclarations (données) ; Trésorerie lit Comptabilité / Facturation / Déclarations |
| **Direction des flux** | Toute communication via BondingBrother ; pas de dialogue direct Opérateur à Opérateur |
| **Types d'échanges** | Données factures, écritures, déclarations préparées, notes de frais validées, indicateurs trésorerie |
| **Niveau de validation** | StrongFather pour validation note de frais, envoi déclaration, export sensible ; Master Butler pour permissions Tools |

### 8.2 Service « Compta + expert-comptable »

L'utilisateur partage ses données avec un expert-comptable externe (autre COG ou tiers). La collaboration est encadrée par un **Mandat de Permission** spécifique et, si l'expert est dans un autre COG, par un **Bridge inter-COG** ou une **Façade Publique Gouvernée** (export sécurisé, accès en lecture clôture, etc.).

| Élément | Description |
|---------|-------------|
| **Acteurs** | Utilisateur (citoyen du COG) ; Opérateurs compta (inchangés) ; Expert-comptable (Utilisateur Visiteur ou consommateur d'une API/export gouverné) |
| **Contrat d'équipe étendu** | L'« équipe » inclut le périmètre de collaboration avec l'expert : flux autorisés = export écritures, export rapports, accès lecture clôture (selon Mandat) |
| **Mandat de Permission « Expert »** | Émis par StrongFather ; définit : quelles données sont partagées (écritures, bilan, liasse, notes de frais), durée de validité, niveau de sécurité (lecture seule vs export), révocation possible |
| **Bridge inter-COG** | Si l'expert opère dans un autre COG : le Bridge transporte identité et autorisations ; aucun pouvoir décisionnel côté Bridge ; l'expert reçoit des données exportées ou un accès en lecture gouverné |
| **Règle** | *« Le COG n'accueille jamais une gouvernance étrangère. »* L'expert n'exécute pas dans le COG utilisateur ; il reçoit des données ou un accès encadré par Mandat. |

### 8.3 Contrats d'équipe et Mandats — tableau récapitulatif

| Service | Équipe d'Opérateurs | Mandat de Permission | Remarque |
|---------|---------------------|----------------------|----------|
| **Compta seule** | Facturation + MiyuCompta + MiyuDeclarations + MiyuExpense + MiyuTreasury | Un Mandat couvre l'usage des Tools par l'utilisateur (ou par un rôle « indépendant ») ; durée selon politique (ex. session, mois). | Aucun acteur externe ; tous les flux restent internes au COG. |
| **Compta + expert** | Même équipe + périmètre « collaboration expert » | Mandat « Expert » : autorise export / accès lecture pour l'expert identifié ; durée limitée ; révocation par StrongFather. | Export = `tool.compta.export.ledger`, `tool.compta.report.liasse.generate`, etc. ; l'expert ne modifie pas les données dans le COG. |

**Référence :** [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md).

---

## 9. Cores impliqués et flux de gouvernance

Les Cores **ne font jamais d'exécution** ; ils gouvernent, décident ou observent.

| Core | Rôle dans le périmètre Comptabilité Indépendants |
|------|--------------------------------------------------|
| **KindMother** | Autorité sur toutes les données : factures, devis, écritures, transactions bancaires, justificatifs, notes de frais, déclarations, paramètres société, barèmes, structures. WriteIntent pour toute écriture. |
| **StrongFather** | Décision finale ALLOW/DENY : validation note de frais, envoi déclaration URSSAF/TVA, export sensible (liasse, écritures), relance, partage expert-comptable. Émission et révocation des Mandats de Permission (dont Mandat « Expert »). Validation des Contrats d'équipe. |
| **Master Butler** | Déclaration des Tools et Toolkits compta ; permissions et capabilities. Catalogue des capacités (invoice.standalone, compta.ledger, compta.declarations, expense.claims, treasury.forecast). |
| **BondingBrother** | Médiation des intentions (utilisateur, expert-comptable) ; traduction vers les Cores et les Opérateurs ; flux entre Opérateurs (Facturation → Comptabilité, Notes de frais → Comptabilité, etc.). |
| **WorrySentinel** | Niveau de sécurité (données fiscales, bancaires, partage expert) ; blocage si menace ou état dégradé. |
| **Caring Nanny** | État système (HEALTHY, DEGRADED, etc.) ; blocage des Tools si environnement dégradé. |
| **Ever Buddy** | Cycle de vie : versions des Tools/Toolkits compta, dépréciation, compatibilité (ex. facturation électronique 2026). |
| **Border Guard** | Frontières et niveaux de confiance ; données France, hébergement, plateformes agréées ; collaboration inter-COG avec expert. |
| **TAMR** | Points d'intervention humaine : arbitrage catégorisation, validation note de frais, décision relance, accord partage expert. |

**Flux générique :** Opérateur → BondingBrother → Master Butler (existence Tool, permissions) → WorrySentinel (niveau sécurité) → Caring Nanny (état système) → StrongFather (ALLOW/DENY) → Exécution Tool ; toute persistance passe par WriteIntent KindMother.

---

## 10. Recoupements avec MiyuBilling et autres référentiels

### 10.1 MiyuBilling (facturation SaaS)

| Concept | MiyuBilling (existant) | Comptabilité Indépendants (ce document) |
|---------|------------------------|----------------------------------------|
| **Factures** | Souscriptions, factures récurrentes, paiements enregistrés, tenant | Devis, factures ponctuelles, relances, facturation électronique B2B ; clients facturation |
| **Usage commun** | Un environnement peut déclarer les deux : MiyuBilling pour abonnements SaaS, toolkit.invoice.standalone pour facturation métier indépendant | Périmètre (tenant / structure) résolu par KindMother et Master Butler |
| **Outils partagés possibles** | `tool.billing.invoice.generate`, `tool.billing.invoice.list`, `tool.billing.payment.record` | `tool.invoice.create` peut alimenter les mêmes modèles de données si unifié ; sinon distinction facturation SaaS vs métier |

### 10.2 Équivalents PoS / Boutique CMS

| Concept | PoS / Boutique | Comptabilité Indépendants |
|---------|----------------|---------------------------|
| **Export compta** | PoS : écritures ventes/paiements exportées vers système compta (BondingBrother, Bridge) | Ce référentiel décrit le **système compta** qui reçoit ces écritures ; l'Opérateur Comptabilité peut ingérer des flux d'autres Opérateurs (Caisse, Commerce) sous Mandat. |
| **Client** | CRM / Fidélité (PoS), Commerce (Boutique) | `tool.invoice.customer.*` ou partage `tool.crm.customer.*` si même base clients. |

---

## 11. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Opérateurs et Terminologie | [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| Mandats et Équipes Opérateurs | [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| Pyramide Architecture Complète | [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Définition COG | [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) |
| Équivalents Boutique CMS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Équivalents PoS | [Miyukini Conceptual References - Equivalents PoS Logiciel Caisse](./Miyukini%20Conceptual%20References%20-%20Equivalents%20PoS%20Logiciel%20Caisse.md) |
| Connexion Inter-COG | [Miyukini Conceptual References - Connexion Inter-COG](./Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — Équivalents Comptabilité Indépendants pour services spécialisés Miyukini COG
