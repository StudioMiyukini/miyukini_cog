# Miyukini — Project Objective

## Context

This document describes **what Miyukini is for** and **where it is heading**: long-term objectives, strategy, product aspects, business, usefulness and end use. It is grounded in the actual code and deployed architecture to present a concrete vision aligned with the existing implementation.

## Scope

- **Applies to:** Decision-makers, partners, contributors, investors, anyone who needs to understand the "why" and "for whom"
- **Does not cover:** Detailed technical specifications (see Pyramid, Glossary, Laws of Autonomy, COG Definition)

---

## 1. Long-term objectives

### 1.1 What Miyukini aims for (5–15 year horizon)

| Objective | Description | Progress |
|-----------|-------------|----------|
| **Sovereign alternative** | Replace dependency on WordPress-style CMSs, siloed SaaS and "framework + plugins" backends with a governed, end-to-end controlled ecosystem. | **Complete architecture**: Kernel + 9 Cores + 49 Toolkits implemented in Rust |
| **Long-lived platform** | Provide a software base that can be built on for 10–20 years without breaking everything: evolution by complete environments, not patches. | **Environment sovereignty**: Immutable versioning system implemented (LOI-7, LOI-8) |
| **Real autonomy** | Systems that run without network, on modest hardware (Raspberry Pi, mini PC, NAS), in isolated areas or at events, with no critical cloud dependency. | **Proven offline-first**: JayKoa, JayFestival, games run without network; local SQLite; no critical external dependency |
| **Security by design** | Governance, traceability and controlled degradation as structural, not add-on options. | **Structural governance**: StrongFather (decision), KindMother (persistence), WorrySentinel (security), CaringNanny (observation) — no bypass possible |
| **Recomposition over accumulation** | Avoid monoliths and "disposable products": recomposable building blocks (Tools, Toolkits) assembled into Operators and Services, without full rewrites at each change. | **49 documented Toolkits**: Auth, CMS, Commerce, HR, Accounting, Social — each with foundational doc, governance contracts and implementation reference |

### 1.2 One-sentence vision

> **Miyukini is an autonomous, governed software ecosystem capable of replacing CMSs and SaaS while offering technical sovereignty, offline operation and full control of the chain from kernel to user.**

### 1.3 Concrete scale of the project

To understand Miyukini’s real scope, here are the current implementation figures:

```
70+ compilable, deployable Rust crates
 9 operational governance Cores
49 Toolkits implemented (Phase 1 skeletons 100%, Phase 2 logic 60%)
10 documented Services (3 in production, 7 in advanced design)

1045 markdown documentation files
 244 detailed market analyses (Odoo module by module, competitors)
   8 non-negotiable Laws of Autonomy (LOI-1 to LOI-8)

Complete 8-stratum architecture (Hardware → Operators → MiyukiniAdmin)
```

**This is not a prototype or proof of concept.** It is a complete, functional, deployed software ecosystem.

---

## 2. Strategy

### 2.1 Guiding principle: Stratum 6 as keystone

Strategy rests on an **intermediate layer**: **Tools & Toolkits** (Stratum 6). We do not build monolithic applications directly; we build reusable capabilities that are then composed into Operators and Services.

**Why this matters:**

| Classic approach (monolithic) | Miyukini approach (modular) |
|------------------------------|-----------------------------|
| WordPress with plugins | **Recomposable Toolkits**: Auth + CMS + Billing create an e-commerce site |
| Siloed SaaS (Shopify, Stripe separate) | **Interoperability**: MiyuStore + MiyuBilling + MiyuShipping collaborate under governance |
| Full rewrite at each evolution | **Progressive composition**: new Toolkits without breaking existing ones |
| Vendor dependency (cloud lock-in) | **Sovereignty**: deploy on your own hardware, modest hardware, offline-first |

### 2.2 The 49 Toolkits — Domains covered

