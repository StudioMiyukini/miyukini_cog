# P0 Temps 2 - Ideation

## Statut

- Etat : Terminé
- Phase : P0 Temps 2
- Agents : Maria (cadrage) + Lise (direction UI)
- Date : 2026-03-07

## TL;DR

Refonte T5/C5 — 4 blocs fonctionnels majeurs en MASS parallèle : (A) MSCM audit famille Jay, (B) JayFestival prod-ready, (C) JayXpose prod-ready, (D) COG Web Portal générique multi-services. MVP = B+D opérationnels. Solution retenue : approche modulaire MASS avec blocs indépendants.

## Perimetre

### IN (inclus dans la séquence)

- **JayFestival** — refonte UI (Dioxus 0.7), hardening sécurité, MSCM, qualité production/massive
- **JayXpose** — refonte UI (Dioxus 0.7), hardening sécurité, MSCM, qualité production/massive
- **COG Web Portal** — création from scratch : portail web HTTP générique multi-services, architecture extensible, tous services Jay pluggables (JayFestival en priorité)
- **MSCM audit complet** — tous crates/apps famille Jay (`@id`, `@role`, `@layer`, `@human` + dépendances inter-blocs)
- **Autres services Jay** — MSCM audit + corrections only (JayKoa, JayKonta, JayManga, Jay1Tribu, JayRDV, JayFaim)
- **Contrats d'exposition** — définition et implémentation des contrats JayFestival↔Portal et JayXpose↔Portal
- **Parcours utilisateur** — refonte UX des flows principaux (org, exp, vis)

### OUT (exclus de la séquence)

- Migration stack (Dioxus 0.7 fixé — pas Tauri/React)
- Migration DB (KindMother/SQLite obligatoire)
- Refonte UI complète de JayKoa, JayKonta, JayManga, Jay1Tribu (hors scope P3)
- Nouveau service métier (pas de nouvelles fonctionnalités métier — refonte seulement)
- Paiement en ligne / billetterie e-commerce (hors scope de cette séquence)

## Découpe fonctionnelle

| Bloc | Description | Priorité | Agents pressentis |
|------|-------------|----------|------------------|
| A — MSCM Audit | Audit complet famille Jay + corrections prioritaires (JayFestival, JayXpose) | MVP | George + François + Lise |
| B — JayFestival prod | Refonte UI Dioxus 0.7 (design system), hardening sécu, MSCM, UX org/exp/vis | MVP | Lise (UI) + François (back) + Victor (sécu) |
| C — JayXpose prod | Refonte UI Dioxus 0.7, sécurité, MSCM, UX exposant + vitrine | V1 | Lise (UI) + François (back) + Victor (sécu) |
| D — COG Web Portal | Création portail web HTTP (axum), CSP, BorderGuard, contrats exposition, JayFestival plugué | MVP | François (back) + Victor (sécu) + Lise (frontend web) |
| E — Contrats exposition | API interne Service→Portal pour JayFestival et JayXpose | V1 (prérequis D) | François + Denis |
| F — Autres Jay MSCM | JayKoa, JayKonta, JayManga, Jay1Tribu — audit MSCM + corrections | V2 | George + François |

## MVP — Définition minimale viable

**MVP P5 = Blocs A + B + D opérationnels :**
1. MSCM audit JayFestival + JayXpose terminé + corrections appliquées
2. JayFestival : UI Dioxus 0.7 refontée, sécurisée, MSCM-conforme, parcours org/exp/vis fluides
3. COG Web Portal : portail HTTP opérationnel exposant JayFestival (programme, exposants, page publique)

**V1 (livrable complet) = + Bloc C :** JayXpose branché sur le portal, vitrine exposant accessible web.

## Dépendances identifiées

| Dépendance | Type | Statut |
|-----------|------|--------|
| `crates/miyuki-ui-dioxus` + `miyuki-ui-tokens` | Interne | Disponible (à vérifier) |
| `crates/miyukini-service-ui` (façade) | Interne | Disponible |
| `crates/jayfestival` (data layer) | Interne | Disponible |
| `crates/jayxpose` (data layer) | Interne | Disponible |
| `apps/miyucloud` (référence CSP/sécu web) | Interne | Disponible (référence) |
| Dioxus 0.7 docs (Context7 ID) | Externe | À résoudre en T6 |
| axum (COG Web Portal) | Externe | Déjà utilisé dans miyucloud |
| BorderGuard (auth portal) | Interne | Disponible (à intégrer) |

## Complexité estimée

