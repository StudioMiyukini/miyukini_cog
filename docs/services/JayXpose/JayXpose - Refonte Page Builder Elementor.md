# JayXpose - Refonte Page Builder (Reference Elementor)

## 1. Contexte

Objectif: faire de XP-E07 un vrai builder operable, avec personnalisation bloc par bloc, persistance fiable et previsualisation utile.

Problemes constates avant refonte:
- ajout de blocs quasi vides
- pas de panneau de personnalisation exploitable
- previsualisation peu utile
- friction forte entre edition et rendu

## 2. Reference UX Elementor retenue

Sources web consultees (officielles + guide pratique):
- https://elementor.com/help/elementor-editor/
- https://elementor.com/help/elementor-navigator/
- https://elementor.com/help/what-is-the-template-library/
- https://elementor.com/help/site-settings/
- https://kinsta.com/blog/how-to-use-elementor/

Mecaniques cibles extraites:
- panneau gauche fixe
- canvas central editable
- inspecteur proprietes par bloc (onglets Content / Style / Advanced)
- navigator (arbre/ordre des elements)
- templates prets a l'emploi
- actions de duplication/suppression/reordonnancement
- preview device desktop/tablet/mobile

## 3. Analyse des captures fournies

Elements UX a reproduire:
- `15-ajouter-une-section` / `17-ajouter-widget`: entree rapide par widgets
- `16-personnaliser-la-section`: edition immediate des proprietes
- `19-continuer-layout`: edition d'un bouton (texte/lien/alignment/size)
- `25-reglages-caches` / `26-reglages-caches-2`: parametres globaux site (couleurs, typo)
- `13-yoast-modeles`: bibliotheque templates
- menu contextuel (dupliquer/copier/supprimer) + navigator

## 4. Architecture UI cible XP-E07

### 4.1 Disposition

- Header: navigation, preview, save
- Toolbar: actions builder + switch device
- Colonne 1 (gauche): librairie widgets + recherche
- Colonne 2 (centre): canvas + navigator visuel + ordre blocs
- Colonne 3 (droite): proprietes bloc selectionne

### 4.2 Proprietes bloc

Onglets:
- `Content`: donnees metier du bloc
- `Style`: couleurs/alignement/typo/espacements
- `Advanced`: classes, ancre, marges techniques

### 4.3 Operations bloc

- ajouter
- selectionner
- dupliquer
- supprimer
- deplacer haut/bas

## 5. Data model builder

Type principal:
- `PageBuilderDocument`
- version
- page_id
- title
- slug
- settings
- blocks[]

Type bloc:
- `PageBuilderBlock`
- id
- block_type
- props JSON

Types bloc MVP:
- hero
- heading
- text
- button
- image
- product_grid
- features
- faq

## 6. Contrat de props par bloc

### 6.1 hero

- `title`: string
- `subtitle`: string
- `cta_text`: string
- `cta_link`: string
- `align`: left|center|right
- `color`: hex
- `bg_color`: hex
- `font_size`: i32
- `padding`: i32

### 6.2 heading

- `text`, `tag`, `align`, `color`, `font_size`, `padding`

### 6.3 text

- `content`, `align`, `color`, `font_size`, `padding`

### 6.4 button

- `text`, `link`, `size`, `align`, `color`, `bg_color`, `font_size`, `padding`

### 6.5 image

- `url`, `alt`, `align`, `padding`

### 6.6 product_grid

- `limit`, `columns`, `show_price`, `show_button`, `padding`

### 6.7 features

- `items_text` (1 item par ligne)
- `padding`

### 6.8 faq

- `qa_text` (lignes Q:/R:)
- `padding`

## 7. Persistance

### 7.1 Stockage

- snapshot document: `vitrine_pages.content`
- structure bloc: `vitrine_blocs`

### 7.2 Sauvegarde

1. construire `PageBuilderDocument`
2. upsert page via `vitrine_page_upsert_return_id`
3. convertir blocs en `VitrineBlock`
4. `vitrine_blocks_replace`

### 7.3 Chargement

- priorite 1: `vitrine_blocs`
- fallback: parser `vitrine_pages.content`
- fallback final: template mini-site par defaut

## 7.b Gouvernance creation non-SQL

Regle appliquee:
- creation/modification non-SQL (draft in-memory, application template, duplication bloc) = mandat obligatoire
- echec mandat => action refusee
- action acceptee => trace d'audit persistÃ©e via `sync_logs`

ImplÃ©mentation:
- contexte: `governance_mandate_id`, `governance_security_level` dans `ExpState`
- gate: `govern_non_sql_create(...)` dans `crates/jayxpose/src/governance.rs`
- appels depuis XP-E07: ajout bloc, duplication, suppression, application template
- provenance mandat:
  - mode Central: mandat dÃ©rivÃ© du profil connectÃ© (`sf-jayxpose-{profile_id}`, niveau 3)
  - mode standalone: mandat local explicite (`local-jayxpose-standalone`, niveau 1)

## 8. Preview XP-E08

Regles de rendu:
- parser document JSON
- fallback sur `vitrine_pagebuilder_blocks` en memoire
- renderer par `block_type`
- appliquer style de base depuis props (`bg_color`, `color`, `font_size`, `padding`)

Pages:
- Accueil: infos societe
- Catalogue: liste produits
- Presentation: rendu builder complet
- Contact: contenu simple ou futur formulaire

