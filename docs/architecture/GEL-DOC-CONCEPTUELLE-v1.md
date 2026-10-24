# Gel de la doc conceptuelle — v1.0

<!-- @id arch.gel.v1 -->
<!-- @role documentation-freeze -->
<!-- @layer 0-9+zones -->
<!-- @human Périmètre, empreintes et règles d'amendement de la doc conceptuelle COG gelée le 2026-10-24 -->
<!-- @do freeze_conceptual_docs -->

- **Date** : 2026-10-24 · **Version** : 1.1 · **Étiquettes git** : `doc-conceptuelle-v1.0`, `doc-conceptuelle-v1.1`
- **Empreintes** : [`GEL-DOC-CONCEPTUELLE-v1.sha256`](GEL-DOC-CONCEPTUELLE-v1.sha256) — contrôle : `sha256sum -c docs/architecture/GEL-DOC-CONCEPTUELLE-v1.sha256`
- **Suite** : [contraintes](../implementation/CONTRAINTES-IMPLEMENTATION.md) · [revue de code](../implementation/REVUE-CODE-PREPARATION.md) · [plan d'implémentation](../implementation/PLAN-IMPLEMENTATION.md)

## 1. Périmètre gelé

<!-- @id arch.gel.perimetre -->
<!-- @role freeze-scope -->
<!-- @do list_frozen_docs -->

- **Architecture**
  - [Vue globale](COG-VUE-GLOBALE.md) — point d'entrée, décisions D1–D9
  - [Zones et couches](MODELE-ZONES-ET-COUCHES-COG.md)
  - [Services hybrides et Portail](SERVICES-HYBRIDES-ET-PORTAIL.md)
  - [Mémoire MAIA](MEMOIRE-MAIA.md) · [IA fédérée](IA-FEDEREE.md) · [modèles locaux](MODELES-IA-PETITES-CONFIGS.md)
  - Schémas : `schema-global/V1…V6`, `schema-service-relaye.mmd`, `../schema-conceptuel-cog.mmd`
- **Sécurité**
  - [Modèle de menaces EX-01..63](../securite/MODELE-MENACES-COG.md)
  - [Lot 2 — identité signée](../securite/ARCHITECTURE-lot2-identite-signee.md)
  - [Audit 2026-10-23](../securite/AUDIT-2026-10-23-auth-wan.md) (historique)
- **Rendu** : [FigJam](https://www.figma.com/board/Nx7DXwWUy2jwetLGkxB0Jn) — non normatif, les `.mmd` font foi

## 2. Hiérarchie

<!-- @id arch.gel.hierarchie -->
<!-- @role precedence -->
<!-- @do define_doc_precedence -->

- La doc gelée **prime** sur tout autre document (contrats des Cores, docs de services, `doc_for_website`, `.mip/`)
- En cas de conflit : la doc subordonnée est corrigée, pas la doc gelée
- Le code se conforme à la doc gelée ; un écart constaté par la revue est un constat `R-NNN`

## 3. Règles d'amendement

<!-- @id arch.gel.amendement -->
<!-- @role change-control -->
<!-- @do amend_frozen_docs -->

- Toute modification = **amendement** `A-NNN` inscrit au §4, écrit **avant** le code (C-82)
- Contenu d'un amendement : motif, documents touchés, décision (`Dx`), impact sur EX / C / plan
- Validation : le propriétaire du projet
- Après application : version mineure (v1.1, …), empreintes régénérées, nouvelle étiquette git
- Exceptions sans amendement : coquilles, liens cassés, encodage — sans changement de sens

## 4. Registre des amendements

<!-- @id arch.gel.registre -->
<!-- @role amendment-log -->
<!-- @do log_amendments -->

| Id | Date | Objet | Version |
|---|---|---|---|
| — | 2026-10-24 | Gel initial | 1.0 |
| A-001 | 2026-10-24 | Vue globale §9 : éléments hors workspace supprimés (décision du propriétaire) | 1.1 |
