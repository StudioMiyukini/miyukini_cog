# Miyukini Account — Niveaux de sécurité et protection des données

## Contexte

Ce document détaille les **niveaux de sécurité** (WorrySentinel 0–4) appliqués aux données et flux du service **Miyukini Account** (COG), ainsi que les **solutions de protection** associées. Il complète le [Document fondateur](../Miyukini%20Account%20-%20Document%20Fondateur.md) et s’aligne sur la [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) et le [Glossaire Miyukini](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) (Niveaux de sécurité, WorrySentinel).

Les données financières sont **hautement sensibles** : revenus, dépenses, factures, devis, moyens de paiement, identité des clients/fournisseurs. La classification et les mesures de protection sont **critiques** pour la confiance et la conformité.

## Portée / Scope

- **Périmètre** : Niveaux de sécurité des données budget/comptabilité (mouvements, devis, factures, rapports) et mesures de protection (résidence, chiffrement, audit, visibilité).
- **Hors périmètre** : Implémentation technique détaillée (référencée dans les contrats d’Opérateurs et Kits).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisée** — Document fondateur Miyukini Account, Politique de résidence, Glossaire Miyukini, Points d’entrée Purse et Account. **Contraintes** : ne pas fusionner avec le Document fondateur ; ne pas anticiper l’implémentation (contrats d’Opérateurs/Kits).

---

## 1. Sensibilité des données budget et comptabilité

### 1.1 Typologie des données traitées

| Type de donnée | Exemple | Sensibilité | Niveau WorrySentinel |
|----------------|---------|-------------|----------------------|
| **Agrégats anonymisés** | Totaux par catégorie sans lien identité, statistiques globales | Faible | 0 |
| **Références génériques** | Catégories, libellés de type (revenu/dépense) sans montant ni identité | Faible à standard | 0–1 |
| **Mouvements personnels (Purse)** | Revenus, dépenses, catégories, budgets occasionnels (vacances, Noël) | Sensible | 2 |
| **Devis et factures** | Montants, TVA, identité client/fournisseur, libellés | Sensible à critique | 2–3 |
| **Données de paiement** | Références de moyen de paiement, RIB, historique encaissements | Critique | 3 |
| **Comptabilité légale** | Pièces comptables, rapports soumis à contrôle, bilans | Critique | 3 |
| **Accès exceptionnel** | Interventions MiyukiniAdmin, TAMR | Highest | 4 |

### 1.2 Différenciation Purse vs Account

| Point d’entrée | Données typiques | Niveau minimal |
|----------------|------------------|----------------|
| **Miyukini Purse** | Mouvements personnels, budgets occasionnels, catégories, objectifs | 2 (Sensitive) |
| **Miyukini Account** | Devis, factures, comptabilité d’entreprise, rapports légaux, moyens de paiement | 2–3 (Sensitive à Critical) |

### 1.3 Niveaux WorrySentinel (rappel)

| Niveau | Nom | Description |
|--------|-----|-------------|
| **0** | Public | Données publiques, aucune contrainte stricte |
| **1** | Standard | Données standard, contraintes de base |
| **2** | Sensitive | Données sensibles, contraintes renforcées |
| **3** | Critical | Données critiques, contraintes strictes |
| **4** | Highest | Sécurité maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sécurité et les états de confiance ; Master Butler gère les permissions ; StrongFather émet les Mandats.

---

## 2. Solutions de protection par niveau

### 2.1 Niveau 0 — Public

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Agrégats anonymisés, statistiques globales sans lien identité. |
| **Accès** | Aucune contrainte stricte ; pas de Mandat obligatoire pour lecture. |
| **Résidence** | Non concerné. |
| **Audit** | Optionnel (trace minimale). |
| **Export** | Export public possible (ex. statistiques anonymes). |

### 2.2 Niveau 1 — Standard

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Catégories, libellés génériques, références sans montant ni identité. |
| **Accès** | Mandat de Permission ou Mandat public d’accès selon contexte ; permissions (Master Butler). |
| **Résidence** | Optionnel selon domaine ; pas d’obligation de résidence centralisée. |
| **Audit** | Traçabilité des accès (qui a consulté quoi, quand). |
| **Export** | Export autorisé si Mandat et niveau du destinataire compatibles ; pas de données personnelles ni financières identifiantes. |

