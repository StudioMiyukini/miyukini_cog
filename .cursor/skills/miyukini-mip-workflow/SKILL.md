# Skill: MIP v2 — Miyukini Implementation Protocol

## Quand utiliser ce skill

Utiliser ce skill pour **toute demande de developpement** impliquant du code, une nouvelle fonctionnalite, un fix, un refactor, ou un nouveau crate/service. Le protocole MIP v2 orchestre l'equipe et structure le travail.

---

## Etape 1 — Classification (OBLIGATOIRE)

Avant toute action, classer la demande :

| Classe | Critere | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 |
| **T2** | Fix cible, 1-3 fichiers, bug connu | P2 → P3 → P5 |
| **T3** | Feature moderee, 3-10 fichiers | P0 → P1 → P2 → P3 → P5 → P6 |
| **T4** | Feature majeure, 10+ fichiers, multi-crate | P0 → P1 → P2 → P3 → P4 → P5 → P6 |
| **T5** | Chantier strategique, nouveau crate/app | P0 → P1 → P2 → P3 → P4 → P5 → P6 |

**Regle** : En cas de doute, classer UN CRAN AU-DESSUS.

**Qui classifie** : Maria (Chef de Projet). En son absence, l'utilisateur ou le coordinateur Claude.

---

## Etape 2 — Routing des phases

### P0 — Cadrage, Brainstorming & Analyse (T3+)

**Agents** : Maria (lead) + Lise (direction visuelle, T3+) + Fabrice (analyse concurrentielle, T4-T5)

P0 est la phase la plus importante : elle determine la direction de tout le travail. Aucun code ne sera ecrit avant la fin de P0. Le brainstorming est **structure en 4 temps**.

#### Temps 1 — Exploration (Maria)

Maria reformule la demande et creuse le contexte :

1. **Reformuler** la demande utilisateur en termes precis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** : lire les fichiers concernes (Glob, Grep, Read) pour comprendre l'etat actuel
4. **Poser des questions** de clarification a l'utilisateur (minimum 2-3 questions ciblees)
5. **Identifier les contraintes** : Lois d'Autonomie applicables, stack technique, compatibilite existante

**Hard gate** : NE PAS passer au temps 2 sans reponses de l'utilisateur.

#### Temps 2 — Ideation (Maria + Lise en parallele)

Deux explorations paralleles :

**Maria** — Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. Definir le **perimetre** : IN / OUT explicite
3. Identifier les **risques** et leurs mitigations
4. Proposer **2-3 approches** techniques differentes avec pros/cons

**Lise** (T3+ si la tache a un aspect front/UI) — Vision graphique :
1. Analyser l'**UI existante** (theme, composants, patterns visuels en place)
2. Proposer la **direction artistique** : style, ton, inspirations visuelles
3. Decrire le **parcours utilisateur** (flux ecran par ecran, interactions)
4. Identifier les **composants** a creer/reutiliser (atomic design : atomes, molecules, organismes)
5. Si pertinent, referencer des **inspirations visuelles** (apps concurrentes, design systems)

#### Temps 3 — Analyse concurrentielle (Fabrice, T4-T5 seulement)

**Fabrice** (lance en parallele du temps 2 pour T4-T5) :
1. Identifier les **produits/services concurrents**
2. Analyser **forces et faiblesses** de chaque concurrent
3. Identifier la **cible utilisateur** et ses attentes
4. Lister les **fonctionnalites differenciantes** a envisager
5. Detecter les **points de friction** des concurrents

#### Temps 4 — Synthese & Brief (Maria)

Maria compile tout dans le brief :

1. **Fusionner** les contributions de tous les agents (Maria + Lise + Fabrice)
2. **Rediger le brief structure** avec toutes les sections
3. **Presenter les approches** avec la recommandation de l'equipe
4. Artefact : `.mip/briefs/YYYY-MM-DD-<slug>.md`

**Template du brief** :

```markdown
# Brief: <titre>

## Metadata
- Classe: T3/T4/T5
- Date: YYYY-MM-DD
- Demandeur: utilisateur

## Contexte
[Pourquoi cette demande, quel probleme elle resout]

## Objectifs
- Objectif principal: ...
- Objectifs secondaires: ...
- Criteres de succes mesurables: ...

## Perimetre
### Inclus
- [Fonctionnalites IN]
### Exclus
- [Fonctionnalites OUT — explicitement rejetees]

## Approches proposees
### Approche A — [nom] (RECOMMANDEE)
- Description: ...
- Pros: ...
- Cons: ...
- Effort: ...

### Approche B — [nom]
- Description: ...
- Pros: ...
- Cons: ...

## Direction visuelle (par Lise)
- Style/ton: ...
- Composants identifies: [atomes, molecules, organismes]
- Parcours utilisateur: [flux ecran par ecran]
- Inspirations: ...

## Analyse concurrentielle (par Fabrice, T4-T5)
- Concurrents: ...
- Differenciateurs: ...
- Cible utilisateur: ...

## Contraintes
- Lois d'Autonomie: LOI-x applicables
- Stack: ...
- Compatibilite: ...

## Risques
| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|------------|
| ... | ... | ... | ... |

## Decision
APPROUVE / REJETE / MODIFIE
```

**Quality Gate P0** : Utilisateur approuve le brief ET choisit l'approche.

**Hard gate** : AUCUN passage en P1 sans brief approuve.

---

### P1 — Specification Technique (T3+)

**Agent** : Denis

