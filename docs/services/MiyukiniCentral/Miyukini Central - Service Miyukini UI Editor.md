# Miyukini Central â€” Service Miyukini UI Editor

## Contexte

**Miyukini UI Editor** est un **Service** exposÃ© par le Hub Miyukini Central. Il permet la **personnalisation persistante de lâ€™interface** : thÃ¨mes prÃ©fabriquÃ©s (clair / sombre), rÃ©glage des couleurs, des formes (rayons des coins, fenÃªtres) et dâ€™Ã©lÃ©ments de dÃ©mo (angle, widgets). Les modifications sont **sauvegardÃ©es automatiquement** et restaurÃ©es Ã  la prochaine ouverture du Service.

**RÃ´le fondateur :** *Permettre Ã  lâ€™utilisateur de modifier de faÃ§on persistante la plupart des Ã©lÃ©ments UI du Central et des Services (thÃ¨me, couleurs, formes), sans toucher au code.*

**Position :** Service consommÃ© comme les autres (catalogue â†’ lancement sous Mandat) ; pas dâ€™autoritÃ© mÃ©tier, pas de persistance cÃ´tÃ© Cores â€” uniquement prÃ©fÃ©rences UI cÃ´tÃ© client (storage eframe).

---

## PortÃ©e / Scope

- **Applicable Ã  :** Conception, dÃ©veloppement et usage du Service Miyukini UI Editor dans Miyukini Central.
- **Audience :** Architectes, dÃ©veloppeurs, designers UX/UI, utilisateurs finaux.
- **Statut :** Document de rÃ©fÃ©rence â€” service de personnalisation dâ€™interface.
- **Hors scope :** Personnalisation des polices (hors pÃ©rimÃ¨tre actuel), thÃ¨mes des OpÃ©rateurs mÃ©tier (chaque OpÃ©rateur peut avoir son propre thÃ¨me ; lâ€™UI Editor cible le rendu egui/Central).

---

## 1. DÃ©finition et objectifs

### 1.1 Nature du Service

| Aspect | DÃ©finition |
|--------|-------------|
| **Type** | Service (OpÃ©rateur dâ€™Interface ou vue dÃ©diÃ©e au Hub) |
| **RÃ´le** | Personnalisation de lâ€™interface : thÃ¨mes, couleurs, formes, cohÃ©rence visuelle |
| **Question fondamentale** | *Â« Comment adapter lâ€™UI Ã  mes prÃ©fÃ©rences de faÃ§on durable ? Â»* |
| **AutoritÃ©** | Aucune â€” pas de dÃ©cision mÃ©tier, pas dâ€™Ã©criture dans les Cores ; persistance locale (storage client) uniquement |

**Phrase fondatrice :**

> **Miyukini UI Editor permet de modifier de faÃ§on persistante lâ€™UI : thÃ¨mes prÃ©fabriquÃ©s et outils pour adapter la plupart des Ã©lÃ©ments visuels.**

### 1.2 Objectifs fonctionnels

1. **ThÃ¨mes prÃ©fabriquÃ©s**  
   - Mode **clair** et mode **sombre** (base).  
   - Synchronisation des couleurs stockÃ©es lors du bascule (Ã©viter mÃ©lange clair/sombre).

2. **RÃ©glage des Ã©lÃ©ments UI**  
   - **Couleurs :** fenÃªtre/fond, panneaux, widget inactif, survol, actif (palette Ã©ditable).  
   - **Formes :** rayons des coins (widgets, fenÃªtres) en px.  
   - **Angle (dÃ©mo) :** 0â€“360Â° pour dÃ©mo visuelle (rotation / forme).

3. **Persistance**  
   - Sauvegarde automatique du thÃ¨me (mode + couleurs + rayons + angle) dans le storage du Central (clÃ© `miyukini_ui_editor_theme`).  
   - Chargement au premier affichage du Service (onglet ouvert).

