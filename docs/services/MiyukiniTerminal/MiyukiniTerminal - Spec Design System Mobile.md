# MiyukiniTerminal — Spécification Design System Mobile

## Contexte

Ce document définit les **composants UI** (boutons, cartes, listes, champs, modals), le **thème** (palette Gaming héritée, adaptée mobile), les tailles touch, la grille responsive et le mode sombre/clair.

**Références :**

- [Alignement Central Dioxus](./MiyukiniTerminal%20-%20Alignement%20Central%20Dioxus.md)
- Central theme : `apps/central/src/theme.rs`

---

## Portée / Scope

- Composants réutilisables
- Palette Gaming (héritée)
- Tailles touch (44pt min)
- Grille responsive
- Typographie, icônes

---

## 1. Palette Gaming (héritée Central)

| Token | Valeur | Usage |
|-------|--------|-------|
| bg_main | #171a21 | Fond principal |
| bg_header | #1b2838 | Header |
| bg_card | #1e2329 | Cartes |
| bg_hover | #2a3f5f | Hover (focus) |
| bg_active | #1a9fff | Élément actif |
| text_primary | #c6d4df | Texte principal |
| text_secondary | #8f98a0 | Texte secondaire |
| text_muted | #5c6873 | Texte atténué |
| accent_blue | #1a9fff | Liens, actions |
| accent_green | #5ba32b | Succès |
| accent_red | #c83737 | Erreur |
| border | #2a3f5f | Bordures |

---

## 2. Tailles touch

| Règle | Valeur |
|-------|--------|
| Zone cible minimale | 44×44 pt (ou dp) |
| Padding bouton | 12–16 pt vertical, 24 pt horizontal |
| Espacement entre éléments cliquables | 8 pt minimum |

---

## 3. Composants

### 3.1 Bouton

| Variant | Style |
|---------|-------|
| Primary | bg_active, text_white, arrondi 8pt |
| Secondary | border, text_primary |
| Danger | accent_red |

| Taille | Hauteur |
|--------|---------|
| Small | 36 pt |
| Normal | 44 pt |
| Large | 52 pt |

### 3.2 Carte (Service, etc.)

- bg_card, border 1px border, radius 8pt
- Padding 16pt
- Ombre légère (elevation 2)

### 3.3 Liste

- Items séparés par border ou 8pt
- Hauteur item min 44pt (touch)
- Divider : 1px border

### 3.4 Champ saisie

- Border, radius 8pt
- Hauteur 44pt
- Padding 12pt
- Placeholder : text_muted

### 3.5 Modal

- Overlay : rgba(0,0,0,0.5)
- Contenu : bg_card, radius 16pt, padding 24pt
- Bouton fermer en haut à droite

---

## 4. Grille responsive

| Breakpoint | Largeur | Colonnes |
|------------|---------|----------|
| Phone | 360–480 dp | 1 colonne |
| Large phone | 480+ dp | 1 colonne (marges accrues) |

| Marges | Valeur |
|--------|--------|
| Horizontal | 16 pt |
| Vertical | 16 pt |
| Entre cartes | 12 pt |

---

## 5. Typographie

| Élément | Taille | Poids |
|---------|--------|-------|
| H1 | 24 pt | Bold |
| H2 | 20 pt | Bold |
| Body | 16 pt | Regular |
| Caption | 14 pt | Regular |
| Small | 12 pt | Regular |

Police : Roboto (Android par défaut) ou système.

---

## 6. Icônes

| Source | Usage |
|--------|-------|
| Material Icons | Standard Android |
| Icônes custom | Si cohérence Central |
| Taille | 24 pt pour navigation, 20 pt inline |

---

## 7. Mode sombre / clair

| Mode | Statut |
|------|--------|
| Sombre (Gaming) | Par défaut |
| Clair | Optionnel (préférence) |

Pour mode clair : inverser bg et text (bg clair, text sombre). Garder accents.

---

## 8. Références

- [Alignement Central Dioxus](./MiyukiniTerminal%20-%20Alignement%20Central%20Dioxus.md)
- [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md)
- Central : `apps/central/src/theme.rs`
