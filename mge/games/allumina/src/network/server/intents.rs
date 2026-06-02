//! In-game intent handling: applying a validated client action (move, skill,
//! pickup, drop, equip) to the authoritative [`GameRoom`] world. Split out of
//! `server.rs` to keep the connection/dispatch layer separate from gameplay
//! mutation. Reached from `dispatch_message` via the re-export in `server`.

use super::*;

// ── In-game intents (only valid inside a room) ──────────────────────────────

pub(super) fn apply_intent_in_room(
    lobby: &Arc<Mutex<LobbyServer>>,
    room_id: u64,
    entity_id: u64,
    msg: ClientMessage,
) {
    let room = match lobby_lock(lobby).rooms.get(&room_id).cloned() {
        Some(r) => r,
        None => return,
    };
    let mut r = room_lock(&room);
    let entity = Entity(entity_id);
    if !r.players.contains_key(&entity_id) {
        return;
    }
    match msg {
        ClientMessage::MoveIntent { target_x, target_y } => {
            apply_move_to(&mut r, entity, target_x, target_y);
        }
        ClientMessage::AttackTarget { entity_id: target_id } => {
            let target = Entity(target_id);
            let valid = r.app.world.positions.contains_key(&target)
                && distance_between(&r, entity, target).is_some_and(|d| d <= MAX_ATTACK_RANGE);
            if valid {
                if let Some(c) = r.app.world.combat.get_mut(&entity) {
                    c.target = Some(target);
                }
            }
        }
        ClientMessage::UseSkill { skill_index, target, cursor_x: _, cursor_y: _ } => {
            try_use_skill(&mut r, entity, skill_index, target);
        }
        ClientMessage::PickupItem { ground_item_id } => {
            try_pickup_item(&mut r, entity, ground_item_id);
        }
        ClientMessage::DropItem { inventory_slot } => {
            try_drop_item(&mut r, entity, inventory_slot);
        }
        ClientMessage::EquipItem { inventory_slot } => {
            try_equip_item(&mut r, entity, inventory_slot);
        }
        ClientMessage::UnequipItem { slot } => {
            try_unequip(&mut r, entity, &slot);
        }
        ClientMessage::ChatMessage { text } => {
            let sender = r
                .players
                .get(&entity_id)
                .map(|p| p.character_name.clone())
                .unwrap_or_default();
            let text = clamp_chat(text);
            r.broadcast(ServerMessage::ChatBroadcast { sender, text });
        }
        ClientMessage::TeleportToWaypoint { wp_id } => {
            let allowed = r
                .app
                .world
                .discovered_wps
                .get(&entity)
                .is_some_and(|d| d.contains(wp_id));
            if allowed {
                crate::systems::waypoint::waypoint_teleport(&mut r.app.world, entity, wp_id);
            }
        }
        _ => {}
    }
}

pub(super) fn distance_between(r: &GameRoom, a: Entity, b: Entity) -> Option<f32> {
    let pa = r.app.world.positions.get(&a)?;
    let pb = r.app.world.positions.get(&b)?;
    Some(pa.distance(*pb))
}

pub(super) fn apply_move_to(r: &mut GameRoom, entity: Entity, target_x: f32, target_y: f32) {
    let Some(&pos) = r.app.world.positions.get(&entity) else { return };
    let goal_pos = Position2D::new(target_x, target_y);
    let (gx, gy) = goal_pos.tile();
    if !r.app.world.map.is_walkable(gx, gy) {
        return;
    }
    if let Some(path) = find_path(&r.app.world.map, pos.tile(), (gx, gy)) {
        if !path.is_empty() {
            r.app.world.move_targets.insert(
                entity,
                MoveTarget::new(Position2D::from_tile(gx, gy), path, PLAYER_SPEED),
            );
        }
    }
}

pub(super) fn try_use_skill(r: &mut GameRoom, entity: Entity, skill_index: u32, target: Option<u64>) {
    let Some(skill_slot) = r.app.world.skills.get(&entity).cloned() else { return };
    let idx = skill_index as usize;
    let Some(skill) = skill_slot.bar.get(idx).cloned() else { return };

    let cd_ready = skill_slot.cooldowns.get(idx).is_some_and(|c| c.is_ready());
    let mana_ok = r.app.world.mana.get(&entity).is_some_and(|m| m.current >= skill.mana_cost);
    if !cd_ready || !mana_ok {
        return;
    }

    if let Some(tid) = target {
        let target_e = Entity(tid);
        let Some(dist) = distance_between(r, entity, target_e) else { return };
        if dist > skill.range {
            return;
        }
    }

    if let Some(slot) = r.app.world.skills.get_mut(&entity) {
        slot.selected = idx;
    }
    if let Some(c) = r.app.world.combat.get_mut(&entity) {
        if let Some(tid) = target {
            c.target = Some(Entity(tid));
        }
    }
    r.app.world.input.command = Some(crate::plugins::player::PlayerCommand::UseSkill {
        idx,
        target: Entity(target.unwrap_or(entity.0)),
    });
}

pub(super) fn try_pickup_item(r: &mut GameRoom, entity: Entity, ground_item_id: u64) {
    let target = Entity(ground_item_id);
    if !r.app.world.ground_items.contains_key(&target) {
        return;
    }
    if let Some(dist) = distance_between(r, entity, target) {
        if dist > 3.0 {
            return;
        }
    }
    let prev_player = r.app.world.player_entity;
    r.app.world.player_entity = Some(entity);
    r.app.world.input.command = Some(crate::plugins::player::PlayerCommand::Pickup(target));
    let _ = prev_player;
}

pub(super) fn try_drop_item(r: &mut GameRoom, entity: Entity, slot: u32) {
    let Some(inv) = r.app.world.inventory.get_mut(&entity) else { return };
    let idx = slot as usize;
    if idx >= inv.slots.len() {
        return;
    }
    inv.slots[idx] = None;
}

pub(super) fn try_equip_item(r: &mut GameRoom, entity: Entity, slot: u32) {
    let idx = slot as usize;
    if !r
        .app
        .world
        .inventory
        .get(&entity)
        .is_some_and(|inv| idx < inv.slots.len())
    {
        return;
    }
    crate::plugins::inventory::equip_from_inventory(&mut r.app.world, entity, idx);
}

pub(super) fn try_unequip(r: &mut GameRoom, entity: Entity, slot: &str) {
    let Some(eq_slot) = parse_equip_slot(slot) else { return };
    crate::plugins::inventory::unequip_to_inventory(&mut r.app.world, entity, eq_slot);
}

pub(super) fn parse_equip_slot(s: &str) -> Option<mge_items::item::EquipSlot> {
    use mge_items::item::EquipSlot::*;
    Some(match s {
        "Head" => Head,
        "Chest" => Chest,
        "Belt" => Belt,
        "Gloves" => Gloves,
        "Boots" => Boots,
        "Ring1" => Ring1,
        "Ring2" => Ring2,
        "Amulet" => Amulet,
        "MainHand" => MainHand,
        "OffHand" => OffHand,
        _ => return None,
    })
}
