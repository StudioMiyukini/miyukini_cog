//! État du jeu Lord of the Castle (Miyukini Survivor).
//! Phases Préparation / Bataille, vague, or, XP, entités.

use crate::castle::Castle;
use crate::constants::wave;
use crate::enemies::{Enemy, EnemyKind};
use crate::loot::{
    generate_loot, roll_identification_expert, roll_identification_self, InventoryEntry, ItemInstance,
    ItemSlot, LootDrop, LootKind,
};
use std::collections::HashMap;
use crate::player::Player;
use crate::towers::Tower;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Nombre maximum de slots d’inventaire.
pub const INVENTORY_MAX_SLOTS: usize = 20;

/// Coût d’identification par un expert (or).
pub const EXPERT_IDENTIFY_COST_GOLD: u32 = 100;

/// Phase de jeu : Préparation (construction, skills) ou Bataille (vagues).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePhase {
    Preparation,
    Battle,
}

/// État complet d'une run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Phase courante.
    pub phase: GamePhase,
    /// Numéro de la vague (1 = première vague).
    pub wave_number: u32,
    /// Or disponible.
    pub gold: u32,
    /// XP courante (vers prochain level).
    pub xp: u32,
    /// Level joueur (pour points de compétences, etc.).
    pub level: u32,
    /// Château (centre de la zone).
    pub castle: Castle,
    /// Joueur.
    pub player: Player,
    /// Ennemis vivants.
    pub enemies: Vec<Enemy>,
    /// Tours construites (non détruites).
    pub towers: Vec<Tower>,
    /// Prochain ID ennemi.
    pub next_enemy_id: u64,
    /// Prochain ID tour.
    pub next_tower_id: u64,
    /// Timer spawn : prochain spawn dans X s.
    pub spawn_timer_s: f32,
    /// Spawn quantity pour cette vague (évolue : ⌈prev×1.1⌉+1).
    pub spawn_quantity: u32,
    /// Spawn rate (s) pour cette vague (évolue : prev×0.99).
    pub spawn_rate_s: f32,
    /// Dernière mise à jour (pour delta time). Non sérialisé ; Option pour Deserialize.
    #[serde(skip)]
    pub last_update: Option<Instant>,
    /// Mode dev : affiche les cercles de portée (joueur bleu, ennemis rouge, tours orange).
    #[serde(skip)]
    pub dev_mode: bool,
    /// Derniers dégâts infligés par le joueur (log affiché en bas). Non sérialisé.
    #[serde(skip)]
    pub player_damage_log: Vec<String>,
    /// Loot au sol (or, xp, objets) — pixels à ramasser.
    pub loot_drops: Vec<LootDrop>,
    /// Inventaire : objets non identifiés (type seulement) ou identifiés.
    pub inventory: Vec<InventoryEntry>,
    /// Objets équipés par slot (par défaut : MainHand = épée courte).
    pub equipped: HashMap<ItemSlot, ItemInstance>,
    /// Une tentative d’identification par soi-même a été faite cette phase (réinitialisé en Préparation).
    #[serde(skip)]
    pub identified_this_phase: bool,
    /// Nombre d'ennemis tués (run).
    pub enemies_killed: u32,
    /// Nombre de boss tués (run).
    pub bosses_killed: u32,
    /// Or cumulé (total ramassé pendant la run).
    pub gold_total: u32,
    /// Vague max atteinte (record).
    pub max_wave_reached: u32,
    /// Ennemis déjà spawnés pour la vague en cours (plafond = spawn_quantity).
    pub enemies_spawned_this_wave: u32,
    /// Ennemis tués pendant la vague en cours (résumé fin de vague).
    pub enemies_killed_this_wave: u32,
    /// Or collecté pendant la vague en cours.
    pub gold_collected_this_wave: u32,
    /// XP gagnée pendant la vague en cours.
    pub xp_gained_this_wave: u32,
    /// Objets trouvés (ramassés) pendant la vague en cours.
    pub items_found_this_wave: u32,
}

impl GameState {
    /// Nouvel état : phase Préparation, vague 1, or 0, château au centre donné (joueur par défaut).
    pub fn new(castle_center_x: f32, castle_center_y: f32) -> Self {
        let player_x = castle_center_x - 60.0;
        let player_y = castle_center_y;
        Self::new_with_player(castle_center_x, castle_center_y, Player::new(player_x, player_y))
    }

