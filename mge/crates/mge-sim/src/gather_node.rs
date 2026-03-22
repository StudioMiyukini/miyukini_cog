// ---------------------------------------------------------------------------
// Gather Nodes — arbres, gisements et autres ressources récoltables
// Séparation logique (ici) / rendu (sodomight/main.rs)
// ---------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use crate::civil_skills::ResourceKind;

// ---------------------------------------------------------------------------
// Types de nœuds récoltables
// ---------------------------------------------------------------------------

/// Catégorie visuelle et mécanique d'un nœud de récolte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatherNodeKind {
    /// Arbre — bûcheronage → bois
    Tree,
    /// Grand arbre — bûcheronage → bois dur / ancien
    HardwoodTree,
    /// Rocher avec minerai — minage → pierre / fer
    OreRock,
    /// Veine de minerai rare — minage → argent / or / gemmes
    RareOreVein,
    /// Buisson d'herbes — cueillette → herbes
    HerbBush,
    /// Champignon lumineux — cueillette → champignons
    Mushroom,
}

impl GatherNodeKind {
    /// Ressource principale récoltée.
    pub fn primary_resource(self, tier: u8) -> ResourceKind {
        match self {
            Self::Tree => ResourceKind::BoisBrut,
            Self::HardwoodTree => {
                if tier >= 4 { ResourceKind::BoisAncien }
                else { ResourceKind::BoisDur }
            }
            Self::OreRock => {
                if tier >= 3 { ResourceKind::MineraiFer }
                else { ResourceKind::PierreBreute }
            }
            Self::RareOreVein => {
                if tier >= 5 { ResourceKind::Gemme }
                else if tier >= 4 { ResourceKind::MineraiOr }
                else { ResourceKind::MineraiArgent }
            }
            Self::HerbBush => {
                if tier >= 4 { ResourceKind::HerbeRare }
                else { ResourceKind::HerbeCommune }
            }
            Self::Mushroom => ResourceKind::Champignon,
        }
    }

    /// Chance de drop bonus (ressource rare en plus).
    pub fn bonus_resource(self, tier: u8) -> Option<ResourceKind> {
        match self {
            Self::Tree if tier >= 2 => Some(ResourceKind::BoisDur),
            Self::HardwoodTree if tier >= 5 => Some(ResourceKind::BoisAncien),
            Self::OreRock if tier >= 3 => Some(ResourceKind::MineraiFer),
            Self::RareOreVein => Some(ResourceKind::Gemme),
            Self::HerbBush if tier >= 3 => Some(ResourceKind::FleurMagique),
            _ => None,
        }
    }

    /// HP max du nœud (nombre de coups pour le dépléter).
    pub fn max_hp(self) -> u32 {
        match self {
            Self::Tree => 3,
            Self::HardwoodTree => 5,
            Self::OreRock => 4,
            Self::RareOreVein => 6,
            Self::HerbBush => 2,
            Self::Mushroom => 1,
        }
    }

    /// Temps de respawn en frames (à 60 FPS).
    pub fn respawn_time(self) -> u32 {
        match self {
            Self::Tree => 900,       // 15s
            Self::HardwoodTree => 1500, // 25s
            Self::OreRock => 1200,    // 20s
            Self::RareOreVein => 2400, // 40s
            Self::HerbBush => 600,    // 10s
            Self::Mushroom => 480,    // 8s
        }
    }

    /// Portée d'interaction pour récolter.
    pub fn interact_range(self) -> f32 {
        match self {
            Self::Tree | Self::HardwoodTree => 1.5,
            Self::OreRock | Self::RareOreVein => 1.2,
            Self::HerbBush | Self::Mushroom => 1.0,
        }
    }

    /// Cooldown entre chaque coup de récolte (frames).
    pub fn harvest_cooldown(self) -> u32 {
        match self {
            Self::Tree => 40,        // ~0.67s
            Self::HardwoodTree => 50, // ~0.83s
            Self::OreRock => 45,     // ~0.75s
            Self::RareOreVein => 55, // ~0.92s
            Self::HerbBush => 30,    // ~0.5s
            Self::Mushroom => 20,    // ~0.33s
        }
    }

    /// Label court pour l'UI.
    pub fn label(self) -> &'static [u8] {
        match self {
            Self::Tree => b"ARBRE",
            Self::HardwoodTree => b"CHENE",
            Self::OreRock => b"ROCHE",
            Self::RareOreVein => b"VEINE",
            Self::HerbBush => b"HERBE",
            Self::Mushroom => b"CHAMPI",
        }
    }
}

// ---------------------------------------------------------------------------
// Instance d'un nœud de récolte dans le monde
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatherNode {
    pub pos: [f32; 2],
    pub kind: GatherNodeKind,
    pub hp: u32,
    pub max_hp: u32,
    pub respawn_timer: u32,
    /// Tier de la zone (affecte la qualité des drops).
    pub tier: u8,
}