4. **AperÃ§u en direct**  
   - Les changements du panneau Â« Options thÃ¨me Â» s'appliquent Ã  **toute l'app** en temps rÃ©el (header, sidebar, onglets, boutons, champs, etc.).  
   - Zone Â« Affichage rÃ©el Â» Ã  droite : rappel du catalogue des Ã©lÃ©ments modifiables et rÃ©glage de l'angle (Â°).

---

## 2. Ã‰lÃ©ments modifiables (outils UI)

### 2.1 RÃ©sumÃ© des Ã©lÃ©ments couverts

| CatÃ©gorie | Ã‰lÃ©ments | Persistant |
|-----------|----------|------------|
| **Mode de base** | Clair / Sombre | Oui |
| **Couleurs â€” Fonds** | FenÃªtre, panneaux, widgets (non interactif, inactif, survol, actif) | Oui |
| **Couleurs â€” Texte** | Texte par Ã©tat, override global, texte faible | Oui |
| **Couleurs â€” Header, Hub, onglets** | Barre exe, barre onglets, sidebars, body, onglets, sÃ©parateur, bouton fermer | Oui (override optionnel) |
| **Formes** | Rayon coins widgets/fenÃªtres (px), angle (Â°) | Oui |

Les modifications s'appliquent Ã  **toute l'app en temps rÃ©el** (affichage rÃ©el, pas de dÃ©mo). Le thÃ¨me est persistÃ© et rechargÃ© Ã  l'ouverture.

### 2.2 Catalogue des Ã©lÃ©ments UI (modification individuelle)

Chaque Ã©lÃ©ment correspond Ã  une propriÃ©tÃ© du thÃ¨me. Override Header/Hub : cocher Â« Appliquer Â» pour utiliser la couleur personnalisÃ©e ; sinon palette Chrome (mode clair/sombre).

**Fonds et panneaux (egui)** : `window_fill`, `panel_fill`, `widget_noninteractive_bg`, `widget_inactive_bg`, `widget_hovered_bg`, `widget_active_bg`.

**Texte / avant-plan** : `widget_noninteractive_fg`, `widget_inactive_fg`, `widget_hovered_fg`, `widget_active_fg`, `override_text_color`, `weak_text_color`.

**Header, Hub, onglets** : `barre_exe_bg`, `tab_bar_bg`, `sidebar_bg`, `body_bg`, `tab_inactive_bg`, `tab_inactive_hover_bg`, `tab_active_text`, `tab_inactive_text`, `tab_separator`, `close_btn_fg`, `close_btn_hover_fg`, `close_btn_hover_bg`.

**Formes** : `corner_radius`, `window_rounding`, `angle_deg`.

### 2.3 Modifications possibles (exhaustif)

**Couleurs** : toutes les couleurs ciâ€‘dessus (fonds par Ã©tat, texte par Ã©tat, override, texte faible, zones Header/Hub/onglets). **Formes** : rayons des coins (px), angle (Â°). **Ã‰tats** : inactif, survol (hover), actif (pressed), non interactif â€” chacun a une couleur de fond et de texte. **Effets** : pas de flou/ombre exposÃ©. **Animations** : pas de rÃ©glage de transition ; changement d'Ã©tat immÃ©diat.

### 2.4 Palette par dÃ©faut (mode clair â€” Central)

En mode clair, le Central utilise une palette beige (hors police) :

| Zone | Couleur (hex) | Usage |
|------|----------------|-------|
| Barre exÃ©cutable (ligne 1 header) | `#d9b99b` | Titre + connexion / profil / config |
| Fond header (ligne 2 â€” onglets) | `#fff0db` | Barre dâ€™onglets |
| Fond sidebars Hub | `#e4d5b7` | Panneau latÃ©ral HUB |

Ces valeurs sont dÃ©finies dans `pixel_theme.rs` (chrome_colors) et utilisÃ©es par le Hub ; lâ€™UI Editor permet en outre de personnaliser les couleurs **dans la zone dÃ©mo** (fenÃªtre, panneaux, widgets) et de rÃ©initialiser le thÃ¨me aux dÃ©fauts.