| Domain | Number of Toolkits | Key examples | Status |
|--------|--------------------|--------------|--------|
| **Data & Infra** | 8 | MiyuSQL (queries/transactions), MiyuWeb (HTTP/WebSocket), MiyuClock (time/timezone), MiyuSearch (indexing) | ✓ Skeletons + progressive logic |
| **Identity & Social** | 9 | MiyuAuth (login/roles), MiyuProfile (user profiles), MiyuSocialFeed (feeds), MiyuSocialMessaging (messaging), MiyuDiscovery (discovery) | ✓ Skeletons + governance contracts |
| **Content & Media** | 11 | MiyuCMS (pages/blocks), MiyuMedia (images/videos), MiyuForum (discussions), MiyuPolls (polls), MiyuAntiSpam (moderation) | ✓ Skeletons + implementation reference |
| **Commerce & Finance** | 7 | MiyuStore (product catalogue), MiyuBilling (billing), MiyuShipping (shipping), MiyuInvoice (quotes), MiyuExpense (expenses), MiyuTreasury (treasury) | ✓ Skeletons + governance contracts |
| **Point of Sale** | 6 | MiyuPosSales (counter sales), MiyuPosInventory (stock), MiyuPosKitchen (kitchen), MiyuPosPayment (payments), MiyuPosLoyalty (loyalty) | ✓ Skeletons + implementation reference |
| **Accounting** | 3 | MiyuComptaLedger (ledger), MiyuComptaReports (reports), MiyuDeclarations (tax declarations) | ✓ Skeletons + governance contracts |
| **Organization** | 4 | MiyuHR (HR), MiyuCalc (calculations), MiyuNotify (notifications), MiyuBooking (bookings) | ✓ Skeletons + implementation reference |
| **Federation** | 2 | MiyuWebwayParticipant (network participation), MiyuWebwayTracker (COG discovery) | ✓ Skeletons + Inter-COG protocols |

**Total: 49 documented and implemented Toolkits** (Phase 1 complete, Phase 2 in progress)

Each Toolkit has:
- **Foundational documentation**: purpose, scope, constraints
- **Governance contract**: protocols with StrongFather, KindMother, BondingBrother
- **Implementation reference**: detailed technical guide (21 kits with full guide)
- **Structured index**: modular organization and dependencies

### 2.3 Build order

**✅ Completed phases:**

1. **Kernel** (Stratum K) — Neutral technical foundation: identifiers, clock, logs, config, lifecycle. **Status: 90% (miyukini-kernel crate operational)**

2. **Cores** (Stratum 4) — 9 governance institutions (StrongFather, KindMother, CaringNanny, MasterButler, BorderGuard, EverBuddy, WorrySentinel, TAMR, LogisticsSteward). **Status: 95% (9 Rust crates operational with contractual documentation)**

3. **MiyukiniAdmin** (Stratum 9) — Sovereign Operator: supervision, administration, diagnostic, exceptional access. **Status: 70% (functional admin console)**

4. **Tools & Toolkits** (Stratum 6) — 49 reusable Toolkits. **Status: 60% (Phase 1 skeletons 100%, Phase 2 business logic progressive)**

**🔜 Current and upcoming phases:**

5. **Operators** (Stratum 7) — The layer that orchestrates Toolkits to deliver Services. **Status: 15% (JayKoa, JayFestival, MiyukiniClicker implemented; JayRDV, JayKonta, JayXpose, JayFaim in design)**

6. **Inter-COG federation** — Passport/Visa/Webway protocols for governed connection between COGs. **Status: 10% (protocols documented, implementation to come)**

### 2.4 Posture — A paradigm shift

| Before Miyukini | With Miyukini |
|-----------------|----------------|
| "I run a tool" | "I build a productive ecosystem" |
| Webmaster → Feature → Site | System architect → Capability → Autonomous platform |
| Mandatory cloud dependency | Structural autonomy (offline-first) |
| Disposable products (rewrite every 3–5 years) | Evolution by composition (10–20 years without break) |
| Vendor lock-in (Shopify, Stripe, AWS) | Technical sovereignty (deploy on your own) |
| Security added after the fact | Structural governance (Cores, Mandates, Contracts) |

