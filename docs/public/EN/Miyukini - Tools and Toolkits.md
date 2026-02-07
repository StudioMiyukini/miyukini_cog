# Miyukini Conceptual References — Tools and Toolkits

## Context

This document defines the **canonical concepts of Tool and Toolkit** in the Miyukini ecosystem. It establishes governance rules, the responsibilities of the Cores involved, and the associated architectural constraints.

**Doctrine phrase:**

> **Tools are governed executable capabilities. Toolkits are official compositions of tools, optimized for efficiency but never for authority.**

> **Les Outils sont des capacités exécutables gouvernées. Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

## Scope

- **Applies to:** System architecture, Operator development, capability governance
- **Audience:** Architects, developers, environment operators
- **Status:** Normative reference document

---

## 1. Canonical definition: Tool

### Canonical statement

> A **Tool** is an **executable capability**, with no authority, no business decision, no knowledge of the calling Operator, governed by the Cores.

### Tool characteristics

| Property | Description |
|----------|-------------|
| **Executable capability** | Does something concrete and atomic |
| **No authority** | Never decides whether the action should be done |
| **No business decision** | Contains no business logic |
| **No context knowledge** | Ignores which Operator calls it and why |
| **Governed by Cores** | Subject to Cores’ authorization and control |

### Fundamental rule

> **👉 A Tool does, but never decides.**

### What a Tool is NOT

| ❌ Is not | Why |
|-----------|-----|
| A business service | No business logic |
| A decision maker | No authority |
| A core | No governance of its own |
| An Operator | No context knowledge |

### Tool examples

| Domain | Tool | Action |
|--------|------|--------|
| UI | `layout.render` | Renders a layout |
| UI | `input.capture` | Captures user input |
| UI | `form.validate` | Validates a form |
| UI | `theme.resolve` | Resolves a theme |
| UI | `event.dispatch` | Dispatches an event |
| Data | `query.execute` | Executes a query |
| Data | `cache.get` | Gets from cache |
| IO | `file.read` | Reads a file |
| IO | `file.write` | Writes a file |

---

## 2. Canonical definition: Toolkit

### Canonical statement

> A **Toolkit** is an **official composition of Tools**, validated and declared by the environment, optimized for efficiency, consistency and performance.

### Toolkit characteristics

| Property | Description |
|----------|-------------|
| **Official composition** | Formal aggregation of existing Tools |
| **Validated by environment** | Declared and governed |
| **Optimized** | For efficiency, consistency, performance |
| **No new capability** | Adds no functionality that Tools do not have |
| **No business logic** | Pure orchestration, no decision |

### Fundamental rule

> **👉 A Toolkit adds no new capability; it properly orchestrates existing Tools.**

### What a Toolkit is NOT

| ❌ Is not | Why |
|-----------|-----|
| A new Tool | It does not create new capability |
| A service | It has no logic of its own |
| A decision maker | It has no authority |
| A free library | It is governed |

### Example: UI Toolkit

```
UI Toolkit
 ├─ layout.render
 ├─ input.capture
 ├─ form.validate
 ├─ theme.resolve
 └─ event.dispatch
```

**What the UI Toolkit does:**
- Groups these Tools
- Optimizes calls
- Normalizes flows

**What the UI Toolkit does NOT do:**
- Decide when to display a UI
- Choose which theme to apply
- Authorize or refuse an action

---

## 3. Mental structure: Tool vs Toolkit

```
Tool
 └─ exposes atomic capabilities

Toolkit
 └─ aggregates Tools
 └─ no business logic
 └─ no decision
```

### Call flow

An Operator may:
- Call an **isolated Tool** for an atomic capability
- Call a **Toolkit** for greater efficiency

**But in both cases:**
- It goes through the **same governance**
- It is subject to the **same rules**

**Terminology note:** The term "product" is incorrect. The correct terminology is **Operator**. See [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md).

### Flow diagram

