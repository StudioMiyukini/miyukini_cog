# WorrySentinel - Vocabulary & Glossary

## 1. Contexte

Ce document dÃ©finit le **vocabulaire canonique** de WorrySentinel. Il Ã©tablit les dÃ©finitions officielles des termes utilisÃ©s dans la documentation, garantissant une comprÃ©hension uniforme et non ambiguÃ«.

**Document fondateur :** [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md)

**Statut :** Ce document est **normatif**. Les dÃ©finitions sont officielles et doivent Ãªtre utilisÃ©es de maniÃ¨re cohÃ©rente dans toute la documentation WorrySentinel.

---

## 2. Termes fondamentaux

### 2.1 WorrySentinel

**DÃ©finition canonique :**

> WorrySentinel est le **core de gouvernance de sÃ©curitÃ© transversale** du Miyukini Core System. Il gouverne les niveaux de sÃ©curitÃ©, les Ã©tats de confiance, et la dÃ©gradation progressive, sans jamais possÃ©der d'autoritÃ© sur l'implÃ©mentation, l'exÃ©cution, ou la persistance.

**CaractÃ©ristiques :**
- Core de gouvernance, pas fonctionnel
- Pression verticale, pas brique horizontale
- Strate 4 dans la Pyramide Miyukini
- Gouverne sans exÃ©cuter

