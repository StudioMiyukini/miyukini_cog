# Miyukini Conceptual References — Mandates and Operator Teams

## Context

This document defines the **mechanisms for collaboration between Operators** in the Miyukini ecosystem. It introduces the concepts of **Permission Mandate (Allow Mandate)**, **Operator Team**, and clarifies the fundamental distinction between **Service** and **Operator**.

**Founding phrase:**

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

## Scope

- **Applies to:** Operator architecture, collaboration governance, system performance
- **Audience:** Architects, developers, environment operators
- **Status:** Normative reference document

---

## 1. Problem: The Monolithic Operator

### Why an "oversized" Operator is a dead end

An Operator built like "WordPress + 50 plugins" creates four major problems:

| Problem | Impact |
|---------|--------|
| **Huge attack surface** | Multiplied vulnerabilities |
| **Forced uniform security** | Everything at highest level = overhead |
| **Functional coupling** | Everything depends on everything |
| **Slow evolution** | Touching one brick = risking the whole |

**👉 Exactly the flaws Miyukini aims to eliminate.**

### Absolute rule

> **❌ An Operator must NEVER become a "super-Operator".**

---

## 2. Fundamental distinction: Service ≠ Operator

### Canonical definitions

| Concept | Definition |
|---------|------------|
| **Service** | Capability perceived by the user |
| **Operator** | Governed execution unit |

### Fundamental rule

> **A Service may be delivered by one Operator... or by an Operator Team.**

### Implications

- The user sees **Services**
- The system executes via **Operators**
- Complexity is handled by **collaboration**, not accumulation

---

## 3. Operator Team

### Canonical definition

> **An Operator Team is a governed collective of Operators that collaborate under explicit rules to deliver a Service.**

**In French:**

> **Une Équipe d'Opérateurs est un collectif gouverné d'Opérateurs qui collaborent sous règles explicites pour délivrer un Service.**

### What an Operator Team is NOT

| ❌ Is not | Why |
|-----------|-----|
| A new Operator | Not an execution entity |
| A product | Not a deliverable unit |
| A free hierarchy | Explicit rules required |

**👉 It is a higher-level orchestration structure.**

### Team composition

A team contains:

| Element | Description |
|---------|-------------|
| **Multiple Operators** | Minimum 2 |
| **Heterogeneity** | Different security, responsibilities, exposure |
| **Team Contract** | Collaboration rules |
| **StrongFather validation** | Rules approved |

### Communication rule

> **📌 No Operator talks freely to another.**

All communication between Operators:
- Goes through BondingBrother
- Is defined in the Team Contract
- Is authorized by a Permission Mandate

---

## 4. Team Contract

### Definition

The **Team Contract** defines the possible collaboration rules between Operators in the same team.

### Contract content

| Element | Description |
|---------|-------------|
| **Member Operators** | List of Operators in the team |
| **Authorized flows** | Who may talk to whom |
| **Flow direction** | Direction of communication |
| **Exchange types** | Nature of interactions |
| **Data types** | Exchangeable data |
| **Conditions** | Prerequisites for exchanges |
| **Validation level** | Governance requirements |

### Characteristics

| Property | Value |
|----------|-------|
| **Nature** | Static |
| **Definition** | At design time |
| **Validation** | By StrongFather |
| **Modification** | Formal process |

### Key rule

> **👉 The contract is validated ONCE, not on every call.**

---

## 5. Permission Mandate (Allow Mandate)

### Canonical definition (EN)

> **An Allow Mandate is a bounded authorization issued by StrongFather that allows a defined set of Operators to collaborate under explicit conditions without requiring repeated governance checks.**

### Canonical definition (FR)

> **Un Mandat de Permission est une autorisation déléguée, temporaire et encadrée, émise par StrongFather, qui permet à des Opérateurs de collaborer sans repasser en permanence par la gouvernance centrale.**

### Why "Mandate" is the right term

| Characteristic | ✅ Present |
|----------------|-----------|
| Delegated authority (not freedom) | ✅ |
| Explicit framework | ✅ |
| Revocable | ✅ |
| Temporary or conditional | ✅ |
| Institutional (legal/political) | ✅ |
| Not technical, not low-level | ✅ |

### Founding rule

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Permission Mandate content

| Element | Description |
|---------|-------------|
| **Unique ID** | Mandate identifier |
| **Authorized Operators** | List of mandated Operators |
| **Authorized flows** | Who talks to whom |
| **Data types** | Exchangeable data |
| **Maximum security level** | Security ceiling |
| **Validity conditions** | When the mandate is valid |
| **Revocation rules** | When the mandate expires |

### What a Permission Mandate is NOT

| ❌ Is not | Why |
|-----------|-----|
| A free token | Strict framework |
| A classic session | Not authentication |
| A decision cache | Not a technical optimization |
| An implicit right | Always explicit |
| A global permission | Always bounded |

---

## 6. Usage cycle with Permission Mandate

### Phase 1: Service initialization

```
User
    │
    ▼
Service request
    │
    ▼
┌───────────────────────────────────────┐
│  StrongFather:                        │
│  - Identifies Operators               │
│  - Verifies security levels           │
│  - Verifies team consistency          │
│  - Verifies WorrySentinel rules       │
└───────────────────────────────────────┘
    │
    ▼
📜 Permission Mandate issued
```

### Phase 2: Operational phase (⚡ high performance)