```
Operator (Stratum 7)
    │
    ▼
┌───────────────────────────────────────┐
│  BondingBrother (mediation)           │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Master Butler: "Does this Tool exist  │
│  and does this Operator have the      │
│  right?"                              │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  WorrySentinel: "Does the security     │
│  level allow this call?"              │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Caring Nanny: "Does system state      │
│  allow this call?"                    │
└───────────────────────────────────────┘
    │
    ▼
┌───────────────────────────────────────┐
│  Tool / Toolkit (execution)           │
└───────────────────────────────────────┘
```

---

## 4. Governance of Tools and Toolkits

### ABSOLUTE rule

> **A Miyukini environment has a finite, declared, governed tool library.**

| Rule | Description |
|------|-------------|
| **No wild injection** | No Tool may be added dynamically without governance |
| **No "local" Tool** | Every Tool must be declared in the environment |
| **No hidden external dependency** | No undeclared external library |

**👉 This is application sovereignty.**

### What the Cores do NOT do

| Core | What it does NOT do |
|------|---------------------|
| **Master Butler** | Does not implement Tools |
| **Master Butler** | Does not describe their logic |
| **Master Butler** | Does not manage their technical lifecycle |

---

## 5. Core responsibilities

### 5.1 Master Butler — Capability & Permission Core

**Central role:** Catalogue of capabilities and permissions.

| Responsibility | Description |
|----------------|-------------|
| **Declare** | Which capabilities exist in the environment |
| **Link** | Capability → Tool |
| **Authorize** | Who may call what |
| **Define** | Access permissions |

**Question Master Butler answers:**

> *"What is possible in this environment?"*

**What Master Butler knows:**
- List of available Tools
- List of declared Toolkits
- Capability → Tool mapping
- Permissions per Operator/role

**What Master Butler does NOT do:**
- Implement Tools
- Execute Tools
- Decide whether a Tool should be called

---

### 5.2 Ever Buddy — Lifecycle & Evolution Core

**Role:** Tool lifecycle and evolution.

| Responsibility | Description |
|----------------|-------------|
| **Versions** | Manages Tool versions |
| **Deprecation** | Marks obsolete Tools |
| **Compatibility** | Verifies Tool ↔ Environment |
| **Migration** | Manages Tool → new version transition |

**Question Ever Buddy answers:**

> *"Does this tool still exist, is it compatible, or must it be migrated?"*

**What Ever Buddy knows:**
- Current version of each Tool
- Deprecated versions
- Migration paths
- Compatibility with COG environment

---

### 5.3 Caring Nanny — Product State Core

**Role:** Global environment consistency.

| Responsibility | Description |
|----------------|-------------|
| **Allowed states** | Defines when a Tool may be used |
| **Conditional blocking** | Blocks if environment is degraded |
| **Observation** | Monitors system state |

**Question Caring Nanny answers:**

> *"Does current system state allow this call?"*

**Example of blocking:**

```
UI Toolkit unavailable because environment in SECURITY_LOCKDOWN state
```

**What Caring Nanny knows:**
- Current environment state
- States that block certain Tools
- Degradation rules

---

### 5.4 WorrySentinel — Security Governance Core

**Role:** Tool security governance.

| Responsibility | Description |
|----------------|-------------|
| **Security level** | Defines level required for each Tool |
| **Degradation** | Manages security degradation |
| **Blocking** | Blocks Tools in case of threat |
| **Audit** | Traces calls for audit |
| **Conditional authorization** | Authorizes under conditions |

**Question WorrySentinel answers:**

> *"Does current security level allow this call?"*

**What WorrySentinel knows:**
- Security level required per Tool
- Current environment security level
- Conditional authorization rules

---

## 6. Toolkit definition

### Who defines Toolkits?

**👉 Not a single Core.**

Toolkits are:

