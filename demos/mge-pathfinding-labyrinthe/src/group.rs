//! Formation de groupes de monstres — MGE Démo Pathfinding
//! MonsterGroup, GroupManager, fusion, maillage autour du chef, raid village.

use crate::chunk::{ChunkCoord, CHUNK_PIXELS};
use crate::grid::Vec2;
use std::collections::HashMap;

pub const FORMATION_RADIUS: f32 = 100.0;
pub const ENCOUNTER_VISION: f32 = 150.0;
pub const RAID_THRESHOLD: usize = 20;

/// Position monde du centre du village (chunk 1, 1)
pub fn village_center() -> Vec2 {
    let origin = ChunkCoord::new(1, 1).world_origin();
    Vec2::new(
        origin.x + (CHUNK_PIXELS / 2.0),
        origin.y + (CHUNK_PIXELS / 2.0),
    )
}

/// Groupe de monstres avec un chef
#[derive(Debug, Clone)]
pub struct MonsterGroup {
    pub id: u32,
    pub leader_idx: usize,
    pub members: Vec<usize>,
    pub formation_radius: f32,
    /// true si le groupe est en raid vers le village
    pub is_raiding: bool,
}

impl MonsterGroup {
    pub fn new(id: u32, leader_idx: usize) -> Self {
        Self {
            id,
            leader_idx,
            members: vec![leader_idx],
            formation_radius: FORMATION_RADIUS,
            is_raiding: false,
        }
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn is_raiding(&self) -> bool {
        self.is_raiding
    }

    pub fn should_raid(&self) -> bool {
        self.member_count() >= RAID_THRESHOLD
    }
}

/// Gestionnaire des groupes de monstres
pub struct GroupManager {
    pub groups: HashMap<u32, MonsterGroup>,
    next_id: u32,
}

impl GroupManager {
    pub fn new() -> Self {
        Self {
            groups: HashMap::new(),
            next_id: 1,
        }
    }

    /// Crée un nouveau groupe à partir de deux monstres qui se rencontrent (tous deux sans groupe).
    /// Le chef est choisi aléatoirement entre les deux premiers.
    pub fn create_group(&mut self, idx_a: usize, idx_b: usize) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let leader_idx = if rand::random::<bool>() { idx_a } else { idx_b };
        let other_idx = if leader_idx == idx_a { idx_b } else { idx_a };
        let mut group = MonsterGroup::new(id, leader_idx);
        group.members.push(other_idx);
        self.groups.insert(id, group);
        id
    }

