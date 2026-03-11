// ---------------------------------------------------------------------------
// Civil Skills — système de compétences artisanales / civiles
// Séparé du disque de combat (skill_disk.rs)
// ---------------------------------------------------------------------------

/// Les 11 compétences civiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CivilSkillId {
    Forge       = 0,  // Smithing — armes, armures métalliques
    Menuiserie  = 1,  // Woodworking — arcs, bâtons, boucliers bois
    Alchimie    = 2,  // Alchemy — potions, élixirs, poisons
    Domptage    = 3,  // Taming — capturer et dresser des créatures
    Bucheronage = 4,  // Lumberjacking — récolter du bois
    Minage      = 5,  // Mining — extraire minerais et gemmes
    Cueillette  = 6,  // Gathering — herbes, plantes, champignons
    Elevage     = 7,  // Breeding — élever des animaux, produire ressources
    Agriculture = 8,  // Farming — cultiver, récolter des cultures
    Construction= 9,  // Building — bâtiments, fortifications, ateliers
    Commerce    = 10, // Trading — acheter/vendre, prix, caravanes
}

pub const CIVIL_SKILL_COUNT: usize = 11;
pub const CIVIL_MAX_LEVEL: u32 = 100;

impl CivilSkillId {
    pub const ALL: [CivilSkillId; CIVIL_SKILL_COUNT] = [
        CivilSkillId::Forge,
        CivilSkillId::Menuiserie,
        CivilSkillId::Alchimie,
        CivilSkillId::Domptage,
        CivilSkillId::Bucheronage,
        CivilSkillId::Minage,
        CivilSkillId::Cueillette,
        CivilSkillId::Elevage,
        CivilSkillId::Agriculture,
        CivilSkillId::Construction,
        CivilSkillId::Commerce,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Forge       => "Forge",
            Self::Menuiserie  => "Menuiserie",
            Self::Alchimie    => "Alchimie",
            Self::Domptage    => "Domptage",
            Self::Bucheronage => "Bucheronage",
            Self::Minage      => "Minage",
            Self::Cueillette  => "Cueillette",
            Self::Elevage     => "Elevage",
            Self::Agriculture => "Agriculture",
            Self::Construction=> "Construction",
            Self::Commerce    => "Commerce",
        }
    }

    /// Nom anglais court pour les identifiants internes.
    pub fn key(self) -> &'static str {
        match self {
            Self::Forge       => "forge",
            Self::Menuiserie  => "woodwork",
            Self::Alchimie    => "alchemy",
            Self::Domptage    => "taming",
            Self::Bucheronage => "lumber",
            Self::Minage      => "mining",
            Self::Cueillette  => "gather",
            Self::Elevage     => "breeding",
            Self::Agriculture => "farming",
            Self::Construction=> "building",
            Self::Commerce    => "trading",
        }
    }

    /// Icône / lettre pour l'UI compacte.
    pub fn icon(self) -> u8 {
        match self {
            Self::Forge       => b'F',
            Self::Menuiserie  => b'W',
            Self::Alchimie    => b'A',
            Self::Domptage    => b'D',
            Self::Bucheronage => b'B',
            Self::Minage      => b'M',
            Self::Cueillette  => b'C',
            Self::Elevage     => b'E',
            Self::Agriculture => b'G',
            Self::Construction=> b'T',
            Self::Commerce    => b'$',
        }
    }

    /// Couleur RGB pour l'UI [r, g, b] (0.0 .. 1.0).
    pub fn color(self) -> [f32; 3] {
        match self {
            Self::Forge       => [0.9, 0.45, 0.2],  // orange forge
            Self::Menuiserie  => [0.65, 0.45, 0.25], // brun bois
            Self::Alchimie    => [0.3, 0.85, 0.4],   // vert potion
            Self::Domptage    => [0.85, 0.7, 0.3],   // doré animal
            Self::Bucheronage => [0.45, 0.6, 0.25],  // vert forêt
            Self::Minage      => [0.6, 0.6, 0.65],   // gris pierre
            Self::Cueillette  => [0.5, 0.8, 0.5],    // vert herbe
            Self::Elevage     => [0.75, 0.55, 0.4],   // brun cuir
            Self::Agriculture => [0.7, 0.75, 0.3],    // jaune blé
            Self::Construction=> [0.55, 0.45, 0.35],  // brun terre
            Self::Commerce    => [0.9, 0.8, 0.3],     // or
        }
    }

    /// Description courte de la compétence.
    pub fn description(self) -> &'static str {
        match self {
            Self::Forge       => "Forger armes et armures metalliques",
            Self::Menuiserie  => "Fabriquer arcs, batons et boucliers en bois",
            Self::Alchimie    => "Brasser potions, elixirs et poisons",
            Self::Domptage    => "Capturer et dresser des creatures",
            Self::Bucheronage => "Abattre des arbres et recolter du bois",
            Self::Minage      => "Extraire minerais et gemmes des roches",
            Self::Cueillette  => "Recolter herbes, plantes et champignons",
            Self::Elevage     => "Elever des animaux et produire des ressources",
            Self::Agriculture => "Cultiver et recolter des cultures",
            Self::Construction=> "Batir structures, fortifications et ateliers",
            Self::Commerce    => "Negocier, acheter et vendre avec bonus",
        }
    }
}

