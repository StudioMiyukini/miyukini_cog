# Miyukini Core System — Laws of Autonomy

## 1. Context

This document defines the **autonomy constraint** as a first-rank architectural constraint of the Miyukini Core System. This constraint states that the ecosystem must operate correctly on simple hardware, sometimes isolated, with no critical external dependency.

This constraint is not an implementation detail. It is a **non-negotiable architectural invariant** that influences all design decisions, from the ecosystem down to Operators.

Autonomy is what distinguishes Miyukini from modern "cloud-native" architectures that assume permanent connectivity. Miyukini takes the opposite stance: **disconnection is a normal state of the system**, not an error to fix.

## 2. Scope

This document defines:
- The 8 non-negotiable laws of autonomy
- The operational definition of autonomy in Miyukini
- Implications for each core of the ecosystem
- Recognized system isolation states
- The distinction between what is synchronizable and what never is
- Bonding Brother’s role as federator

This document **does not cover**:
- Implementation details of synchronization mechanisms (see KindMother - Sync & Conflict Resolution Contract)
- Inter-instance communication protocols (see BondingBrother - Bilateral Flow Contract)
- Conflict resolution strategies (see KindMother - Failure & Degradation Contract)

---

## 3. Operational definition of autonomy

### What "autonomous" means in Miyukini

A Miyukini autonomous system is one that:

| Characteristic | Description |
|----------------|-------------|
| **Starts without network** | The system starts and reaches an operational state with no external connection |
| **Works without cloud** | All essential business operations are executable locally |
| **Degrades cleanly in isolation** | Isolation triggers an explicit degraded mode, not a cascade of errors |
| **Predictable without sync** | System behaviour is deterministic even without synchronized data |
| **Administerable locally** | Administration and diagnostics do not require an external connection |
| **Evolves when the network returns** | Reconnection triggers reconciliation, not reconstruction |

### What "autonomous" does NOT mean

Miyukini autonomy is not:

- **"Offline-first" marketing**: Miyukini does not hide a network dependency behind a local cache. Autonomy is structural, not cosmetic.
- **Disguised cache**: Local data is not a cache of remote data. It is a local source of truth with later reconciliation.
- **Magic replication**: Synchronization is explicit, controlled, and observable. No silent automatic sync.
- **Mandatory distributed real time**: Miyukini does not assume real-time consistency across nodes. Each node is consistent with itself.

---

## 4. The 8 laws of autonomy (non-negotiable)

These 8 laws are **architectural invariants**. Every design, implementation, or evolution decision must respect them. Violating one of these laws is an architectural violation.

---

### LOI-1: No critical external dependency at runtime

**Statement:** A Miyukini Operator must be able to start, decide, operate, and be audited without any mandatory external call.

**Implications:**
- External APIs are **optional**, not fundamental
- Remote services are **enhancements**, never foundations
- Cores (StrongFather, KindMother, Caring Nanny) are **self-sufficient**
- Absence of connection never blocks system startup

**Verification:** For each component, ask: *"Does this component work if the network is unavailable?"* If the answer is no, LOI-1 is violated.

**Relation to cores:**
- **StrongFather**: Policies are local. No evaluation requires an external call.
- **KindMother**: Local persistence is always available (Daughter DB in offline-first mode).
- **Caring Nanny**: State observation works locally.

---

### LOI-2: The system treats isolation as a normal state

**Statement:** Isolation is not an error. It is a valid and explicitly recognized state of the system.

**Implications:**
- No "network unreachable" exception cascading up
- No infinite blocking retry
- No decision block waiting for synchronization
- The system makes decisions **with what it has**, not with what it might have

**Verification:** For each operation, ask: *"Does this operation block waiting for an external resource?"* If yes, LOI-2 is violated.

**Relation to cores:**
- **StrongFather**: Produces decisions with available local context. Never refuses a decision on the grounds of missing external context.
- **KindMother**: WriteIntents are accepted locally and synchronized later. No blocking waiting for remote validation.
- **Caring Nanny**: Recognizes and reports the "isolated" state as normal, not as an anomaly.

---

### LOI-3: Local state is sovereign

**Statement:** When a node is isolated, its state is the local truth, its decisions are valid locally, its logs are auditable locally.

**Implications:**
- An isolated node does not doubt its own consistency
- Decisions made in isolation are not invalidated afterwards
- Local logs form a complete audit trail
- On reconnection: reconciliation, comparison, explanation — never "silent correction"

