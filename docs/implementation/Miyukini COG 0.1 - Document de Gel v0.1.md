# Miyukini COG 0.1 — Document de Gel v0.1

## Contexte

Ce document formalise le gel de la version **Miyukini COG 0.1 (Core-Orchestrated Governance Environment)** conformément au plan d'implémentation et aux contrats de versionnement.

## Version et date

- **Version gelée :** v0.1.0  
- **Date de gel :** 2025-01-28  
- **Environnement :** COG 0.1  

## Éléments gelés

### Phase 1 — Kernel

- **miyukini-kernel** : config, id, time, log, lifecycle, lib
- Tests d'intégration Kernel
- Index MIP Phase 1 (mscm_index/)

### Phase 2 — Cores système

- **strongfather** : intent, policy, decision, policy_engine, priority, validator, lib
- **kindmother** : state, storage, sync, api, threat, observability, lib
- **borderguard** : boundary, trust_level, crossing, lib
- **caringnanny** : observer, metrics, health, lib
- **masterbutler** : workflow, orchestrator, step, lib
- **bondingbrother** : connection, sync, translation, lib
- **everbuddy** : compatibility, migration, version, lib
- **worrysentinel** : threat_detector, security_level, degradation, lib
- **tamr** : taxonomy, metadata, classification, lib
- **logisticssteward** : resource, optimization, allocation, lib
- Index MIP Phase 2 (Kernel + Cores)

### Phase 3 — MiyukiniAdmin

- **miyukini-admin** : config, monitoring_service, database_service, security_service, testing_service, bonding_brother_bridge, audit_logger, api_handlers, api_routes, main (backend)
- Index MIP final (Kernel + Cores + MiyukiniAdmin)

## Index MIP final

- Emplacement : `mscm_index/`
- Fichiers : registry.json, blocks.json, hierarchy.json, graph.json, flows.json, domains.json, layers.json, dependencies.json, files.json, stats.json
- Intégrité : `registry.json` → `integrity: "ok"`

## Règles d'évolution futures

1. Toute modification des signatures publiques (traits, types exportés) des composants gelés nécessite une version majeure (v0.2.0 ou supérieure).
2. Les ajouts non breaking (nouvelles fonctions, nouveaux types) peuvent être faits en version mineure (v0.1.x).
3. Les correctifs de bugs sans changement de contrat peuvent être faits en version patch (v0.1.0 → v0.1.1).
4. L'index MIP doit être régénéré après toute modification du code balisé MSCM.

## Conditions de dégel

Le gel COG 0.1 peut être levé pour :

- Passage à une version majeure (COG 0.2) après validation des contrats et mise à jour de la documentation fondatrice.
- Correction d’anomalies bloquantes validées par l’équipe, avec mise à jour du présent document et du numéro de version (patch ou mineur selon le cas).

## Références

- Plan d’implémentation : `.cursor/plans/implémentation_miyukini_cog_0.1_b2a6f2ec.plan.md`
- Documentation Implementation Reference : `docs/implementation/Miyukini COG 0.1 - Documentation Implementation Reference.md`
- MSCM MIP Compliance Checklist : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- Kernel — Gel et Versionnement : `docs/kernel/Kernel - Gel et Versionnement v0.1.md` (si existant)