// ---------------------------------------------------------------------------
// XP curve — XP requise pour atteindre le niveau suivant
// Courbe quadratique : xp_for_next(lv) = BASE + lv² × SCALE
// ---------------------------------------------------------------------------

/// XP de base pour passer du niveau 1 au 2.
pub const CIVIL_XP_BASE: f32 = 50.0;
/// Facteur quadratique de la courbe.
pub const CIVIL_XP_SCALE: f32 = 12.0;

/// XP nécessaire pour passer de `level` à `level + 1`.
pub fn xp_for_next_level(level: u32) -> f32 {
    CIVIL_XP_BASE + (level as f32) * (level as f32) * CIVIL_XP_SCALE
}

/// XP cumulée totale pour atteindre `level` depuis le niveau 1.
pub fn xp_total_for_level(level: u32) -> f32 {
    let mut total = 0.0;
    for lv in 1..level {
        total += xp_for_next_level(lv);
    }
    total
}

// ---------------------------------------------------------------------------
// État d'une compétence civile individuelle
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CivilSkillState {
    pub id: CivilSkillId,
    pub level: u32,
    pub xp: f32,
}

impl CivilSkillState {
    pub fn new(id: CivilSkillId) -> Self {
        Self { id, level: 1, xp: 0.0 }
    }

    /// XP requise pour le prochain niveau.
    pub fn xp_to_next(&self) -> f32 {
        if self.level >= CIVIL_MAX_LEVEL { return 0.0; }
        xp_for_next_level(self.level)
    }

    /// Progression vers le prochain niveau (0.0 .. 1.0).
    pub fn progress(&self) -> f32 {
        let needed = self.xp_to_next();
        if needed <= 0.0 { return 1.0; }
        (self.xp / needed).min(1.0)
    }

    /// Ajouter de l'XP et gérer les level-ups. Retourne le nombre de niveaux gagnés.
    pub fn add_xp(&mut self, amount: f32) -> u32 {
        if self.level >= CIVIL_MAX_LEVEL { return 0; }
        self.xp += amount;
        let mut levels_gained = 0_u32;
        loop {
            if self.level >= CIVIL_MAX_LEVEL { break; }
            let needed = xp_for_next_level(self.level);
            if self.xp < needed { break; }
            self.xp -= needed;
            self.level += 1;
            levels_gained += 1;
        }
        if self.level >= CIVIL_MAX_LEVEL {
            self.xp = 0.0;
        }
        levels_gained
    }
}

// ---------------------------------------------------------------------------
// Recettes — chaque recette requiert un niveau minimum et des ressources
// ---------------------------------------------------------------------------

