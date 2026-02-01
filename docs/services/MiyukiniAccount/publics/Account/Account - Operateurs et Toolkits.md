# Miyukini Account — Besoins en Opérateurs et Toolkits (point d’entrée entreprise)

## Contexte

Ce document décrit les **besoins en Opérateurs** (Strate 7) et en **Toolkits** (Strate 6) du point d’entrée **Miyukini Account** (entreprise) du service COG Miyukini Account. Il s’appuie sur l’[analyse des besoins](./Account%20-%20Analyse%20des%20besoins.md) et le document [Parcours, capacités et livrables](./Account%20-%20Parcours%20Capacites%20Livrables.md). Il vise à fournir une **réponse explicite** pour chaque besoin : **Service**, **Opérateur** (ou Équipe d’Opérateurs / Contrat d’équipe), **Toolkit**.

## Portée / Scope

- **Public** : Professionnels, associations, TPE/PME, organisateurs (point d’entrée Miyukini Account).
- **Périmètre** : Identification des Opérateurs et Toolkits nécessaires pour couvrir tous les besoins du public Account (compte, grand livre, devis, facturation, rapports, intégration MFS/JayRDV).
- **Hors périmètre** : Spécifications d’implémentation (API, schémas, code) ; définition détaillée des Cores — référencés dans le glossaire Miyukini.

---

## 1. Référence glossaire Miyukini

| Concept | Définition (Glossaire) |
|---------|-------------------------|
| **Opérateur** | Entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l’utilisateur (Strate 7). |
| **Outil (Tool)** | Capacité exécutable gouvernée, sans autorité, sans décision métier (Strate 6). |
| **Kit d’Outils (Toolkit)** | Composition officielle d’Outils, validée et déclarée par l’environnement (Strate 6). |
| **Équipe d’Opérateurs** | Collectif gouverné d’Opérateurs qui collaborent sous règles explicites (Contrat d’équipe). |
| **Mandat de Permission** | Autorisation déléguée, temporaire et encadrée, émise par StrongFather. |
| **Service** | Capacité perçue par l’utilisateur ; ici le **service COG Miyukini Account** exposé via le point d’entrée **Miyukini Account** (entreprise). |

Les utilisateurs Account **interagissent avec** des Opérateurs gouvernés (tableau de bord, grand livre, devis, facturation, rapports) ; ces Opérateurs s’appuient sur les Toolkits du service COG Miyukini Account et sur Miyauth, Miyunotify. Les **services consommateurs** (MFS, JayRDV) **appellent** les Opérateurs Miyukini Account (quote.create, invoice.emit, budget.movements.record) pour devis, facturation et budget.

---

## 2. Besoins en Opérateurs (point d’entrée Account)

### 2.1 Opérateur « Miyukini Account » (tableau de bord et capacités entreprise)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Exposer le **point d’entrée Account** : tableau de bord (CA, encaissements, factures en attente), grand livre, journal, devis, facturation, relances, encaissements, rapports légaux, export. |
| **Public servi** | Professionnels, associations, TPE/PME, organisateurs authentifiés (point d’entrée Miyukini Account, Master Butler, rôles admin/comptable/lecture seule). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; rôles et permissions (Master Butler, Contrat d’équipe) ; persistance (KindMother) ; sécurité (WorrySentinel niveau 2–3). |
| **Capacités exposées** | Compte Account (Miyauth, identité légale SIRET si exigé) ; rôles et permissions ; mouvements (grand livre, journal, ventilation catégorie/projet) ; devis (création, envoi, suivi, conversion en facture) ; facturation (émission, relances, encaissements) ; rapports (tableaux de bord, rapports légaux, export PDF/CSV) ; intégration MFS/JayRDV (quote.create, invoice.emit, budget.movements.record). |
| **Ne fait pas** | Décision métier des services consommateurs (MFS détient exposants/éditions, JayRDV détient professionnels/RDV) ; Miyukini Account détient les **données comptables** (mouvements, devis, factures). |

Cet Opérateur est le **point d’entrée** principal du public Account : il agrège toutes les capacités du périmètre entreprise et s’appuie sur les Toolkits listés en § 3.

### 2.2 Synthèse des Opérateurs (Account)

