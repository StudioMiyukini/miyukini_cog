# Odoo Accounting — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Accounting d'Odoo, en respectant l'architecture COG et la gouvernance Miyukini.

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Miyukini Account - Document Fondateur](../../../../services/MiyukiniAccount/Miyukini%20Account%20-%20Document%20Fondateur.md)
- [Odoo Accounting - Logique Métier](../00_logique_metier/Odoo%20Accounting%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalents Accounting
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

**Hors scope :**
- Implémentation technique détaillée (voir Guide d'Implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

L'équivalent Accounting dans Miyukini s'appuie sur le **service COG Miyukini Account** existant, avec des **Opérateurs spécialisés** pour la comptabilité d'entreprise.

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **AccountLedger** | Gestion du grand livre et des écritures | Opérateur de Service |
| **AccountJournal** | Gestion des journaux comptables | Opérateur de Service |
| **AccountChart** | Gestion du plan comptable | Opérateur de Service |
| **AccountReconciliation** | Rapprochements bancaires et réconciliations | Opérateur de Service |
| **AccountInvoice** | Facturation (déjà existant via MiyuInvoice) | Opérateur de Service |
| **AccountReport** | Génération de rapports comptables | Opérateur de Service |
| **AccountUI** | Interface utilisateur comptabilité | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : AccountService

**Définition :**
> **AccountService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de comptabilité d'entreprise.**

**Composition :**
- AccountLedger (niveau sécurité 2)
- AccountJournal (niveau sécurité 2)
- AccountChart (niveau sécurité 2)
- AccountReconciliation (niveau sécurité 2)
- AccountInvoice (niveau sécurité 2-3)
- AccountReport (niveau sécurité 1-2)
- AccountUI (niveau sécurité 1)

**Contrat d'Équipe :** Voir section 2

---

## 2. Opérateurs Détaillés

### 2.1 AccountLedger

**Rôle :** Gestion du grand livre comptable et des écritures.

**Capacités :**
- Création/modification d'écritures comptables
- Validation d'écritures (sous gouvernance StrongFather)
- Consultation du grand livre
- Export des écritures

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation
- **KindMother** : Persistance des écritures (WriteIntent)
- **Master Butler** : Permissions de création/modification
- **WorrySentinel** : Vérification niveau sécurité, état système

**Contrat d'équipe :**
- Consomme : AccountChart (plan comptable), AccountJournal (journaux)
- Expose : `ledger.entry.create`, `ledger.entry.validate`, `ledger.entry.query`

**Mandat de Permission requis :**
- Création écriture : Mandat avec AccountChart + AccountJournal
- Validation écriture : Mandat avec StrongFather (décision)

### 2.2 AccountJournal

**Rôle :** Gestion des journaux comptables (ventes, achats, banque, caisse, divers).

**Capacités :**
- Création/modification de journaux
- Configuration des séquences de numérotation
- Gestion des comptes par défaut

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des journaux
- **Ever Buddy** : Gestion des séquences (cycle de vie)
- **Master Butler** : Permissions de configuration

**Contrat d'équipe :**
- Consommé par : AccountLedger, AccountInvoice
- Expose : `journal.get`, `journal.sequence.next`

### 2.3 AccountChart

**Rôle :** Gestion du plan comptable (comptes, types, hiérarchie).

**Capacités :**
- Création/modification de comptes
- Import de plans comptables standards (PCG France, etc.)
- Gestion de la hiérarchie des comptes

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance du plan comptable
- **Master Butler** : Permissions de modification
- **WorrySentinel** : Vérification avant modification (comptes utilisés)

**Contrat d'équipe :**
- Consommé par : AccountLedger, AccountInvoice, AccountReconciliation
- Expose : `chart.account.get`, `chart.account.validate`

### 2.4 AccountReconciliation

**Rôle :** Rapprochements bancaires et réconciliations d'écritures.

**Capacités :**
- Import de relevés bancaires
- Correspondance automatique d'écritures
- Réconciliation manuelle
- Validation de réconciliations

**Niveau de sécurité :** 2-3 (Sensitive à Critical selon données bancaires)

**Gouvernance :**
- **StrongFather** : Décision de validation de réconciliation
- **KindMother** : Persistance des réconciliations (WriteIntent)
- **Master Butler** : Permissions de réconciliation
- **WorrySentinel** : Niveau sécurité selon données bancaires

**Contrat d'équipe :**
- Consomme : AccountLedger (écritures), AccountJournal (journaux bancaires)
- Expose : `reconciliation.suggest`, `reconciliation.validate`, `reconciliation.record`

**Mandat de Permission requis :**
- Validation réconciliation : Mandat avec StrongFather (décision) + KindMother (WriteIntent)

### 2.5 AccountInvoice

**Rôle :** Facturation (déjà couvert par MiyuInvoice, intégration AccountService).

**Note :** AccountInvoice utilise les Kits MiyuInvoice existants, avec intégration dans AccountService pour la comptabilisation automatique.

**Capacités :**
- Création de factures clients/fournisseurs
- Génération d'écritures comptables automatiques
- Gestion des conditions de paiement

**Niveau de sécurité :** 2-3 (Sensitive à Critical)

**Gouvernance :**
- **StrongFather** : Décision d'émission facture
- **KindMother** : Persistance factures + écritures (WriteIntent)
- **MiyuInvoice** : Outils de facturation

**Contrat d'équipe :**
- Consomme : AccountChart (comptes), AccountJournal (journaux), MiyuInvoice (outils)
- Expose : `invoice.create`, `invoice.post` (comptabilisation)

### 2.6 AccountReport

**Rôle :** Génération de rapports comptables (grand livre, balance, etc.).

**Capacités :**
- Génération de rapports standards
- Export PDF/Excel
- Filtrage et agrégation

**Niveau de sécurité :** 1-2 (Standard à Sensitive selon données)

**Gouvernance :**
- **KindMother** : Lecture des données (pas de WriteIntent)
- **Master Butler** : Permissions de consultation
- **WorrySentinel** : Niveau sécurité selon données agrégées

**Contrat d'équipe :**
- Consomme : AccountLedger (écritures), AccountChart (plan comptable)
- Expose : `report.generate`, `report.export`

### 2.7 AccountUI

**Rôle :** Interface utilisateur pour la comptabilité.

**Capacités :**
- Affichage des écritures, journaux, plan comptable
- Formulaires de saisie
- Tableaux de bord

**Niveau de sécurité :** 1 (Standard pour UI, données selon Opérateurs)

**Gouvernance :**
- **Master Butler** : Permissions d'accès UI
- **WorrySentinel** : Niveau sécurité selon données affichées

**Contrat d'équipe :**
- Consomme : Tous les autres Opérateurs AccountService
- Expose : Interface web/mobile

---

## 3. Contrat d'Équipe AccountService

### 3.1 Définition

**Contrat d'Équipe :** AccountService

**Opérateurs membres :**
- AccountLedger
- AccountJournal
- AccountChart
- AccountReconciliation
- AccountInvoice
- AccountReport
- AccountUI

### 3.2 Flux Autorisés

**Flux de données :**

```
AccountUI → AccountLedger → KindMother (WriteIntent)
AccountUI → AccountJournal → KindMother
AccountUI → AccountChart → KindMother
AccountUI → AccountReconciliation → StrongFather (décision) → KindMother (WriteIntent)
AccountUI → AccountInvoice → MiyuInvoice → AccountLedger → KindMother
AccountUI → AccountReport → AccountLedger (lecture) → AccountChart (lecture)
```

**Règles :**
- AccountUI ne communique jamais directement avec KindMother (toujours via Opérateurs)
- AccountReconciliation passe par StrongFather pour validation
- AccountInvoice utilise MiyuInvoice pour facturation, puis AccountLedger pour comptabilisation

### 3.3 Types d'Échanges

**Types de données :**
- Écritures comptables (AccountLedger)
- Journaux (AccountJournal)
- Comptes (AccountChart)
- Réconciliations (AccountReconciliation)
- Factures (AccountInvoice)
- Rapports (AccountReport)

**Niveau de validation requis :**
- Création : Validation Master Butler (permissions)
- Modification : Validation Master Butler + WorrySentinel (niveau sécurité)
- Validation écriture : Validation StrongFather (décision) + KindMother (WriteIntent)
- Réconciliation : Validation StrongFather (décision) + KindMother (WriteIntent)

### 3.4 Conditions Préalables

**Avant activation de l'Équipe :**
1. Plan comptable configuré (AccountChart)
2. Journaux créés (AccountJournal)
3. Mandats de Permission émis (StrongFather)
4. Niveaux de sécurité définis (WorrySentinel)

---

## 4. Mandats de Permission

### 4.1 Mandat Standard : AccountService Standard

**Émis par :** StrongFather

**Opérateurs autorisés :**
- AccountUI (lecture/écriture)
- AccountLedger (création/modification écritures)
- AccountJournal (lecture)
- AccountChart (lecture)
- AccountReport (génération rapports)

**Flux autorisés :**
- AccountUI → AccountLedger → KindMother (WriteIntent)
- AccountUI → AccountReport → AccountLedger (lecture)

**Types de données :**
- Écritures comptables
- Journaux
- Comptes
- Rapports

**Niveau de sécurité maximum :** 2 (Sensitive)

**Conditions de validité :**
- Utilisateur authentifié
- Environnement en état HEALTHY ou DEGRADED
- Permissions Master Butler accordées

**Révocation :**
- Fin de session utilisateur
- Changement d'état système (T3-T4)
- Violation de règle WorrySentinel

### 4.2 Mandat Validation : AccountService Validation

**Émis par :** StrongFather

**Opérateurs autorisés :**
- AccountLedger (validation écritures)
- AccountReconciliation (validation réconciliations)

**Flux autorisés :**
- AccountLedger → StrongFather (décision) → KindMother (WriteIntent)
- AccountReconciliation → StrongFather (décision) → KindMother (WriteIntent)

**Types de données :**
- Écritures à valider
- Réconciliations à valider

**Niveau de sécurité maximum :** 2-3 (Sensitive à Critical)

**Conditions de validité :**
- Utilisateur avec rôle comptable ou responsable financier
- Écriture en état draft
- Environnement en état HEALTHY

**Révocation :**
- Validation effectuée
- Annulation de l'opération
- Changement d'état système

### 4.3 Mandat Configuration : AccountService Configuration

**Émis par :** StrongFather

**Opérateurs autorisés :**
- AccountJournal (modification journaux)
- AccountChart (modification plan comptable)

**Flux autorisés :**
- AccountJournal → KindMother (WriteIntent)
- AccountChart → KindMother (WriteIntent)

**Types de données :**
- Configuration journaux
- Modification plan comptable

**Niveau de sécurité maximum :** 2 (Sensitive)

**Conditions de validité :**
- Utilisateur avec rôle administrateur comptable
- Environnement en état HEALTHY
- Validation WorrySentinel (comptes non utilisés)

**Révocation :**
- Fin de session
- Changement d'état système

---

## 5. Niveaux de Sécurité

### 5.1 Classification des Données

| Type de donnée | Niveau | Justification |
|----------------|--------|---------------|
| Plan comptable (structure) | 2 (Sensitive) | Données métier sensibles |
| Écritures comptables | 2 (Sensitive) | Données financières |
| Factures | 2-3 (Sensitive à Critical) | Selon montants et identités |
| Réconciliations bancaires | 3 (Critical) | Données bancaires sensibles |
| Rapports agrégés | 1-2 (Standard à Sensitive) | Selon niveau de détail |
| Interface utilisateur | 1 (Standard) | UI seule, données selon Opérateurs |

### 5.2 Mesures de Protection

**Niveau 1 (Standard) :**
- Contrôle d'accès (Mandat, Master Butler)
- Traçabilité des accès

**Niveau 2 (Sensitive) :**
- Résidence centralisée (KindMother)
- Chiffrement en transit
- Audit des lectures/écritures
- Mandats de Permission requis

**Niveau 3 (Critical) :**
- Résidence centralisée obligatoire
- Chiffrement au repos et en transit
- Audit complet
- Révocation immédiate possible

---

## 6. Intégration avec les Cores

### 6.1 StrongFather

**Rôle :** Décision stratégique

**Interventions :**
- Validation d'écritures comptables
- Validation de réconciliations
- Émission de Mandats de Permission
- Révocation de Mandats si nécessaire

**Règles :**
- StrongFather ne modifie jamais les données (KindMother)
- StrongFather ne persiste jamais (KindMother)
- StrongFather décide, KindMother exécute

### 6.2 KindMother

**Rôle :** Autorité absolue des données

**Responsabilités :**
- Persistance des écritures comptables (WriteIntent)
- Persistance du plan comptable
- Persistance des journaux
- Persistance des réconciliations (WriteIntent)

**Règles :**
- Toute écriture = WriteIntent vers KindMother
- KindMother valide la cohérence avant persistance
- KindMother gère la résidence des données sensibles

### 6.3 Master Butler

**Rôle :** Registre des capacités et permissions

**Responsabilités :**
- Déclaration des Opérateurs AccountService
- Déclaration des capacités (ledger.entry.create, etc.)
- Gestion des permissions utilisateur
- Validation des Mandats de Permission

**Règles :**
- Master Butler ne décide pas (StrongFather)
- Master Butler ne persiste pas (KindMother)
- Master Butler déclare et valide

### 6.4 WorrySentinel

**Rôle :** Gouvernance de sécurité

**Responsabilités :**
- Définition des niveaux de sécurité (2-3 pour AccountService)
- Vérification avant opérations sensibles
- Blocage en cas de menace
- Audit de sécurité

**Règles :**
- WorrySentinel ne décide pas (StrongFather)
- WorrySentinel ne persiste pas (KindMother)
- WorrySentinel gouverne la sécurité

### 6.5 Ever Buddy

**Rôle :** Cycle de vie et évolution

**Responsabilités :**
- Gestion des séquences de numérotation (journaux)
- Gestion des versions de plan comptable
- Compatibilité des écritures

**Règles :**
- Ever Buddy ne décide pas (StrongFather)
- Ever Buddy ne persiste pas (KindMother)
- Ever Buddy gère l'évolution

---

## 7. Workflows Gouvernés

### 7.1 Workflow : Création d'Écriture

```
1. AccountUI → AccountLedger.create_entry()
2. AccountLedger → Master Butler (vérification permissions)
3. Master Butler → WorrySentinel (vérification niveau sécurité)
4. WorrySentinel → AccountLedger (autorisation)
5. AccountLedger → StrongFather (décision création)
6. StrongFather → KindMother (WriteIntent)
7. KindMother → Persistance écriture
8. KindMother → AccountLedger (confirmation)
9. AccountLedger → AccountUI (affichage)
```

### 7.2 Workflow : Validation d'Écriture

```
1. AccountUI → AccountLedger.validate_entry()
2. AccountLedger → StrongFather (décision validation)
3. StrongFather → Vérification équilibre comptable
4. StrongFather → Ever Buddy (génération numéro séquence)
5. StrongFather → KindMother (WriteIntent validation)
6. KindMother → Persistance écriture validée
7. KindMother → AccountLedger (confirmation)
8. AccountLedger → AccountUI (affichage)
```

### 7.3 Workflow : Réconciliation

```
1. AccountUI → AccountReconciliation.suggest()
2. AccountReconciliation → AccountLedger (lecture écritures)
3. AccountReconciliation → Algorithme correspondance
4. AccountReconciliation → AccountUI (suggestions)
5. AccountUI → AccountReconciliation.validate()
6. AccountReconciliation → StrongFather (décision validation)
7. StrongFather → KindMother (WriteIntent réconciliation)
8. KindMother → Persistance réconciliation
9. KindMother → AccountReconciliation (confirmation)
10. AccountReconciliation → AccountUI (affichage)
```

---

## 8. Contraintes et Bornage

### 8.1 Contraintes Architecturales

**COG :**
- Un Opérateur ne peut jamais contourner la gouvernance
- Toute écriture = WriteIntent vers KindMother
- Toute décision = StrongFather

**Sécurité :**
- Niveau minimum 2 (Sensitive) pour données comptables
- Niveau 3 (Critical) pour données bancaires
- Mandats de Permission requis pour toutes opérations

**Performance :**
- WriteIntent asynchrone pour opérations non critiques
- WriteIntent synchrone pour validation d'écritures
- Requêtes optimisées pour rapports

### 8.2 Bornage Fonctionnel

**Inclus :**
- Gestion du grand livre
- Plan comptable
- Journaux comptables
- Écritures manuelles
- Factures (via MiyuInvoice)
- Réconciliations bancaires
- Rapports comptables standards

**Exclus (hors scope initial) :**
- Comptabilité analytique avancée (module séparé)
- Gestion de trésorerie avancée (module séparé)
- Déclarations fiscales (module séparé)
- Multi-company avancé (gestion simplifiée)

---

## 9. Conclusion

Les **spécifications d'Opérateurs Miyukini** pour Accounting définissent :

- **7 Opérateurs** spécialisés dans la comptabilité
- **1 Équipe d'Opérateurs** (AccountService) avec contrat explicite
- **3 Mandats de Permission** (Standard, Validation, Configuration)
- **Niveaux de sécurité** adaptés (1-3 selon données)
- **Intégration complète** avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Prochaines étapes :** Voir [Guide d'Intégration COG](./05_integration_cog/Odoo%20Accounting%20-%20Guide%20Integration%20COG.md) pour les détails d'implémentation.

---

**Document** : Odoo Accounting — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Spécifications complètes — référence pour implémentation