/// Types de ressources récoltables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ResourceKind {
    // Bûcheronage
    BoisBrut      = 0,
    BoisDur       = 1,
    BoisAncien    = 2,
    // Minage
    PierreBreute  = 3,
    MineraiFer    = 4,
    MineraiArgent = 5,
    MineraiOr     = 6,
    Gemme         = 7,
    // Cueillette
    HerbeCommune  = 8,
    HerbeRare     = 9,
    Champignon    = 10,
    FleurMagique  = 11,
    // Agriculture
    Ble           = 12,
    Legume        = 13,
    Fruit         = 14,
    // Élevage
    Cuir          = 15,
    Laine         = 16,
    Oeuf          = 17,
    Lait          = 18,
}

pub const RESOURCE_KIND_COUNT: usize = 19;

impl ResourceKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::BoisBrut      => "Bois brut",
            Self::BoisDur       => "Bois dur",
            Self::BoisAncien    => "Bois ancien",
            Self::PierreBreute  => "Pierre brute",
            Self::MineraiFer    => "Minerai de fer",
            Self::MineraiArgent => "Minerai d'argent",
            Self::MineraiOr     => "Minerai d'or",
            Self::Gemme         => "Gemme",
            Self::HerbeCommune  => "Herbe commune",
            Self::HerbeRare     => "Herbe rare",
            Self::Champignon    => "Champignon",
            Self::FleurMagique  => "Fleur magique",
            Self::Ble           => "Ble",
            Self::Legume        => "Legume",
            Self::Fruit         => "Fruit",
            Self::Cuir          => "Cuir",
            Self::Laine         => "Laine",
            Self::Oeuf          => "Oeuf",
            Self::Lait          => "Lait",
        }
    }

    /// Compétence civile qui récolte cette ressource.
    pub fn gathering_skill(self) -> CivilSkillId {
        match self {
            Self::BoisBrut | Self::BoisDur | Self::BoisAncien => CivilSkillId::Bucheronage,
            Self::PierreBreute | Self::MineraiFer | Self::MineraiArgent
                | Self::MineraiOr | Self::Gemme => CivilSkillId::Minage,
            Self::HerbeCommune | Self::HerbeRare | Self::Champignon
                | Self::FleurMagique => CivilSkillId::Cueillette,
            Self::Ble | Self::Legume | Self::Fruit => CivilSkillId::Agriculture,
            Self::Cuir | Self::Laine | Self::Oeuf | Self::Lait => CivilSkillId::Elevage,
        }
    }

    /// Niveau minimum de la compétence de récolte pour obtenir cette ressource.
    pub fn min_gather_level(self) -> u32 {
        match self {
            Self::BoisBrut | Self::PierreBreute | Self::HerbeCommune
                | Self::Ble | Self::Cuir | Self::Oeuf => 1,
            Self::BoisDur | Self::MineraiFer | Self::Champignon
                | Self::Legume | Self::Laine | Self::Lait => 15,
            Self::BoisAncien | Self::MineraiArgent | Self::HerbeRare
                | Self::Fruit => 35,
            Self::MineraiOr | Self::FleurMagique => 55,
            Self::Gemme => 75,
        }
    }

    /// XP donnée à la compétence de récolte quand on récolte 1 unité.
    pub fn gather_xp(self) -> f32 {
        match self {
            Self::BoisBrut | Self::PierreBreute | Self::HerbeCommune
                | Self::Ble | Self::Cuir | Self::Oeuf => 8.0,
            Self::BoisDur | Self::MineraiFer | Self::Champignon
                | Self::Legume | Self::Laine | Self::Lait => 18.0,
            Self::BoisAncien | Self::MineraiArgent | Self::HerbeRare
                | Self::Fruit => 35.0,
            Self::MineraiOr | Self::FleurMagique => 55.0,
            Self::Gemme => 80.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Produits craftés
// ---------------------------------------------------------------------------

/// Catégorie de produit craftable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CraftCategory {
    // Forge
    EpeeFer       = 0,
    EpeeAcier     = 1,
    ArmureFer     = 2,
    ArmureAcier   = 3,
    CasqueFer     = 4,
    BouclierFer   = 5,
    // Menuiserie
    Arc           = 6,
    ArcLong       = 7,
    Baton         = 8,
    BouclierBois  = 9,
    // Alchimie
    PotionSoin    = 10,
    PotionMana    = 11,
    PotionForce   = 12,
    PotionVitesse = 13,
    ElixirResist  = 14,
    PoisonLame    = 15,
    // Construction
    Atelier       = 16,
    Fortification = 17,
    MaisonBois    = 18,
    Entrepot      = 19,
}

pub const CRAFT_CATEGORY_COUNT: usize = 20;

/// Ingrédient d'une recette : ressource + quantité.
#[derive(Debug, Clone, Copy)]
pub struct Ingredient {
    pub resource: ResourceKind,
    pub quantity: u32,
}

/// Définition d'une recette.
#[derive(Debug, Clone)]
pub struct Recipe {
    pub product: CraftCategory,
    pub name: &'static str,
    pub skill: CivilSkillId,
    pub min_level: u32,
    pub ingredients: &'static [Ingredient],
    pub craft_xp: f32,
    pub craft_ticks: u32,  // durée en ticks de jeu
}

/// XP de craft ajoutée à la compétence Commerce pour chaque vente.
pub const TRADE_XP_PER_SALE: f32 = 5.0;
/// Bonus prix de vente par niveau Commerce : +0.5% / niveau.
pub const TRADE_PRICE_BONUS_PER_LEVEL: f32 = 0.005;
/// Bonus prix d'achat (réduction) par niveau Commerce : -0.3% / niveau.
pub const TRADE_BUY_DISCOUNT_PER_LEVEL: f32 = 0.003;

// Macro helper pour les ingrédients const
macro_rules! ingredients {
    ( $( $res:expr => $qty:expr ),+ $(,)? ) => {
        &[ $( Ingredient { resource: $res, quantity: $qty } ),+ ]
    };
}

/// Catalogue de toutes les recettes.
pub fn all_recipes() -> &'static [Recipe] {
    use ResourceKind::*;
    use CivilSkillId::*;
    use CraftCategory::*;

    static RECIPES: &[Recipe] = &[
        // ===== FORGE =====
        Recipe {
            product: EpeeFer, name: "Epee de fer", skill: Forge, min_level: 1,
            ingredients: ingredients!(MineraiFer => 3, BoisBrut => 1),
            craft_xp: 25.0, craft_ticks: 120,
        },
        Recipe {
            product: EpeeAcier, name: "Epee d'acier", skill: Forge, min_level: 25,
            ingredients: ingredients!(MineraiFer => 5, MineraiArgent => 2),
            craft_xp: 60.0, craft_ticks: 200,
        },
        Recipe {
            product: ArmureFer, name: "Armure de fer", skill: Forge, min_level: 10,
            ingredients: ingredients!(MineraiFer => 6, Cuir => 2),
            craft_xp: 40.0, craft_ticks: 180,
        },
        Recipe {
            product: ArmureAcier, name: "Armure d'acier", skill: Forge, min_level: 35,
            ingredients: ingredients!(MineraiFer => 8, MineraiArgent => 3, Cuir => 3),
            craft_xp: 80.0, craft_ticks: 300,
        },
        Recipe {
            product: CasqueFer, name: "Casque de fer", skill: Forge, min_level: 5,
            ingredients: ingredients!(MineraiFer => 4),
            craft_xp: 30.0, craft_ticks: 140,
        },
        Recipe {
            product: BouclierFer, name: "Bouclier de fer", skill: Forge, min_level: 15,
            ingredients: ingredients!(MineraiFer => 5, BoisDur => 2),
            craft_xp: 45.0, craft_ticks: 160,
        },

        // ===== MENUISERIE =====
        Recipe {
            product: Arc, name: "Arc court", skill: Menuiserie, min_level: 1,
            ingredients: ingredients!(BoisBrut => 3, Cuir => 1),
            craft_xp: 20.0, craft_ticks: 100,
        },
        Recipe {
            product: ArcLong, name: "Arc long", skill: Menuiserie, min_level: 20,
            ingredients: ingredients!(BoisDur => 4, Cuir => 2),
            craft_xp: 50.0, craft_ticks: 160,
        },
        Recipe {
            product: Baton, name: "Baton magique", skill: Menuiserie, min_level: 10,
            ingredients: ingredients!(BoisDur => 3, FleurMagique => 1),
            craft_xp: 35.0, craft_ticks: 120,
        },
        Recipe {
            product: BouclierBois, name: "Bouclier de bois", skill: Menuiserie, min_level: 5,
            ingredients: ingredients!(BoisBrut => 5, Cuir => 1),
            craft_xp: 25.0, craft_ticks: 110,
        },

        // ===== ALCHIMIE =====
        Recipe {
            product: PotionSoin, name: "Potion de soin", skill: Alchimie, min_level: 1,
            ingredients: ingredients!(HerbeCommune => 2),
            craft_xp: 15.0, craft_ticks: 60,
        },
        Recipe {
            product: PotionMana, name: "Potion de mana", skill: Alchimie, min_level: 5,
            ingredients: ingredients!(HerbeCommune => 1, FleurMagique => 1),
            craft_xp: 20.0, craft_ticks: 60,
        },
        Recipe {
            product: PotionForce, name: "Potion de force", skill: Alchimie, min_level: 20,
            ingredients: ingredients!(HerbeRare => 2, Champignon => 1),
            craft_xp: 40.0, craft_ticks: 90,
        },
        Recipe {
            product: PotionVitesse, name: "Potion de vitesse", skill: Alchimie, min_level: 25,
            ingredients: ingredients!(HerbeRare => 1, FleurMagique => 1),
            craft_xp: 45.0, craft_ticks: 90,
        },
        Recipe {
            product: ElixirResist, name: "Elixir de resistance", skill: Alchimie, min_level: 40,
            ingredients: ingredients!(HerbeRare => 3, Champignon => 2, Gemme => 1),
            craft_xp: 70.0, craft_ticks: 150,
        },
        Recipe {
            product: PoisonLame, name: "Poison de lame", skill: Alchimie, min_level: 30,
            ingredients: ingredients!(Champignon => 3, HerbeRare => 1),
            craft_xp: 55.0, craft_ticks: 100,
        },

        // ===== CONSTRUCTION =====
        Recipe {
            product: Atelier, name: "Atelier", skill: Construction, min_level: 10,
            ingredients: ingredients!(BoisBrut => 10, PierreBreute => 5),
            craft_xp: 50.0, craft_ticks: 400,
        },
        Recipe {
            product: Fortification, name: "Fortification", skill: Construction, min_level: 30,
            ingredients: ingredients!(PierreBreute => 15, BoisDur => 8, MineraiFer => 4),
            craft_xp: 90.0, craft_ticks: 600,
        },
        Recipe {
            product: MaisonBois, name: "Maison en bois", skill: Construction, min_level: 20,
            ingredients: ingredients!(BoisBrut => 20, BoisDur => 5, PierreBreute => 3),
            craft_xp: 70.0, craft_ticks: 500,
        },
        Recipe {
            product: Entrepot, name: "Entrepot", skill: Construction, min_level: 15,
            ingredients: ingredients!(BoisBrut => 15, PierreBreute => 8),
            craft_xp: 60.0, craft_ticks: 450,
        },
    ];

    RECIPES
}

