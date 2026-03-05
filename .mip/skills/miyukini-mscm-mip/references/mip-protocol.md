# MIP v1 — MSCM Index Protocol — Reference complete

## Fichiers d'index detailles

### registry.json (gouvernance)

```json
{
  "version": "mip_v1",
  "mscm_version": "v1",
  "generated_at": "2026-01-28T12:00:00Z",
  "files_count": 0,
  "blocks_count": 0,
  "integrity": "ok"
}
```

### blocks.json (identite semantique)

Champs obligatoires : `id`, `file`, `start_line`, `end_line`, `do`
Champs optionnels : `role`, `layer`, `human`

```json
[
  {
    "id": "auth_token_validation",
    "file": "src/auth/token.rs",
    "start_line": 42,
    "end_line": 98,
    "role": "security",
    "layer": "domain",
    "do": "validate_jwt_and_build_context",
    "human": "Valide un token JWT et construit le contexte utilisateur securise"
  }
]
```

### hierarchy.json (structure parent-enfant)

```json
{
  "auth_token_validation": [
    "auth_decode",
    "auth_context_build"
  ]
}
```

### graph.json (relations transverses)

```json
{
  "auth_token_validation": ["user_repository", "jwt_service"],
  "pricing_engine": ["tax_module", "market_service"]
}
```

### flows.json (processus metier)

```json
{
  "auth_pipeline": [
    "auth_input",
    "auth_decode",
    "auth_context_build",
    "auth_finalize"
  ]
}
```

### domains.json (vision metier)

```json
{
  "auth": ["auth_token_validation", "auth_context_build"],
  "billing": ["invoice_generation", "payment_validation"]
}
```

### layers.json (architecture technique)

```json
{
  "domain": ["auth_token_validation", "pricing_engine"],
  "infra": ["db_adapter", "redis_cache"]
}
```

### dependencies.json (dependances logiques)

```json
{
  "auth_token_validation": ["user_repository", "jwt_service"]
}
```

### files.json (cartographie)

```json
{
  "src/auth/token.rs": ["auth_token_validation", "auth_decode", "auth_context_build"]
}
```

### stats.json (metriques)

```json
{
  "blocks": 428,
  "files": 97,
  "depth_max": 5,
  "domains": 6,
  "layers": 5
}
```

## Regles d'integrite

1. ID unique global — aucun doublon
2. Aucun bloc orphelin — tout bloc doit etre referencable
3. Aucun cycle invalide dans les dependances
4. Hierarchie coherente — pas de parent inexistant
5. Pas de duplication de blocs
6. Pas de conflit layer — un bloc = une layer

## Pipeline de generation detaille

```
1. Scan codebase
   → Parcourt tous les fichiers source
   
2. Parse MSCM
   → Detecte les annotations @id, @do, @role, @layer, @human
   
3. Extraction BLOCKS
   → Cree un bloc par annotation @id trouvee
   → Determine start_line et end_line
   
4. Construction hierarchie
   → Analyse l'imbrication des blocs
   → Genere hierarchy.json
   
5. Construction graphes
   → Analyse les references entre blocs
   → Genere graph.json et dependencies.json
   
6. Projection domaines
   → Groupe les blocs par domaine fonctionnel
   → Genere domains.json
   
7. Projection layers
   → Groupe les blocs par couche technique
   → Genere layers.json
   
8. Generation index
   → Ecrit tous les fichiers JSON
   → Calcule stats.json
   → Met a jour registry.json avec integrite
```

## Outil MIP Generator

Situe dans `tools/mscm-generator/`. Utilise pour regenerer l'index complet.

## Compatibilite agents IA

Les agents utilisent MIP pour :
- Navigation systeme (trouver un bloc par domaine ou layer)
- Raisonnement multi-modules (comprendre les dependances)
- Refactor global (mesurer l'impact d'un changement)
- QA structurel (detecter les orphelins ou cycles)
- Audit de securite (tracer les flux sensibles)
- Simulation d'impact (prevoir les consequences)
