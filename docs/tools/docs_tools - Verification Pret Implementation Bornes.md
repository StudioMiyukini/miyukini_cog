# docs_tools â€” VÃ©rification prÃªt Ã  l'implÃ©mentation (guides et bornes)

**Version :** 1.0  
**Statut :** Rapport de vÃ©rification  
**Date :** 2026-01-30  
**RÃ©fÃ©rences :** [Miyukini Protocol - Ecriture Enrichie Toolkits](..//contrats//Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md), [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md), [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md)

---

## 1. Contexte

Ce document vÃ©rifie que **tous les Toolkits** sont prÃªts Ã  Ãªtre implÃ©mentÃ©s **sans risque de dÃ©viation** par rapport aux contrats : prÃ©sence des trois livrables obligatoires (Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract), **explicitation des bornes (BOUND-*)** et prÃ©sence dâ€™un **guide dâ€™implÃ©mentation** ou dâ€™un **contrat de bornage** (Runtime Boundary Contract).

**CritÃ¨re Â« prÃªt sans dÃ©viation Â» :** outre les 3 livrables obligatoires, le kit dispose soit dâ€™un **Reference Implementation Guidelines** (avec tableau BOUND-1 Ã  BOUND-6 et principes), soit dâ€™un **Runtime Boundary Contract** (bornes explicites). Sans cela, lâ€™implÃ©menteur doit dÃ©duire les bornes du Template et du contrat Governance seul â†’ **risque de dÃ©viation** (BOUND-4, BOUND-5, BOUND-6 notamment peu prÃ©sents dans les contrats Governance).

---

## 2. PortÃ©e / Scope

| Inclus | Exclus |
|--------|--------|
| Tous les Toolkits sous `docs/tools/<MiyuXXX>/` | Cores, Kernel |
| Bornes BOUND-1 Ã  BOUND-6 (dÃ©cision, choix mÃ©tier, accÃ¨s direct, contexte, OpÃ©rateur, capacitÃ© nouvelle) | ImplÃ©mentation technique effective |
| Reference Implementation Guidelines, Runtime Boundary Contract | Contrats optionnels KindMother / Security / Dependencies (non requis pour Â« prÃªt Â») |

---

## 3. Rappel des bornes (BOUND-*)

Les bornes sont dÃ©finies dans le [Template Reference Implementation Guidelines](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) et optionnellement dans un **Runtime Boundary Contract** par kit.

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | ExÃ©cution uniquement sur mandat StrongFather |
| **BOUND-2** | Pas de choix mÃ©tier | ExÃ©cution sur donnÃ©es/paramÃ¨tres fournis |
| **BOUND-3** | Pas d'accÃ¨s direct non gouvernÃ© | WriteIntent KindMother ou pas de persistance mÃ©tier |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© (niveau, permissions) |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s |

**Source de vÃ©ritÃ© :** Master Butler - Tool Governance Compliance Template (obligations communes) + contrats spÃ©cifiques par kit. Les bornes BOUND-* rendent ces obligations **explicites et vÃ©rifiables** en implÃ©mentation.

---

## 4. Ã‰tat par Toolkit

### 4.1 SynthÃ¨se

| CritÃ¨re | Nombre de kits |
|---------|----------------|
| **PrÃªt Ã  implÃ©menter sans dÃ©viation** (3 livrables + bornes explicites) | **21** |
| **Avec prÃ©cautions** (3 livrables, pas de bornes explicites) | **28** |
| **Ã€ complÃ©ter** (manque livrable obligatoire) | **0** |

### 4.2 Kits Â« PrÃªt Â» (bornes explicites)

Ces kits disposent des **3 livrables obligatoires** et soit dâ€™un **Reference Implementation Guidelines** (avec BOUND-*), soit dâ€™un **Runtime Boundary Contract**.

| Kit | Doc Fondatrice | Reference Outils | Contrat Governance | Contrat Boundary | Guide implÃ©mentation |
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

**Conclusion :** 21 kits sont prÃªts Ã  Ãªtre implÃ©mentÃ©s sans risque de dÃ©viation ; les bornes sont explicites (guide et/ou contrat Boundary). *Mise Ã  jour 2026-01-30 :* 11 guides Reference Implementation Guidelines ajoutÃ©s (MiyuBilling, MiyuBooking, MiyuCMS, MiyuMedia, MiyuShipping, MiyuStore, MiyuWidgets, MiyuInvoice, MiyuComptaLedger, MiyuExpense, MiyuTreasury).

### 4.3 Kits Â« Avec prÃ©cautions Â» (pas de bornes explicites)

Ces kits disposent des **3 livrables obligatoires** (Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract) mais **ni** Reference Implementation Guidelines **ni** Runtime Boundary Contract. Lâ€™implÃ©menteur doit sâ€™appuyer sur :

- le [Master Butler - Tool Governance Compliance Template](..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) (obligations communes) ;
- le [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) (BOUND-* gÃ©nÃ©riques) ;
- le contrat Governance du kit (obligations spÃ©cifiques).

**Risque :** BOUND-4, BOUND-5, BOUND-6 et dÃ©tails dâ€™implÃ©mentation (gestion dâ€™erreurs, traÃ§abilitÃ©, refus en cas de violation) ne sont pas rappelÃ©s dans le kit â†’ possible dÃ©viation si le Template nâ€™est pas lu.

**Kits concernÃ©s (28) :**  
MiyuAntiSpam, MiyuBookmarks, MiyuCalc, MiyuComptaReports, MiyuContacts, MiyuDeclarations, MiyuDiscovery, MiyuExport, MiyuFeeds, MiyuHR, MiyuJobs, MiyuLocale, MiyuModerationForum, MiyuPolls, MiyuPosAnalytics, MiyuPosInventory, MiyuPosKitchen, MiyuPosLoyalty, MiyuPosPayment, MiyuPosSales, MiyuProfile, MiyuSocialFeed, MiyuSocialMessaging, MiyuSocialModeration, MiyuSocialProfile, MiyuStory, MiyuText, MiyuValidate.

---

## 5. Recommandations

### 5.1 Avant implÃ©mentation (kits Â« Avec prÃ©cautions Â»)

Pour **rÃ©duire le risque de dÃ©viation** avant de coder :

1. **Option recommandÃ©e :** rÃ©diger un **Reference Implementation Guidelines** minimal (dÃ©rivÃ© du [template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md)) : objectif, sources contractuelles, **tableau BOUND-1 Ã  BOUND-6** adaptÃ© au kit, interdictions, gestion dâ€™erreurs et traÃ§abilitÃ©. Une Ã  deux pages par kit suffisent.
2. **Option alternative :** rÃ©diger un **Runtime Boundary Contract** (`contracts/boundaries/<MiyuXXX> - Runtime Boundary Contract.md`) qui formalise les BOUND-* pour le kit (voir [Template - Ecriture Enrichie Toolkits](..//contrats//Template%20-%20Ecriture%20Enrichie%20Toolkits.md) Â§ 7).
3. **Minimum :** avant toute implÃ©mentation, faire lire Ã  lâ€™implÃ©menteur le [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) (sections 2 et 3 : principes et tableau BOUND-*) et le contrat Governance du kit.

### 5.2 Priorisation des guides

En prioritÃ© pour les kits Ã  **fort impact** (donnÃ©es sensibles, commerce, facturation, contenu) :

- MiyuBilling, MiyuStore, MiyuShipping (commerce / paiement)
- MiyuCMS, MiyuMedia (contenu)
- MiyuAuth (identitÃ© â€” dÃ©jÃ  prÃªt)
- MiyuInvoice, MiyuComptaLedger, MiyuExpense, MiyuTreasury (compta / finance)
- MiyuWidgets (page builder, exposition web)

### 5.3 VÃ©rification continue

- Lors de lâ€™ajout dâ€™un **nouveau Toolkit** : prÃ©voir dÃ¨s la documentation enrichie soit un Reference Implementation Guidelines, soit un Runtime Boundary Contract.
- Lors dâ€™une **revue dâ€™implÃ©mentation** : utiliser le tableau BOUND-* comme checklist (aucune dÃ©cision ALLOW/DENY, pas dâ€™accÃ¨s direct, pas de modification du contexte, pas dâ€™identitÃ© OpÃ©rateur, uniquement les ToolIds dÃ©clarÃ©s).

---

## 6. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| Protocole Ecriture Enrichie Toolkits | [Miyukini Protocol - Ecriture Enrichie Toolkits](..//contrats//Miyukini%20Protocol%20-%20Ecriture%20Enrichie%20Toolkits.md) |
| Template Reference Implementation Guidelines | [docs_tools - Reference Implementation Guidelines Template](./docs_tools%20-%20Reference%20Implementation%20Guidelines%20Template.md) |
| Audit QualitÃ© ConformitÃ© SÃ©curitÃ© | [docs_tools - Audit Qualite Conformite Securite Implementation](./docs_tools%20-%20Audit%20Qualite%20Conformite%20Securite%20Implementation.md) |
| Master Butler - Tool Governance Compliance Template | [Master Butler - Tool Governance Compliance Template](..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md) |
| Template Ecriture Enrichie (Runtime Boundary) | [Template - Ecriture Enrichie Toolkits](..//contrats//Template%20-%20Ecriture%20Enrichie%20Toolkits.md) Â§ 7 |
| Index docs/tools | [_index](./_index.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Statut :** Rapport de vÃ©rification â€” Ã  mettre Ã  jour aprÃ¨s ajout de guides ou contrats Boundary