**Ne pas confondre avec :**
- Un systÃ¨me de sÃ©curitÃ© (WorrySentinel ne fait pas de sÃ©curitÃ©, il la gouverne)
- Un contrÃ´leur (WorrySentinel ne contrÃ´le pas, il contraint)
- Un exÃ©cuteur (WorrySentinel ne rÃ©alise pas d'action)

---

### 2.2 Gouvernance

**DÃ©finition canonique :**

> La **gouvernance** est l'action de dÃ©finir des rÃ¨gles, des contraintes, et des niveaux qui influencent le comportement des cores fonctionnels, sans jamais implÃ©menter ou exÃ©cuter ces rÃ¨gles directement.

**Distinction clÃ© :**

| Gouvernance (WorrySentinel) | ImplÃ©mentation (Cores fonctionnels) |
|-----------------------------|-------------------------------------|
| DÃ©finit les rÃ¨gles | Applique les rÃ¨gles |
| Ã‰tablit les contraintes | ExÃ©cute selon les contraintes |
| DÃ©clare les Ã©tats | RÃ©agit aux Ã©tats dÃ©clarÃ©s |
| Ne possÃ¨de pas de logique technique | PossÃ¨de la logique technique |

**Invariant associÃ© :** INV-GOV-7 (SÃ©paration gouvernance/implÃ©mentation)

---

### 2.3 Pression verticale

**DÃ©finition canonique :**

> La **pression verticale** est la capacitÃ© de WorrySentinel Ã  contraindre tous les cores fonctionnels de maniÃ¨re transversale, traversant toutes les couches de l'architecture sans appartenir Ã  aucune.

**Visualisation :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Cores fonctionnels (Strate 5)                        â”‚
â”‚ â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”    â”‚
â”‚ â”‚ SF      â”‚ â”‚ KM      â”‚ â”‚ MB      â”‚ â”‚ BG      â”‚    â”‚
â”‚ â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â”‚
â”‚      â”‚          â”‚          â”‚          â”‚            â”‚
â”‚      â–¼          â–¼          â–¼          â–¼            â”‚
â”‚   â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•       â”‚
â”‚              PRESSION WORRYSENTINEL                 â”‚
â”‚   â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•       â”‚
â”‚                                                      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**CaractÃ©ristiques :**
- Traverse toutes les couches
- N'appartient Ã  aucune couche spÃ©cifique
- Contraint sans remplacer
- Non nÃ©gociable

---

## 3. Niveaux de sÃ©curitÃ©

### 3.1 Niveau de sÃ©curitÃ©

**DÃ©finition canonique :**

> Un **niveau de sÃ©curitÃ©** est un profil de risque (0-4) attribuÃ© Ã  un produit ou composant, caractÃ©risant le degrÃ© de sensibilitÃ© des donnÃ©es et dÃ©terminant les contraintes de sÃ©curitÃ© applicables.

**Les cinq niveaux :**

| Niveau | Code | DÃ©signation | Profil de risque |
|--------|------|-------------|------------------|
| **0** | N0 | Public / Display | Minimal |
| **1** | N1 | Standard / CMS | Faible |
| **2** | N2 | Sensitive Data | ModÃ©rÃ© |
| **3** | N3 | Critical System | Ã‰levÃ© |
| **4** | N4 | Hardened / Isolated | Maximal |

**Invariant associÃ© :** INV-GOV-1 (Niveaux de sÃ©curitÃ© explicites)

---

### 3.2 Niveau 0 â€” Public

**DÃ©finition canonique :**

> Le **niveau 0 (Public)** caractÃ©rise les produits et composants manipulant des donnÃ©es publiques, sans sensibilitÃ©, et pour lesquels aucune contrainte de sÃ©curitÃ© stricte n'est requise.

**Principe directeur :** *"Si Ã§a casse, ce n'est pas grave."*

**Cas d'usage :** Site vitrine, dashboards en lecture seule, affichage public.

---

### 3.3 Niveau 1 â€” Standard

**DÃ©finition canonique :**

> Le **niveau 1 (Standard)** caractÃ©rise les produits et composants manipulant des donnÃ©es standard, avec une sensibilitÃ© faible, nÃ©cessitant des contraintes de sÃ©curitÃ© de base.

**Principe directeur :** *"On protÃ¨ge l'accÃ¨s, pas le systÃ¨me."*

**Cas d'usage :** CMS, backoffice simple, contenu Ã©ditorial.

---

### 3.4 Niveau 2 â€” Sensitive Data

**DÃ©finition canonique :**

> Le **niveau 2 (Sensitive Data)** caractÃ©rise les produits et composants manipulant des donnÃ©es sensibles nÃ©cessitant une protection renforcÃ©e.

**Principe directeur :** *"On protÃ¨ge les donnÃ©es."*

**Cas d'usage :** DonnÃ©es personnelles, comptes utilisateurs, profils.

---

### 3.5 Niveau 3 â€” Critical System

**DÃ©finition canonique :**

> Le **niveau 3 (Critical System)** caractÃ©rise les produits et composants manipulant des donnÃ©es critiques nÃ©cessitant une protection maximale.

**Principe directeur :** *"On protÃ¨ge le systÃ¨me avant l'UX."*

**Cas d'usage :** Authentification, paiement, autorisations, cores internes.

---

### 3.6 Niveau 4 â€” Hardened

**DÃ©finition canonique :**

> Le **niveau 4 (Hardened / Isolated)** caractÃ©rise les produits et composants nÃ©cessitant une sÃ©curitÃ© maximale, avec des contraintes absolues.

**Principe directeur :** *"On protÃ¨ge l'intÃ©gritÃ© coÃ»te que coÃ»te."*

**Cas d'usage :** Environnement isolÃ©, hardware non fiable, contexte hostile.

---

## 4. Ã‰tats de confiance

### 4.1 Ã‰tat de confiance

**DÃ©finition canonique :**

> Un **Ã©tat de confiance** est un niveau d'intÃ©gritÃ© (T0-T4) caractÃ©risant la santÃ© globale du systÃ¨me Ã  un instant donnÃ©, dÃ©terminant les capacitÃ©s disponibles et les restrictions applicables.

**Les cinq Ã©tats :**

| Ã‰tat | Code | DÃ©signation | Signification |
|------|------|-------------|---------------|
| **Normal** | T0 | Nominal | SystÃ¨me sain |
| **Instable** | T1 | Doute | Anomalie dÃ©tectÃ©e |
| **DÃ©gradÃ©** | T2 | Suspect | IncohÃ©rence persistante |
| **Restreint** | T3 | Critique | Suspicion forte |
| **BloquÃ©** | T4 | Compromis | IntÃ©gritÃ© rompue |

**Invariant associÃ© :** INV-GOV-2 (Ã‰tats de confiance uniques)

---

### 4.2 T0 â€” Normal

**DÃ©finition canonique :**

> L'Ã©tat **T0 (Normal)** indique un systÃ¨me sain, sans anomalie dÃ©tectÃ©e, oÃ¹ toutes les capacitÃ©s sont disponibles et le monitoring est standard.

**Symbole :** ðŸŸ¢

**CapacitÃ©s :** Toutes les capacitÃ©s disponibles, dÃ©cisions normales, extensions dynamiques autorisÃ©es.

---

### 4.3 T1 â€” Instable

**DÃ©finition canonique :**

> L'Ã©tat **T1 (Instable)** indique qu'une anomalie a Ã©tÃ© dÃ©tectÃ©e mais pas encore confirmÃ©e, nÃ©cessitant un log renforcÃ© et une traÃ§abilitÃ© Ã©tendue sans blocage.

**Symbole :** ðŸŸ¡

**CapacitÃ©s :** Log renforcÃ©, traÃ§abilitÃ© Ã©tendue, aucun blocage, surveillance accrue.

---

### 4.4 T2 â€” DÃ©gradÃ©

**DÃ©finition canonique :**

> L'Ã©tat **T2 (DÃ©gradÃ©)** indique une incohÃ©rence persistante nÃ©cessitant la dÃ©sactivation de certaines capacitÃ©s et des dÃ©cisions plus strictes.

**Symbole :** ðŸŸ 

**CapacitÃ©s :** Certaines capacitÃ©s dÃ©sactivÃ©es, refus des extensions dynamiques, monitoring visible.

---

### 4.5 T3 â€” Restreint

**DÃ©finition canonique :**

> L'Ã©tat **T3 (Restreint)** indique une suspicion forte d'intÃ©gritÃ© potentiellement compromise, nÃ©cessitant le gel des produits non essentiels et l'intervention TAMR.

**Symbole :** ðŸ”´

**CapacitÃ©s :** Gel des produits non essentiels, refus de nouveaux modules, TAMR requis pour override.

---

### 4.6 T4 â€” BloquÃ©

**DÃ©finition canonique :**

> L'Ã©tat **T4 (BloquÃ©)** indique que l'intÃ©gritÃ© du systÃ¨me est rompue, nÃ©cessitant l'arrÃªt de toute dÃ©cision opÃ©rationnelle et ne permettant que les diagnostics.

**Symbole :** â›”

**CapacitÃ©s :** Uniquement diagnostics, Ã©tat lisible, sortie propre possible.

**CaractÃ©ristique :** Ã‰tat terminal â€” aucune transition sortante.

---

## 5. DÃ©gradation progressive

### 5.1 DÃ©gradation progressive

**DÃ©finition canonique :**

> La **dÃ©gradation progressive** est le mÃ©canisme par lequel WorrySentinel orchestre la rÃ©duction contrÃ´lÃ©e des capacitÃ©s du systÃ¨me selon l'Ã©volution de l'Ã©tat de confiance, garantissant qu'aucun blocage brutal ne se produise.

**Principe fondamental :**

> *"Un systÃ¨me autonome ne bloque jamais brutalement. Il observe, interprÃ¨te, dÃ©grade, puis bloque seulement quand il est sÃ»r."*

**Invariant associÃ© :** INV-GOV-4 (DÃ©gradation progressive uniquement)

---

### 5.2 Transition d'Ã©tat

**DÃ©finition canonique :**

> Une **transition d'Ã©tat** est le passage d'un Ã©tat de confiance Ã  un autre, gouvernÃ© par des rÃ¨gles explicites et toujours justifiÃ© et tracÃ©.

**Transitions autorisÃ©es :**

| De | Vers | Condition |
|----|------|-----------|
| T0 | T1 | DÃ©tection d'anomalie |
| T1 | T0 | RÃ©solution d'anomalie |
| T1 | T2 | Persistance d'anomalie |
| T2 | T1 | AmÃ©lioration de l'Ã©tat |
| T2 | T3 | Aggravation de l'Ã©tat |
| T3 | T2 | Confirmation de sÃ©curitÃ© |
| T3 | T4 | Confirmation de compromission |

**RÃ¨gle :** Les transitions directes (ex: T0â†’T4) sont interdites.

**Invariant associÃ© :** INV-GOV-3 (Transitions justifiÃ©es)

---

## 6. Invariants

### 6.1 Invariant

**DÃ©finition canonique :**

> Un **invariant** est une rÃ¨gle absolue qui ne peut jamais Ãªtre violÃ©e, quel que soit le contexte, la situation, ou les considÃ©rations pratiques.

**CaractÃ©ristiques d'un invariant :**
- Ne peut jamais Ãªtre violÃ©
- Est vÃ©rifiable
- Est indÃ©pendant du contexte
- Est non nÃ©gociable

**ConsÃ©quence de violation :** Faute architecturale fondamentale.

---

### 6.2 Invariants WorrySentinel (INV-WS)

| Code | Ã‰noncÃ© court |
|------|--------------|
| **INV-WS-1** | Aucune autoritÃ© sur l'implÃ©mentation |
| **INV-WS-2** | Aucune autoritÃ© sur l'exÃ©cution |
| **INV-WS-3** | Aucune autoritÃ© sur la persistance |
| **INV-WS-4** | Aucune modification d'Ã©tat |
| **INV-WS-5** | Aucune logique temporelle technique |
| **INV-WS-6** | Zero-trust |
| **INV-WS-7** | Gouvernance explicite |
| **INV-WS-8** | TraÃ§abilitÃ© complÃ¨te |

---

### 6.3 Invariants de gouvernance (INV-GOV)

| Code | Ã‰noncÃ© court |
|------|--------------|
| **INV-GOV-1** | Niveaux de sÃ©curitÃ© explicites |
| **INV-GOV-2** | Ã‰tats de confiance uniques |
| **INV-GOV-3** | Transitions justifiÃ©es |
| **INV-GOV-4** | DÃ©gradation progressive uniquement |
| **INV-GOV-5** | PrÃ©servation des invariants |
| **INV-GOV-6** | CohÃ©rence inter-composants |
| **INV-GOV-7** | SÃ©paration gouvernance/implÃ©mentation |
| **INV-GOV-8** | TraÃ§abilitÃ© complÃ¨te de gouvernance |

---

## 7. Concepts complÃ©mentaires

### 7.1 Zero-trust

**DÃ©finition canonique :**

> Le **zero-trust** est le principe selon lequel WorrySentinel ne fait confiance Ã  aucun appelant et Ã©value chaque demande selon les rÃ¨gles, sans prÃ©supposer la validitÃ©, l'authenticitÃ©, ou la lÃ©gitimitÃ©.

**Invariant associÃ© :** INV-WS-6

---

### 7.2 Contrainte

**DÃ©finition canonique :**

> Une **contrainte** est une rÃ¨gle imposÃ©e par WorrySentinel aux cores fonctionnels, dÃ©finissant les limites de leur comportement selon le niveau de sÃ©curitÃ© et l'Ã©tat de confiance.

**Types de contraintes :**
- Contraintes de sÃ©vÃ©ritÃ© (StrongFather)
- Contraintes de permissions (MasterButler)
- Contraintes de frontiÃ¨res (BorderGuard)
- Contraintes de monitoring (CaringNanny)
- Contraintes de ressources (LogisticsSteward)

---

### 7.3 Signal d'intÃ©gritÃ©

**DÃ©finition canonique :**

> Un **signal d'intÃ©gritÃ©** est une information remontÃ©e par un core fonctionnel vers WorrySentinel, indiquant une anomalie, une incohÃ©rence, ou un Ã©tat particulier du systÃ¨me.

**Sources de signaux :**
- Kernel (signaux clock, id, traces)
- StrongFather (dÃ©cisions refusÃ©es)
- BorderGuard (anomalies I/O)
- CaringNanny (anomalies monitoring)
- KindMother (incohÃ©rences donnÃ©es)
- LogisticsSteward (dÃ©rives allocation)

---

### 7.4 CorrÃ©lation

**DÃ©finition canonique :**

> La **corrÃ©lation** est l'action de WorrySentinel de croiser et analyser les signaux d'intÃ©gritÃ© provenant de multiples sources pour dÃ©terminer l'Ã©tat de confiance global du systÃ¨me.

**Processus :**
1. RÃ©ception des signaux
2. Analyse de cohÃ©rence
3. DÃ©tection de patterns
4. DÃ©claration d'Ã©tat

---

### 7.5 TraÃ§abilitÃ©

**DÃ©finition canonique :**

> La **traÃ§abilitÃ©** est la capacitÃ© de retracer toute dÃ©cision de gouvernance avec son contexte, ses rÃ¨gles appliquÃ©es, et sa justification.

**Ã‰lÃ©ments de traÃ§abilitÃ© obligatoires :**
- Contexte de la dÃ©cision
- RÃ¨gles appliquÃ©es
- Justification
- Niveau de sÃ©curitÃ©
- Ã‰tat de confiance
- RÃ©sultat

**Invariants associÃ©s :** INV-WS-8, INV-GOV-8

---

### 7.6 MÃ©diation

**DÃ©finition canonique :**

> La **mÃ©diation** est le processus de validation gouvernÃ© par WorrySentinel permettant Ã  un composant d'accÃ©der Ã  un niveau de sÃ©curitÃ© supÃ©rieur au sien.

**RÃ¨gle :** Un composant de niveau N ne peut pas accÃ©der directement Ã  un composant de niveau > N sans mÃ©diation explicite.

**Invariant associÃ© :** INV-GOV-6

---

## 8. Termes Ã  ne pas confondre

### 8.1 Niveau de sÃ©curitÃ© vs Ã‰tat de confiance

| Aspect | Niveau de sÃ©curitÃ© (0-4) | Ã‰tat de confiance (T0-T4) |
|--------|--------------------------|---------------------------|
| **Nature** | Profil de risque | Ã‰tat d'intÃ©gritÃ© |
| **PortÃ©e** | Produit/composant | SystÃ¨me global |
| **StabilitÃ©** | Statique (pendant opÃ©ration) | Dynamique |
| **DÃ©terminÃ© par** | Profil du produit | Signaux d'intÃ©gritÃ© |

### 8.2 Gouvernance vs ImplÃ©mentation

| Aspect | Gouvernance | ImplÃ©mentation |
|--------|-------------|----------------|
| **Responsable** | WorrySentinel | Cores fonctionnels |
| **Action** | DÃ©finir les rÃ¨gles | Appliquer les rÃ¨gles |
| **Nature** | DÃ©clarative | ImpÃ©rative |
| **ExÃ©cution** | Jamais | Toujours |

### 8.3 Contrainte vs ContrÃ´le

| Aspect | Contrainte | ContrÃ´le |
|--------|------------|----------|
| **Source** | WorrySentinel | Cores fonctionnels |
| **Nature** | DÃ©clarative | ExÃ©cutive |
| **Action** | Limite le comportement | VÃ©rifie et applique |

---

## 9. RÃ©fÃ©rences croisÃ©es

| Document | Relation |
|----------|----------|
| [WorrySentinel - Documentation Fondatrice](../foundation/WorrySentinel%20-%20Documentation%20Fondatrice.md) | DÃ©finitions officielles |
| [WorrySentinel - Invariants & Guarantees](../contracts/governance/WorrySentinel%20-%20Invariants%20&%20Guarantees.md) | Invariants dÃ©taillÃ©s |
| [Miyukini Conceptual References - Security Levels](..//..//..//miyukini-webway-system//reference//_index.md) | Niveaux de sÃ©curitÃ© complets |
| [Miyukini Conceptual References - Integrity Degradation System](..//..//..//miyukini-webway-system//reference//_index.md) | Ã‰tats de confiance complets |
| [Miyukini Conceptual References - Glossaire](..//..//..//miyukini-webway-system//reference//_index.md) | Glossaire global Miyukini |

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif â€” Vocabulaire officiel  
**Type :** Glossaire et vocabulaire

