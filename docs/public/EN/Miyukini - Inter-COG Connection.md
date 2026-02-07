# Miyukini Conceptual References — Inter-COG Connection

## Context

This document defines the **governed inter-COG visit** architecture in the Miyukini ecosystem. It formalizes the mechanisms that allow a user from one COG to temporarily access another COG’s services, under strict control of the host governance.

## Scope

- Definition of inter-COG connection actors
- Conceptual objects (Passport, Visa, Visit Intent)
- Full connection sequence
- Security levels and degradation cases
- Non-negotiable fundamental principles

---

## 1. Overview

Miyukini supports a **governed inter-COG visit** model in which:

- A **Host COG** exposes Services
- A **Visitor User** from a **Home COG**
- **Temporarily** accesses those services without importing their governance
- Under **strict, traceable, revocable, and tiered** control

### Cardinal principles

> **The COG is the sovereign unit.**  
> **The user is never sovereign outside their COG.**

---

## 2. Fundamental actors

### 2.1 Home COG

**Role:**
- User identity authority
- Guarantor of origin environment compliance
- Issuer of the User Passport

**Responsibilities:**
- Verify local integrity
- Attest Core stratum version
- Provide a verifiable identity
- **Does NOT participate in remote execution**

---

### 2.2 Visitor User

**Status:**
- Citizen in their Home COG
- Governed visitor in the Host COG

**Characteristics:**
- Keeps their identity
- Loses all execution sovereignty
- Acts only via a Connection Visa
- Carries no core, no logic, no state

---

### 2.3 Host COG

**Role:**
- Executive sovereign of the session
- Single source of truth for state
- Security and arbitration authority

**Responsibilities:**
- Verify the visitor
- Grant or refuse access
- Strictly frame execution
- Monitor the session (WorrySentinel)
- Revoke at any time

---

### 2.4 Inter-COG Bridge (BondingBrother extended)

**Role:**
- Diplomatic channel
- Transport of identities, intents and authorizations
- **No decision-making power**
- **No business state**

> **The bridge never trusts; it transports.**

---

## 3. Key conceptual objects

### 3.1 User Passport

**Issued by:** Home COG

**Contains:**
| Field | Description |
|-------|-------------|
| `user_id` | Unique user identity |
| `cog_origin_id` | Home COG ID |
| `core_version` | Exact Core stratum version |
| `integrity_hash` | Integrity fingerprint (Core + Kernel) |
| `issued_at` | Issue timestamp |
| `valid_until` | Validity duration |
| `signature` | Home COG signature |

**Guarantees:**
- Not forgeable
- Not transferable
- Readable but not modifiable

> **The passport grants no rights.**  
> **It only proves who you are and where you come from.**

---

### 3.2 Visit Intent

**Issued by:** Visitor User

**Contains:**
| Field | Description |
|-------|-------------|
| `requested_services` | List of requested Services |
| `usage_nature` | Nature of use (read, interaction, real-time, etc.) |
| `security_level` | Required security level |
| `terminal_context` | Terminal context (PC, mobile, web…) |

> **It is an intent, not a permission.**

---

### 3.3 Connection Visa

**Issued by:** Host COG

**Contains:**
| Field | Description |
|-------|-------------|
| `authorized_services` | Authorized services |
| `accessible_cores` | Accessible cores (indirectly) |
| `security_level` | Granted security level |
| `execution_rules` | Execution rules |
| `time_limits` | Time limits |
| `functional_limits` | Functional limits |
| `terminal_constraints` | Terminal constraints |
| `revocation_conditions` | Revocation conditions |

**Characteristics:**
- Temporary
- Revocable
- Non-transferable
- Audited
- Strictly interpreted

> **The Visa defines the visitor’s legal universe.**

---

## 4. Visa security levels

| Level | Name | Typical use | Characteristics |
|-------|------|-------------|-----------------|
| **S1** | Observation | Read, spectator | No modifiable state |
| **S2** | Controlled interaction | UI, forms | Strict validation |
| **S3** | Real time | Game, collaboration | Enhanced monitoring |
| **S4** | Sensitive | Admin, finance | Continuous audit |
| **S5** | Critical | MiyukiniAdmin | Ultimate arbitration |