1. Explorer le code existant (Glob, Grep, Read)
2. Identifier les fichiers a modifier/creer avec numeros de ligne
3. Definir les types, traits, API (signatures completes)
4. Verifier la conformite architecturale :
   - [ ] Lois d'Autonomie respectees (LOI-1 a LOI-8)
   - [ ] `unsafe_code = "forbid"` dans tout nouveau Cargo.toml
   - [ ] Strate correcte dans la pyramide COG
   - [ ] Annotations MSCM planifiees (@id, @do, @role, @layer)
5. Artefact : `.mip/specs/YYYY-MM-DD-<slug>.md`

**Quality Gate P1** : Denis valide la faisabilite + utilisateur approuve l'approche.

---

### P2 — Plan d'Execution (T2+)

**Agent** : Denis

Decomposer la spec en taches atomiques (2-5 minutes chacune).

**Chaque tache DOIT contenir** :
- Agent assigne (Francois ou Lise)
- Fichier(s) exact(s) a modifier (chemin complet)
- Code complet a ecrire (pas de "ajouter de la validation")
- Commande de test : `cargo test -p {crate} -- {pattern}`
- Output attendu : `"test xxx ... ok"`
- Message de commit : `"type(scope): description"`

**Principe** : Presumer que l'executant n'a AUCUN contexte projet.

Artefact : `.mip/plans/YYYY-MM-DD-<slug>.md`

**Quality Gate P2** : Denis valide le plan complet.

---

### P3 — Implementation (toutes classes)

**Agents** : Francois (back-end) + Lise (front-end) en PARALLELE

**Execution par subagent frais** : Chaque tache est executee par un subagent frais pour eviter la pollution de contexte.

**Cycle TDD par tache** :
1. **RED** — Ecrire le test qui echoue
2. **GREEN** — Ecrire le code minimal pour que le test passe
3. **REFACTOR** — Nettoyer si necessaire
4. **VERIFY** — `cargo test -p {crate}` passe
5. **LINT** — `cargo clippy -p {crate} -- -D warnings` propre
6. **COMMIT** — Commit atomique avec message conventionnel

**Parallelisme** : Francois et Lise travaillent simultanement quand leurs taches sont independantes. Les taches avec dependances sont sequencees par Denis.

**Quality Gate P3** : Chaque tache passe test + clippy.

---

### P4 — Integration & Audit (T4-T5)

**Agents** : Denis + George

**Denis** :
1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo clippy --workspace -- -D warnings`
4. Verifier l'integration back + front

**George** — Audit de conformite :
- [ ] Build workspace OK
- [ ] Tests workspace OK
- [ ] Clippy propre
- [ ] Pas de `unwrap()` en production (hors `#[cfg(test)]`)
- [ ] Pas d'URL hardcodees
- [ ] Pas de donnees sensibles en clair
- [ ] Annotations MSCM presentes sur les nouveaux fichiers
- [ ] Lois d'Autonomie respectees
- [ ] Parcours utilisateur coherent (si UI)

Artefact : `.mip/audits/YYYY-MM-DD-<slug>.md`

**Quality Gate P4** : George valide — 0 defaut BLOQUANT.

---

### P5 — Livraison (toutes classes)

**Agent** : Denis

1. Commit final structure (message conventionnel)
2. Tag si release
3. Presentation a l'utilisateur

**Quality Gate P5** : Utilisateur confirme la livraison.

---

### P6 — Archivage & Capitalisation (T3+)

**Agent** : Arianne

1. Archiver les artefacts MIP (brief, spec, plan, audit)
2. Extraire les apprentissages :
   - Patterns confirmes → `memory/mip-decisions.md`
   - Erreurs a eviter → `memory/mip-antipatterns.md`
   - Lecons par chantier → `memory/mip-lessons.md`
   - Competences par agent → `memory/team-skills-audit.md`
3. Mettre a jour `memory/MEMORY.md` (index, max 200 lignes)

---

## Regles NON NEGOCIABLES

1. **Classification avant action** — Aucun code sans classification T1-T5
2. **Spec avant code** (T3+) — Pas d'implementation sans spec Denis
3. **Plan avant execution** (T2+) — Pas d'implementation sans plan atomique
4. **TDD obligatoire** — RED-GREEN-REFACTOR, pas d'exception
5. **Subagent frais par tache** — Eviter la pollution de contexte
6. **Gates non-bypassables** — Chaque gate doit etre explicitement validee
7. **Artefacts obligatoires** — Chaque phase produit son artefact dans `.mip/`
8. **Clippy propre** — `cargo clippy -p {crate} -- -D warnings` apres chaque tache
9. **Pas de unwrap() en prod** — Uniquement dans `#[cfg(test)]`
10. **Archivage systematique** (T3+) — Arianne capitalise apres chaque livraison

---

## Integration SuperClaude

Ce protocole s'appuie sur les skills SuperClaude quand ils sont disponibles :

| Phase MIP | Skill SuperClaude | Usage |
|-----------|-------------------|-------|
| P0 | `brainstorming` | Maria structure le brief (4 temps : exploration → ideation → analyse → synthese) |
| P2 | `writing-plans` | Denis cree les taches atomiques |
| P3 | `subagent-driven-development` | Execution par subagent frais |
| P3 | `test-driven-development` | Cycle RED-GREEN-REFACTOR |
| P3 | `systematic-debugging` | Root cause avant tout fix |
| P4 | `verification-before-completion` | George verifie |
| P5 | `finishing-a-development-branch` | Denis finalise |

---

## Raccourcis pour taches simples

**T1 (micro-fix)** : Pas besoin de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis ecrit un mini-plan (1-3 taches), execution directe.

Le protocole est **proportionnel** : les petites taches ne sont pas alourdies.