**Verification:** For each local datum, ask: *"Is this datum considered valid until explicitly reconciled?"* If no, LOI-3 is violated.

**Relation to cores:**
- **KindMother**: Daughter DB holds local authority. Reconciliation with Mother DB is explicit and traceable (see KindMother - Sync & Conflict Resolution Contract).
- **StrongFather**: Locally taken decisions are recorded and never erased, even if a different decision would have been made with more context.
- **Caring Nanny**: Records local history completely and autonomously.

---

### LOI-4: No global time required

**Statement:** An autonomous system does not depend on a network clock, global order, or timestamps synchronized across nodes.

**Implications:**
- Time is local, relative, and contextual
- Temporal comparisons between nodes are avoided or explicitly framed
- Sync conflicts are not resolved by implicit "latest wins"
- Logical or vector clocks are preferred over absolute timestamps for ordering

**Verification:** For each temporal mechanism, ask: *"Does this mechanism work if node clocks differ by minutes/hours?"* If no, LOI-4 is violated.

**Relation to cores:**
- **StrongFather**: Already designed without technical temporal logic (see INV-SF-4 in Foundational Documentation).
- **KindMother**: Synchronization uses deltas and sync points, not absolute timestamps.
- **Caring Nanny**: Observations are locally timestamped. Inter-node comparison is explicitly framed.

---

### LOI-5: Cost must be proportional to hardware

**Statement:** The system must run on simple hardware: mini PC, NAS, Raspberry Pi, isolated VM, field server.

**Implications:**
- Bounded and predictable memory
- Predictable CPU, no unpredictable spikes
- No phantom services consuming resources in the background
- No useless workers or expensive dormant processes
- Functional purity (no side effects, no hidden state) supports predictability

**Verification:** For each component, ask: *"Does this component run acceptably on a Raspberry Pi 4 with 4 GB RAM?"* If no, LOI-5 is violated.

**Relation to cores:**
- **StrongFather**: Pure decision engine, no persistent state, no permanent worker.
- **KindMother**: Internal SQLite, optimized for limited resources.
- **Caring Nanny**: Passive observer, minimal consumption.

---

### LOI-6: Autonomy does not prevent federation

**Statement:** Autonomous does not mean solitary. The system must be able to run alone, then connect to others, without changing its nature.

**Implications:**
- Federation is explicit (conscious decision to connect)
- Federation is controlled (rules on what is shared)
- Federation is observable (traceability of exchanges)
- Federation is reversible (ability to disconnect)
- A federated node remains autonomous in the sense of LOI-1 to LOI-5

**Verification:** For each federation mechanism, ask:
- *"Can the node choose not to federate?"*
- *"Can the node leave federation without losing local functionality?"*
- *"Are federated exchanges traceable and explicit?"*

If any answer is no, LOI-6 is violated.

**Relation to cores:**
- **Bonding Brother**: Becomes the **synchronization bridge** and federation mediator. Single controlled entry/exit point for inter-node exchanges.
- **Border Guard**: Controls what enters and leaves the system. Nothing implicit.

---

### LOI-7: The Cores stratum is immutable — evolution by environment

**Statement:** In Miyukini, the Cores stratum (Stratum 4) is never patched individually. Any evolution is done by creating a new complete environment. Operators are bound to a single environment and cannot exist outside it.

**Implications:**
- No micro-patch on the Cores stratum
- Complete environment versions only
- No wild hotfix in production
- A COG (Core-Orchestrated Governance Environment) is a sovereign entity bound to an environment

**Verification:** For any evolution, ask:
- *"Are we modifying an existing core in place or deploying a new environment?"*
- *"Can an Operator exist in more than one environment at once?"*

If the evolution patches a core in place or an Operator is shared across environments, LOI-7 is violated.

**Relation to cores:**
- **All cores (Stratum 4)**: Evolve by environment version, never by isolated patch.
- **See:** [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md), [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md).

---

### LOI-8: Migration = diplomacy between environments

**Statement:** Migration of an Operator or data between environments is an explicit, negotiated, and traceable operation — never automatic or implicit.

**Implications:**
- Migration is a conscious decision, not a side effect
- Migration protocols are documented and governed
- Each environment remains sovereign; exchange is diplomatic
- No "magic sync" that moves Operators between environments without agreement

**Verification:** For any mechanism moving between environments, ask:
- *"Is migration explicitly triggered and authorized?"*
- *"Do both environments (source and target) participate in the decision?"*

If migration is implicit, automatic, or not traceable, LOI-8 is violated.

