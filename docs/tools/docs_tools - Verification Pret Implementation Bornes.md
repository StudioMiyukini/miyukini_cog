# docs_tools — Vérification prêt à l'implémentation (guides et bornes)

**Version :** 1.0  
**Statut :** Rapport de vérification  
**Date :** 2026-01-30  
**Références :** [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md), [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md), [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md)

---

## 1. Contexte

Ce document vérifie que **tous les Toolkits** sont prêts à être implémentés **sans risque de déviation** par rapport aux contrats : présence des trois livrables obligatoires (Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract), **explicitation des bornes (BOUND-*)** et présence d’un **guide d’implémentation** ou d’un **contrat de bornage** (Runtime Boundary Contract).

**Critère « prêt sans déviation » :** outre les 3 livrables obligatoires, le kit dispose soit d’un **Reference Implementation Guidelines** (avec tableau BOUND-1 à BOUND-6 et principes), soit d’un **Runtime Boundary Contract** (bornes explicites). Sans cela, l’implémenteur doit déduire les bornes du Template et du contrat Governance seul → **risque de déviation** (BOUND-4, BOUND-5, BOUND-6 notamment peu présents dans les contrats Governance).

---

## 2. Portée / Scope

| Inclus | Exclus |
|--------|--------|
| Tous les Toolkits sous `docs/tools/<MiyuXXX>/` | Cores, Kernel |
| Bornes BOUND-1 à BOUND-6 (décision, choix métier, accès direct, contexte, Opérateur, capacité nouvelle) | Implémentation technique effective |
| Reference Implementation Guidelines, Runtime Boundary Contract | Contrats optionnels KindMother / Security / Dependencies (non requis pour « prêt ») |

---

## 3. Rappel des bornes (BOUND-*)

Les bornes sont définies dans le [Template Reference Implementation Guidelines](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) et optionnellement dans un **Runtime Boundary Contract** par kit.

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Exécution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix métier | Exécution sur données/paramètres fournis |
| **BOUND-3** | Pas d'accès direct non gouverné | WriteIntent KindMother ou pas de persistance métier |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé (niveau, permissions) |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés |

**Source de vérité :** Master Butler - Tool Governance Compliance Template (obligations communes) + contrats spécifiques par kit. Les bornes BOUND-* rendent ces obligations **explicites et vérifiables** en implémentation.

---

## 4. État par Toolkit

### 4.1 Synthèse

| Critère | Nombre de kits |
|---------|----------------|
| **Prêt à implémenter sans déviation** (3 livrables + bornes explicites) | **21** |
| **Avec précautions** (3 livrables, pas de bornes explicites) | **28** |
| **À compléter** (manque livrable obligatoire) | **0** |

### 4.2 Kits « Prêt » (bornes explicites)

Ces kits disposent des **3 livrables obligatoires** et soit d’un **Reference Implementation Guidelines** (avec BOUND-*), soit d’un **Runtime Boundary Contract**.

| Kit | Doc Fondatrice | Reference Outils | Contrat Governance | Contrat Boundary | Guide implémentation |
|-----|----------------|------------------|--------------------|------------------|----------------------|
| MiyuAuth | Oui | Oui | Oui | Oui | Oui |
| MiyuWeb | Oui | Oui | Oui | Oui | Oui |
| MiyuSQL | Oui | Oui | Oui | Oui | Oui |
| MiyuClock | Oui | Oui | Oui | Oui | Non |
| MiyuForum | Oui | Oui | Oui | Non | Oui |
| MiyuPM | Oui | Oui | Oui | Non | Oui |
| MiyuNotify | Oui | Oui | Oui | Non | Oui |
| MiyuSearch | Oui | Oui | Oui | Non | Oui |
| MiyuWebwayParticipant | Oui | Oui | Oui | Non | Oui |
| MiyuWebwayTracker | Oui | Oui | Oui | Non | Oui |
| MiyuBilling | Oui | Oui | Oui | Non | Oui |
| MiyuBooking | Oui | Oui | Oui | Non | Oui |
| MiyuCMS | Oui | Oui | Oui | Non | Oui |
| MiyuMedia | Oui | Oui | Oui | Non | Oui |
| MiyuShipping | Oui | Oui | Oui | Non | Oui |
| MiyuStore | Oui | Oui | Oui | Non | Oui |
| MiyuWidgets | Oui | Oui | Oui | Non | Oui |
| MiyuInvoice | Oui | Oui | Oui | Non | Oui |
| MiyuComptaLedger | Oui | Oui | Oui | Non | Oui |
| MiyuExpense | Oui | Oui | Oui | Non | Oui |
| MiyuTreasury | Oui | Oui | Oui | Non | Oui |