impl GatherNode {
    pub fn new(pos: [f32; 2], kind: GatherNodeKind, tier: u8) -> Self {
        let max_hp = kind.max_hp();
        Self { pos, kind, hp: max_hp, max_hp, respawn_timer: 0, tier }
    }

    /// Le nœud est-il disponible pour la récolte ?
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Le nœud est-il en cours de respawn ?
    pub fn is_depleted(&self) -> bool {
        self.hp == 0 && self.respawn_timer > 0
    }

    /// Frapper le nœud pour récolter. Retourne la ressource obtenue si le coup aboutit.
    pub fn hit(&mut self) -> Option<ResourceKind> {
        if self.hp == 0 { return None; }
        self.hp -= 1;
        let resource = self.kind.primary_resource(self.tier);
        if self.hp == 0 {
            self.respawn_timer = self.kind.respawn_time();
        }
        Some(resource)
    }

    /// Tick de mise à jour — gère le respawn.
    pub fn update(&mut self) {
        if self.respawn_timer > 0 {
            self.respawn_timer -= 1;
            if self.respawn_timer == 0 {
                self.hp = self.max_hp;
            }
        }
    }

    /// Ressource principale affichée au joueur.
    pub fn resource_label(&self) -> &'static str {
        self.kind.primary_resource(self.tier).name()
    }
}

// ---------------------------------------------------------------------------
// Génération procédurale des nœuds par zone
// ---------------------------------------------------------------------------

use crate::constants::*;
use crate::entity::ZoneType;

/// Génère les nœuds de récolte pour une zone donnée.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn generate_gather_nodes(
    zone_type: ZoneType,
    tier: u8,
    terrain: &[[u8; MAP_W]; MAP_H],
    seed: u32,
) -> Vec<GatherNode> {
    if zone_type == ZoneType::Town {
        return vec![];
    }

    let mut nodes = Vec::new();

    // --- Arbres ---
    let tree_count = match zone_type {
        ZoneType::Forest => 35,
        ZoneType::Wilderness => 20,
        ZoneType::Marsh => 10,
        ZoneType::Plains | ZoneType::Field => 8,
        ZoneType::Highlands => 5,
        ZoneType::Graveyard => 3,
        ZoneType::Cave | ZoneType::Town => 0,
    };
    let hardwood_count = match zone_type {
        ZoneType::Forest => 8,
        ZoneType::Wilderness => 4,
        ZoneType::Highlands => 3,
        _ => 0,
    };

    // --- Gisements ---
    let ore_count = match zone_type {
        ZoneType::Cave => 20,
        ZoneType::Highlands => 12,
        ZoneType::Field => 6,
        ZoneType::Wilderness => 4,
        ZoneType::Plains => 3,
        ZoneType::Forest | ZoneType::Marsh => 2,
        ZoneType::Graveyard | ZoneType::Town => 0,
    };
    let rare_ore_count = match zone_type {
        ZoneType::Cave => 5,
        ZoneType::Highlands => 3,
        _ if tier >= 3 => 1,
        _ => 0,
    };

    // --- Herbes ---
    let herb_count = match zone_type {
        ZoneType::Forest => 12,
        ZoneType::Marsh => 15,
        ZoneType::Wilderness | ZoneType::Plains => 8,
        ZoneType::Field => 6,
        ZoneType::Graveyard => 4,
        ZoneType::Highlands => 3,
        ZoneType::Cave | ZoneType::Town => 0,
    };
    let mushroom_count = match zone_type {
        ZoneType::Cave => 12,
        ZoneType::Forest => 6,
        ZoneType::Marsh => 8,
        ZoneType::Graveyard => 4,
        _ => 0,
    };

    let placements = [
        (GatherNodeKind::Tree, tree_count),
        (GatherNodeKind::HardwoodTree, hardwood_count),
        (GatherNodeKind::OreRock, ore_count),
        (GatherNodeKind::RareOreVein, rare_ore_count),
        (GatherNodeKind::HerbBush, herb_count),
        (GatherNodeKind::Mushroom, mushroom_count),
    ];

    for (kind, count) in placements {
        let kind_seed = seed.wrapping_add(kind as u32 * 7919);
        for i in 0..(count * 3) {
            if nodes.iter().filter(|n: &&GatherNode| n.kind == kind).count() >= count {
                break;
            }
            let h1 = hash(i as u32, 0, kind_seed);
            let h2 = hash(0, i as u32, kind_seed.wrapping_add(3));
            let x = (h1 % (MAP_W as u32 - 6) + 3) as f32 + 0.5;
            let y = (h2 % (MAP_H as u32 - 6) + 3) as f32 + 0.5;

            // Must be on walkable terrain
            let tx = x as usize;
            let ty = y as usize;
            if tx >= MAP_W || ty >= MAP_H || terrain[ty][tx] == 0 || terrain[ty][tx] == 5 {
                continue;
            }

            // Minimum spacing from other nodes
            let too_close = nodes.iter().any(|n: &GatherNode| {
                let dx = n.pos[0] - x;
                let dy = n.pos[1] - y;
                dx * dx + dy * dy < 4.0  // 2 tiles minimum
            });
            if too_close { continue; }

            // Trees prefer grass (1), ore prefers stone (3) or cave (4)
            let tt = terrain[ty][tx];
            let terrain_ok = match kind {
                GatherNodeKind::Tree | GatherNodeKind::HardwoodTree => tt == 1 || tt == 2,
                GatherNodeKind::OreRock | GatherNodeKind::RareOreVein => tt == 3 || tt == 4 || tt == 2,
                GatherNodeKind::HerbBush => tt == 1,
                GatherNodeKind::Mushroom => tt == 4 || tt == 2 || tt == 1,
            };
            if !terrain_ok { continue; }

            nodes.push(GatherNode::new([x, y], kind, tier));
        }
    }

    nodes
}

