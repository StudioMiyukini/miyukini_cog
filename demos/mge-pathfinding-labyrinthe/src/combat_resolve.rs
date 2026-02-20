//! Résolution du combat — MGE Démo Pathfinding
//! Helpers pour réduire la duplication combat + FloatingText

use crate::entity::AttackResult;
use crate::floating_text::FloatingText;

/// Convertit un résultat d'attaque en FloatingText et l'ajoute à la liste.
pub fn push_attack_result(
    floating_texts: &mut Vec<FloatingText>,
    res: AttackResult,
    hitbox_half_size: f32,
) {
    match res {
        AttackResult::Dodge(pos) => {
            floating_texts.push(FloatingText::new_dodge(pos, hitbox_half_size));
        }
        AttackResult::Hit(pos, dmg) => {
            floating_texts.push(FloatingText::new_damage(pos, hitbox_half_size, dmg));
        }
    }
}