> **One user = one Visa = one unique level.**

---

## 5. Full connection sequence

### Step 1 — Local pre-validation

Home COG verifies:
- Kernel integrity
- Core version
- Local compliance

**Result:** Issues the **User Passport**

---

### Step 2 — Presentation to the Bridge

Visitor submits:
- Passport
- Visit Intent

**The Bridge forwards without interpretation**

---

### Step 3 — Host COG customs

Host COG verifies:
- Passport validity
- Version compatibility
- Admission policy
- Required security level
- Internal state (load, risks)

> **Refusal possible without detailed justification**

---

### Step 4 — Visa issuance

Host COG:
- Generates a strict Visa
- Records the session
- Activates monitoring (WorrySentinel)

---

### Step 5 — Active session

- Visitor is treated as a local user
- **BUT** only within the Visa scope
- Any action out of scope = **rejection**

---

### Step 6 — End or breach

Possible causes:
- Natural expiration
- Manual revocation
- Detected degradation
- Network break
- Rule violation

> **State remains 100% in the Host COG.**

---

## 6. Sequence diagram

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Home COG    │    │   Bridge    │    │  Host COG   │    │   Visitor   │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │                  │
       │◄─── Passport request ──────────────────────────────────┤
       │                  │                  │                  │
       ├─── Integrity check ───►     │                  │
       │                  │                  │                  │
       │──── Passport ──────────────────────────────────────►   │
       │                  │                  │                  │
       │                  │◄── Passport + Visit Intent ──────────┤
       │                  │                  │                  │
       │                  ├─── Transport ───►│                  │
       │                  │                  │                  │
       │                  │                  ├── Customs check  │
       │                  │                  │                  │
       │                  │◄─── Visa ────────┤                  │
       │                  │                  │                  │
       │                  ├─────────────────────── Visa ───────►│
       │                  │                  │                  │
       │                  │                  │◄── Visa actions ─┤
       │                  │                  │                  │
       │                  │                  ├── WorrySentinel  │
       │                  │                  │   monitoring    │
       │                  │                  │                  │
