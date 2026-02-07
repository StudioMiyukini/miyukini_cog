//! État du jeu Lord of the Castle (Miyukini Survivor).
//! Phases Préparation / Bataille, vague, or, XP, entités.
//!
//! @id: lord_of_the_castle_game_state
//! @do: hold_full_game_state_and_mutations
//! @role: data
//! @layer: domain
//! @human: État complet d'une run : phases, vagues, or, XP, inventaire, équipement, marchand.

use crate::castle::Castle;
use crate::character_creation::{CharacterStats, Stat};
use crate::constants::{size, wave, TOWER_BASE_COST_GOLD};
use crate::enemies::{Enemy, EnemyKind};
use crate::troops::{Troop, TroopKind, TroopState};
use crate::warrior_skills::{prerequisites_met, warrior_skill_def, WarriorSkillId};
use crate::constants::{hp, speed};
use crate::loot::{
    generate_loot, generate_merchant_pools, roll_identification_expert, roll_identification_self,
    InventoryEntry, ItemInstance, ItemSlot, LootDrop, LootKind, MerchantEntry,
};
use std::collections::HashMap;
use crate::player::Player;
use crate::secondary_player::{SecondaryPlayer, SecondaryPlayerKind, SecondaryProjectile};
use crate::towers::{Tower, TowerProjectile};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Nombre maximum de slots d’inventaire.
pub const INVENTORY_MAX_SLOTS: usize = 20;

/// Coût d’identification par un expert (or).
pub const EXPERT_IDENTIFY_COST_GOLD: u32 = 20;

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
    /// Projectiles des tours en vol (2×2 px noir, 600 px/s).
    #[serde(default)]
    pub tower_projectiles: Vec<TowerProjectile>,
    /// Troupes alliées (zone de commandement, recrutement).
    #[serde(default)]
    pub troops: Vec<Troop>,
    /// Prochain ID ennemi.
    pub next_enemy_id: u64,
    /// Prochain ID tour.
    pub next_tower_id: u64,
    /// Prochain ID troupe.
    #[serde(default)]
    pub next_troop_id: u64,
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
    /// Points de compétence disponibles (level up).
    #[serde(default)]
    pub skill_points_available: u32,
    /// Points de statistique disponibles (tous les 5 niveaux), à placer dans For/Con/Agi/etc.
    #[serde(default)]
    pub stat_points_available: u32,
    /// Rangs des compétences Guerrier (skill_id → rang actuel, 0 = non apprise).
    #[serde(default)]
    pub warrior_skill_ranks: HashMap<WarriorSkillId, u32>,
    /// Pools du marchand (renouvelées à chaque fin de vague). Non sérialisé.
    #[serde(skip)]
    pub merchant_weapons: Vec<Option<MerchantEntry>>,
    #[serde(skip)]
    pub merchant_armor: Vec<Option<MerchantEntry>>,
    #[serde(skip)]
    pub merchant_accessories: Vec<Option<MerchantEntry>>,
    /// Accumulateur pour régénération GigaChad (1 PV/s). Non sérialisé.
    #[serde(skip)]
    pub gigachad_regen_accumulator: f32,
    /// Joueur secondaire (2 joueurs) : Tombilol, Tal Ratchou ou Sergent Garcia. 10×10 px bleu-vert fluo.
    #[serde(default)]
    pub secondary_player: Option<SecondaryPlayer>,
    /// Projectiles homing de Tal Ratchou.
    #[serde(default)]
    pub secondary_projectiles: Vec<SecondaryProjectile>,
    /// Prochain ID joueur secondaire (pour Garcia miliciens).
    #[serde(default)]
    pub next_secondary_id: u64,
    /// Notifications « niveau atteint » à afficher en bas à droite (drainées par l'app chaque frame).
    #[serde(skip, default)]
    pub pending_level_up_notifications: Vec<String>,
}

