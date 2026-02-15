# Miyukini Conceptual References — Complete Architecture Pyramid

## 1. Context

This document presents the **Miyukini Pyramid**: the complete architecture of the Miyukini Core System ecosystem, from the physical stratum (hardware) up to end-user usage and practices.

This pyramid defines the **7 architectural strata** plus the **Kernel** that form the technical and conceptual foundation of the ecosystem. Each stratum has distinct responsibilities, specific invariants, and clearly defined relations with adjacent strata.

**Strategic vision:** This pyramid enables control from hardware to UX, delivery of any layer alone or combined, serving B2B / B2C / B2B2C, and operation offline/isolated/low-resource, while remaining modular, scalable, and autonomous. The key lies in **Stratum 6 — Tools & Toolkits**: ready-to-use, recomposable capabilities, independent of business context.

**Fundamental principle:** Dependency is strictly one-way, top to bottom. Each stratum depends only on lower strata, never the reverse.

## 2. Scope

This document defines:
- The complete structure of the Miyukini Pyramid (7 strata + Kernel)
- The responsibilities of each stratum
- Relations between strata
- Architectural invariants at each level
- The Kernel’s role as a neutral technical substrate

This document **does not cover**:
- Implementation details of each stratum (see cores’ foundational documentation)
- Inter-stratum communication protocols (see specific contracts)
- Evolution and compatibility rules (see EverBuddy - Foundational Documentation)

---

## 3. ASCII diagram of the Pyramid

```
┌──────────────────────────────────────────────┐
│ 🔧 STRATUM 9 — MiyukiniAdmin (EXCEPTION)     │
│ Sovereign administration console             │
│ → Out-of-band, like BIOS/hypervisor          │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟩 STRATUM 7 — OPERATORS                      │
│ Service · Interface · Automation · Domain     │
│ B2C · B2B · B2B2C                             │
│ → Governed functional entities                │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟦 STRATUM 6 — TOOLS & TOOLKITS               │
│ Auth · Billing · Content · Realtime · Admin   │
│ Monitoring · Workflow · Notification         │
│ → Governed capabilities & compositions       │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟨 STRATUM 5 — INTERFACES & ADAPTATION        │
│ UI · API · CLI · WebSocket · Edge             │
│ Bonding Brother                               │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟥 STRATUM 4 — SYSTEM CORES                   │
│ StrongFather · KindMother · Caring Nanny      │
│ Master Butler · Border Guard · Ever Buddy     │
│ TAMR                                         │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟪 STRATUM 3 — INVARIANTS & CONTRACTS         │
│ Decision ≠ Execution · Zero-trust             │
│ Determinism · Auditability · Autonomy        │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ ⚙️ KERNEL — TECHNICAL SUBSTRATE               │
│ Id · Logger · Clock (trace) · Minimal IO      │
│ Portable · Local · Offline                   │
└──────────────────────────────────────────────┘
                    ▲
┌──────────────────────────────────────────────┐
│ 🟫 STRATUM 0 — HARDWARE & OS                  │
│ CPU · RAM · Disk · Network · Failures         │
└──────────────────────────────────────────────┘
```

**Important note:** MiyukiniAdmin (Stratum 9) is a **deliberate exception** to standard Operator logic. It sits above the pyramid, not inside it. It observes, installs, arbitrates, but does not live in the normal flow. See [Miyukini Conceptual References - MiyukiniAdmin Status](Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md) for details.

---

## 4. Detailed description of strata

### 🟫 STRATUM 0 — PHYSICAL REALITY

**Role:** Material foundation and physical constraints of the system.

**Constituent elements:**
- **Hardware:** CPU, RAM, disk, network
- **OS:** Operating system (Linux, Windows, macOS, etc.)
- **Physical constraints:** Network latency, hardware failures, geographic isolation
- **Limited resources:** Memory, CPU, bandwidth, storage

**Invariants:**
- Failures are normal, not exceptions
- Network isolation is a valid state
- Resources are limited and unpredictable
- Physical time is not synchronized across nodes

**Relation to upper strata:**
- All upper strata must accept these constraints
- No stratum may assume permanent connectivity
- No stratum may assume unlimited resources

**Related documentation:**
- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md) — LOI-1, LOI-2, LOI-5

