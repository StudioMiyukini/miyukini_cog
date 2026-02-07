# Miyukini Conceptual References — Glossary

## Context

This document is the **official dictionary** of the Miyukini ecosystem. It brings together all canonical definitions, official terminology, and fundamental concepts.

**This glossary is the terminological source of truth.**

## Scope

- **Applies to:** All documentation, communication, development
- **Audience:** Everyone (architects, developers, marketing, AI)
- **Status:** Normative reference document — OFFICIAL GLOSSARY

---

## Component nomenclature (prefixes)

The following prefixes identify the **type of component** designed by Miyukini or belonging to the official Jay family:

| Prefix / pattern | Meaning |
|------------------|---------|
| **MiyuXxx** | Generic **Toolkit** name by Miyukini |
| **MiyukiniOpsXxx** | Generic **Operator** name by Miyukini |
| **MiyukiniXxx** | Generic **Service** name by Miyukini |
| **JayXxx** | **Jay Service** — Official services of the "Jay" family |

**Examples:** MiyuClock (Toolkit), MiyukiniOpsAdmin (Operator), MiyukiniSales (Service), JayRDV, JayFestival, JayXpose (Jay Services).

**See also:** Tool, Toolkit, Operator, Service.

---

## A

### ACTIVE — life state

State of an element in normal use. The element is stable, documented, supported, and usable by all authorized consumers. Changes are subject to compatibility rules.

**See also:** DRAFT, DEPRECATED, RETIRED, Ever Buddy

---

## B

### BondingBrother

**Mediation Core** (Stratum 5). Fraternal interface that translates Operator intentions into requests for the Cores, and translates responses into results.

**Role:** Mediation only, never authority.

**Fundamental question:** *"How do we translate this intention for the authorities?"*

**See also:** Cores, Operator

---

### Border Guard

**Boundary Core** (Stratum 4). Defines system boundaries and trust levels.

**Role:** Conceptual definition of boundaries, not direct application.

**Fundamental question:** *"Where are the system boundaries, and what rules govern crossing them?"*

**See also:** Cores, Migration

---

### Inter-COG Bridge

**Diplomatic channel** between COGs, extension of BondingBrother for inter-environment communication.

**Role:**
- Transport of identities, intents and authorizations
- **No decision-making power**
- **No business state**

**Fundamental rule:**

> **The bridge never trusts; it transports.**

**See also:** BondingBrother, Host COG, Home COG, Inter-COG Governed Visit

---

## C

### Capability

Technical power that a component has. What a module, adapter, or Operator can do technically, regardless of permissions.

**Characteristics:**
- Intrinsic to the component
- Technical (describes a functional power)
- Declarative (declared by the component)
- Identifiable (unique, stable identifier)

**See also:** Permission, Tool, Master Butler

---

### Caring Nanny

**State observation Core** (Stratum 4). System state observer that detects, classifies and propagates states.

**Role:** Observe and report system state, without ever modifying, deciding, or executing.

**Fundamental question:** *"What state is the system in at a given moment?"*

**Tool responsibility:** State consistency — blocks Tools if the environment is degraded.

**See also:** Cores, Trust states

---

### Mandated Collaboration

**Cooperation between Operators under a Permission Mandate.** Operators never collaborate freely — all collaboration is framed by a mandate issued by StrongFather.

**Rules:**
- No direct communication between Operators
- Mandatory passage through BondingBrother
- Strict respect of the Permission Mandate

**See also:** Permission Mandate, Operator Team, BondingBrother

---

### Team Contract

**Static collaboration rules** between Operators in the same Operator Team.

**Contract content:** Member Operators, authorized flows, flow direction, exchange types, exchangeable data types, preconditions, required validation level.

**Characteristics:** Static (defined at design time), validated by StrongFather, modification = formal process.

**Key rule:** The contract is validated ONCE, not on every call.

**See also:** Operator Team, Permission Mandate

---

### COG (Core-Orchestrated Governance Environment)

**Official Miyukini definition.**

> **Miyukini is a COG — an environment of governance orchestrated by cores.**