    /// Nouvel état avec un joueur déjà créé (après parcours création de personnage).
    pub fn new_with_player(castle_center_x: f32, castle_center_y: f32, mut player: Player) -> Self {
        player.x = castle_center_x - 60.0;
        player.y = castle_center_y;
        let mut equipped = HashMap::new();
        equipped.insert(ItemSlot::MainHand, ItemInstance::default_short_sword());
        Self {
            phase: GamePhase::Preparation,
            wave_number: 1,
            gold: 0,
            xp: 0,
            level: 1,
            castle: Castle::new(castle_center_x, castle_center_y),
            player,
            enemies: Vec::new(),
            towers: Vec::new(),
            next_enemy_id: 0,
            next_tower_id: 0,
            spawn_timer_s: 0.0,
            spawn_quantity: wave::SPAWN_QUANTITY_INIT,
            spawn_rate_s: wave::SPAWN_RATE_INIT_S,
            last_update: Some(Instant::now()),
            dev_mode: false,
            player_damage_log: Vec::new(),
            loot_drops: Vec::new(),
            inventory: Vec::new(),
            equipped,
            identified_this_phase: false,
            enemies_killed: 0,
            bosses_killed: 0,
            gold_total: 0,
            max_wave_reached: 1,
            enemies_spawned_this_wave: 0,
            enemies_killed_this_wave: 0,
            gold_collected_this_wave: 0,
            xp_gained_this_wave: 0,
            items_found_this_wave: 0,
        }
    }

    /// XP requise pour le niveau suivant (formule simple : level × 100).
    pub fn xp_required_for_next_level(&self) -> u32 {
        self.level * 100
    }

    /// Spawn du loot à la mort d'un monstre (position, hp_max du monstre). Utilise player.luck comme chance.
    pub fn spawn_loot_from_kill(&mut self, x: f32, y: f32, monster_hp_max: i32) {
        let chance_pct = self.player.luck;
        let drops = generate_loot(x, y, monster_hp_max, chance_pct, &mut || rand_simple());
        self.loot_drops.extend(drops);
    }

    /// Ramasse le loot à portée du joueur (or, xp, objets). Retourne les indices des drops à retirer.
    pub fn collect_loot_near_player(&mut self, player_x: f32, player_y: f32, pickup_radius: f32) {
        let mut to_remove = Vec::new();
        for (i, drop_) in self.loot_drops.iter().enumerate() {
            let dx = drop_.x - player_x;
            let dy = drop_.y - player_y;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= pickup_radius {
                match &drop_.kind {
                    crate::loot::LootKind::Gold(amt) => {
                        self.gold += amt;
                        self.gold_total += amt;
                        self.gold_collected_this_wave += amt;
                        to_remove.push(i);
                    }
                    crate::loot::LootKind::Xp(amt) => {
                        self.xp += amt;
                        self.xp_gained_this_wave += amt;
                        to_remove.push(i);
                    }
                    LootKind::Item(slot) => {
                        if self.inventory.len() < INVENTORY_MAX_SLOTS {
                            self.inventory.push(InventoryEntry::Unidentified(*slot));
                            self.items_found_this_wave += 1;
                            to_remove.push(i);
                        }
                    }
                }
            }
        }
        // Retirer en ordre décroissant pour ne pas décaler les indices.
        for i in to_remove.into_iter().rev() {
            self.loot_drops.remove(i);
        }
    }

    /// Enregistre une ligne dans le log des dégâts joueur (max 50 lignes).
    pub fn record_player_damage(&mut self, damage: i32, enemy_id: u64) {
        self.player_damage_log
            .push(format!("Dégâts: {} (ennemi #{}), vague {}", damage, enemy_id, self.wave_number));
        if self.player_damage_log.len() > 50 {
            self.player_damage_log.remove(0);
        }
    }

    /// Constitution du joueur (pour PV ennemis) : base 10 + stats.con.
    pub fn player_constitution(&self) -> i32 {
        (10 + self.player.stats.con).max(1)
    }

