# Brief de Cadrage P0 -- MiyuCloud v2 Reprise (Securite + Onboarding + Infrastructure)

<!-- @id: mip.brief.miyucloud-v2.reprise -->
<!-- @do: brief_p0_refait_sur_base_mip_v2 -->
<!-- @role: Maria (Chef de Projet) -->
<!-- @layer: S7-Operator -->

**Classification MIP** : T4 (Feature majeure)
**Date** : 2026-03-05
**Responsable P0** : Maria
**Statut** : APPROUVE UTILISATEUR (2026-03-05)
**Mode autonomie** : FULL

---

## TL;DR

MiyuCloud v2 doit reprendre un cadrage deja etabli, mais l'execution precedente etait basee sur un MIP obsolete.
Ce brief refait conserve les objectifs metier et securite, puis les recadre dans une sequence MIP v2 propre.
Objectif principal : score securite 72 -> >95/100, 2FA TOTP complete, onboarding guide, infrastructure de deploiement.
Plan propose : 67 taches en 10 vagues, execution modulaire avec gates de validation.
Recommendation : Approche A en BIG_STEPS pour garder le controle des transitions critiques.

---

## 1. Contexte et etat des lieux

### 1.1 Pourquoi cette reprise

Une tentative precedente de cadrage/execution a suivi un flux MIP devenu obsolete dans le repo.
La reprise vise a relancer une sequence conforme MIP v2, avec artefacts ranges dans un dossier de sequence dedie.
Le contenu fonctionnel et securite reste valide, mais il doit etre re-cadre proprement avant P3.

### 1.2 Probleme a resoudre (4 axes)

1. **Cloud de confiance** : viser un niveau securite certif-ready (>95/100, OWASP, RGPD)
2. **Vitrine qualite** : MiyuCloud est une surface publique du COG, UX exigeante
3. **Ecosysteme** : integration Central, onboarding guide, monitoring de sante
4. **Usage quotidien** : deploiement Internet stable pour 5-10 utilisateurs

### 1.3 Etat du code existant (base projet)

| Metrique | Valeur |
|----------|--------|
| Lignes de code | 18 510 |
| Tests unitaires | 226 |
| Fichiers source | 64 |
| Routes API | 27 + 4 web |
| Couverture MSCM | 100% |
| Score securite George (v1) | 87/100 |
| Score securite Victor (approfondi) | 72/100 |
| Stack crypto | X25519 + ChaCha20-Poly1305 + Argon2id + HKDF |
| DB | KindMother (SQLite gouverne) |

### 1.4 Decisions v1 a conserver

| Decision | Description |
|----------|-------------|
| D1 -- P2P | Sync pair-a-pair, pas de serveur central |
| D2 -- Surface web | Exposition minimale, sandboxee, liens non predictibles |
| D3 -- Remplacement Jay1Tribu | MiyuCloud remplace Jay1Tribu pour le partage |
| D4 -- Chiffrement | E2E obligatoire, at-rest ChaCha20-Poly1305 |

---

## 2. Perimetre MVP v2

### 2.1 Inclus (IN)

**Securite (22 corrections)**
- Fix critique F-11 (timing attack comparaison token COG)
- Fix majeurs F-04 XSS, F-12 header injection, F-14 IP non loggee, F-02 timing
- Remplacement comparaisons XOR par `subtle::ConstantTimeEq`
- Validation UUID anti path traversal
- Zeroization des secrets TOTP en memoire
- Headers HTTP securite
- Purge periodique `rate_limiter`
- `trust_proxy` pour rate limiting

**Authentification 2FA TOTP**
- Setup TOTP avec QR code (RFC 6238)
- Verification code 6 chiffres
- Recovery codes (8, usage unique)
- Sessions et revocation
- 4 tables SQLite nouvelles
- 15 endpoints API nouveaux

**Onboarding guide**
- Wizard 4 etapes (premier lancement)
- Health dashboard (statuts, metriques, disque)
- Badge sante dans sidebar Central

**Infrastructure**
- Pipeline GitHub Actions (lint -> test -> security -> build)
- Service systemd sandboxe
- Caddy reverse proxy + Let's Encrypt
- Scripts `deploy.sh`, `backup.sh`, `healthcheck.sh`
- Rollback < 5 minutes

**Front-end (Central UI)**
- 6 composants nouveaux
- Palette MiyuCloud dans theme
- Drag and drop upload + progress bar
- File preview inline (image, texte, PDF)
- Toggle grille/liste

**Documentation**
- MSCM 100% nouveaux fichiers
- Documentation technique complete

### 2.2 Exclu (OUT)

- Sync P2P (deja en v1)
- Migration Jay1Tribu (projet separe)
- Multi-utilisateurs
- Quotas de stockage
- Application mobile
- Notifications push
- Versioning fichiers
- Corbeille (deja en v1)

---

## 3. Approches possibles

### Approche A -- Sprint unique T4 (RECOMMANDEE)

**Description** : executer les 67 taches en 10 vagues dans une sequence unifiee.

| Critere | Evaluation |
|---------|-----------|
| Avantages | Coherence complete, pas de dette inter-sprint |
| Risques | Charge importante, merge final plus complexe |
| Duree | 1 session longue |
| Faisabilite | Confirmee (base mature + extensions ciblees) |

### Approche B -- 2 sprints

**Description** : Sprint 1 back+secu (V0-V5), Sprint 2 front+infra+doc (V6-V9).