**Conclusion :** 21 kits sont prêts à être implémentés sans risque de déviation ; les bornes sont explicites (guide et/ou contrat Boundary). *Mise à jour 2026-01-30 :* 11 guides Reference Implementation Guidelines ajoutés (MiyuBilling, MiyuBooking, MiyuCMS, MiyuMedia, MiyuShipping, MiyuStore, MiyuWidgets, MiyuInvoice, MiyuComptaLedger, MiyuExpense, MiyuTreasury).

### 4.3 Kits « Avec précautions » (pas de bornes explicites)

Ces kits disposent des **3 livrables obligatoires** (Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract) mais **ni** Reference Implementation Guidelines **ni** Runtime Boundary Contract. L’implémenteur doit s’appuyer sur :

- le [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) (obligations communes) ;
- le [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) (BOUND-* génériques) ;
- le contrat Governance du kit (obligations spécifiques).

**Risque :** BOUND-4, BOUND-5, BOUND-6 et détails d’implémentation (gestion d’erreurs, traçabilité, refus en cas de violation) ne sont pas rappelés dans le kit → possible déviation si le Template n’est pas lu.

**Kits concernés (28) :**  
MiyuAntiSpam, MiyuBookmarks, MiyuCalc, MiyuComptaReports, MiyuContacts, MiyuDeclarations, MiyuDiscovery, MiyuExport, MiyuFeeds, MiyuHR, MiyuJobs, MiyuLocale, MiyuModerationForum, MiyuPolls, MiyuPosAnalytics, MiyuPosInventory, MiyuPosKitchen, MiyuPosLoyalty, MiyuPosPayment, MiyuPosSales, MiyuProfile, MiyuSocialFeed, MiyuSocialMessaging, MiyuSocialModeration, MiyuSocialProfile, MiyuStory, MiyuText, MiyuValidate.

---

## 5. Recommandations

### 5.1 Avant implémentation (kits « Avec précautions »)

Pour **réduire le risque de déviation** avant de coder :

1. **Option recommandée :** rédiger un **Reference Implementation Guidelines** minimal (dérivé du [template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md)) : objectif, sources contractuelles, **tableau BOUND-1 à BOUND-6** adapté au kit, interdictions, gestion d’erreurs et traçabilité. Une à deux pages par kit suffisent.
2. **Option alternative :** rédiger un **Runtime Boundary Contract** (`contracts/boundaries/<MiyuXXX> - Runtime Boundary Contract.md`) qui formalise les BOUND-* pour le kit (voir [Template - Ecriture Enrichie Toolkits](../protocols/Template%20-%20Ecriture%20Enrichie%20Toolkits.md) § 7).
3. **Minimum :** avant toute implémentation, faire lire à l’implémenteur le [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) (sections 2 et 3 : principes et tableau BOUND-*) et le contrat Governance du kit.

### 5.2 Priorisation des guides

En priorité pour les kits à **fort impact** (données sensibles, commerce, facturation, contenu) :

- MiyuBilling, MiyuStore, MiyuShipping (commerce / paiement)
- MiyuCMS, MiyuMedia (contenu)
- MiyuAuth (identité — déjà prêt)
- MiyuInvoice, MiyuComptaLedger, MiyuExpense, MiyuTreasury (compta / finance)
- MiyuWidgets (page builder, exposition web)

### 5.3 Vérification continue

- Lors de l’ajout d’un **nouveau Toolkit** : prévoir dès la documentation enrichie soit un Reference Implementation Guidelines, soit un Runtime Boundary Contract.
- Lors d’une **revue d’implémentation** : utiliser le tableau BOUND-* comme checklist (aucune décision ALLOW/DENY, pas d’accès direct, pas de modification du contexte, pas d’identité Opérateur, uniquement les ToolIds déclarés).

---

## 6. Références

| Document | Lien |
|----------|------|
| Protocole Ecriture Enrichie Toolkits | [Miyukini Protocol - Ecriture Enrichie Toolkits](../protocols/Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Template Reference Implementation Guidelines | [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| Audit Qualité Conformité Sécurité | [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| Template Ecriture Enrichie (Runtime Boundary) | [Template - Ecriture Enrichie Toolkits](../protocols/Template%20-%20Ecriture%20Enrichie%20Toolkits.md) § 7 |
| Index docs/tools | [_index](./_index.md) |

---

**Date de création :** 2026-01-30  
**Statut :** Rapport de vérification — à mettre à jour après ajout de guides ou contrats Boundary