| Letter | Meaning |
|--------|---------|
| **C** | Core — Cores are the fundamental units of governance |
| **O** | Orchestrated — Active, coordinated (not "operating") |
| **G** | Governance Environment — Active governance environment |

**What COG implies:** Orchestrated > Operating (Miyukini is not an OS); Governance > Governed (active, institutional); Environment — complete ecosystem.

**See also:** Environment, Cores

---

### Host COG

**Sovereign COG that hosts a Visitor User** from another environment. Executive sovereign of the session, single source of truth for state, security and arbitration authority. Verifies visitor, grants or refuses access, frames execution, monitors session (WorrySentinel), revokes at any time.

**Fundamental rule:** *A COG never hosts a foreign governance. It only hosts visitors, under a visa, in a framework it defines alone.*

**See also:** Home COG, Connection Visa, Inter-COG Governed Visit

---

### Home COG

**COG to which a Visitor User belongs**, which attests to their identity. Identity authority, guarantor of origin environment compliance, issuer of the User Passport. Does NOT participate in remote execution.

**See also:** Host COG, User Passport, Inter-COG Governed Visit

---

### Reference COG (Official COG)

**COG designated as canonical holder** of sensitive data for a given domain. Hosts the KindMother Mother Instance (or equivalent "server") for that domain. Canonical holder of centralized-residence data; accessible by authorized actors (Governed Visit, sync). Sensitive data with centralized residence must not have their only copy on a terminal or third-party COG; their canonical copy resides on the reference COG.

**See also:** Sensitive Data Residence Policy, KindMother (Mother Instance), Host COG, WorrySentinel

---

### COG Tracker (Webway Tracker)

**COG whose administrator has chosen to endorse the Tracker role**: voluntarily expose an address (IP or domain name) to participate in the Miyukini Webway System (MWS) mesh and serve as a rendezvous point for discovery. Official port: 21000. Not a super-COG, not a carrier of business data, not a Visa or Passport authority.

**See also:** Miyukini Webway System, Host COG, Inter-COG Bridge

---

### Cores

**Conceptual engines** (Stratum 4) that govern system behaviour. Each core has exclusive authority in its domain. StrongFather (strategic decision), KindMother (data and persistence), Caring Nanny (state observation), Master Butler (capabilities and permissions), Border Guard (boundaries and trust), Ever Buddy (lifecycle and evolution), WorrySentinel (security governance), TAMR (human intervention).

**Fundamental rule:** Cores decide or govern, but never execute.

**See also:** Each core individually

---

## D

### Webway Host Session Declaration

**Announcement by a Host COG to the MWS network** that it is hosting a session of a given service and is accepting connections (address and port). Does not grant any access right; indicates where to present to request a Visa.

**See also:** Miyukini Webway System, Host COG, MWS secure declaration standard

---

### Visit Intent

**Access intent** issued by a Visitor User towards a Host COG. Contains requested_services, usage_nature, security_level, terminal_context. **It is an intent, not a permission.**

**See also:** User Passport, Connection Visa, Inter-COG Governed Visit

---

### DEPRECATED — life state

State of an element still functional but whose use is discouraged. A successor exists or is in preparation. Deprecation period defined; consumers advised to migrate; mandatory step before RETIRED.

**See also:** ACTIVE, RETIRED, Ever Buddy

---

### Silent Divergence

**Situation detectable by the Kernel** where a system declares a version but exhibits a different behavioural fingerprint. Typical causes: build recompiled differently, dependency modified silently, non-reproducible build, code injection. Maintenance signal, not error; detectable without network; deterministic and replayable. **The Kernel signals the divergence but never corrects it.**

**See also:** Behaviour Fingerprint, Explainable Maintenance, Kernel Maintenance Observability Contract

---

### Domain Operator

**Type of Operator** that performs a specific trade. Examples: Blog, Catalogue, Support, Knowledge base, Forum. *"Performs this specific trade."*

**See also:** Operator, Service Operator

---

### DRAFT — life state

State of an element under definition. Not usable in production, may change freely, no stability commitment.

**See also:** ACTIVE, DEPRECATED, RETIRED, Ever Buddy

---

## E

### Behaviour Fingerprint

