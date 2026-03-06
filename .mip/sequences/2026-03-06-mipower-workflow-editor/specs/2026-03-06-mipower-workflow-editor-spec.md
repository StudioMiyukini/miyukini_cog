# Specification 2026-03-06-mipower-workflow-editor

## Statut

- Etat : TERMINE
- Phase : P0 Temps 6
- Responsable principal : Francois

## TL;DR

MIPOWER = application desktop Tauri v2 (Rust backend + Svelte 5 frontend). 4 modules V1 : Dashboard sequences (live, depuis sequences/index.json), Lecteur rapport Markdown riche (Mermaid + tableaux), Suivi temps reel (file watcher Rust), Prompt builder (formulaire -> premier prompt MIP + init arborescence). Index interne SQLite (rusqlite). Securite bornee par Tauri capabilities.

---

## 1. Architecture globale

```
mipower/                          <- nouveau workspace Rust (hors Miyukini-COG)
  src-tauri/                      <- backend Rust (Tauri)
    src/
      main.rs                     <- entrypoint Tauri
      commands/
        sequences.rs              <- commandes IPC : list, get, refresh
        watcher.rs                <- file watcher -> emit events
        prompt_builder.rs         <- generation premier prompt + init sequence
        db.rs                     <- SQLite : open, migrate, query
      db/
        schema.sql                <- schema SQLite (sequences, artefacts, metrics_snapshot)
        migrations/               <- migrations futures
      models.rs                   <- structs Rust (Sequence, Artefact, Metrics)
    Cargo.toml
    tauri.conf.json               <- capabilities Tauri (fs bornee)
    capabilities/
      default.json                <- fs:read .mip/, %APPDATA%/mipower/
  src/                            <- frontend Svelte 5
    lib/
      components/
        Dashboard.svelte          <- vue liste sequences
        SequenceCard.svelte       <- carte sequence (statut, T/C, date)
        ReportViewer.svelte       <- lecteur MD riche (Mermaid, tableaux)
        ProgressTracker.svelte    <- suivi temps reel phases/temps
        PromptBuilder.svelte      <- formulaire guide
        Sidebar.svelte            <- navigation principale
      stores/
        sequences.ts              <- store Svelte : liste sequences
        active.ts                 <- sequence/artefact actif
      utils/
        markdown.ts               <- wrapper marked.js + DOMPurify + mermaid
        ipc.ts                    <- wrappers commandes Tauri
    routes/
      +page.svelte                <- router principal
    app.html
    app.css                       <- TailwindCSS entry
  package.json
  vite.config.ts
  tailwind.config.ts
```

---

## 2. Modules V1

### 2.1 Dashboard sequences

