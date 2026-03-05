<!-- @id cert.knowledge.terraform -->
<!-- @do define_required_knowledge_for_terraform -->
<!-- @role certification_prep -->
<!-- @layer reference -->
<!-- @human Knowledge requirements for Terraform -->

# Terraform - Knowledge Requirements

> TL;DR: This file lists the concrete knowledge and evidence expected before claiming readiness for this certification.

**Owner MIP** : Hugo
**Validation type** : Examen individuel

## Required Knowledge Blocks

| Block | Knowledge to master | Expected proof |
|-------|---------------------|----------------|
| M1 | HCL syntax, modules, variables and outputs | Build reusable IaC modules |
| M2 | State management and backend locking | Prevent state corruption and race conditions |
| M3 | Plan apply workflow, import and drift handling | Operate safe infrastructure changes |
| M4 | Policy controls, testing and security integrations | Add guardrails to deployment pipelines |

## Pre-certification Validation

- [ ] Publish reusable module with versioning
- [ ] Set remote backend with locking and access control
- [ ] Run CI pipeline with validate and plan gates

## MIP Integration

Use this file as the detailed path. Keep REFERENCE.md short for selective loading and point deep work to this module.

## Sources officielles
Voir ../sources/hugo.md (maj 2026-03-05).