**Structural signature** of the loaded system, produced by the Kernel. Captures: load order, structural call graph, invoked contracts, solicited invariants. It is a signature, not a log; no business content; no runtime data; deterministic and replayable. Use: compare two versions, detect silent drift, prove functional equivalence of builds. **The fingerprint observes and attests, but never corrects.**

**See also:** Silent Divergence, Explainable Maintenance, Kernel Maintenance Observability Contract

---

### Environment (COG)

**Sovereign, versioned, isolated and uniquely identified entity.** Complete cores version, unique iteration, unique ID (kernel-generated), bound Operators, strict boundaries. **Fundamental rule:** The Cores stratum is immutable. Any evolution is by creating a new complete environment.

**See also:** COG, Sovereignty, LOI-7

---

### Trust states (T0–T4)

States characterizing system integrity, governed by WorrySentinel. T0 Normal, T1 Unstable, T2 Degraded, T3 Restricted, T4 Blocked.

**See also:** WorrySentinel, Security levels

---

### Operator Team

**Governed collective of Operators** that collaborate under explicit rules to deliver a Service. Minimum 2 Operators; heterogeneous in security, responsibilities, exposure; bound by a Team Contract; rules validated by StrongFather. Not a new Operator, not a product, not a free hierarchy. **Key rule:** An Operator Team can exist operationally only under a valid Permission Mandate.

**See also:** Operator, Service, Permission Mandate, Team Contract

---

### Ever Buddy

**Lifecycle Core** (Stratum 4). Governs the evolution of structures, contracts and entities over time. Observes what was, what is, what will be, without ever executing migration. **Fundamental question:** *"How does the system evolve without ever breaking?"* Tool responsibility: versions, deprecation, compatibility, migration.

**See also:** Life states, Tool

---

### Governed Public Exposure Surface

**Exposure buffer zone** allowing external users to interact with a COG without entering it. Strictly unidirectional; no mandatory persistent identity; no access to cores or internal logic; no sovereign state. **Fundamental rule:** *The COG goes out to the external user, never the reverse.*

**See also:** External User, Public Access Mandate, Border Guard

---

## G

### Local Freeze

**Kernel capability** to mark a component as structurally frozen without affecting the rest of the system. Allowed: mark as frozen, refuse replacement or reload, let the rest evolve. Use: stabilize a critical zone during intervention, fix elsewhere without regression risk. Governance: StrongFather (authorization), EverBuddy (compatibility), Kernel (execution). **The freeze is decided by governance, executed by the Kernel, never reversed.**

**See also:** Kernel Maintenance Observability, StrongFather, Ever Buddy

---

## I

### Interface Operator

**Type of Operator** that exposes services in a usable way. Examples: Web UI, Mobile app, Dashboard, Admin panel. *"Exposes services in a usable way."*

**See also:** Operator, Service Operator

---

## K

### Kernel

**Neutral technical substrate** (between Stratum 0 and Stratum 3). Reusable, agnostic technical foundation, no business logic. Components: Id, Logger, Clock (trace only), Config, Lifecycle. Invariants: no business logic, no critical external dependency, no application protocol.

**See also:** Pyramid, Cores

---

### Kernel Maintenance Observability

**Set of low-level Kernel capabilities** for assisting code maintenance without ever performing automatic correction. Capabilities: Behavioural fingerprint, divergence detection, complexity map, local freeze, ambiguity detection, explainable maintenance. Kernel MAY: observe, attest, compare, signal, explain. Kernel may NEVER: correct, mutate, self-repair. **Miyukini does not maintain the code in place of the human. It makes the code maintainable without ambiguity.**

**See also:** Behaviour Fingerprint, Silent Divergence, Explainable Maintenance, Local Freeze

---

### KindMother

**Data Core** (Stratum 4). Absolute authority over data and persistence. Role: persistence, synchronization, data consistency. **Fundamental question:** *"How is data persisted and synchronized?"*

**See also:** Cores, WriteIntent

---

## L

### Local Sovereign ID (LSI)

**Level 1 environment identity.** Generated by the local kernel, always valid locally, guaranteed locally. Use case: isolated, permanently offline environment. Trust: sovereign — the environment self-declares.