    /// Démarre la phase Bataille : timer de spawn à 0 pour premier spawn immédiat.
    /// Nombre d'ennemis à spawn par batch : 10 + (vague-1)×10% de 10 + vague = 10 + (vague-1) + vague = 9 + 2×vague.
    /// Ex. vague 2 = 10+1+2 = 13, vague 3 = 10+2+3 = 15.
    pub fn start_battle_phase(&mut self) {
        self.max_wave_reached = self.max_wave_reached.max(self.wave_number);
        self.phase = GamePhase::Battle;
        self.spawn_timer_s = 0.0; // premier spawn immédiat
        self.spawn_quantity = (10 + (self.wave_number as i32 - 1) + self.wave_number as i32).max(1) as u32;
        self.enemies_spawned_this_wave = 0;
        self.enemies_killed_this_wave = 0;
        self.gold_collected_this_wave = 0;
        self.xp_gained_this_wave = 0;
        self.items_found_this_wave = 0;
        self.last_update = Some(Instant::now());
    }

    /// Passe en phase Préparation (fin de vague gagnée).
    pub fn start_preparation_phase(&mut self) {
        self.max_wave_reached = self.max_wave_reached.max(self.wave_number);
        self.phase = GamePhase::Preparation;
        self.wave_number += 1;
        self.enemies.clear();
        self.identified_this_phase = false; // une tentative d’ID par soi-même par phase
        // spawn_quantity est recalculé au début de la prochaine bataille : 10 + (vague-1) + vague
        self.spawn_rate_s = (self.spawn_rate_s * 0.99).max(0.5);
        if self.player.dead {
            self.player.revive_after_wave();
        }
        // Garder le loot au sol entre les vagues (le joueur peut encore ramasser).
        self.last_update = Some(Instant::now());
    }

    /// Crée des ennemis pour un spawn (bord de l'écran). Retourne le nombre créés.
    pub fn spawn_enemies(&mut self, spawn_x: f32, spawn_y: f32, count: u32) -> u32 {
        let constitution = self.player_constitution();
        let mut created = 0u32;
        for _ in 0..count {
            let id = self.next_enemy_id;
            self.next_enemy_id += 1;
            let kind = if self.wave_number > 0 && self.wave_number % 10 == 0 && created == 0 {
                EnemyKind::Boss
            } else if rand_simple() < 0.15 {
                EnemyKind::MiniBoss
            } else {
                EnemyKind::Normal
            };
            let hp_max = kind.hp_max_from_constitution(constitution);
            self.enemies.push(Enemy {
                id,
                x: spawn_x,
                y: spawn_y,
                hp: hp_max,
                hp_max,
                kind,
                damage_flash_start: None,
            });
            created += 1;
        }
        created
    }

    /// Vague gagnée si plus d'ennemis et château vivant.
    pub fn is_wave_won(&self) -> bool {
        self.phase == GamePhase::Battle && self.enemies.is_empty() && !self.castle.is_destroyed()
    }

    /// Game over si château détruit.
    pub fn is_game_over(&self) -> bool {
        self.castle.is_destroyed()
    }

    /// Position aléatoire sur la bordure de la zone 800×800 (centre = château).
    /// Utilisé pour le spawn des ennemis n'importe où sur le périmètre.
    pub fn random_spawn_position_on_border(&self) -> (f32, f32) {
        use crate::constants::COMBAT_SURFACE_SIZE;
        let half = COMBAT_SURFACE_SIZE / 2.0;
        let cx = self.castle.x;
        let cy = self.castle.y;
        // Périmètre = 4 * 800 = 3200. Tirage uniforme sur [0, 3200).
        let t = rand_simple() * 3200.0f32;
        if t < 800.0 {
            // Bord haut : y = cy - half, x de cx - half à cx + half
            (cx - half + t, cy - half)
        } else if t < 1600.0 {
            // Bord droit : x = cx + half, y de cy - half à cy + half
            (cx + half, cy - half + (t - 800.0))
        } else if t < 2400.0 {
            // Bord bas : y = cy + half, x de cx + half à cx - half
            (cx + half - (t - 1600.0), cy + half)
        } else {
            // Bord gauche : x = cx - half, y de cy + half à cy - half
            (cx - half, cy + half - (t - 2400.0))
        }
    }