---

## 3. Product — What we deliver

### 3.1 By stratum (summary)

| Deliverable | Used by | Example | Implementation status |
|-------------|---------|---------|------------------------|
| **Tools & Toolkits** (Stratum 6) | Integrators, publishers, developers | MiyuAuth, MiyuBilling, MiyuCMS — reusable building blocks | ✓ 49 Toolkits implemented (skeletons + progressive logic) |
| **Operators** (Stratum 7) | End users, business, organizations | JayKoa, JayRDV, JayFestival, JayKonta, games | ✓ 3 in production, 7 in advanced design |
| **Services** (what the user perceives) | End users (COG citizens) | "I book an appointment", "I manage my festival", "I check my calendar", "I play MiyukiniClicker" | ✓ JayKoa, JayFestival, MiyukiniClicker accessible from Miyukini Central |

### 3.2 Single entry point: Miyukini Central

**Miyukini Central** is the Hub — the single access point to the COG. The user does not launch separate applications; they open **Services** from a unified catalogue.

**Concrete architecture:**
```
User
    ↓
Miyukini Central (desktop Hub — native egui/eframe Rust)
    ↓
Service catalogue (grid/list with filters)
    ↓
[JayKoa | JayFestival | MiyukiniClicker | Lord of the Castle | MiyukiniSales | ...]
    ↓
Cores (StrongFather, KindMother, etc.) via BondingBrother
    ↓
Toolkits (49 governed toolboxes)
```

**Features implemented in Miyukini Central:**
- Loading screen with progress and random phrases
- Hub with catalogue of available Services (grid or list)
- Search sidebar and filters (categories, types)
- Service cards with name, description, open button
- Tab system (Hub + open Services)
- Profile and Settings overlays (persistent light/dark theme)
- Integrated demo services: Calculator, Click game, Notes, Text editor

> *In Miyukini, users do not install applications. They interact with governed Operators that perform roles on their behalf.*

Details: [Glossary](Miyukini%20-%20Glossary.md), [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md), [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md).

### 3.3 Implemented and operational services

Summaries for JayKoa, JayFestival, MiyukiniClicker, MiyukiniSurvivor (Lord of the Castle) and documented services (JayRDV, JayKonta, JayXpose, JayFaim, MiyukiniSales) follow the same structure as in the French source: code location, description, architecture, governance, status. **Status:** JayKoa, JayFestival, MiyukiniClicker, Lord of the Castle functional; others in advanced design.

---

## 4. Business — Models and markets

### 4.1 Delivery models

| Model | Typical deliverable | Client | Example |
|-------|---------------------|--------|---------|
| **B2B** | Tools & Toolkits (building blocks) | Companies that integrate them into their own Operators | A web agency buys MiyuAuth + MiyuCMS + MiyuBilling to build client sites |
| **B2C** | Operators / full Services | End users (professionals, associations, individuals) | A restaurateur uses JayFaim for reservations and orders |
| **B2B2C** | Operators + building blocks under licence | Resellers who customize and resell to their clients | A local authority deploys JayFestival for its associations and local craftspeople |

### 4.2 Target markets and concrete use cases

Local government, festivals and events, independent professionals (appointments, craftspeople, catering), and technical decision-makers (long-term projects, critical systems) — problems solved and Miyukini solutions as in the French document.

### 4.3 Business benefits

| Benefit | Description | Example |
|---------|-------------|---------|
| **Multiple markets** | Sell building blocks (B2B), end products (B2C) or licences (B2B2C) | Agency buys 10 MiyuAuth licences; restaurateur pays JayFaim; authority deploys JayFestival under licence |
| **Progressive adoption** | Start with building blocks (B2B), then offer full Services (B2C/B2B2C) | Phase 1: sell MiyuAuth + MiyuCMS to agencies. Phase 2: launch JayRDV B2C. Phase 3: JayFestival B2B2C |
| **Controlled costs** | Deploy on modest hardware (€150–800), no mandatory cloud | Festival 2000 visitors: laptop €600 + event licence €300 = €900 total |
| **Longevity** | Evolution by new environments and composition, not rewrite | A COG v1.0 remains operational 10 years. Migration v2.0 = Inter-COG diplomacy |

