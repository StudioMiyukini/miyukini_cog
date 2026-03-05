<!-- @id cert.knowledge.openapi -->
<!-- @do define_required_knowledge_for_openapi -->
<!-- @role certification_prep -->
<!-- @layer reference -->
<!-- @human Knowledge requirements for OpenAPI 3.1 -->

# OpenAPI 3.1 - Knowledge Requirements

> TL;DR: This file lists the concrete knowledge and evidence expected before claiming readiness for this certification.

**Owner MIP** : Francois
**Validation type** : Conformite de specification

## Required Knowledge Blocks

| Block | Knowledge to master | Expected proof |
|-------|---------------------|----------------|
| M1 | OAS structure: paths, operations, components and JSON Schema | Build reusable schemas with refs |
| M2 | HTTP semantics and status code discipline | Use idempotence and error semantics correctly |
| M3 | Security schemes and contract level error models | Define auth flows and standard error payloads |
| M4 | Versioning, examples, and contract testing | Keep spec and implementation synchronized |

## Pre-certification Validation

- [ ] Pass spec linting and validation without blocking issues
- [ ] Generate client or server stubs from spec
- [ ] Run contract tests against implementation

## MIP Integration

Use this file as the detailed path. Keep REFERENCE.md short for selective loading and point deep work to this module.

## Sources officielles
Voir ../sources/francois.md (maj 2026-03-05).

