# Miyukini — COG Environments Behavior and Schema

## Context

This document describes the **behavior of COG environments** (Core-Orchestrated Governance) in the Miyukini ecosystem: the roles of the different environment types, how they interact, and the mechanisms for governance, discovery, and communication. It is based on the architectural schema of COGs (environments A, B, Official, Tracker) to formalize their use.

## Scope

- Behavior of an **autonomous COG** (governance, BondingBrother, Central, Portal, Services).
- Behavior of **COG Official** and **COG Tracker** environments (registry, versioning, discovery).
- **Inter-COG communication** (network discovery, multiplayer, Passport/Visa).
- Clear differentiation from a simple web front-end application.

**Audience:** Architects, developers, and any decision-making about integration or service exposure.

---

## 1. Reference schema: COG environments

The schema below illustrates the four environment types and their relationships. Each frame represents a **sovereign unit** or a **COG management system**.

![Architectural schema of COG environments — COG A, COG B, COG Official, COG Tracker](../assets/Miyukini-Schema-Comportement-Environnements-COG.png)

### Schema legend

| Colour / zone | Meaning |
|---------------|---------|
| **Red frame** | Boundary of a COG environment |
| **Light blue** | Orchestration components (BondingBrother, Central, Portal) |
| **Green** | Services (JayFestival, JayXpose, MiyukiniPoS, others) |
| **Purple** | User applications (Passport/visit, Visa, Miyukini Survivor, Miyukini Clicker, JayKonta, etc.) |
| **Grey** | Database (state, configuration) |
| **Arrows** | Information or call flows |

---

## 2. Behavior of an autonomous COG environment (e.g. COG A)

An “autonomous” COG contains the full chain: governance, orchestration, services, and applications.

### 2.1 COG governance (decision core)

| Behavior | Description |
|----------|-------------|
| **Authority** | Decision-making and administrative core of the environment. |
| **Persistence** | Interacts with a local database (state, configuration, policies). |
| **Identity and access** | Initiates **Passport/visit** and **Visa** (authentication and authorization for service access). |
| **Coordination** | Bidirectional communication with **BondingBrother** to coordinate intentions and decisions. |
| **Announcement** | Presents itself to the network and announces its capabilities (e.g. “waiting for COG B for Miyukini Survivor 2”) via the DB and the **COG Official** environment. |

Governance **does not execute**; it decides and delegates via BondingBrother and the Cores.

### 2.2 BondingBrother (internal orchestrator)

| Behavior | Description |
|----------|-------------|
| **Role** | Internal orchestrator or event bus; translation and delegation (stratum 5 — Interfaces & Adaptation). |
| **Inputs** | Receives **Write Intent** or **Action** (write intentions, commands, requests). |
| **Routing** | Bidirectional interactions with Services (JayFestival, JayXpose, MiyukiniPoS, etc.) and with applications (Miyukini Survivor, Miyukini Clicker). |
| **Rules** | Does not decide on behalf of the Cores; transports and routes according to governance decisions. |

### 2.3 Passport/visit and Visa flow

| Step | Actor | Behavior |
|------|-------|----------|
| 1 | Governance | Issues **Passport/visit** (identity, origin, integrity). |
| 2 | BondingBrother | Validates and applies the **Visa** for authorized access. |
| 3 | Services / Applications | Act only within the scope of the Visa (see [Inter-COG Connection](Miyukini%20-%20Inter-COG%20Connection.md)). |

### 2.4 Central and applications

| Component | Behavior |
|-----------|----------|
| **Central** | Entry point for the COG user; hosts applications (Miyukini Survivor, Miyukini Clicker, Miyukini Survivor 2 multiplayer, etc.). |
| **Applications** | Send data/actions to **JayKonta** and consume Services via BondingBrother. |
| **JayKonta** | Aggregates or processes data/actions and forwards them to the Portal and Miyukini Survivor 2 (multiplayer). |

### 2.5 Portal and exposed Services

