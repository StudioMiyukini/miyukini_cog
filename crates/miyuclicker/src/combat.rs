//! Résolution des combats (attaquant vs défenseur).
//! Tool MiyuClickerCombat — resolve.
//!
//! @id: miyuclicker_combat_module
//! @role: mutator
//! @layer: domain
//! @do: resolve_combat_attacker_defender

use crate::state::Proprietaire;

/// Résultat d'un combat (vainqueur, troupes restantes att/def).
/// @id: miyuclicker_combat_combat_result
/// @do: represent_combat_outcome
/// @role: data
#[derive(Debug, Clone)]
pub struct CombatResult {
    pub vainqueur: Proprietaire,
    pub troupes_att_restantes: i64,
    pub troupes_def_restantes: i64,
}

/// Résolution simplifiée : hasard + rapport de force. Seed permet tests reproductibles.
/// @id: miyuclicker_combat_resolve
/// @do: resolve_combat_result
/// @role: mutator
/// @human: stats_att/stats_def 1.0 en MVP ; seed = graine pour dé (tests ou SystemTime).
pub fn resolve(
    attaquant: i64,
    defenseur: i64,
    _stats_att: f64,
    _stats_def: f64,
    seed: u64,
) -> CombatResult {
    if attaquant <= 0 {
        return CombatResult {
            vainqueur: Proprietaire::Adverse(0),
            troupes_att_restantes: 0,
            troupes_def_restantes: defenseur,
        };
    }
    if defenseur <= 0 {
        return CombatResult {
            vainqueur: Proprietaire::Joueur,
            troupes_att_restantes: attaquant,
            troupes_def_restantes: 0,
        };
    }
    let roll = ((seed % 100) as f64) / 100.0;
    let rapport = (attaquant as f64) / (defenseur as f64).max(0.1);
    let seuil = 0.5 + (rapport / (1.0 + rapport)) * 0.4;
    let joueur_gagne = roll < seuil;
    let (troupes_att, troupes_def) = if joueur_gagne {
        let pertes_def = defenseur;
        let pertes_att = (attaquant as f64 * 0.3).max(1.0) as i64;
        (
            (attaquant - pertes_att).max(0),
            (defenseur - pertes_def).max(0),
        )
    } else {
        let pertes_att = attaquant;
        let pertes_def = (defenseur as f64 * 0.2).max(1.0) as i64;
        (
            (attaquant - pertes_att).max(0),
            (defenseur - pertes_def).max(0),
        )
    };
    CombatResult {
        vainqueur: if joueur_gagne {
            Proprietaire::Joueur
        } else {
            Proprietaire::Adverse(0)
        },
        troupes_att_restantes: troupes_att,
        troupes_def_restantes: troupes_def,
    }
}