**See also:** Verified ID, Witnessed ID

---

### Webway COG List

**List maintained by each COG participating in the Miyukini Webway System (MWS)** associating each known COG with a **status** (Trusted, Neutral, Under review, Distrusted, Rejected). Enables analysis and, if needed, rejection of a COG or connection considered malicious or unreliable.

**See also:** Miyukini Webway System, COG Tracker

---

### LOI-1 to LOI-8 (Laws of Autonomy)

**8 non-negotiable laws of autonomy** governing the Miyukini architecture. LOI-1 No critical external dependency at runtime; LOI-2 System accepts isolation as normal state; LOI-3 Local state is sovereign; LOI-4 No global time required; LOI-5 Cost proportional to hardware; LOI-6 Autonomy does not prevent federation; LOI-7 Cores stratum immutable — evolution by environment; LOI-8 Migration = diplomacy between environments.

**See also:** Autonomy, Sovereignty

---

## M

### Explainable Maintenance

**Kernel diagnostic mode** to provide governed traceability during incidents without exposing sensitive technical data. Provides: why a decision reached this point, which contracts were traversed, where governance stopped. Never provides: classic stacktrace, memory dump, user data. Governed traceability, not technical; understandable by a human without source code knowledge; works offline. **The diagnostic explains the governance path, never the implementation.**

**See also:** Kernel Maintenance Observability, Caring Nanny

---

### Master Butler

**Capabilities Core** (Stratum 4). Central registry of system capabilities and permissions. Role: catalogue of capabilities, definition of permissions, discovery. **Fundamental question:** *"What is possible in this environment?"* Tool responsibility: declares which Tools exist, links Capability → Tool, defines access permissions. Does NOT implement Tools, execute Tools, or decide whether a Tool should be called.

**See also:** Capability, Permission, Tool

---

### Permission Mandate (Allow Mandate)

**Delegated, temporary, bounded authorization** issued by StrongFather, allowing Operators to collaborate without constantly returning to central governance. Content: unique ID, authorized Operators, authorized flows, data types, maximum security level, validity conditions, revocation rules. Not a free token, not a classic session, not a decision cache, not an implicit right, not a global permission. **An Allow Mandate is not an optimization. It is a delegated act of governance.** Revocation causes: service ended, condition out of scope, rule violation, WorrySentinel alert, user leaves flow, environment change.

**See also:** StrongFather, Operator Team, Team Contract

---

### Public Access Mandate

**Authorization attached to a public service** to frame access by uncertified external users. Defined by Host COG. Contains public_services, allowed_methods, quotas, rate_limits, security_level (S1–S3), expected_behavior. Difference from Connection Visa: recipient (Visitor User vs exposed service), identity required (Passport vs no). **The mandate is attached to the service, not to the user.**

**See also:** External User, Governed Public Exposure Surface, Connection Visa

---

### Migration

**Process of data exchange between environments.** Migration ≠ direct communication. Rules: migration = formal process, explicit contract, controlled boundary, translation not raw copy. Actors: Border Guard (rules), BondingBrother (translation), StrongFather (decision), KindMother (persistence), Ever Buddy (compatibility).

**See also:** LOI-8, Environment

---

### Miyukini Webway System (MWS)

**Presence and discovery layer** for COG environments with network access. Allows COGs to declare themselves, know who is on the mesh, and facilitate initiation of governed visits (Passport, Visa) without transferring business data. **The Webway normalizes presence and facilitates exchange; it does not transport governance or data.**

**See also:** COG Tracker, Webway COG List, Inter-COG Connection, Inter-COG Bridge

---

### MiyukiniAdmin

**Sovereign Operator** — Sovereign administration console (Stratum 9). Exception to standard Operator logic; quasi-institutional authority; not usable by other Operators; acts under special protocol. Functions: installation, diagnostic, arbitration, exceptional access.

**See also:** Sovereign Operator, Operator

---

## N

### MWS Secure Declaration Standard

**Standard to be created and applied** for MWS announcements: exposed services, addresses (IP/ports), hosted sessions. Aims at authentication of declaration origin, integrity, unified format, abuse limitation.

