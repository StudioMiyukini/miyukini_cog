# Miou — Protocole MIP-Miou et Mémoire LLM

Pour le **mode LLM du chatbot**, les données stockées sont **archivées** et **indexées** par un protocole inspiré du MIP (MSCM Index Protocol), mais spécifique à Miou : **MIP-Miou**. La table **« Miou LLM Memory »** possède des **tags** et des **catégories** pour que le LLM trouve plus facilement l'information dont il a besoin et identifie celle qu'il peut ignorer pour répondre.

---

## 1. Contexte et principe

| Aspect | Description |
|--------|-------------|
| **Analogie** | Comme MIP pour le code (sémantique → index → gouvernance), MIP-Miou pour les connaissances utilisateur (donnée → index → pertinence) |
| **Objectif** | Permettre au LLM de **retrieval ciblé** : charger uniquement les entrées pertinentes pour le contexte de conversation |
| **Source** | Données du [Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md), extraites des bulles, du chatbot, des Paramètres |

**Principe fondateur :**

> La donnée est dans la base.  
> La structure est dans l'index.  
> La pertinence est dans les tags et catégories.

---

## 2. Architecture MIP-Miou

```
miou_data/                    # Données brutes (chiffrées)
├── user_responses.db         # Réponses stockées (Catalogue connaissances)
└── conversation_archive/    # Historique conversations (optionnel)

miou_memory_index/            # Index MIP-Miou (généré)
├── registry.json             # Version, intégrité
├── memory_blocks.json        # Entrées indexées (équivalent blocks.json)
├── categories.json           # Projection par catégorie
├── tags_index.json           # Index inverse tags → blocks
├── relevance_rules.json      # Règles de pertinence par type de requête
└── stats.json                # Métriques
```

---

## 3. Table « Miou LLM Memory »

### 3.1 Schéma d'une entrée (memory block)

Chaque donnée stockée est indexée sous forme de **block** :

```json
{
  "id": "mem_uuid_xxx",
  "key": "reconfort",
  "category": "emotions",
  "value_preview": "une tisane",
  "value_hash": "sha256_xxx",
  "source": "bulle",
  "question_id": "q3_1",
  "tags": ["pause", "bien-être", "reconfort", "personnel"],
  "ignore_for": ["technique", "specs", "culture_generale"],
  "created_at": "2026-02-15T10:30:00Z",
  "updated_at": "2026-02-15T10:30:00Z",
  "relation_palier_min": 3
}
```

### 3.2 Champs obligatoires

| Champ | Type | Description |
|-------|------|-------------|
| `id` | UUID | Identifiant unique du block |
| `key` | string | Clé de la donnée (ex. `reconfort`, `hobby`) |
| `category` | string | Catégorie d'affichage (identite, emotions, projets...) |
| `value_preview` | string | Aperçu court (max 50 car.) — pour affichage index, pas la valeur complète |
| `source` | enum | `bulle`, `chatbot`, `parametres`, `rite_entree` |
| `tags` | string[] | Tags pour retrieval — ce pour quoi l'entrée est pertinente |
| `ignore_for` | string[] | Contextes où le LLM doit ignorer cette entrée |

### 3.3 Champs optionnels

| Champ | Type | Description |
|-------|------|-------------|
| `question_id` | string | Ex. `q3_1` si source bulle |
| `value_hash` | string | Hash pour intégrité |
| `relation_palier_min` | int | Palier minimal pour utiliser cette donnée |
| `created_at`, `updated_at` | DateTime | Horodatage |

---

## 4. Tags et catégories — Rôle

### 4.1 Tags : « Trouver l'info »

Les **tags** indiquent **quand** une entrée est pertinente. Le LLM (ou un module de retrieval) les utilise pour :

- **Inclure** : Si la requête ou le contexte de conversation matche un tag → charger l'entrée
- **Exemple** : Tag `pause` → entrées `reconfort`, `activite_deconnexion` utiles pour une bulle pause

| Type de tag | Exemples | Usage |
|-------------|----------|-------|
| **Contexte conversation** | `pause`, `accueil`, `retour`, `curiosite` | Quelle bulle / quel échange |
| **Thème** | `bien-être`, `projet`, `loisir` | Sujet abordé |
| **Personnalisation** | `personnel`, `reconfort`, `soutien` | Ton, réutilisation |
| **Dérivé de la clé** | Même nom que la clé | `reconfort`, `hobby` |

### 4.2 Ignore_for : « Ignorer l'info »

Le champ **`ignore_for`** indique **quand** une entrée ne doit **pas** être chargée :

- **Exclure** : Si la requête matche un contexte `ignore_for` → ne pas inclure
- **Exemple** : `reconfort` peut avoir `ignore_for: ["technique", "culture_generale"]` — inutile pour une question sur les specs machine ou une blague pop culture

| Contexte ignore_for | Description |
|---------------------|-------------|
| `technique` | Questions specs, bugs, paramètres système |
| `culture_generale` | Blagues, références pop — pas besoin de données personnelles |
| `accueil_froid` | Première interaction, ton neutre |
| `autre_utilisateur` | (si multi-utilisateur futur) Données d'un autre profil |

### 4.3 Catégories : Regroupement et filtrage

Les **catégories** correspondent au [Catalogue Exhaustif](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md) :

