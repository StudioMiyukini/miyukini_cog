# Miyukini Conceptual References — Operators and Terminology

## Context

This document defines the **canonical terminology** of Miyukini regarding what the user "uses". It replaces the incorrect term "product" with the correct terminology **Operator**.

This document formalizes why the word "product" is conceptually wrong in Miyukini and establishes the correct terminology to use in all documentation.

## Scope

- **Applies to:** All documentation, communication, architecture
- **Audience:** Architects, developers, marketing, AI
- **Status:** Normative reference document — OFFICIAL TERMINOLOGY

---

## 1. Why "Product" is an Incorrect Term

### What the word "product" implies

| Implication | Problem for Miyukini |
|-------------|----------------------|
| Finished object | ❌ An Operator evolves with the environment |
| Delivered as-is | ❌ An Operator is governed dynamically |
| Passively consumed | ❌ The user interacts, mandates, delegates |
| Autonomous | ❌ An Operator depends on the environment |
| Merchandise | ❌ An Operator is a functional actor |

### What the user actually "uses"

In Miyukini, what the user uses is:

| Property | Description |
|----------|-------------|
| ❌ **Not autonomous** | Depends on the COG environment |
| ❌ **Not sovereign** | Subject to governance rules |
| ❌ **Not free** | Constrained by the Cores |
| ❌ **Not a simple app** | Governed actor |
| ❌ **Not a bundle of features** | Structured functional entity |

### What it really is

**👉 It is a specialized actor that:**

- Operates within an institutional framework
- Acts on behalf of the user
- Applies rules
- Orchestrates capabilities
- Delivers a structured service

---

## 2. The Right Analogy: The Environment as a Country

If the COG environment is a **country**:

| Analogy | Miyukini equivalent |
|---------|----------------------|
| Institutions | Cores (StrongFather, KindMother, etc.) |
| Skills / know-how | Tools |
| Trades | Toolkits |
| Citizen / client | User |
| **Specialized professional** | **Operator** |

**👉 The user does not launch an app.**  
**👉 They call upon an Operator.**

---

## 3. Canonical Definition: Operator

### Official definition

> **An Operator is a governed functional entity that performs a role on behalf of the user within a Miyukini environment.**

### French translation

> **Un Opérateur est une entité fonctionnelle gouvernée qui exécute un rôle pour le compte de l'utilisateur au sein d'un environnement Miyukini.**

### Operator characteristics

| Property | Description |
|----------|-------------|
| **Active** | It acts (not passive) |
| **Professional** | Like a specialized human operator |
| **Non-sovereign** | Subject to environment governance |
| **Governed** | Constrained by the Cores |
| **B2B / B2C / B2B2C compatible** | Usable in all models |

### Why "Operator" is the right term

| Criterion | Assessment |
|-----------|------------|
| Active | ✅ It acts |
| Professional | ✅ Like a human operator |
| Non-sovereign | ✅ It has no authority of its own |
| Business compatible | ✅ B2B / B2C / B2B2C |
| Already used in serious contexts | ✅ Telco, infra, ops |

---

## 4. Operator typology

### 4.1 Service Operator

**Role:** Manages a functional domain for the user.

| Examples | Description |
|----------|-------------|
| CMS | Content management |
| Auth | Authentication and identity |
| E-commerce | Electronic commerce |
| CRM | Customer relationship management |
| Monitoring | Surveillance and metrics |
| Search | Search and indexing |
| Billing | Billing and payment |

**Typical phrase:** *"Manages this domain for me."*

---

### 4.2 Interface Operator

**Role:** Exposes services in a usable way.

| Examples | Description |
|----------|-------------|
| Web UI | Web interface |
| Mobile app | Mobile application |
| Dashboard | Dashboard |
| Admin panel | Administration panel |

**Typical phrase:** *"Exposes services in a usable way."*

---

### 4.3 Automation Operator

**Role:** Acts automatically within a defined framework.

| Examples | Description |
|----------|-------------|
| Workflows | Automated processes |
| Agents | Autonomous agents |
| Batch | Batch processing |
| Rules | Automatic rules |

**Typical phrase:** *"Acts automatically within this framework."*

---

### 4.4 Domain Operator

**Role:** Performs a specific trade.

| Examples | Description |
|----------|-------------|
| Blog | Article publishing |
| Catalogue | Catalogue management |
| Support | Customer support |
| Knowledge base | Knowledge base |
| Forum | Community discussion |

**Typical phrase:** *"Performs this specific trade."*

---

### 4.5 Sovereign Operator — EXCEPTION

**Role:** Quasi-institutional authority.

| Example | Description |
|---------|-------------|
| **MiyukiniAdmin** | Sovereign administration console |

**Special characteristics:**