// ---------------------------------------------------------------------------
// Bonus passifs par niveau de compétence civile
// ---------------------------------------------------------------------------

/// Bonus passifs que le joueur reçoit grâce à ses niveaux civils.
#[derive(Debug, Clone, Default)]
pub struct CivilBonuses {
    /// Forge : +defense_flat (0.3 / niveau)
    pub defense_flat: f32,
    /// Forge : qualité crafts (niveau / 100)
    pub forge_quality: f32,
    /// Menuiserie : +dégâts armes bois %
    pub wood_weapon_dmg_pct: f32,
    /// Alchimie : +efficacité potions %
    pub potion_efficiency_pct: f32,
    /// Domptage : nombre max compagnons
    pub max_companions: u32,
    /// Domptage : bonus dégâts compagnon %
    pub companion_dmg_pct: f32,
    /// Bûcheronage : vitesse récolte bois %
    pub lumber_speed_pct: f32,
    /// Minage : vitesse récolte minerai %
    pub mining_speed_pct: f32,
    /// Cueillette : chance double récolte %
    pub gather_double_pct: f32,
    /// Élevage : rendement produits animaux %
    pub breeding_yield_pct: f32,
    /// Agriculture : rendement cultures %
    pub farm_yield_pct: f32,
    /// Construction : solidité bâtiments %
    pub building_hp_pct: f32,
    /// Commerce : réduction prix achat %
    pub buy_discount_pct: f32,
    /// Commerce : bonus prix vente %
    pub sell_bonus_pct: f32,
}