---

### ⚙️ KERNEL — NEUTRAL TECHNICAL SUBSTRATE

**Role:** Reusable, agnostic technical foundation, no business logic.

**Constituent elements:**
- **Id:** Generation and management of unique identifiers
- **Logger:** Structured, traceable logging
- **Clock:** Local clock (trace only, no synchronization)
- **Config:** Local configuration
- **Lifecycle:** Lifecycle management (boot, shutdown)

**Invariants:**
- No business logic
- No critical external dependency
- Local safe primitives only
- No application protocol (HTTP, WebSocket, etc.)
- No ORM, no data access layer

**Relation to strata:**
- Used by all upper strata
- Depends only on Stratum 0
- Provides technical primitives, never application logic

**Related documentation:**
- [Miyukini Core System - Definition Kernel](../kernel/Miyukini%20Core%20System%20-%20Definition%20Kernel.md)
- [Miyukini Core System - Kernel Structure](../kernel/Miyukini%20Core%20System%20-%20Structure%20du%20Kernel.md)

---

### 🟪 STRATUM 3 — INVARIANTS & CONTRACTS

**Role:** Fundamental architectural principles and non-negotiable invariants.

**Constituent elements:**
- **Decision ≠ Execution separation:** StrongFather decides, KindMother executes
- **Functional purity:** No hidden side effects, determinism
- **Zero-trust:** No implicit trust, everything is verified
- **Auditability:** Every action is traceable and verifiable
- **Autonomy:** Operation without critical external dependency
- **Determinism:** Predictable behaviour even in isolation

**Invariants:**
- These principles are non-negotiable
- Any violation is an architectural violation
- They apply to all upper strata

**Relation to strata:**
- These invariants govern all upper strata
- No stratum may violate these principles
- They are the conceptual base of the architecture

**Related documentation:**
- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md)
- [StrongFather - Foundational Documentation](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md) — INV-SF-1, INV-SF-2
- [KindMother - Foundational Documentation](../core/KindMother/KindMother%20-%20Documentation%20Fondatrice.md)

---

### 🟥 STRATUM 4 — SYSTEM CORES

**Role:** Conceptual engines that govern system behaviour.

**Constituent elements:**

#### StrongFather
- **Role:** Strategic and policy decision engine
- **Question:** "Should this action be done?"
- **Authority:** Decision only, never execution

#### KindMother
- **Role:** Absolute authority over data and persistence
- **Question:** "How is data persisted and synchronized?"
- **Authority:** Persistence, synchronization, consistency

#### Caring Nanny
- **Role:** System state observer
- **Question:** "What state is the system in?"
- **Authority:** Observation only, no modification

#### Master Butler
- **Role:** Registry of capabilities and permissions
- **Question:** "What can be done, and who has the right to do it?"
- **Authority:** Knowledge of possibilities, never decision

#### Border Guard
- **Role:** Definition of boundaries and trust levels
- **Question:** "Where are the system boundaries, and what rules govern crossing them?"
- **Authority:** Conceptual definition only, not application

#### Ever Buddy
- **Role:** Lifecycle and evolution governance
- **Question:** "How does the system evolve without breaking?"
- **Authority:** Evolution governance, never migration execution

#### TAMR
- **Role:** Definition of human intervention points
- **Question:** "When may humans intervene in the system?"
- **Authority:** Definition of intervention points, never decision

**Invariants:**
- Each core has exclusive authority in its domain
- No core may violate Stratum 3 invariants
- Cores collaborate but never substitute for each other
- Each core respects the [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md)

**Relation to strata:**
- Use the Kernel for technical primitives
- Respect Stratum 3 invariants
- Are used by Stratum 5 (BondingBrother) for mediation