    /// Ajoute un membre à un groupe existant.
    pub fn add_member(&mut self, group_id: u32, member_idx: usize) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            if !group.members.contains(&member_idx) {
                group.members.push(member_idx);
            }
            group.is_raiding = group.should_raid();
        }
    }

    /// Fusionne le groupe `absorbed_id` dans le groupe `target_id`.
    /// Les membres du groupe absorbé rejoignent le groupe cible.
    pub fn merge_groups(&mut self, target_id: u32, absorbed_id: u32) {
        if target_id == absorbed_id {
            return;
        }
        let absorbed_members = match self.groups.remove(&absorbed_id) {
            Some(g) => g.members,
            None => return,
        };
        if let Some(target) = self.groups.get_mut(&target_id) {
            for idx in absorbed_members {
                if !target.members.contains(&idx) {
                    target.members.push(idx);
                }
            }
            target.is_raiding = target.should_raid();
        }
    }

    /// Retourne l'ID du groupe dont le membre est le plus proche du monstre à `idx`,
    /// dans le rayon de vision, autre que le groupe actuel (si déjà dans un groupe).
    pub fn find_nearby_group(
        &self,
        idx: usize,
        positions: &[Vec2],
        current_group_id: Option<u32>,
    ) -> Option<(u32, usize)> {
        let pos = positions.get(idx)?;
        let mut best: Option<(u32, usize, f32)> = None;

        for (gid, group) in &self.groups {
            if Some(*gid) == current_group_id {
                continue;
            }
            for &mid in &group.members {
                if mid == idx {
                    continue;
                }
                if let Some(other_pos) = positions.get(mid) {
                    let d = pos.distance_to(other_pos);
                    if d <= ENCOUNTER_VISION {
                        if best.is_none() || d < best.unwrap().2 {
                            best = Some((*gid, mid, d));
                        }
                    }
                }
            }
        }
        best.map(|(gid, mid, _)| (gid, mid))
    }

    /// Trouve un monstre sans groupe qui est dans la vision d'un autre monstre.
    /// Retourne (idx_solitary, idx_nearby) où idx_solitary n'a pas de groupe
    /// et idx_nearby est un monstre proche (avec ou sans groupe).
    pub fn find_encounter_pair(
        &self,
        positions: &[Vec2],
        group_ids: &[Option<u32>],
        alive: &[bool],
    ) -> Option<(usize, usize)> {
        for i in 0..positions.len() {
            if !alive.get(i).copied().unwrap_or(false) {
                continue;
            }
            if group_ids.get(i).copied().flatten().is_some() {
                continue;
            }
            // i n'a pas de groupe, cherche un voisin dans la vision
            for j in 0..positions.len() {
                if i == j || !alive.get(j).copied().unwrap_or(false) {
                    continue;
                }
                let d = positions[i].distance_to(&positions[j]);
                if d <= ENCOUNTER_VISION {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// Trouve deux groupes qui se croisent (un membre de l'un dans le rayon de l'autre).
    /// Retourne (smaller_id, larger_id) où larger absorbe smaller.
    pub fn find_merging_groups(
        &self,
        positions: &[Vec2],
        alive: &[bool],
    ) -> Option<(u32, u32)> {
        let mut candidates: Vec<(u32, u32)> = Vec::new();
        let gids: Vec<u32> = self.groups.keys().copied().collect();
        for (i, &gid_a) in gids.iter().enumerate() {
            let group_a = self.groups.get(&gid_a)?;
            for &gid_b in gids.iter().skip(i + 1) {
                let group_b = self.groups.get(&gid_b)?;
                for &ma in &group_a.members {
                    if !alive.get(ma).copied().unwrap_or(false) {
                        continue;
                    }
                    let Some(pos_a) = positions.get(ma) else { continue };
                    for &mb in &group_b.members {
                        if !alive.get(mb).copied().unwrap_or(false) {
                            continue;
                        }
                        let Some(pos_b) = positions.get(mb) else { continue };
                        if pos_a.distance_to(pos_b) <= ENCOUNTER_VISION * 1.5 {
                            let (smaller, larger) =
                                if group_a.member_count() >= group_b.member_count() {
                                    (gid_b, gid_a)
                                } else {
                                    (gid_a, gid_b)
                                };
                            if !candidates.contains(&(smaller, larger)) {
                                candidates.push((smaller, larger));
                            }
                        }
                    }
                }
            }
        }
        candidates.into_iter().next()
    }

    /// Supprime un membre mort du groupe. Si le groupe devient vide, le supprime.
    pub fn remove_dead_member(&mut self, group_id: u32, member_idx: usize) {
        if let Some(group) = self.groups.get_mut(&group_id) {
            group.members.retain(|&i| i != member_idx);
            if group.members.is_empty() {
                self.groups.remove(&group_id);
            } else if group.leader_idx == member_idx && !group.members.is_empty() {
                // Le chef est mort, le premier membre restant devient chef
                group.leader_idx = group.members[0];
            }
        }
    }

    /// Met à jour les groupes : enlève les morts, fusionne si nécessaire, active le raid.
    pub fn cleanup_and_update(
        &mut self,
        _positions: &[Vec2],
        _group_ids: &[Option<u32>],
        alive: &[bool],
    ) {
        // Nettoyer les morts
        let mut to_remove: Vec<(u32, usize)> = Vec::new();
        for (gid, group) in &self.groups {
            for &mid in &group.members.clone() {
                if !alive.get(mid).copied().unwrap_or(true) {
                    to_remove.push((*gid, mid));
                }
            }
        }
        for (gid, mid) in to_remove {
            self.remove_dead_member(gid, mid);
        }

        // Activer le raid pour les groupes >= 20
        for group in self.groups.values_mut() {
            if group.should_raid() {
                group.is_raiding = true;
            }
        }
    }
}
