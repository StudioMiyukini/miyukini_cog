# Border Guard - Crossing Rules Contract

## 1. Contexte

Ce document dÃ©finit les **rÃ¨gles de franchissement** gouvernÃ©es par Border Guard dans l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie formellement ce qu'est une rÃ¨gle de franchissement, sa structure, ses types, et les conditions d'application selon les frontiÃ¨res et les niveaux de confiance.

**Document fondateur :** [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md)

**Statut contractuel :** Ce document est **contractuel, normatif, et non nÃ©gociable**. Il dÃ©rive directement de la Documentation Fondatrice (Section 4 - Concepts fondamentaux : RÃ¨gle de franchissement).

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toute interaction traversant une frontiÃ¨re dans l'Ã©cosystÃ¨me Miyukini
- **Responsable :** Border Guard (responsabilitÃ© exclusive d'Ã©tablissement des rÃ¨gles - Documentation Fondatrice Section 5)
- **ExÃ©cutant :** BondingBrother (application des rÃ¨gles - INV-BG-7)
- **Consommateurs :** BondingBrother (application), StrongFather (contexte de dÃ©cision)
- **Ne couvre pas :** L'application technique des rÃ¨gles (responsabilitÃ© de BondingBrother)

---

## 3. DÃ©finition canonique de la rÃ¨gle de franchissement

### 3.1 Qu'est-ce qu'une rÃ¨gle de franchissement ?

Une **rÃ¨gle de franchissement** est une condition dÃ©clarative qui doit Ãªtre satisfaite pour qu'une interaction puisse traverser une frontiÃ¨re. Elle exprime **ce qui est requis**, pas **comment le vÃ©rifier techniquement**.

**CaractÃ©ristiques fondamentales :**

1. **DÃ©clarative** â€” Exprime une condition, pas une procÃ©dure
2. **Non ambiguÃ«** â€” SpÃ©cifie clairement les conditions sans interprÃ©tation possible
3. **AssociÃ©e** â€” LiÃ©e Ã  une frontiÃ¨re spÃ©cifique ou un ensemble de frontiÃ¨res
4. **IndÃ©pendante** â€” Ne dÃ©pend pas de l'implÃ©mentation technique

**Ce qu'une rÃ¨gle de franchissement n'est PAS :**

- âŒ Un algorithme de validation
- âŒ Un code de vÃ©rification
- âŒ Une procÃ©dure d'authentification
- âŒ Une rÃ¨gle de filtrage technique

### 3.2 ResponsabilitÃ© de Border Guard

Border Guard est **exclusivement responsable** de l'Ã©tablissement des rÃ¨gles de franchissement. Cette responsabilitÃ© inclut :

- DÃ©finir les rÃ¨gles associÃ©es Ã  chaque frontiÃ¨re
- SpÃ©cifier les conditions de franchissement
- Ã‰tablir les exceptions et cas particuliers
- Maintenir la cohÃ©rence des rÃ¨gles entre frontiÃ¨res

**Invariant associÃ© :** INV-BG-6 â€” Toutes les rÃ¨gles de franchissement **doivent** Ãªtre dÃ©claratives. Aucune rÃ¨gle procÃ©durale ou impÃ©rative n'est autorisÃ©e.

### 3.3 SÃ©paration dÃ©finition/application

**Border Guard dÃ©finit.** BondingBrother applique.

| ResponsabilitÃ© | Border Guard | BondingBrother |
|----------------|--------------|----------------|
| DÃ©finir les rÃ¨gles | âœ… | âŒ |
| SpÃ©cifier les conditions | âœ… | âŒ |
| ImplÃ©menter la vÃ©rification | âŒ | âœ… |
| ExÃ©cuter le blocage | âŒ | âœ… |
| Tracer l'application | âŒ | âœ… |

**Invariant associÃ© :** INV-BG-7 â€” La dÃ©finition des frontiÃ¨res et des rÃ¨gles est **strictement sÃ©parÃ©e** de leur application.

---

## 4. Structure d'une rÃ¨gle de franchissement

### 4.1 PropriÃ©tÃ©s obligatoires

Toute rÃ¨gle de franchissement possÃ¨de les propriÃ©tÃ©s suivantes :

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant unique de la rÃ¨gle | âœ… Oui |
| **Nom** | Nom descriptif de la rÃ¨gle | âœ… Oui |
| **Description** | Description de ce que la rÃ¨gle vÃ©rifie | âœ… Oui |
| **FrontiÃ¨re(s)** | FrontiÃ¨re(s) Ã  laquelle/lesquelles la rÃ¨gle s'applique | âœ… Oui |
| **Condition** | Condition dÃ©clarative Ã  satisfaire | âœ… Oui |
| **Type** | Type de rÃ¨gle (niveau, authentification, donnÃ©es, etc.) | âœ… Oui |
| **PrioritÃ©** | PrioritÃ© d'Ã©valuation | âœ… Oui |
| **Action si Ã©chec** | Que faire si la condition n'est pas satisfaite | âœ… Oui |

### 4.2 PropriÃ©tÃ©s optionnelles

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Exceptions** | Cas oÃ¹ la rÃ¨gle ne s'applique pas | âŒ Non |
| **Conditions prÃ©alables** | Autres rÃ¨gles qui doivent Ãªtre satisfaites avant | âŒ Non |
| **Niveau de sÃ©curitÃ©** | Niveau de sÃ©curitÃ© minimum pour appliquer cette rÃ¨gle | âŒ Non |
| **MÃ©tadonnÃ©es** | Informations supplÃ©mentaires | âŒ Non |

### 4.3 Format dÃ©claratif

Une rÃ¨gle est exprimÃ©e de maniÃ¨re dÃ©clarative selon le format :

```
RÃˆGLE: <identifiant>
NOM: <nom descriptif>
FRONTIÃˆRE: <frontiÃ¨re(s) cible>
CONDITION: <condition dÃ©clarative>
TYPE: <type de rÃ¨gle>
PRIORITÃ‰: <1-100>
Ã‰CHEC: <action si Ã©chec>
```

**Exemple :**

```
RÃˆGLE: CROSS-EXT-001
NOM: Niveau de confiance minimum pour frontiÃ¨re externe
FRONTIÃˆRE: FrontiÃ¨res externes
CONDITION: source.trust_level â‰¥ VERIFIED
TYPE: niveau_confiance
PRIORITÃ‰: 10
Ã‰CHEC: REFUSER
```

---

## 5. Types de rÃ¨gles de franchissement

Border Guard reconnaÃ®t cinq types canoniques de rÃ¨gles.

### 5.1 RÃ¨gles de niveau de confiance

**DÃ©finition :** RÃ¨gles portant sur le niveau de confiance requis pour franchir une frontiÃ¨re.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Code type** | `niveau_confiance` |
| **Condition type** | Comparaison de niveau (â‰¥, =, â‰¤) |
| **Variables** | `source.trust_level`, `destination.trust_level` |

**Exemples :**

| RÃ¨gle | Condition | Description |
|-------|-----------|-------------|
| CROSS-TRUST-001 | `source.trust_level â‰¥ VERIFIED` | Source doit Ãªtre au moins VERIFIED |
| CROSS-TRUST-002 | `source.trust_level = TRUSTED` | Source doit Ãªtre TRUSTED |
| CROSS-TRUST-003 | `source.trust_level â‰  HOSTILE` | Source ne doit pas Ãªtre HOSTILE |

### 5.2 RÃ¨gles d'authentification

**DÃ©finition :** RÃ¨gles portant sur l'Ã©tat d'authentification requis pour franchir une frontiÃ¨re.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Code type** | `authentification` |
| **Condition type** | Ã‰tat d'authentification |
| **Variables** | `source.authenticated`, `source.auth_method`, `source.session_valid` |

**Exemples :**

| RÃ¨gle | Condition | Description |
|-------|-----------|-------------|
| CROSS-AUTH-001 | `source.authenticated = true` | Source doit Ãªtre authentifiÃ©e |
| CROSS-AUTH-002 | `source.session_valid = true` | Session doit Ãªtre valide |
| CROSS-AUTH-003 | `source.auth_method IN [MFA, SSO]` | MÃ©thode d'auth renforcÃ©e requise |

**Note :** Border Guard ne gÃ¨re pas l'authentification technique. Ces rÃ¨gles vÃ©rifient l'**Ã©tat** d'authentification fourni par le module auth.

### 5.3 RÃ¨gles de donnÃ©es

**DÃ©finition :** RÃ¨gles portant sur les donnÃ©es autorisÃ©es Ã  traverser une frontiÃ¨re.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Code type** | `donnees` |
| **Condition type** | Nature ou classification des donnÃ©es |
| **Variables** | `data.classification`, `data.type`, `data.sensitivity` |

**Exemples :**

| RÃ¨gle | Condition | Description |
|-------|-----------|-------------|
| CROSS-DATA-001 | `data.classification â‰¤ PUBLIC` | Seules donnÃ©es publiques autorisÃ©es |
| CROSS-DATA-002 | `data.sensitivity â‰  CRITICAL` | DonnÃ©es critiques interdites |
| CROSS-DATA-003 | `data.type IN [text, json, image]` | Types de donnÃ©es autorisÃ©s |

### 5.4 RÃ¨gles d'action

**DÃ©finition :** RÃ¨gles portant sur les actions autorisÃ©es Ã  traverser une frontiÃ¨re.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Code type** | `action` |
| **Condition type** | Nature de l'action |
| **Variables** | `action.type`, `action.scope`, `action.impact` |

**Exemples :**

| RÃ¨gle | Condition | Description |
|-------|-----------|-------------|
| CROSS-ACT-001 | `action.type = READ` | Seule lecture autorisÃ©e |
| CROSS-ACT-002 | `action.impact â‰¤ LOW` | Actions Ã  faible impact uniquement |
| CROSS-ACT-003 | `action.type NOT IN [DELETE, DROP]` | Actions destructives interdites |

### 5.5 RÃ¨gles temporelles

**DÃ©finition :** RÃ¨gles portant sur les conditions temporelles de franchissement.

| Aspect | SpÃ©cification |
|--------|---------------|
| **Code type** | `temporel` |
| **Condition type** | Contraintes de temps |
| **Variables** | `timestamp`, `session.age`, `rate.limit` |

**Exemples :**

| RÃ¨gle | Condition | Description |
|-------|-----------|-------------|
| CROSS-TIME-001 | `session.age < 1h` | Session de moins d'une heure |
| CROSS-TIME-002 | `rate.requests_per_minute â‰¤ 100` | Limite de dÃ©bit |
| CROSS-TIME-003 | `timestamp.hour IN [9, 18]` | Heures ouvrables uniquement |

---

## 6. PrioritÃ©s et Ã©valuation

### 6.1 Niveaux de prioritÃ©

Les rÃ¨gles sont Ã©valuÃ©es selon leur prioritÃ© (1 = plus haute prioritÃ©).

| Plage | Description | Exemples |
|-------|-------------|----------|
| **1-10** | RÃ¨gles de sÃ©curitÃ© critiques | Blocage hostile, rate limiting |
| **11-30** | RÃ¨gles de niveau de confiance | VÃ©rification TRUSTED, VERIFIED |
| **31-50** | RÃ¨gles d'authentification | Session valide, MFA |
| **51-70** | RÃ¨gles de donnÃ©es | Classification, types |
| **71-90** | RÃ¨gles d'action | Lecture seule, impact |
| **91-100** | RÃ¨gles temporelles et autres | Heures, quotas |

### 6.2 Algorithme d'Ã©valuation

```
POUR chaque rÃ¨gle R ordonnÃ©e par prioritÃ© :
    SI R.frontiÃ¨re correspond Ã  la frontiÃ¨re traversÃ©e :
        SI NON satisfaite(R.condition) :
            RETOURNER R.action_echec
        FIN SI
    FIN SI
FIN POUR
RETOURNER AUTORISER
```

**RÃ¨gle fondamentale :** L'Ã©valuation s'arrÃªte Ã  la premiÃ¨re rÃ¨gle non satisfaite.

### 6.3 Combinaison de rÃ¨gles

| Combinaison | Description | Comportement |
|-------------|-------------|--------------|
| **ET (implicite)** | Toutes les rÃ¨gles doivent Ãªtre satisfaites | Ã‰chec si une seule Ã©choue |
| **OU (explicite)** | Au moins une rÃ¨gle doit Ãªtre satisfaite | RÃ¨gles alternatives |
| **EXCEPTION** | La rÃ¨gle ne s'applique pas dans ce cas | Bypass contrÃ´lÃ© |

---

## 7. Actions en cas d'Ã©chec

### 7.1 Actions canoniques

| Action | Code | Description | GravitÃ© |
|--------|------|-------------|---------|
| **REFUSER** | `DENY` | Refuser le franchissement | Ã‰levÃ©e |
| **DIFFÃ‰RER** | `DEFER` | Soumettre Ã  StrongFather pour dÃ©cision | Moyenne |
| **DÃ‰GRADER** | `DEGRADE` | Autoriser avec restrictions | Faible |
| **ALERTER** | `ALERT` | Autoriser mais alerter | Information |
| **JOURNALISER** | `LOG` | Autoriser et journaliser | Information |

### 7.2 Matrice action/gravitÃ©

| GravitÃ© de la violation | Action recommandÃ©e |
|------------------------|-------------------|
| Critique (HOSTILE, violation grave) | `DENY` |
| Ã‰levÃ©e (rÃ¨gle de sÃ©curitÃ©) | `DENY` ou `DEFER` |
| Moyenne (rÃ¨gle d'authentification) | `DEFER` ou `DEGRADE` |
| Faible (rÃ¨gle de donnÃ©es/action) | `DEGRADE` ou `ALERT` |
| Information (rÃ¨gle temporelle) | `ALERT` ou `LOG` |

### 7.3 Escalade

```
Ã‰chec rÃ¨gle
    â”‚
    â”œâ”€â”€ GravitÃ© critique â”€â”€â”€â–º DENY (immÃ©diat)
    â”‚
    â”œâ”€â”€ GravitÃ© Ã©levÃ©e â”€â”€â”€â–º DENY ou DEFER (selon contexte)
    â”‚
    â”œâ”€â”€ GravitÃ© moyenne â”€â”€â”€â–º DEFER Ã  StrongFather
    â”‚
    â””â”€â”€ GravitÃ© faible â”€â”€â”€â–º DEGRADE ou ALERT
```

---

## 8. RÃ¨gles par type de frontiÃ¨re

### 8.1 RÃ¨gles pour frontiÃ¨res externes

**Objectif :** Protection maximale contre les entrÃ©es non autorisÃ©es.

| RÃ¨gle | PrioritÃ© | Condition | Action Ã©chec |
|-------|----------|-----------|--------------|
| `EXT-001` | 1 | `source.trust_level â‰  HOSTILE` | DENY |
| `EXT-002` | 5 | `rate.requests_per_minute â‰¤ LIMIT` | DENY + ALERT |
| `EXT-003` | 15 | `source.trust_level â‰¥ UNKNOWN` | DENY |
| `EXT-004` | 35 | `source.authenticated = true` (si requis) | DEFER |
| `EXT-005` | 55 | `data.type IN ALLOWED_TYPES` | DENY |

### 8.2 RÃ¨gles pour frontiÃ¨res internes

**Objectif :** DÃ©fense en profondeur, cloisonnement des zones.

| RÃ¨gle | PrioritÃ© | Condition | Action Ã©chec |
|-------|----------|-----------|--------------|
| `INT-001` | 1 | `source.trust_level â‰  HOSTILE` | DENY |
| `INT-002` | 20 | `source.trust_level â‰¥ zone.required_level` | DEFER |
| `INT-003` | 40 | `source.session_valid = true` | DENY |
| `INT-004` | 60 | `action.authorized_in_zone = true` | DEFER |

### 8.3 RÃ¨gles pour frontiÃ¨res d'intÃ©gration

**Objectif :** ContrÃ´le des Ã©changes avec les systÃ¨mes intÃ©grÃ©s.

| RÃ¨gle | PrioritÃ© | Condition | Action Ã©chec |
|-------|----------|-----------|--------------|
| `INTEG-001` | 1 | `integration.status = ACTIVE` | DENY |
| `INTEG-002` | 10 | `source.trust_level â‰¥ VERIFIED` | DEFER |
| `INTEG-003` | 30 | `integration.credentials_valid = true` | DENY |
| `INTEG-004` | 50 | `data.classification â‰¤ integration.max_classification` | DENY |
| `INTEG-005` | 70 | `action.type IN integration.allowed_actions` | DEFER |

---

## 9. Adaptation selon les niveaux de sÃ©curitÃ©

Les rÃ¨gles de franchissement s'adaptent selon le niveau de sÃ©curitÃ© dÃ©clarÃ©.

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md)

### 9.1 Adaptation des seuils

| Niveau de sÃ©curitÃ© | Niveau confiance min. (externe) | Rate limit | TolÃ©rance erreur |
|--------------------|--------------------------------|------------|------------------|
| **0 - PUBLIC** | UNKNOWN | Haut | Haute |
| **1 - STANDARD** | UNKNOWN | Standard | Standard |
| **2 - SENSITIVE** | VERIFIED | RÃ©duit | Faible |
| **3 - CRITICAL** | VERIFIED+ | Strict | Minimale |
| **4 - HARDENED** | TRUSTED | Ultra-strict | ZÃ©ro |

### 9.2 RÃ¨gles spÃ©cifiques par niveau

#### Niveau 0 - PUBLIC

```
# RÃ¨gles assouplies
EXT-003: source.trust_level â‰¥ UNKNOWN (mÃªme non authentifiÃ© autorisÃ©)
INT-002: source.trust_level â‰¥ UNKNOWN (zones ouvertes)
```

#### Niveau 3 - CRITICAL

```
# RÃ¨gles strictes
EXT-003: source.trust_level â‰¥ VERIFIED (auth obligatoire)
EXT-ADD: source.auth_method IN [MFA, SSO] (auth renforcÃ©e)
INT-002: source.trust_level â‰¥ VERIFIED (cloisonnement strict)
```

#### Niveau 4 - HARDENED

```
# RÃ¨gles ultra-strictes
EXT-003: source.trust_level = TRUSTED (seuls trusted autorisÃ©s)
INT-002: source.trust_level = TRUSTED (isolement)
ALL: rate.limit = MINIMAL (quasi aucun trafic)
```

---

## 10. IntÃ©gration avec les protocoles de sÃ©curitÃ©

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md)