| Property | Description |
|----------|-------------|
| **Not a normal citizen** | Exception status |
| **Acts under special protocol** | Particular rules |
| **Has quasi-institutional authority** | May arbitrate |
| **Not usable by other Operators** | Strict isolation |

**Typical phrase:** *"Administers the environment itself."*

---

## 5. What the User Becomes

### The user does not "consume a product"

The user:

| Action | Description |
|--------|-------------|
| **Mandates** | Delegates a task to an Operator |
| **Consults** | Queries an Operator |
| **Interacts** | Exchanges with an Operator |
| **Configures** | Configures an Operator |
| **Delegates** | Entrusts responsibility to an Operator |

**👉 They call upon Operators according to their needs.**

---

## 6. Official reformulations

### Correspondence table

| ❌ Old incorrect term | ✅ New correct term |
|------------------------|---------------------|
| Create a product | **Deploy an Operator** |
| Use an app | **Interact with an Operator** |
| Product marketplace | **Operator registry** |
| Finished product | **Operator** |
| Intermediate product | **Tool or Toolkit** |
| Launch an app | **Call upon an Operator** |
| App/Site | **Interface Operator** |
| Business service | **Service Operator** |

---

## 7. Relation to other concepts

### Conceptual hierarchy

```
Tools = skills
    ↓
Toolkits = trades
    ↓
Operators = equipped professionals
```

### What an Operator does

| Action | Yes/No |
|--------|--------|
| Code | ❌ No |
| Implement | ❌ No |
| **Orchestrate** | ✅ Yes |
| **Delegate to Tools** | ✅ Yes |
| **Apply governance** | ✅ Yes |
| **Collaborate under mandate** | ✅ Yes (via Operator Team) |

### Collaboration between Operators

An Operator never works alone in a complex way. For complex Services:

| Mechanism | Description |
|-----------|-------------|
| **Operator Team** | Governed collective to deliver a Service |
| **Team Contract** | Static collaboration rules |
| **Permission Mandate** | Dynamic collaboration authorization |

**Foundational rule:**

> **In Miyukini, complexity is handled by collaboration, not accumulation.**

**Full documentation:** [Mandates and Operator Teams](Miyukini%20-%20Mandates%20and%20Operator%20Teams.md)

### Updated architecture

```
Kernel
└── Core Governance Layer
    ├── StrongFather
    ├── KindMother
    ├── MasterButler
    ├── WorrySentinel
    ├── EverBuddy
    ├── CaringNanny
    └── BorderGuard
        ↓
    BondingBrother
        ↓
    Tools & Toolkits
        ↓
    Operators
        ↓
    Interfaces
        ↓
    Users
```

---

## 8. Founding phrase (to be engraved)

### English

> **In Miyukini, users do not install applications.**  
> **They interact with governed Operators that perform roles on their behalf.**

### French

> **Dans Miyukini, les utilisateurs n'installent pas d'applications.**  
> **Ils interagissent avec des Opérateurs gouvernés qui exécutent des rôles pour leur compte.**

---

## 9. Impact on existing documentation

### Documents to update (terminology)

| Document | Change |
|----------|--------|
| **Complete Architecture Pyramid** | Replace "Products" with "Operators" |
| **Strategic Vision** | Terminology update |
| **Project Objective** | Terminology update |
| **All Core contracts** | Reference Operators |

### Stratum correspondence

| Old terminology | New terminology |
|-----------------|-----------------|
| Stratum 6 — Intermediate Products | Stratum 6 — **Tools & Toolkits** |
| Stratum 7 — Finished Products | Stratum 7 — **Operators** |

---

## 10. Summary of fundamental rules

| # | Rule | Status |
|---|------|--------|
| 1 | "Product" is an incorrect term | **TERMINOLOGY** |
| 2 | An Operator is a governed functional entity | **DEFINITION** |
| 3 | The user mandates/interacts with Operators | **USAGE** |
| 4 | Operators orchestrate, they do not code | **INVARIANT** |
| 5 | MiyukiniAdmin is a Sovereign Operator (exception) | **EXCEPTION** |
| 6 | Complexity = collaboration, not accumulation | **PRINCIPLE** |
| 7 | Mandated collaboration only | **NON-NEGOTIABLE** |

---

**Date of creation:** 2026-01-27  
**Version:** 1.2 (Operator terminology)  
**Status:** Normative reference document — OFFICIAL TERMINOLOGY

**Cross-references:**
- [Glossary](Miyukini%20-%20Glossary.md): Official dictionary
- [COG Definition](Miyukini%20-%20Definition%20COG.md): Official COG definition
- [Mandates and Operator Teams](Miyukini%20-%20Mandates%20and%20Operator%20Teams.md): **Mandates and Teams**
- [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md): Tool governance
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md): The environment as a country
- [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md): Stratified architecture
- [Miyukini Conceptual References - MiyukiniAdmin Status](./Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md): Sovereign Operator
