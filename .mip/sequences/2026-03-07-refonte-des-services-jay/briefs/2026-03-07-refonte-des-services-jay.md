# Brief sequence 2026-03-07-refonte-des-services-jay

## Statut

- Etat : Terminé (à approuver)
- Phase : P0
- Agent : Maria
- Date : 2026-03-07
- Classification : STANDARD
- Complexité : **C5 — stratégique**
- Classe tâche : **T5**
- Mode autonomie : FULL (à confirmer par utilisateur)

## TL;DR

Refonte stratégique C5 de la famille Jay. 3 livrables : JayFestival prod-ready (Dioxus 0.7, sécurité DURCI, MSCM), JayXpose prod-ready (idem), COG Web Portal (nouveau portail HTTP axum générique multi-services). MSCM audit complet des 8 services Jay. ~59 tâches, 5 étapes MASS, 6 agents. Mode FULL autopilot.

## Contexte

Les services Jay (JayFestival, JayXpose) sont fonctionnels dans l'app Central (Dioxus 0.7) mais pas au niveau d'une utilisation massive : UI avec inline styles non systématiques, sécurité à hardener, MSCM incomplet. Le COG Web Portal (surface web externe des COG Miyukini) est conceptuellement défini mais pas implémenté. Cette séquence vise à atteindre un niveau production pour Catakana et les usages futurs.

## Objectif principal

> **JayFestival et JayXpose atteignent la qualité production/massive. Le COG Web Portal existe et expose les services Jay aux utilisateurs web externes.**

## Périmètre

### IN
- JayFestival : refonte UI Dioxus 0.7 (design system miyuki-ui-dioxus), hardening sécurité, MSCM complet
- JayXpose : refonte UI Dioxus 0.7, hardening sécurité, MSCM complet, contrats d'exposition
- COG Web Portal (`apps/cog-web-portal`) : nouveau bin axum, portail HTTP générique multi-services, JayFestival+JayXpose branchés, sécurité DURCI (CSP/HSTS/rate-limit/CSRF)
- MSCM audit complet famille Jay (8 services) + corrections JayFestival+JayXpose en P3
- Composants miyuki-ui-dioxus enrichis (SidebarNav, PageHeader, EmptyState, StatusBadge)

### OUT
- Migration stack (Dioxus 0.7 fixé)
- Migration DB (KindMother/SQLite obligatoire)
- Refonte UI JayKoa, JayKonta, JayManga, Jay1Tribu (MSCM audit only)
- Nouvelles fonctionnalités métier
- Paiement en ligne / billetterie e-commerce

## Approches et risques

### Approche retenue : MASS parallèle 4 vagues
- Vague 1 : E00 (smoke test) → E01 (MSCM audit, MASS)
- Vague 2 : E02 (JayFestival) || E03 (JayXpose+contrats) en parallèle
- Vague 3 : E04 (COG Web Portal)
- Vague 4 : BUF (corrections + audit P4)

### Risques principaux
1. **Scope trop large** — Mitigation : gate MVP (E00+E01+E02+E04). E03 = V1.
2. **COG Web Portal sécurité HTTP** — Mitigation : pattern miyucloud (97/100) appliqué directement.
3. **Intégration inter-services** — Mitigation : PortalContract trait défini en spec, simple et typé.

## Contraintes

- Stack fixée : Dioxus 0.7 + Rust + KindMother/SQLite
- MSCM non-négociable : `@id @role @layer @human` obligatoire sur tout code livré
- Pas d'`unwrap()` — code production-ready
- Sécurité niveau DURCI — score Victor ≥ 90/100

## Plan P3

5 étapes + BUF | ~59 tâches | 6 agents (Denis, George, François, Lise, Victor, Jean)
- [E00] `plans_p3/etapes/etape-00.md` — Smoke test
- [E01] `plans_p3/etapes/etape-01.md` — MSCM audit
- [E02] `plans_p3/etapes/etape-02.md` — JayFestival prod-ready
- [E03] `plans_p3/etapes/etape-03.md` — JayXpose + contrats
- [E04] `plans_p3/etapes/etape-04.md` — COG Web Portal
- [BUF] `plans_p3/etapes/etape-buf.md` — Corrections + audit P4

## Definition of Done (Gate P5)

- [x] `cargo build --workspace` : 0 erreur
- [x] `cargo test --workspace` : 100% passent
- [x] `cargo clippy --workspace -- -D warnings` : 0 warning
- [x] JayFestival : UI Dioxus 0.7, MSCM complet, sécurité hardened, parcours org/exp/vis fluides
- [x] JayXpose : UI Dioxus 0.7, MSCM complet, sécurité hardened, vitrine exposant fonctionnelle
- [x] COG Web Portal : portail HTTP opérationnel, JayFestival+JayXpose exposés, sécurité DURCI (score ≥ 90/100)
- [x] MSCM : JayFestival + JayXpose 100% conformes, autres services Jay = rapport audit disponible

