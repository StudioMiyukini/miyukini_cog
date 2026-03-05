<!-- @id cert.francois.istqb -->
<!-- @do provide_istqb_reference_knowledge -->
<!-- @role testing -->
<!-- @layer reference -->
<!-- @human Referentiel ISTQB Foundation pour Francois -->

# ISTQB Foundation v4.0 â€” Francois

> **TL;DR** Test logiciel : niveaux, types, techniques (BVA, Decision Table, State Transition). TDD P3, vÃ©rif P4.

**IdentitÃ©** : ISTQB | Volontaire | Ã€ vie (Foundation)

## 7 principes | Niveaux | Techniques

| # | Principe | Implication |
|---|----------|-------------|
| 1 | Test = prÃ©sence dÃ©fauts | Ne prouve pas l'absence |
| 2 | Exhaustif impossible | Prioriser risque/criticitÃ© |
| 3 | Tester tÃ´t (shift-left) | DÃ¨s conception |
| 5 | Paradoxe pesticide | Varier les tests |

| Niveau | Scope | Responsable |
|--------|-------|-------------|
| Unit | Crate, module | FranÃ§ois |
| Integration | Interfaces crates | FranÃ§ois+Denis |
| System | SystÃ¨me complet | Denis+George |
| Acceptance | Validation mÃ©tier | P5 utilisateur |

| Technique | Usage Miyukini |
|-----------|----------------|
| BVA | Limites numÃ©riques, tailles |
| Decision Table | Loot, combat, permissions |
| State Transition | FSM MGE, workflows UI |

## Gestion dÃ©fauts

SeveritÃ©: Critique/Majeur/Mineur. PrioritÃ©: Urgenteâ†’Basse. Statut: Nouveauâ†’RÃ©soluâ†’FermÃ©.

## Checklist

- [ ] StratÃ©gie P0 (niveaux, couverture cible)
- [ ] Unit: Ã©quivalence + BVA par fonction publique
- [ ] Integration: crates, API, serialization
- [ ] Regression: `cargo test` avant push
- [ ] Report P4: couverture, dÃ©fauts, verdict

## Anti-patterns

| Erreur | Correction |
|--------|------------|
| Happy path seul | BVA, cas erreur |
| Tests fragiles | IndÃ©pendants, fixtures |
| Pas regression post-fix | Test obligatoire |
| Tests NF ignorÃ©s | Benchmarks + cargo audit |

## Miyukini

Unit=`#[test]` | Integration=`cargo test -p` | Acceptance=P5 | Couverture 80% unit, 60% int.
## Parcours obtention
Voir KNOWLEDGE.md pour les connaissances requises et les preuves de maitrise.