### 10.1 Protocoles temps rÃ©el (RT-SEC)

| Protocole | RÃ¨gles concernÃ©es |
|-----------|-------------------|
| **RT-SEC-1** (Session Ã©phÃ©mÃ¨re) | RÃ¨gles de session (`session.age`, `session.valid`) |
| **RT-SEC-2** (Auth en couches) | RÃ¨gles d'authentification (`auth_method`, `context`) |
| **RT-SEC-3** (Validation systÃ©matique) | Toutes les rÃ¨gles (aucun bypass) |
| **RT-SEC-4** (DÃ©tection anomalie) | RÃ¨gles de rate limiting, dÃ©tection patterns |

### 10.2 Protocoles asynchrones (AS-SEC)

| Protocole | RÃ¨gles concernÃ©es |
|-----------|-------------------|
| **AS-SEC-1** (Actions non engagÃ©es) | RÃ¨gles d'action (`action.status = PENDING`) |
| **AS-SEC-2** (Signature locale faible) | RÃ¨gles de signature (`signature.valid`) |
| **AS-SEC-3** (Revalidation) | Toutes les rÃ¨gles (rÃ©Ã©valuation complÃ¨te) |
| **AS-SEC-4** (Anti-replay) | RÃ¨gles de sÃ©quence (`request.id`, `request.timestamp`) |