| Component | Behavior |
|-----------|----------|
| **Portal** (Miyukini Web Portal) | Encapsulates the **public façades** of Services (JayFestival, JayXpose, MiyukiniPoS, others); entry point for external users or for network discovery. |
| **Network presence** | The Portal **presents itself to the network** and **searches** for other COGs (e.g. COG A) via **INTERNET**, according to COG Official / Tracker mechanisms. |

Services remain governed: exposure ≠ transfer of governance.

---

## 3. Behavior of a secondary COG environment (e.g. COG B)

A COG can be simplified and dedicated to a specific use (e.g. multiplayer).

| Behavior | Description |
|----------|-------------|
| **Content** | May host an instance of a shared application (e.g. **Miyukini Survivor 2 (multiplayer)**). |
| **Connection** | Receives the connection or data from the peer instance in another COG (e.g. COG A). |
| **Discovery** | **Presents itself to the network** and **searches** for the other COG (e.g. COG A) via INTERNET, according to policies and the registry (COG Official / Tracker). |

Each COG remains **sovereign**; the multiplayer session is governed by both environments and inter-COG rules (Visa, host policy).

---

## 4. Behavior of the COG Official environment

The **COG Official** is the central authority for registry, control, and distribution.

| Behavior | Description |
|----------|-------------|
| **Uniqueness of environments** | Ensures each COG is uniquely identifiable. |
| **Versioning** | Controls versions of Cores and COGs on the networks (compatibility, compliance). |
| **Distribution** | Manages **official distribution** of COGs or their components. |
| **Official tracker** | Exposes an **open IP** as a contact point for discovery. |
| **List and policies** | Holds a **copy of the list of connected COGs** and their **policies** (Public list / Friends only / Closed). |
| **Inputs** | Receives information from COG A’s DB (presence, expectations, announcements). |
| **Outputs** | Communicates with the **COG Tracker** environment for dissemination. |

It **does not govern** COG content; it governs **registration, discovery, and openness policy** of environments.

---

## 5. Behavior of the COG Tracker environment

| Behavior | Description |
|----------|-------------|
| **List and policies** | Holds a **copy of the list of connected COGs** and their **policies** (public / friends / closed). |
| **Role** | Dissemination and monitoring of information about connected COGs. |
| **Network** | Interacts with **INTERNET** to disseminate or retrieve information about COGs. |
| **Link** | Fed or synchronized with **COG Official**. |

The Tracker enables **discovery** and **location** of COGs without holding definition authority (reserved for COG Official).

---

## 6. Summary: why this is not “a React/Vite app without COG”

The schema and behaviors above show that COGs **cannot be reduced** to a web front-end application (e.g. React/TypeScript Vite). Summary of differences:

| Aspect | Classic web app (e.g. Vite) | Miyukini COG environments |
|--------|----------------------------|---------------------------|
| **Unit** | Light client + centralized backend | **COG = autonomous unit** with governance, DB, services, applications. |
| **Discovery** | Fixed URL / API | **Presentation to the network**, announcement, search for other COGs via COG Official / Tracker. |
| **Inter-COG** | Often absent | **Inter-COG communication** (multiplayer, visit, Visa, Passport). |
| **Orchestration** | Single backend | **BondingBrother** + Cores: routing, Write Intent, validation, no direct execution by governance. |
| **Policies** | Ad hoc management | **Explicit policies** (public list / friends / closed), versioning, official distribution. |
| **Sovereignty** | Host owns the data | **Local sovereign state**; each COG keeps its governance (see [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md)). |

A **Vite/React app can be the surface** (Portal, UI) of a COG; it does not replace the chain Governance → BondingBrother → Cores → Services nor the COG Official / Tracker environments.

---

## 7. Cross-references

- [Inter-COG Connection](Miyukini%20-%20Inter-COG%20Connection.md) — Passport, Visa, governed visit, external users.
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md) — Sovereign, versioned, isolated entity.
- [COG Definition](Miyukini%20-%20Definition%20COG.md) — Definition of COG and the pyramid.
- [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md) — Strata and position of Cores, BondingBrother, Operators.

---

*Public document — Last updated: 2026-02-08*