| Step | Responsible Core |
|------|-------------------|
| **Declared** | Master Butler |
| **Composed** | Documentation + Manifest |
| **Validated** | WorrySentinel |
| **Compatibility** | Ever Buddy |

### Toolkit structure

A Toolkit is defined by:

| Element | Description |
|---------|-------------|
| **Identifier** | Unique Toolkit name |
| **Tool list** | Tools composing the Toolkit |
| **Version** | Toolkit version |
| **Security level** | Level required to use the Toolkit |
| **Allowed states** | System states in which the Toolkit works |

### Example Toolkit manifest

```yaml
toolkit:
  id: "ui.standard"
  version: "1.0.0"
  description: "Standard UI Toolkit"
  tools:
    - layout.render
    - input.capture
    - form.validate
    - theme.resolve
    - event.dispatch
  security_level: 2
  allowed_states:
    - HEALTHY
    - DEGRADED
  disallowed_states:
    - SECURITY_LOCKDOWN
    - MAINTENANCE
```

---

## 7. Usage rules for Operators

### What an Operator may do

| Action | Authorized |
|--------|------------|
| Call an isolated Tool | ✅ Yes (if authorized) |
| Call a Toolkit | ✅ Yes (if authorized) |
| Create a local Tool | ❌ No |
| Modify a Tool | ❌ No |
| Bypass governance | ❌ No |

### Call flow from an Operator

```
Operator: "I want a UI"
    │
    ▼
Environment: "Here are the authorized Tools, in this framework"
    │
    ▼
Tool / Toolkit: Execution
```

### What is forbidden

| ❌ Forbidden | Why |
|--------------|-----|
| Tool injection | No undeclared Tool |
| Local Tool | Everything must be in the environment |
| Hidden dependency | No ungoverned external library |
| Direct call | Always via BondingBrother |

---

## 8. Summary: Responsibility table

| Element | Role |
|---------|------|
| **Tool** | Atomic executable capability |
| **Toolkit** | Official composition of Tools |
| **Master Butler** | Catalogue of capabilities and permissions |
| **Ever Buddy** | Lifecycle and versions |
| **Caring Nanny** | System state consistency |
| **WorrySentinel** | Security and audit |
| **Operator** | Tool user (via governance) |

---

## 9. Fundamental rules (to be engraved)

| # | Rule | Status |
|---|------|--------|
| 1 | A Tool does, but never decides | **NON-NEGOTIABLE** |
| 2 | A Toolkit orchestrates, but adds no capability | **NON-NEGOTIABLE** |
| 3 | The tool library is finite and governed | **NON-NEGOTIABLE** |
| 4 | No wild Tool injection | **NON-NEGOTIABLE** |
| 5 | Every call goes through governance | **NON-NEGOTIABLE** |

### Official formulation

> **Tools are governed executable capabilities.**  
> **Toolkits are official compositions of tools, optimized for efficiency but never for authority.**

> **Les Outils sont des capacités exécutables gouvernées.**  
> **Les Kits d'Outils sont des compositions officielles d'outils, optimisées pour l'efficience mais jamais pour l'autorité.**

---

**Date of creation:** 2026-01-27  
**Version:** 1.3 (French terminology Tool, Toolkit, Operator)  
**Status:** Normative reference document

**Cross-references:**
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md): Official Operator terminology
- [Master Butler - Foundational Documentation](../core/MasterButler/Master%20Butler%20-%20Documentation%20Fondatrice.md): Capability catalogue
- [Ever Buddy - Foundational Documentation](../core/EverBuddy/Ever%20Buddy%20-%20Documentation%20Fondatrice.md): Lifecycle
- [Caring Nanny - Foundational Documentation](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md): State consistency
- [WorrySentinel - Foundational Documentation](../core/WorrySentinel/WorrySentinel%20-%20Documentation%20Fondatrice.md): Security
- [BondingBrother - Foundational Documentation](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md): Mediation
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md): Application sovereignty
- [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md): Stratified architecture
