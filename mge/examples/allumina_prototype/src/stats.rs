//! @id allumina.prototype.stats
//! @role data
//! @layer application
//! @domain allumina
//! @do character_stats_allumina_spec
//!
//! Système de personnage Allumina — conforme à la spec :
//! "Allumina — Caractéristiques, Aptitudes de combat et Compétences"
//!
//! Caractéristiques (carac 1-10) → statistiques dérivées → aptitudes de combat.
//! Formules normatives (section 3 et 4.2 du doc spec) :
//!   PV max    = (For + Con) × 10
//!   PM max    = (Int + Sag) × 10
//!   End max   = (For + Con×2) × 10
//!   atk       = Dex × 10
//!   atk speed = Agi × 10
//!   jet       = For × 10
//!   esq       = Agi × 10
//!   par       = (For + Con) / 2 × 10
//!   tirC      = (For/2 + Per) × 10
//!   tirP      = (Agi + Per) × 10
//!   tirE      = (For + Per) × 10
//!   magie     = (Int + Sag) / 2
//!   cast speed= (Int + Sag) / 2
//!
//! Compétences (section 10-11) :
//!   Plafond = Carac × 10 + 20
//!   Minimum = Carac / 2 × 10 (= Carac × 5)
//!   30 compétences rattachées aux 9 caractéristiques (Luk n'a pas de compétence)

use mge_ecs::Component;

// ── Caractéristiques (10 caracs, range 1-10) ──────────────────────────────────

/// Les 10 caractéristiques normatives d'Allumina.
/// Codes doc : For, Con, Agi, Dex, Per, Vol, Int, Sag, Cha, Luk.
#[derive(Debug, Clone, Copy)]
pub struct CharacterStats {
    /// Force — dégâts CàC, poids max, endurance
    pub for_: u8,
    /// Constitution — PV, endurance, aggro
    pub con: u8,
    /// Agilité — vitesse attaque, esquive, déplacement
    pub agi: u8,
    /// Dextérité — précision (toucher mêlée), gestes fins
    pub dex: u8,
    /// Perception — détection, portée, visée tirs
    pub per: u8,
    /// Volonté — réserve mana, résistance mentale
    pub vol: u8,
    /// Intelligence — magie, cast speed, énigmes
    pub int: u8,
    /// Sagesse — sorts, soins, craft
    pub sag: u8,
    /// Charisme — commandement troupes, commerce, social
    pub cha: u8,
    /// Chance — seuil critique, modificateur dé global
    pub luk: u8,
}
impl Component for CharacterStats {}

impl CharacterStats {
    /// Joueur débutant — toutes caracs à 3 (archétype neutre).
    pub fn player() -> Self {
        Self { for_: 3, con: 3, agi: 3, dex: 3, per: 3, vol: 3, int: 3, sag: 3, cha: 3, luk: 3 }
    }

    /// Mob selon archétype (kind 0-4).
    pub fn mob(kind: u8) -> Self {
        match kind % 5 {
            0 => Self { for_: 2, con: 2, agi: 2, dex: 2, per: 2, vol: 1, int: 1, sag: 1, cha: 1, luk: 1 },
            1 => Self { for_: 5, con: 5, agi: 3, dex: 3, per: 3, vol: 3, int: 2, sag: 2, cha: 2, luk: 2 },
            2 => Self { for_: 4, con: 4, agi: 3, dex: 3, per: 3, vol: 2, int: 2, sag: 2, cha: 2, luk: 2 },
            3 => Self { for_: 2, con: 2, agi: 4, dex: 4, per: 4, vol: 2, int: 2, sag: 2, cha: 1, luk: 2 },
            _ => Self { for_: 4, con: 3, agi: 3, dex: 3, per: 2, vol: 2, int: 2, sag: 2, cha: 2, luk: 2 },
        }
    }

    // ── Statistiques dérivées (spec section 3) ────────────────────────────────

    /// PV max = (For + Con) × 10
    pub fn pv_max(&self) -> i32 { (self.for_ as i32 + self.con as i32) * 10 }

    /// PM max = (Int + Sag) × 10
    pub fn pm_max(&self) -> i32 { (self.int as i32 + self.sag as i32) * 10 }

    /// End max = (For + Con×2) × 10
    pub fn end_max(&self) -> i32 { (self.for_ as i32 + self.con as i32 * 2) * 10 }

    /// Aggro = Con + For (priorité de ciblage ennemi)
    pub fn aggro(&self) -> i32 { self.con as i32 + self.for_ as i32 }

    /// Poids max = (For + Con) × 5 (porter sans malus)  (spec section 8 : (For + Con) × 5)
    pub fn pds_max(&self) -> i32 { (self.for_ as i32 + self.con as i32) * 5 }