**Related documentation:**
- [StrongFather - Foundational Documentation](../core/StrongFather/StrongFather%20-%20Documentation%20Fondatrice.md)
- [KindMother - Foundational Documentation](../core/KindMother/KindMother%20-%20Documentation%20Fondatrice.md)
- [Caring Nanny - Foundational Documentation](../core/CaringNanny/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [Master Butler - Foundational Documentation](../core/MasterButler/Master%20Butler%20-%20Documentation%20Fondatrice.md)
- [Border Guard - Foundational Documentation](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md)
- [Ever Buddy - Foundational Documentation](../core/EverBuddy/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- [TAMR - Foundational Documentation](../core/TAMR/TAMR%20-%20Documentation%20Fondatrice.md)

---

### 🟨 STRATUM 5 — INTERFACES & ADAPTATION

**Role:** User interfaces, interaction points, and mediation between products and cores.

**Constituent elements:**

#### Interfaces
- **UI:** Graphical interfaces (web, desktop, mobile)
- **CLI:** Command line for administration and operations
- **API:** Programmatic interfaces (REST, GraphQL, etc.)
- **WebSocket:** Real-time communication
- **Edge:** Network edge deployment

#### BondingBrother
- **Role:** Fraternal mediation interface between Operators and authorities
- **Function:** Translates Operator intentions into requests for cores, and translates responses into results for Operators
- **Principle:** Mediation only, never authority

#### Operator Adapters
- **Role:** Translation between SPM CMS modules and KindMother
- **Function:** Implement SPM modules’ functional traits using KindMother
- **Principle:** One adapter per Operator, complete isolation

**Invariants:**
- All interfaces use Tools & Toolkits (Stratum 6) or Operators (Stratum 7)
- No interface accesses cores (Stratum 4) directly
- BondingBrother never makes decisions
- BondingBrother never has authority
- Adapters are the only entry point to KindMother

**Relation to strata:**
- Uses Stratum 4 cores (StrongFather, KindMother, etc.)
- Uses Kernel for technical primitives
- Is used by Stratum 6 (Tools & Toolkits) and Stratum 7 (Operators)

**Related documentation:**
- [BondingBrother - Foundational Documentation](../core/BondingBrother/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Miyukini Core System - Product Adapter Conceptual Documentation](../core/Miyukini%20Core%20System%20-%20Adaptateur%20Produit%20Documentation%20Conceptuelle.md)

---

### 🟦 STRATUM 6 — TOOLS & TOOLKITS

**Role:** Governed, recomposable, business-context-independent executable capabilities. **This is the strategic key layer** that provides skills to Operators.

**Terminology note:** This stratum was formerly called "Intermediate Products". The correct terminology is **Tools & Toolkits**. See [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md).

**Constituent elements:**

#### Cross-cutting Tools & Toolkits
- **Auth / Identity:** Authentication, identity management, roles, tokens
- **Billing Core:** Billing, pricing plans, subscriptions
- **Content Engine:** Content management, pages, blocks, media
- **Realtime Engine:** WebSocket, real-time events, live state
- **Workflow Engine:** States, transitions, generic business processes
- **Notification:** Email, push, local notifications
- **Search / Index:** Fast search, indexing
- **MiyukiniAdmin:** System supervision, administration, monitoring

**Fundamental characteristics:**
- **Recomposable:** Can be combined to create Operators
- **Cross-cutting:** Usable in different contexts (B2B, B2C, B2B2C)
- **Independent of business context:** No specific business logic
- **Use cores:** Use StrongFather, KindMother, etc. via BondingBrother
- **Never decide alone:** Delegate decisions to cores
- **Ready to use:** Functional without complex business configuration

**Invariants:**
- No Tool or Toolkit contains client-specific business logic
- All use cores via BondingBrother (Stratum 5)
- All are usable everywhere (B2B, B2C, B2B2C)
- None makes strategic decisions (delegated to StrongFather)
- None manages persistence directly (delegated to KindMother)

**Relation to strata:**
- Use Stratum 5 interfaces and BondingBrother
- Use Stratum 4 cores
- Are consumed by Stratum 7 (Operators)
- Use Kernel for technical primitives

**Why this stratum is strategic:**
- **Avoids the WordPress/SaaS monolith trap:** We do not build "a CMS with plugins", we build "a system that can produce a CMS"
- **Enables selling at all levels:** B2B (building blocks), B2C (Operator), B2B2C (Operator + Tools under licence)
- **Stays compatible with weak hardware:** Pure logic, no cloud dependency, local deployment possible
- **Ideal for:** Local government, events, IoT, edge computing, isolated areas

**Related documentation:**
- [Miyukini Conceptual References - Catalogue Capacites Produit](Miyukini%20Conceptual%20References%20-%20Catalogue%20Capacites%20Produit.md)
- [Miyukini Conceptual References - Capacites Mutualisables](Miyukini%20Conceptual%20References%20-%20Capacites%20Mutualisables.md)

---

### 🔧 STRATUM 9 — MiyukiniAdmin (EXCEPTION)

**Role:** Sovereign administration console, orchestration and control tool.

**Status:** Deliberate exception to standard Operator logic.

**Constituent elements:**
- **Installation & Bootstrap:** Complete Miyukini environment installation
- **Monitoring & Metrics:** Passive reading of system metrics
- **Technical Tests:** Diagnostic environment
- **Security & Arbitration:** Controlled intervention when needed
- **Data Access:** Controlled access via KindMother (normal case)
- **Exceptional Recovery:** Direct DB write in maintenance mode (extreme case)

**Invariants:**
- ❌ No other Operator may depend on MiyukiniAdmin
- ❌ MiyukiniAdmin consumes no Tool or Toolkit
- ❌ MiyukiniAdmin exposes no public API
- ❌ MiyukiniAdmin is never embedded in a client Operator
- ✅ Always via BondingBrother
- ✅ Never silent, never implicit

**Relation to strata:**
- Above the pyramid, not inside it
- Observes, installs, arbitrates, but does not live in the normal flow
- Exclusive access to cores via BondingBrother

**Related documentation:**
- [Miyukini Conceptual References - MiyukiniAdmin Status](Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md): Official and canonical status

---

### 🟩 STRATUM 7 — OPERATORS

**Role:** Governed functional entities that perform roles on behalf of the user.

**Terminology note:** This stratum was formerly called "Finished Products". The correct terminology is **Operators**. See [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md).

**Canonical definition:**

> **An Operator is a governed functional entity that performs a role on behalf of the user within a Miyukini environment.**

**Operator types:**

| Type | Role | Examples |
|------|------|----------|
| **Service Operator** | Manages a functional domain | CMS, Auth, E-commerce, CRM |
| **Interface Operator** | Exposes services | Web UI, Mobile app, Dashboard |
| **Automation Operator** | Acts automatically | Workflows, Agents, Batch |
| **Domain Operator** | Performs a specific trade | Blog, Catalogue, Support |
| **Sovereign Operator** | System authority (exception) | MiyukiniAdmin |

**Delivery models:**
- **B2C:** Operators for end consumers
- **B2B:** Operators for businesses
- **B2B2C:** Operators + Tools under licence for resellers

**Invariants:**
- Operators orchestrate Tools & Toolkits (Stratum 6)
- Operators do not code, they orchestrate
- Use Stratum 5 interfaces
- Respect autonomy constraints (Stratum 3)
- Are governed by Cores (Stratum 4)

**Relation to strata:**
- Orchestrate Stratum 6 Tools & Toolkits
- Use Stratum 5 interfaces
- Respect Stratum 3 invariants
- Run on Kernel and Stratum 0

**Founding phrase:**

> **In Miyukini, users do not install applications. They interact with governed Operators that perform roles on their behalf.**

**Related documentation:**
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md)
- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md)

