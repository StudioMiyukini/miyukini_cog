# Miyukini Central — Service Miyukini UI Editor

## Contexte

**Miyukini UI Editor** est un **Service** exposé par le Hub Miyukini Central. Il permet la **personnalisation persistante de l’interface** : thèmes préfabriqués (clair / sombre), réglage des couleurs, des formes (rayons des coins, fenêtres) et d’éléments de démo (angle, widgets). Les modifications sont **sauvegardées automatiquement** et restaurées à la prochaine ouverture du Service.

**Rôle fondateur :** *Permettre à l’utilisateur de modifier de façon persistante la plupart des éléments UI du Central et des Services (thème, couleurs, formes), sans toucher au code.*

**Position :** Service consommé comme les autres (catalogue → lancement sous Mandat) ; pas d’autorité métier, pas de persistance côté Cores — uniquement préférences UI côté client (storage eframe).

---

## Portée / Scope

- **Applicable à :** Conception, développement et usage du Service Miyukini UI Editor dans Miyukini Central.
- **Audience :** Architectes, développeurs, designers UX/UI, utilisateurs finaux.
- **Statut :** Document de référence — service de personnalisation d’interface.
- **Hors scope :** Personnalisation des polices (hors périmètre actuel), thèmes des Opérateurs métier (chaque Opérateur peut avoir son propre thème ; l’UI Editor cible le rendu egui/Central).

---

## 1. Définition et objectifs

### 1.1 Nature du Service

| Aspect | Définition |
|--------|-------------|
| **Type** | Service (Opérateur d’Interface ou vue dédiée au Hub) |
| **Rôle** | Personnalisation de l’interface : thèmes, couleurs, formes, cohérence visuelle |
| **Question fondamentale** | *« Comment adapter l’UI à mes préférences de façon durable ? »* |
| **Autorité** | Aucune — pas de décision métier, pas d’écriture dans les Cores ; persistance locale (storage client) uniquement |

**Phrase fondatrice :**

> **Miyukini UI Editor permet de modifier de façon persistante l’UI : thèmes préfabriqués et outils pour adapter la plupart des éléments visuels.**

### 1.2 Objectifs fonctionnels

1. **Thèmes préfabriqués**  
   - Mode **clair** et mode **sombre** (base).  
   - Synchronisation des couleurs stockées lors du bascule (éviter mélange clair/sombre).

2. **Réglage des éléments UI**  
   - **Couleurs :** fenêtre/fond, panneaux, widget inactif, survol, actif (palette éditable).  
   - **Formes :** rayons des coins (widgets, fenêtres) en px.  
   - **Angle (démo) :** 0–360° pour démo visuelle (rotation / forme).

3. **Persistance**  
   - Sauvegarde automatique du thème (mode + couleurs + rayons + angle) dans le storage du Central (clé `miyukini_ui_editor_theme`).  
   - Chargement au premier affichage du Service (onglet ouvert).

4. **Aperçu en direct**  
   - Les changements du panneau « Options thème » s'appliquent à **toute l'app** en temps réel (header, sidebar, onglets, boutons, champs, etc.).  
   - Zone « Affichage réel » à droite : rappel du catalogue des éléments modifiables et réglage de l'angle (°).

---

## 2. Éléments modifiables (outils UI)

### 2.1 Résumé des éléments couverts

| Catégorie | Éléments | Persistant |
|-----------|----------|------------|
| **Mode de base** | Clair / Sombre | Oui |
| **Couleurs — Fonds** | Fenêtre, panneaux, widgets (non interactif, inactif, survol, actif) | Oui |
| **Couleurs — Texte** | Texte par état, override global, texte faible | Oui |
| **Couleurs — Header, Hub, onglets** | Barre exe, barre onglets, sidebars, body, onglets, séparateur, bouton fermer | Oui (override optionnel) |
| **Formes** | Rayon coins widgets/fenêtres (px), angle (°) | Oui |

Les modifications s'appliquent à **toute l'app en temps réel** (affichage réel, pas de démo). Le thème est persisté et rechargé à l'ouverture.

### 2.2 Catalogue des éléments UI (modification individuelle)

Chaque élément correspond à une propriété du thème. Override Header/Hub : cocher « Appliquer » pour utiliser la couleur personnalisée ; sinon palette Chrome (mode clair/sombre).

