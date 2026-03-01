// @id: MGE-Math-Vec2 @do: implement @role: back-end @layer: 1 @human: miyuk

//! Vec2 -- vecteur 2D f32 simple.
//!
//! Fournit les operations vectorielles de base : addition, soustraction,
//! multiplication scalaire, longueur, normalisation, produit scalaire, lerp.

/// Vecteur 2D a composantes `f32`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Vec2 {
    /// Composante X.
    pub x: f32,
    /// Composante Y.
    pub y: f32,
}

impl Vec2 {
    /// Cree un nouveau vecteur.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Vecteur nul (0, 0).
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Vecteur unitaire (1, 1).
    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// Longueur (norme euclidienne) du vecteur.
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Carre de la longueur (evite un `sqrt` quand on compare des distances).
    pub fn length_sq(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Retourne le vecteur normalise (longueur 1).
    ///
    /// Si le vecteur est quasi-nul (longueur < `f32::EPSILON`), retourne `Vec2::zero()`.
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len < f32::EPSILON {
            return Self::zero();
        }
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }

    /// Produit scalaire (dot product).
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Interpolation lineaire entre deux vecteurs.
    ///
    /// `t = 0.0` retourne `self`, `t = 1.0` retourne `other`.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