```

---

## 7. Degradation & breach cases

### 7.1 Network desynchronization

The session may:
- Be suspended
- Switch to read-only
- Be cleanly terminated

### 7.2 Suspected intrusion

- Immediate Visa revocation
- Journaling
- Option to blacklist Home COG

### 7.3 Home COG failure

- **No direct impact**
- Host remains sovereign
- Session continues or ends per local policy

---

## 8. Non-negotiable principles

| Forbidden | Required |
|-----------|----------|
| ❌ No core is shared | ✅ Single active governance |
| ❌ No state is migrated directly | ✅ Identity ≠ authority |
| ❌ No power is delegated | ✅ Security before fluidity |

---

## 9. Position in the Miyukini pyramid

| Component | Role in inter-COG |
|-----------|-------------------|
| **Kernel** | Identity, timestamping, integrity |
| **BorderGuard** | Inter-COG boundary |
| **BondingBrother** | Diplomatic transport |
| **WorrySentinel** | Monitoring & degradation |
| **StrongFather** | Access decisions |
| **MasterButler** | Exposable services |

---

## 10. Summary phrase

> **A COG never hosts a foreign governance. It only hosts visitors, under a visa, in a framework it defines alone.**

---

## 11. Uncertified external users

*(Public Users / Anonymous Users / Web Visitors)*

### 11.1 Conceptual positioning

These users:
- **Are NOT** citizens
- **Are NOT** inter-COG visitors
- **Have** no governance

> **They are consumers of exposed services, not system participants.**

### 11.2 Fundamental principle

> **An external user never enters a COG.**  
> **They interact only with a governed exposure surface.**  
> **The COG goes out to them, never the reverse.**

---

### 11.3 Governed Public Exposure Surface

**Characteristics:**
- Strictly unidirectional
- No mandatory persistent identity
- No access to cores
- No access to internal logic
- No sovereign state

> **It is a governed, filtered, instrumented buffer zone.**

---

### 11.4 Key difference from inter-COG Visitor

| Criterion | Inter-COG Visitor | External user |
|-----------|-------------------|---------------|
| **Verified identity** | ✅ Passport | ❌ Optional |
| **Home COG** | ✅ Yes | ❌ No |
| **Visa** | ✅ Yes | ❌ No |
| **Governance** | Host COG | Host COG |
| **Core access** | Indirect | ❌ Never |
| **Persistent state** | Possible (local) | ❌ |
| **Use case** | Game, tools, SaaS | Website, showcase |

---

### 11.5 Treatment in Miyukini

#### Status: Public user

An external user is treated as:
> **An actor with no sovereign identity, subject to strict public rules.**

#### Public Access Mandate

Instead of Passport + Visa, a **Public Access Mandate** is used.

**Defined by the Host COG, it specifies:**

| Field | Description |
|-------|-------------|
| `public_services` | Accessible public services |
| `allowed_methods` | Authorized methods |
| `quotas` | Usage quotas |
| `rate_limits` | Rate limits |
| `security_level` | Security level |
| `expected_behavior` | Expected behaviour |

> **The mandate is attached to the service, not to the user.**

#### Applied governance

Even without strong identity:

| Core | Role |
|------|------|
| **StrongFather** | Decides if the request is admissible |
| **MasterButler** | Limits exposed capabilities |
| **WorrySentinel** | Monitors abuse, suspicious patterns, anomalies |
| **BorderGuard** | Filters entries |
| **KindMother** | May read (never write) |

---

### 11.6 Concrete cases

#### Public website (CMS-like)

| Aspect | Detail |
|--------|--------|
| **User** | Browser, no login |
| **Access** | Content read, search, navigation |
| **Security** | Public Mandate S1 (Observation), Rate limiting, Anti-scraping, Behavioural detection |

#### Public form

| Aspect | Detail |
|--------|--------|
| **User** | Uncertified, incoming data |
| **Access** | Encapsulated write, strict validation, no direct execution |
| **Security** | Public Mandate S2, Logical sandbox, Strong validation, Journaling |

#### Interactive demo / web game

| Aspect | Detail |
|--------|--------|
| **User** | Anonymous or pseudo |
| **Access** | Real-time interaction, no critical state |
| **Security** | Limited Public Mandate S3, Quotas, Short session, Automatic degradation |

---

### 11.7 Degradation & blocking

For external users, degradation is **much more aggressive**:

| Action | Description |
|--------|-------------|
| **Throttle** | Slowdown |
| **Downgrade** | Fewer features |
| **Freeze** | Read-only |
| **Block** | IP / session / pattern |
| **Blackhole** | Neutral response, no exploitable error |

> **No acquired right, no negotiation.**

---

### 11.8 Robustness guarantees

**No external user:**
- ❌ May write to a core
- ❌ May influence governance
- ❌ May trigger migration

**Any exposure is:**
- ✅ Voluntary
- ✅ Bounded
- ✅ Revocable
- ✅ Observable

---

### 11.9 Position in the pyramid

```
        External User
               ↓
    Governed Public Exposure Surface
               ↓
    BorderGuard + WorrySentinel
               ↓
    StrongFather / MasterButler
               ↓
        Cores (inaccessible)
               ↓
            Kernel
```

---

### 11.10 Summary phrase

> **External users are not visitors. They are consumers of exposed surfaces, under a public mandate.**

---

## 12. Future evolutions

- [ ] Formalize an **Inter-COG Visit Contract**
- [ ] Define the **version compatibility matrix**
- [ ] Integrate with **cross-terminal** properly
- [ ] Specify the **inter-COG blacklist** protocol
- [ ] Define **visitor session SLAs**
- [ ] Specify **standard quotas per Public Mandate level**
- [ ] Define the **blackhole** protocol
- [ ] Integrate **Miyukini Webway System (MWS)** presence layer for COG discovery → see [Miyukini Webway System](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md)

---

## Cross-references

- [Miyukini Webway System (MWS)](./Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20System.md) — presence and discovery layer
- [Ecosystem Dependency Contract](./Miyukini%20Conceptual%20References%20-%20Ecosystem%20Dependency%20Contract.md)
- [Security Levels](./Miyukini%20Conceptual%20References%20-%20Security%20Levels.md)
- [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md)
- [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md)
- [COG Definition](Miyukini%20-%20Definition%20COG.md)

---

*Document created 27/01/2026*  
*Classification: Conceptual reference*