### 10.3 Flux avec BondingBrother

```
Border Guard                          BondingBrother
     â”‚                                      â”‚
     â”‚ rÃ¨gles de franchissement             â”‚
     â”‚ (dÃ©claratives)                       â”‚
     â”‚ â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–º   â”‚
     â”‚                                      â”‚
     â”‚                                      â”‚ implÃ©mentation
     â”‚                                      â”‚ des vÃ©rifications
     â”‚                                      â”‚
     â”‚ rÃ©sultat application                 â”‚
     â”‚ (pour traÃ§abilitÃ©)                   â”‚
     â”‚ â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€   â”‚
```

---

## 11. Exceptions et cas particuliers

### 11.1 DÃ©finition d'une exception

Une **exception** est un cas oÃ¹ une rÃ¨gle ne s'applique pas, dÃ©fini de maniÃ¨re explicite et traÃ§able.

| PropriÃ©tÃ© | Description | Obligatoire |
|-----------|-------------|-------------|
| **Identifiant** | Identifiant de l'exception | âœ… Oui |
| **RÃ¨gle concernÃ©e** | Quelle rÃ¨gle est exceptÃ©e | âœ… Oui |
| **Condition d'exception** | Quand l'exception s'applique | âœ… Oui |
| **Justification** | Pourquoi cette exception existe | âœ… Oui |
| **DurÃ©e** | Temporaire ou permanente | âœ… Oui |

