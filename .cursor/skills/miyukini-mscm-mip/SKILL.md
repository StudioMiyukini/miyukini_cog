---
name: miyukini-mscm-mip
description: Protocole MSCM (Miyukini Semantic Code Markup) et MIP (MSCM Index Protocol) pour le balisage semantique du code et l'indexation globale. Utiliser quand on ajoute des balises MSCM au code, quand on genere ou met a jour l'index MIP, quand on travaille sur mscm_index/, ou quand on veut comprendre la structure semantique du projet.
---

# MSCM & MIP — Balisage semantique et indexation

## Principe fondamental

> La semantique est dans le code (MSCM).
> La structure est dans l'index (MIP).
> La gouvernance est dans le graphe.

## MSCM — Balisage dans le code

Annotations semantiques placees dans les commentaires du code source :

```rust
//! @id toolkit.auth.miyauth
//! @role security
//! @layer domain
//! @human Kit d'outils d'authentification
//! @do manage_authentication_and_identity
```

### Champs MSCM

| Champ | Obligatoire | Description |
|-------|-------------|-------------|
| `@id` | Oui | Identifiant unique du bloc |
| `@do` | Oui | Description fonctionnelle |
| `@role` | Non | Role semantique (security, data, ui...) |
| `@layer` | Non | Couche architecturale (domain, infra, ui...) |
| `@human` | Non | Description humaine lisible |

## MIP — Index genere

L'index MIP est genere a partir du code balise MSCM et stocke dans `mscm_index/`.

### Fichiers d'index

```
mscm_index/
├── registry.json      # Gouvernance (version, integrite)
├── blocks.json        # Identite semantique de chaque bloc
├── hierarchy.json     # Structure parent-enfant
├── graph.json         # Relations transverses
├── flows.json         # Processus metier
├── domains.json       # Vision metier par domaine
├── layers.json        # Architecture technique par couche
├── dependencies.json  # Dependances logiques
├── files.json         # Cartographie code → blocs
└── stats.json         # Metriques globales
```

### Regles d'integrite

- **ID unique global** — pas de doublons
- **Aucun bloc orphelin** — tout bloc doit etre referencable
- **Aucun cycle invalide** dans les dependances
- **Hierarchie coherente**
- **Pas de conflit layer**

### Pipeline de generation

```
Scan codebase → Parse MSCM → Extraction BLOCKS → Construction hierarchie
→ Construction graphes → Projection domaines → Projection layers → Generation index
```

### Outil de generation

Le generateur MIP se trouve dans `tools/mip-generator/`.

### Format `blocks.json`

```json
{
  "id": "auth_token_validation",
  "file": "src/auth/token.rs",
  "start_line": 42,
  "end_line": 98,
  "role": "security",
  "layer": "domain",
  "do": "validate_jwt_and_build_context",
  "human": "Valide un token JWT et construit le contexte utilisateur"
}
```

## Regles pour l'IA

1. Le code est la **seule source de verite**
2. L'index est **reconstruit**, jamais modifie manuellement
3. MSCM est la source semantique
4. MIP est la source structurelle
5. Tout projet MSCM sans MIP = non gouverne

## References

- Protocole MIP complet : [references/mip-protocol.md](references/mip-protocol.md)
- Documentation : `docs/protocols/MIP v1 MSCM Index Protocol.md`
