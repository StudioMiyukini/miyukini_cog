# Miou â€” Protocole MIP-Miou et MÃ©moire LLM

Pour le **mode LLM du chatbot**, les donnÃ©es stockÃ©es sont **archivÃ©es** et **indexÃ©es** par un protocole inspirÃ© du MIP (MSCM Index Protocol), mais spÃ©cifique Ã  Miou : **MIP-Miou**. La table **Â« Miou LLM Memory Â»** possÃ¨de des **tags** et des **catÃ©gories** pour que le LLM trouve plus facilement l'information dont il a besoin et identifie celle qu'il peut ignorer pour rÃ©pondre.

---

## 1. Contexte et principe

| Aspect | Description |
|--------|-------------|
| **Analogie** | Comme MIP pour le code (sÃ©mantique â†’ index â†’ gouvernance), MIP-Miou pour les connaissances utilisateur (donnÃ©e â†’ index â†’ pertinence) |
| **Objectif** | Permettre au LLM de **retrieval ciblÃ©** : charger uniquement les entrÃ©es pertinentes pour le contexte de conversation |
| **Source** | DonnÃ©es du [Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md), extraites des bulles, du chatbot, des ParamÃ¨tres |

**Principe fondateur :**

> La donnÃ©e est dans la base.  
> La structure est dans l'index.  
> La pertinence est dans les tags et catÃ©gories.

---

## 2. Architecture MIP-Miou

```
miou_data/                    # DonnÃ©es brutes (chiffrÃ©es)
â”œâ”€â”€ user_responses.db         # RÃ©ponses stockÃ©es (Catalogue connaissances)
â””â”€â”€ conversation_archive/    # Historique conversations (optionnel)

miou_memory_index/            # Index MIP-Miou (gÃ©nÃ©rÃ©)
â”œâ”€â”€ registry.json             # Version, intÃ©gritÃ©
â”œâ”€â”€ memory_blocks.json        # EntrÃ©es indexÃ©es (Ã©quivalent blocks.json)
â”œâ”€â”€ categories.json           # Projection par catÃ©gorie
â”œâ”€â”€ tags_index.json           # Index inverse tags â†’ blocks
â”œâ”€â”€ relevance_rules.json      # RÃ¨gles de pertinence par type de requÃªte
â””â”€â”€ stats.json                # MÃ©triques
```

---

## 3. Table Â« Miou LLM Memory Â»

### 3.1 SchÃ©ma d'une entrÃ©e (memory block)

Chaque donnÃ©e stockÃ©e est indexÃ©e sous forme de **block** :

