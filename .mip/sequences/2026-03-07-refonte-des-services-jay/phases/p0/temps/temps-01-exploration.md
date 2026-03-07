# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : En cours (questionnaire sections 1-5 à compléter)
- Phase : P0 Temps 1
- Responsable principal : Maria
- T0 : 2026-03-07

## TL;DR

Refonte stratégique (T5/C5) des services Jay : JayFestival et JayXpose (qualité production/massive), et création du COG Web Portal (surface web par COG). Uniformisation MSCM sur toute la famille Jay. Stack : Dioxus 0.7 + Rust + KindMother.

## Section 0 — Orientation (déduite depuis le premier prompt)

> Maria remplit ce tableau SEULE, depuis le premier prompt.

| Question | Réponse déduite | Confiance |
|----------|----------------|-----------|
| Pourquoi exactement ? Quel problème est résolu ? | Services Jay (JayFestival, JayXpose) insuffisants pour usage massif : UI approximative (inline styles bruts), sécurité à hardener, MSCM non uniforme. COG Web Portal manquant : aucune surface web unifiée pour exposer les services aux utilisateurs externes. | haute |
| Exemple concret d'usage attendu ? | Catakana Festival : organisateur gère éditions/exposants dans JayFestival, exposants ont leur vitrine JayXpose, visiteurs accèdent au portail web COG pour consulter et réserver — le tout à grande échelle, proprement. | haute |
| Solution existante proche dans le projet ? | `apps/central/src/services/jayfestival/` (30+ fichiers Dioxus) + `apps/central/src/services/jayxpose/`. Crates `crates/jayfestival/`, `crates/jayxpose/`. Docs `MiyukiniWebPortal/` (conceptuel uniquement, pas implémenté). | haute |
| Pour qui ? (utilisateur final, persona) | Organisateurs de festivals, exposants professionnels, visiteurs/clients. Utilisateurs web externes via COG Web Portal. | haute |
| Fonction Online / MWS requise ? | Oui — COG Web Portal = surface HTTP externe (MWS). Services Jay = online via KindMother/tokio. | haute |
| Open-source / forkable ou from scratch ? | Refonte de l'existant. Stack confirmée : Dioxus 0.7 desktop (pas Tauri+React malgré l'ancien plan migration). | haute |
| Classification estimée (T1-T5) | T5 — chantier stratégique : 3 sujets majeurs (JayFestival prod, JayXpose prod, COG Web Portal) | haute |

## Exploration codebase — Résultats

### Famille Jay — cartographie

| Service | App standalone | UI dans Central | Crate backend | Rôle métier |
|---------|---------------|-----------------|---------------|-------------|
| JayFestival | `apps/jayfestival` (Dioxus 0.7) | `apps/central/src/services/jayfestival/` (30+ fichiers) | `crates/jayfestival` (auth, data, services) | Festivals, éditions, exposants, visiteurs, réservations |
| JayXpose | `apps/jayxpose` (Dioxus 0.7) | `apps/central/src/services/jayxpose/` | `crates/jayxpose` (auth, data, governance) | Profil exposant, catalogue, vitrine, coffre-fort docs |
| JayKoa | `apps/jaykoa` | `apps/central/src/services/jaykoa/` | `crates/jaykoa` | Calendrier/agenda |
| JayKonta | `apps/jaykonta` | `apps/central/src/services/jaykonta/` | `crates/jaykonta` | Finances/comptes |
| JayManga | `apps/jaymanga` | — | `crates/jaymanga` (web portal interne) | Manga + portal |
| Jay1Tribu | `apps/jay1tribu` | `apps/central/src/services/jay1tribu/` | `crates/jay1tribu` | Communauté |
| JayRDV | — | — | `crates/jayrdv` | Rendez-vous |
| JayFaim | — | — | `crates/jayfaim` | [à découvrir] |