| Opérateur | Usage par l’utilisateur | Livrables couverts |
|-----------|-------------------------|---------------------|
| **Miyukini Account** (point d’entrée entreprise) | Connexion, tableau de bord, grand livre, devis, facturation, relances, encaissements, rapports, export ; intégration MFS/JayRDV (appels quote.create, invoice.emit, budget.movements.record). | Tous les besoins MAC-01 à MAC-19. |

*Note :* Selon l’architecture, des sous-Opérateurs (ex. Account Devis, Account Facturation, Account Grand livre) peuvent être distingués au sein de l’Équipe d’Opérateurs Account ; le **Contrat d’équipe** définit les flux entre eux. Pour la traçabilité besoin → réponse, l’**Opérateur** de référence reste « Miyukini Account » (point d’entrée entreprise).

---

## 3. Besoins en Toolkits (point d’entrée Account)

### 3.1 Kit « Compte Account » (Miyauth / Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Authentification et compte entreprise : inscription/compte Account, identité légale (SIRET, etc.) si exigé, rôles (admin, comptable, lecture seule), permissions (Master Butler). |
| **Outils agrégés (exemples)** | `auth.register`, `auth.login`, `auth.logout`, `auth.resetPassword`, `profile.get`, `profile.update` (identité légale), `roles.list`, `permissions.check`. |
| **Consommé par** | Miyukini Account (point d’entrée entreprise). |
| **Composants sous-jacents** | Miyauth, Master Butler (permissions, rôles), WorrySentinel (niveau 2–3). |

### 3.2 Kit « Grand livre et Mouvements Account » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Enregistrement des revenus et dépenses (date, montant, libellé, catégorie, client/fournisseur, pièce justificative) ; ventilation par catégorie/projet/édition ; grand livre et journal ; export pour expert-comptable. |
| **Outils agrégés (exemples)** | `budget.movements.record`, `budget.movements.list`, `budget.movements.import` (CSV), `budget.categories.list`, `budget.projects.list`, `report.ledger`, `report.journal`, `report.export` (CSV/Excel). |
| **Consommé par** | Miyukini Account ; MFS (budget.movements.record avec référence édition). |
| **Composants sous-jacents** | KindMother, WorrySentinel (niveau 2–3). |

### 3.3 Kit « Devis Account » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Création de devis (client, lignes, montants, TVA, conditions, validité) ; envoi (email, lien) ; suivi statut (envoyé, accepté, refusé) ; conversion devis → facture ; intégration MFS/JayRDV (quote.create avec référence métier). |
| **Outils agrégés (exemples)** | `quote.create`, `quote.list`, `quote.get`, `quote.send`, `quote.status.update`, `quote.convertToInvoice`. |
| **Consommé par** | Miyukini Account ; MFS, JayRDV (appel quote.create). |
| **Composants sous-jacents** | KindMother, Miyunotify (envoi, notification), WorrySentinel (niveau 2–3). |

### 3.4 Kit « Facturation Account » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Émission de factures (client, lignes, montants, TVA, numérotation, conditions de paiement) ; conformité (TVA, numérotation selon juridiction) ; relances (factures impayées, rappels, escalade) ; suivi des encaissements (statut payé/partiel/impayé) ; pas de stockage RIB/carte en clair (token ou référence opaque) ; intégration MFS/JayRDV (invoice.emit). |
| **Outils agrégés (exemples)** | `invoice.emit`, `invoice.list`, `invoice.get`, `invoice.pdf`, `invoice.remind`, `payment.record`, `payment.linkToInvoice`, `payment.status`. |
| **Consommé par** | Miyukini Account ; MFS, JayRDV (appel invoice.emit). |
| **Composants sous-jacents** | KindMother, Miyunotify (relances), WorrySentinel (niveau 2–3), conformité PCI-DSS. |

### 3.5 Kit « Rapports et Export Account » (Miyukini Account)

| Attribut | Description |
|----------|-------------|
| **Rôle** | Tableaux de bord (CA, encaissements, factures en attente, répartition par catégorie/projet) ; rapports légaux (bilan, compte de résultat, journal, grand livre) ; export PDF/CSV pour conformité et expert-comptable ; pas d’export de données de paiement brutes ; audit de l’export. |
| **Outils agrégés (exemples)** | `report.balance`, `report.dashboard`, `report.byCategory`, `report.byProject`, `report.legal` (bilan, compte de résultat), `report.export.pdf`, `report.export.csv`, `report.export.audit`. |
| **Consommé par** | Miyukini Account ; MFS (vue budget organisateur : rapports par édition). |
| **Composants sous-jacents** | KindMother, WorrySentinel (niveau 2–3). |

