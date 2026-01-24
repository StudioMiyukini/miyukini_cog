# Module Blocs — Contrat fonctionnel

> Capacité CMS cœur. Composition logique de contenu par blocs structurés : organisation, références, hiérarchie interne.

---

## 1. Responsabilité du module

Le Module Blocs décrit la **composition logique** d'un contenu à partir de blocs structurés. Il organise des blocs, référence du contenu et des médias, et définit une hiérarchie interne simple. Il ne décide jamais du rendu, du layout, de l'ordre visuel, des styles ou de l'édition.

---

## 2. Concepts fondamentaux

### Block

Un **bloc** est une unité logique de composition. Chaque bloc possède :
- Une identité unique (BlockId)
- Un type (BlockType) — opaque, défini par le produit
- Des données (BlockData) — opaque, structure définie par le produit
- Une position dans un arbre de blocs (BlockTree)
- Des références optionnelles vers du contenu (ContentId)
- Des références optionnelles vers des médias (MediaId)

### BlockId

Identifiant unique d'un bloc. Fourni par le kernel (`Id`). Immuable.

### BlockType

Type de bloc (ex. "paragraph", "heading", "image", "quote", "custom"). Format opaque défini par le produit. Le module ne valide pas le type, ne l'interprète pas, ne le transforme pas.

### BlockData

Données spécifiques au bloc. Format opaque (structure définie par le produit). Le module stocke et restitue ces données sans les interpréter.

### BlockTree / BlockNode

Arbre de blocs représentant la structure logique d'un contenu. Chaque nœud (BlockNode) contient :
- Un bloc (Block)
- Une liste ordonnée d'enfants (BlockNode)
- Une référence vers le contenu parent (ContentId)

La structure est un arbre simple : un bloc peut avoir zéro ou plusieurs enfants, un bloc a un seul parent (sauf racine), pas de cycles.

### Relations avec Content

Un bloc appartient à un contenu (ContentId). Un contenu peut avoir zéro ou plusieurs blocs racines. La relation est unidirectionnelle : bloc → contenu.

### Relations avec Media

Un bloc peut référencer zéro ou plusieurs médias (MediaId). La relation est unidirectionnelle : bloc → média. Un média peut être référencé par plusieurs blocs.

---

## 3. Invariants fonctionnels

### Garantis

1. **Identité unique :** Un bloc a un identifiant unique et immuable.
2. **Appartenance contenu :** Un bloc appartient à exactement un contenu.
3. **Structure arborescente :** La structure des blocs est un arbre : un bloc a au plus un parent, pas de cycles.
4. **Ordre logique :** Les enfants d'un bloc sont ordonnés (ordre logique, pas visuel).
5. **Cohérence référentielle :** Les références vers du contenu (ContentId) et des médias (MediaId) pointent vers des entités existantes (validation par le produit).
6. **Données opaques :** Les BlockType et BlockData sont stockés et restitués sans transformation.

### Interdits

1. **Pas de cycles :** Un bloc ne peut pas être son propre ancêtre.
2. **Pas de rendu :** Le module ne génère aucun rendu (HTML, JSON, etc.).
3. **Pas de layout :** Le module ne définit aucun layout, positionnement, responsive.
4. **Pas de styles :** Le module ne gère aucun style, CSS, thème.
5. **Pas d'édition :** Le module ne fournit aucun éditeur, UI, interface.
6. **Pas de logique métier :** Le module ne valide pas les règles business, ne transforme pas les données.
7. **Pas de profondeur imposée :** Aucune limite de profondeur n'est imposée (mais le produit peut en définir une).

---

## 4. Opérations fonctionnelles (conceptuelles)

### Créer un bloc

