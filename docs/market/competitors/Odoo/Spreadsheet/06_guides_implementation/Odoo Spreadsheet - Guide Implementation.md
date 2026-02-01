# Odoo Spreadsheet — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Spreadsheet dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (Proposition)

```
crates/
├── miyuspreadsheet/                    # SpreadsheetOperator + logique métier
│   ├── src/
│   │   ├── lib.rs
│   │   ├── spreadsheet.rs              # Modèle Spreadsheet, feuilles
│   │   ├── datasource.rs               # Sources (List, Pivot, Chart)
│   │   ├── formula.rs                  # Résolution formules (ODOO.LIST, PIVOT, etc.)
│   │   ├── template.rs                # Templates (délégué ou intégré)
│   │   ├── version.rs                 # Snapshots, restauration
│   │   ├── locale.rs                  # Paramètres régionaux
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuspreadsheet-ui/                 # SpreadsheetUI (frontend)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── grid.rs                     # Grille, cellules, barre de formule
│   │   ├── menus.rs                    # File, Edit, Insert, Data, View
│   │   ├── panels.rs                   # Data, Filtres, Settings, Version history
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── (Dépendances existantes)
    miyukini-kernel, miyukini-central, miyucalc, miyuinvoice, miyustore, ...
```

**Note :** Le moteur de grille et de formules peut être un composant existant (type o-spreadsheet en JS/TS) ou une implémentation Rust (calcul côté serveur) ; l’architecture ci-dessus suppose une séparation claire entre logique métier (Rust) et rendu/édition (UI).

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucalc` : Calculs, formules (si réutilisation pour fonctions standard)
- `miyuinvoice` / `miyucptaledger` : Fonctions type BALANCE, CREDIT, DEBIT, FISCALYEAR
- `miyustore` / `miyuinvoice` / etc. : Opérateurs métier exposant vues (liste, pivot, graph)
- `miyuclock` : Dates, locale
- Opérateur Documents ou Fichiers : Stockage, dossiers, partage

---

## 2. Schémas de Données

### 2.1 Modèle Spreadsheet (Classeur)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spreadsheet {
    pub id: SpreadsheetId,
    pub name: String,
    pub folder_id: Option<FolderId>,
    pub owner_id: UserId,
    pub company_id: CompanyId,
    pub locale: LocaleId,
    pub content: SpreadsheetContent,  // JSON ou structure typée (feuilles, cellules, sources)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_template: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpreadsheetContent {
    pub sheets: Vec<Sheet>,
    pub global_filters: Vec<GlobalFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: SheetId,
    pub name: String,
    pub order: u32,
    pub cells: Option<GridCells>,   // ou référence à un blob
    pub inserted_objects: Vec<InsertedObject>,  // list, pivot, chart + position
}
```

### 2.2 Modèle DataSource

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceType {
    List { list_id: u32, model: String, domain: Vec<DomainTerm>, sort: Vec<SortSpec>, columns: Vec<String>, row_count: u32 },
    Pivot { pivot_id: u32, model: String, domain: Vec<DomainTerm>, rows: Vec<String>, cols: Vec<String>, measures: Vec<String>, dynamic: bool },
    Chart { chart_id: u32, model: String, domain: Vec<DomainTerm>, config: ChartConfig },
}

pub struct DataSource {
    pub id: DataSourceId,
    pub spreadsheet_id: SpreadsheetId,
    pub name: String,
    pub type_: DataSourceType,
    pub created_at: DateTime<Utc>,
}
```

### 2.3 Modèle Version (Snapshot)

```rust
pub struct SpreadsheetVersion {
    pub id: VersionId,
    pub spreadsheet_id: SpreadsheetId,
    pub content: SpreadsheetContent,  // copie immuable
    pub created_at: DateTime<Utc>,
    pub created_by: UserId,
    pub name: Option<String>,
}
```

### 2.4 Modèle Template

```rust
pub struct SpreadsheetTemplate {
    pub id: TemplateId,
    pub name: String,
    pub content: SpreadsheetContent,
    pub company_id: Option<CompanyId>,  // None = global
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## 3. API et Contrats

### 3.1 Opérations Classeur

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `create_spreadsheet(intent, mandate)` | Créer un classeur (vide ou depuis template) | spreadsheet.create |
| `get_spreadsheet(id, mandate)` | Récupérer métadonnées + contenu | Viewer/Editor |
| `update_spreadsheet(id, intent, mandate)` | Mettre à jour nom, locale, contenu | spreadsheet.update |
| `share_spreadsheet(id, intent, mandate)` | Partager (Viewer/Editor, freeze) | spreadsheet.share |
| `export_xlsx(id, mandate)` | Exporter en .xlsx (formules → valeurs) | Viewer/Editor |
| `convert_to_dashboard(id, intent, mandate)` | Créer un dashboard à partir du classeur | spreadsheet.convert_to_dashboard |

### 3.2 Opérations Sources

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `insert_list(spreadsheet_id, intent, mandate)` | Insérer une source liste | datasource.create |
| `insert_pivot(spreadsheet_id, intent, mandate)` | Insérer une source pivot | datasource.create |
| `insert_chart(spreadsheet_id, intent, mandate)` | Insérer une source graphique | datasource.create |
| `refresh_datasources(spreadsheet_id, mandate)` | Rafraîchir toutes les sources | datasource.refresh |
| `delete_datasource(spreadsheet_id, source_id, mandate)` | Supprimer une source | datasource.delete |

### 3.3 Opérations Formules

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `resolve_formula(spreadsheet_id, formula, context, mandate)` | Résoudre une formule (ODOO.LIST, PIVOT, BALANCE, etc.) | Lecture données selon source |

### 3.4 Opérations Template / Version

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `save_as_template(spreadsheet_id, name, mandate)` | Enregistrer comme template | template.create |
| `create_from_template(template_id, name, folder_id, mandate)` | Créer classeur depuis template | spreadsheet.create |
| `list_versions(spreadsheet_id, mandate)` | Lister les versions | version.list |
| `restore_version(spreadsheet_id, version_id, mandate)` | Restaurer une version | version.restore |
| `copy_version(spreadsheet_id, version_id, mandate)` | Copier une version (nouveau classeur) | version.copy + spreadsheet.create |

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Fondations)

