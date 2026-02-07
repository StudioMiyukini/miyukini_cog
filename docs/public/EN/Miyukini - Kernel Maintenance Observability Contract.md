# Miyukini Conceptual References — Kernel Maintenance Observability Contract

## Context

This document defines the **low-level Kernel capabilities** for assisting code maintenance without ever performing automatic correction. It establishes the fundamental boundary between observation/attestation and intervention.

This boundary is what distinguishes Miyukini from opaque self-healing systems (Kubernetes, SaaS platforms, extensible CMS). **Miyukini does not maintain the code in place of the human. It makes the code maintainable without ambiguity.**

These capabilities are comparable to those found in critical systems (critical OS, military systems, industrial systems).

## Scope

- **Applies to:** Kernel, Cores, low-level infrastructure
- **Audience:** Architects, developers, auditors
- **Status:** Normative reference document

---

## 1. Intangible principle

### What the low level MAY say

| Capability | Description |
|------------|-------------|
| **What is** | Current system state, loaded structure |
| **What changed** | Deltas, changes, evolutions |
| **What violates** | Invariant violations, unmet contracts |
| **What is fragile** | At-risk zones, tight coupling, instability |

### What the low level may NEVER say

| Forbidden | Reason |
|-----------|--------|
| **What to fix** | Governed human decision |
| **How to fix** | Governed human decision |
| **When to fix** | Governed human decision |

> **Every action remains governed + human.**

---

## 2. Capability summary table

| Capability | Allowed | Forbidden |
|------------|:-------:|:---------:|
| Observe | ✅ | |
| Attest | ✅ | |
| Compare | ✅ | |
| Signal | ✅ | |
| Explain | ✅ | |
| Correct | | ❌ |
| Mutate | | ❌ |
| Self-repair | | ❌ |

> **This is exactly the boundary of a maintainable sovereign system.**

---

## 3. Low-level controls for code maintenance

### 3.1 Stable behavioural fingerprint (Behavior Fingerprint)

The Kernel may produce a **behavioural fingerprint** of the loaded system:

| Captured element | Description |
|------------------|-------------|
| **Load order** | Component initialization sequence |
| **Structural call graph** | Relations between components (not business) |
| **Invoked contracts** | List of active contracts |
| **Solicited invariants** | Invariants checked at load |

> **It is a signature, not a log.**

#### Use

- Compare two versions of the system
- Detect silent drift
- Prove that a build is functionally "equivalent"

#### Guarantees

- No business content
- No runtime data
- Deterministic and replayable

---

### 3.2 Silent divergence detector

The Kernel may signal a situation where:

| Condition | Implication |
|-----------|-------------|
| Same declared version | Identical version hash |
| Different behavioural fingerprint | Distinct structural behaviour |

#### Typical detectable cases

- Build recompiled differently
- Dependency modified silently
- Non-reproducible build
- Code injection or post-build modification

> **Extremely valuable for serious maintenance and security audit.**

---

### 3.3 Structural complexity heat map

Without analysing business logic, the Kernel may expose:

| Metric | Description |
|--------|-------------|
| **Graph depth** | Number of dependency levels |
| **Dependency density** | Connections/components ratio |
| **High-coupling zones** | Heavily interconnected components |
| **Low-stability zones** | Frequently changed or fragile components |

#### Use

- Anticipate technical debt
- Identify priority intervention zones
- Plan refactorings
- **Without ever touching the code**

---

### 3.4 Local freeze point per component

Instead of freezing the whole environment, the low level may:

| Action | Description |
|--------|-------------|
| **Mark a component as frozen** | Structurally frozen, not functionally |
| **Refuse its replacement** | Block reload or modification |
| **Let the rest evolve** | Freeze limited to target component |

#### Use

- Stabilize a critical zone during intervention
- Fix elsewhere without regression risk
- Maintain strong SLAs on specific components

#### Governance

| Actor | Role |
|-------|------|
| **StrongFather** | Decides freeze authorization |
| **EverBuddy** | Validates freeze compatibility |
| **Kernel** | Executes and applies the freeze |

---

### 3.5 Contractual ambiguity detection

