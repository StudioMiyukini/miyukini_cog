// @id: MGE-Math-Interp @do: implement @role: back-end @layer: 1 @human: miyuk

//! Fonctions d'interpolation.
//!
//! Fournit `lerp`, `smoothstep` et leurs variantes pour l'animation,
//! le mouvement de camera et les transitions UI.

/// Interpolation lineaire entre `a` et `b`.
///
/// - `t = 0.0` retourne `a`
/// - `t = 1.0` retourne `b`
/// - `t = 0.5` retourne le milieu
///
/// Note : `t` n'est pas clamp -- des valeurs hors `[0, 1]` donnent une extrapolation.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Alias de `lerp` pour la lisibilite quand le type est ambigu.
pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    lerp(a, b, t)
}

/// Smoothstep cubique (Hermite).
///
/// Retourne 0.0 si `t <= a`, 1.0 si `t >= b`, et une courbe S entre les deux.
/// Utile pour des transitions douces (camera follow, fade in/out).
///
/// Formule : `3t^2 - 2t^3` apres normalisation et clamping.
pub fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
    let t = ((t - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Inverse lerp : retourne le `t` tel que `lerp(a, b, t) == v`.
///
/// Retourne 0.0 si `a == b` (division par zero evitee).
pub fn inverse_lerp(a: f32, b: f32, v: f32) -> f32 {
    let denom = b - a;
    if denom.abs() < f32::EPSILON {
        return 0.0;
    }
    (v - a) / denom
}
