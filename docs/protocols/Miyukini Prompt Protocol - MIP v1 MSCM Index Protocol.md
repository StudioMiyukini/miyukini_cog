# MIP v1 — MSCM Index Protocol

**Nom complet :** Miyukini Index Protocol
**Version :** 1.0
**Statut :** Standard officiel
**Dépendance :** MSCM v1 (Miyukini Semantic Code Markup)

---

## 1. Rôle du MIP

Le **MIP (MSCM Index Protocol)** définit le système d'indexation globale du code balisé en MSCM.

Il transforme un codebase MSCM en :

* graphe sémantique
* structure système globale
* modèle exploitable par IA
* mémoire structurelle du projet
* couche de gouvernance

👉 MSCM = sémantique locale
👉 MIP = intelligence structurelle globale

---

## 2. Principe fondamental

> La sémantique est dans le code.
> La structure est dans l'index.
> La gouvernance est dans le graphe.

---

## 3. Architecture générale

```txt
codebase/
│
├── src/                # Code + MSCM
│
└── mscm_index/         # MIP (généré)
    ├── registry.json
    ├── blocks.json
    ├── hierarchy.json
    ├── graph.json
    ├── flows.json
    ├── domains.json
    ├── layers.json
    ├── dependencies.json
    ├── files.json
    └── stats.json
```

---

## 4. Règles globales

* L'index est **externe** au code
* L'index est **reconstruit**, jamais modifié manuellement
* Le code est la seule source de vérité
* MSCM est la source sémantique
* MIP est la source structurelle
* Toute structure macro est dérivée

---

## 5. Pipeline de génération

```txt
Scan codebase
   ↓
Parse MSCM
   ↓
Extraction BLOCKS
   ↓
Construction hiérarchie
   ↓
Construction graphes
   ↓
Projection domaines
   ↓
Projection layers
   ↓
Génération index
```

---

## 6. Fichiers d'index

### 6.1 registry.json (gouvernance)

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

---

### 6.2 blocks.json (identité sémantique)

```json
[
  {
    "id": "auth_token_validation",
    "file": "src/auth/token.ts",
    "start_line": 42,
    "end_line": 98,
    "role": "security",
    "layer": "domain",
    "do": "validate_jwt_and_build_context",
    "human": "Valide un token JWT et construit le contexte utilisateur sécurisé"
  }
]
```

**Champs obligatoires :**
- `id` : Identifiant unique du bloc (obligatoire)
- `file` : Chemin du fichier source (obligatoire)
- `start_line` : Ligne de début (obligatoire)
- `end_line` : Ligne de fin (obligatoire)
- `do` : Description fonctionnelle (obligatoire)

**Champs optionnels :**
- `role` : Rôle sémantique du bloc (optionnel)
- `layer` : Couche architecturale (optionnel)
- `human` : Description humaine lisible (optionnel)

---

### 6.3 hierarchy.json (structure)

```json
{
  "auth_token_validation": [
    "auth_decode",
    "auth_context_build"
  ]
}
```

---

### 6.4 graph.json (relations transverses)

```json
{
  "auth_token_validation": ["user_repository", "jwt_service"],
  "pricing_engine": ["tax_module", "market_service"]
}
```

---

### 6.5 flows.json (process métier)

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

---

### 6.6 domains.json (vision métier)

```json
{
  "auth": ["auth_token_validation", "auth_context_build"],
  "billing": ["invoice_generation", "payment_validation"]
}
```

---

### 6.7 layers.json (architecture technique)

```json
{
  "domain": ["auth_token_validation", "pricing_engine"],
  "infra": ["db_adapter", "redis_cache"]
}
```

---

### 6.8 dependencies.json (dépendances logiques)

```json
{
  "auth_token_validation": ["user_repository", "jwt_service"]
}
```

---

### 6.9 files.json (cartographie code)

```json
{
  "src/auth/token.ts": ["auth_token_validation", "auth_decode", "auth_context_build"]
}
```

---

### 6.10 stats.json (métriques)

```json
{
  "blocks": 428,
  "files": 97,
  "depth_max": 5,
  "domains": 6,
  "layers": 5
}
```

---

## 7. Modèle conceptuel

```txt
CODE  → MSCM  → MIP INDEX  → GRAPH MODEL  → AGENTS / IA / QA / GOV
```

---

## 8. Règles d'intégrité

* ID unique global
* Aucun bloc orphelin
* Aucun cycle invalide
* Hiérarchie cohérente
* Pas de duplication
* Pas de conflit layer

---

## 9. Gouvernance

* L'humain valide la macro-structure
* L'IA propose la structure
* MIP impose la cohérence
* MSCM impose la sémantique

---

## 10. Compatibilité agents

Les agents IA utilisent MIP pour :

* navigation système
* raisonnement multi-modules
* refactor global
* QA structurel
* audit
* sécurité
* simulation d'impact

---

## 11. Compatibilité Cursor

Cursor :

* lit MSCM
* écrit MSCM
* déclenche MIP
* utilise index
* respecte la hiérarchie
* raisonne par bloc

---

## 12. Philosophie

MIP n'est pas un index.

C'est une **mémoire système**.

Une **conscience structurelle** du projet.

Une **base cognitive** pour agents IA.

---

## 13. Statut

Le MIP est **contractuel**.

Tout projet MSCM sans MIP est considéré comme :

* non gouverné
* non structuré globalement
* non IA-native
* non scalable cognitivement

---

# Fin du protocole MIP v1
