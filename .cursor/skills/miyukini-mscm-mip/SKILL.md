---
name: miyukini-mscm-mip
description: Protocole MSCM (Miyukini Semantic Code Markup) et MIP (MSCM Index Protocol) pour le balisage semantique du code et l'indexation globale. Utiliser quand on ajoute des balises MSCM au code, quand on genere ou met a jour l'index MIP, quand on travaille sur mscm_index/, ou quand on veut comprendre la structure semantique du projet. Les blocs MIP sont utilises dans la verification de conformite MWS (Phase B, relay).
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

## Lien avec le MWS (Webway)

Lors de la **verification de conformite** (relay / Origin), la **Phase B** utilise des **blocs de code MIP** : le relay demande un bloc aleatoire par Service, le COG envoie un paquet chiffre contenant ce bloc ; le relay verifie avec les references Origin. Les blocs MSCM/MIP servent donc a l'attestation d'integrite des Services. Voir `docs/miyukini-webway-system/verification/` et `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md` (Phase B).

## References

*Chemins `docs/` relatifs a la racine du workspace. Liens relatifs au skill pour references/.*

- **Protocole MIP (skill)** : [references/mip-protocol.md](references/mip-protocol.md)
- **Protocole MIP (workspace)** : `docs/contrats/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol.md`
- **Implementation / MSCM** : `docs/implementation/Miyukini COG 0.1 - MSCM MIP Compliance Checklist.md`
- **Verification MWS (blocs MIP)** : `docs/miyukini-webway-system/verification/MWS - Flux de Verification.md`
- **Relay (Phase B)** : `docs/reference/Miyukini Conceptual References - Miyukini Webway Relay.md` (section Phase B)
- **Index MIP** : repertoire `mscm_index/` a la racine du projet