### 3.6 Synthèse des Toolkits (Account)

| Toolkit | Opérateur(s) consommateur(s) | Livrables couverts |
|---------|-----------------------------|---------------------|
| **Compte Account** | Miyukini Account | MAC-01, MAC-02, MAC-03 (compte, rôles, données niveau 2–3). |
| **Grand livre et Mouvements Account** | Miyukini Account, MFS | MAC-04, MAC-05, MAC-06, MAC-18, MAC-19 (mouvements, ventilation, grand livre, budget par édition MFS). |
| **Devis Account** | Miyukini Account, MFS, JayRDV | MAC-07, MAC-08, MAC-09, MAC-10. |
| **Facturation Account** | Miyukini Account, MFS, JayRDV | MAC-11, MAC-12, MAC-13, MAC-14. |
| **Rapports et Export Account** | Miyukini Account, MFS | MAC-15, MAC-16, MAC-17, MAC-19 (vue budget organisateur). |

---

## 4. Matrice Besoin → Service / Opérateur / Toolkit (exhaustive)

Chaque besoin dispose d’une **réponse explicite** par **Service**, **Opérateur** et **Toolkit**.

### 4.1 Besoins fonctionnels (MAC-01 à MAC-19)

| Id besoin | Besoin | Service | Opérateur | Toolkit(s) |
|-----------|--------|---------|-----------|------------|
| **MAC-01** | Compte entreprise (Account) | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Compte Account (Miyauth) |
| **MAC-02** | Rôles et permissions | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Compte Account (Master Butler, Contrat d’équipe) |
| **MAC-03** | Données niveau 2–3 | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Gouvernance (WorrySentinel, KindMother) — appliqué à tous les Toolkits Account |
| **MAC-04** | Enregistrement des mouvements | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Grand livre et Mouvements Account |
| **MAC-05** | Ventilation par catégorie/projet | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Grand livre et Mouvements Account |
| **MAC-06** | Grand livre et journal | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Grand livre et Mouvements Account |
| **MAC-07** | Création de devis | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Devis Account |
| **MAC-08** | Envoi et suivi des devis | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Devis Account |
| **MAC-09** | Conversion devis → facture | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Devis Account, Facturation Account |
| **MAC-10** | Intégration MFS/JayRDV (devis) | Miyukini Account (COG) | Miyukini Account (appelé par MFS, JayRDV) | Devis Account (quote.create) |
| **MAC-11** | Émission de factures | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Facturation Account |
| **MAC-12** | Relances | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Facturation Account |
| **MAC-13** | Suivi des encaissements | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Facturation Account |
| **MAC-14** | Intégration MFS/JayRDV (facturation) | Miyukini Account (COG) | Miyukini Account (appelé par MFS, JayRDV) | Facturation Account (invoice.emit) |
| **MAC-15** | Tableaux de bord | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Rapports et Export Account |
| **MAC-16** | Rapports légaux | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Rapports et Export Account |
| **MAC-17** | Export pour tiers | Miyukini Account (COG), point d’entrée Miyukini Account | Miyukini Account | Rapports et Export Account |
| **MAC-18** | Budget par édition (MFS) | Miyukini Account (COG) | Miyukini Account (appelé par MFS) | Grand livre et Mouvements Account (budget.movements.record) |
| **MAC-19** | Vue budget organisateur (MFS) | Miyukini Account (COG) | Miyukini Account (données fournies à MFS) | Rapports et Export Account (rapports par édition) |

### 4.2 Besoins non fonctionnels (NFR-MAC-01 à NFR-MAC-08)