| Catégorie | Clé | Entrées typiques |
|-----------|-----|------------------|
| `identite` | identite | pseudo, date_naissance, contexte_activite |
| `preferences_pratiques` | preferences_pratiques | preference_rappel, preference_ton, moment_prefere |
| `loisirs` | loisirs | hobby, loisir_lecture, activite_deconnexion |
| `emotions` | emotions | reconfort, besoin_presence, bonheur_quotidien |
| `projets` | projets | projet_coeur, valeur_actuelle, reve |
| `soutien` | soutien | soutien_prefere, aide_soin |
| `accompagnement` | accompagnement | style_accompagnement, style_conseil |
| `humeur` | humeur | humeur_actuelle, theme_ambiance |

**Usage :** Le LLM peut demander « charge tout sauf `identite` » ou « charge uniquement `emotions` et `soutien` » selon le type de réponse à générer.

---

## 5. Pipeline d'indexation

```
1. Utilisateur répond (bulle) ou partage (chatbot) → stockage dans user_responses.db
2. Trigger indexation : nouvel enregistrement ou modification
3. Extraction : key, category, value_preview, source, question_id
4. Génération tags : mapping key→tags, dérivation contexte
5. Génération ignore_for : règles par catégorie
6. Création memory block
7. Mise à jour memory_blocks.json, tags_index.json, categories.json
```

### 5.1 Mapping key → tags (exemples)

| key | tags par défaut |
|-----|-----------------|
| `reconfort` | pause, bien-être, reconfort, personnel |
| `hobby` | curiosite, loisir, personnalisation |
| `projet_coeur` | retour, projet, soutien |
| `preference_ton` | accueil, frequence, parametres |
| `activite_deconnexion` | pause, bien-être, déconnexion |

### 5.2 Mapping category → ignore_for (exemples)

| category | ignore_for par défaut |
|----------|----------------------|
| `emotions` | technique, culture_generale |
| `projets` | technique, accueil_froid |
| `identite` | (aucun — toujours pertinent pour personnalisation) |
| `preferences_pratiques` | culture_generale |

---

## 6. Format des fichiers d'index

### 6.1 memory_blocks.json

```json
[
  {
    "id": "mem_abc123",
    "key": "reconfort",
    "category": "emotions",
    "value_preview": "une tisane",
    "source": "bulle",
    "question_id": "q3_1",
    "tags": ["pause", "bien-être", "reconfort", "personnel"],
    "ignore_for": ["technique", "culture_generale"],
    "relation_palier_min": 3,
    "created_at": "2026-02-15T10:30:00Z"
  }
]
```

### 6.2 tags_index.json (index inverse)

```json
{
  "pause": ["mem_abc123", "mem_def456"],
  "reconfort": ["mem_abc123"],
  "bien-être": ["mem_abc123", "mem_def456"],
  "curiosite": ["mem_ghi789"]
}
```

### 6.3 categories.json

```json
{
  "emotions": ["mem_abc123"],
  "loisirs": ["mem_def456"],
  "projets": ["mem_ghi789"]
}
```

### 6.4 relevance_rules.json (règles de retrieval)

```json
{
  "bulle_pause": {
    "include_tags": ["pause", "bien-être", "reconfort", "activite_deconnexion"],
    "include_categories": ["emotions", "loisirs"],
    "exclude_contexts": ["technique"]
  },
  "bulle_accueil": {
    "include_tags": ["accueil", "bonheur_quotidien", "moment_prefere"],
    "include_categories": ["emotions", "preferences_pratiques"]
  },
  "chatbot_general": {
    "include_categories": ["identite", "emotions", "projets", "loisirs"],
    "exclude_contexts": ["technique"]
  }
}
```

---

## 7. Usage par le LLM

### 7.1 Avant chaque réponse

1. **Contexte détecté** : type de requête (bulle pause, accueil, chatbot libre, etc.)
2. **Règles appliquées** : `relevance_rules.json` pour ce contexte
3. **Retrieval** : charger les blocks dont les tags/categories matchent, et dont `ignore_for` ne matche pas
4. **Injection** : Les valeurs (déchiffrées) sont injectées dans le prompt du LLM

### 7.2 Exemple de prompt enrichi

```
[CONTEXTE] Bulle pause santé — l'utilisateur est connecté depuis 2h.

[DONNÉES PERTINENTES - MIOU LLM MEMORY]
- reconfort: "une tisane"
- activite_deconnexion: "courir"

[INSTRUCTIONS] Génère une bulle de pause en réutilisant ces données si pertinent. Ton chaleureux, palier Amie.
```

### 7.3 Ce que le LLM peut ignorer

- Données dont le contexte actuel est dans `ignore_for`
- Données dont `relation_palier_min` > palier actuel
- Données supprimées par l'utilisateur (absence dans l'index)

---

## 8. Archivage

Les données sont **archivées** avant indexation :

- **Snapshot** : copie de l'état des réponses au moment de l'indexation
- **Versioning** : chaque modification crée une nouvelle entrée ou met à jour le block
- **Traçabilité** : `created_at`, `updated_at` sur chaque block

---

## 9. Références

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)
- [docs/contrats/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](../../../contrats/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md)

---

**Version :** 1.0  
**Statut :** Spécification protocole MIP-Miou et Miou LLM Memory