Créer un nouveau bloc dans un contenu. Spécifier : type, données, contenu parent, position (racine ou enfant d'un bloc existant), références optionnelles vers contenu/médias. Retourne l'identifiant du bloc créé.

### Supprimer un bloc

Supprimer un bloc. Les enfants deviennent des racines (ou sont supprimés selon la politique du produit). Retourne confirmation ou erreur si contraintes.

### Déplacer un bloc

Déplacer un bloc d'un parent à un autre (ou le rendre racine). Vérifie l'absence de cycles. Retourne confirmation ou erreur si cycle détecté.

### Réordonner des enfants

Modifier l'ordre logique des enfants d'un bloc. Spécifier le nouvel ordre (liste d'identifiants). Retourne confirmation.

### Attacher un média

Ajouter une référence vers un média à un bloc. Retourne confirmation ou erreur si média inexistant.

### Détacher un média

Retirer une référence vers un média d'un bloc. Retourne confirmation.

### Lire un bloc

Lire un bloc complet : identité, type, données, parent, enfants, références contenu/médias. Retourne le bloc ou erreur si inexistant.

### Lire la structure complète

Lire l'arbre complet des blocs d'un contenu. Retourne la liste des blocs racines avec leurs sous-arbres.

### Lister les blocs d'un contenu

Lister tous les blocs d'un contenu (aplatis ou arborescents). Filtres optionnels : type, présence de média, etc. Retourne liste de blocs.

### Rechercher des blocs

Rechercher des blocs par critères (type, contenu, média référencé, etc.). Retourne liste de blocs correspondants.

---

## 5. Hors-scope explicite

### Rendu et affichage

- Génération de HTML, Markdown, JSON, XML
- Templates de rendu
- Transformation des données en format de sortie
- Prévisualisation
- Export vers formats externes

### Layout et positionnement

- Définition de layouts (grilles, colonnes, flexbox)
- Positionnement visuel (coordonnées, zones)
- Responsive design
- Breakpoints
- Alignement, espacement visuel

### Styles et thèmes

- Gestion de CSS, styles inline
- Thèmes, palettes de couleurs
- Animations, transitions
- Typographie, polices

### Édition et UI

- Éditeur visuel (WYSIWYG, drag & drop)
- Interface utilisateur
- Formulaires d'édition
- Validation côté client
- Auto-save, undo/redo

### Logique métier

- Validation des règles business
- Transformation des données selon règles métier
- Workflows d'approbation
- Règles conditionnelles (si/alors)
- A/B testing, expérimentations

### Templating et DSL

- Langage de template
- DSL (Domain Specific Language)
- Moteur de rendu conditionnel
- Macros, fonctions de rendu

### Recherche et indexation

- Indexation full-text des blocs
- Recherche sémantique
- Ranking, scoring
- Facettes, filtres avancés

### Permissions et accès

- Qui peut créer/modifier/supprimer des blocs
- Règles d'accès par rôle
- Validation des permissions
- Audit des accès

### Stockage et persistance

- Format de stockage (DB, fichiers)
- Schéma de base de données
- Indexation et optimisation
- Réplication et sauvegarde

---

## 6. Cas d'usage supportés

### Page CMS classique

Composition d'une page avec titre, paragraphes, images, citations. Structure logique : titre → paragraphe → image → paragraphe.

### Landing page

Composition d'une landing page avec sections (hero, features, testimonials, CTA). Chaque section est un bloc, avec sous-blocs (texte, image, bouton).

### Documentation

Composition d'un document technique avec chapitres, sous-chapitres, exemples de code, diagrammes. Hiérarchie logique : chapitre → sous-chapitre → paragraphe → exemple.

### Scène de jeu narratif

Composition d'une scène de jeu avec dialogues, choix, conditions. Structure logique : scène → dialogue → choix → conséquence.

### Article de blog

Composition d'un article avec titre, chapeau, paragraphes, images, citations, call-to-action. Structure logique : titre → chapeau → paragraphe → image → citation → CTA.

---

## 7. Cas d'usage explicitement refusés

### Moteur de rendu

Le module ne génère aucun rendu. Un produit peut utiliser les blocs pour générer du HTML, mais ce n'est pas la responsabilité du module.

### Responsive layout

Le module ne définit aucun layout responsive. Le produit gère les breakpoints, les grilles, les colonnes.

### Animation et transitions

Le module ne gère aucune animation, transition, effet visuel. Le produit applique les animations lors du rendu.

### Règles conditionnelles

Le module ne supporte pas de règles conditionnelles (si/alors, afficher/masquer selon conditions). Le produit interprète les données et applique les règles.

### A/B testing

Le module ne gère pas d'expérimentations, de variantes, de tests A/B. Le produit gère les variantes et sélectionne les blocs à afficher.

### Éditeur visuel

Le module ne fournit aucun éditeur visuel, drag & drop, WYSIWYG. Le produit construit l'éditeur en consommant les opérations du module.

### Validation métier

Le module ne valide pas les règles métier (ex. "un bloc image doit avoir une légende", "un bloc titre ne peut pas être vide"). Le produit valide avant d'appeler le module.

---

## 8. Interactions avec les autres modules SPM

### Content

**Relation :** Un bloc appartient à un contenu (ContentId).

**Opérations :**
- Créer un bloc nécessite un ContentId valide
- Supprimer un contenu peut nécessiter de supprimer ses blocs (politique du produit)
- Lire les blocs d'un contenu

**Dépendance :** Le module Blocs dépend du module Content pour valider l'existence des contenus référencés.

### Media

**Relation :** Un bloc peut référencer des médias (MediaId).

**Opérations :**
- Attacher un média à un bloc nécessite un MediaId valide
- Supprimer un média peut nécessiter de détacher les références (politique du produit)
- Rechercher les blocs référençant un média

**Dépendance :** Le module Blocs dépend du module Media pour valider l'existence des médias référencés.

### Publication

**Relation :** Indirecte via Content. Un bloc appartient à un contenu, qui peut avoir un statut de publication.

**Opérations :**
- Aucune opération directe. Le produit gère la cohérence entre statut de publication du contenu et visibilité des blocs.

**Dépendance :** Aucune dépendance directe. Le produit orchestre la cohérence.

### Hierarchy

**Relation :** Aucune relation directe. Le module Blocs gère sa propre hiérarchie interne (arbre de blocs), indépendante de la hiérarchie de contenus.

**Opérations :**
- Aucune opération croisée. Les deux hiérarchies sont indépendantes.

**Dépendance :** Aucune dépendance.

---

## 9. Règles d'évolution

### Quand on pourra ajouter

**Nouveau concept :**
- Si besoin partagé par ≥2 produits CMS
- Si responsabilité strictement fonctionnelle (pas de métier, pas de rendu)
- Si dépendances claires et unidirectionnelles

**Exemples acceptables :**
- Métadonnées de bloc (format opaque)
- Références vers d'autres blocs (liens logiques)
- Tags ou labels sur blocs (format opaque)

**Nouvelle capacité :**
- Si opération fonctionnelle pure (CRUD, navigation, recherche)
- Si pas de logique métier
- Si pas de rendu, layout, style

**Exemples acceptables :**
- Dupliquer un bloc (avec ses enfants)
- Fusionner des blocs (concaténation logique)
- Recherche par contenu de BlockData (full-text hors scope, mais recherche par structure acceptable)

### Quand on devra REFUSER

**Logique métier :**
- Validation de règles business
- Transformation selon règles métier
- Workflows applicatifs

**Rendu et affichage :**
- Génération de HTML, Markdown, JSON
- Templates, moteurs de rendu
- Prévisualisation

**Layout et styles :**
- Définition de layouts, grilles, colonnes
- Gestion de CSS, styles
- Responsive design

**Édition et UI :**
- Éditeur visuel, drag & drop
- Interface utilisateur
- Formulaires d'édition

**Templating et DSL :**
- Langage de template
- DSL spécifique
- Macros, fonctions de rendu

**Anticipation :**
- Fonctionnalités "au cas où"
- Capacités non demandées par ≥2 produits
- Optimisations prématurées

---

## Mini résumé erreurs / dérives évitées

### 1. Dérive vers le page builder

**Piège :** Transformer le module en page builder avec drag & drop, layouts, responsive, styles.

**Évitement :** Contrat strict : pas de rendu, pas de layout, pas de styles. Le module décrit la structure logique, le produit gère le rendu.

### 2. Dérive vers le moteur de template

**Piège :** Ajouter un langage de template, des macros, des fonctions de rendu conditionnel.

**Évitement :** Hors-scope explicite : pas de templating, pas de DSL. Les BlockData sont opaques, le produit interprète.

### 3. Dérive vers la logique métier

**Piège :** Valider des règles business (ex. "un bloc image doit avoir une légende", "un bloc titre ne peut pas être vide"), gérer des workflows.

**Évitement :** Hors-scope explicite : pas de logique métier, pas de validation business. Le produit valide avant d'appeler le module.

### 4. Dérive vers l'éditeur visuel

**Piège :** Fournir un éditeur WYSIWYG, drag & drop, interface utilisateur intégrée.

**Évitement :** Hors-scope explicite : pas d'édition, pas d'UI. Le module expose des opérations fonctionnelles, le produit construit l'éditeur.

### 5. Anticipation et bloat

**Piège :** Ajouter des fonctionnalités "au cas où" (A/B testing, règles conditionnelles, animations) avant qu'elles ne soient demandées par ≥2 produits.

**Évitement :** Règles d'évolution strictes : besoin partagé par ≥2 produits, responsabilité fonctionnelle pure. Refus explicite de l'anticipation.