| Id besoin | Besoin | Service | Opérateur | Toolkit / gouvernance |
|-----------|--------|---------|-----------|------------------------|
| **NFR-MAC-01** | Données niveau 2–3 (Sensitive à Critical) | Miyukini Account (COG) | Miyukini Account | WorrySentinel, tous les Toolkits Account (flux chiffrés, résidence centralisée) |
| **NFR-MAC-02** | Résidence centralisée | Miyukini Account (COG) | Miyukini Account | KindMother, contrat du service (COG de référence) |
| **NFR-MAC-03** | Pas de stockage des données de paiement en clair | Miyukini Account (COG) | Miyukini Account | Facturation Account (token ou référence opaque, PCI-DSS) |
| **NFR-MAC-04** | Audit complet | Miyukini Account (COG) | Miyukini Account | WorrySentinel, Master Butler (traçabilité lectures/écritures, révocation) |
| **NFR-MAC-05** | Conformité facturation | Miyukini Account (COG) | Miyukini Account | Facturation Account (numérotation, TVA, mentions légales) |
| **NFR-MAC-06** | Conformité rapports | Miyukini Account (COG) | Miyukini Account | Rapports et Export Account (bilan, compte de résultat selon juridiction) |
| **NFR-MAC-07** | Temps de chargement des rapports | Miyukini Account (COG) | Miyukini Account | Rapports et Export Account (performance) |
| **NFR-MAC-08** | Émission de facture (< 3 s) | Miyukini Account (COG) | Miyukini Account | Facturation Account |

---

## 5. Matrice Parcours / Livrables / Opérateur / Toolkits

| Parcours ou livrable | Opérateur | Toolkit(s) | Service |
|----------------------|-----------|------------|---------|
| Onboarding (compte Account) | Miyukini Account | Compte Account (Miyauth) | Miyukini Account (COG), point d’entrée Miyukini Account |
| Tableau de bord (CA, encaissements, factures en attente) | Miyukini Account | Rapports et Export Account | Miyukini Account (COG), point d’entrée Miyukini Account |
| Grand livre, journal, mouvements | Miyukini Account | Grand livre et Mouvements Account | Miyukini Account (COG), point d’entrée Miyukini Account |
| Devis (création, envoi, suivi, conversion) | Miyukini Account | Devis Account | Miyukini Account (COG), point d’entrée Miyukini Account |
| Facturation (émission, relances, encaissements) | Miyukini Account | Facturation Account | Miyukini Account (COG), point d’entrée Miyukini Account |
| Rapports légaux, export expert-comptable | Miyukini Account | Rapports et Export Account | Miyukini Account (COG), point d’entrée Miyukini Account |
| Intégration MFS (devis, factures, budget édition) | Miyukini Account (appelé par MFS) | Devis Account, Facturation Account, Grand livre et Mouvements Account | Miyukini Account (COG) |
| Intégration JayRDV (devis, factures) | Miyukini Account (appelé par JayRDV) | Devis Account, Facturation Account | Miyukini Account (COG) |

---

## 6. Dépendances (composants Miyukini)

| Besoin | Composant | Rôle |
|--------|-----------|------|
| Compte, session, rôles | Miyauth, Master Butler | Authentification, rôles (admin, comptable, lecture seule), permissions. |
| Mandat | StrongFather | Émission Mandat de Permission pour Account. |
| Persistance, résidence | KindMother | Données mouvements, devis, factures ; résidence centralisée sur COG de référence. |
| Niveau 2–3, audit | WorrySentinel | Classification niveau 2–3, traçabilité, révocation. |
| Relances, notifications | Miyunotify | Envoi relances, notifications (devis envoyé, facture payée). |
| Intégration MFS/JayRDV | MFS, JayRDV | Données métier (exposant, professionnel, édition) ; appels quote.create, invoice.emit, budget.movements.record. |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Account - Analyse des besoins](./Account%20-%20Analyse%20des%20besoins.md) | Liste exhaustive des besoins MAC-01 à MAC-19, NFR-MAC-01 à NFR-MAC-08. |
| [Account - Parcours Capacites Livrables](./Account%20-%20Parcours%20Capacites%20Livrables.md) | Parcours, capacités et livrables Account. |
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée Purse/Account. |
| [Integration Services](../../reference/Miyukini%20Account%20-%20Integration%20Services.md) | Flux MFS, JayRDV, responsabilités. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Account, capacités exposées. |
| [Glossaire Miyukini](../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Opérateur, Toolkit, Mandat, Service. |

---

**Document** : Miyukini Account — Operateurs et Toolkits (point d’entrée entreprise)  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (réponse besoin → Service / Opérateur / Toolkit)