// ---------------------------------------------------------------------------
// Tableau complet des compétences civiles du joueur
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CivilSkills {
    pub skills: [CivilSkillState; CIVIL_SKILL_COUNT],
    pub inventory: [u32; RESOURCE_KIND_COUNT],
    pub bonuses: CivilBonuses,
}

impl CivilSkills {
    pub fn new() -> Self {
        let skills = CivilSkillId::ALL.map(CivilSkillState::new);
        Self {
            skills,
            inventory: [0; RESOURCE_KIND_COUNT],
            bonuses: CivilBonuses::default(),
        }
    }

    /// Accès à une compétence par ID.
    pub fn skill(&self, id: CivilSkillId) -> &CivilSkillState {
        &self.skills[id as usize]
    }

    /// Accès mutable à une compétence par ID.
    pub fn skill_mut(&mut self, id: CivilSkillId) -> &mut CivilSkillState {
        &mut self.skills[id as usize]
    }

    /// Quantité de ressource en inventaire.
    pub fn resource_count(&self, r: ResourceKind) -> u32 {
        self.inventory[r as usize]
    }

    /// Ajouter des ressources à l'inventaire.
    pub fn add_resource(&mut self, r: ResourceKind, qty: u32) {
        self.inventory[r as usize] = self.inventory[r as usize].saturating_add(qty);
    }