**See also:** Miyukini Webway System, Webway Host Session Declaration, COG Tracker

---

### Security levels (0–4)

**Levels characterizing risk profile**, governed by WorrySentinel. 0 Public, 1 Standard, 2 Sensitive, 3 Critical, 4 Highest.

**See also:** WorrySentinel, Trust states

---

## O

### Operator

**Governed functional entity** that performs a role on behalf of the user (Stratum 7). **Canonical definition:** *An Operator is a governed functional entity that performs a role on behalf of the user within a Miyukini environment.* Types: Service Operator, Interface Operator, Automation Operator, Domain Operator, Sovereign Operator. An Operator is NOT: a product, an app, autonomous, sovereign. **In Miyukini, users do not install applications. They interact with governed Operators that perform roles on their behalf.**

**See also:** Tool, Toolkit

---

## P

### Permission

**Right granted to access a capability.** Conceptual authorization to use a capability. Explicit, associated with capabilities, assignable to roles, revocable, traceable. **Capability vs Permission:** Capability = technical power / "Can it be done?"; Permission = granted right / "Do we have the right?"

**See also:** Capability, Master Butler

---

### User Passport

**Identity attestation** issued by a Home COG to allow a user to visit other COGs. Issued by Home COG. Contains user_id, cog_origin_id, core_version, integrity_hash, issued_at, valid_until, signature. Guarantees: not forgeable, not transferable, readable but not modifiable. **The passport grants no rights. It only proves who you are and where you come from.**

**See also:** Home COG, Connection Visa, Inter-COG Governed Visit

---

### Sensitive Data Residence Policy

**Rule governing where the canonical copy of sensitive data resides**: certain data (personal, critical business) must not have their only copy on terminals or third-party COGs. Canonical copy resides on a reference COG; terminals/third-party COGs access via Governed Visit or sync; writes as WriteIntent, validated and persisted on Mother (reference COG).

**See also:** Reference COG, KindMother (Mother Instance), WorrySentinel, Security levels, Migration

---

### Miyukini Pyramid

**Stratified architecture** of the Miyukini ecosystem. Stratum 9 MiyukiniAdmin, 7 Operators, 6 Tools & Toolkits, 5 Interfaces & Adaptation, 4 System Cores, 3 Invariants & Contracts, K Kernel, 0 Hardware & OS.

**See also:** Each stratum individually

---

## R

### RETIRED — life state

State of an element removed from the system. Unavailable, use impossible. Mandatory transition from DEPRECATED; no return possible.

**See also:** DEPRECATED, Ever Buddy

---

## S

### Heterogeneous Security

**Principle that an Operator Team may combine different security levels**, each Operator keeping its own level. **One Operator has only one security level. A Team may combine several.** Rules: an Operator may never raise its level; a flow may never go down in security; bridges between levels are explicit, rare, auditable; bridges validated by WorrySentinel.

**See also:** Security levels, Operator Team, WorrySentinel

---

### Service

**Capability perceived by the user.** The Service is what the user sees and uses. **Service** = capability perceived by the user; **Operator** = governed execution unit. **A Service may be delivered by one Operator... or by an Operator Team.** User sees Services; system executes via Operators; complexity handled by collaboration, not accumulation.

**See also:** Operator, Operator Team

---

### Service Operator

**Type of Operator** that manages a functional domain. Examples: CMS, Auth, E-commerce, CRM, Monitoring, Search, Billing. *"Manages this domain for me."*

**See also:** Operator, Domain Operator, Service

---

### Sovereignty (Environment)

**Principle that a COG is a sovereign entity**, versioned, isolated and identifiable. Cores stratum immutable; no patch, only complete environments; Operator bound to a single environment; Migration = explicit diplomacy.

**See also:** Environment, LOI-7, LOI-8

---

### Sovereign Operator

**Exception type of Operator** with quasi-institutional authority. Only example: MiyukiniAdmin. Not a normal citizen; acts under special protocol; not usable by other Operators.

**See also:** MiyukiniAdmin, Operator

---

### StrongFather