**Relation to cores:**
- **Bonding Brother**: May facilitate exchanges between environments, but under explicit rules (LOI-6, LOI-8).
- **See:** [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md).

---

## 5. Recognized system isolation states

The system explicitly recognizes the following isolation states. Caring Nanny is responsible for observing and classifying these states.

### 5.1 State: Connected (healthy)

**Description:** The system is connected to the Mother DB and/or other federated nodes. All functionality is available.

**Characteristics:**
- Active synchronization
- Real-time or near-real-time reconciliation
- Globally consistent state

### 5.2 State: Isolated (offline)

**Description:** The system runs with no external connection. This is a normal state, not an error.

**Characteristics:**
- All local operations work
- WriteIntents are stored for later synchronization
- Local state is sovereign
- No aggressive automatic reconnection attempts

### 5.3 State: Partially synchronized (syncing)

**Description:** The system is reconciling with one or more nodes. Some operations may be deferred.

**Characteristics:**
- Deltas being processed
- Potential conflicts awaiting resolution
- Transient state towards "connected" or "isolated"

### 5.4 State: Degraded (degraded)

**Description:** The system runs but some functionality is unavailable or limited.

**Characteristics:**
- Essential functionality available
- Non-essential functionality disabled or limited
- Cause identified and communicated

### 5.5 State: Federated (federated)

**Description:** The system is connected to other nodes in a federation. It retains its autonomy while participating in a larger network.

**Characteristics:**
- Local autonomy preserved (LOI-1 to LOI-5)
- Active federated exchanges via Bonding Brother
- Explicit and controlled sharing rules

---

## 6. What is synchronizable vs what never is

### 6.1 Synchronizable (via KindMother + Bonding Brother)

| Category | Description | Mechanism |
|----------|-------------|-----------|
| **Business data** | Contents, entities, relations | KindMother Deltas |
| **Produced decisions** | Results of StrongFather evaluations | Exportable journals |
| **Observed states** | Caring Nanny state history | Exportable journals |
| **Sync metadata** | Sync points, checksums | KindMother internal |

### 6.2 Never synchronizable

| Category | Reason | Consequence |
|----------|--------|-------------|
| **Active StrongFather policies** | Local sovereignty | Each node defines its own policies |
| **Local permission rules** | Local sovereignty | Permissions evaluated locally only |
| **Real-time system state** | Volatile and local | Caring Nanny observes locally |
| **Sessions and auth tokens** | Security and volatility | Managed locally by the Operator |
| **Cache and transient data** | Ephemeral nature | Not persisted, not synchronized |

### 6.3 Conditionally synchronizable

| Category | Condition | Decided by |
|----------|-----------|------------|
| **Policies (templates)** | If federation allows policy sharing | Bonding Brother + Border Guard |
| **Data schemas** | If versions compatible | Ever Buddy compatibility check |
| **Declarative business rules** | If explicitly shared | Operator configuration |

---

## 7. Bonding Brother as federator

### 7.1 Role in autonomy

Bonding Brother is the **single control point** for all inter-node communication. In an autonomy context:

- **Without federation:** Bonding Brother only handles local mediation between products and authorities (KindMother, StrongFather).
- **With federation:** Bonding Brother becomes the synchronization bridge to federated nodes.

### 7.2 Federation guarantees

Bonding Brother ensures federation respects the laws of autonomy:

| Guarantee | Description |
|-----------|-------------|
| **Non-mandatory** | A node may refuse any federation (LOI-1) |
| **Non-blocking** | Federation never blocks local operations (LOI-2) |
| **Traceable** | All federated exchanges are journaled (LOI-3) |
| **Independent of global time** | Exchanges use logical clocks (LOI-4) |
| **Lightweight** | Exchanges are optimized (deltas, compression) (LOI-5) |
| **Reversible** | A node may leave federation at any time (LOI-6) |

### 7.3 Federation flow

```
NODE A (autonomous)                    NODE B (autonomous)
      │                                    │
      ▼                                    ▼
[Bonding Brother A]  ◄──────────────►  [Bonding Brother B]
      │                                    │
      │  Explicit federated exchange       │
      │  - Filtered by Border Guard        │
      │  - Translated by Bonding Brother     │
      │  - Journaled locally                │
      │                                    │
      ▼                                    ▼
[KindMother A]                       [KindMother B]
(sovereign over its data)            (sovereign over its data)
```

---

## 8. Impact on ecosystem cores

### 8.1 StrongFather

