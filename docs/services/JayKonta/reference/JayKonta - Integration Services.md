# JayKonta — Intégration avec les autres services

## Contexte

Le **service COG JayKonta** expose des **Opérateurs** et **Kits d’outils** pour la comptabilité multi-échelle (budgets, devis, facturation, rapports). Les **services métier** (JayRDV, futurs services) **consomment** JayKonta pour éviter la duplication de la logique budget/facturation et garantir une cohérence gouvernée.

Ce document décrit le **modèle d’intégration** : services consommateurs identifiés, flux de données, responsabilités (données métier vs données comptables), et règles de résidence et de sécurité.

## Portée / Scope

- **Périmètre** : Intégration de JayKonta avec JayRDV, JayKoa (optionnel), futurs services.
- **Hors périmètre** : Spécifications techniques détaillées des API et contrats d’Opérateurs (référencés dans d’autres documents).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisée** — Document fondateur JayKonta, Points d’entrée JayBudget et JayKonta, Document fondateur JayRDV, Politique de résidence. **Contraintes** : ne pas fusionner avec le Document fondateur ni avec les analyses des besoins ; ne pas anticiper les contrats d’API détaillés.

---

## 1. Services consommateurs identifiés

### 1.1 Tableau récapitulatif

| Service | Usage de JayKonta | Données concernées | Point d’entrée |
|---------|---------------------------|--------------------|----------------|
| **JayRDV** | Facturation professionnels, abonnements, encaissements | Factures, encaissements | JayKonta (entreprise) |
| **JayKoa** | Optionnel : rappels ou jalons liés à des échéances budget (échéances factures, objectifs) | Références temporelles, pas de données financières canoniques | N/A (références uniquement) |
| **Futurs services** | Tout service nécessitant devis, facturation ou suivi de budget | À définir par service | Purse ou Account selon contexte |

### 1.2 JayRDV

| Besoin | Description |
|--------|-------------|
| **Facturation professionnels** | Facturation des prestations (RDV, abonnements), émission de factures, relances. |
| **Encaissements** | Suivi des encaissements, moyens de paiement (tokens, pas de stockage RIB/carte en clair). |

**Responsabilité** : JayRDV détient les **données métier** (qui est professionnel, quels RDV, quels abonnements) ; JayKonta gère l’**émission des factures** et le **suivi des encaissements** selon Mandat et niveau de sécurité.

### 1.3 JayKoa (optionnel)

| Besoin | Description |
|--------|-------------|
| **Rappels / jalons** | Rappels ou jalons liés à des échéances budget (ex. échéance facture, objectif d’épargne). | JayKoa ne détient **pas** la copie canonique des données financières ; il reçoit des **références** (date, type d’échéance, identifiant opaque) pour afficher des rappels dans l’agenda. La **source de vérité** reste JayKonta / KindMother. |

---

## 2. Modèle d’intégration

### 2.1 Principe

- **JayKonta** (COG) expose des **Opérateurs** et **Kits** (ex. : `budget.movements.record`, `quote.create`, `invoice.emit`, `report.balance`, `report.export`).
- Chaque **service consommateur** (JayRDV) :
  - détient les **données métier** (identité des acteurs, contrats, éditions, RDV) ;
  - **appelle** JayKonta pour enregistrer des mouvements, émettre des devis/factures, produire des rapports, selon **Mandat de Permission** et **permissions** (Master Butler) ;
  - **déclare** le niveau de sécurité (WorrySentinel) des données qu’il transmet ; JayKonta applique les règles de résidence, chiffrement et audit en conséquence.
- **KindMother** : la **résidence** des données sensibles (données financières personnelles ou d’entreprise) est définie par le [contrat du service](..//..//..//miyukini-webway-system//reference//_index.md) et le point d’entrée (Purse vs Account). Le **COG de référence** pour les données Account (entreprise) est désigné par le contrat (ex. COG du service).

### 2.2 Flux typiques

| Flux | Acteur initiateur | JayKonta | Résultat |
|------|------------------|------------------|----------|
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
| [JayKonta - Document Fondateur](..//_index.md) | Contexte, besoins, positionnement, sécurité. |
| [JayKonta - Points Entree JayBudget et JayKonta](./JayKonta%20-%20Points%20Entree%20JayBudget%20et%20JayKonta.md) | Périmètres JayBudget vs JayKonta. |
| [JayRDV - Document Fondateur](../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Politique de résidence des données sensibles](..//..//..//miyukini-webway-system//reference//_index.md) | COG de référence, résidence centralisée. |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : JayKonta — Intégration avec les autres services  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de référence (intégration). Enrichi selon [Protocole d’écriture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).