    // ── Aptitudes de combat à la création (spec section 4.2) ─────────────────

    /// atk = Dex × 10  (toucher corps à corps)
    pub fn atk_apt(&self) -> i32 { self.dex as i32 * 10 }

    /// atk speed = Agi × 10  (vitesse d'attaque)
    pub fn atk_speed_apt(&self) -> i32 { self.agi as i32 * 10 }

    /// jet = For × 10  (lancer d'objets)
    pub fn jet_apt(&self) -> i32 { self.for_ as i32 * 10 }

    /// esq = Agi × 10  (esquive)
    pub fn esq_apt(&self) -> i32 { self.agi as i32 * 10 }

    /// par = (For + Con) / 2 × 10 = (For + Con) × 5  (parade)
    pub fn par_apt(&self) -> i32 { (self.for_ as i32 + self.con as i32) * 5 }

    /// tirC = (For/2 + Per) × 10  (tir corde : arc, fronde)
    pub fn tir_corde_apt(&self) -> i32 { (self.for_ as i32 / 2 + self.per as i32) * 10 }

    /// tirP = (Agi + Per) × 10  (tir poing : pistolet)
    pub fn tir_poing_apt(&self) -> i32 { (self.agi as i32 + self.per as i32) * 10 }

    /// tirE = (For + Per) × 10  (tir épaule : arbalète, fusil)
    pub fn tir_epaule_apt(&self) -> i32 { (self.for_ as i32 + self.per as i32) * 10 }

    /// magie = (Int + Sag) / 2  (aptitude magique)
    pub fn magie_apt(&self) -> i32 { (self.int as i32 + self.sag as i32) / 2 }

    /// cast speed = (Int + Sag) / 2  (réduction temps d'incantation en %)
    pub fn cast_speed_apt(&self) -> i32 { (self.int as i32 + self.sag as i32) / 2 }

    /// Dégâts CàC de base : For × 3
    pub fn melee_damage(&self) -> i32 { (self.for_ as i32 * 3).max(1) }
}

// ── Compétences (spec sections 10-11) ────────────────────────────────────────

/// Les 30 compétences d'Allumina, organisées par caractéristique.
///
/// Plafond = Carac × 10 + 20 (spec section 10)
/// Minimum = Carac × 5       (spec section 10 : Carac / 2 × 10)
///
/// La valeur stockée est la valeur courante de la compétence (1-120).
/// La Chance (Luk) n'a pas de compétence dédiée (section 11.10).
#[derive(Debug, Clone, Copy)]
pub struct Competences {
    // ── Force (section 11.1) ────────────────────────────────────────────────
    /// Saut — distance de saut, franchissement d'obstacles
    pub saut: i32,
    /// Natation — vitesse dans l'eau, réduction pénalité aquatique
    pub natation: i32,

    // ── Constitution (section 11.2) ─────────────────────────────────────────
    /// Athlétisme — vitesse de déplacement au sol, endurance
    pub athletisme: i32,

    // ── Agilité (section 11.3) ──────────────────────────────────────────────
    /// Acrobatie — passages étroits, équilibre, figures
    pub acrobatie: i32,
    /// Escalade — vitesse de grimpe, consommation endurance
    pub escalade: i32,
    /// Discrétion — furtivité, opposition vs Détection/Vigilance
    pub discretion: i32,
    /// Larcin — pickpocket
    pub larcin: i32,
    /// Danse — attirer l'attention, bonus affinité PNJ
    pub danse: i32,
    /// Equitation — prérequis montures, contrôle en selle
    pub equitation: i32,

    // ── Dextérité (section 11.4) ────────────────────────────────────────────
    /// Crochetage — ouverture des serrures
    pub crochetage: i32,
    /// Pièges — détecter, installer, désarmer
    pub pieges: i32,

    // ── Perception (section 11.5) ───────────────────────────────────────────
    /// Détection — détecter ennemis camouflés (vs Discrétion)
    pub detection: i32,
    /// Pistage — informations sur le passage de PNJ/créatures
    pub pistage: i32,
    /// Fouille — trouver des objets dans conteneurs/zones
    pub fouille: i32,
    /// Musique — prérequis instruments, qualité de jeu (buff/social)
    pub musique: i32,
    /// Vigilance — détecter ennemis de plus loin
    pub vigilance: i32,

    // ── Intelligence (section 11.7) ─────────────────────────────────────────
    /// Orientation — mise à jour carte, réduction brouillard de guerre
    pub orientation: i32,
    /// Raisonnement — résoudre énigmes, indices
    pub raisonnement: i32,