---

## 5. Pyramid principles

### 5.1 Unidirectional dependency

**Principle:** Each stratum depends only on lower strata, never the reverse.

**Implications:**
- Stratum 7 cannot depend directly on Stratum 4
- The Kernel cannot depend on an upper stratum
- Cores (Stratum 4) cannot depend on Tools or Operators (Stratum 6 or 7)

**Verification:** For each dependency, ask: *"Does this dependency go to a lower stratum?"* If not, there is an architectural violation.

### 5.2 Ecosystem governance — Vertical dependency

**Principle:** Strata 0 to 5 form a non-substitutable foundation, strictly governed by Miyukini. Strata 6 and 7 allow external extension, but within Miyukini’s strict framework.

**Founding rule (LOI-7):**

> **In Miyukini, the Cores stratum is immutable.**  
> **Any evolution is done by creating a new complete environment.**  
> **Operators are bound to a single environment and cannot exist outside it.**

**Full documentation:** [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md)

#### 🔻 Strata 0 → 5: Non-substitutable foundation

| Stratum | Nature | Governance |
|---------|--------|------------|
| **Stratum 0 — Hardware & OS** | Physical | Material constraint |
| **Kernel** | Technical | Miyukini only — No substitution |
| **Stratum 3 — Invariants & Contracts** | Conceptual | Miyukini only — No substitution |
| **Stratum 4 — System Cores** | Conceptual | Miyukini only — No substitution |
| **Stratum 5 — Interfaces & Adaptation** | Technical | Miyukini only — No substitution |