### 11.2 Exceptions autorisÃ©es

| Type d'exception | Conditions | AutoritÃ© |
|------------------|------------|----------|
| **Urgence sÃ©curitÃ©** | Faille critique, besoin immÃ©diat | TAMR + StrongFather |
| **Migration** | PÃ©riode de transition, compatibilitÃ© | EverBuddy |
| **Maintenance** | OpÃ©rations planifiÃ©es | Admin + CaringNanny |
| **Test** | Environnement de test uniquement | Environnement non-production |

### 11.3 Exceptions interdites

| Exception interdite | Raison |
|--------------------|--------|
| Exception permanente sans justification | Viole INV-BG-8 (traÃ§abilitÃ©) |
| Exception contournant HOSTILE | Viole sÃ©curitÃ© fondamentale |
| Exception dÃ©finie par BondingBrother | Viole INV-BG-7 (sÃ©paration) |
| Exception non traÃ§able | Viole INV-BG-8 |

---

## 12. TraÃ§abilitÃ© des rÃ¨gles

### 12.1 Ã‰lÃ©ments Ã  tracer

| Ã‰lÃ©ment | Obligatoire | Description |
|---------|-------------|-------------|
| RÃ¨gle Ã©valuÃ©e | âœ… Oui | Identifiant de la rÃ¨gle |
| Condition | âœ… Oui | Condition Ã©valuÃ©e |
| RÃ©sultat | âœ… Oui | Satisfaite / Non satisfaite |
| Action | âœ… Oui | Action exÃ©cutÃ©e si Ã©chec |
| Contexte | âœ… Oui | Contexte de l'Ã©valuation |
| Timestamp | âœ… Oui | Horodatage |

