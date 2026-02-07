# Miyukini Conceptual References — Équivalents PoS (Point of Sale) et logiciel de caisse

## Contexte

Ce document constitue la **référence conceptuelle** pour transposer, dans l'environnement Miyukini COG, les fonctionnalités des **logiciels de caisse / Point of Sale (PoS)** tels que **Loyverse**, **Odoo POS** et logiciels de caisse génériques. Il vise à permettre la création d'**outils**, **opérateurs** et **services** Miyukini pour proposer des **services PoS spécialisés** :

- **Caisse et ventes** (tickets, articles, variantes, modificateurs, remises, remboursements, reçus)
- **Inventaire** (stock, multi-magasin, transferts, inventaires physiques, production, étiquettes)
- **Employés et plannings** (droits, pointeuse, ventes par employé)
- **Analytics ventes** (tendances, articles populaires, shifts, taxes, export)
- **CRM et fidélité** (clients, points, cartes fidélité, historique achats)
- **Restaurant / bar** (cuisine, affichage cuisine, types de service, tickets prédéfinis)
- **Paiements** (espèces, CB, terminaux, partage d'addition)
- **Intégrations** (compta, e-commerce, API)

Il **s'appuie sur** la documentation conceptuelle existante : [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md), [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md), [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md), [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md), [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md).

---

## Fondements conceptuels (alignement documentation existante)

Ce document applique les **définitions canoniques** et **règles** des références listées ci-dessus. Les équivalents PoS respectent en particulier :

### Outils (Tools) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Outil est une capacité exécutable, sans autorité, sans décision métier, sans connaissance de l'Opérateur appelant, gouvernée par les Cores.
- **Règle :** *« Un Outil fait, mais ne décide jamais. »* Les Tools PoS (ex. `tool.pos.sale.create`, `tool.pos.refund.record`) exécutent des actions ; la décision (remboursement autorisé ou non, etc.) appartient à **StrongFather**.

### Kits d'Outils (Toolkits) — [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)

- **Définition :** Un Kit d'Outils est une composition officielle d'Outils, validée et déclarée par l'environnement, optimisée pour efficience et cohérence.
- **Règle :** *« Un Kit d'Outils n'ajoute aucune capacité nouvelle, il orchestre proprement des Outils existants. »* Les Toolkits PoS (`toolkit.pos.sales`, `toolkit.pos.inventory`, etc.) agrègent des Tools existants sans logique métier propre.

### Opérateurs (Operators) — [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)

- **Définition :** Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.
- Les Opérateurs PoS (Caisse, Inventaire, Fidélité/CRM, etc.) sont des **Opérateurs de Domaine** ou **d'Interface** (Strate 7) ; ils n'ont pas d'autorité propre et passent par la gouvernance pour toute action.

### Service vs Opérateur — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- **Service** = capacité perçue par l'utilisateur. **Opérateur** = unité d'exécution gouvernée.
- **Règle :** *« Un Service peut être porté par un Opérateur... ou par une Équipe d'Opérateurs. »* Les services PoS spécialisés (caisse retail, caisse + inventaire, restaurant complet) sont donc livrés par un ou plusieurs Opérateurs sous **Contrat d'équipe** et **Mandat de Permission**.

### Collaboration entre Opérateurs — [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)

- *« Aucun Opérateur ne parle librement à un autre. »* Toute communication Caisse ↔ Inventaire, Caisse ↔ Fidélité, etc. passe par **BondingBrother**, est définie dans le **Contrat d'équipe** et autorisée par un **Mandat de Permission** émis par StrongFather.
- *« Le contrat est validé UNE FOIS, pas à chaque appel. »* Le Contrat d'équipe est statique (conception) ; le Mandat de Permission encadre l'exécution.

### Pyramide et COG — [Pyramide Architecture Complète](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md), [Définition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md)

- **Strate 6** : Tools & Toolkits PoS (capacités gouvernées).
- **Strate 7** : Opérateurs PoS (Caisse, Inventaire, Fidélité, Analytics, Restaurant, Paiement, RH).
- **COG** : environnement de gouvernance orchestré par des Cores ; Miyukini n'est pas un OS, c'est le « cog » qui fait fonctionner les systèmes ensemble.

### Données et écriture — [Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md)

- **KindMother** : autorité sur toutes les données (ventes, tickets, stock, clients, points, mouvements caisse). Toute écriture passe par **WriteIntent** sous autorité KindMother.
- **StrongFather** : décision ALLOW/DENY (remboursements, ajustements stock, clôture shift, partage paiement, octroi/rédemption points). N'exécute jamais.

---

## Portée / Scope

**Ce document définit :**

- La cartographie détaillée **PoS / logiciel de caisse** → Outils, Opérateurs, Services Miyukini
- Les **Kits d'outils (Toolkits)** et **Outils (Tools)** à créer ou à réutiliser, conformes aux définitions de [Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)
- Les **Opérateurs** (Domaine, Interface, Service) à déployer, conformes à [Opérateurs et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md)
- Les **Services** perçus par l'utilisateur et les **Équipes d'Opérateurs** / **Contrats d'équipe** / **Mandats de Permission**, conformes à [Mandats et Équipes Opérateurs](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md)
- Les **Cores** impliqués et les flux de gouvernance (Glossaire)
- Les **patterns** hors ligne, multi-magasin et intégrations en termes COG

**Hors scope :**

- L'implémentation technique détaillée (code, schémas DB)
- Les contrats d'intégration par outil (voir documentations fondatrices des Tools)
- La stratégie commerciale ou marketing des services

**Statut :** Document de référence normatif — source de vérité pour la conception des services PoS Miyukini.

---

## 1. Périmètre cible et objectifs

### 1.1 Équivalents logiciels ciblés

| Équivalent | Rôle | Objectif Miyukini |
|------------|------|--------------------|
| **Loyverse POS** | Caisse mobile, inventaire, employés, fidélité, restaurant, paiements | Opérateurs Caisse, Inventaire, Fidélité, Analytics + Toolkits pos.sales, pos.inventory, pos.loyalty, pos.payment |
| **Odoo POS** | Caisse web, multi-devices, inventaire/compta/CRM intégrés | Même modèle COG ; intégration KindMother (stock, compta) |
| **Logiciel de caisse générique** | Ventes, reçus, stock, multi-caisses, rapports | Toolkits atomiques + Opérateurs gouvernés |

### 1.2 Services utilisateur visés

| Service | Description | Opérateurs / Tools principaux |
|--------|-------------|-------------------------------|
| **Caisse / vente au détail** | Enregistrement des ventes, tickets, reçus, remises, remboursements | Opérateur Caisse (Interface/Domaine), Toolkit pos.sales |
| **Inventaire multi-magasin** | Stock, transferts, inventaires, bons de commande, alertes | Opérateur Inventaire, Toolkit pos.inventory |
| **Gestion employés** | Droits, plannings, pointeuse, ventes par employé | Opérateur RH ou Domaine ; Tools hr.*, analytics.* ; permissions = Master Butler + StrongFather |
| **Analytics ventes** | Tendances, articles populaires, shifts, taxes, export | Opérateur Analytics ou Domaine, Toolkit pos.analytics |
| **CRM et fidélité** | Clients, points, cartes fidélité, historique achats | Opérateur Fidélité / CRM, Toolkits pos.loyalty, pos.crm |
| **Restaurant / bar** | Cuisine, affichage cuisine, types de service, tickets tables | Opérateur Restaurant (Domaine/Interface), Tools pos.kitchen.*, pos.ticket.* |
| **Paiements** | Espèces, CB, terminaux, partage d'addition | Opérateur Paiement ou Tools dans flux caisse ; Toolkit pos.payment |
| **Intégrations** | Compta, e-commerce, API | BondingBrother, Bridge inter-COG, Mandats ; pas d'Opérateur métier dédié |

---

## 2. Cartographie Point of Sale (caisse) → Miyukini COG

### 2.1 Fonctionnalités caisse et équivalents

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Vente smartphone / tablette** | Opérateur d'Interface **Caisse** | Opérateur | UI caisse ; utilise tool.pos.sale.*, tool.pos.ticket.*, tool.pos.receipt.* ; données fournies dans le flux (KindMother en amont). |
| **Multi-magasin** | KindMother (sites) + contexte magasin | Core + flux | `tool.pos.context.store.resolve` ; pas de décision métier dans le Tool. |
| **Tickets ouverts / sauvegarde** | Tools ticket | Tools | `tool.pos.ticket.open`, `tool.pos.ticket.save`, `tool.pos.ticket.close`, `tool.pos.ticket.list` ; persistance = KindMother. |
| **Reçus imprimés / email** | Tools reçu | Tools | `tool.pos.receipt.render`, `tool.pos.receipt.print`, `tool.pos.receipt.send` ; contenu fourni dans le flux. |
| **Remises (article ou reçu)** | Tool remise | Tool | `tool.pos.discount.apply` (scope item ou receipt) ; règles = StrongFather. |
| **Remboursements** | Tool remboursement | Tool | `tool.pos.refund.record` (item ou reçu) ; autorisation = StrongFather. |
| **Variantes d'articles (taille, couleur)** | Tool variante | Tool | `tool.pos.item.variant.resolve` ; catalogue = KindMother. |
| **Modificateurs (options, add-ons)** | Tool modificateur | Tool | `tool.pos.item.modifier.apply` ; règles = données fournies. |
| **Gestion caisse (mouvements espèces)** | Tools caisse | Tools | `tool.pos.cash.register.open`, `tool.pos.cash.register.close`, `tool.pos.cash.movement.record` ; traçabilité = KindMother. |
| **Vente hors ligne + sync** | Caring Nanny (état) + KindMother | Cores | Sync = WriteIntent / flux gouverné ; pas de Tool dédié « offline », état système autorise ou restreint. Voir section 13 (Patterns). |
| **Codes-barres (dont poids)** | Tool scan | Tool | `tool.pos.barcode.parse` (poids embarqué si applicable) ; retourne item/quantité. |
| **Matériel (imprimante, tiroir, scanner)** | Tools ou périmètre Opérateur | Tools / Opérateur | `tool.pos.hardware.receipt.print`, `tool.pos.hardware.drawer.open`, `tool.pos.hardware.scan` — ou délégation à un Opérateur périmètre matériel. |
| **Affichage client (second écran)** | Tool affichage | Tool | `tool.pos.display.push` (données reçu/ordre fournies dans le flux). |
| **Thème sombre** | MiyuWeb / contexte UI | Toolkit existant | `tool.web.theme.resolve` avec contexte « pos » / « dark ». |

### 2.2 Opérateur Caisse (Point of Sale)

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur d'Interface ou Opérateur de Domaine |
| **Rôle** | Expose l'interface de caisse et enregistre les ventes (tickets, articles, paiements, reçus) pour le compte de l'utilisateur. |
| **Service perçu** | « Caisse / vente au détail » |
| **Tools utilisés** | `tool.pos.sale.*`, `tool.pos.ticket.*`, `tool.pos.receipt.*`, `tool.pos.discount.apply`, `tool.pos.refund.record`, `tool.pos.cash.*`, `tool.pos.item.variant.resolve`, `tool.pos.item.modifier.apply`, `tool.pos.barcode.parse`, évent. `tool.pos.hardware.*`, `tool.pos.display.push` ; affichage via MiyuWeb. |
| **Données** | KindMother (ventes, tickets, reçus, mouvements caisse, catalogue). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; écriture = WriteIntent vers KindMother. |

### 2.3 Outils caisse (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.pos.sale.create` | Crée une vente (ticket) à partir du contexte fourni | 1–2 |
| `tool.pos.sale.add_item` | Ajoute une ligne (article, variante, modificateurs, qté) à une vente | 1–2 |
| `tool.pos.sale.remove_item` | Retire une ligne d'une vente | 1–2 |
| `tool.pos.ticket.open` | Ouvre un ticket (ordre) pour paiement différé | 1–2 |
| `tool.pos.ticket.save` | Sauvegarde un ticket sans le clôturer | 1–2 |
| `tool.pos.ticket.close` | Clôture un ticket (après paiement ou annulation) | 1–2 |
| `tool.pos.ticket.list` | Liste les tickets ouverts (filtres fournis) | 0–1 |
| `tool.pos.ticket.preset.assign` | Assigne un libellé prédéfini (ex. Table 1) à un ticket | 1 |
| `tool.pos.discount.apply` | Applique une remise (article ou reçu) à partir de données fournies | 1–2 |
| `tool.pos.refund.record` | Enregistre un remboursement (item ou reçu) ; autorisation = StrongFather | 2 |
| `tool.pos.receipt.render` | Produit le contenu du reçu à partir des données de vente | 0–1 |
| `tool.pos.receipt.print` | Envoie le reçu à l'imprimante (données fournies) | 1 |
| `tool.pos.receipt.send` | Envoie le reçu par email (données fournies) | 1–2 |
| `tool.pos.receipt.list` | Liste les reçus (filtres fournis) | 0–1 |
| `tool.pos.item.variant.resolve` | Résout une variante (taille, couleur, etc.) pour un article | 0–1 |
| `tool.pos.item.modifier.apply` | Applique des modificateurs (add-ons) à une ligne | 0–1 |
| `tool.pos.cash.register.open` | Ouvre une session caisse (ouverture de tiroir) | 2 |
| `tool.pos.cash.register.close` | Clôture une session caisse (comptage, écart) | 2 |
| `tool.pos.cash.movement.record` | Enregistre un mouvement espèces (entrée/sortie) | 2 |
| `tool.pos.barcode.parse` | Parse un code-barres (optionnel : poids) ; retourne item + quantité | 0–1 |
| `tool.pos.context.store.resolve` | Résout le magasin/point de vente courant pour le contexte | 0–1 |
| `tool.pos.display.push` | Envoie les données à afficher sur l'écran client | 0–1 |
| `tool.pos.order.service_type.set` | Définit le type de service (sur place / à emporter / livraison) | 1 |

---

## 3. Cartographie Inventaire → Miyukini COG

### 3.1 Fonctionnalités inventaire et équivalents

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Import articles (CSV)** | Tool import | Tool | `tool.data.import.csv` ou `tool.inventory.import.items` ; persistance = KindMother. |
| **Suivi stock / composants** | KindMother + Tools stock | Core + Tools | `tool.inventory.stock.get`, `tool.inventory.stock.adjust` ; BOM/recettes = données. |
| **Alertes stock bas** | Tool ou règle Ever Buddy | Tool / Core | `tool.inventory.alert.low.evaluate` (retourne liste) ; notifications = périmètre Opérateur ou Ever Buddy. |
| **Bons de commande fournisseur** | Tools commande | Tools | `tool.inventory.purchase_order.create`, `tool.inventory.purchase_order.update`, `tool.inventory.purchase_order.track` ; données = KindMother. |
| **Transferts entre magasins** | Tools transfert | Tools | `tool.inventory.transfer.create`, `tool.inventory.transfer.execute`, `tool.inventory.transfer.list` ; autorisation = StrongFather. |
| **Ajustements stock (réception, casse, perte)** | Tool ajustement | Tool | `tool.inventory.stock.adjust` (raison, quantité, site) ; décision = StrongFather. |
| **Inventaire physique (comptage)** | Tools comptage | Tools | `tool.inventory.count.start`, `tool.inventory.count.record`, `tool.inventory.count.reconcile` ; écarts = WriteIntent. |
| **Production (recettes / composants)** | Tool production | Tool | `tool.inventory.production.record` ; débit composants + crédit produit = KindMother. |
| **Étiquettes codes-barres** | Tool impression | Tool | `tool.pos.label.print` (données article/prix fournies). |
| **Historique / valorisation stock** | KindMother + Tools lecture | Core + Tools | `tool.inventory.history.list`, `tool.inventory.valuation.report` (lecture seule). |

### 3.2 Opérateur Inventaire

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère le cycle de vie du stock (entrées, sorties, transferts, comptages, production, alertes) pour un ou plusieurs magasins. |
| **Service perçu** | « Inventaire multi-magasin » |
| **Tools utilisés** | `tool.inventory.*` ; import/export évent. `tool.data.import.*`, `tool.data.export.*`. |
| **Données** | KindMother (articles, stock, mouvements, bons de commande, transferts, comptages). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather ; écriture = WriteIntent. |

### 3.3 Outils inventaire (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.inventory.stock.get` | Retourne le stock (et composants si applicable) pour un article/site | 0–1 |
| `tool.inventory.stock.adjust` | Ajuste le stock (réception, casse, perte) ; décision = StrongFather | 2 |
| `tool.inventory.import.items` | Importe des articles à partir d'un flux structuré (ex. CSV) ; persistance = KindMother | 2 |
| `tool.inventory.alert.low.evaluate` | Évalue les articles sous seuil bas (données seuils fournies) | 0–1 |
| `tool.inventory.purchase_order.create` | Crée un bon de commande fournisseur à partir de données fournies | 1–2 |
| `tool.inventory.purchase_order.update` | Met à jour un bon de commande (réception partielle, etc.) | 1–2 |
| `tool.inventory.purchase_order.track` | Retourne le statut / suivi d'un bon de commande | 0–1 |
| `tool.inventory.transfer.create` | Crée un transfert entre sites à partir de données fournies | 1–2 |
| `tool.inventory.transfer.execute` | Exécute (confirme) un transfert ; autorisation = StrongFather | 2 |
| `tool.inventory.transfer.list` | Liste les transferts (filtres fournis) | 0–1 |
| `tool.inventory.count.start` | Démarre une session d'inventaire physique | 1–2 |
| `tool.inventory.count.record` | Enregistre un comptage (article, quantité) pour une session | 1–2 |
| `tool.inventory.count.reconcile` | Clôture un comptage et propose/applique les écarts ; décision = StrongFather | 2 |
| `tool.inventory.production.record` | Enregistre une production (débit composants, crédit produit) ; données recette fournies | 1–2 |
| `tool.pos.label.print` | Imprime une étiquette code-barres (données fournies) | 1 |
| `tool.inventory.history.list` | Liste l'historique des mouvements (filtres fournis) | 0–1 |
| `tool.inventory.valuation.report` | Retourne un rapport de valorisation (coût / marge potentielle) ; lecture seule | 0–1 |

---

## 4. Cartographie Employés et plannings → Miyukini COG

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Ventes par employé** | Données KindMother + Tool analytics | Core + Tool | `tool.analytics.sales.by_employee` (agrégation) ; pas de décision métier. |
| **Droits d'accès (sensible)** | Master Butler + StrongFather | Cores | Permissions et capacités ; pas d'Opérateur « utilisateurs » dédié. |
| **Pointeuse (clock in/out)** | Tools RH | Tools | `tool.hr.time_clock.in`, `tool.hr.time_clock.out` ; heures = KindMother. |
| **Charge / plannings** | Données + Tool ou Opérateur RH | Tools / Opérateur | `tool.hr.schedule.get` (lecture) ; édition = WriteIntent sous StrongFather. |

### 4.1 Outils employés (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.hr.time_clock.in` | Enregistre une entrée (début de shift) | 1–2 |
| `tool.hr.time_clock.out` | Enregistre une sortie (fin de shift) | 1–2 |
| `tool.hr.schedule.get` | Retourne le planning (shifts) pour un employé/période | 0–1 |
| `tool.analytics.sales.by_employee` | Retourne les ventes agrégées par employé (filtres fournis) | 0–1 |

---

## 5. Cartographie Analytics ventes → Miyukini COG

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Tendances ventes** | Tool analytics | Tool | `tool.analytics.sales.trend` (jour/semaine/mois, comparaison) ; données = KindMother. |
| **Articles populaires** | Tool analytics | Tool | `tool.analytics.sales.by_item` ; lecture seule. |
| **Shifts / écarts caisse** | Tools caisse + analytics | Tools | `tool.pos.shift.close` (clôture shift), `tool.analytics.cash.discrepancy` (écart) ; décision = StrongFather. |
| **Historique reçus** | Données KindMother + Tool | Core + Tool | `tool.pos.receipt.list` (déjà en § 2.3). |
| **Rapport taxes** | Tool analytics | Tool | `tool.analytics.tax.report` (données fournies). |
| **Export (tableur)** | Tool export | Tool | `tool.data.export.spreadsheet` (données fournies). |

### 5.1 Outils analytics (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.analytics.sales.trend` | Retourne tendance ventes (période, comparaison) | 0–1 |
| `tool.analytics.sales.by_item` | Retourne ventes par article (top N, filtres) | 0–1 |
| `tool.pos.shift.close` | Clôture un shift caisse (comptage, écart) ; autorisation = StrongFather | 2 |
| `tool.analytics.cash.discrepancy` | Retourne l'écart caisse pour un shift | 0–1 |
| `tool.analytics.tax.report` | Retourne rapport taxes (période, filtres) | 0–1 |
| `tool.data.export.spreadsheet` | Exporte des données en format tableur (données fournies) | 0–1 |

---

## 6. Cartographie CRM et fidélité → Miyukini COG

### 6.1 Fonctionnalités CRM / fidélité et équivalents

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Base clients** | KindMother + Tools CRM | Core + Tools | `tool.crm.customer.get`, `tool.crm.customer.list`, `tool.crm.customer.create`, `tool.crm.customer.update`. |
| **Programme points** | Tools fidélité | Tools | `tool.loyalty.points.grant`, `tool.loyalty.points.redeem`, `tool.loyalty.balance.get` ; règles = StrongFather. |
| **Historique achats client** | Données KindMother + `tool.pos.receipt.list` (filtre client) | Core + Tool | Déjà couvert. |
| **Cartes fidélité (scan)** | Tool résolution | Tool | `tool.loyalty.card.resolve` (retourne client + solde points). |
| **Adresse client (livraison)** | Données KindMother + Tool | Core + Tool | `tool.crm.customer.address.get` ou champs client. |
| **Notes clients** | Tool CRM | Tool | `tool.crm.customer.note.add`, `tool.crm.customer.note.list`. |

### 6.2 Opérateur Fidélité / CRM

| Attribut | Valeur |
|----------|--------|
| **Type** | Opérateur de Domaine |
| **Rôle** | Gère la base clients, les points fidélité, les cartes et l'historique d'achats pour personnaliser le service. |
| **Service perçu** | « Programme fidélité / CRM caisse » |
| **Tools utilisés** | `tool.crm.customer.*`, `tool.loyalty.*`, `tool.pos.receipt.list` (par client). |
| **Données** | KindMother (clients, adresses, notes, points, transactions). |
| **Gouvernance** | BondingBrother → Master Butler → WorrySentinel → Caring Nanny → StrongFather. |

### 6.3 Outils CRM et fidélité (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.crm.customer.get` | Retourne un client par identifiant | 0–1 |
| `tool.crm.customer.list` | Liste les clients (filtres, recherche) | 0–1 |
| `tool.crm.customer.create` | Crée un client à partir de données fournies | 1–2 |
| `tool.crm.customer.update` | Met à jour un client | 1–2 |
| `tool.crm.customer.address.get` | Retourne l'adresse (livraison) du client | 0–1 |
| `tool.crm.customer.note.add` | Ajoute une note à un client | 1–2 |
| `tool.crm.customer.note.list` | Liste les notes d'un client | 0–1 |
| `tool.loyalty.points.grant` | Accorde des points (règles fournies ou gouvernées) | 1–2 |
| `tool.loyalty.points.redeem` | Déduit des points (échange) ; autorisation = StrongFather | 1–2 |
| `tool.loyalty.balance.get` | Retourne le solde points d'un client | 0–1 |
| `tool.loyalty.card.resolve` | Résout une carte fidélité (code/QR) → client + solde | 0–1 |

---

## 7. Cartographie Restaurant / Bar → Miyukini COG

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Imprimantes cuisine** | Tool impression | Tool | `tool.pos.kitchen.print` ou `tool.print.send` (destination cuisine) ; contenu = données commande. |
| **Affichage cuisine (KDS)** | Opérateur d'Interface ou Tool | Opérateur / Tool | Affichage des ordres en cours ; `tool.pos.kitchen.order.push`, `tool.pos.kitchen.order.update_status` ; données = flux. |
| **Options (sur place / à emporter / livraison)** | Déjà en § 2.3 | Tool | `tool.pos.order.service_type.set`. |
| **Tickets prédéfinis (Table 1, 2…)** | Déjà en § 2.3 | Tool | `tool.pos.ticket.preset.assign`. |

### 7.1 Outils cuisine (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.pos.kitchen.print` | Envoie la commande à l'imprimante cuisine (données fournies) | 1 |
| `tool.pos.kitchen.order.push` | Envoie un ordre à l'affichage cuisine | 1 |
| `tool.pos.kitchen.order.update_status` | Met à jour le statut d'un ordre cuisine (en cours, prêt) | 1 |

---

## 8. Cartographie Paiements → Miyukini COG

| Fonctionnalité PoS | Équivalent Miyukini | Type | Détail |
|--------------------|----------------------|------|--------|
| **Espèces** | Tool enregistrement | Tool | `tool.pos.payment.cash.record` ; montant + session ; KindMother. |
| **Chèque** | Tool enregistrement | Tool | `tool.pos.payment.check.record`. |
| **Carte (intégrée SumUp, Zettle…)** | Adaptateur / Opérateur périmètre | Tool / Opérateur | `tool.payment.terminal.authorize`, `tool.payment.terminal.capture` ; intégrations externes sous gouvernance (WorrySentinel, StrongFather). |
| **Partage d'addition** | Tool partage | Tool | `tool.pos.payment.split` (répartition montants/moyens fournie) ; autorisation = StrongFather. |

### 8.1 Outils paiement (liste canonique)

| ToolId | Description courte | Niveau sécurité typique |
|--------|---------------------|--------------------------|
| `tool.pos.payment.cash.record` | Enregistre un paiement espèces | 2 |
| `tool.pos.payment.check.record` | Enregistre un paiement chèque | 2 |
| `tool.pos.payment.split` | Répartit le paiement entre plusieurs moyens (données fournies) | 2 |
| `tool.payment.terminal.authorize` | Demande une autorisation à un terminal CB (données fournies) | 2–3 |
| `tool.payment.terminal.capture` | Confirme (capture) un paiement CB précédemment autorisé | 2–3 |

---

## 9. Synthèse — Kits d'outils (Toolkits)

Les Kits d'outils sont des **compositions officielles d'Outils**, déclarées au Master Butler, sans logique métier. Ils orchestrent des Tools existants pour efficience et cohérence.

| ToolkitId | Domaine | Composition (résumé) | Usage principal |
|-----------|---------|----------------------|------------------|
| `toolkit.pos.sales` | pos | tool.pos.sale.*, ticket.*, receipt.*, discount.*, refund.*, cash.*, context.*, item.variant.*, item.modifier.*, barcode.*, display.*, order.service_type | Caisse : ventes, tickets, reçus, remises, remboursements, caisse, contexte magasin |
| `toolkit.pos.inventory` | pos / inventory | tool.inventory.*, tool.pos.label.print | Inventaire : stock, transferts, comptages, production, alertes, bons de commande |
| `toolkit.pos.analytics` | pos / analytics | tool.analytics.*, tool.pos.shift.*, tool.data.export.* | Analytics ventes : tendances, par article, shifts, taxes, export |
| `toolkit.pos.loyalty` | pos / crm | tool.crm.customer.*, tool.loyalty.* | Fidélité et CRM : clients, points, cartes, notes |
| `toolkit.pos.kitchen` | pos | tool.pos.kitchen.*, tool.pos.order.service_type.*, tool.pos.ticket.preset.* | Restaurant / bar : cuisine, affichage, type de service |
| `toolkit.pos.payment` | pos / payment | tool.pos.payment.*, tool.payment.terminal.* | Paiements : espèces, chèque, partage ; terminaux CB = adaptateurs |

**Invariant :** Chaque Toolkit contient au moins deux Tools. Les Toolkits sont validés par Ever Buddy (cycle de vie, versions) et déclarés au Master Butler.

---

## 10. Synthèse — Outils (Tools) catalogue complet

Format ToolId : `tool.<domain>.<action>` ou `tool.<domain>.<sous-domaine>.<action>` (conforme Master Butler).

### 10.1 Caisse (pos)

| ToolId | Action courte |
|--------|----------------|
| `tool.pos.sale.create` | Crée une vente |
| `tool.pos.sale.add_item` | Ajoute une ligne à une vente |
| `tool.pos.sale.remove_item` | Retire une ligne |
| `tool.pos.ticket.open` | Ouvre un ticket |
| `tool.pos.ticket.save` | Sauvegarde un ticket |
| `tool.pos.ticket.close` | Clôture un ticket |
| `tool.pos.ticket.list` | Liste les tickets |
| `tool.pos.ticket.preset.assign` | Assigne un libellé prédéfini |
| `tool.pos.discount.apply` | Applique une remise |
| `tool.pos.refund.record` | Enregistre un remboursement |
| `tool.pos.receipt.render` | Produit le contenu du reçu |
| `tool.pos.receipt.print` | Imprime le reçu |
| `tool.pos.receipt.send` | Envoie le reçu par email |
| `tool.pos.receipt.list` | Liste les reçus |
| `tool.pos.item.variant.resolve` | Résout une variante article |
| `tool.pos.item.modifier.apply` | Applique des modificateurs |
| `tool.pos.cash.register.open` | Ouvre une session caisse |
| `tool.pos.cash.register.close` | Clôture une session caisse |
| `tool.pos.cash.movement.record` | Enregistre un mouvement espèces |
| `tool.pos.barcode.parse` | Parse un code-barres |
| `tool.pos.context.store.resolve` | Résout le magasin courant |
| `tool.pos.display.push` | Envoie les données à l'écran client |
| `tool.pos.order.service_type.set` | Définit le type de service |
| `tool.pos.shift.close` | Clôture un shift caisse |
| `tool.pos.label.print` | Imprime une étiquette code-barres |
| `tool.pos.kitchen.print` | Envoie à l'imprimante cuisine |
| `tool.pos.kitchen.order.push` | Envoie un ordre à la cuisine |
| `tool.pos.kitchen.order.update_status` | Met à jour le statut ordre cuisine |
| `tool.pos.payment.cash.record` | Enregistre paiement espèces |
| `tool.pos.payment.check.record` | Enregistre paiement chèque |
| `tool.pos.payment.split` | Répartit le paiement |

### 10.2 Inventaire (inventory)

| ToolId | Action courte |
|--------|----------------|
| `tool.inventory.stock.get` | Retourne le stock |
| `tool.inventory.stock.adjust` | Ajuste le stock |
| `tool.inventory.import.items` | Importe des articles |
| `tool.inventory.alert.low.evaluate` | Évalue les alertes stock bas |
| `tool.inventory.purchase_order.create` | Crée un bon de commande |
| `tool.inventory.purchase_order.update` | Met à jour un bon de commande |
| `tool.inventory.purchase_order.track` | Suivi bon de commande |
| `tool.inventory.transfer.create` | Crée un transfert |
| `tool.inventory.transfer.execute` | Exécute un transfert |
| `tool.inventory.transfer.list` | Liste les transferts |
| `tool.inventory.count.start` | Démarre un inventaire physique |
| `tool.inventory.count.record` | Enregistre un comptage |
| `tool.inventory.count.reconcile` | Clôture et réconcilie un comptage |
| `tool.inventory.production.record` | Enregistre une production |
| `tool.inventory.history.list` | Liste l'historique des mouvements |
| `tool.inventory.valuation.report` | Rapport de valorisation |

### 10.3 Analytics (analytics)

| ToolId | Action courte |
|--------|----------------|
| `tool.analytics.sales.trend` | Tendance ventes |
| `tool.analytics.sales.by_item` | Ventes par article |
| `tool.analytics.sales.by_employee` | Ventes par employé |
| `tool.analytics.cash.discrepancy` | Écart caisse pour un shift |
| `tool.analytics.tax.report` | Rapport taxes |
| `tool.data.export.spreadsheet` | Export tableur |

### 10.4 CRM et fidélité (crm, loyalty)

| ToolId | Action courte |
|--------|----------------|
| `tool.crm.customer.get` | Retourne un client |
| `tool.crm.customer.list` | Liste les clients |
| `tool.crm.customer.create` | Crée un client |
| `tool.crm.customer.update` | Met à jour un client |
| `tool.crm.customer.address.get` | Adresse livraison client |
| `tool.crm.customer.note.add` | Ajoute une note client |
| `tool.crm.customer.note.list` | Liste les notes client |
| `tool.loyalty.points.grant` | Accorde des points |
| `tool.loyalty.points.redeem` | Déduit des points |
| `tool.loyalty.balance.get` | Solde points client |
| `tool.loyalty.card.resolve` | Résout une carte fidélité |

### 10.5 Paiement et terminaux (payment)

| ToolId | Action courte |
|--------|----------------|
| `tool.payment.terminal.authorize` | Autorisation terminal CB |
| `tool.payment.terminal.capture` | Capture paiement CB |

### 10.6 RH (hr)

| ToolId | Action courte |
|--------|----------------|
| `tool.hr.time_clock.in` | Entrée (début shift) |
| `tool.hr.time_clock.out` | Sortie (fin shift) |
| `tool.hr.schedule.get` | Planning employé |

---

## 11. Synthèse — Opérateurs catalogue complet

| Opérateur | Type | Service perçu | Tools principaux |
|-----------|------|----------------|------------------|
| **Caisse (Point of Sale)** | Interface / Domaine | Caisse / vente au détail | tool.pos.sale.*, ticket.*, receipt.*, discount.*, refund.*, cash.*, item.*, barcode.*, display.* ; MiyuWeb pour rendu |
| **Inventaire** | Domaine | Inventaire multi-magasin | tool.inventory.*, tool.pos.label.print |
| **Fidélité / CRM** | Domaine | Programme fidélité / CRM caisse | tool.crm.customer.*, tool.loyalty.*, tool.pos.receipt.list (par client) |
| **Analytics** | Domaine (optionnel) | Rapports et tendances ventes | tool.analytics.*, tool.pos.shift.*, tool.data.export.* |
| **Restaurant (cuisine)** | Interface / Domaine (optionnel) | Affichage cuisine, tickets tables | tool.pos.kitchen.*, tool.pos.order.service_type.*, tool.pos.ticket.preset.* |
| **Paiement** | Périmètre ou Domaine (optionnel) | Enregistrement paiements, terminaux CB | tool.pos.payment.*, tool.payment.terminal.* |
| **RH / Employés** | Domaine (optionnel) | Plannings, pointeuse, ventes par employé | tool.hr.*, tool.analytics.sales.by_employee ; permissions = Master Butler + StrongFather |

*Identité et droits employés : MiyuAuth + Cores (Master Butler, StrongFather) — pas d'Opérateur « utilisateurs » dédié.*

---

## 12. Synthèse — Services utilisateur et Équipes d'Opérateurs

### 12.1 Services PoS spécialisés

Un **Service** est ce que l'utilisateur perçoit. Il peut être porté par un seul Opérateur ou par une **Équipe d'Opérateurs** sous **Contrat d'équipe** et **Mandat de Permission**.

| Service PoS spécialisé | Opérateur(s) / Équipe | Toolkits impliqués |
|-------------------------|------------------------|--------------------|
| **Caisse retail** | Caisse + (Paiement dans flux) | toolkit.pos.sales, toolkit.pos.payment |
| **Caisse + inventaire** | Caisse + Inventaire | toolkit.pos.sales, toolkit.pos.inventory |
| **Caisse + fidélité** | Caisse + Fidélité / CRM | toolkit.pos.sales, toolkit.pos.loyalty |
| **Restaurant complet** | Caisse + Restaurant (cuisine) + (Fidélité optionnel) | toolkit.pos.sales, toolkit.pos.kitchen, toolkit.pos.loyalty |
| **Multi-magasin + analytics** | Caisse (par site) + Inventaire + Analytics | toolkit.pos.sales, toolkit.pos.inventory, toolkit.pos.analytics |
| **Back-office PoS** | Inventaire + Analytics + (RH optionnel) | toolkit.pos.inventory, toolkit.pos.analytics |

### 12.2 Équipes d'Opérateurs et Contrats d'équipe

Pour délivrer un **Service** combiné (ex. caisse + inventaire + fidélité), on constitue une **Équipe d'Opérateurs** liée par un **Contrat d'équipe** (membres, flux autorisés, types d'échanges, niveau de validation). L'équipe n'existe opérationnellement que sous un **Mandat de Permission** émis par StrongFather.

**Exemple — Équipe « Caisse retail complète » :**

| Membre | Rôle dans l'équipe |
|--------|---------------------|
| Opérateur Caisse | Enregistre ventes, tickets, reçus ; envoie lignes de vente à Inventaire pour déstockage |
| Opérateur Inventaire | Met à jour le stock après vente ; fournit alertes stock bas ; reçoit flux ventes via BondingBrother |
| Opérateur Fidélité / CRM | Fournit client et solde points en caisse ; accorde points après achat ; reçoit flux ventes pour historique |
| Opérateur Paiement (ou Tools dans flux) | Enregistre paiements espèces/CB ; partage d'addition |

**Flux autorisés (exemple) :** Caisse → BondingBrother → Inventaire (déstockage) ; Caisse → BondingBrother → Fidélité (points, historique) ; Caisse → Paiement (enregistrement).

**Règle (Mandats et Équipes Opérateurs) :** *« Aucun Opérateur ne parle librement à un autre. »* Toute communication passe par BondingBrother, est définie dans le Contrat d'équipe et autorisée par un Mandat de Permission. *« Le contrat est validé UNE FOIS, pas à chaque appel. »*

**Référence :** [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md).

---

## 13. Patterns : hors ligne, multi-magasin, intégrations

### 13.1 Vente hors ligne et synchronisation

- **Hors ligne** : Pas de Tool dédié « offline ». L'état système (Caring Nanny) et la politique (WorrySentinel) déterminent si les Tools peuvent s'exécuter avec mise en file ou refus. En mode dégradé (réseau faible), la gouvernance peut autoriser l'exécution locale avec **WriteIntent différée**.
- **Synchronisation** : Lors du rétablissement de la connexion, les WriteIntent en attente sont soumises à KindMother via le flux gouverné. StrongFather valide la cohérence (pas de double enregistrement, pas de conflit). La décision de fusion ou rejet reste à StrongFather ; les Tools ne font qu'exécuter la persistance mandatée.
- **Invariant** : Le Kernel et les Cores n'imposent pas de protocole réseau ; l'autonomie (LOI-1, LOI-2) s'applique : le système accepte l'isolement comme état normal.

### 13.2 Multi-magasin

- **Site / magasin** = périmètre de données (KindMother) ; identifié par `tool.pos.context.store.resolve`. Chaque vente, mouvement caisse, ajustement stock est rattaché à un site.
- **Transferts entre magasins** : `tool.inventory.transfer.create` / `execute` ; autorisation StrongFather. Aucun Opérateur ne peut transférer sans Mandat.
- **Analytics multi-sites** : Données agrégées par site ou consolidées ; Tools analytics reçoivent le périmètre (site ou tous) dans le flux.

### 13.3 Intégrations (compta, e-commerce, API)

- **Comptabilité** : Les écritures comptables (ventes, paiements, stocks) sont des **données exportées ou répliquées** sous gouvernance. Un Opérateur de Domaine « Export compta » ou un flux BondingBrother → Bridge inter-COG peut envoyer des écritures vers un système tiers ; StrongFather autorise le flux ; KindMother reste la source de vérité locale.
- **E-commerce** : Catalogue et commandes peuvent être synchronisés avec un Opérateur Commerce (voir [Équivalents Boutique CMS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md)) ; Contrat d'équipe Caisse ↔ Commerce ; flux via BondingBrother.
- **API externe** : Toute exposition (Loyverse API–style) est une **Façade Publique Gouvernée** ou un **Visa de Connexion** si l'appelant est un Utilisateur Visiteur. Les capacités exposées sont des Tools déclarés au Master Butler ; l'API ne contourne pas la gouvernance.

---

## 14. Cores impliqués et flux de gouvernance

Les Cores **ne font jamais d'exécution** ; ils gouvernent, décident ou observent.

| Core | Rôle dans le périmètre PoS |
|------|----------------------------|
| **KindMother** | Autorité sur toutes les données : ventes, tickets, reçus, stock, mouvements caisse, clients, points fidélité, plannings, shifts. WriteIntent pour toute écriture. |
| **StrongFather** | Décision finale ALLOW/DENY : remboursements, ajustements stock, clôture shift, partage paiement, octroi/rédemption points, transferts. Émission et révocation des Mandats de Permission. Validation des Contrats d'équipe. |
| **Master Butler** | Déclaration des Tools et Toolkits PoS ; permissions et capabilities. Catalogue des capacités (pos.sales, inventory, loyalty, analytics, payment). |
| **BondingBrother** | Médiation des intentions (caissier, gestionnaire stock, client fidélité) ; traduction vers les Cores et les Opérateurs. |
| **WorrySentinel** | Niveau de sécurité (paiements, données sensibles, terminaux CB) ; blocage si menace ou état dégradé. |
| **Caring Nanny** | État système (HEALTHY, DEGRADED, etc.) ; blocage des Tools si environnement dégradé ; hors ligne = politique d'autorisation. |
| **Ever Buddy** | Cycle de vie : versions des Tools/Toolkits PoS, dépréciation, compatibilité. |
| **Border Guard** | Frontières et niveaux de confiance ; multi-magasin ou multi-COG (franchise) si applicable. |
| **TAMR** | Points d'intervention humaine : arbitrage remboursement, écart caisse, alerte stock, validation manuelle. |

**Flux générique :** Opérateur → BondingBrother → Master Butler (existence Tool, permissions) → WorrySentinel (niveau sécurité) → Caring Nanny (état système) → StrongFather (ALLOW/DENY) → Exécution Tool ; toute persistance passe par WriteIntent KindMother.

---

## 15. Recoupements avec Équivalents Boutique CMS

Les services PoS et les services **boutique en ligne** (WooCommerce-like) partagent des concepts ; la différence est le **contexte d'exécution** (caisse physique vs. panier web).

| Concept | PoS (ce document) | Boutique / Commerce ([Équivalents Boutique CMS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md)) |
|---------|-------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| **Catalogue articles** | Données KindMother ; `tool.pos.item.variant.resolve` ; peut réutiliser `tool.commerce.product.*` si même modèle données | `tool.commerce.product.list`, `tool.commerce.product.resolve`, `tool.commerce.product.variations` |
| **Paiement** | `tool.pos.payment.*`, `tool.payment.terminal.*` (espèces, CB terminal) | `tool.commerce.payment.capture`, `tool.commerce.payment.refund`, `tool.commerce.payment.status` |
| **Client** | `tool.crm.customer.*`, `tool.loyalty.*` | Peut partager les mêmes Tools CRM si même base clients |
| **Commande / Vente** | Ticket = ordre en caisse ; `tool.pos.sale.*`, `tool.pos.ticket.*` | Commande e-commerce ; `tool.commerce.order.*` |

Un **même environnement COG** peut héberger à la fois l'Opérateur Caisse (PoS) et l'Opérateur Commerce (boutique en ligne) ; les données catalogue et clients sont alors partagées sous KindMother ; les flux sont distincts (caisse vs. checkout web).

---

## 16. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](./Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](./Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Opérateurs et Terminologie | [Miyukini Conceptual References - Operators et Terminologie](./Miyukini%20Conceptual%20References%20-%20Operators%20et%20Terminologie.md) |
| Mandats et Équipes Opérateurs | [Miyukini Conceptual References - Mandats et Equipes Operators](./Miyukini%20Conceptual%20References%20-%20Mandats%20et%20Equipes%20Operators.md) |
| Pyramide Architecture Complète | [Miyukini Conceptual References - Pyramide Architecture Complete](./Miyukini%20Conceptual%20References%20-%20Pyramide%20Architecture%20Complete.md) |
| Définition COG | [Miyukini Conceptual References - Definition COG](./Miyukini%20Conceptual%20References%20-%20Definition%20COG.md) |
| Équivalents Boutique CMS | [Miyukini Conceptual References - Equivalents Boutique CMS Reservation SaaS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md) |
| Objectif Final | [Miyukini Conceptual References - Objectif Final](./Miyukini%20Conceptual%20References%20-%20Objectif%20Final.md) |
| Lois Autonomie Système | [Miyukini Conceptual References - Lois Autonomie Systeme](./Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence normatif — Équivalents PoS et logiciel de caisse pour services spécialisés Miyukini COG
