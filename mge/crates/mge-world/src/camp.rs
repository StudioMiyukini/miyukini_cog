use serde::{Deserialize, Serialize};

/// World-space 2-D position (tile coordinates).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TilePos {
    pub x: i32,
    pub y: i32,
}

impl TilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Town services offered at a service anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceKind {
    WeaponVendor,
    ArmorVendor,
    MiscVendor,
    GambleVendor,
    Blacksmith,
    Healer,
    Stash,
    Waypoint,
    TownPortalPad,
    QuestGiver,
}

/// A named NPC or service point fixed in the camp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceAnchor {
    pub id: u32,
    pub name: String,
    pub kind: ServiceKind,
    pub pos: TilePos,
    /// Whether this service is initially unlocked or requires a quest.
    pub initially_unlocked: bool,
}

/// Entry/exit portal definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampPortal {
    pub id: u32,
    pub name: String,
    pub pos: TilePos,
    /// Zone the portal leads to.
    pub target_zone_id: u32,
}

/// The town camp definition (fixed layout).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampDef {
    pub name: String,
    /// Radius in tiles considered safe (no monsters spawn within).
    pub safe_radius: u32,
    pub entry_portal: CampPortal,
    pub exit_portal: CampPortal,
    pub anchors: Vec<ServiceAnchor>,
}

impl CampDef {
    pub fn anchor_by_kind(&self, kind: ServiceKind) -> Option<&ServiceAnchor> {
        self.anchors.iter().find(|a| a.kind == kind)
    }

    pub fn unlocked_anchors(&self) -> Vec<&ServiceAnchor> {
        self.anchors.iter().filter(|a| a.initially_unlocked).collect()
    }
}

/// Act 1 Rogue Encampment definition.
pub fn act1_camp() -> CampDef {
    CampDef {
        name: "Rogue Encampment".into(),
        safe_radius: 30,
        entry_portal: CampPortal {
            id: 1,
            name: "Blood Moor Gate".into(),
            pos: TilePos::new(50, 10),
            target_zone_id: 10, // Blood Moor
        },
        exit_portal: CampPortal {
            id: 2,
            name: "Return from Wilderness".into(),
            pos: TilePos::new(50, 10),
            target_zone_id: 0, // camp itself
        },
        anchors: vec![
            ServiceAnchor {
                id: 1,
                name: "Akara".into(),
                kind: ServiceKind::Healer,
                pos: TilePos::new(20, 15),
                initially_unlocked: true,
            },
            ServiceAnchor {
                id: 2,
                name: "Charsi".into(),
                kind: ServiceKind::Blacksmith,
                pos: TilePos::new(35, 12),
                initially_unlocked: true,
            },
            ServiceAnchor {
                id: 3,
                name: "Gheed".into(),
                kind: ServiceKind::MiscVendor,
                pos: TilePos::new(45, 20),
                initially_unlocked: true,
            },
            ServiceAnchor {
                id: 4,
                name: "Shared Stash".into(),
                kind: ServiceKind::Stash,
                pos: TilePos::new(25, 20),
                initially_unlocked: true,
            },
            ServiceAnchor {
                id: 5,
                name: "Waypoint".into(),
                kind: ServiceKind::Waypoint,
                pos: TilePos::new(40, 18),
                initially_unlocked: true,
            },
            ServiceAnchor {
                id: 6,
                name: "Deckard Cain".into(),
                kind: ServiceKind::QuestGiver,
                pos: TilePos::new(30, 15),
                initially_unlocked: false, // unlocked by Rescue Cain quest
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camp_has_healer() {
        let camp = act1_camp();
        assert!(camp.anchor_by_kind(ServiceKind::Healer).is_some());
    }

    #[test]
    fn camp_cain_initially_locked() {
        let camp = act1_camp();
        let cain = camp.anchor_by_kind(ServiceKind::QuestGiver).unwrap();
        assert!(!cain.initially_unlocked);
    }

    #[test]
    fn camp_unlocked_anchors_excludes_cain() {
        let camp = act1_camp();
        let unlocked = camp.unlocked_anchors();
        assert!(unlocked.iter().all(|a| a.kind != ServiceKind::QuestGiver));
    }

    #[test]
    fn camp_entry_portal_targets_blood_moor() {
        let camp = act1_camp();
        assert_eq!(camp.entry_portal.target_zone_id, 10);
    }
}