**Fonds et panneaux (egui)** : `window_fill`, `panel_fill`, `widget_noninteractive_bg`, `widget_inactive_bg`, `widget_hovered_bg`, `widget_active_bg`.

**Texte / avant-plan** : `widget_noninteractive_fg`, `widget_inactive_fg`, `widget_hovered_fg`, `widget_active_fg`, `override_text_color`, `weak_text_color`.

**Header, Hub, onglets** : `barre_exe_bg`, `tab_bar_bg`, `sidebar_bg`, `body_bg`, `tab_inactive_bg`, `tab_inactive_hover_bg`, `tab_active_text`, `tab_inactive_text`, `tab_separator`, `close_btn_fg`, `close_btn_hover_fg`, `close_btn_hover_bg`.

**Formes** : `corner_radius`, `window_rounding`, `angle_deg`.

### 2.3 Modifications possibles (exhaustif)

**Couleurs** : toutes les couleurs ci‑dessus (fonds par état, texte par état, override, texte faible, zones Header/Hub/onglets). **Formes** : rayons des coins (px), angle (°). **États** : inactif, survol (hover), actif (pressed), non interactif — chacun a une couleur de fond et de texte. **Effets** : pas de flou/ombre exposé. **Animations** : pas de réglage de transition ; changement d'état immédiat.

### 2.4 Palette par défaut (mode clair — Central)

En mode clair, le Central utilise une palette beige (hors police) :

| Zone | Couleur (hex) | Usage |
|------|----------------|-------|
| Barre exécutable (ligne 1 header) | `#d9b99b` | Titre + connexion / profil / config |
| Fond header (ligne 2 — onglets) | `#fff0db` | Barre d’onglets |
| Fond sidebars Hub | `#e4d5b7` | Panneau latéral HUB |

Ces valeurs sont définies dans `pixel_theme.rs` (chrome_colors) et utilisées par le Hub ; l’UI Editor permet en outre de personnaliser les couleurs **dans la zone démo** (fenêtre, panneaux, widgets) et de réinitialiser le thème aux défauts.

### 2.5 Éléments non couverts (hors scope actuel)

- **Polices** : taille, famille, graisse — non modifiables via l’UI Editor pour l’instant.  
- **Layout** : espacements, marges globales — non exposés.  
- **Thèmes des Opérateurs métier** : chaque Service peut avoir son propre thème ; l’UI Editor pilote le thème **egui** appliqué dans son onglet (et le défaut du Central pour le mode clair/sombre côté Hub).

---

## 3. Parcours utilisateur

### 3.1 Accès au Service

1. Depuis le **HUB** : sélectionner « Miyukini UI Editor » dans le catalogue (ou Mes Services).  
2. Cliquer sur **Lancer** : ouverture d’un nouvel onglet « Miyukini UI Editor ».  
3. Le thème **persisté** (s’il existe) est chargé ; sinon, thème par défaut (mode sombre).

### 3.2 Utilisation typique

1. **Panneau gauche « Options thème »**  
   - Cocher / décocher **Mode sombre (base)** : les couleurs se synchronisent (clair ↔ sombre).  
   - Ouvrir **Couleurs** : modifier fenêtre, panneaux, widgets (inactif, survol, actif).  
   - Ouvrir **Formes (rayons)** : ajuster coins widgets et fenêtres (px).  
   - Ouvrir **Angle (démo)** : régler l’angle pour la forme tournée à droite.  
   - **Réinitialiser le thème** : retour aux valeurs par défaut (mode sombre).

2. **Zone droite « Affichage réel »**  
   - Les réglages s'appliquent à toute l'app (header, sidebar, onglets, boutons, champs).  
   - Catalogue des éléments modifiables et réglage de l'angle.

3. **Fermeture de l’onglet**  
   - Les changements ont été sauvegardés automatiquement (si au moins une modification a été faite).  
   - À la prochaine ouverture du Service, le thème persisté est rechargé.

### 3.3 Lien vers la démo egui complète

Pour disposer de **tous les outils** egui (About, Bézier, Code Editor, Widget Gallery, Modals, etc.), le Service indique comment lancer la démo officielle depuis le dépôt egui :

- `cd deps/egui`  
- `cargo run --release -p egui_demo_app`  