**Source** : `sequences/index.json` (lu depuis le workspace .mip/ de l'utilisateur)
**Commande Tauri** : `list_sequences(mip_root: String) -> Vec<SequenceMeta>`
**UI** : grille de SequenceCard, tri par date desc par defaut, filtre par statut/classe

```rust
// models.rs
#[derive(Serialize, Deserialize)]
pub struct SequenceMeta {
    pub slug: String,
    pub date: String,
    pub status: String,          // "active" | "done" | "archived"
    pub task_class: String,      // "T3" | "T4" | "T5"
    pub complexity: String,      // "C3" | "C4" | "C5"
    pub path: String,
    pub tags: Vec<String>,
}
```

### 2.2 Lecteur rapport (Must work #1)

**Input** : chemin .md d'un artefact MIP (brief, spec, plan, rapport)
**Commande Tauri** : `read_artefact(path: String) -> String` (contenu brut UTF-8)
**Rendu frontend** :
- `marked.js` -> HTML
- `DOMPurify.sanitize()` -> securise le HTML inline
- `mermaid.js` -> diagrams inline (detectes par fenced code ` ```mermaid `)
- TailwindCSS typography plugin (`prose`) -> tableaux, listes, titres

**Navigation** : arbre des artefacts dans le panel gauche (briefs/ specs/ plans_p3/ audits/ rapports_finaux/)

### 2.3 Suivi temps reel

**Crate** : `notify` v6 (debounce integre)
**Commande Tauri** : `start_watcher(seq_path: String)` -> emet `sequence-updated` events
**Frontend** : subscribe via `listen("sequence-updated", ...)` -> refresh store -> rerender ProgressTracker

**Calcul progression** : lecture des fichiers `phases/p0/temps/temps-*.md` -> detecte "Etat : TERMINE" -> jauge par phase

```
P0: T1...T11 -> progression = nb(TERMINE) / 11
P3: etapes/etape-*.md -> progression = nb(TERMINE) / total
P4/P5/P6: phases/*-trace.md -> detecte sections TERMINE
```

### 2.4 Prompt builder

**Formulaire Svelte** :
- Champ : titre de la sequence (slug auto-derive)
- Select : classe tache (T3/T4/T5)
- Select : domaine (back/front/infra/full-stack/autre)
- Textarea : description demande
- Champ : contraintes techniques connues
- Select : stack (Rust / TypeScript / Python / autre)
- Champ : tags

**Sortie** :
1. Texte du premier prompt MIP (copie clipboard + affiche dans UI)
2. (optionnel) Appel `init_sequence(slug, date)` -> invoque `init-sequence-base.ps1` via Tauri shell

**Commande Tauri** : `init_sequence(slug: String, mip_root: String) -> Result<String, String>`

---

## 3. Base de donnees SQLite

**Fichier** : `%APPDATA%/mipower/mipower.db` (cree au premier lancement)

```sql
CREATE TABLE sequences (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    slug        TEXT    NOT NULL UNIQUE,
    date        TEXT    NOT NULL,
    status      TEXT    NOT NULL DEFAULT 'active',
    task_class  TEXT,
    complexity  TEXT,
    path        TEXT    NOT NULL,
    tags        TEXT    DEFAULT '[]',   -- JSON array
    indexed_at  TEXT    NOT NULL
);

CREATE TABLE artefacts (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    seq_id      INTEGER NOT NULL REFERENCES sequences(id),
    type        TEXT    NOT NULL,       -- 'brief'|'spec'|'plan'|'audit'|'rapport'
    path        TEXT    NOT NULL,
    last_mod    TEXT
);

CREATE TABLE metrics_snapshot (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    seq_id      INTEGER NOT NULL REFERENCES sequences(id),
    tokens_total INTEGER,
    duration_ms  INTEGER,
    snapshot_at  TEXT    NOT NULL
);
```

**Indexation** : au demarrage, MIPOWER lit `sequences/index.json` + scanne le FS -> upsert SQLite -> UI charge depuis SQLite (rapide, sans relire le FS a chaque render)

---

## 4. IPC Tauri — Commandes exposees

| Commande | Parametres | Retour | Description |
|----------|-----------|--------|-------------|
| `list_sequences` | `mip_root: String` | `Vec<SequenceMeta>` | Liste depuis SQLite (indexee) |
| `get_sequence` | `slug: String` | `SequenceDetail` | Detail + artefacts |
| `read_artefact` | `path: String` | `String` | Contenu .md brut |
| `start_watcher` | `seq_path: String` | `()` | Lance file watcher, emet events |
| `stop_watcher` | `seq_path: String` | `()` | Arrete watcher |
| `init_sequence` | `slug, mip_root` | `Result<String>` | Cree arborescence MIP |
| `reindex` | `mip_root: String` | `()` | Re-scanne et met a jour SQLite |
| `generate_prompt` | `PromptBuilderInput` | `String` | Genere premier prompt MIP |

---

## 5. Tauri capabilities (securite)

```json
// capabilities/default.json
{
  "permissions": [
    "core:default",
    { "identifier": "fs:read", "allow": [{ "path": "$APPDATA/mipower/**" }, { "path": "$HOME/**/.mip/**" }] },
    { "identifier": "fs:write", "allow": [{ "path": "$APPDATA/mipower/**" }] },
    "shell:allow-execute"
  ]
}
```

**CSP tauri.conf.json** :
```json
"security": {
  "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'"
}
```
Note : `unsafe-inline` autorise pour les styles TailwindCSS inline ; scripts = `self` uniquement.

---

## 6. Dependances Cargo

```toml
[dependencies]
tauri        = { version = "2", features = ["shell-open"] }
serde        = { version = "1", features = ["derive"] }
serde_json   = "1"
rusqlite     = { version = "0.31", features = ["bundled"] }
notify       = "6"
notify-debouncer-mini = "0.4"
tokio        = { version = "1", features = ["rt-multi-thread", "macros"] }
thiserror    = "1"
```

---

## 7. Dependances frontend (package.json)

```json
{
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "vite": "^5",
    "svelte": "^5",
    "@sveltejs/vite-plugin-svelte": "^3",
    "tailwindcss": "^3",
    "autoprefixer": "^10"
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "marked": "^12",
    "dompurify": "^3",
    "mermaid": "^10"
  }
}
```

---

## 8. Hors scope V1

- Edition inline des artefacts .md dans MIPOWER
- Versioning git integre
- Knowledge Base interactive
- Integration LLM runtime
- Collaboration multi-utilisateurs
- Cloud sync

---

## 9. Verification docs librairies (Context7)

| Librairie | ID a resoudre | Usage |
|-----------|--------------|-------|
| Tauri v2 | a resoudre | IPC, capabilities, CSP, updater |
| notify v6 | a resoudre | file watcher debounce |
| rusqlite 0.31 | a resoudre | SQLite parameterized queries |
| marked.js v12 | a resoudre | Markdown -> HTML renderer |
| mermaid v10 | a resoudre | Diagrammes inline |
| Svelte 5 | a resoudre | Runes, stores, composants |

> A resoudre en P3 via `resolve-library-id` avant chaque tache concernee.

---

## Statut final T6

- Etat : TERMINE
- Horodatage : 2026-03-06
- Agent : Francois