| Critere | Evaluation |
|---------|-----------|
| Avantages | Charge fractionnee, livrable intermediaire testable |
| Risques | Overhead de coordination |
| Duree | 2 sessions |
| Faisabilite | Bonne |

### Approche C -- 3 micro-sprints

**Description** : securite -> features -> UX/infra.

| Critere | Evaluation |
|---------|-----------|
| Avantages | Risque reduit par sprint |
| Risques | Surcout de contexte et orchestration |
| Duree | 3 sessions |
| Faisabilite | Possible, non optimale |

**Recommandation Maria** : Approche A.
**Mode recommande** : BIG_STEPS (gate P3->P4 puis P4->P5).

---

## 4. Plan retenu (10 vagues, 67 taches)

### 4.1 Vue d'ensemble

| Vague | Nom | Taches | Agents | Gate |
|-------|-----|--------|--------|------|
| V0 | Setup | 5 | Denis | Branch + deps + smoke test |
| V1 | Securite critique | 8 | Francois + Victor | Defauts critiques corriges |
| V2 | Auth TOTP + Sessions | 10 | Francois + Lise | Tests TOTP/sessions |
| V3 | Onboarding + Monitoring | 8 | Francois + Lise | Onboarding e2e + health |
| V4 | API Handlers + Routes | 8 | Francois | Endpoints fonctionnels |
| V5 | Hardening | 7 | Francois + Victor | Headers + fix securite |
| V6 | UX Polish | 8 | Lise | Build UI OK |
| V7 | Infrastructure | 6 | Hugo | CI/CD + systemd + Caddy |
| V8 | Documentation + MSCM | 3 | Denis + Francois | MSCM 100% nouveaux fichiers |
| V9 | Integration finale | 4 | Denis + George + Victor | Build/test/audit >95 |
| **Total** | | **67** | **5 agents** | |

### 4.2 DAG de dependances

```text
V0 -> V1 -> V2 -> V3 -> V4 -> V5 -> V8 -> V9
 \-> V6(partiel) ----^      \-> V7 ----^ 
```

### 4.3 Parallelisme

- V1 + V6 partiel (back/front disjoints)
- V2 back + V2 front
- V3 back + V3 front
- V5 + V7

### 4.4 Distribution agents

| Agent | Taches | Perimetre |
|-------|--------|-----------|
| Francois | 38 | Back-end securite/auth/onboarding/API |
| Lise | 15 | Front-end TOTP/sessions/onboarding/UX |
| Hugo | 6 | Infra CI/CD/systemd/Caddy/deploy |
| Denis | 6 | Setup/coordination/doc/integration |
| Victor | 2 | Spot-checks securite V1 + V9 |
| George | - | Audit conformite en V9 |

### 4.5 Livrables cles

| Livrable | Quantite |
|----------|----------|
| Fichiers a creer | 19 |
| Fichiers a modifier | 22 |
| Lignes nouvelles | 2 300 |
| Tests nouveaux | 75 |
| Endpoints API nouveaux | 15 |
| Tables SQLite nouvelles | 4 |
| Dependencies Cargo nouvelles | 4 |

---

## 5. Risques et mitigations

| # | Risque | Probabilite | Impact | Mitigation |
|---|--------|-------------|--------|------------|
| R1 | Regressions sur tests existants | Moyen | Eleve | TDD strict + tests frequents |
| R2 | Score securite final <95 | Faible | Eleve | Spot-checks Victor + buffer corrections |
| R3 | Conflits merge back/front | Moyen | Moyen | Decoupage fichiers + commits atomiques |
| R4 | Incompatibilite `totp-rs` | Faible | Moyen | Versions epinglees + verification docs |
| R5 | Budget tokens depasse | Moyen | Faible | BIG_STEPS + gates + split possible |

---

## 6. Metriques cibles

| Metrique | Valeur actuelle | Cible v2 |
|----------|----------------|----------|
| Score securite Victor | 72/100 | >95/100 |
| Score audit George | 87/100 | >90/100 |
| Tests unitaires | 226 | 301 |
| Clippy warnings | 0 | 0 |
| Couverture MSCM | 100% | 100% |
| Endpoints API | 31 (27+4 web) | 46 (42+4 web) |
| Tables SQLite | existantes | +4 |
| Dependencies Cargo | 15 | +4 |

---

## 7. Artefacts P0 de cette reprise

| Temps | Artefact | Chemin |
|-------|----------|--------|
| T10 | Brief refait (ce document) | `.mip/sequences/2026-03-05-miyucloud-v2-reprise/briefs/2026-03-05-miyucloud-v2-reprise.md` |
| T10 | Trace P0 | `.mip/sequences/2026-03-05-miyucloud-v2-reprise/phases/p0-trace.md` |
| T10 | Metriques sequence | `.mip/sequences/2026-03-05-miyucloud-v2-reprise/metrics/2026-03-05-miyucloud-v2-reprise.json` |

---

## 8. Decision de cadrage

- **Reprise validee** : oui, en sequence dediee MIP v2
- **Classification maintenue** : T4
- **Approche recommandee** : A (sprint unique)
- **Mode autonomie recommande** : BIG_STEPS

---

## 9. Prochaines etapes

1. Approbation utilisateur du brief
2. Choix final du mode autonomie (FULL, BIG_STEPS, GUIDED)
3. Creation branche `feat/miyucloud-v2-reprise`
4. Demarrage P3 V0

---

*Brief redige par Maria -- Chef de Projet, Miyukini AI Studio*
*2026-03-05 -- P0 Temps 10 (Reprise)*