    /// Retirer des ressources. Retourne false si pas assez.
    pub fn remove_resource(&mut self, r: ResourceKind, qty: u32) -> bool {
        let slot = &mut self.inventory[r as usize];
        if *slot < qty { return false; }
        *slot -= qty;
        true
    }

    /// Vérifier si le joueur peut récolter une ressource donnée.
    pub fn can_gather(&self, r: ResourceKind) -> bool {
        let skill = self.skill(r.gathering_skill());
        skill.level >= r.min_gather_level()
    }

    /// Récolter une ressource : ajoute l'item + donne l'XP. Retourne (quantité, niveaux gagnés).
    pub fn gather(&mut self, r: ResourceKind) -> Option<(u32, u32)> {
        if !self.can_gather(r) { return None; }
        // Quantité : 1 de base, +1 si double récolte (cueillette bonus)
        let mut qty = 1_u32;
        if r.gathering_skill() == CivilSkillId::Cueillette && self.bonuses.gather_double_pct > 0.0 {
            // Pseudo random basé sur le niveau — simplifié, dans le vrai jeu ce sera un RNG
            let chance = self.bonuses.gather_double_pct;
            if chance >= 1.0 { qty += 1; }
        }
        // Rendement agriculture / élevage
        match r.gathering_skill() {
            CivilSkillId::Agriculture => {
                if self.bonuses.farm_yield_pct >= 0.5 { qty += 1; }
            }
            CivilSkillId::Elevage => {
                if self.bonuses.breeding_yield_pct >= 0.5 { qty += 1; }
            }
            _ => {}
        }
        self.add_resource(r, qty);
        let xp = r.gather_xp();
        let skill_id = r.gathering_skill();
        let levels = self.skill_mut(skill_id).add_xp(xp);
        if levels > 0 { self.recompute_bonuses(); }
        Some((qty, levels))
    }

    /// Vérifier si une recette peut être craftée.
    pub fn can_craft(&self, recipe: &Recipe) -> bool {
        if self.skill(recipe.skill).level < recipe.min_level { return false; }
        for ing in recipe.ingredients {
            if self.resource_count(ing.resource) < ing.quantity { return false; }
        }
        true
    }

    /// Crafter une recette : consomme les ressources, donne l'XP. Retourne niveaux gagnés.
    pub fn craft(&mut self, recipe: &Recipe) -> Option<u32> {
        if !self.can_craft(recipe) { return None; }
        for ing in recipe.ingredients {
            self.remove_resource(ing.resource, ing.quantity);
        }
        let levels = self.skill_mut(recipe.skill).add_xp(recipe.craft_xp);
        if levels > 0 { self.recompute_bonuses(); }
        Some(levels)
    }

