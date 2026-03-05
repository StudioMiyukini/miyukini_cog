<!-- @id cert.hugo.docker -->
<!-- @do provide_docker_reference_knowledge -->
<!-- @role containerization -->
<!-- @layer reference -->
<!-- @human Referentiel Docker pour Hugo -->

# Docker â€” Hugo

> **TL;DR** Conteneurisation. Dockerfile multi-stage, Compose, volumes. Images slim, non-root, HEALTHCHECK.

**IdentitÃ©** : Docker Inc. | Standard DevOps | Permanent

## Dockerfile | Compose | SÃ©curitÃ©

| Pratique | DÃ©tail |
|----------|--------|
| Multi-stage | rust build â†’ debian:slim runtime |
| .dockerignore | target/, .git/, .mip/ |
| USER non-root | useradd + USER |
| HEALTHCHECK | curl -f localhost/health |
| Volumes | Named (prod), tmpfs (secrets) |

| Compose | Usage |
|---------|-------|
| depends_on | condition: service_healthy |
| volumes | Named pour KindMother, miyucloud |
| networks | Bridge par groupe |
| deploy | limits CPU/memory |

## Checklist

- [ ] Multi-stage par service Rust
- [ ] .dockerignore complet
- [ ] Non-root USER
- [ ] HEALTHCHECK sur chaque API
- [ ] Scan Trivy/Grype en CI

## Anti-patterns

| Erreur | Correction |
|--------|------------|
| FROM ubuntu:latest | Version pinÃ©e + slim |
| Secrets ENV/COPY | Build secrets, runtime inject |
| Image >1Go | Multi-stage, base minimale |
| Pas healthcheck | HEALTHCHECK obligatoire |

## Miyukini

Central, MiyuCloud, LLM Bridge: multi-stage. Volumes: kindmother-db, miyucloud-data. Trivy en CI.
## Parcours obtention
Voir KNOWLEDGE.md pour les connaissances requises et les preuves de maitrise.