**Current status:** Already compatible with the laws of autonomy.

**Compliance points:**
- ✅ Local decisions (INV-SF-1, INV-SF-2)
- ✅ No execution (pure engine)
- ✅ No network dependency
- ✅ No technical temporal logic (INV-SF-4)

**No change required.**

### 8.2 KindMother

**Current status:** Compatible if offline-first principles are respected.

**Compliance points:**
- ✅ Local storage first (Daughter DB)
- ✅ Optional synchronization
- ✅ Explicit journals

**Points to watch:**
- Mother/Daughter sync must remain explicit and non-blocking
- Conflict resolution must not assume global time

### 8.3 Bonding Brother

**Current status:** Becomes strategic for federation.

**New role:**
- Inter-node synchronization bridge
- Control of federated exchanges
- Never a vital dependency (system works without federation)

### 8.4 Caring Nanny

**Current status:** Compatible with required extension.

**Compliance points:**
- ✅ Local observation
- ✅ Local history

**Required extension:**
- Must recognize and report isolation states (Section 5)
- Must distinguish "isolated" from "error"

### 8.5 Border Guard

**Current status:** Becomes critical for autonomy.

**Reinforced role:**
- Controls everything entering and leaving the system
- Nothing implicit in external communications
- Explicit validation of federated exchanges

### 8.6 Master Butler, Ever Buddy, TAMR

**Status:** To be assessed in their foundational documentation.

**Principle:** These cores must respect the laws of autonomy from design.

---

## 9. Architectural validation question

From this document, **every architecture decision must answer this question**:

> *"Does it still work if the system is alone, slow, and isolated?"*

- If the answer is **yes** → the decision is compliant.
- If the answer is **no** → the decision must be reconsidered or explicitly justified as an exception.
- If the answer is **"it depends"** → the decision requires clarification of conditions.

---

## 10. Strategic benefits of autonomy

Autonomy positions Miyukini for contexts that "cloud-native" architectures cannot serve:

| Context | Miyukini advantage |
|---------|---------------------|
| **Industry** | Operation in a factory without reliable internet |
| **Field** | Mobile applications in coverage gaps |
| **Associations** | Organizations with no cloud budget or limited connectivity |
| **Small structures** | Deployment on minimal existing hardware |
| **Regulatory constraints** | Local data, no mandatory cloud |
| **Long term** | System still works if the cloud provider disappears |

This positioning is **deliberate and strategic**. Miyukini does not aim for:
- "Scalable cloud-native"
- "Microservices everywhere"
- "Distributed real time"

Miyukini aims for:
> **Resilient, explainable, local-first, federable.**

---

## 11. Contractual status

This document is **contractual, normative, and REFERENCE status**. It establishes first-rank architectural constraints that apply to all components of the Miyukini ecosystem.

The 8 laws of autonomy are **non-negotiable**. Any exception must be:
1. Explicitly documented
2. Justified by an insurmountable technical constraint
3. Approved as an exception, not as a rule

---

**Version:** 1.1  
**Date:** 2026-01-26  
**Status:** REFERENCE — Normative architectural constraint  
**Dependencies:**
- KindMother - Foundational Documentation v1.1
- StrongFather - Foundational Documentation v1.1
- BondingBrother - Foundational Documentation v1.1
- Caring Nanny - Foundational Documentation v1.2
- Border Guard - Foundational Documentation v1.2
- TAMR - Foundational Documentation v1.1
- Ever Buddy - Foundational Documentation v1.1
- Master Butler - Foundational Documentation v1.1

**Related reference documents:**
- [Project Objective](Miyukini%20-%20Project%20Objective.md): Synthesis vision and pillar 6 "Operational autonomy"
- [Miyukini Conceptual References - Integrity & Degradation System](Miyukini%20Framework%20-%20Integrity%20Degradation%20System.md): Graduated degradation compliant with laws of autonomy
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](Miyukini%20Framework%20-%20External%20Signal%20Trust%20Reinforcement%20Contract.md): Internet as signal, not dependency (LOI-1)
- [Miyukini Conceptual References - Carte Optimisation](Miyukini%20Conceptual%20References%20-%20Carte%20Optimisation.md): Optimization levers by zone without violating invariants
- [Kernel Maintenance Observability Contract](Miyukini%20-%20Kernel%20Maintenance%20Observability%20Contract.md): Low-level maintenance capabilities (observation without correction)
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md): Sovereignty rules, versioning and migration (LOI-7, LOI-8)