/// Simple deterministic hash for procedural generation.
fn hash(x: u32, y: u32, seed: u32) -> u32 {
    let mut h = x.wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263))
        .wrapping_add(seed);
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_lifecycle() {
        let mut node = GatherNode::new([5.0, 5.0], GatherNodeKind::Tree, 1);
        assert!(node.is_alive());
        assert_eq!(node.hp, 3);

        // Hit 3 times to deplete
        for _ in 0..3 {
            let r = node.hit();
            assert_eq!(r, Some(ResourceKind::BoisBrut));
        }
        assert!(!node.is_alive());
        assert!(node.is_depleted());

        // Hit when depleted returns None
        assert_eq!(node.hit(), None);

        // Simulate respawn
        for _ in 0..node.kind.respawn_time() {
            node.update();
        }
        assert!(node.is_alive());
        assert_eq!(node.hp, 3);
    }

    #[test]
    fn hardwood_tier_scaling() {
        let low = GatherNode::new([0.0, 0.0], GatherNodeKind::HardwoodTree, 2);
        assert_eq!(low.kind.primary_resource(low.tier), ResourceKind::BoisDur);

        let high = GatherNode::new([0.0, 0.0], GatherNodeKind::HardwoodTree, 4);
        assert_eq!(high.kind.primary_resource(high.tier), ResourceKind::BoisAncien);
    }

    #[test]
    fn ore_tier_scaling() {
        assert_eq!(GatherNodeKind::OreRock.primary_resource(1), ResourceKind::PierreBreute);
        assert_eq!(GatherNodeKind::OreRock.primary_resource(3), ResourceKind::MineraiFer);
        assert_eq!(GatherNodeKind::RareOreVein.primary_resource(3), ResourceKind::MineraiArgent);
        assert_eq!(GatherNodeKind::RareOreVein.primary_resource(4), ResourceKind::MineraiOr);
        assert_eq!(GatherNodeKind::RareOreVein.primary_resource(5), ResourceKind::Gemme);
    }

    #[test]
    fn generate_forest_has_trees() {
        let mut terrain = [[1u8; MAP_W]; MAP_H]; // all grass
        // Add some stone for ore
        terrain[50][50] = 3;
        let nodes = generate_gather_nodes(ZoneType::Forest, 2, &terrain, 42);
        assert!(!nodes.is_empty());
        let trees = nodes.iter().filter(|n| n.kind == GatherNodeKind::Tree).count();
        assert!(trees > 0, "forest should have trees");
    }

    #[test]
    fn generate_cave_has_ore() {
        let mut terrain = [[4u8; MAP_W]; MAP_H]; // all cave floor
        terrain[0] = [0; MAP_W]; // wall border
        let nodes = generate_gather_nodes(ZoneType::Cave, 3, &terrain, 99);
        let ore = nodes.iter().filter(|n| n.kind == GatherNodeKind::OreRock).count();
        assert!(ore > 0, "cave should have ore deposits");
    }

    #[test]
    fn town_has_no_nodes() {
        let terrain = [[3u8; MAP_W]; MAP_H];
        let nodes = generate_gather_nodes(ZoneType::Town, 0, &terrain, 1);
        assert!(nodes.is_empty());
    }

    #[test]
    fn minimum_spacing() {
        let terrain = [[1u8; MAP_W]; MAP_H];
        let nodes = generate_gather_nodes(ZoneType::Forest, 2, &terrain, 42);
        for (i, a) in nodes.iter().enumerate() {
            for b in &nodes[i + 1..] {
                let dx = a.pos[0] - b.pos[0];
                let dy = a.pos[1] - b.pos[1];
                assert!(dx * dx + dy * dy >= 4.0, "nodes too close");
            }
        }
    }
}