    /// Identifie un objet non identifié par soi-même (une tentative par phase). Retourne true si fait.
    pub fn identify_self(&mut self, slot_index: usize) -> bool {
        if self.identified_this_phase {
            return false;
        }
        let slot = match self.inventory.get(slot_index) {
            Some(InventoryEntry::Unidentified(s)) => *s,
            _ => return false,
        };
        let mut roll = || rand_simple();
        let item = roll_identification_self(
            &mut roll,
            self.player.luck,
            self.player.stats.sag,
            slot,
        );
        self.inventory[slot_index] = InventoryEntry::Identified(item);
        self.identified_this_phase = true;
        true
    }

    /// Identifie un objet non identifié par un expert (100 or). Retourne true si fait.
    pub fn identify_expert(&mut self, slot_index: usize) -> bool {
        if self.gold < EXPERT_IDENTIFY_COST_GOLD {
            return false;
        }
        let slot = match self.inventory.get(slot_index) {
            Some(InventoryEntry::Unidentified(s)) => *s,
            _ => return false,
        };
        self.gold -= EXPERT_IDENTIFY_COST_GOLD;
        let mut roll = || rand_simple();
        let item = roll_identification_expert(&mut roll, slot);
        self.inventory[slot_index] = InventoryEntry::Identified(item);
        true
    }

    /// Vend un objet identifié à l’index donné. Retourne true si vendu.
    pub fn sell_item(&mut self, slot_index: usize) -> bool {
        let price = match self.inventory.get(slot_index) {
            Some(InventoryEntry::Identified(item)) => item.sell_price(),
            _ => return false,
        };
        self.gold += price;
        self.gold_total += price;
        self.inventory.remove(slot_index);
        true
    }

    /// Retourne l’objet équipé dans le slot, s’il y en a un.
    pub fn get_equipped(&self, slot: ItemSlot) -> Option<&ItemInstance> {
        self.equipped.get(&slot)
    }

    /// Déséquipe le slot et met l’objet dans l’inventaire. Retourne false si inventaire plein.
    pub fn unequip_to_inventory(&mut self, slot: ItemSlot) -> bool {
        let Some(item) = self.equipped.remove(&slot) else {
            return true;
        };
        if self.inventory.len() >= INVENTORY_MAX_SLOTS {
            self.equipped.insert(slot, item);
            return false;
        }
        self.inventory.push(InventoryEntry::Identified(item));
        true
    }

    /// Peut-on équiper l’objet à l’index d’inventaire ? (slot libre ou place en inventaire pour l’objet actuellement équipé)
    pub fn can_equip_from_inventory(&self, slot_index: usize) -> bool {
        let Some(InventoryEntry::Identified(ref item)) = self.inventory.get(slot_index) else {
            return false;
        };
        let slot = item.slot;
        if !slot.is_weapon_or_shield() && !slot.is_ammo() {
            // Slots équipement (armure, etc.)
            if self.equipped.contains_key(&slot) {
                self.inventory.len() < INVENTORY_MAX_SLOTS
            } else {
                true
            }
        } else {
            if self.equipped.contains_key(&slot) {
                self.inventory.len() < INVENTORY_MAX_SLOTS
            } else {
                true
            }
        }
    }

    /// Équipe l’objet à l’index d’inventaire (doit être identifié). Retourne false si impossible (inventaire plein pour swap).
    pub fn equip_item(&mut self, slot_index: usize) -> bool {
        let item = match self.inventory.get(slot_index) {
            Some(InventoryEntry::Identified(it)) => it.clone(),
            _ => return false,
        };
        let slot = item.slot;
        if self.equipped.contains_key(&slot) {
            if self.inventory.len() >= INVENTORY_MAX_SLOTS {
                return false;
            }
            let old = self.equipped.remove(&slot).unwrap();
            self.inventory.push(InventoryEntry::Identified(old));
        }
        self.inventory.remove(slot_index);
        self.equipped.insert(slot, item);
        true
    }
}

/// Générateur pseudo-aléatoire minimal (pour spawn, loot, création perso). Utilise le temps + compteur.
pub(crate) fn rand_simple() -> f32 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEED: AtomicU64 = AtomicU64::new(12345);
    let s = SEED.fetch_add(1, Ordering::Relaxed);
    let t = Instant::now().elapsed().as_nanos() as u64;
    let x = s.wrapping_add(t) % 1000;
    (x as f32) / 1000.0
}