## 9. Etat applicatif requis

Dans `ExpState`:
- `pagebuilder_selected_block_idx`
- `pagebuilder_selected_canvas_idx`
- `pagebuilder_search`
- `pagebuilder_active_tab`
- `pagebuilder_device_preview`
- `vitrine_pagebuilder_blocks`

## 10. Comportements critiques

- toujours avoir une selection valide ou `None`
- aucune edition sans bloc selectionne
- sauvegarde idempotente
- previsualisation jamais vide si blocs existants
- message utilisateur explicite en cas d'erreur persistance

## 11. Plan d'implementation recommande

### Etape 1 - Fondation

- types blocs + props defaults
- helpers lecture/ecriture props JSON

### Etape 2 - UI edition

- panneau widgets
- canvas selectable
- proprietes onglets

### Etape 3 - Persistance

- save/load page + blocs
- templates fallback

### Etape 4 - Preview

- renderer par bloc
- rendu styles basiques

### Etape 5 - Qualite

- tests charge/save/load
- tests rendu bloc
- tests reordonnancement

## 12. Definition of done (M3)

Le builder est considere fonctionnel si:
- l'utilisateur peut ajouter un bloc
- modifier son contenu (titre, texte, CTA)
- modifier des styles de base
- reordonner/supprimer/dupliquer
- sauvegarder et reouvrir sans perte
- voir un rendu lisible en preview

## 13. Evolution v2

- drag and drop natif
- navigator en arbre section/column/widget
- undo/redo
- copy/paste style
- librairie templates avec preview miniatures
- global site settings complets
- responsive controls par device

## 14. Fichiers techniques lies

- `crates/jayxpose/src/screens/exp/e07_vitrine_presentation.rs`
- `crates/jayxpose/src/screens/exp/e08_vitrine_preview.rs`
- `crates/jayxpose/src/screens/exp/mod.rs`
- `crates/jayxpose/src/data/types.rs`
- `crates/jayxpose/src/data/kindmother_db.rs`

## 15. Etat implémente (2026-02-08)

### 15.1 Mécanismes livrés dans XP-E07

- Canvas hiérarchique `Section -> Colonnes -> Widgets`
- Ajout rapide de section via toolbar:
  - `+ Section 1 col`
  - `+ Section 2 col`
  - `+ Section 3 col`
- Insertion widget contextuelle:
  - si colonne sélectionnée, le widget est rattaché à cette colonne (`parent_id`)
- Navigator visuel:
  - sélection section/colonne/widget
  - réordonnancement section
  - suppression avec purge des enfants directs
- Panneau propriétés `Content / Style / Advanced` opérationnel
- Persistance `vitrine_pages` + `vitrine_blocs` conservée
- Gouvernance non-SQL active (mandat requis + audit)

### 15.2 Mécanismes livrés dans XP-E08

- Rendu structuré par section
- Colonnes rendues en layout multi-colonnes
- Widgets rendus dans la colonne parente
- Fallback sans section (compat JSON ancien)

### 15.3 Mapping image -> implémentation

- `15-ajouter-une-section` -> boutons de création section multi-colonnes
- `17-ajouter-widget` -> librairie widgets + insertion contextuelle
- `19-continuer-layout` -> édition CTA bouton (texte/lien/size)
- `25/26-reglages-caches` -> onglets propriétés + styles globaux de base
- `navigator` (capture colonne) -> sélection hiérarchique section/colonne/widget

## 16. Backlog court terme pour parité Elementor

1. Drag-and-drop natif (reorder intra-colonne + inter-colonnes)
2. Copier/coller widget + copier/coller style
3. Undo/Redo complet
4. Responsive controls par device (desktop/tablet/mobile) par propriété
5. Templates library visuelle avec miniatures réelles
6. Popover global site settings (couleurs/typo) appliqué à tous les blocs
7. Menu contextuel clic droit complet (duplicate/copy/paste/delete/navigator)

## 17. Contrat technique de structure

### 17.1 Parentage

- `section`: pas de `parent_id`
- `column`: `parent_id = section.id`
- `widget`: `parent_id = column.id`

### 17.2 Compatibilité descendante

- Si aucune `section` trouvée au rendu, fallback rendu flat (legacy)

### 17.3 Persistance

- Ordre de rendu = ordre `vitrine_blocs.position`
- Chaque bloc sauvegardé avec:
  - `block_key`
  - `block_type`
  - `props_json`

## 18. Lot implémenté: mécanismes Elementor avancés

- [x] Structure editor en `Section -> Colonnes -> Widgets`
- [x] Création rapide de sections multi-colonnes (1/2/3)
- [x] Insertion widget contextuelle dans colonne sélectionnée (`parent_id`)
- [x] Navigator hiérarchique visuel (section/colonne/widgets)
- [x] Undo/Redo (historique snapshots JSON)
- [x] Copier style / Coller style entre widgets
- [x] Template mini-site migré vers structure section/colonnes
- [x] Preview XP-E08 compatible structure hiérarchique

### Détail technique

- `pagebuilder_history` / `pagebuilder_future` dans `ExpState`
- `pagebuilder_style_clipboard` dans `ExpState`
- Fonctions clés XP-E07:
  - `record_snapshot`
  - `undo_builder`
  - `redo_builder`
  - `extract_style_json`
  - `apply_style_json`
  - `add_section_with_columns`