- **Classeur** : Création, lecture, mise à jour (nom, locale, une feuille, cellules simples sans source Odoo).
- **Stockage** : Persistance contenu (KindMother) ; pas encore d’intégration Documents (dossier, partage) si lourd.
- **UI** : Grille minimale, barre de formule, une feuille, pas de panneau Data.
- **Pas encore** : Listes/pivots/graphiques Odoo, formules ODOO.LIST/PIVOT, templates, versions, partage.

**Livrable :** Classeur éditable avec cellules et formules standard (SUM, IF, etc.) si moteur côté serveur ; ou intégration minimale d’un moteur type o-spreadsheet (lecture/écriture JSON).

### Phase 2 — Sources de Données (Liste)

- **DataSource List** : Insertion d’une liste depuis un Opérateur métier (vue liste) ; model, domain, sort, columns, row_count.
- **Formules** : ODOO.LIST.HEADER, ODOO.LIST ; résolution côté backend.
- **Panneau Data** : Liste des sources, propriétés liste (domain, tri, colonnes), rafraîchissement.
- **Filtres globaux** : Modèle minimal (nom, valeur) ; application au domain des listes au rafraîchissement.

**Livrable :** Classeur avec listes liées à des vues Odoo-like (Miyukini), formules ODOO.LIST, rafraîchissement.

### Phase 3 — Pivots et Graphiques

- **DataSource Pivot** : Insertion pivot ; dimensions, mesures ; formules PIVOT.HEADER, PIVOT.VALUE, PIVOT (dynamique).
- **DataSource Chart** : Insertion graphique depuis vue graph ; pas de formules cellule, affichage uniquement.
- **Panneau Data** : Propriétés pivot/chart ; duplication/suppression de sources.

**Livrable :** Classeur avec listes, pivots, graphiques ; formules PIVOT ; rafraîchissement global.

### Phase 4 — Fonctions « Odoo » et Comptabilité

- **Fonctions Miyukini** : Équivalents ODOO.BALANCE, ODOO.CREDIT, ODOO.DEBIT, ODOO.FISCALYEAR.START/END, ODOO.CURRENCY.RATE, etc. (intégration miyucptaledger / miyuinvoice).
- **Résolution** : Côté serveur avec Mandat et WorrySentinel (niveau sécurité).
- **Documentation** : Liste des fonctions disponibles dans l’UI (aide, palette de commandes).

**Livrable :** Classeur avec formules comptables/financières gouvernées.

### Phase 5 — Templates, Versions, Partage

- **Templates** : Save as template ; création classeur depuis template ; Configuration ‣ Spreadsheet Templates (copie, édition, suppression).
- **Versions** : Sauvegarde automatique ; Version history ; Restore / Copy version ; nommage.
- **Partage** : Intégration Opérateur Documents (Viewer/Editor, freeze and share).
- **Export .xlsx** : Génération fichier avec formules métier converties en valeurs.

**Livrable :** Expérience complète type Odoo Spreadsheet (templates, versions, partage, export).

### Phase 6 — Dashboards et Optimisations

- **Conversion dashboard** : File ‣ Add to dashboard ; création entité dashboard ; premier onglet = face avant.
- **Performance** : Cache rafraîchissement, pagination ou limite sur grosses listes/pivots.
- **UX** : Raccourcis clavier (palette type Ctrl+K), messages d’erreur explicites (source inaccessible, permission).

**Livrable :** Tableaux de bord dérivés des classeurs ; performance et UX renforcées.

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

- Création / édition d’un classeur avec une feuille et des cellules.
- Formules standard (SUM, IF, etc.) si moteur le permet.
- Persistance (KindMother) ; pas de listes/pivots/graphiques Odoo, pas de templates, pas de versions, pas de partage.

### 5.2 Complet (Phases 2–6)

- Listes, pivots, graphiques insérés depuis les Opérateurs métier.
- Formules ODOO.LIST, PIVOT et fonctions comptables (BALANCE, etc.).
- Filtres globaux.
- Templates (création, utilisation, gestion).
- Historique des versions (restauration, copie).
- Partage (Viewer/Editor, freeze and share).
- Export .xlsx.
- Conversion en dashboard.
- Intégration Documents (dossiers, corbeille) selon périmètre.

### 5.3 Hors Périmètre (V1)

- Édition collaborative temps réel (type Google Sheets) : non exclu mais non engagé en V1.
- Formules personnalisées par l’utilisateur (définition de nouvelles fonctions) : non prévu en V1.
- Import .xlsx avec conservation des formules Odoo : complexe ; à traiter en phase ultérieure si besoin.

---

**Document créé le :** 2026-02-01