    // ── Sagesse (section 11.8) ──────────────────────────────────────────────
    /// Médecine — soigner PNJ/joueurs, qualité soins
    pub medecine: i32,
    /// Mécanique — craft, catalogue recettes, taux réussite
    pub mecanique: i32,
    /// Herboristerie — préparation soins, remèdes, poisons
    pub herboristerie: i32,
    /// Apprentissage — prérequis lecture, gain compétences via livres
    pub apprentissage: i32,
    /// Science — machines déployables (tours, pièges auto)
    pub science: i32,

    // ── Charisme (section 11.9) ─────────────────────────────────────────────
    /// Bluff — plus de choix PNJ, tromperie
    pub bluff: i32,
    /// Rhétorique — meilleurs choix PNJ, arguments convaincants
    pub rhetorique: i32,
    /// Masque — réduit mauvais choix et leur impact
    pub masque: i32,
    /// Marchandage — influence prix vente/achat (1%/pt écart, max 50%)
    pub marchandage: i32,
    /// Commandement (Cmd) — pool pts troupes, prérequis types
    pub commandement: i32,
    /// Persuasion — chances de réponse favorable, choix de dialogue
    pub persuasion: i32,
    /// Dressage — dresser animaux (1 tentative/jour), cap = dizaine
    pub dressage: i32,
}
impl Component for Competences {}

impl Competences {
    /// Compétences initiales calculées depuis les caractéristiques.
    /// Chaque compétence démarre au minimum : Carac × 5 (spec section 10).
    pub fn from_char_stats(cs: &CharacterStats) -> Self {
        let f = cs.for_ as i32;
        let c = cs.con as i32;
        let a = cs.agi as i32;
        let d = cs.dex as i32;
        let p = cs.per as i32;
        let i = cs.int as i32;
        let s = cs.sag as i32;
        let ch = cs.cha as i32;

        Self {
            // Force
            saut:          f * 5,
            natation:      f * 5,
            // Constitution
            athletisme:    c * 5,
            // Agilité
            acrobatie:     a * 5,
            escalade:      a * 5,
            discretion:    a * 5,
            larcin:        a * 5,
            danse:         a * 5,
            equitation:    a * 5,
            // Dextérité
            crochetage:    d * 5,
            pieges:        d * 5,
            // Perception
            detection:     p * 5,
            pistage:       p * 5,
            fouille:       p * 5,
            musique:       p * 5,
            vigilance:     p * 5,
            // Intelligence
            orientation:   i * 5,
            raisonnement:  i * 5,
            // Sagesse
            medecine:      s * 5,
            mecanique:     s * 5,
            herboristerie: s * 5,
            apprentissage: s * 5,
            science:       s * 5,
            // Charisme
            bluff:         ch * 5,
            rhetorique:    ch * 5,
            masque:        ch * 5,
            marchandage:   ch * 5,
            commandement:  ch * 5,
            persuasion:    ch * 5,
            dressage:      ch * 5,
        }
    }

    /// Plafond d'une compétence = Carac × 10 + 20 (spec section 10).
    pub fn cap(carac: u8) -> i32 { carac as i32 * 10 + 20 }

    /// Minimum d'une compétence = Carac × 5 (spec section 10).
    pub fn minimum(carac: u8) -> i32 { carac as i32 * 5 }

    /// Retourne toutes les compétences avec leur nom, valeur, et caractéristique
    /// parente sous forme de tuples (catégorie, nom, valeur, carac_parente).
    /// Utilisé pour le rendu de la fenêtre compétences.
    pub fn as_list(&self, cs: &CharacterStats) -> Vec<(&'static str, &'static str, i32, u8)> {
        vec![
            // (catégorie, nom, valeur, carac parente)
            ("Force",        "Saut",          self.saut,          cs.for_),
            ("Force",        "Natation",      self.natation,      cs.for_),
            ("Constitution", "Athletisme",    self.athletisme,    cs.con),
            ("Agilite",      "Acrobatie",     self.acrobatie,     cs.agi),
            ("Agilite",      "Escalade",      self.escalade,      cs.agi),
            ("Agilite",      "Discretion",    self.discretion,    cs.agi),
            ("Agilite",      "Larcin",        self.larcin,        cs.agi),
            ("Agilite",      "Danse",         self.danse,         cs.agi),
            ("Agilite",      "Equitation",    self.equitation,    cs.agi),
            ("Dexterite",    "Crochetage",    self.crochetage,    cs.dex),
            ("Dexterite",    "Pieges",        self.pieges,        cs.dex),
            ("Perception",   "Detection",     self.detection,     cs.per),
            ("Perception",   "Pistage",       self.pistage,       cs.per),
            ("Perception",   "Fouille",       self.fouille,       cs.per),
            ("Perception",   "Musique",       self.musique,       cs.per),
            ("Perception",   "Vigilance",     self.vigilance,     cs.per),
            ("Intelligence", "Orientation",   self.orientation,   cs.int),
            ("Intelligence", "Raisonnement",  self.raisonnement,  cs.int),
            ("Sagesse",      "Medecine",      self.medecine,      cs.sag),
            ("Sagesse",      "Mecanique",     self.mecanique,     cs.sag),
            ("Sagesse",      "Herboristerie", self.herboristerie, cs.sag),
            ("Sagesse",      "Apprentissage", self.apprentissage, cs.sag),
            ("Sagesse",      "Science",       self.science,       cs.sag),
            ("Charisme",     "Bluff",         self.bluff,         cs.cha),
            ("Charisme",     "Rhetorique",    self.rhetorique,    cs.cha),
            ("Charisme",     "Masque",        self.masque,        cs.cha),
            ("Charisme",     "Marchandage",   self.marchandage,   cs.cha),
            ("Charisme",     "Commandement",  self.commandement,  cs.cha),
            ("Charisme",     "Persuasion",    self.persuasion,    cs.cha),
            ("Charisme",     "Dressage",      self.dressage,      cs.cha),
        ]
    }
}

