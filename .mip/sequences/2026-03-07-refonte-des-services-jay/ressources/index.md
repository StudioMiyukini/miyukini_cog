# Index des ressources - 2026-03-07-refonte-des-services-jay

> Fichier squelette rempli par Maria lors de la creation de la structure de sequence.
> Liste des ressources necessaires: docs, certs, libs, IDs Context7, securite.

## Documentation

- `docs/services/MiyukiniWebPortal/` — gouvernance COG Web Portal, contrats d'exposition
- `docs/services/JayFestival/` — docs fondateurs, audit, spec UI, bornage implémentation
- `docs/services/JayXpose/` — docs fondateurs, architecture, API reference
- `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md` — checklist MSCM
- `apps/miyucloud/src/security_headers.rs` — référence CSP/HSTS (score 97/100)
- `.mip/memory/patterns-and-lessons.md` — anti-patterns à charger avant chaque sprint

## Certifications / Referentiels

- OWASP Top 10 — applicable (XSS, SQLi, CSRF, path traversal, broken auth)
- RGPD minimal — IP logs hachés SHA-256, pas de PII brut en log
- MSCM Compliance Checklist — contractuel obligatoire sur tout code livré

## Securite (P0 T5 -> P4)

### RPS (Rapport Préliminaire de Sécurité)
- Niveau requis : **DURCI** (COG Web Portal + Jay auth)
- Surfaces CRIT : Portal HTTP externe (formulaires, routes, uploads)
- Surfaces HIGH : Auth JayFestival/JayXpose, Documents JayXpose
- Normes : OWASP Top 10 + RGPD minimal
- Référence implémentation : `apps/miyucloud/src/security_headers.rs`

### Volet GPI sécurité
- Implémentation CSP nonce Portal : Étape 4 P3 (prérequis routes OK)
- Rate limiting Portal : Étape 4 P3 (tower-http RateLimitLayer)
- Hardening auth Jay : Étape 2-3 P3 (en parallèle refonte UI)
- Audit P4 : PASS-0 (traversal + auth) + PASS-01 (headers + CVE) + RAS score /100

## Librairies / Paquets

| Lib | Version | Usage | Statut |
|-----|---------|-------|--------|
| dioxus | 0.7 | UI Jay services | Confirmé |
| axum | 0.7 | COG Web Portal HTTP | Confirmé |
| tower-http | 0.5 | Middleware Portal | À ajouter |
| rusqlite | 0.32 | KindMother DB | Déjà présent |
| sha2 | 0.10 | Hachage IP/tokens | Déjà présent |
| uuid | 1.x | IDs entités | Déjà présent |
| chrono | 0.4 | Dates | Déjà présent |
| thiserror | 2 | Erreurs typées | Déjà présent |

## IDs Context7 (vérification docs)

| Lib | Context7 ID | Score | Usage |
|-----|-------------|-------|-------|
| **Dioxus 0.7** | `/dioxuslabs/dioxus/v0.7.2` | 73 | Composants, RSX, signals |
| **Dioxus 0.7 full** | `/llmstxt/dioxuslabs_learn_0_7_llms-full_txt` | 71 | Patterns avancés |
| **Dioxus learn** | `/websites/dioxuslabs_learn` | 90 | Référence générale |
| **Dioxus Components** | `/dioxuslabs/components` | 62 | Composants ARIA |
| **axum** | `/tokio-rs/axum/axum_v0_7_9` | — | Routing, extractors, middleware |

## Audit securite P4 (PASS -> RAS)

- PASS-0 : plan d'audit securite
- PASS-XX : rapports par tache d'audit (1 tache = 1 agent)
- RAS : compilation finale + score securite /100

Regle de rebouclage:
- Breche critique ou score < 60/100 -> retour cycle MIP (P0 Temps 1) avec RAS