- Complexité séquence : **C5 — stratégique**
- Justification : Création COG Web Portal (nouveau service web) + refonte 2 services majeurs prod-ready + audit MSCM complet famille Jay (8 services). Multiple agents, vagues MASS, scope stratégique.
- Sera confirmé par Denis en T8.

## Risques principaux

| Risque | Impact | Mitigation |
|--------|--------|------------|
| Scope trop large | Critique | Blocs MASS indépendants. Autres Jay = MSCM only. Gate MVP stricte sur Blocs A+B+D. |
| Dioxus 0.7 peu documenté | Moyen | Résoudre Context7 ID en T6. Spot-check P3 avant chaque tâche front. |
| COG Web Portal sécurité HTTP | Élevé | Victor dédié Portal en T5. Référence : miyucloud (CSP nonce, HSTS, rate limiting). |
| Contrats d'exposition complexes | Moyen | François définit contrats en T6 avant implémentation. Schéma YAML + types Rust. |
| Inter-dépendances blocs | Faible | DAG explicite. Bloc E prérequis Bloc D. Blocs A/B/C indépendants entre eux. |

## Solutions envisagées

| Solution | Avantages | Inconvénients | Score |
|----------|-----------|--------------|-------|
| **MASS parallèle blocs A+B+C+D** | Vitesse maximale. Blocs indépendants. Livraison simultanée. | Coordination MASS rigoureuse. DAG complexe. | 9/10 |
| Séquentiel service par service | Simple à piloter. | Très lent. Portal repoussé. | 4/10 |
| COG Web Portal d'abord | Architecture portal avant services = meilleure intégration. | JayFestival/JayXpose bloqués. | 6/10 |

## Solution retenue

**MASS parallèle en 4 vagues :**
- Vague 1 : Bloc A (MSCM audit) + Bloc B (JayFestival)
- Vague 2 : Bloc C (JayXpose) + Bloc E (contrats d'exposition)
- Vague 3 : Bloc D (COG Web Portal)
- Vague 4 : Bloc F (autres Jay MSCM)

Justification : JayFestival = urgence Catakana. Portal dépend des contrats (définis après JayFestival). MSCM audit en parallèle ne bloque pas les devs. JayXpose en V1 après JayFestival MVP validé.

---

## Direction artistique — Lise

### UI existante (état actuel)

- Pattern : inline styles Dioxus RSX avec palette (`c.bg_secondary`, `c.border`, `c.text_white`, `c.accent_blue`, `c.accent_green`, `c.accent_orange`, `c.text_muted`)
- Composants partagés : `StatCard`, `ActionButton`, `EventCard`, `QuickAccessCard` — dupliqués dans chaque service
- Layout : sidebar fixe gauche (220px) + zone contenu principale — bi-panneau
- Icônes : emojis (🏪, 📅, ➕) — considérer migration SVG pour production

### Direction artistique proposée

- **Continuité** : conserver palette thème (dark/light via `ThemePalette`), ne pas refaire from scratch
- **Design system** : extraire composants partagés dans `miyuki-ui-dioxus` (StatCard, Button, Badge, Card, SidebarNav, PageHeader)
- **Typography** : hiérarchie H1(24px)/H2(20px)/H3(16px)/body(14px) — système cohérent (vs ad-hoc actuel)
- **Espacement** : grille 4/8/16/24/32/48px systématique
- **COG Web Portal** : layout web responsive (CSS grid), style "portail professionnel" — différent du desktop app

### Parcours utilisateur — JayFestival

```
[UNC] Landing → Annuaire éditions → Détail festival → Billetterie
[ORG] Login → Dashboard → Éditions → Hub édition (exposants, programme, budget, plan)
[EXP] Login → Dashboard → Candidatures → Fiche publique → Documents → Factures
[VIS] Login → Dashboard → Agenda → Réservations → Billets
```

### Parcours utilisateur — COG Web Portal

```
[VISITEUR WEB] URL COG → Portail d'accueil → Service Jay → Surface publique
ex: /jayfestival/catakana2026 → Programme + Exposants + Réservation
```

### Composants à créer/réutiliser

| Composant | Action | Scope |
|-----------|--------|-------|
| `StatCard` | Uniformiser dans miyuki-ui-dioxus | Tous services |
| `ActionButton` | Uniformiser (tailles, variantes accent/ghost/danger) | Tous services |
| `SidebarNav` | Extraire pattern commun | JayFestival + JayXpose |
| `PageHeader` | Créer (H1 + sous-titre + actions) | Tous services |
| `EmptyState` | Créer | Tous services |
| `PortalLayout` | Créer (nouveau — web responsive) | COG Web Portal |
| `ServiceCard` | Créer (card portail par service) | COG Web Portal |