    /// Bonus de prix de vente Commerce.
    pub fn sell_price_multiplier(&self) -> f32 {
        1.0 + self.skill(CivilSkillId::Commerce).level as f32 * TRADE_PRICE_BONUS_PER_LEVEL
    }

    /// Réduction de prix d'achat Commerce.
    pub fn buy_price_multiplier(&self) -> f32 {
        1.0 - self.skill(CivilSkillId::Commerce).level as f32 * TRADE_BUY_DISCOUNT_PER_LEVEL
    }

    /// Ajouter de l'XP Commerce lors d'une vente.
    pub fn record_sale(&mut self) -> u32 {
        let levels = self.skill_mut(CivilSkillId::Commerce).add_xp(TRADE_XP_PER_SALE);
        if levels > 0 { self.recompute_bonuses(); }
        levels
    }

    /// Recalculer tous les bonus passifs d'après les niveaux actuels.
    pub fn recompute_bonuses(&mut self) {
        let b = &mut self.bonuses;
        *b = CivilBonuses::default();

        let forge_lv = self.skills[CivilSkillId::Forge as usize].level as f32;
        b.defense_flat = forge_lv * 0.3;
        b.forge_quality = forge_lv / 100.0;

        let menu_lv = self.skills[CivilSkillId::Menuiserie as usize].level as f32;
        b.wood_weapon_dmg_pct = menu_lv * 0.005;

        let alch_lv = self.skills[CivilSkillId::Alchimie as usize].level as f32;
        b.potion_efficiency_pct = alch_lv * 0.008;

        let domp_lv = self.skills[CivilSkillId::Domptage as usize].level;
        b.max_companions = 1 + domp_lv / 25; // 1 at lv1, 2 at lv25, 3 at lv50, 4 at lv75, 5 at lv100
        b.companion_dmg_pct = domp_lv as f32 * 0.006;

        let buch_lv = self.skills[CivilSkillId::Bucheronage as usize].level as f32;
        b.lumber_speed_pct = buch_lv * 0.01;

        let mine_lv = self.skills[CivilSkillId::Minage as usize].level as f32;
        b.mining_speed_pct = mine_lv * 0.01;

        let cueil_lv = self.skills[CivilSkillId::Cueillette as usize].level as f32;
        b.gather_double_pct = cueil_lv * 0.005;

        let elev_lv = self.skills[CivilSkillId::Elevage as usize].level as f32;
        b.breeding_yield_pct = elev_lv * 0.008;

        let agri_lv = self.skills[CivilSkillId::Agriculture as usize].level as f32;
        b.farm_yield_pct = agri_lv * 0.008;

        let cons_lv = self.skills[CivilSkillId::Construction as usize].level as f32;
        b.building_hp_pct = cons_lv * 0.01;

        let comm_lv = self.skills[CivilSkillId::Commerce as usize].level as f32;
        b.buy_discount_pct = comm_lv * TRADE_BUY_DISCOUNT_PER_LEVEL;
        b.sell_bonus_pct = comm_lv * TRADE_PRICE_BONUS_PER_LEVEL;
    }
}

impl Default for CivilSkills {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xp_curve_monotonic() {
        for lv in 1..CIVIL_MAX_LEVEL {
            assert!(xp_for_next_level(lv + 1) > xp_for_next_level(lv));
        }
    }

    #[test]
    fn test_level_up() {
        let mut s = CivilSkillState::new(CivilSkillId::Forge);
        assert_eq!(s.level, 1);
        // Level 1→2 needs 50 + 1*12 = 62 XP
        let gained = s.add_xp(62.0);
        assert_eq!(gained, 1);
        assert_eq!(s.level, 2);
    }

    #[test]
    fn test_multi_level_up() {
        let mut s = CivilSkillState::new(CivilSkillId::Minage);
        // Dump enough XP to level multiple times
        let gained = s.add_xp(10000.0);
        assert!(gained >= 2);
        assert!(s.level > 2);
    }