**Decision Core** (Stratum 4). Strategic and policy decision engine. **Issuer of Permission Mandates.** Role: Decide whether an action should be done, without ever executing it. **Fundamental question:** *"Should this action be done?"* Key responsibilities: strategic decision, Team Contract validation, Permission Mandate issuance, mandate revocation if needed. Invariants: never has execution authority, never modifies state or fact; Decision ≠ Execution.

**See also:** Cores, KindMother, Permission Mandate, Team Contract

---

## T

### TAMR (Trust & Authority Mediation Resolver)

**Human intervention Core** (Stratum 4). Defines the points of human intervention in the system. Role: Define when humans have the right to intervene. **Fundamental question:** *"When do humans have the right to intervene in the system?"*

**See also:** Cores, WorrySentinel

---

### Tool

**Governed executable capability** (Stratum 6), no authority, no business decision, no context knowledge. **Canonical definition:** *A Tool is an executable capability, without authority, without business decision, without knowledge of the calling Operator, governed by the Cores.* Atomic capability, no authority, no business logic, governed by Cores. **A Tool does, but never decides.** Examples: layout.render, form.validate, query.execute.

**See also:** Toolkit, Operator

---

### Toolkit

**Official composition of Tools** (Stratum 6), validated and declared by the environment. **Canonical definition:** *A Toolkit is an official composition of Tools, validated and declared by the environment, optimized for efficiency, consistency and performance.* Aggregates existing Tools; adds no new capability; no business logic; governed. **A Toolkit orchestrates, but adds no capability.**

**See also:** Tool, Master Butler

---

## U

### External User (Public User / Anonymous User / Web Visitor)

**Uncertified consumer** of services exposed by a COG, with no governance of their own. Not a citizen, not an inter-COG visitor, not a system participant. No sovereign identity, no Home COG, no Passport or Visa; access only via Governed Public Exposure Surface; subject to a Public Access Mandate. Aggressive degradation possible: Throttle, Downgrade, Freeze, Block, Blackhole. **An external user never enters a COG. They interact only with a governed exposure surface.** **External users are not visitors. They are consumers of exposed surfaces, under a public mandate.**

**See also:** Governed Public Exposure Surface, Public Access Mandate, Visitor User

---

### Visitor User

**User temporarily accessing a foreign COG** via a governed visit mechanism. Citizen in their Home COG; governed visitor in the Host COG. Keeps identity; loses all execution sovereignty; acts only via a Connection Visa; carries no core, no logic, no state. **The user is never sovereign outside their COG.**

**See also:** Home COG, Host COG, Connection Visa, User Passport

---

## V

### Verified ID (VID)

**Level 2 environment identity.** LSI verified by a global registry. Use case: connected, federated environment. Trust: attested — a third party has verified the identity.

**See also:** Local Sovereign ID, Witnessed ID

---

## W

### Witnessed ID (WID)

**Level 3 environment identity.** LSI verified by indirect exchange. Use case: semi-connected environment, USB key, QR, signature. Trust: witnessed — other environments attest.

**See also:** Local Sovereign ID, Verified ID

---

### Connection Visa

**Temporary authorization** issued by a Host COG to frame a Visitor User’s session. Issued by Host COG. Contains authorized_services, accessible_cores, security_level, execution_rules, time_limits, functional_limits, terminal_constraints, revocation_conditions. Temporary, revocable, non-transferable, audited, strictly interpreted. Visa security levels: S1 Observation, S2 Controlled interaction, S3 Real time, S4 Sensitive, S5 Critical. **The Visa defines the visitor’s legal universe. One user = one Visa = one unique level.**

**See also:** Host COG, Visitor User, Inter-COG Governed Visit

---

### Inter-COG Governed Visit

**Temporary access model** allowing a user from one COG to access another COG’s services without importing their governance. Actors: Home COG (identity authority, Passport issuer), Visitor User (citizen visiting under foreign governance), Host COG (executive sovereign, Visa issuer), Inter-COG Bridge (diplomatic channel). Sequence: local pre-validation, presentation to Bridge (Passport + Visit Intent), Host COG customs, Visa issuance, active session, end or breach. Non-negotiable: no core shared, no state migrated directly, no power delegated; single active governance; identity ≠ authority; security before fluidity. **A COG never hosts a foreign governance. It only hosts visitors, under a visa, in a framework it defines alone.**