### 2.3 Niveau 2 — Sensitive

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Mouvements personnels (Purse), budgets occasionnels, devis et factures (montants, identité client/fournisseur). |
| **Accès** | Mandat de Permission obligatoire ; résidence centralisée sur COG de référence recommandée ou obligatoire selon contrat (voir [Politique de résidence](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md)). |
| **Résidence** | COG de référence désigné par le contrat du service (Purse : optionnel selon politique ; Account : recommandé). Accès via Visite gouvernée ou session. |
| **Chiffrement** | Chiffrement en transit obligatoire ; chiffrement au repos recommandé. |
| **Audit** | Audit des lectures et écritures ; traçabilité complète. |
| **Export** | Export contrôlé ; pas d’exposition hors périmètre autorisé ; pas de données de paiement (RIB, cartes) en export. |

### 2.4 Niveau 3 — Critical

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Données de paiement (références RIB, tokens, historique encaissements), comptabilité légale, pièces comptables. |
| **Accès** | Mandat strict ; résidence centralisée obligatoire ; chiffrement au repos et en transit. |
| **Résidence** | COG de référence unique ; pas de copie sur terminal ou COG tiers sans gouvernance. |
| **Chiffrement** | Chiffrement au repos et en transit obligatoire ; conformité PCI-DSS / réglementation en vigueur. |
| **Audit** | Audit complet ; révocation immédiate possible (StrongFather, WorrySentinel). |
| **Export** | Export très restreint ; procédure d’autorisation explicite ; pas d’export de données de paiement brutes. |

### 2.5 Niveau 4 — Highest

| Aspect | Mesure |
|--------|--------|
| **Données concernées** | Accès MiyukiniAdmin, interventions TAMR, données de sécurité maximale. |
| **Accès** | Isolement renforcé ; procédures d’accès exceptionnel (TAMR, MiyukiniAdmin). |
| **Résidence** | COG de référence dédié ; contraintes maximales. |
| **Audit** | Audit exhaustif ; pas d’accès sans traçabilité. |
| **Export** | Interdit ou procédure exceptionnelle validée. |

---

## 3. Règles de sécurité spécifiques Miyukini Account

| Règle | Description |
|-------|-------------|
| **MAC-SEC-1** | Les données financières personnelles ou métier (mouvements, factures, devis, moyens de paiement) sont classées au minimum niveau 2 (Sensitive) ; les flux sont chiffrés et soumis à Mandat. |
| **MAC-SEC-2** | La résidence des données sensibles (niveau 2+) est définie par le contrat du service et le point d’entrée (Purse vs Account) ; COG de référence désigné pour Account et, selon politique, pour Purse. |
| **MAC-SEC-3** | Aucune donnée de paiement (RIB, cartes, tokens sensibles) n’est stockée en clair ; référencement par token ou identifiant opaque ; conformité PCI-DSS / réglementation en vigueur. |
| **MAC-SEC-4** | Toute émission de devis ou facture par un service consommateur (MFS, JayRDV) transite par les Opérateurs Miyukini Account avec audit et niveau de sécurité déclaré. |
| **MAC-SEC-5** | En état de confiance dégradé (T2–T4), les capacités d’écriture ou d’export peuvent être restreintes (Caring Nanny, WorrySentinel). |
| **MAC-SEC-6** | Les exports (PDF, CSV) ne doivent pas inclure de données au-delà du niveau autorisé pour le destinataire (ex. pas de RIB ni de données de paiement en export standard). |

---

## 4. États de confiance (T0–T4)

En cas de dégradation de l’intégrité du système (états de confiance T1–T4 gouvernés par WorrySentinel), les capacités de Miyukini Account peuvent être restreintes :

| État | Effet possible sur Miyukini Account |
|------|------------------------------------|
| **T0** | Normal — toutes capacités disponibles. |
| **T1** | Instable — surveillance accrue ; pas de restriction par défaut. |
| **T2** | Dégradé — écriture ou export peuvent être limités ; lecture des mouvements et rapports de base possible. |
| **T3** | Restreint — capacités d’écriture et d’export restreintes ; lecture seule pour consultation. |
| **T4** | Bloqué — uniquement diagnostics ; pas d’écriture ni d’export. |

Caring Nanny et WorrySentinel gouvernent ces restrictions ; Miyukini Account ne décide pas seul.

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Account - Document Fondateur](../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sécurité synthétique. |
| [Politique de résidence des données sensibles](../../reference/Miyukini%20Conceptual%20References%20-%20Politique%20Residence%20Donnees%20Sensibles.md) | Résidence centralisée, COG de référence, niveaux 2+. |
| [Glossaire — Niveaux de sécurité, WorrySentinel, États de confiance](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Définitions officielles. |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : Miyukini Account — Niveaux de sécurité et protection des données  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de référence (sécurité). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