### COG Web Portal
- Docs : `docs/services/MiyukiniWebPortal/` (gouvernance, contrats d'exposition définis)
- Implémentation : **inexistante** (à créer de zéro)
- Principe : chaque COG expose son portail HTTP — surface web pour utilisateurs externes

### UI partagée
- `crates/miyukini-service-ui` — façade vers `miyuki-ui-dioxus` + `miyuki-ui-tokens`
- Pattern courant : inline styles avec palette (`c.bg_secondary`, `c.border`, `c.text_white`)
- Dioxus 0.7 (≠ 0.6 — Context7 ID à résoudre en T6)

### MSCM
- Obligations : `@id`, `@role`, `@layer`, `@human` sur tout bloc fonctionnel
- Checklist : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- État actuel : balisage partiel (à auditer)

## Brainstorming — Réponses questionnaire (T5 complet)

### Section 1 — COMPRENDRE

- **1.1 Problème résolu** : Services Jay (JayFestival, JayXpose) insuffisants pour usage massif. COG Web Portal absent. MSCM non uniforme. Sécurité à hardener.
- **1.2 Pourquoi maintenant** : Catakana en approche + besoin de qualité production. Les services existent mais ne sont pas production-ready.
- **1.3 Utilisateurs** : Organisateurs festivals (primaire), exposants pro (primaire), visiteurs/clients (secondaire), utilisateurs web externes via COG Web Portal.
- **1.4 Flux actuel / friction** : Central Dioxus 0.7 opérationnel mais UX approximative. Aucune surface web externe (COG Web Portal manquant). MSCM partiel.
- **1.5 Pourquoi Dioxus 0.7** : Stack fixée. Pas de migration. Dioxus 0.7 est confirmé comme stack UI définitive.

### Section 2 — CADRER

- **2.1 Contraintes** : KindMother/SQLite obligatoire (pas de migration DB). Dioxus 0.7 fixé. MSCM non-négociable sur tout code livré.
- **2.2 Périmètre** :
  - INCLUS : JayFestival (prod-ready), JayXpose (prod-ready), COG Web Portal (architecture générique multi-services), MSCM audit complet famille Jay (tous services), autres services Jay (dans le scope)
  - EXCLUS : Migration vers autre stack, changement de DB
- **2.3 Priorité** : (a) minimal = JayFestival prod-ready + COG Web Portal MVP 1 service ; (b) souhaité = + JayXpose prod-ready + portail multi-services ; (c) nice-to-have = audit MSCM complet tous services
- **2.4 Échéance** : Catakana Festival (date non précisée)
- **2.5 Données existantes** : Docs MiyukiniWebPortal (gouvernance, contrats d'exposition), MSCM Compliance Checklist, JayFestival/JayXpose implémentés

### Section 3 — IMAGINER

- **3.1 Approche technique** : Parallèle MASS — MSCM audit + JayFestival refonte + JayXpose refonte simultanés → COG Web Portal en dernière vague
- **3.2 Similarités dans le projet** : `apps/miyucloud` (web surface, CSP, sécurité) = référence pour COG Web Portal. `crates/miyukini-service-ui` = base UI partagée.
- **3.3 Combinaison** : COG Web Portal branché directement sur crates JayFestival/JayXpose — réutilise les data layers existants
- **3.4 Simplifications** : Scope "Autres services Jay" (JayKoa, JayKonta, etc.) = MSCM audit uniquement en P3, pas de refonte UI (trop large pour 1 séquence)
- **3.5 Concurrents** : Eventbrite (JayFestival), Shopify vitrine (JayXpose), sites portails festivals classiques (COG Web Portal)
- **3.6 HMW** : *Comment pourrions-nous faire que chaque COG Miyukini ait une surface web de qualité professionnelle sans effort de maintenance supplémentaire ?*

### Section 4 — ÉVALUER

- **4.1 Bénéfice principal** : 3 livrables opérationnels — JayFestival prod-ready + JayXpose prod-ready + COG Web Portal générique multi-services
- **4.2 Risques identifiés** : Scope potentiellement trop large (tous services Jay + audit complet MSCM + 3 livrables), COG Web Portal complexité HTTP (CSP, sécurité, rate limiting), intégration inter-services (contrats d'exposition JayFestival ↔ JayXpose ↔ Portal)
- **4.3 Complexité séquence** : C5 — stratégique (architecture + 3 services + audit complet)

## Hypothèses retenues

1. **Stack Dioxus 0.7 confirmée** — pas de migration, toute l'UI reste en Dioxus RSX
2. **COG Web Portal = architecture générique** — extensible, tous services Jay pluggables dès le départ
3. **MSCM = audit complet** famille Jay (mais corrections = scope P3 prioritaire : JayFestival + JayXpose d'abord)
4. **Parallélisation MASS** — JayFestival + JayXpose + MSCM audit en vagues parallèles, COG Web Portal en vague finale
5. **Scope protection** : "Autres services Jay" = MSCM audit only en P3, pas de refonte UI complète

## Hypothèses écartées

1. ~~Migration vers Tauri+React~~ — stack fixée Dioxus 0.7
2. ~~MSCM nouveau code seulement~~ — audit complet demandé
3. ~~COG Web Portal minimal (1 service)~~ — architecture générique multi-services souhaitée
4. ~~Séquentiel service par service~~ — parallèle MASS choisi

## Classification post-T1

- Classe tâche (T1-T5) : T5 — chantier stratégique multi-services + création COG Web Portal
- Complexité séquence estimée (C1-C5) : C5 — stratégique (production-ready pour usage massif + nouveau service web)
  > Sera confirmé en T2 et validé par Denis en T8.