**Absolute rule:** No external implementation may replace or bypass these strata.

#### 🔺 Strata 6 → 7: Authorized extension, imposed framework

**Permissions for third-party developers:**
- ✅ Create Tools and Toolkits (Stratum 6)
- ✅ Create Operators (Stratum 7)
- ✅ Create both

**Mandatory constraints:**
- ❌ Respect Miyukini protocols
- ❌ Use official interfaces (Stratum 5)
- ❌ Accept voluntary limitations
- ❌ Comply with system contracts
- ❌ No direct access to cores (Stratum 4)
- ❌ No reverse dependency

**Fundamental principle:** Third-party developers do not code "on top of" Miyukini, they code "inside" Miyukini.

**Related documentation:**
- [Miyukini Conceptual References - Ecosystem Dependency Contract](Miyukini%20Conceptual%20References%20-%20Ecosystem%20Dependency%20Contract.md): Formal dependency contract
- [Miyukini Conceptual References - Vision Stratégique](Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md): Section 8 — Ecosystem Governance Principle

### 5.3 Responsibility isolation

**Principle:** Each stratum has exclusive responsibilities and may not encroach on another stratum’s responsibilities.

**Examples:**
- Stratum 4 (Cores) may not contain business logic (Stratum 6 or 7)
- Stratum 6 (Intermediate Products) may not manage persistence directly (Stratum 4 - KindMother)
- Stratum 5 (Interfaces) may not make strategic decisions (Stratum 4 - StrongFather)

### 5.4 Autonomy at each level

**Principle:** Each stratum must be able to operate autonomously, without critical external dependency.

**Implications:**
- Kernel runs without network
- Cores run with local data
- Modules run with local adapters
- Interfaces run with local modules

**Related documentation:**
- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md) — LOI-1, LOI-2, LOI-3

### 5.5 Multi-environment coexistence

**Principle:** Multiple COG environments may coexist on the same physical hardware, without conflict.

**Architecture diagram:**

```
Physical Hardware
 │
 ├─ Miyukini Env A (COG vers. 1.2 LTS)
 │   ├─ Operators A1, A2
 │   └─ [ID: env-a-uuid]
 │
 ├─ Miyukini Env B (COG vers. 2.0)
 │   ├─ Products B1
 │   └─ [ID: env-b-uuid]
 │
 └─ Miyukini Env C (isolated / offline)
     ├─ Products C1
     └─ [ID: env-c-uuid]
```

**Why there is no conflict:**
- No shared patch between environments
- No shared core between environments
- No cross dependency
- Complete isolation (each environment has its own cores)

**Related documentation:**
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md) — Section 4

---

### 5.6 Traceability and auditability

**Principle:** Every action across strata must be traceable and auditable.

**Implications:**
- Kernel provides logging (Logger)
- Cores journal all their operations
- BondingBrother traces all mediations
- Interfaces trace user interactions

**Related documentation:**
- [BondingBrother - Audit & Traceability Contract](../core/BondingBrother/BondingBrother%20-%20Audit%20%26%20Traceability%20Contract.md)
- [StrongFather - Audit & Trace Contract](../core/StrongFather/StrongFather%20-%20Audit%20%26%20Trace%20Contract.md)

---

## 6. Typical flows through the Pyramid

### 6.1 User → Action flow

```
Stratum 7 (Operator - User)
    ↓
Stratum 6 (Tools & Toolkits - Auth, Content, etc.)
    ↓
Stratum 5 (API/UI interface + BondingBrother)
    ↓
Stratum 4 (StrongFather → KindMother)
    ↓
Stratum 3 (Invariants respected)
    ↓
Kernel (Logger, Clock, Id)
    ↓
Stratum 0 (Physical persistence)
```

### 6.2 State observation flow