    #[test]
    fn test_max_level_cap() {
        let mut s = CivilSkillState::new(CivilSkillId::Commerce);
        s.add_xp(f32::MAX / 2.0);
        assert_eq!(s.level, CIVIL_MAX_LEVEL);
        // No more levels after max
        let gained = s.add_xp(99999.0);
        assert_eq!(gained, 0);
    }

    #[test]
    fn test_gather_requires_level() {
        let cs = CivilSkills::new();
        // Level 1 can gather BoisBrut (min 1) but not BoisDur (min 15)
        assert!(cs.can_gather(ResourceKind::BoisBrut));
        assert!(!cs.can_gather(ResourceKind::BoisDur));
    }

    #[test]
    fn test_gather_adds_resource_and_xp() {
        let mut cs = CivilSkills::new();
        assert_eq!(cs.resource_count(ResourceKind::BoisBrut), 0);
        let result = cs.gather(ResourceKind::BoisBrut);
        assert!(result.is_some());
        let (qty, _) = result.unwrap();
        assert!(qty >= 1);
        assert!(cs.resource_count(ResourceKind::BoisBrut) >= 1);
        assert!(cs.skill(CivilSkillId::Bucheronage).xp > 0.0);
    }

    #[test]
    fn test_craft_consumes_resources() {
        let mut cs = CivilSkills::new();
        // Give enough resources for Potion de soin (2 HerbeCommune, Alchimie lv1)
        cs.add_resource(ResourceKind::HerbeCommune, 5);
        let recipes = all_recipes();
        let potion_recipe = recipes.iter().find(|r| r.product == CraftCategory::PotionSoin).unwrap();
        assert!(cs.can_craft(potion_recipe));
        let levels = cs.craft(potion_recipe);
        assert!(levels.is_some());
        assert_eq!(cs.resource_count(ResourceKind::HerbeCommune), 3); // 5 - 2
    }

    #[test]
    fn test_craft_requires_level() {
        let cs = CivilSkills::new();
        let recipes = all_recipes();
        // Epee d'acier requires Forge lv25
        let steel_sword = recipes.iter().find(|r| r.product == CraftCategory::EpeeAcier).unwrap();
        assert!(!cs.can_craft(steel_sword));
    }

    #[test]
    fn test_commerce_bonuses() {
        let mut cs = CivilSkills::new();
        assert!((cs.sell_price_multiplier() - 1.005).abs() < 0.001); // lv1 → 1 + 1*0.005
        assert!((cs.buy_price_multiplier() - 0.997).abs() < 0.001); // lv1 → 1 - 1*0.003
        // Level up commerce
        cs.skill_mut(CivilSkillId::Commerce).level = 50;
        assert!((cs.sell_price_multiplier() - 1.25).abs() < 0.001); // lv50 → +25%
    }

    #[test]
    fn test_all_recipes_valid() {
        let recipes = all_recipes();
        assert!(recipes.len() == CRAFT_CATEGORY_COUNT);
        for r in recipes {
            assert!(!r.ingredients.is_empty());
            assert!(r.min_level >= 1);
            assert!(r.min_level <= CIVIL_MAX_LEVEL);
            assert!(r.craft_xp > 0.0);
        }
    }

    #[test]
    fn test_resource_gathering_skill_mapping() {
        // Chaque ressource a une compétence de récolte valide
        for r in [
            ResourceKind::BoisBrut, ResourceKind::MineraiFer,
            ResourceKind::HerbeCommune, ResourceKind::Ble, ResourceKind::Cuir,
        ] {
            let _ = r.gathering_skill(); // ne doit pas paniquer
        }
    }

    #[test]
    fn test_recompute_bonuses() {
        let mut cs = CivilSkills::new();
        cs.skills[CivilSkillId::Forge as usize].level = 50;
        cs.skills[CivilSkillId::Domptage as usize].level = 50;
        cs.recompute_bonuses();
        assert!((cs.bonuses.defense_flat - 15.0).abs() < 0.01); // 50 * 0.3
        assert_eq!(cs.bonuses.max_companions, 3); // 1 + 50/25
    }
}