```json
{
  "id": "mem_uuid_xxx",
  "key": "reconfort",
  "category": "emotions",
  "value_preview": "une tisane",
  "value_hash": "sha256_xxx",
  "source": "bulle",
  "question_id": "q3_1",
  "tags": ["pause", "bien-Ãªtre", "reconfort", "personnel"],
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
| `key` | string | ClÃ© de la donnÃ©e (ex. `reconfort`, `hobby`) |
| `category` | string | CatÃ©gorie d'affichage (identite, emotions, projets...) |
| `value_preview` | string | AperÃ§u court (max 50 car.) â€” pour affichage index, pas la valeur complÃ¨te |
| `source` | enum | `bulle`, `chatbot`, `parametres`, `rite_entree` |
| `tags` | string[] | Tags pour retrieval â€” ce pour quoi l'entrÃ©e est pertinente |
| `ignore_for` | string[] | Contextes oÃ¹ le LLM doit ignorer cette entrÃ©e |

### 3.3 Champs optionnels

| Champ | Type | Description |
|-------|------|-------------|
| `question_id` | string | Ex. `q3_1` si source bulle |
| `value_hash` | string | Hash pour intÃ©gritÃ© |
| `relation_palier_min` | int | Palier minimal pour utiliser cette donnÃ©e |
| `created_at`, `updated_at` | DateTime | Horodatage |

---

## 4. Tags et catÃ©gories â€” RÃ´le

### 4.1 Tags : Â« Trouver l'info Â»

Les **tags** indiquent **quand** une entrÃ©e est pertinente. Le LLM (ou un module de retrieval) les utilise pour :

- **Inclure** : Si la requÃªte ou le contexte de conversation matche un tag â†’ charger l'entrÃ©e
- **Exemple** : Tag `pause` â†’ entrÃ©es `reconfort`, `activite_deconnexion` utiles pour une bulle pause

| Type de tag | Exemples | Usage |
|-------------|----------|-------|
| **Contexte conversation** | `pause`, `accueil`, `retour`, `curiosite` | Quelle bulle / quel Ã©change |
| **ThÃ¨me** | `bien-Ãªtre`, `projet`, `loisir` | Sujet abordÃ© |
| **Personnalisation** | `personnel`, `reconfort`, `soutien` | Ton, rÃ©utilisation |
| **DÃ©rivÃ© de la clÃ©** | MÃªme nom que la clÃ© | `reconfort`, `hobby` |

### 4.2 Ignore_for : Â« Ignorer l'info Â»

Le champ **`ignore_for`** indique **quand** une entrÃ©e ne doit **pas** Ãªtre chargÃ©e :

- **Exclure** : Si la requÃªte matche un contexte `ignore_for` â†’ ne pas inclure
- **Exemple** : `reconfort` peut avoir `ignore_for: ["technique", "culture_generale"]` â€” inutile pour une question sur les specs machine ou une blague pop culture

| Contexte ignore_for | Description |
|---------------------|-------------|
| `technique` | Questions specs, bugs, paramÃ¨tres systÃ¨me |
| `culture_generale` | Blagues, rÃ©fÃ©rences pop â€” pas besoin de donnÃ©es personnelles |
| `accueil_froid` | PremiÃ¨re interaction, ton neutre |
| `autre_utilisateur` | (si multi-utilisateur futur) DonnÃ©es d'un autre profil |

### 4.3 CatÃ©gories : Regroupement et filtrage

Les **catÃ©gories** correspondent au [Catalogue Exhaustif](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md) :

| CatÃ©gorie | ClÃ© | EntrÃ©es typiques |
|-----------|-----|------------------|
| `identite` | identite | pseudo, date_naissance, contexte_activite |
| `preferences_pratiques` | preferences_pratiques | preference_rappel, preference_ton, moment_prefere |
| `loisirs` | loisirs | hobby, loisir_lecture, activite_deconnexion |
| `emotions` | emotions | reconfort, besoin_presence, bonheur_quotidien |
| `projets` | projets | projet_coeur, valeur_actuelle, reve |
| `soutien` | soutien | soutien_prefere, aide_soin |
| `accompagnement` | accompagnement | style_accompagnement, style_conseil |
| `humeur` | humeur | humeur_actuelle, theme_ambiance |

**Usage :** Le LLM peut demander Â« charge tout sauf `identite` Â» ou Â« charge uniquement `emotions` et `soutien` Â» selon le type de rÃ©ponse Ã  gÃ©nÃ©rer.

---

## 5. Pipeline d'indexation

```
1. Utilisateur rÃ©pond (bulle) ou partage (chatbot) â†’ stockage dans user_responses.db
2. Trigger indexation : nouvel enregistrement ou modification
3. Extraction : key, category, value_preview, source, question_id
4. GÃ©nÃ©ration tags : mapping keyâ†’tags, dÃ©rivation contexte
5. GÃ©nÃ©ration ignore_for : rÃ¨gles par catÃ©gorie
6. CrÃ©ation memory block
7. Mise Ã  jour memory_blocks.json, tags_index.json, categories.json
```

### 5.1 Mapping key â†’ tags (exemples)

| key | tags par dÃ©faut |
|-----|-----------------|
| `reconfort` | pause, bien-Ãªtre, reconfort, personnel |
| `hobby` | curiosite, loisir, personnalisation |
| `projet_coeur` | retour, projet, soutien |
| `preference_ton` | accueil, frequence, parametres |
| `activite_deconnexion` | pause, bien-Ãªtre, dÃ©connexion |

### 5.2 Mapping category â†’ ignore_for (exemples)

| category | ignore_for par dÃ©faut |
|----------|----------------------|
| `emotions` | technique, culture_generale |
| `projets` | technique, accueil_froid |
| `identite` | (aucun â€” toujours pertinent pour personnalisation) |
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
    "tags": ["pause", "bien-Ãªtre", "reconfort", "personnel"],
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
  "bien-Ãªtre": ["mem_abc123", "mem_def456"],
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

### 6.4 relevance_rules.json (rÃ¨gles de retrieval)

```json
{
  "bulle_pause": {
    "include_tags": ["pause", "bien-Ãªtre", "reconfort", "activite_deconnexion"],
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

### 7.1 Avant chaque rÃ©ponse

1. **Contexte dÃ©tectÃ©** : type de requÃªte (bulle pause, accueil, chatbot libre, etc.)
2. **RÃ¨gles appliquÃ©es** : `relevance_rules.json` pour ce contexte
3. **Retrieval** : charger les blocks dont les tags/categories matchent, et dont `ignore_for` ne matche pas
4. **Injection** : Les valeurs (dÃ©chiffrÃ©es) sont injectÃ©es dans le prompt du LLM

### 7.2 Exemple de prompt enrichi

```
[CONTEXTE] Bulle pause santÃ© â€” l'utilisateur est connectÃ© depuis 2h.

[DONNÃ‰ES PERTINENTES - MIOU LLM MEMORY]
- reconfort: "une tisane"
- activite_deconnexion: "courir"

[INSTRUCTIONS] GÃ©nÃ¨re une bulle de pause en rÃ©utilisant ces donnÃ©es si pertinent. Ton chaleureux, palier Amie.
```

### 7.3 Ce que le LLM peut ignorer

- DonnÃ©es dont le contexte actuel est dans `ignore_for`
- DonnÃ©es dont `relation_palier_min` > palier actuel
- DonnÃ©es supprimÃ©es par l'utilisateur (absence dans l'index)

---

## 8. Archivage

Les donnÃ©es sont **archivÃ©es** avant indexation :

- **Snapshot** : copie de l'Ã©tat des rÃ©ponses au moment de l'indexation
- **Versioning** : chaque modification crÃ©e une nouvelle entrÃ©e ou met Ã  jour le block
- **TraÃ§abilitÃ©** : `created_at`, `updated_at` sur chaque block

---

## 9. RÃ©fÃ©rences

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)
- [docs/contrats/Miyukini Prompt Protocol - MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md)

---

**Version :** 1.0  
**Statut :** SpÃ©cification protocole MIP-Miou et Miou LLM Memory