```
Stratum 4 (Caring Nanny observes)
    ↓
Stratum 4 (KindMother, StrongFather, etc.)
    ↓
Stratum 5 (BondingBrother propagates)
    ↓
Stratum 6 (Intermediate Products inform)
    ↓
Stratum 7 (Finished Products display)
```

### 6.3 Strategic decision flow

```
Stratum 6 (Tool expresses intention)
    ↓
Stratum 5 (BondingBrother translates)
    ↓
Stratum 4 (Master Butler: capabilities?)
    ↓
Stratum 4 (StrongFather: decision?)
    ↓
Stratum 4 (KindMother: execution?)
    ↓
Kernel (Logger, Clock for traceability)
```

---

## 7. Evolution and compatibility

### 7.1 Stratum evolution

**Principle:** Each stratum may evolve independently, subject to respecting contracts with adjacent strata.

**Governance:**
- Evolution is governed by **Ever Buddy** (Stratum 4)
- Compatibility rules are defined by **Border Guard** (Stratum 4)
- Evolution decisions are made by **StrongFather** (Stratum 4)

**Related documentation:**
- [Ever Buddy - Foundational Documentation](../core/EverBuddy/Ever%20Buddy%20-%20Documentation%20Fondatrice.md)
- [Border Guard - Foundational Documentation](../core/BorderGuard/Border%20Guard%20-%20Documentation%20Fondatrice.md)

### 7.2 Backward compatibility

**Principle:** Evolutions of a stratum must not break upper strata that use it.

**Guarantees:**
- Kernel maintains stability of public contracts
- Cores maintain API compatibility
- Modules maintain trait compatibility

---

## 8. Conclusion

The Miyukini Pyramid defines a strict layered architecture, where each stratum has exclusive responsibilities and clearly defined relations. This structure guarantees:

- **Autonomy:** Each stratum operates independently
- **Consistency:** Invariants are respected at all levels
- **Evolvability:** Each stratum may evolve without breaking others
- **Traceability:** Every action is observable and auditable
- **Security:** Boundaries are clearly defined and protected

This pyramid is the architectural reference for all development in the Miyukini ecosystem.

---

**Related documentation:**
- [Miyukini Conceptual References - Vision Stratégique](Miyukini%20Conceptual%20References%20-%20Vision%20Strategique.md): Strategic objectives and ecosystem vision
- [COG Definition](Miyukini%20-%20Definition%20COG.md): Official COG definition (Core-Orchestrated Governance Environment)
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md): Sovereignty rules, versioning and migration (LOI-7, LOI-8)
- [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md): Governance of executable capabilities (Stratum 6)
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md): Official Operator terminology (Stratum 7)
- [Miyukini Conceptual References - MiyukiniAdmin Status](Miyukini%20Conceptual%20References%20-%20MiyukiniAdmin%20Status.md): Official and canonical status (Stratum 9 - Sovereign Operator)
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Conceptual%20References%20-%20Integrity%20Degradation%20System.md): Graduated degradation system (T0-T4)
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Conceptual%20References%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md): Internet integration as external signal
- [Miyukini Conceptual References - Mobile & WebApp Strategy](Miyukini%20Conceptual%20References%20-%20Mobile%20WebApp%20Strategy.md): Mobile and WebApp architecture (Stratum 5 - Interfaces)
- [Miyukini Conceptual References - Security Protocols](Miyukini%20Conceptual%20References%20-%20Security%20Protocols.md): Security protocols (real-time and asynchronous)
- [Miyukini Conceptual References - Security Performance Impact](Miyukini%20Conceptual%20References%20-%20Security%20Performance%20Impact.md): Actual performance impact
- [Miyukini Conceptual References - Security Levels](Miyukini%20Conceptual%20References%20-%20Security%20Levels.md): Security levels (0-4) — core adaptation by level
- [Miyukini Conceptual References - Carte Optimisation](Miyukini%20Conceptual%20References%20-%20Carte%20Optimisation.md): Optimization levers allowed by zone
- [Project Objective](Miyukini%20-%20Project%20Objective.md): Synthesis vision and fundamental pillars

---

**Date of creation:** 2026-01-26  
**Version:** 2.7 (complete Operator terminology)  
**Status:** Contractual reference document