impl GameState {
    /// Nouvel état : phase Préparation, vague 1, or 0, château au centre donné (joueur par défaut).
    #[must_use] 
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
        let (w, a, acc) = generate_merchant_pools(&mut || rand_simple());
        let merchant_weapons = w.into_iter().map(Some).collect();
        let merchant_armor = a.into_iter().map(Some).collect();
        let merchant_accessories = acc.into_iter().map(Some).collect();
        Self {
            phase: GamePhase::Preparation,
            wave_number: 1,
            gold: 100,
            xp: 0,
            level: 1,
            castle: Castle::new(castle_center_x, castle_center_y),
            player,
            enemies: Vec::new(),
            towers: Vec::new(),
            tower_projectiles: Vec::new(),
            troops: Vec::new(),
            next_enemy_id: 0,
            next_tower_id: 0,
            next_troop_id: 0,
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
            skill_points_available: 0,
            stat_points_available: 0,
            warrior_skill_ranks: HashMap::new(),
            merchant_weapons,
            merchant_armor,
            merchant_accessories,
            gigachad_regen_accumulator: 0.0,
            secondary_player: None,
            secondary_projectiles: Vec::new(),
            next_secondary_id: 0,
            pending_level_up_notifications: Vec::new(),
        }
    }

    /// Régénère les 3 pools du marchand (armes, armures, accessoires). Utilisé en fin de vague et en dev (bouton reroll). Les slots vides sont remplis.
    pub fn refresh_merchant_pools(&mut self) {
        let (w, a, acc) = generate_merchant_pools(&mut || rand_simple());
        self.merchant_weapons = w.into_iter().map(Some).collect();
        self.merchant_armor = a.into_iter().map(Some).collect();
        self.merchant_accessories = acc.into_iter().map(Some).collect();
    }

    /// XP requise pour le niveau suivant : 100 + (niveau actuel).
    #[must_use] 
    pub fn xp_required_for_next_level(&self) -> u32 {
        100 + self.level
    }

    /// Applique les level up tant que l'XP le permet : niveau +1, xp remis à zéro, 1 point de compétence ; tous les 5 niveaux +1 point de statistique.
    pub fn try_level_ups(&mut self) {
        while self.xp >= self.xp_required_for_next_level() {
            self.level += 1;
            self.xp = 0;
            self.skill_points_available += 1;
            if self.level.is_multiple_of(5) {
                self.stat_points_available += 1;
            }
            self.pending_level_up_notifications
                .push(format!("Niveau {} atteint !", self.level));
        }
    }

    /// Dépense un point de statistique (panneau joueur). Met à jour hp_max (formule sur stats effectives + Pot de Wey). Retourne false si aucun point.
    pub fn spend_stat_point(&mut self, stat: Stat) -> bool {
        if self.stat_points_available == 0 {
            return false;
        }
        self.stat_points_available -= 1;
        self.player.stats.add(stat, 1);
        self.player.agility = (5 + self.player.stats.agi).max(0);
        self.player.luck = self.player.stats.luk.max(0).min(100);
        let new_hp_max = self.effective_hp_max();
        self.player.hp_max = new_hp_max;
        if self.player.hp > new_hp_max {
            self.player.hp = new_hp_max;
        }
        true
    }

    /// Bonus aux stats issus des compétences Guerrier (EncorePlusFort→For, Musculation→Con, Tireur→Dex).
    #[must_use] 
    pub fn stat_bonus_from_warrior_skills(&self) -> CharacterStats {
        use crate::warrior_skills::WarriorSkillId;
        let r = |id: WarriorSkillId| *self.warrior_skill_ranks.get(&id).unwrap_or(&0) as i32;
        CharacterStats {
            for_: r(WarriorSkillId::EncorePlusFort),
            con: r(WarriorSkillId::Musculation),
            agi: 0,
            dex: r(WarriorSkillId::Tireur),
            int: 0,
            sag: 0,
            cha: 0,
            luk: 0,
        }
    }

    /// Stats effectives = base (création + points de stat) + bonus (compétences). Utilisées dans toutes les formules.
    #[must_use] 
    pub fn effective_stats(&self) -> CharacterStats {
        let b = self.stat_bonus_from_warrior_skills();
        CharacterStats {
            for_: self.player.stats.for_ + b.for_,
            con: self.player.stats.con + b.con,
            agi: self.player.stats.agi + b.agi,
            dex: self.player.stats.dex + b.dex,
            int: self.player.stats.int + b.int,
            sag: self.player.stats.sag + b.sag,
            cha: self.player.stats.cha + b.cha,
            luk: self.player.stats.luk + b.luk,
        }
    }

    /// PV max : formule sur stats effectives, puis bonus Pot de Wey (+15 % par niveau).
    #[must_use] 
    pub fn effective_hp_max(&self) -> i32 {
        let eff = self.effective_stats();
        let base_hp = (eff.con * 2 + eff.for_).max(hp::PLAYER_MIN_MAX);
        let pot_rank = *self.warrior_skill_ranks.get(&WarriorSkillId::PotDeWey).unwrap_or(&0) as f32;
        let mult = 1.0 + 0.15 * pot_rank;
        (base_hp as f32 * mult).floor() as i32
    }

    /// Vitesse de déplacement (px/s) : formule sur agilité effective (5 + Agi).
    #[must_use] 
    pub fn effective_move_speed(&self) -> f32 {
        let agi = 5 + self.effective_stats().agi.max(0);
        speed::PLAYER_BASE * speed::PLAYER_SPEED_MULTIPLIER * (1.0 + (agi as f32 / 100.0))
    }

    /// Nombre max de troupes (Charisme effectif).
    #[must_use] 
    pub fn max_troops(&self) -> usize {
        self.effective_stats().cha.max(0) as usize
    }

    /// Grille de construction : origine (0,0) = coin inférieur gauche du château (castle.x - 20, castle.y - 20).
    /// Bâtiment 2×2 dont le coin inférieur gauche est (cell_i, cell_j) → centre monde = (castle.x + ci*20, castle.y + cj*20).
    fn cell_to_tower_center(castle_x: f32, castle_y: f32, cell_i: i32, cell_j: i32) -> (f32, f32) {
        let cell_size = size::CONSTRUCTION_CELL_SIZE;
        (
            castle_x + (cell_i as f32) * cell_size,
            castle_y + (cell_j as f32) * cell_size,
        )
    }

    /// Château occupe les cases (0,0), (1,0), (0,1), (1,1). Retourne true si on peut construire un bâtiment 2×2 dont le coin inférieur gauche est (cell_i, cell_j).
    #[must_use] 
    pub fn can_build_at_cell(&self, cell_i: i32, cell_j: i32) -> bool {
        // Pas de chevauchement avec le château (cases 0,0 à 1,1).
        let overlaps_castle = cell_i < 2 && cell_i + 2 > 0 && cell_j < 2 && cell_j + 2 > 0;
        if overlaps_castle {
            return false;
        }
        // Pas de chevauchement avec une tour existante (chaque tour 40×40 = 2×2 cases, centre = coin + 20).
        let cell_size = size::CONSTRUCTION_CELL_SIZE;
        for t in &self.towers {
            if t.hp <= 0 {
                continue;
            }
            let t_ci = ((t.x - self.castle.x) / cell_size).floor() as i32;
            let t_cj = ((t.y - self.castle.y) / cell_size).floor() as i32;
            let overlaps = cell_i < t_ci + 2 && cell_i + 2 > t_ci && cell_j < t_cj + 2 && cell_j + 2 > t_cj;
            if overlaps {
                return false;
            }
        }
        true
    }

    /// Construit une tour avec coin inférieur gauche en (cell_i, cell_j). Retourne true si la construction a eu lieu (place + or suffisant).
    pub fn build_tower_at_cell(&mut self, cell_i: i32, cell_j: i32) -> bool {
        if !self.can_build_at_cell(cell_i, cell_j) {
            return false;
        }
        if self.gold < TOWER_BASE_COST_GOLD {
            return false;
        }
        self.gold -= TOWER_BASE_COST_GOLD;
        let (cx, cy) = Self::cell_to_tower_center(self.castle.x, self.castle.y, cell_i, cell_j);
        let id = self.next_tower_id;
        self.next_tower_id = self.next_tower_id.wrapping_add(1);
        self.towers.push(Tower::new(id, cx, cy));
        true
    }

    /// Recrute une troupe du type donné si la limite n'est pas atteinte. La troupe apparaît à côté du joueur, état InZone.
    pub fn recruit_troop(&mut self, kind: TroopKind) -> bool {
        let count_active = self.troops.iter().filter(|t| t.is_active_in_squad()).count();
        if count_active >= self.max_troops() {
            return false;
        }
        let id = self.next_troop_id;
        self.next_troop_id = self.next_troop_id.wrapping_add(1);
        let hp_max = kind.hp_max_base();
        let troop = Troop {
            id,
            x: self.player.x + 20.0,
            y: self.player.y,
            hp: hp_max,
            hp_max,
            kind,
            state: TroopState::InZone,
            follow_secondary_id: None,
            target_enemy_id: None,
            last_attack: None,
        };
        self.troops.push(troop);
        true
    }

    /// Apprend ou améliore une compétence Guerrier (dépense 1 point de compétence). Retourne false si impossible.
    pub fn learn_warrior_skill(&mut self, id: WarriorSkillId) -> bool {
        if self.skill_points_available == 0 {
            return false;
        }
        let def = warrior_skill_def(id);
        let current = *self.warrior_skill_ranks.get(&id).unwrap_or(&0);
        if current >= def.max_rank {
            return false;
        }
        if !prerequisites_met(&self.warrior_skill_ranks, &def) {
            return false;
        }
        self.skill_points_available -= 1;
        *self.warrior_skill_ranks.entry(id).or_insert(0) += 1;
        // Recalcul PV max (Pot de Wey, etc.) et plafonner les PV actuels.
        let new_hp_max = self.effective_hp_max();
        self.player.hp_max = new_hp_max;
        if self.player.hp > new_hp_max {
            self.player.hp = new_hp_max;
        }
        true
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
        self.try_level_ups();
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

    /// Dégâts de l’attaque auto du joueur selon équipement :
    /// - Mains nues : For + Con/2 (min 1)
    /// - Arme CàC : dégâts de l’arme + For/2 (min 1)
    /// - Arme à distance : dégâts de l’arme + Dex/2 (min 1)
    /// Armure totale du joueur (somme des armures effectives des pièces équipées). Réduction plate ; dégâts min subis = 1.
    #[must_use] 
    pub fn player_total_armor(&self) -> i32 {
        self.equipped.values().map(super::loot::ItemInstance::effective_armor).sum()
    }

    /// Applique des dégâts bruts au joueur (après armure : réduction plate, dégâts min = 1).
    pub fn apply_damage_to_player(&mut self, raw_damage: i32) {
        let armor = self.player_total_armor();
        let actual = (raw_damage - armor).max(1);
        self.player.hp = (self.player.hp - actual).max(0);
        if self.player.hp <= 0 {
            self.player.dead = true;
        }
    }

    /// Dégâts de l'attaque auto du joueur selon équipement :
    /// - Mains nues : For + Con/2 (min 1)
    /// - Arme CàC : dégâts de l'arme + For/2 (min 1)
    /// - Arme à distance : dégâts de l'arme + Dex/2 (min 1)
    #[must_use] 
    pub fn player_auto_attack_damage(&self) -> i32 {
        let eff = self.effective_stats();
        let main_hand = self.get_equipped(ItemSlot::MainHand);
        let (weapon_dmg, is_ranged) = if let Some((d, r)) = main_hand.and_then(|w| w.effective_weapon_damage().map(|d| (d, w.is_ranged))) { (d, r) } else {
            let baston = *self.warrior_skill_ranks.get(&WarriorSkillId::Baston).unwrap_or(&0) as i32;
            let for_unarmed = eff.for_ + baston;
            return (for_unarmed + eff.con / 2).max(1);
        };
        if is_ranged {
            (weapon_dmg + eff.dex / 2).max(1)
        } else {
            (weapon_dmg + eff.for_ / 2).max(1)
        }
    }

    /// Constitution du joueur (pour PV ennemis) : 10 + Con effectif.
    #[must_use] 
    pub fn player_constitution(&self) -> i32 {
        (10 + self.effective_stats().con).max(1)
    }

    /// Démarre la phase Bataille : timer de spawn à 0 pour premier spawn immédiat.
    /// Nombre d'ennemis à spawn par batch : 10 + (vague-1)×10% de 10 + vague = 10 + (vague-1) + vague = 9 + 2×vague.
    /// Ex. vague 2 = 10+1+2 = 13, vague 3 = 10+2+3 = 15.
    pub fn start_battle_phase(&mut self) {
        self.max_wave_reached = self.max_wave_reached.max(self.wave_number);
        self.phase = GamePhase::Battle;
        self.player.x = self.castle.x;
        self.player.y = self.castle.y + 30.0;
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
        self.player.x = self.castle.x;
        self.player.y = self.castle.y + 30.0;
        self.wave_number += 1;
        self.enemies.clear();
        self.identified_this_phase = false; // une tentative d’ID par soi-même par phase
        // spawn_quantity est recalculé au début de la prochaine bataille : 10 + (vague-1) + vague
        self.spawn_rate_s = (self.spawn_rate_s * 0.99).max(0.5);
        if self.player.dead {
            self.player.revive_after_wave();
        }
        self.refresh_merchant_pools();
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
            let kind = if self.wave_number > 0 && self.wave_number.is_multiple_of(10) && created == 0 {
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
    #[must_use] 
    pub fn is_wave_won(&self) -> bool {
        self.phase == GamePhase::Battle && self.enemies.is_empty() && !self.castle.is_destroyed()
    }

    /// Game over si château détruit.
    #[must_use] 
    pub fn is_game_over(&self) -> bool {
        self.castle.is_destroyed()
    }

    /// Position aléatoire sur la bordure de la zone 800×800 (centre = château).
    /// Utilisé pour le spawn des ennemis n'importe où sur le périmètre.
    #[must_use] 
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

    /// Spawn un seul ennemi Normal à la position donnée (dev).
    pub fn spawn_enemy_normal_at(&mut self, x: f32, y: f32) {
        let constitution = self.player_constitution();
        let id = self.next_enemy_id;
        self.next_enemy_id += 1;
        let kind = EnemyKind::Normal;
        let hp_max = kind.hp_max_from_constitution(constitution);
        self.enemies.push(Enemy {
            id,
            x,
            y,
            hp: hp_max,
            hp_max,
            kind,
            damage_flash_start: None,
        });
    }

    /// Spawn 50 ennemis normaux répartis aléatoirement sur la zone de spawn (bord 800×800). Dev uniquement.
    pub fn dev_spawn_50_normal_enemies_random(&mut self) {
        for _ in 0..50 {
            let (x, y) = self.random_spawn_position_on_border();
            self.spawn_enemy_normal_at(x, y);
        }
    }

    /// Accorde un level up immédiat (niveau +1, xp remise à 0, +1 point de compétence ; +1 point de stat si niveau multiple de 5). Dev uniquement.
    pub fn dev_give_level_up(&mut self) {
        self.level += 1;
        self.xp = 0;
        self.skill_points_available += 1;
        if self.level.is_multiple_of(5) {
            self.stat_points_available += 1;
        }
        self.pending_level_up_notifications
            .push(format!("Niveau {} atteint !", self.level));
    }

    /// Spawn le joueur secondaire « Tombilol » (30 PV, attaque 360°, 30 px, 1 att/s). Un seul joueur secondaire à la fois.
    pub fn spawn_secondary_tombilol(&mut self) -> bool {
        if self.secondary_player.is_some() {
            return false;
        }
        let id = self.next_secondary_id;
        self.next_secondary_id = self.next_secondary_id.wrapping_add(1);
        let x = self.castle.x + 60.0;
        let y = self.castle.y;
        let kind = SecondaryPlayerKind::Tombilol;
        let level = 1u32;
        let hp_max = kind.hp_max_at_level(level);
        self.secondary_player = Some(SecondaryPlayer {
            id,
            x,
            y,
            kind,
            level,
            hp: hp_max,
            hp_max,
            last_attack: None,
            last_projectile_time: None,
        });
        true
    }

    /// Spawn le joueur secondaire « Tal Ratchou » (20 PV, projectile homing, 0,6 proj/s, 200 px).
    pub fn spawn_secondary_tal_ratchou(&mut self) -> bool {
        if self.secondary_player.is_some() {
            return false;
        }
        let id = self.next_secondary_id;
        self.next_secondary_id = self.next_secondary_id.wrapping_add(1);
        let x = self.castle.x + 60.0;
        let y = self.castle.y;
        let kind = SecondaryPlayerKind::TalRatchou;
        let level = 1u32;
        let hp_max = kind.hp_max_at_level(level);
        self.secondary_player = Some(SecondaryPlayer {
            id,
            x,
            y,
            kind,
            level,
            hp: hp_max,
            hp_max,
            last_attack: None,
            last_projectile_time: None,
        });
        true
    }

    /// Spawn le joueur secondaire « Sergent Garcia » (10 PV, 6 miliciens). Crée 6 troupes qui suivent Garcia.
    pub fn spawn_secondary_sergent_garcia(&mut self) -> bool {
        if self.secondary_player.is_some() {
            return false;
        }
        let id = self.next_secondary_id;
        self.next_secondary_id = self.next_secondary_id.wrapping_add(1);
        let x = self.castle.x + 60.0;
        let y = self.castle.y;
        let kind = SecondaryPlayerKind::SergentGarcia;
        let level = 1u32;
        let hp_max = kind.hp_max_at_level(level);
        self.secondary_player = Some(SecondaryPlayer {
            id,
            x,
            y,
            kind,
            level,
            hp: hp_max,
            hp_max,
            last_attack: None,
            last_projectile_time: None,
        });
        let count = kind.militiamen_count_at_level(level);
        for i in 0..count {
            let troop_id = self.next_troop_id;
            self.next_troop_id = self.next_troop_id.wrapping_add(1);
            let hp_max_t = TroopKind::Milicien.hp_max_base();
            let troop = Troop {
                id: troop_id,
                x: x + (i as f32) * 8.0 - (count as f32) * 4.0,
                y: y + 12.0,
                hp: hp_max_t,
                hp_max: hp_max_t,
                kind: TroopKind::Milicien,
                state: TroopState::InZone,
                follow_secondary_id: Some(id),
                target_enemy_id: None,
                last_attack: None,
            };
            self.troops.push(troop);
        }
        true
    }

    /// Accorde un level up au joueur secondaire (si présent).
    pub fn secondary_give_level_up(&mut self) -> bool {
        let Some(ref mut sec) = self.secondary_player else {
            return false;
        };
        sec.give_level_up();
        self.pending_level_up_notifications
            .push(format!("Joueur 2 : Niveau {} atteint !", sec.level));
        if sec.kind == SecondaryPlayerKind::SergentGarcia {
            let extra = sec.kind.militiamen_count_at_level(sec.level)
                .saturating_sub(sec.kind.militiamen_count_at_level(sec.level - 1));
            for _ in 0..extra {
                let troop_id = self.next_troop_id;
                self.next_troop_id = self.next_troop_id.wrapping_add(1);
                let hp_max_t = TroopKind::Milicien.hp_max_base();
                let troop = Troop {
                    id: troop_id,
                    x: sec.x,
                    y: sec.y + 12.0,
                    hp: hp_max_t,
                    hp_max: hp_max_t,
                    kind: TroopKind::Milicien,
                    state: TroopState::InZone,
                    follow_secondary_id: Some(sec.id),
                    target_enemy_id: None,
                    last_attack: None,
                };
                self.troops.push(troop);
            }
        }
        true
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

    /// Identifie un objet non identifié par un expert (20 or par objet). Retourne true si fait.
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
    pub fn buy_merchant_weapon(&mut self, index: usize) -> bool {
        let Some(entry) = self.merchant_weapons.get_mut(index).and_then(Option::take) else {
            return false;
        };
        if self.inventory.len() >= INVENTORY_MAX_SLOTS || self.gold < entry.price {
            self.merchant_weapons[index] = Some(entry);
            return false;
        }
        self.gold -= entry.price;
        self.inventory.push(entry.entry);
        true
    }

    pub fn buy_merchant_armor(&mut self, index: usize) -> bool {
        let Some(entry) = self.merchant_armor.get_mut(index).and_then(Option::take) else {
            return false;
        };
        if self.inventory.len() >= INVENTORY_MAX_SLOTS || self.gold < entry.price {
            self.merchant_armor[index] = Some(entry);
            return false;
        }
        self.gold -= entry.price;
        self.inventory.push(entry.entry);
        true
    }

    pub fn buy_merchant_accessory(&mut self, index: usize) -> bool {
        let Some(entry) = self.merchant_accessories.get_mut(index).and_then(Option::take) else {
            return false;
        };
        if self.inventory.len() >= INVENTORY_MAX_SLOTS || self.gold < entry.price {
            self.merchant_accessories[index] = Some(entry);
            return false;
        }
        self.gold -= entry.price;
        self.inventory.push(entry.entry);
        true
    }

    #[must_use] 
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
    #[must_use] 
    pub fn can_equip_from_inventory(&self, slot_index: usize) -> bool {
        let Some(InventoryEntry::Identified(ref item)) = self.inventory.get(slot_index) else {
            return false;
        };
        let slot = item.slot;
        // Les deux branches (arme/bouclier/ammo ou armure) appliquent la même logique :
        // si le slot est déjà occupé, il faut de la place pour le swap ; sinon OK.
        if self.equipped.contains_key(&slot) {
            self.inventory.len() < INVENTORY_MAX_SLOTS
        } else {
            true
        }
    }

    /// Équipe l’objet à l’index d’inventaire (doit être identifié). Retourne false si impossible (inventaire plein pour swap).
    pub fn equip_item(&mut self, slot_index: usize) -> bool {
        let item = match self.inventory.get(slot_index) {
            Some(InventoryEntry::Identified(it)) => it.clone(),
            Some(InventoryEntry::Unidentified(_)) | None => return false,
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
