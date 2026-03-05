# JayKonta â€” Niveaux de sÃ©curitÃ© et protection des donnÃ©es

## Contexte

Ce document dÃ©taille les **niveaux de sÃ©curitÃ©** (WorrySentinel 0â€“4) appliquÃ©s aux donnÃ©es et flux du service **JayKonta** (COG), ainsi que les **solutions de protection** associÃ©es. Il complÃ¨te le [Document fondateur](../JayKonta%20-%20Document%20Fondateur.md) et sâ€™aligne sur la [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md) et le [Glossaire Miyukini](..//..//..//miyukini-webway-system//reference//_index.md) (Niveaux de sÃ©curitÃ©, WorrySentinel).

Les donnÃ©es financiÃ¨res sont **hautement sensibles** : revenus, dÃ©penses, factures, devis, moyens de paiement, identitÃ© des clients/fournisseurs. La classification et les mesures de protection sont **critiques** pour la confiance et la conformitÃ©.

## PortÃ©e / Scope

- **PÃ©rimÃ¨tre** : Niveaux de sÃ©curitÃ© des donnÃ©es budget/comptabilitÃ© (mouvements, devis, factures, rapports) et mesures de protection (rÃ©sidence, chiffrement, audit, visibilitÃ©).
- **Hors pÃ©rimÃ¨tre** : ImplÃ©mentation technique dÃ©taillÃ©e (rÃ©fÃ©rencÃ©e dans les contrats dâ€™OpÃ©rateurs et Kits).

### Cadre de travail (protocole documentation conceptuelle)

ConformÃ©ment au [Protocole dâ€™Ã©criture de la documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) : **documentation autorisÃ©e** â€” Document fondateur JayKonta, Politique de rÃ©sidence, Glossaire Miyukini, Points dâ€™entrÃ©e JayBudget et JayKonta. **Contraintes** : ne pas fusionner avec le Document fondateur ; ne pas anticiper lâ€™implÃ©mentation (contrats dâ€™OpÃ©rateurs/Kits).

---

## 1. SensibilitÃ© des donnÃ©es budget et comptabilitÃ©

### 1.1 Typologie des donnÃ©es traitÃ©es

| Type de donnÃ©e | Exemple | SensibilitÃ© | Niveau WorrySentinel |
|----------------|---------|-------------|----------------------|
| **AgrÃ©gats anonymisÃ©s** | Totaux par catÃ©gorie sans lien identitÃ©, statistiques globales | Faible | 0 |
| **RÃ©fÃ©rences gÃ©nÃ©riques** | CatÃ©gories, libellÃ©s de type (revenu/dÃ©pense) sans montant ni identitÃ© | Faible Ã  standard | 0â€“1 |
| **Mouvements personnels (Purse)** | Revenus, dÃ©penses, catÃ©gories, budgets occasionnels (vacances, NoÃ«l) | Sensible | 2 |
| **Devis et factures** | Montants, TVA, identitÃ© client/fournisseur, libellÃ©s | Sensible Ã  critique | 2â€“3 |
| **DonnÃ©es de paiement** | RÃ©fÃ©rences de moyen de paiement, RIB, historique encaissements | Critique | 3 |
| **ComptabilitÃ© lÃ©gale** | PiÃ¨ces comptables, rapports soumis Ã  contrÃ´le, bilans | Critique | 3 |
| **AccÃ¨s exceptionnel** | Interventions MiyukiniAdmin, TAMR | Highest | 4 |

### 1.2 DiffÃ©renciation Purse vs Account

| Point dâ€™entrÃ©e | DonnÃ©es typiques | Niveau minimal |
|----------------|------------------|----------------|
| **JayBudget** | Mouvements personnels, budgets occasionnels, catÃ©gories, objectifs | 2 (Sensitive) |
| **JayKonta** | Devis, factures, comptabilitÃ© dâ€™entreprise, rapports lÃ©gaux, moyens de paiement | 2â€“3 (Sensitive Ã  Critical) |

### 1.3 Niveaux WorrySentinel (rappel)

| Niveau | Nom | Description |
|--------|-----|-------------|
| **0** | Public | DonnÃ©es publiques, aucune contrainte stricte |
| **1** | Standard | DonnÃ©es standard, contraintes de base |
| **2** | Sensitive | DonnÃ©es sensibles, contraintes renforcÃ©es |
| **3** | Critical | DonnÃ©es critiques, contraintes strictes |
| **4** | Highest | SÃ©curitÃ© maximale, contraintes maximales |

**Gouvernance** : WorrySentinel gouverne les niveaux de sÃ©curitÃ© et les Ã©tats de confiance ; Master Butler gÃ¨re les permissions ; StrongFather Ã©met les Mandats.

---

## 2. Solutions de protection par niveau

### 2.1 Niveau 0 â€” Public

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | AgrÃ©gats anonymisÃ©s, statistiques globales sans lien identitÃ©. |
| **AccÃ¨s** | Aucune contrainte stricte ; pas de Mandat obligatoire pour lecture. |
| **RÃ©sidence** | Non concernÃ©. |
| **Audit** | Optionnel (trace minimale). |
| **Export** | Export public possible (ex. statistiques anonymes). |

### 2.2 Niveau 1 â€” Standard

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | CatÃ©gories, libellÃ©s gÃ©nÃ©riques, rÃ©fÃ©rences sans montant ni identitÃ©. |
| **AccÃ¨s** | Mandat de Permission ou Mandat public dâ€™accÃ¨s selon contexte ; permissions (Master Butler). |
| **RÃ©sidence** | Optionnel selon domaine ; pas dâ€™obligation de rÃ©sidence centralisÃ©e. |
| **Audit** | TraÃ§abilitÃ© des accÃ¨s (qui a consultÃ© quoi, quand). |
| **Export** | Export autorisÃ© si Mandat et niveau du destinataire compatibles ; pas de donnÃ©es personnelles ni financiÃ¨res identifiantes. |

### 2.3 Niveau 2 â€” Sensitive

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | Mouvements personnels (Purse), budgets occasionnels, devis et factures (montants, identitÃ© client/fournisseur). |
| **AccÃ¨s** | Mandat de Permission obligatoire ; rÃ©sidence centralisÃ©e sur COG de rÃ©fÃ©rence recommandÃ©e ou obligatoire selon contrat (voir [Politique de rÃ©sidence](..//..//..//miyukini-webway-system//reference//_index.md)). |
| **RÃ©sidence** | COG de rÃ©fÃ©rence dÃ©signÃ© par le contrat du service (Purse : optionnel selon politique ; Account : recommandÃ©). AccÃ¨s via Visite gouvernÃ©e ou session. |
| **Chiffrement** | Chiffrement en transit obligatoire ; chiffrement au repos recommandÃ©. |
| **Audit** | Audit des lectures et Ã©critures ; traÃ§abilitÃ© complÃ¨te. |
| **Export** | Export contrÃ´lÃ© ; pas dâ€™exposition hors pÃ©rimÃ¨tre autorisÃ© ; pas de donnÃ©es de paiement (RIB, cartes) en export. |

### 2.4 Niveau 3 â€” Critical

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | DonnÃ©es de paiement (rÃ©fÃ©rences RIB, tokens, historique encaissements), comptabilitÃ© lÃ©gale, piÃ¨ces comptables. |
| **AccÃ¨s** | Mandat strict ; rÃ©sidence centralisÃ©e obligatoire ; chiffrement au repos et en transit. |
| **RÃ©sidence** | COG de rÃ©fÃ©rence unique ; pas de copie sur terminal ou COG tiers sans gouvernance. |
| **Chiffrement** | Chiffrement au repos et en transit obligatoire ; conformitÃ© PCI-DSS / rÃ©glementation en vigueur. |
| **Audit** | Audit complet ; rÃ©vocation immÃ©diate possible (StrongFather, WorrySentinel). |
| **Export** | Export trÃ¨s restreint ; procÃ©dure dâ€™autorisation explicite ; pas dâ€™export de donnÃ©es de paiement brutes. |

### 2.5 Niveau 4 â€” Highest

| Aspect | Mesure |
|--------|--------|
| **DonnÃ©es concernÃ©es** | AccÃ¨s MiyukiniAdmin, interventions TAMR, donnÃ©es de sÃ©curitÃ© maximale. |
| **AccÃ¨s** | Isolement renforcÃ© ; procÃ©dures dâ€™accÃ¨s exceptionnel (TAMR, MiyukiniAdmin). |
| **RÃ©sidence** | COG de rÃ©fÃ©rence dÃ©diÃ© ; contraintes maximales. |
| **Audit** | Audit exhaustif ; pas dâ€™accÃ¨s sans traÃ§abilitÃ©. |
| **Export** | Interdit ou procÃ©dure exceptionnelle validÃ©e. |

---

## 3. RÃ¨gles de sÃ©curitÃ© spÃ©cifiques JayKonta

| RÃ¨gle | Description |
|-------|-------------|
| **MAC-SEC-1** | Les donnÃ©es financiÃ¨res personnelles ou mÃ©tier (mouvements, factures, devis, moyens de paiement) sont classÃ©es au minimum niveau 2 (Sensitive) ; les flux sont chiffrÃ©s et soumis Ã  Mandat. |
| **MAC-SEC-2** | La rÃ©sidence des donnÃ©es sensibles (niveau 2+) est dÃ©finie par le contrat du service et le point dâ€™entrÃ©e (Purse vs Account) ; COG de rÃ©fÃ©rence dÃ©signÃ© pour Account et, selon politique, pour Purse. |
| **MAC-SEC-3** | Aucune donnÃ©e de paiement (RIB, cartes, tokens sensibles) nâ€™est stockÃ©e en clair ; rÃ©fÃ©rencement par token ou identifiant opaque ; conformitÃ© PCI-DSS / rÃ©glementation en vigueur. |
| **MAC-SEC-4** | Toute Ã©mission de devis ou facture par un service consommateur (JayFestival, JayRDV) transite par les OpÃ©rateurs JayKonta avec audit et niveau de sÃ©curitÃ© dÃ©clarÃ©. |
| **MAC-SEC-5** | En Ã©tat de confiance dÃ©gradÃ© (T2â€“T4), les capacitÃ©s dâ€™Ã©criture ou dâ€™export peuvent Ãªtre restreintes (Caring Nanny, WorrySentinel). |
| **MAC-SEC-6** | Les exports (PDF, CSV) ne doivent pas inclure de donnÃ©es au-delÃ  du niveau autorisÃ© pour le destinataire (ex. pas de RIB ni de donnÃ©es de paiement en export standard). |

---

## 4. Ã‰tats de confiance (T0â€“T4)

En cas de dÃ©gradation de lâ€™intÃ©gritÃ© du systÃ¨me (Ã©tats de confiance T1â€“T4 gouvernÃ©s par WorrySentinel), les capacitÃ©s de JayKonta peuvent Ãªtre restreintes :

| Ã‰tat | Effet possible sur JayKonta |
|------|------------------------------------|
| **T0** | Normal â€” toutes capacitÃ©s disponibles. |
| **T1** | Instable â€” surveillance accrue ; pas de restriction par dÃ©faut. |
| **T2** | DÃ©gradÃ© â€” Ã©criture ou export peuvent Ãªtre limitÃ©s ; lecture des mouvements et rapports de base possible. |
| **T3** | Restreint â€” capacitÃ©s dâ€™Ã©criture et dâ€™export restreintes ; lecture seule pour consultation. |
| **T4** | BloquÃ© â€” uniquement diagnostics ; pas dâ€™Ã©criture ni dâ€™export. |

Caring Nanny et WorrySentinel gouvernent ces restrictions ; JayKonta ne dÃ©cide pas seul.

---

## 5. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [JayKonta - Document Fondateur](../JayKonta%20-%20Document%20Fondateur.md) | Contexte, besoins, positionnement, sÃ©curitÃ© synthÃ©tique. |
| [Politique de rÃ©sidence des donnÃ©es sensibles](..//..//..//miyukini-webway-system//reference//_index.md) | RÃ©sidence centralisÃ©e, COG de rÃ©fÃ©rence, niveaux 2+. |
| [Glossaire â€” Niveaux de sÃ©curitÃ©, WorrySentinel, Ã‰tats de confiance](..//..//..//miyukini-webway-system//reference//_index.md) | DÃ©finitions officielles. |
| [Miyukini Prompt Protocol â€” Ã‰criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole dâ€™Ã©criture de la documentation conceptuelle (cadre de travail, contraintes). |

---

**Document** : JayKonta â€” Niveaux de sÃ©curitÃ© et protection des donnÃ©es  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document de rÃ©fÃ©rence (sÃ©curitÃ©). Enrichi selon [Protocole dâ€™Ã©criture documentation conceptuelle](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).


