# Audit — Moteur de Rendu 2D Isométrique MGE

**Date :** 2025-02-23  
**Périmètre :** Pipeline graphique Allumina Prototype (mge/examples/allumina_prototype)  
**Contexte :** Rendu 2D isométrique, ECS, wgpu, séparation simulation/rendu

---

## 1. Synthèse des Problèmes Identifiés

| Problème | Cause racine | Gravité | Priorité |
|----------|--------------|---------|----------|
| Sprites entités cropés | `copy_sprite_to_atlas` force 100×100, crop si source différent | Haute | P1 |
| Sprites entités mal positionnés | `draw_sprites` ne gère pas le pivot BottomCenter | Haute | P1 |
| GUI sprites mal affichés | Même pipeline que monde, pas d'espace coordonnées GUI dédié | Moyenne | P2 |
| Texte non visible/tronqué | `load_font()` Windows-only, atlas 128px limité, métriques mal utilisées | Haute | P1 |
| Z-sorting incorrect | Pas de Y-sort sur entity_sprites, ordre ECS arbitraire | Haute | P1 |
| Artifacts alignement pixel | Pas de half-pixel offset, NDC float brut, FilterMode::Nearest sans snap | Moyenne | P2 |

---

## 2. Architecture de la Pipeline Actuelle

### 2.1 Flux de Rendu (draw_tilemap)

```
1. Layer 0 : Tuiles (grass atlas)     → pipeline sprite.wgsl, vertex buffer
2. Layer 1 : Entités (character atlas) → instanced_pipeline, quad + instances
3. Layer 2 : Overlays monde           → colored_pipeline (grille, pathfinding, hitboxes, vision)
4. Layer 3 : GUI (quads colorés)      → colored_pipeline
5. Layer 4 : Texte Dev (labels)       → instanced_pipeline + text_bind_group
```

### 2.2 Fichiers Clés

| Fichier | Responsabilité |
|---------|----------------|
| `renderer.rs` | WgpuRenderer, draw_tilemap, build_dev_overlay_rects, atlas |
| `isometric.rs` | world_to_screen, IsoCamera.project, visible_tile_bounds |
| `dev_text.rs` | build_dev_text_atlas, fontdue, LabelRect |
| `dev_ui.rs` | DevState, HitRect, options panneau |
| `shaders/sprite_instanced.wgsl` | Quad instancing, NDC |
| `shaders/colored_quad.wgsl` | Quads colorés sans texture |

### 2.3 Espaces de Coordonnées

- **Monde :** (world_x, world_y) en unités tuile, centre tuile = +0.5
- **Écran (avant caméra) :** `screen_x = (wx - wy) * TILE_HALF_W`, `screen_y = (wx + wy) * TILE_HALF_H`
- **Viewport :** `(screen - cam_center) * zoom + viewport/2`
- **NDC (shader) :** `corner / view_size * 2.0 - 1.0`, `y = -y`

---

## 3. Causes Racines Détaillées

### 3.1 Sprites Entités Cropés

**Localisation :** `renderer.rs:938-976` — `copy_sprite_to_atlas`

```rust
let copy_w = 100.min(w);
let copy_h = 100.min(h);
// ...
for y in 0..copy_h {
    for x in 0..copy_w {
        // Copie vers slot 100px fixe
```

**Problème :** L'atlas personnages est fixé à 300×100 (3 slots de 100×100). Si l'asset source (`Test_joueur.png`, Soldier, Archer) a des dimensions différentes (ex. 64×128, 100×150), seul le rectangle (0,0)-(min(100,w), min(100,h)) est copié. Le reste est perdu.

**Standard moteurs :** Godot/Unity utilisent des métadonnées par sprite (frame rect) ; l'atlas s'adapte ou les sources sont validés à l'import.

**Correctif :** Valider dimensions à l'import ou supporter des tailles variables par slot dans l'atlas (padding, métadonnées).

---

### 3.2 Sprites Entités Mal Positionnés

**Localisation :** `renderer.rs:1200-1212` — `draw_sprites`

```rust
let (sx, sy) = camera.project(s.world_x, s.world_y, view_size.0, view_size.1);
InstanceRaw {
    screen_pos: [sx, sy],  // ← BUG: traité comme coin haut-gauche
    size: [s.size_w, s.size_h],
    uv_rect: [...],
}
```

**Problème :** `camera.project(wx, wy)` retourne la position écran du point monde (typiquement centre/pieds). Le shader interprète `screen_pos` comme coin haut-gauche du quad (`corner = position * size + instance_pos`). Pour un pivot BottomCenter (convention MGE), il faudrait :

```rust
screen_pos: [sx - s.size_w / 2.0, sy - s.size_h]
```

**Comparaison :** Dans `draw_tilemap` (l.707-724), l'ajustement est correct :

```rust
let top_left_y = sy - entity_size;
screen_pos: [sx - half_w, top_left_y]
```

Donc `draw_sprites` est incohérent avec `draw_tilemap`.

---

### 3.3 GUI Sprites Mal Affichés