The Kernel may signal:

| Signal | Description |
|--------|-------------|
| **Invoked but incomplete contract** | Partially defined or used contract |
| **Never-active invariant** | Declared but never checked invariant |
| **Never-encountered rule** | Policy never evaluated at runtime |

> **Not an error. A maintenance signal.**

#### Use

- Simplify a system by removing dead code
- Detect never-executed code
- Prepare simplification or refactoring
- Identify contracts to consolidate

---

### 3.6 "Explainable maintenance" mode

When an incident occurs, the low level may provide:

| Provided information | Description |
|---------------------|-------------|
| **Why a decision reached this point** | Governance path traversed |
| **Which contracts were traversed** | List of evaluated contracts |
| **Where governance stopped** | Blocking point or final decision |

#### What is NEVER provided

| Exclusion | Reason |
|-----------|--------|
| Classic stacktrace | Technical information leak |
| Memory dump | Sensitive data leak |
| User data | Privacy protection |

> **It is governed traceability, not technical.**

---

## 4. Compatibility with an isolated COG

All these controls:

| Characteristic | Status |
|----------------|--------|
| Work offline | ✅ |
| Require no SaaS | ✅ |
| Require no external agent | ✅ |
| Are deterministic | ✅ |
| Are replayable | ✅ |

### Practical consequences

| Context | Compatible |
|---------|:----------:|
| Weak hardware (Raspberry Pi, mini PC) | ✅ |
| Isolated environment (air-gapped) | ✅ |
| Long version cycle (LTS) | ✅ |
| Post-mortem audit | ✅ |

---

## 5. Strategic positioning

### What Miyukini does differently

| System | Approach | Miyukini |
|--------|----------|----------|
| **Kubernetes** | Opaque self-repair | Observation + human signal |
| **SaaS platforms** | Automatic correction | Attestation without mutation |
| **Extensible CMS** | Silent hotfix | Explicit governed freeze |
| **Self-repairing systems** | Magic box | Full transparency |

### Key formulation

> **Miyukini does not maintain the code in place of the human.**  
> **It makes the code maintainable without ambiguity.**

---

## 6. Contract invariants

### INV-MOC-1: Non-mutation

> The Kernel never modifies code, configuration, or data to "fix" a situation.

### INV-MOC-2: Determinism

> Every observation or attestation produces the same result for the same input state.

### INV-MOC-3: Explainability

> Every piece of information provided is understandable by a human without knowledge of the source code.

### INV-MOC-4: Local sovereignty

> Controls work without external dependency (network, SaaS, agent).

### INV-MOC-5: Preserved governance

> No observation capability bypasses the governance chain (StrongFather, EverBuddy).

---

## 7. Capability recap

| # | Capability | Main use |
|---|------------|----------|
| 1 | Behavioural fingerprint | Comparison, equivalence |
| 2 | Divergence detector | Audit, security |
| 3 | Complexity map | Planning, technical debt |
| 4 | Local freeze | Stabilization, SLA |
| 5 | Ambiguity detection | Simplification, dead code |
| 6 | Explainable maintenance | Diagnostic, traceability |

---

## 8. Contractual status

This document is **contractual, normative, and REFERENCE status**. It establishes the Kernel’s capabilities and strict limits for assisting maintenance without ever performing automatic correction.

The INV-MOC-* invariants are **non-negotiable**. Every implementation must respect them.

---

**Version:** 1.0  
**Date:** 2026-01-27  
**Status:** REFERENCE — Low-level capability contract  

**Cross-references:**

- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md): Autonomy constraints (LOI-1 to LOI-6)
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md): Sovereign, isolated model
- [Miyukini Conceptual References - Carte Optimisation](./Miyukini%20Conceptual%20References%20-%20Carte%20Optimisation.md): Optimization levers by zone
- [Miyukini Conceptual References - Integrity Degradation System](./Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md): Graduated degradation
- [Miyukini Core System - Definition Kernel](../kernel/Miyukini%20Core%20System%20-%20Definition%20Kernel.md): Technical kernel definition
- [StrongFather - Foundational Documentation](../core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md): Decision governance