### 2.5 Ã‰lÃ©ments non couverts (hors scope actuel)

- **Polices** : taille, famille, graisse â€” non modifiables via lâ€™UI Editor pour lâ€™instant.  
- **Layout** : espacements, marges globales â€” non exposÃ©s.  
- **ThÃ¨mes des OpÃ©rateurs mÃ©tier** : chaque Service peut avoir son propre thÃ¨me ; lâ€™UI Editor pilote le thÃ¨me **egui** appliquÃ© dans son onglet (et le dÃ©faut du Central pour le mode clair/sombre cÃ´tÃ© Hub).

---

## 3. Parcours utilisateur

### 3.1 AccÃ¨s au Service

1. Depuis le **HUB** : sÃ©lectionner Â« Miyukini UI Editor Â» dans le catalogue (ou Mes Services).  
2. Cliquer sur **Lancer** : ouverture dâ€™un nouvel onglet Â« Miyukini UI Editor Â».  
3. Le thÃ¨me **persistÃ©** (sâ€™il existe) est chargÃ© ; sinon, thÃ¨me par dÃ©faut (mode sombre).

### 3.2 Utilisation typique

1. **Panneau gauche Â« Options thÃ¨me Â»**  
   - Cocher / dÃ©cocher **Mode sombre (base)** : les couleurs se synchronisent (clair â†” sombre).  
   - Ouvrir **Couleurs** : modifier fenÃªtre, panneaux, widgets (inactif, survol, actif).  
   - Ouvrir **Formes (rayons)** : ajuster coins widgets et fenÃªtres (px).  
   - Ouvrir **Angle (dÃ©mo)** : rÃ©gler lâ€™angle pour la forme tournÃ©e Ã  droite.  
   - **RÃ©initialiser le thÃ¨me** : retour aux valeurs par dÃ©faut (mode sombre).

2. **Zone droite Â« Affichage rÃ©el Â»**  
   - Les rÃ©glages s'appliquent Ã  toute l'app (header, sidebar, onglets, boutons, champs).  
   - Catalogue des Ã©lÃ©ments modifiables et rÃ©glage de l'angle.

3. **Fermeture de lâ€™onglet**  
   - Les changements ont Ã©tÃ© sauvegardÃ©s automatiquement (si au moins une modification a Ã©tÃ© faite).  
   - Ã€ la prochaine ouverture du Service, le thÃ¨me persistÃ© est rechargÃ©.

### 3.3 Lien vers la dÃ©mo egui complÃ¨te

Pour disposer de **tous les outils** egui (About, BÃ©zier, Code Editor, Widget Gallery, Modals, etc.), le Service indique comment lancer la dÃ©mo officielle depuis le dÃ©pÃ´t egui :

- `cd deps/egui`  
- `cargo run --release -p egui_demo_app`  

Voir aussi : [Miyukini UI Editor - Adaptation Demo egui](../../tools/Miyukini%20UI%20Editor%20-%20Adaptation%20Demo%20egui.md).

---

## 4. IntÃ©gration COG et OpÃ©rateurs

### 4.1 Outils et Cores concernÃ©s

Le Service **Miyukini UI Editor** sâ€™appuie sur les mÃªmes mÃ©canismes que les autres Services du Hub :

| Acteur | RÃ´le pour lâ€™UI Editor |
|--------|------------------------|
| **Miyukini Central (Hub)** | Expose le Service dans le catalogue ; ouvre lâ€™onglet sous Mandat ; fournit le storage (eframe) pour la persistance du thÃ¨me. |
| **StrongFather** | DÃ©cision dâ€™autorisation pour lancer le Service (Mandat de Permission). |
| **Master Butler** | Catalogue des Services (dont Miyukini UI Editor) ; dÃ©couverte, mÃ©tadonnÃ©es. |
| **BondingBrother** | MÃ©diation des intentions (ouvrir un Service) vers StrongFather / Master Butler. |