Voir aussi : [Miyukini UI Editor - Adaptation Demo egui](../../tools/Miyukini%20UI%20Editor%20-%20Adaptation%20Demo%20egui.md).

---

## 4. Intégration COG et Opérateurs

### 4.1 Outils et Cores concernés

Le Service **Miyukini UI Editor** s’appuie sur les mêmes mécanismes que les autres Services du Hub :

| Acteur | Rôle pour l’UI Editor |
|--------|------------------------|
| **Miyukini Central (Hub)** | Expose le Service dans le catalogue ; ouvre l’onglet sous Mandat ; fournit le storage (eframe) pour la persistance du thème. |
| **StrongFather** | Décision d’autorisation pour lancer le Service (Mandat de Permission). |
| **Master Butler** | Catalogue des Services (dont Miyukini UI Editor) ; découverte, métadonnées. |
| **BondingBrother** | Médiation des intentions (ouvrir un Service) vers StrongFather / Master Butler. |

Aucun Core supplémentaire n’est requis pour l’UI Editor : il s’agit d’un Service d’interface dont la persistance est **locale au client** (préférences UI), conformément à la règle Hub « préférences locales uniquement » et « Persistence : via mécanisme client (eframe persistence) » (réf. Miyukini Central Hub Services).

### 4.2 Persistance et gouvernance

- **Pas d’écriture métier** : le thème ne remonte pas à KindMother ni à aucun Core.  
- **Storage** : clé `miyukini_ui_editor_theme` dans le storage eframe du Central.  
- **Chargement** : à la création de l’onglet (EguiEditorService::from_storage).  
- **Sauvegarde** : après chaque frame, si le thème a été modifié (flag `theme_dirty`), le Hub appelle `persist_if_needed` sur le Service ; l’UI Editor sérialise le thème en JSON et l’enregistre.

---

## 5. Spécifications techniques (résumé)

### 5.1 Implémentation

- **Crate** : `miyukini-central`.  
- **Module** : `services::egui_editor`.  
- **Structure** : `EguiEditorService` (état : `EditorThemeState`, `theme_dirty`, démo texte/slider/combo, etc.).  
- **Trait** : `ServiceUi` (id, title, show, persist_if_needed).  
- **Identifiant** : `ServiceId::EguiEditor` ; titre affiché « Miyukini UI Editor ».

### 5.2 Clé de persistance

- **Clé** : `miyukini_ui_editor_theme`.  
- **Format** : JSON (v3) — dark_mode, window_fill, panel_fill, widget_*_bg/fg, override_text_color, weak_text_color, corner_radius, window_rounding, angle_deg, et optionnellement barre_exe_bg, tab_bar_bg, sidebar_bg, body_bg, tab_inactive_bg, tab_inactive_hover_bg, tab_active_text, tab_inactive_text, tab_separator, close_btn_fg, close_btn_hover_fg, close_btn_hover_bg (tableaux [r,g,b]).

### 5.3 Thème Central (palette beige mode clair)

Défini dans `pixel_theme.rs` :

- Barre exe : `#d9b99b` (BARRE_EXE_LIGHT).  
- Header / onglets : `#fff0db` (TAB_BAR_BG_LIGHT).  
- Sidebars Hub : `#e4d5b7` (SIDEBAR_BG_LIGHT).

---

## 6. Références croisées

- [Miyukini Conceptual References - Miyukini Central Hub Services](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Central%20Hub%20Services.md) : rôle du Hub, préférences locales, persistance client.  
- [Miyukini Central - Ecrans et UI](./Miyukini%20Central%20-%20Ecrans%20et%20UI.md) : écrans et UI du Central.  
- [Miyukini UI Editor - Adaptation Demo egui](../../tools/Miyukini%20UI%20Editor%20-%20Adaptation%20Demo%20egui.md) : démo egui complète (tous les outils).  
- [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) : Opérateur, Service, Mandat, Cores.

---

**Date de création :** 2026-02-03  
**Version :** 1.1  
**Statut :** Document de référence — Service Miyukini UI Editor.  
**Changelog (1.1) :** Catalogue exhaustif des éléments UI, modifications possibles (couleurs, formes, états, effets, animations), couleurs Header/Hub/onglets éditables individuellement (override optionnel), format de persistance v3.
