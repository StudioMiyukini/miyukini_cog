<!-- @id cert.knowledge.docker -->
<!-- @do define_required_knowledge_for_docker -->
<!-- @role certification_prep -->
<!-- @layer reference -->
<!-- @human Knowledge requirements for Docker -->

# Docker - Knowledge Requirements

> TL;DR: This file lists the concrete knowledge and evidence expected before claiming readiness for this certification.

**Owner MIP** : Hugo
**Validation type** : Competence pratique

## Required Knowledge Blocks

| Block | Knowledge to master | Expected proof |
|-------|---------------------|----------------|
| M1 | Image build strategy and multi stage optimization | Create minimal reproducible images |
| M2 | Runtime isolation, networking and volumes | Control runtime behavior and persistence |
| M3 | Compose orchestration and health management | Operate multi service local and preprod stacks |
| M4 | Container security scanning and hardening | Reduce vulnerabilities and privilege exposure |

## Pre-certification Validation

- [ ] Deliver hardened Dockerfile with non root runtime
- [ ] Run compose stack with healthchecks
- [ ] Publish vulnerability scan report and remediations

## MIP Integration

Use this file as the detailed path. Keep REFERENCE.md short for selective loading and point deep work to this module.

## Sources officielles
Voir ../sources/hugo.md (maj 2026-03-05).

