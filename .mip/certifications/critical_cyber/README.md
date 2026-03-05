<!-- @id cert.critical_cyber.readme -->
<!-- @do describe_critical_cyber_knowledge_base -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Documentation base cyber critique -->

# Critical Cyber Certifications KB

> Base de connaissance cyber critique pour agents MIP, orientee verification de controles.

## Design

- Granularite: 1 fichier = 1 connaissance.
- Indexation: annotations MSCM dans chaque fichier + `MSCM_INDEX.json`.
- Usages: audit de conformite de controle, preparation certification, entrainement agent.

## Dossier

- `INDEX.md` : routage global
- `REFERENCE.md` : point d entree compact compatible `registry/load-map`
- `PATHWAY.md` : parcours de montee en competence IA
- `SOURCES.md` : sources officielles web
- `MSCM_INDEX.json` : index machine des connaissances
- `../<cert>/critical_kb/` : fichiers atomiques regroupes dans les dossiers certification existants
