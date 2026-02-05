# JayKonta — Intégration avec les autres services

## Contexte

Le **service COG JayKonta** expose des **Opérateurs** et **Kits d’outils** pour la comptabilité multi-échelle (budgets, devis, facturation, rapports). Les **services métier** (JayFestival, JayRDV, futurs services) **consomment** JayKonta pour éviter la duplication de la logique budget/facturation et garantir une cohérence gouvernée.

Ce document décrit le **modèle d’intégration** : services consommateurs identifiés, flux de données, responsabilités (données métier vs données comptables), et règles de résidence et de sécurité.

## Portée / Scope

- **Périmètre** : Intégration de JayKonta avec JayFestival, JayRDV, JayKoa (optionnel), futurs services.
- **Hors périmètre** : Spécifications techniques détaillées des API et contrats d’Opérateurs (référencés dans d’autres documents).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisée** — Document fondateur JayKonta, Points d’entrée JayBudget et JayKonta, Document fondateur JayFestival, Document fondateur JayRDV, Politique de résidence. **Contraintes** : ne pas fusionner avec le Document fondateur ni avec les analyses des besoins ; ne pas anticiper les contrats d’API détaillés.

---

## 1. Services consommateurs identifiés

### 1.1 Tableau récapitulatif

| Service | Usage de JayKonta | Données concernées | Point d’entrée |
|---------|---------------------------|--------------------|----------------|
| **JayFestival** | Budget par édition, devis et factures exposants, ventilation revenus/dépenses | Budget édition, facturation exposants | JayKonta (entreprise) |
| **JayRDV** | Facturation professionnels, abonnements, encaissements | Factures, encaissements | JayKonta (entreprise) |
| **JayKoa** | Optionnel : rappels ou jalons liés à des échéances budget (échéances factures, objectifs) | Références temporelles, pas de données financières canoniques | N/A (références uniquement) |
| **Futurs services** | Tout service nécessitant devis, facturation ou suivi de budget | À définir par service | Purse ou Account selon contexte |

### 1.2 JayFestival (JayFestival)

| Besoin | Description |
|--------|-------------|
| **Budget par édition** | Revenus/dépenses par édition (stands, inscriptions, prestations), ventilation (Miyucptaledger, Miyuexpense, Miyucomptareports — alignés sur JayKonta). |
| **Devis et factures exposants** | Émission de devis et factures aux exposants (emplacements, options), suivi des encaissements. |
| **Rapports** | Synthèses budget par édition, rapports pour les organisateurs. |

**Responsabilité** : JayFestival détient les **données métier** (qui est exposant, quelle édition, quel stand) ; JayKonta détient les **données comptables** (mouvements, devis, factures, montants) et applique les règles de résidence et de sécurité. La **liaison** (ex. facture ↔ exposant, facture ↔ édition) est gérée par des références opaques ou des identifiants métier validés par StrongFather/Master Butler.

### 1.3 JayRDV

| Besoin | Description |
|--------|-------------|
| **Facturation professionnels** | Facturation des prestations (RDV, abonnements), émission de factures, relances. |
| **Encaissements** | Suivi des encaissements, moyens de paiement (tokens, pas de stockage RIB/carte en clair). |

**Responsabilité** : JayRDV détient les **données métier** (qui est professionnel, quels RDV, quels abonnements) ; JayKonta gère l’**émission des factures** et le **suivi des encaissements** selon Mandat et niveau de sécurité.

### 1.4 JayKoa (optionnel)

| Besoin | Description |
|--------|-------------|
| **Rappels / jalons** | Rappels ou jalons liés à des échéances budget (ex. échéance facture, objectif d’épargne). | JayKoa ne détient **pas** la copie canonique des données financières ; il reçoit des **références** (date, type d’échéance, identifiant opaque) pour afficher des rappels dans l’agenda. La **source de vérité** reste JayKonta / KindMother. |

---

## 2. Modèle d’intégration

### 2.1 Principe

- **JayKonta** (COG) expose des **Opérateurs** et **Kits** (ex. : `budget.movements.record`, `quote.create`, `invoice.emit`, `report.balance`, `report.export`).
- Chaque **service consommateur** (JayFestival, JayRDV) :
  - détient les **données métier** (identité des acteurs, contrats, éditions, RDV) ;
  - **appelle** JayKonta pour enregistrer des mouvements, émettre des devis/factures, produire des rapports, selon **Mandat de Permission** et **permissions** (Master Butler) ;
  - **déclare** le niveau de sécurité (WorrySentinel) des données qu’il transmet ; JayKonta applique les règles de résidence, chiffrement et audit en conséquence.
- **KindMother** : la **résidence** des données sensibles (données financières personnelles ou d’entreprise) est définie par le [contrat du service](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) et le point d’entrée (Purse vs Account). Le **COG de référence** pour les données Account (entreprise) est désigné par le contrat (ex. COG du service ou COG de l’organisateur pour JayFestival).

### 2.2 Flux typiques

| Flux | Acteur initiateur | JayKonta | Résultat |
|------|------------------|------------------|----------|
| **Création devis (JayFestival)** | JayFestival (organisateur) | `quote.create` avec référence exposant/édition, montants, TVA | Devis enregistré, identifiant retourné |
| **Émission facture (JayFestival)** | JayFestival | `invoice.emit` avec référence devis ou données facture, client (exposant) | Facture émise, suivi relances/encaissements |
| **Enregistrement mouvement budget (JayFestival)** | JayFestival | `budget.movements.record` par édition, catégorie, montant | Mouvement enregistré, rapports mis à jour |
| **Facturation professionnel (JayRDV)** | JayRDV | `invoice.emit` avec référence professionnel, prestations, montants | Facture émise, encaissement suivi |
| **Rapport balance (Purse ou Account)** | Utilisateur (Purse ou Account) | `report.balance` selon Mandat et périmètre | Synthèse, export PDF/CSV selon niveau autorisé |

### 2.3 Règles d’intégrité

| Règle | Description |
|-------|-------------|
| **INT-1** | Les services consommateurs ne dupliquent pas la logique devis/facturation/budget ; ils appellent les Opérateurs JayKonta. |
| **INT-2** | La liaison entre entités métier (exposant, professionnel, édition) et entités comptables (devis, facture, mouvement) est gérée par des **références** validées par StrongFather/Master Butler ; JayKonta ne détient pas la copie canonique des données métier des consommateurs. |
| **INT-3** | Toute donnée transmise à JayKonta (montants, identité client/fournisseur) est classée et protégée selon le niveau WorrySentinel déclaré ; audit et résidence s’appliquent. |
| **INT-4** | En état de confiance dégradé (T2–T4), les capacités d’écriture ou d’export peuvent être restreintes ; les services consommateurs doivent gérer les refus ou reports (Caring Nanny, WorrySentinel). |

---

## 3. Références

| Document | Rôle |
|----------|------|
| [JayKonta - Document Fondateur](../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sécurité. |
| [JayKonta - Points Entree JayBudget et JayKonta](./JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | Périmètres JayBudget vs JayKonta. |
| [JayFestival - Document Fondateur](../../JayFestival/JayFestival%20-%20Document%20Fondateur.md) | Service consommateur (budget édition, facturation exposants). |
| [JayRDV - Document Fondateur](../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | COG de référence, résidence centralisée. |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : JayKonta — Intégration avec les autres services  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de référence (intégration). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