Aucun Core supplÃ©mentaire nâ€™est requis pour lâ€™UI Editor : il sâ€™agit dâ€™un Service dâ€™interface dont la persistance est **locale au client** (prÃ©fÃ©rences UI), conformÃ©ment Ã  la rÃ¨gle Hub Â« prÃ©fÃ©rences locales uniquement Â» et Â« Persistence : via mÃ©canisme client (eframe persistence) Â» (rÃ©f. Miyukini Central Hub Services).

### 4.2 Persistance et gouvernance

- **Pas dâ€™Ã©criture mÃ©tier** : le thÃ¨me ne remonte pas Ã  KindMother ni Ã  aucun Core.  
- **Storage** : clÃ© `miyukini_ui_editor_theme` dans le storage eframe du Central.  
- **Chargement** : Ã  la crÃ©ation de lâ€™onglet (EguiEditorService::from_storage).  
- **Sauvegarde** : aprÃ¨s chaque frame, si le thÃ¨me a Ã©tÃ© modifiÃ© (flag `theme_dirty`), le Hub appelle `persist_if_needed` sur le Service ; lâ€™UI Editor sÃ©rialise le thÃ¨me en JSON et lâ€™enregistre.

---

## 5. SpÃ©cifications techniques (rÃ©sumÃ©)

### 5.1 ImplÃ©mentation

- **Crate** : `miyukini-central`.  
- **Module** : `services::egui_editor`.  
- **Structure** : `EguiEditorService` (Ã©tat : `EditorThemeState`, `theme_dirty`, dÃ©mo texte/slider/combo, etc.).  
- **Trait** : `ServiceUi` (id, title, show, persist_if_needed).  
- **Identifiant** : `ServiceId::EguiEditor` ; titre affichÃ© Â« Miyukini UI Editor Â».

### 5.2 ClÃ© de persistance

- **ClÃ©** : `miyukini_ui_editor_theme`.  
- **Format** : JSON (v3) â€” dark_mode, window_fill, panel_fill, widget_*_bg/fg, override_text_color, weak_text_color, corner_radius, window_rounding, angle_deg, et optionnellement barre_exe_bg, tab_bar_bg, sidebar_bg, body_bg, tab_inactive_bg, tab_inactive_hover_bg, tab_active_text, tab_inactive_text, tab_separator, close_btn_fg, close_btn_hover_fg, close_btn_hover_bg (tableaux [r,g,b]).

### 5.3 ThÃ¨me Central (palette beige mode clair)

DÃ©fini dans `pixel_theme.rs` :

- Barre exe : `#d9b99b` (BARRE_EXE_LIGHT).  
- Header / onglets : `#fff0db` (TAB_BAR_BG_LIGHT).  
- Sidebars Hub : `#e4d5b7` (SIDEBAR_BG_LIGHT).

---

## 6. RÃ©fÃ©rences croisÃ©es

- [Miyukini Conceptual References - Miyukini Central Hub Services](..//..//miyukini-webway-system//reference//_index.md) : rÃ´le du Hub, prÃ©fÃ©rences locales, persistance client.  
- [Miyukini Central - Ecrans et UI](./Miyukini%20Central%20-%20Ecrans%20et%20UI.md) : Ã©crans et UI du Central.  
- [Miyukini UI Editor - Adaptation Demo egui](../../tools/Miyukini%20UI%20Editor%20-%20Adaptation%20Demo%20egui.md) : dÃ©mo egui complÃ¨te (tous les outils).  
- [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) : OpÃ©rateur, Service, Mandat, Cores.

---

**Date de crÃ©ation :** 2026-02-03  
**Version :** 1.1  
**Statut :** Document de rÃ©fÃ©rence â€” Service Miyukini UI Editor.  
**Changelog (1.1) :** Catalogue exhaustif des Ã©lÃ©ments UI, modifications possibles (couleurs, formes, Ã©tats, effets, animations), couleurs Header/Hub/onglets Ã©ditables individuellement (override optionnel), format de persistance v3.