**See also:** Host COG, Home COG, User Passport, Connection Visa, Visitor User, Inter-COG Bridge

---

### WorrySentinel

**Security governance Core** (Stratum 4). Governs security levels and trust states. Role: Govern security without executing technical control. **Fundamental question:** *"What security level and what trust state apply?"* Tool responsibility: required security level, blocking in case of threat, audit. Decides: global trust level, active security level, authorized operating mode. Does NOT decide: actions, permissions, data.

**See also:** Security levels, Trust states

---

### WriteIntent

**Write intent** submitted to KindMother. Represents a data modification request. Subject to validation; traceable; may be accepted, refused, or deferred.

**See also:** KindMother

---

## Founding phrases (summary)

**COG:** *"Miyukini is not an OS. It's the cog that makes digital systems work together."*

**Operator:** *In Miyukini, users do not install applications. They interact with governed Operators that perform roles on their behalf.*

**Tool & Toolkit:** *Tools are governed executable capabilities. Toolkits are official compositions of tools, optimized for efficiency but never for authority.*

**Sovereignty:** *In Miyukini, the Cores stratum is immutable. Any evolution is by creating a new complete environment. Operators are bound to a single environment and cannot exist outside it.*

**Autonomy:** *The network improves the system; it does not condition it.*

**Complexity:** *In Miyukini, complexity is handled by collaboration, not accumulation.*

**Permission Mandate:** *An Allow Mandate is not an optimization. It is a delegated act of governance.*

**Security:** *Segmented risk, not uniform security.*

**Inter-COG Visit:** *A COG never hosts a foreign governance. It only hosts visitors, under a visa, in a framework it defines alone.*

**External users:** *External users are not visitors. They are consumers of exposed surfaces, under a public mandate.* *An external user never enters a COG. The COG goes out to them, never the reverse.*

---

## Terminological correspondence table

| ❌ Incorrect term | ✅ Correct term |
|------------------|-----------------|
| Product | **Operator** |
| App | **Operator** or **Interface Operator** |
| Finished product | **Operator** |
| Intermediate product | **Tool** or **Toolkit** |
| Create a product | **Deploy an Operator** |
| Use an app | **Interact with an Operator** |
| Marketplace | **Operator registry** |
| Decision Window | **Permission Mandate** |
| Temporary Decision | **Mandated Authorization** |
| Fast Path | **Mandated Path** |
| Operator Collaboration (free) | **Mandated Collaboration** |
| Super-Operator | **Operator Team** |
| Tool | **Tool** |
| Toolkit | **Toolkit** |
| Operator | **Operator** |
| User Passport | **User Passport** |
| Connection Visa | **Connection Visa** |
| Visit Intent | **Visit Intent** |
| Visitor User | **Visitor User** |
| Host COG | **Host COG** |
| Home COG | **Home COG** |
| Reference COG / Official COG | **Reference COG** |
| Inter-COG Bridge | **Inter-COG Bridge** |
| Public User | **External User** |
| Anonymous User | **External User** |
| Web Visitor | **External User** |
| Public Exposure Surface | **Governed Public Exposure Surface** |
| Public Access Mandate | **Public Access Mandate** |
| Tracker (Webway role) | **COG Tracker** |

---

**Date of creation:** 2026-01-27  
**Version:** 1.10 (addition of Reference COG, Sensitive Data Residence Policy)  
**Status:** Normative reference document — OFFICIAL GLOSSARY

**Cross-references:**
- [COG Definition](Miyukini%20-%20Definition%20COG.md)
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md)
- [Mandates and Operator Teams](Miyukini%20-%20Mandates%20and%20Operator%20Teams.md)
- [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md)
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md)
- [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md)
- [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md)
- [Project Objective](Miyukini%20-%20Project%20Objective.md)
- [Inter-COG Connection](Miyukini%20-%20Inter-COG%20Connection.md)
- [Kernel Maintenance Observability Contract](Miyukini%20-%20Kernel%20Maintenance%20Observability%20Contract.md)
