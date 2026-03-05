# Certifications MIP - Schema

<!-- @id cert.schema
     @do define_structure_certifications
     @role config
     @layer config -->

## Structure

```
.mip/certifications/
|-- SCHEMA.md
|-- README.md
|-- INDEX.md
|-- registry.json       # Source de verite: cert_id -> folder, agent, tldr
|-- load-map.json       # Tache -> cert_ids (chargement selectif)
|-- sources/            # Sources officielles compactes par agent
|   |-- maria.md
|   |-- fabrice.md
|   |-- denis.md
|   |-- francois.md
|   |-- lise.md
|   |-- arianne.md
|   |-- george.md
|   |-- victor.md
|   |-- hugo.md
|   `-- jean.md
|-- diplomas/
|   |-- INDEX.md        # Index passeports agents
|   `-- {agent}.md      # Diplome simulation par agent
|-- legal/
|   `-- OBLIGATIONS.md  # HDS, NF525, NF203, RGPD
`-- {folder}/           # Un dossier par certification
    |-- REFERENCE.md    # Referentiel court (chargement IA)
    |-- KNOWLEDGE.md    # Connaissances requises pour obtention
    `-- academy/        # Formation locale modulaire
        |-- INDEX.md
        |-- M1.md
        |-- M2.md
        |-- M3.md
        |-- M4.md
        |-- ASSESSMENT.md
        `-- CAPSTONE.md
```

## Convention REFERENCE.md

- Annotations MSCM obligatoires en tete:
  - `@id`: `cert.{agent}.{ref}`
  - `@do`: `provide_{ref}_reference_knowledge`
  - `@role`: testing, security, audit, accessibility...
  - `@layer`: `reference`
  - `@human`: Referentiel {Cert} pour {Agent}
- TL;DR: 1-2 lignes en bloc citation
- Taille cible: <=65 lignes (compact pour les agents IA)
- Format: Markdown, tableaux pour listes
- Reference croisee obligatoire vers `KNOWLEDGE.md`

## Convention KNOWLEDGE.md

- Objet: expliciter les connaissances reelles requises pour preparer l obtention
- Sections minimales:
  - TL;DR
  - blocs de connaissances
  - preuves de maitrise / pre-validation
  - integration MIP
- Format: modulaire, actionnable, non monolithique
- Reference requise vers le module `sources/{agent}.md`

## Convention academy/

- `INDEX.md`: navigation rapide du parcours local
- `M1..M4.md`: un module par bloc de connaissance
- `ASSESSMENT.md`: gate interne readiness
- `CAPSTONE.md`: mission de validation integrale

## Nomenclature dossiers

- Snake_case: `iso_25010`, `lean_startup`
- Normes ISO avec tiret: `iso-iec_27001`, `iso-iec_20000-1`
- Conserver les noms existants (retrocompatibilite)

## Resolution chemins

- `registry.json` et `load-map.json` resolvent vers `.mip/certifications/{folder}/REFERENCE.md`
- `REFERENCE.md` redirige vers `.mip/certifications/{folder}/KNOWLEDGE.md` pour le detail
- Synchronisation passeports: `powershell -File .mip/scripts/sync-cert-diplomas.ps1`