While the Mandate is valid:

```
Operator Team (active)
    │
    ▼
┌───────────────────────────────────────┐
│  Communication via BondingBrother     │
│  - Without calling StrongFather       │
│  - Strict respect of mandate          │
└───────────────────────────────────────┘
    │
    ▼
Tools & Toolkits
    │
    ▼
Results
```

**👉 Predictable, high performance**  
**👉 Governance preserved**

### Phase 3: End, breach or anomaly

The mandate is **immediately revoked** if:

| Condition | Effect |
|-----------|--------|
| Service ended | Normal revocation |
| Condition out of scope | Security revocation |
| Rule violation | Immediate revocation |
| WorrySentinel alert | Emergency revocation |
| User leaves the flow | Normal revocation |
| Environment change | Context revocation |

**➡️ Mandatory return to StrongFather**

---

## 7. Team Contract / Permission Mandate relation

| Element | Nature | Role |
|---------|--------|------|
| **Team Contract** | Static | Describes possible collaboration |
| **Permission Mandate** | Dynamic | Authorizes a real instance |

### Key rule

> **An Operator Team can exist operationally only under a valid Permission Mandate.**

- The team is not "active" by default
- It is **mandated**

---

## 8. Heterogeneous security

### Fundamental principle

> **An Operator has only one security level.**  
> **A Team may combine several.**

### Concrete example

| Operator | Role | Security |
|----------|------|----------|
| UI Operator | Display | 🟢 Low (1) |
| Content Operator | CMS | 🟡 Medium (2) |
| Auth Operator | Identity | 🔴 High (3) |
| Audit Operator | Logs | 🔴 High (3) |

### Result

- Fast UI
- Flexible CMS
- Ultra-secure Auth
- **Segmented risk**

### Absolute security rules

| Rule | Status |
|------|--------|
| An Operator may never raise its level | **NON-NEGOTIABLE** |
| A flow may never go down in security | **NON-NEGOTIABLE** |
| Bridges between levels are explicit | **NON-NEGOTIABLE** |
| Bridges between levels are rare | **NON-NEGOTIABLE** |
| Bridges between levels are auditable | **NON-NEGOTIABLE** |
| Bridges are validated by WorrySentinel | **NON-NEGOTIABLE** |

---

## 9. Complete mental diagram

```
┌─────────────────────────────────────────────────────────┐
│                    USER                                  │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              SERVICE REQUEST                            │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   STRONGFATHER                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │ • Operator identification                       │   │
│  │ • Security verification                        │   │
│  │ • Team Contract verification                   │   │
│  │ • WorrySentinel consultation                   │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│           📜 PERMISSION MANDATE ISSUED                   │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│              OPERATOR TEAM (active)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │ UI Op.   │  │ CMS Op.  │  │ Auth Op. │             │
│  │   🟢     │  │   🟡     │  │   🔴     │             │
│  └──────────┘  └──────────┘  └──────────┘             │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  BONDINGBROTHER                          │
│         (mediation without repeated governance)           │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                 TOOLS & TOOLKITS                         │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                     RESULTS                              │
└─────────────────────────────────────────────────────────┘
```

---

## 10. Summary of fundamental rules

| # | Rule | Status |
|---|------|--------|
| 1 | No monolithic Operator | **NON-NEGOTIABLE** |
| 2 | Specialized Operators | **REQUIRED** |
| 3 | Operator Teams for complex Services | **RECOMMENDED** |
| 4 | Services perceived by the user | **REQUIRED** |
| 5 | Permission Mandates for performance | **REQUIRED** |
| 6 | Segmented security, not global | **NON-NEGOTIABLE** |

---

## 11. Official vocabulary

### Terminological correspondence

| English | French | Definition |
|---------|--------|------------|
| Allow Mandate | Mandat de Permission | Delegated, bounded authorization |
| Operator Team | Équipe d'Opérateurs | Governed collective of Operators |
| Team Contract | Contrat d'Équipe | Static collaboration rules |
| Service | Service | Capability perceived by the user |
| Mandated Collaboration | Collaboration Mandatée | Cooperation under mandate |
| Mandated Path | Chemin Mandaté | Flow authorized by mandate |
| Operator | Opérateur | Governed functional entity |

### Obsolete terms

| ❌ Old term | ✅ New term |
|-------------|-------------|
| Decision Window | Permission Mandate |
| Temporary Decision | Mandated Authorization |
| Fast Path | Mandated Path |
| Operator Collaboration (free) | Mandated Collaboration |
| Operator | Opérateur |

---

## 12. Founding phrases

### Complexity

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

> **Dans Miyukini, la complexité est gérée par la collaboration, pas par l'accumulation.**

### Permission Mandate

> **An Allow Mandate is not an optimization. It is a delegated act of governance.**

> **Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance délégué.**

### Security

> **Segmented risk, not uniform security.**

---

**Date of creation:** 2026-01-27  
**Version:** 1.1 (Operator terminology)  
**Status:** Normative reference document

**Cross-references:**
- [Glossary](Miyukini%20-%20Glossary.md): Official dictionary
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md): Operator definition
- [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md): Capability governance
- [StrongFather - Foundational Documentation](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md): Mandate issuer
- [WorrySentinel - Foundational Documentation](../core/WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md): Security validation
- [BondingBrother - Foundational Documentation](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md): Mediation
- [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md): Stratified architecture
