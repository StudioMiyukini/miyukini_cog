// @id: MGE-Math-Iso @do: implement @role: back-end @layer: 1 @human: miyuk

//! Coordonnees isometriques dimetriques 2:1 -- standard Sodomight (clone D2).
//!
//! ## Projection dimetric 2:1
//!
//! ```text
//!         +--32px--+
//!        /          \
//!       /            \  16px
//!      /              \
//!     +    64px wide   +
//!      \              /
//!       \            /  16px
//!        \          /
//!         +--------+
//! ```
//!
//! Formules de conversion :
//! - `screen_x = (tile_x - tile_y) * (TILE_W / 2)`
//! - `screen_y = (tile_x + tile_y) * (TILE_H / 2)`
//!
//! Avec `TILE_W = 64` et `TILE_H = 32`, cela donne :
//! - `screen_x = (tile_x - tile_y) * 32`
//! - `screen_y = (tile_x + tile_y) * 16`

/// Largeur d'une tuile isometrique en pixels (64px).
pub const TILE_W: f32 = 64.0;

/// Hauteur d'une tuile isometrique en pixels (32px).
pub const TILE_H: f32 = 32.0;

/// Coordonnees de tuile entieres.
///
/// Represente une position discrete sur la grille isometrique.
/// Utile pour le tilemap, le pathfinding, et l'identification de tuile sous le curseur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct IsoCoord {
    /// Coordonnee X de la tuile.
    pub tx: i32,
    /// Coordonnee Y de la tuile.
    pub ty: i32,
}

impl IsoCoord {
    /// Cree de nouvelles coordonnees de tuile.
    pub const fn new(tx: i32, ty: i32) -> Self {
        Self { tx, ty }
    }

    /// Convertit tuile vers pixels ecran (coin superieur gauche du losange).
    ///
    /// Formule dimetrique 2:1 :
    ///   `sx = (tx - ty) * (TILE_W / 2)`, `sy = (tx + ty) * (TILE_H / 2)`
    pub fn to_screen(self) -> (f32, f32) {
        let sx = (self.tx - self.ty) as f32 * (TILE_W / 2.0);
        let sy = (self.tx + self.ty) as f32 * (TILE_H / 2.0);
        (sx, sy)
    }

    /// Convertit pixels ecran vers coordonnees de tuile (arrondi au sol).
    ///
    /// Inverse de `to_screen`. Utile pour identifier la tuile sous le curseur.
    pub fn from_screen(sx: f32, sy: f32) -> Self {
        let half_w = TILE_W / 2.0;
        let half_h = TILE_H / 2.0;
        let tx = (sx / half_w + sy / half_h) / 2.0;
        let ty = (sy / half_h - sx / half_w) / 2.0;
        Self {
            tx: tx.floor() as i32,
            ty: ty.floor() as i32,
        }
    }

    /// Z-order pour le painter's algorithm.
    ///
    /// Les tuiles plus en bas a l'ecran (tx + ty plus grand) ont un Z plus grand
    /// et sont dessinees en dernier (par-dessus).
    pub const fn z_order(self) -> i32 {
        self.tx + self.ty
    }
}

/// Position monde flottante pour les entites en mouvement entre tuiles.
///
/// Contrairement a `IsoCoord` (entier), `WorldPos` permet une position continue
/// pour le deplacement fluide des entites (joueur, monstres, projectiles).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorldPos {
    /// Position X en coordonnees monde (unites : tuiles).
    pub x: f32,
    /// Position Y en coordonnees monde (unites : tuiles).
    pub y: f32,
}

impl WorldPos {
    /// Cree une nouvelle position monde.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Position monde a l'origine (0, 0).
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Convertit position monde vers pixels ecran.
    ///
    /// Meme formule que `IsoCoord::to_screen` mais en flottant.
    pub fn to_screen(self) -> (f32, f32) {
        let sx = (self.x - self.y) * (TILE_W / 2.0);
        let sy = (self.x + self.y) * (TILE_H / 2.0);
        (sx, sy)
    }

    /// Construit une `WorldPos` a partir d'un `IsoCoord`.
    pub fn from_iso(coord: IsoCoord) -> Self {
        Self {
            x: coord.tx as f32,
            y: coord.ty as f32,
        }
    }

    /// Distance euclidienne entre deux positions monde.
    pub fn distance(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Interpolation lineaire entre deux positions.
    ///
    /// `t = 0.0` retourne `self`, `t = 1.0` retourne `other`.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Z-order flottant pour le tri des entites.
    ///
    /// Les entites avec un `z_order` plus grand sont dessinees en dernier.
    pub fn z_order(self) -> f32 {
        self.x + self.y
    }
}

/// Convertit pixels ecran vers position monde (inverse de `WorldPos::to_screen`).
///
/// Utile pour la conversion du clic souris en position monde.
pub fn screen_to_world(sx: f32, sy: f32) -> WorldPos {
    let half_w = TILE_W / 2.0;
    let half_h = TILE_H / 2.0;
    let x = (sx / half_w + sy / half_h) / 2.0;
    let y = (sy / half_h - sx / half_w) / 2.0;
    WorldPos { x, y }
}