// ── CombatStats — état dynamique (HP + Endurance courants) ───────────────────

/// État de combat courant : HP et Endurance actuels.
/// Les valeurs max sont calculées depuis CharacterStats.
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub hp: i32,
    pub hp_max: i32,
    /// Endurance courante
    pub end: i32,
    /// Endurance maximum
    pub end_max: i32,
    pub level: u8,
}
impl Component for CombatStats {}

impl CombatStats {
    /// Initialise depuis les caractéristiques (HP et End à plein).
    pub fn from_char_stats(cs: &CharacterStats) -> Self {
        let hp_max = cs.pv_max();
        let end_max = cs.end_max();
        Self { hp: hp_max, hp_max, end: end_max, end_max, level: 1 }
    }

    /// Ratio HP [0.0, 1.0]
    pub fn hp_ratio(&self) -> f32 {
        if self.hp_max <= 0 { return 0.0; }
        (self.hp as f32 / self.hp_max as f32).clamp(0.0, 1.0)
    }

    /// Ratio Endurance [0.0, 1.0]
    pub fn end_ratio(&self) -> f32 {
        if self.end_max <= 0 { return 0.0; }
        (self.end as f32 / self.end_max as f32).clamp(0.0, 1.0)
    }
}

// ── PlayerAttack — cooldown + portée + RNG de combat ─────────────────────────

/// Attaque du joueur : cooldown, portée, graine RNG pour les jets de dés.
pub struct PlayerAttack {
    /// Cooldown restant (secondes)
    pub cooldown: f32,
    /// Cooldown maximum entre deux frappes (secondes)
    pub cooldown_max: f32,
    /// Portée de frappe (tiles)
    pub range: f32,
    /// Graine RNG déterministe pour les jets de combat (LCG)
    pub rng_seed: u64,
}
impl Component for PlayerAttack {}

impl PlayerAttack {
    pub fn new() -> Self {
        Self { cooldown: 0.0, cooldown_max: 1.0, range: 1.5, rng_seed: 0xCAFE_BABE_0000_0001 }
    }
}

// ── PlayerGameStats — mana + expérience ──────────────────────────────────────

/// Stats de progression : mana, expérience et or.
pub struct PlayerGameStats {
    pub mana: i32,
    pub mana_max: i32,
    pub xp: u32,
    pub xp_to_next: u32,
    /// Or accumulé
    pub gold: u32,
}
impl Component for PlayerGameStats {}

impl PlayerGameStats {
    pub fn new() -> Self {
        // PM max = (Int+Sag)×10 = (3+3)×10 = 60 pour un joueur For=3 base
        Self { mana: 60, mana_max: 60, xp: 0, xp_to_next: 100, gold: 0 }
    }
}

// ── Dead / PendingDamage ──────────────────────────────────────────────────────

/// Tag : entité morte — sera despawn par le game loop principal.
pub struct Dead;
impl Component for Dead {}

/// Dommages en attente à appliquer sur cette entité (accumulés dans le tick).
pub struct PendingDamage {
    pub amount: i32,
}
impl Component for PendingDamage {}

// ── XpReward — récompense accordée au joueur à la mort de l'entité ────────────

/// XP et or obtenus par le joueur quand cette entité est tuée.
pub struct XpReward {
    pub xp: u32,
    pub gold: u32,
}
impl Component for XpReward {}