**Problème :** La GUI (bouton Dev, panneau, métriques) utilise les mêmes quads colorés et le même espace de coordonnées que les overlays monde. Les positions sont en pixels écran (`viewport_w`, `viewport_h`), ce qui est correct. Les causes possibles :

1. **Ordre de draw :** Texte après quads colorés — correct. Mais si `text_instances` est vide (police non chargée), les labels ne s'affichent pas.
2. **Overflow :** Les panneaux métriques/log ont des dimensions fixes ; si `viewport` est petit, ils peuvent déborder ou se chevaucher.
3. **Pas de scaling UI :** Pas de facteur d'échelle DPI/ratio pour les tailles GUI sur écrans haute résolution.

**Standard :** Godot/Unity ont un système UI avec anchors, containers, et scale factor. Le MGE utilise des coordonnées brutes.

---

### 3.4 Texte Non Visible ou Tronqué

**Localisation :** `dev_text.rs:16-41` — `load_font()`

```rust
fn load_font() -> Option<Font> {
    let paths = [
        std::env::var("WINDIR").ok().map(|w| Path::new(&w).join("Fonts").join("arial.ttf")),
        // ... segoeui, consola
    ];
```

**Problèmes :**

1. **Windows-only :** Sur Linux/Mac, `WINDIR` n'existe pas → `load_font()` = None → `build_dev_text_atlas()` retourne `(vec![], 0, 0, vec![])` → pas de texture → `dev_label_rects` vide → texte jamais ajouté.
2. **Atlas 128px :** `atlas_w = 128` initial ; si les labels dépassent, le layout fait des retours à la ligne. Le calcul `cursor_x + total_w > atlas_w` peut mal placer les glyphes.
3. **Métriques fontdue :** `font.rasterize(*label, FONT_SIZE)` — `metrics.width/height` peuvent être 0 pour certains glyphes (police corrompue, caractères spéciaux).
4. **Pas de métriques avancées :** Pas de baseline, kern, ou hinting. Le texte peut sembler tronqué si la hauteur de ligne est trop petite.

**Correctif :** Font embarquée (assets), chemins multiplateforme, ou fontdue fontes packagées. Augmenter atlas si nécessaire. Valider `metrics` avant placement.

---

### 3.5 Z-Sorting (Profondeur) Incorrect

**Localisation :** `main.rs:76-78` — collecte entity_sprites

```rust
for (_, pos, sprite) in engine.world().iter2::<Position2D, EntitySprite>() {
    entity_sprites.push((pos.x, pos.y, sprite.character_id));
}
```

**Problème :** Aucun tri. L'ordre est celui de l'itération ECS (arbitraire). En isométrique, les entités avec un Y monde plus grand (plus "au sud") doivent être dessinées au-dessus pour l'occlusion correcte.

**Spécification MGE :** `docs/.../z-order-couches.md` — Y-sort requis :

- `sort_key = -world_y` (plus Y grand = dessiné après = devant)
- Convention : bas écran = proche, haut = loin

**Correctif :**

```rust
entity_sprites.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
```

Ou tri par `(world_x + world_y)` pour une approximation isométrique.

---

### 3.6 Artifacts d'Alignement Pixel

**Localisation :** Shaders `sprite_instanced.wgsl`, `sprite.wgsl`

```wgsl
out.clip_position = vec4<f32>(corner / view_size * 2.0 - 1.0, 0.0, 1.0);
```

**Problèmes :**

1. **Pas de half-pixel offset :** En NDC, les positions float peuvent tomber entre pixels. Les moteurs classiques (OpenGL legacy, DirectX) appliquent souvent un offset (0.5/width, 0.5/height) pour aligner le centre du texel au centre du pixel.
2. **FilterMode::Nearest :** Correct pour pixel art, mais combiné à des positions non snapées, crée du shimmer/wobble lors du scroll.
3. **Pas de snap caméra :** La caméra suit le joueur avec `camera.center_x/y` en float. Pas de rounding pour pixel-perfect.
4. **Zoom non entier :** `camera.zoom = 1.4` — scaling non entier peut produire des artefacts de moiré.