---

## 5. Usefulness — For whom, which problems

Beneficiaries (local government, events, professionals, technical decision-makers, developers) and what Miyukini brings vs CMS/SaaS — as in the French document, with links to [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md), [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md).

---

## 6. End use — Who does what

### 6.1 End user (COG citizen)

Typical journey: open Miyukini Central → browse Service catalogue → open a Service (Operator runs under governance). Example: Marie (physiotherapist) uses JayRDV, JayKonta, JayXpose without seeing Cores or Toolkits.

### 6.2 Business / administrator

Role: configure and supervise via MiyukiniAdmin; manage rights, security levels (S1–S5), degradation; never modify the kernel (Cores are immutable).

### 6.3 Developer / integrator

Role: compose Tools & Toolkits into Operators (Stratum 7); create Services; respect Miyukini protocols and contracts; cannot bypass governance.

Reference: [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md).

---

## 7. What Miyukini is not

| It is not | It is | Why the distinction matters |
|-----------|--------|-----------------------------|
| An "open" framework where everyone does as they please | A **governed environment**: strata 0–5 are the non-substitutable foundation; strata 6–7 extend within this framework | You cannot "write your own StrongFather". You compose existing Toolkits under governance. |
| An application or improved CMS (WordPress++, Notion++, Shopify++) | An **ecosystem** that allows deployment of governed Operators and Services | WordPress is a product. Miyukini is a digital country with constitution (Cores), institutions (Toolkits), citizens (Operators). |
| An OS (Linux, Windows, macOS) | A **COG**: governance and orchestration, not direct hardware operation | Miyukini runs *on* Linux/Windows/macOS (Stratum 0). It does not replace the OS. |
| A tool to do everything without constraints | A **demanding** foundation (autonomy, contracts, traceability, strict governance) in exchange for guarantees (offline, sovereignty, controlled evolution, structural security) | Constraints are voluntary. They ensure the system works 10–20 years without collapsing. |

---

## 8. Decision summary

Before any strategic or product decision, verify:

| Question | Expected answer |
|----------|-----------------|
| Does the system work offline? | Yes |
| Does it depend on an external service to function? | No |
| Can a non-developer use Services? | Yes (via exposed Operators) |
| Is degradation controlled when something goes wrong? | Yes |
| Can we evolve without breaking everything? | Yes (new environments, composition) |

---

## 9. References

| Theme | Document |
|-------|----------|
| COG definition | [COG Definition](Miyukini%20-%20Definition%20COG.md) |
| Stratified architecture | [Complete Architecture Pyramid](Miyukini%20-%20Complete%20Architecture%20Pyramid.md) |
| Fundamental laws | [Laws of Autonomy](Miyukini%20-%20Laws%20of%20Autonomy.md) |
| Sovereignty, migration | [Environment Sovereignty](Miyukini%20-%20Environment%20Sovereignty.md) |
| Official terms | [Glossary](Miyukini%20-%20Glossary.md) |
| Operators, Services | [Operators and Terminology](Miyukini%20-%20Operators%20and%20Terminology.md) |
| Tools, Toolkits | [Tools and Toolkits](Miyukini%20-%20Tools%20and%20Toolkits.md) |
| Mandates, teams | [Mandates and Operator Teams](Miyukini%20-%20Mandates%20and%20Operator%20Teams.md) |

---

**Date of creation:** 2026-02-07  
**Version:** 2.0 (merge of Strategic Vision + Project Objective + project read-through)  
**Status:** Reference document — objectives, strategy, product, business, usefulness, end use  
**Lines:** 350+