### 12.2 Format de trace

```
Crossing Rule Evaluation:
- rule_id: <identifiant>
- frontier_id: <frontiÃ¨re traversÃ©e>
- condition: <condition Ã©valuÃ©e>
- result: <PASS|FAIL>
- action_taken: <si FAIL>
- context: {source, destination, data, action}
- timestamp: <ISO 8601>
```

**Invariant associÃ© :** INV-BG-8 â€” Toute Ã©valuation de rÃ¨gle est **traÃ§able**.

---

## 13. RÃ©fÃ©rences croisÃ©es

### Invariants associÃ©s (Documentation Fondatrice - Section 7)

| Invariant | Ã‰noncÃ© | Relation |
|-----------|--------|----------|
| INV-BG-6 | RÃ¨gles dÃ©claratives | Fondement de ce contrat |
| INV-BG-7 | SÃ©paration dÃ©finition/application | Border Guard dÃ©finit, BondingBrother applique |
| INV-BG-8 | TraÃ§abilitÃ© complÃ¨te | Toute rÃ¨gle et Ã©valuation est traÃ§able |
| INV-BG-9 | CohÃ©rence globale | Pas de contradiction entre rÃ¨gles |

### Documents associÃ©s

| Document | Relation |
|----------|----------|
| [Border Guard - Documentation Fondatrice](../../foundation/Border%20Guard%20-%20Documentation%20Fondatrice.md) | Document source |
| [Border Guard - Boundary Definition Contract](./Border%20Guard%20-%20Boundary%20Definition%20Contract.md) | FrontiÃ¨res auxquelles les rÃ¨gles s'appliquent |
| [Border Guard - Trust Level Classification Contract](./Border%20Guard%20-%20Trust%20Level%20Classification%20Contract.md) | Niveaux utilisÃ©s dans les rÃ¨gles |
| [Miyukini Conceptual References - Security Levels](..//..//..//..//miyukini-webway-system//reference//_index.md) | Adaptation selon niveau sÃ©curitÃ© |
| [Miyukini Conceptual References - Security Protocols](..//..//..//..//miyukini-webway-system//reference//_index.md) | Protocoles utilisant les rÃ¨gles |

### RÃ©fÃ©rences glossaire

| Terme | DÃ©finition |
|-------|------------|
| **RÃ¨gle de franchissement** | Condition dÃ©clarative pour autoriser un franchissement |
| **Franchissement** | Acte de traverser une frontiÃ¨re |
| **Condition dÃ©clarative** | Expression de ce qui est requis, pas comment le vÃ©rifier |
| **PrioritÃ©** | Ordre d'Ã©valuation des rÃ¨gles |
| **Exception** | Cas oÃ¹ une rÃ¨gle ne s'applique pas |

**Source :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 14. SynthÃ¨se contractuelle

### Garanties de ce contrat

Ce contrat garantit que :

1. **Les rÃ¨gles sont dÃ©claratives** â€” Expriment ce qui est requis, pas comment le vÃ©rifier (INV-BG-6)
2. **La sÃ©paration est stricte** â€” Border Guard dÃ©finit, BondingBrother applique (INV-BG-7)
3. **Cinq types de rÃ¨gles** â€” Confiance, authentification, donnÃ©es, action, temporel
4. **L'Ã©valuation est dÃ©terministe** â€” PrioritÃ©s claires, algorithme dÃ©fini
5. **L'adaptation est automatique** â€” Les rÃ¨gles s'adaptent au niveau de sÃ©curitÃ©
6. **La traÃ§abilitÃ© est complÃ¨te** â€” Toute rÃ¨gle et Ã©valuation est traÃ§able

### Phrase de synthÃ¨se

> **Une rÃ¨gle de franchissement est une condition dÃ©clarative, dÃ©finie exclusivement par Border Guard et appliquÃ©e par BondingBrother, qui spÃ©cifie ce qui est requis pour traverser une frontiÃ¨re, selon une prioritÃ© et avec une action dÃ©finie en cas d'Ã©chec.**

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Contrat â€” Normatif  
**RÃ©fÃ©rence :** Border Guard v1.5, Documentation Fondatrice Section 4 et 5  
**Type :** Contrat de rÃ¨gles de franchissement