**Référence :** [Pixel Perfect Rendering (Slow Rush)](https://slowrush.dev/news/pixel-perfect-rendering), Unity `PixelPerfectRendering.pixelSnapSpacing`, Godot pixel snap.

---

## 4. Comparaison avec Moteurs Modernes

| Aspect | MGE (Allumina) | Godot 2D | Unity 2D | Bevy 2D |
|--------|----------------|----------|----------|---------|
| Y-sort entités | ❌ Aucun | ✅ YSort node | ✅ Order in Layer + Sort Layer | ✅ Z-index + spawn order |
| Pivot sprite | Partiel (draw_tilemap only) | ✅ Texture rect + offset | ✅ Pivot (0-1) | ✅ Anchor |
| Texte | fontdue, atlas custom | Label node, TTF | TextMeshPro, UGUI | Bevy_text, fontdue |
| Pixel perfect | ❌ | ✅ Option projet | ✅ PixelPerfectCamera | Via viewport scale |
| GUI / Monde | Même pipeline | CanvasLayer séparé | Screen Space Overlay | Separate pass |
| Depth buffer | ❌ | Optionnel 2D | Optionnel | Optionnel |

---

## 5. Correctifs Proposés

### 5.1 P1 — Sprites (position, pivot)

**Fichier :** `renderer.rs`

Dans `draw_sprites`, aligner sur le comportement de `draw_tilemap` :

```rust
let half_w = s.size_w / 2.0;
let top_left_y = sy - s.size_h;  // pivot BottomCenter
InstanceRaw {
    screen_pos: [sx - half_w, top_left_y],
    size: [s.size_w, s.size_h],
    uv_rect: [s.uv_min.0, s.uv_min.1, s.uv_max.0, s.uv_max.1],
}
```

Ajouter un paramètre `pivot` (TopLeft, Center, BottomCenter) si besoin de flexibilité.

---

### 5.2 P1 — Atlas sprites (éviter crop)

**Fichier :** `renderer.rs` — `copy_sprite_to_atlas`

- Option A : Valider à l'import que les assets font 100×100, erreur sinon.
- Option B : Supporter des rects variables par slot, avec métadonnées (JSON) ou déduction des dimensions réelles.
- Option C : Redimensionner (scale) si w/h ≠ 100, avec avertissement de qualité.

Recommandation : A court terme (validation + log) ; B moyen terme (métadonnées).

---

### 5.3 P1 — Y-sort entités

**Fichier :** `main.rs` — fonction `render_frame`

Avant le passage à `draw_tilemap` :

```rust
entity_sprites.sort_by(|a, b| {
    // Isométrique : (x+y) croissant = dessiné avant = derrière
    let sum_a = a.0 + a.1;
    let sum_b = b.0 + b.1;
    sum_a.partial_cmp(&sum_b).unwrap_or(Ordering::Equal)
});
```

Alternative avec tie-breaker : `then_with(|| a.0.partial_cmp(&b.0))` pour stabilité.

---

### 5.4 P1 — Texte (police, plateformes)

**Fichier :** `dev_text.rs`

1. **Font embarquée :** Inclure une TTF dans `assets/fonts/` et la charger en priorité.
2. **Fallback multi-plateforme :**
   - Windows : `WINDIR/Fonts`
   - macOS : `/System/Library/Fonts`
   - Linux : `/usr/share/fonts`
3. **Augmenter atlas :** `atlas_w = 256` ou dynamique selon contenu.
4. **Vérifier métriques :** Si `w == 0 || h == 0`, skip ou fallback caractère.

---

### 5.5 P2 — Alignement pixel (shader)

**Fichier :** `assets/shaders/sprite_instanced.wgsl`

Option half-pixel offset (si pixel-perfect souhaité) :

```wgsl
// Uniform : pixel_size = 1.0 / view_size
let pixel_offset = vec2(0.5) / view_size;
let snapped = floor(corner) + 0.5;  // ou variante selon objectif
out.clip_position = vec4<f32>(snapped / view_size * 2.0 - 1.0, 0.0, 1.0);
```

Attention : peut introduire du jitter sur objets animés. À combiner avec snap caméra ou technique type sub-texel smooth scrolling (voir Slow Rush).

---

### 5.6 P2 — GUI / Monde séparés

Structurer en passes distinctes :

1. Pass monde (tuiles, entités, overlays monde)
2. Pass GUI (quads, texte) avec viewport ou matrice dédiée

Permet un éventuel DPI scaling ou canvas séparé pour l'UI.

---

## 6. Plan d'Implémentation Recommandé

| Phase | Tâches | Estimation |
|-------|--------|------------|
| 1 | Correctifs P1 (pivot draw_sprites, Y-sort, font) | 1-2 j |
| 2 | Validation atlas + métadonnées sprites | 0.5 j |
| 3 | Half-pixel / pixel-perfect (optionnel) | 1 j |
| 4 | Refactor passes (monde vs GUI) | 1-2 j |

---

## 7. Tests de Validation

- [ ] Deux entités à Y différents : celle avec Y plus grand est dessinée devant
- [ ] Sprite 64×128 dans atlas 100×100 : pas de crop silencieux (validation ou log)
- [ ] Panneau Dev ouvert : labels "Dev", "Grille", etc. visibles sur Windows/Linux/Mac
- [ ] Zoom 1.0, caméra fixe : pas de wobble sur bords de tuiles
- [ ] `draw_sprites` avec position pieds : sprite aligné comme `draw_tilemap`

---

## Annexe : Références

- MGE docs : `z-order-couches.md`, `gestion-sprites.md`
- [Pixel Perfect Rendering](https://slowrush.dev/news/pixel-perfect-rendering)
- [Subpixel-perfect smooth scrolling](https://code-disaster.com/2016/02/subpixel-perfect-smooth-scrolling.html)
- Unity PixelPerfectRendering, Godot 2D Project Settings
