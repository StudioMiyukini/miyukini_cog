//! @id     games_allumina_data_uniques
//! @do     f
//! @role   donnee
//! @layer  core
//! @human  games allumina — module « data uniques ».

//! Named Unique items for Allumina — 5 uniques per (item type × 20-level tier).
//!
//! Tiers (by `level_req`): 1-20, 21-40, 41-60, 61-80, 81-100.
//! Themes:
//! - T1 (1-20)   : nature / wolves / village / apprentice
//! - T2 (21-40)  : crypts / undead / wraiths / bone
//! - T3 (41-60)  : war / fortress / knights / steel
//! - T4 (61-80)  : elemental / dragons / flame / frost / storm
//! - T5 (81-100) : divine / demonic / eternal / chaos
//!
//! IDs are blocked by item kind (sword=10_000.., axe=10_100.., …) so every
//! unique is stable across runs and serializable to saves by id.

use mge_gameplay::stats::{Modifier, StatKind};
use mge_items::item::ItemKind;
use mge_items::uniques::{FixedMod, UniqueDef};

use ItemKind as K;
use StatKind as S;

/// Flat mod helper.
fn f(stat: StatKind, v: i32) -> FixedMod {
    FixedMod { stat, modifier: Modifier::Flat(v) }
}

/// Percent mod helper.
fn p(stat: StatKind, v: i32) -> FixedMod {
    FixedMod { stat, modifier: Modifier::Percentage(v) }
}

/// Compact builder used by every entry below.
fn mk(id: u32, name: &str, kind: ItemKind, lvl: u32, mods: Vec<FixedMod>, flavour: &str) -> UniqueDef {
    UniqueDef {
        id,
        name: name.into(),
        base_kind: kind,
        min_ilvl: lvl,
        mods,
        flavour: flavour.into(),
    }
}

/// The full named-unique table for Allumina.
#[allow(clippy::too_many_lines)]
pub fn uniques_table() -> Vec<UniqueDef> {
    let mut v = Vec::with_capacity(425);

    // ── SWORDS (10_000..) ──────────────────────────────────────────────────
    // T1 (1-20): Novice / Wolf / Guardian / Bloody / Dawn
    v.push(mk(10_000, "Lame du Novice", K::Sword, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 5), f(S::Strength, 3)], "Premiere arme du heros."));
    v.push(mk(10_001, "Croc de Loup", K::Sword, 10, vec![f(S::DamageMin, 4), f(S::DamageMax, 8), p(S::AttackSpeed, 5), f(S::Life, 10)], "Forgee avec un croc de loup alpha."));
    v.push(mk(10_002, "Epee du Gardien", K::Sword, 15, vec![f(S::DamageMin, 6), f(S::DamageMax, 12), f(S::Defense, 15), f(S::Vitality, 5)], "Portee par les gardiens du village."));
    v.push(mk(10_003, "Lame Sanglante", K::Sword, 18, vec![f(S::DamageMin, 8), f(S::DamageMax, 15), p(S::CritChance, 5), f(S::LifeOnHit, 3)], "Chaque coup l'abreuve."));
    v.push(mk(10_004, "Fendoir d'Aube", K::Sword, 20, vec![f(S::DamageMin, 10), f(S::DamageMax, 18), f(S::Strength, 10), p(S::AttackSpeed, 10)], "Tranche meme la nuit."));
    // T2 (21-40): Crypt / Spectre / Sepulchre / Ash / Soul
    v.push(mk(10_005, "Lame des Cryptes", K::Sword, 25, vec![f(S::DamageMin, 14), f(S::DamageMax, 24), f(S::Strength, 12), f(S::ColdResist, 15)], "Tiree d'un tombeau scelle."));
    v.push(mk(10_006, "Epee Spectrale", K::Sword, 30, vec![f(S::DamageMin, 17), f(S::DamageMax, 30), p(S::AttackSpeed, 12), f(S::LifeOnHit, 6)], "Le metal hurle entre les coups."));
    v.push(mk(10_007, "Lame Sepulcrale", K::Sword, 35, vec![f(S::DamageMin, 21), f(S::DamageMax, 36), f(S::Strength, 18), p(S::CritChance, 8)], "Forgee sur un autel funeraire."));
    v.push(mk(10_008, "Tranche-Cendres", K::Sword, 38, vec![f(S::DamageMin, 25), f(S::DamageMax, 42), p(S::AttackSpeed, 15), f(S::FireResist, 20)], "La cendre s'y colle au fer."));
    v.push(mk(10_009, "Tueuse d'Ames", K::Sword, 40, vec![f(S::DamageMin, 28), f(S::DamageMax, 48), f(S::Strength, 22), f(S::LifeOnHit, 10), p(S::CritChance, 10)], "Aucune ame ne lui echappe."));
    // T3 (41-60): War / Crusader / Siege / Iron / Champion
    v.push(mk(10_010, "Acier du Roi", K::Sword, 45, vec![f(S::DamageMin, 32), f(S::DamageMax, 56), f(S::Strength, 25), f(S::Defense, 40)], "Servait la couronne dechue."));
    v.push(mk(10_011, "Lame Croisee", K::Sword, 50, vec![f(S::DamageMin, 38), f(S::DamageMax, 64), p(S::AttackSpeed, 18), p(S::CritChance, 12)], "Benie a la veille du siege."));
    v.push(mk(10_012, "Fendoir de Siege", K::Sword, 55, vec![f(S::DamageMin, 44), f(S::DamageMax, 72), f(S::Strength, 32), f(S::Vitality, 18)], "Brise plus de portes que de cous."));
    v.push(mk(10_013, "Garde-de-Fer", K::Sword, 58, vec![f(S::DamageMin, 50), f(S::DamageMax, 80), f(S::Defense, 60), f(S::BlockChance, 8)], "Le bouclier qui coupe."));
    v.push(mk(10_014, "Lame du Champion", K::Sword, 60, vec![f(S::DamageMin, 56), f(S::DamageMax, 90), f(S::Strength, 40), p(S::AttackSpeed, 22), f(S::LifeOnHit, 15)], "Trophee de mille duels."));
    // T4 (61-80): Flame / Frost / Storm / Drake / Wyrm
    v.push(mk(10_015, "Lame de Flamme", K::Sword, 65, vec![f(S::DamageMin, 65), f(S::DamageMax, 105), f(S::FireResist, 30), p(S::AttackSpeed, 18)], "Brule encore apres l'eclipse."));
    v.push(mk(10_016, "Givre-Croc", K::Sword, 70, vec![f(S::DamageMin, 72), f(S::DamageMax, 118), f(S::ColdResist, 35), f(S::Strength, 30)], "Le froid l'a fendu et reforgee."));
    v.push(mk(10_017, "Foudre Tranchante", K::Sword, 75, vec![f(S::DamageMin, 80), f(S::DamageMax, 130), f(S::LightningResist, 35), p(S::AttackSpeed, 25)], "Frappe avant le tonnerre."));
    v.push(mk(10_018, "Lame Draconique", K::Sword, 78, vec![f(S::DamageMin, 88), f(S::DamageMax, 145), f(S::Strength, 45), p(S::CritChance, 15)], "Une ecaille a servi de garde."));
    v.push(mk(10_019, "Ecaille du Wyrm", K::Sword, 80, vec![f(S::DamageMin, 95), f(S::DamageMax, 160), f(S::FireResist, 25), f(S::ColdResist, 25), p(S::AttackSpeed, 28)], "Le dragon dormait encore quand on l'a prise."));
    // T5 (81-100): Divine / Demon / Eternal / Apocalypse / Chaos
    v.push(mk(10_020, "Epee Divine", K::Sword, 85, vec![f(S::DamageMin, 110), f(S::DamageMax, 185), f(S::Strength, 55), f(S::LifeOnHit, 25), p(S::CritChance, 18)], "Tombee du ciel pour un seul porteur."));
    v.push(mk(10_021, "Tranche-Demon", K::Sword, 90, vec![f(S::DamageMin, 125), f(S::DamageMax, 200), f(S::FireResist, 40), p(S::AttackSpeed, 30), f(S::LifeOnHit, 30)], "Connait le nom de chaque demon qu'elle a tue."));
    v.push(mk(10_022, "Lame de l'Eternel", K::Sword, 95, vec![f(S::DamageMin, 140), f(S::DamageMax, 220), f(S::Strength, 65), p(S::CritChance, 22), f(S::Life, 80)], "Elle se souvient des heros precedents."));
    v.push(mk(10_023, "Apocalypse", K::Sword, 98, vec![f(S::DamageMin, 155), f(S::DamageMax, 245), p(S::AttackSpeed, 35), p(S::CritChance, 25), f(S::LifeOnHit, 35)], "Annonce la fin a chaque ennemi."));
    v.push(mk(10_024, "Lame du Chaos", K::Sword, 100, vec![f(S::DamageMin, 170), f(S::DamageMax, 280), f(S::Strength, 80), p(S::AttackSpeed, 40), p(S::CritChance, 28), f(S::LifeOnHit, 40)], "Aucune main mortelle n'aurait du la lever."));

    // ── AXES (10_100..) ────────────────────────────────────────────────────
    v.push(mk(10_100, "Hache du Bucheron", K::Axe, 5, vec![f(S::DamageMin, 3), f(S::DamageMax, 7), f(S::Strength, 4)], "Toujours du sang sous le manche."));
    v.push(mk(10_101, "Tribu-Loup", K::Axe, 10, vec![f(S::DamageMin, 5), f(S::DamageMax, 11), f(S::Strength, 6), f(S::Vitality, 4)], "Marquee par dix clans en exil."));
    v.push(mk(10_102, "Sauvage", K::Axe, 15, vec![f(S::DamageMin, 8), f(S::DamageMax, 15), p(S::CritChance, 6), f(S::LifeOnHit, 4)], "Plus elle frappe, plus elle a faim."));
    v.push(mk(10_103, "Boucher", K::Axe, 18, vec![f(S::DamageMin, 10), f(S::DamageMax, 19), f(S::Strength, 12), p(S::AttackSpeed, 8)], "Pour les bouchers du village frontiere."));
    v.push(mk(10_104, "Griffe d'Ourse", K::Axe, 20, vec![f(S::DamageMin, 13), f(S::DamageMax, 22), f(S::Strength, 14), p(S::CritChance, 8)], "Forgee dans une caverne sanglante."));
    v.push(mk(10_105, "Bourreau", K::Axe, 25, vec![f(S::DamageMin, 16), f(S::DamageMax, 28), f(S::Strength, 18), f(S::LifeOnHit, 8)], "Coupe net, sans question."));
    v.push(mk(10_106, "Hache d'Ossuaire", K::Axe, 30, vec![f(S::DamageMin, 20), f(S::DamageMax, 34), p(S::CritChance, 12), f(S::ColdResist, 18)], "Affutee sur des cranes."));
    v.push(mk(10_107, "Cri-Funeste", K::Axe, 35, vec![f(S::DamageMin, 24), f(S::DamageMax, 40), f(S::Strength, 22), p(S::AttackSpeed, 12)], "Hurle quand on la sort de son etui."));
    v.push(mk(10_108, "Ravage-Tombe", K::Axe, 38, vec![f(S::DamageMin, 28), f(S::DamageMax, 46), f(S::Strength, 26), f(S::LifeOnHit, 12)], "Profanait les tombeaux royaux."));
    v.push(mk(10_109, "Hache de la Goule", K::Axe, 40, vec![f(S::DamageMin, 32), f(S::DamageMax, 54), p(S::CritChance, 15), f(S::PoisonResist, 25), f(S::LifeOnHit, 14)], "La goule lui parle encore."));
    v.push(mk(10_110, "Hache du Soldat", K::Axe, 45, vec![f(S::DamageMin, 36), f(S::DamageMax, 62), f(S::Strength, 28), f(S::Defense, 30)], "Regimentaire, sans poesie."));
    v.push(mk(10_111, "Berserker", K::Axe, 50, vec![f(S::DamageMin, 42), f(S::DamageMax, 70), p(S::AttackSpeed, 20), p(S::CritChance, 14)], "Plus on saigne, plus elle est rapide."));
    v.push(mk(10_112, "Brise-Bouclier", K::Axe, 55, vec![f(S::DamageMin, 50), f(S::DamageMax, 80), f(S::Strength, 35), f(S::BlockChance, -5)], "Ouvre les portes a l'epaule."));
    v.push(mk(10_113, "Hache de Bataille", K::Axe, 58, vec![f(S::DamageMin, 56), f(S::DamageMax, 88), f(S::Strength, 40), f(S::Vitality, 20)], "Trois rois l'ont portee."));
    v.push(mk(10_114, "Hache de Forge", K::Axe, 60, vec![f(S::DamageMin, 62), f(S::DamageMax, 98), f(S::FireResist, 25), f(S::Strength, 42), p(S::CritChance, 16)], "Toujours chaude au toucher."));
    v.push(mk(10_115, "Foudre-Lame", K::Axe, 65, vec![f(S::DamageMin, 70), f(S::DamageMax, 115), f(S::LightningResist, 30), p(S::AttackSpeed, 20)], "Frappe deux fois pour chaque coup."));
    v.push(mk(10_116, "Briseur de Glace", K::Axe, 70, vec![f(S::DamageMin, 78), f(S::DamageMax, 128), f(S::ColdResist, 35), f(S::Strength, 38)], "Sa lame ne degele jamais."));
    v.push(mk(10_117, "Bouche-Volcan", K::Axe, 75, vec![f(S::DamageMin, 88), f(S::DamageMax, 142), f(S::FireResist, 40), p(S::CritChance, 18)], "Forgee dans la gueule du dragon."));
    v.push(mk(10_118, "Drakor", K::Axe, 78, vec![f(S::DamageMin, 96), f(S::DamageMax, 155), f(S::Strength, 50), f(S::LifeOnHit, 22)], "On l'a payee en ecailles."));
    v.push(mk(10_119, "Souffle-Wyrm", K::Axe, 80, vec![f(S::DamageMin, 105), f(S::DamageMax, 168), f(S::FireResist, 25), f(S::ColdResist, 25), p(S::AttackSpeed, 25)], "Le wyrm a expire dans le metal."));
    v.push(mk(10_120, "Hache du Seraphin", K::Axe, 85, vec![f(S::DamageMin, 120), f(S::DamageMax, 195), f(S::Strength, 58), f(S::LifeOnHit, 28), p(S::CritChance, 18)], "Aile blanche, lame noire."));
    v.push(mk(10_121, "Furie-Demon", K::Axe, 90, vec![f(S::DamageMin, 138), f(S::DamageMax, 215), f(S::FireResist, 40), p(S::AttackSpeed, 30), p(S::CritChance, 22)], "La rage du puits."));
    v.push(mk(10_122, "Cataclysme", K::Axe, 95, vec![f(S::DamageMin, 155), f(S::DamageMax, 240), f(S::Strength, 70), f(S::Life, 70), p(S::CritChance, 24)], "Tousse les batiments comme des dents."));
    v.push(mk(10_123, "Ragnarok", K::Axe, 98, vec![f(S::DamageMin, 175), f(S::DamageMax, 265), p(S::AttackSpeed, 35), f(S::LifeOnHit, 35), p(S::CritChance, 26)], "Annonce la fin du monde."));
    v.push(mk(10_124, "Fendoir du Neant", K::Axe, 100, vec![f(S::DamageMin, 195), f(S::DamageMax, 305), f(S::Strength, 85), p(S::CritChance, 30), p(S::AttackSpeed, 40), f(S::LifeOnHit, 40)], "Coupe meme l'absence."));

    // ── MACES (10_200..) ───────────────────────────────────────────────────
    v.push(mk(10_200, "Masse de l'Apprenti", K::Mace, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 6), f(S::Strength, 3), f(S::Defense, 5)], "Lourde pour un bras maigre."));
    v.push(mk(10_201, "Marteau de Pierre", K::Mace, 10, vec![f(S::DamageMin, 4), f(S::DamageMax, 9), f(S::Strength, 6), f(S::Life, 15)], "Faite d'une pierre du tumulus."));
    v.push(mk(10_202, "Masse du Garde", K::Mace, 15, vec![f(S::DamageMin, 6), f(S::DamageMax, 13), f(S::Defense, 20), f(S::Vitality, 6)], "Pour mater les tavernes."));
    v.push(mk(10_203, "Marteau-Loup", K::Mace, 18, vec![f(S::DamageMin, 8), f(S::DamageMax, 17), f(S::Strength, 10), p(S::AttackSpeed, 5)], "Brise les os d'un seul coup."));
    v.push(mk(10_204, "Forge-Ame", K::Mace, 20, vec![f(S::DamageMin, 11), f(S::DamageMax, 20), f(S::Strength, 12), f(S::Life, 30)], "Battue par un saint forgeron."));
    v.push(mk(10_205, "Masse Sepulcrale", K::Mace, 25, vec![f(S::DamageMin, 14), f(S::DamageMax, 26), f(S::Strength, 16), f(S::PoisonResist, 18)], "Frappee sur la cloche du tombeau."));
    v.push(mk(10_206, "Marteau-Ossuaire", K::Mace, 30, vec![f(S::DamageMin, 18), f(S::DamageMax, 32), f(S::Strength, 20), f(S::Defense, 35)], "Os de geant, poignee de cuir noir."));
    v.push(mk(10_207, "Marteau-Necrose", K::Mace, 35, vec![f(S::DamageMin, 22), f(S::DamageMax, 38), f(S::LifeOnHit, 10), f(S::PoisonResist, 25)], "La rouille s'y nourrit toute seule."));
    v.push(mk(10_208, "Esprit-Brise", K::Mace, 38, vec![f(S::DamageMin, 26), f(S::DamageMax, 44), f(S::Strength, 24), f(S::Mana, 30)], "Brisait les murmures."));
    v.push(mk(10_209, "Marteau du Tombeau", K::Mace, 40, vec![f(S::DamageMin, 30), f(S::DamageMax, 50), f(S::Strength, 28), f(S::Defense, 45), f(S::LifeOnHit, 12)], "Scelle les portes pour toujours."));
    v.push(mk(10_210, "Acier-Lourd", K::Mace, 45, vec![f(S::DamageMin, 34), f(S::DamageMax, 58), f(S::Strength, 32), f(S::Defense, 50)], "Un quintal de jugement."));
    v.push(mk(10_211, "Briseur", K::Mace, 50, vec![f(S::DamageMin, 40), f(S::DamageMax, 66), f(S::Strength, 38), p(S::AttackSpeed, 15)], "Brise les armures comme du bois."));
    v.push(mk(10_212, "Masse de Croisade", K::Mace, 55, vec![f(S::DamageMin, 46), f(S::DamageMax, 76), f(S::FireResist, 20), f(S::Strength, 40)], "Benie par un eveque mort en route."));
    v.push(mk(10_213, "Justicier", K::Mace, 58, vec![f(S::DamageMin, 52), f(S::DamageMax, 84), f(S::Defense, 60), f(S::Vitality, 25)], "Aucun jugement n'a survecu a sa tete."));
    v.push(mk(10_214, "Forteresse", K::Mace, 60, vec![f(S::DamageMin, 58), f(S::DamageMax, 92), f(S::Strength, 45), f(S::Defense, 70), f(S::Life, 60)], "Un mur tenu dans une main."));
    v.push(mk(10_215, "Pyrocaust", K::Mace, 65, vec![f(S::DamageMin, 66), f(S::DamageMax, 108), f(S::FireResist, 35), p(S::CritChance, 14)], "Fond le metal sur lequel elle s'abat."));
    v.push(mk(10_216, "Givre-Pic", K::Mace, 70, vec![f(S::DamageMin, 74), f(S::DamageMax, 122), f(S::ColdResist, 35), f(S::Strength, 40)], "Plante un hiver dans la chair."));
    v.push(mk(10_217, "Foudroyante", K::Mace, 75, vec![f(S::DamageMin, 84), f(S::DamageMax, 134), f(S::LightningResist, 35), p(S::AttackSpeed, 20)], "Brille avant de tomber."));
    v.push(mk(10_218, "Drakebone", K::Mace, 78, vec![f(S::DamageMin, 92), f(S::DamageMax, 148), f(S::Strength, 50), f(S::Vitality, 30)], "Os de dragon, poigne de fer."));
    v.push(mk(10_219, "Coeur-Volcan", K::Mace, 80, vec![f(S::DamageMin, 100), f(S::DamageMax, 162), f(S::FireResist, 35), f(S::LifeOnHit, 22), p(S::CritChance, 16)], "Bat encore au creux du metal."));
    v.push(mk(10_220, "Masse d'Aurore", K::Mace, 85, vec![f(S::DamageMin, 115), f(S::DamageMax, 188), f(S::Strength, 60), f(S::Life, 80), p(S::CritChance, 18)], "Premiere lumiere apres la fin."));
    v.push(mk(10_221, "Cherubin", K::Mace, 90, vec![f(S::DamageMin, 132), f(S::DamageMax, 208), f(S::Defense, 80), f(S::LifeOnHit, 30), p(S::AttackSpeed, 25)], "L'ange a quatre faces l'a tenue."));
    v.push(mk(10_222, "Tueuse-Demon", K::Mace, 95, vec![f(S::DamageMin, 148), f(S::DamageMax, 230), f(S::FireResist, 40), f(S::Strength, 70), p(S::CritChance, 22)], "Connait chaque vrai nom."));
    v.push(mk(10_223, "Marteau Divin", K::Mace, 98, vec![f(S::DamageMin, 165), f(S::DamageMax, 255), f(S::Strength, 80), f(S::Life, 120), p(S::CritChance, 24)], "Forge des serments."));
    v.push(mk(10_224, "Fin-du-Monde", K::Mace, 100, vec![f(S::DamageMin, 185), f(S::DamageMax, 290), f(S::Strength, 90), f(S::Defense, 100), p(S::AttackSpeed, 30), f(S::LifeOnHit, 40)], "Au dernier coup, il n'y aura plus rien."));

    // ── DAGGERS (10_300..) ─────────────────────────────────────────────────
    v.push(mk(10_300, "Lame Sournoise", K::Dagger, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 4), f(S::Dexterity, 4), p(S::CritChance, 6)], "Glisse mieux qu'elle ne coupe."));
    v.push(mk(10_301, "Furtive", K::Dagger, 10, vec![f(S::DamageMin, 3), f(S::DamageMax, 7), f(S::Dexterity, 6), p(S::AttackSpeed, 8)], "On l'a deja perdue trois fois."));
    v.push(mk(10_302, "Croc-d'Ombre", K::Dagger, 15, vec![f(S::DamageMin, 5), f(S::DamageMax, 10), p(S::CritChance, 10), f(S::LifeOnHit, 4)], "Mord dans l'obscur."));
    v.push(mk(10_303, "Voleuse", K::Dagger, 18, vec![f(S::DamageMin, 7), f(S::DamageMax, 13), f(S::Dexterity, 10), f(S::GoldFind, 20)], "Trouve toujours une bourse."));
    v.push(mk(10_304, "Crochet d'Aube", K::Dagger, 20, vec![f(S::DamageMin, 9), f(S::DamageMax, 16), f(S::Dexterity, 12), p(S::CritChance, 12), p(S::AttackSpeed, 10)], "On la voit a peine au matin."));
    v.push(mk(10_305, "Echeveau", K::Dagger, 25, vec![f(S::DamageMin, 11), f(S::DamageMax, 20), f(S::Dexterity, 15), p(S::CritChance, 13)], "Demele les nerfs."));
    v.push(mk(10_306, "Ombre", K::Dagger, 30, vec![f(S::DamageMin, 14), f(S::DamageMax, 24), p(S::AttackSpeed, 14), f(S::LifeOnHit, 8)], "Une ombre y dort encore."));
    v.push(mk(10_307, "Saigne-Tombeau", K::Dagger, 35, vec![f(S::DamageMin, 17), f(S::DamageMax, 30), f(S::Dexterity, 20), p(S::CritChance, 16)], "Tirait son tribut aux morts."));
    v.push(mk(10_308, "Murmure", K::Dagger, 38, vec![f(S::DamageMin, 20), f(S::DamageMax, 34), p(S::AttackSpeed, 16), f(S::Mana, 25)], "On ne l'entend qu'apres."));
    v.push(mk(10_309, "Cri-Cendre", K::Dagger, 40, vec![f(S::DamageMin, 23), f(S::DamageMax, 38), f(S::FireResist, 20), p(S::CritChance, 18), p(S::AttackSpeed, 18)], "Crepite quand elle entre."));
    v.push(mk(10_310, "Lame-Cachee", K::Dagger, 45, vec![f(S::DamageMin, 27), f(S::DamageMax, 45), f(S::Dexterity, 28), p(S::CritChance, 20)], "Cachee dans une manche, jamais vue."));
    v.push(mk(10_311, "Stiletto", K::Dagger, 50, vec![f(S::DamageMin, 31), f(S::DamageMax, 52), p(S::AttackSpeed, 20), f(S::LifeOnHit, 12)], "Une aiguille pour les hommes en armure."));
    v.push(mk(10_312, "Assassin", K::Dagger, 55, vec![f(S::DamageMin, 36), f(S::DamageMax, 60), f(S::Dexterity, 32), p(S::CritChance, 22)], "Le contrat est paye d'avance."));
    v.push(mk(10_313, "Cellule", K::Dagger, 58, vec![f(S::DamageMin, 41), f(S::DamageMax, 68), p(S::AttackSpeed, 22), f(S::Dexterity, 35)], "Sortie d'une cellule sans serrure."));
    v.push(mk(10_314, "Lame de Ruelle", K::Dagger, 60, vec![f(S::DamageMin, 46), f(S::DamageMax, 76), f(S::Dexterity, 38), p(S::CritChance, 24), f(S::LifeOnHit, 14)], "Connait toutes les arteres."));
    v.push(mk(10_315, "Croc de Vipere", K::Dagger, 65, vec![f(S::DamageMin, 52), f(S::DamageMax, 88), f(S::PoisonResist, 30), p(S::AttackSpeed, 22)], "Le venin coule en continu."));
    v.push(mk(10_316, "Frisson", K::Dagger, 70, vec![f(S::DamageMin, 58), f(S::DamageMax, 98), f(S::ColdResist, 30), p(S::CritChance, 26)], "Le porteur ne grelotte plus."));
    v.push(mk(10_317, "Souffle-Sombre", K::Dagger, 75, vec![f(S::DamageMin, 65), f(S::DamageMax, 108), p(S::AttackSpeed, 26), f(S::LifeOnHit, 18)], "Boit l'air avec le sang."));
    v.push(mk(10_318, "Drakecut", K::Dagger, 78, vec![f(S::DamageMin, 72), f(S::DamageMax, 118), f(S::Dexterity, 42), p(S::CritChance, 28)], "L'ecaille la cache mieux qu'un fourreau."));
    v.push(mk(10_319, "Ecaillure", K::Dagger, 80, vec![f(S::DamageMin, 78), f(S::DamageMax, 128), f(S::FireResist, 22), f(S::ColdResist, 22), p(S::AttackSpeed, 28)], "Le wyrm la portait dans son ventre."));
    v.push(mk(10_320, "Eternelle", K::Dagger, 85, vec![f(S::DamageMin, 90), f(S::DamageMax, 148), f(S::Dexterity, 50), p(S::CritChance, 30), p(S::AttackSpeed, 28)], "Ne s'ebreche jamais."));
    v.push(mk(10_321, "Lacrymal", K::Dagger, 90, vec![f(S::DamageMin, 102), f(S::DamageMax, 162), f(S::LifeOnHit, 30), p(S::CritChance, 32)], "Pleure quand on la rengaine."));
    v.push(mk(10_322, "Coupe-Ame", K::Dagger, 95, vec![f(S::DamageMin, 115), f(S::DamageMax, 180), f(S::Dexterity, 60), p(S::AttackSpeed, 32), f(S::LifeOnHit, 35)], "L'ame sort par le sourire qu'elle ouvre."));
    v.push(mk(10_323, "Tueuse de Dieux", K::Dagger, 98, vec![f(S::DamageMin, 128), f(S::DamageMax, 200), p(S::CritChance, 35), p(S::AttackSpeed, 34), f(S::LifeOnHit, 38)], "Une seule fois. Elle a suffi."));
    v.push(mk(10_324, "Lame du Neant", K::Dagger, 100, vec![f(S::DamageMin, 145), f(S::DamageMax, 225), f(S::Dexterity, 70), p(S::CritChance, 38), p(S::AttackSpeed, 38), f(S::LifeOnHit, 40)], "Coupe la phrase avant qu'elle ne soit dite."));

    // ── BOWS (10_400..) ────────────────────────────────────────────────────
    v.push(mk(10_400, "Arc du Chasseur", K::Bow, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 6), f(S::Dexterity, 4)], "Tendu par un trappeur du nord."));
    v.push(mk(10_401, "Arc de Pin", K::Bow, 10, vec![f(S::DamageMin, 4), f(S::DamageMax, 10), f(S::Dexterity, 6), p(S::AttackSpeed, 6)], "Sent encore la resine."));
    v.push(mk(10_402, "Coeur-de-Foret", K::Bow, 15, vec![f(S::DamageMin, 6), f(S::DamageMax, 14), f(S::Dexterity, 8), p(S::CritChance, 8)], "Tire les arbres avec son tireur."));
    v.push(mk(10_403, "Bramement", K::Bow, 18, vec![f(S::DamageMin, 8), f(S::DamageMax, 18), f(S::Dexterity, 12), f(S::Vitality, 8)], "Resonne comme un cerf en rut."));
    v.push(mk(10_404, "Arc d'Aube", K::Bow, 20, vec![f(S::DamageMin, 11), f(S::DamageMax, 22), f(S::Dexterity, 14), p(S::AttackSpeed, 10), p(S::CritChance, 10)], "Tire la nuit hors du ciel."));
    v.push(mk(10_405, "Arc d'Os", K::Bow, 25, vec![f(S::DamageMin, 14), f(S::DamageMax, 28), f(S::Dexterity, 18), p(S::CritChance, 12)], "Une cote de geant pour bois."));
    v.push(mk(10_406, "Arc Spectral", K::Bow, 30, vec![f(S::DamageMin, 17), f(S::DamageMax, 34), p(S::AttackSpeed, 14), f(S::LifeOnHit, 8)], "Encoche meme sans corde."));
    v.push(mk(10_407, "Arc de Cypres", K::Bow, 35, vec![f(S::DamageMin, 21), f(S::DamageMax, 40), f(S::Dexterity, 22), f(S::PoisonResist, 20)], "Bois du cimetiere."));
    v.push(mk(10_408, "Linceul", K::Bow, 38, vec![f(S::DamageMin, 24), f(S::DamageMax, 46), p(S::CritChance, 15), f(S::ColdResist, 20)], "Une corde tissee d'un suaire."));
    v.push(mk(10_409, "Lune-Morte", K::Bow, 40, vec![f(S::DamageMin, 28), f(S::DamageMax, 52), f(S::Dexterity, 25), p(S::AttackSpeed, 16), p(S::CritChance, 16)], "Mire les ombres."));
    v.push(mk(10_410, "Garde-Ville", K::Bow, 45, vec![f(S::DamageMin, 32), f(S::DamageMax, 60), f(S::Dexterity, 28), f(S::Defense, 30)], "Aux mains des sentinelles d'enceinte."));
    v.push(mk(10_411, "Arc de Compagnie", K::Bow, 50, vec![f(S::DamageMin, 38), f(S::DamageMax, 68), p(S::AttackSpeed, 18), p(S::CritChance, 18)], "Memes encoches que vingt autres."));
    v.push(mk(10_412, "Veteran", K::Bow, 55, vec![f(S::DamageMin, 44), f(S::DamageMax, 76), f(S::Dexterity, 32), f(S::LifeOnHit, 12)], "Marque par chaque automne."));
    v.push(mk(10_413, "Arc du Mur", K::Bow, 58, vec![f(S::DamageMin, 50), f(S::DamageMax, 84), p(S::CritChance, 20), f(S::AttackRating, 40)], "Defendait la ligne du nord."));
    v.push(mk(10_414, "Sentinelle", K::Bow, 60, vec![f(S::DamageMin, 56), f(S::DamageMax, 92), f(S::Dexterity, 38), p(S::AttackSpeed, 22), p(S::CritChance, 22)], "Ne dort que d'un oeil."));
    v.push(mk(10_415, "Vent-Glace", K::Bow, 65, vec![f(S::DamageMin, 64), f(S::DamageMax, 105), f(S::ColdResist, 30), p(S::AttackSpeed, 22)], "Souffle a chaque tir."));
    v.push(mk(10_416, "Arc-Pyromane", K::Bow, 70, vec![f(S::DamageMin, 72), f(S::DamageMax, 118), f(S::FireResist, 30), p(S::CritChance, 22)], "Les fleches s'embrasent en vol."));
    v.push(mk(10_417, "Arc-Foudre", K::Bow, 75, vec![f(S::DamageMin, 80), f(S::DamageMax, 130), f(S::LightningResist, 30), p(S::AttackSpeed, 25)], "Le tonnerre suit les fleches."));
    v.push(mk(10_418, "Drake-Tendon", K::Bow, 78, vec![f(S::DamageMin, 88), f(S::DamageMax, 142), f(S::Dexterity, 45), p(S::CritChance, 24)], "Corde tendue d'un nerf de dragon."));
    v.push(mk(10_419, "Wyrmbow", K::Bow, 80, vec![f(S::DamageMin, 95), f(S::DamageMax, 155), f(S::FireResist, 25), f(S::ColdResist, 25), p(S::AttackSpeed, 27)], "Souffle de wyrm scelle dans le bois."));
    v.push(mk(10_420, "Etoile-Filante", K::Bow, 85, vec![f(S::DamageMin, 108), f(S::DamageMax, 180), f(S::Dexterity, 55), p(S::CritChance, 26), p(S::AttackSpeed, 27)], "Trace un sillon meme dans le neant."));
    v.push(mk(10_421, "Seraph-Tir", K::Bow, 90, vec![f(S::DamageMin, 122), f(S::DamageMax, 200), f(S::LifeOnHit, 25), p(S::CritChance, 28), p(S::AttackSpeed, 30)], "Six ailes pour decocher."));
    v.push(mk(10_422, "Demon-Trait", K::Bow, 95, vec![f(S::DamageMin, 138), f(S::DamageMax, 220), f(S::Dexterity, 65), f(S::FireResist, 30), p(S::CritChance, 32)], "Chaque fleche connait un nom."));
    v.push(mk(10_423, "Arc d'Eternite", K::Bow, 98, vec![f(S::DamageMin, 152), f(S::DamageMax, 240), p(S::AttackSpeed, 34), p(S::CritChance, 32), f(S::LifeOnHit, 32)], "Tend sans corde, frappe sans fin."));
    v.push(mk(10_424, "Arc du Chaos", K::Bow, 100, vec![f(S::DamageMin, 170), f(S::DamageMax, 270), f(S::Dexterity, 80), p(S::AttackSpeed, 38), p(S::CritChance, 35), f(S::LifeOnHit, 38)], "Le destin file dans sa corde."));

    // ── CROSSBOWS (10_500..) ───────────────────────────────────────────────
    v.push(mk(10_500, "Arbalete du Garde", K::Crossbow, 5, vec![f(S::DamageMin, 3), f(S::DamageMax, 8), f(S::Dexterity, 3), f(S::AttackRating, 10)], "Postee a chaque passe."));
    v.push(mk(10_501, "Ranger", K::Crossbow, 10, vec![f(S::DamageMin, 5), f(S::DamageMax, 12), f(S::Dexterity, 6), p(S::CritChance, 8)], "Pour traquer plus large."));
    v.push(mk(10_502, "Sentinelle", K::Crossbow, 15, vec![f(S::DamageMin, 8), f(S::DamageMax, 16), f(S::Dexterity, 8), f(S::AttackRating, 20)], "Ne dort pas avant l'aurore."));
    v.push(mk(10_503, "Embuscade", K::Crossbow, 18, vec![f(S::DamageMin, 10), f(S::DamageMax, 20), p(S::CritChance, 12), f(S::Dexterity, 10)], "Un seul carreau a la fois."));
    v.push(mk(10_504, "Tonneau d'Aube", K::Crossbow, 20, vec![f(S::DamageMin, 13), f(S::DamageMax, 24), f(S::Dexterity, 12), p(S::CritChance, 14), f(S::AttackRating, 25)], "Premier tir de la matinee."));
    v.push(mk(10_505, "Croc-Mort", K::Crossbow, 25, vec![f(S::DamageMin, 16), f(S::DamageMax, 30), p(S::CritChance, 14), f(S::PoisonResist, 18)], "Le carreau colle a la plaie."));
    v.push(mk(10_506, "Caveau", K::Crossbow, 30, vec![f(S::DamageMin, 20), f(S::DamageMax, 36), f(S::Dexterity, 18), f(S::AttackRating, 35)], "On y entend les fleches en cage."));
    v.push(mk(10_507, "Charnier", K::Crossbow, 35, vec![f(S::DamageMin, 24), f(S::DamageMax, 42), p(S::CritChance, 16), f(S::LifeOnHit, 8)], "Boyaux de cordage."));
    v.push(mk(10_508, "Cri-Tombeau", K::Crossbow, 38, vec![f(S::DamageMin, 28), f(S::DamageMax, 48), f(S::Dexterity, 22), f(S::AttackRating, 45)], "Chacun de ses tirs est un nom."));
    v.push(mk(10_509, "Veuve", K::Crossbow, 40, vec![f(S::DamageMin, 32), f(S::DamageMax, 54), p(S::CritChance, 18), f(S::PoisonResist, 25), f(S::Dexterity, 22)], "Mariee a un mort, fidele."));
    v.push(mk(10_510, "Siegebreaker", K::Crossbow, 45, vec![f(S::DamageMin, 36), f(S::DamageMax, 62), f(S::Dexterity, 26), f(S::AttackRating, 55)], "Brise les portes par les meurtrieres."));
    v.push(mk(10_511, "Carcase", K::Crossbow, 50, vec![f(S::DamageMin, 42), f(S::DamageMax, 70), p(S::CritChance, 20), f(S::Vitality, 18)], "Bois noir, fer noir."));
    v.push(mk(10_512, "Arsenal", K::Crossbow, 55, vec![f(S::DamageMin, 48), f(S::DamageMax, 78), f(S::AttackRating, 70), f(S::Dexterity, 30)], "Tient lieu de garnison."));
    v.push(mk(10_513, "Croisade", K::Crossbow, 58, vec![f(S::DamageMin, 54), f(S::DamageMax, 86), p(S::CritChance, 22), f(S::FireResist, 20)], "Beni par un chapelain au mur."));
    v.push(mk(10_514, "Bombard", K::Crossbow, 60, vec![f(S::DamageMin, 60), f(S::DamageMax, 96), f(S::Dexterity, 38), p(S::CritChance, 24), f(S::AttackRating, 80)], "Pas une arme, un argument."));
    v.push(mk(10_515, "Brise-Glace", K::Crossbow, 65, vec![f(S::DamageMin, 68), f(S::DamageMax, 110), f(S::ColdResist, 30), p(S::CritChance, 24)], "Les carreaux gelent en vol."));
    v.push(mk(10_516, "Lame-Feu", K::Crossbow, 70, vec![f(S::DamageMin, 76), f(S::DamageMax, 122), f(S::FireResist, 30), f(S::AttackRating, 90)], "Embrase ce qu'elle vise."));
    v.push(mk(10_517, "Boom-Foudre", K::Crossbow, 75, vec![f(S::DamageMin, 84), f(S::DamageMax, 135), f(S::LightningResist, 30), p(S::CritChance, 26)], "Eclate l'air."));
    v.push(mk(10_518, "Drakefire", K::Crossbow, 78, vec![f(S::DamageMin, 92), f(S::DamageMax, 148), f(S::Dexterity, 45), f(S::FireResist, 25)], "Forgee avec un crachat de dragon."));
    v.push(mk(10_519, "Wyrmbolt", K::Crossbow, 80, vec![f(S::DamageMin, 100), f(S::DamageMax, 160), f(S::Dexterity, 48), p(S::CritChance, 28), f(S::AttackRating, 100)], "Le wyrm en garde un dans la gueule."));
    v.push(mk(10_520, "Justicier", K::Crossbow, 85, vec![f(S::DamageMin, 115), f(S::DamageMax, 185), f(S::Dexterity, 55), p(S::CritChance, 30), f(S::AttackRating, 120)], "Le verdict est en route."));
    v.push(mk(10_521, "Seraph-Boulon", K::Crossbow, 90, vec![f(S::DamageMin, 130), f(S::DamageMax, 205), f(S::LifeOnHit, 25), p(S::CritChance, 32)], "Six ailes pour bander la corde."));
    v.push(mk(10_522, "Demon-Tir", K::Crossbow, 95, vec![f(S::DamageMin, 148), f(S::DamageMax, 230), f(S::FireResist, 35), f(S::Dexterity, 65), p(S::CritChance, 34)], "Vide les enfers carreau par carreau."));
    v.push(mk(10_523, "Apocalypse-Tir", K::Crossbow, 98, vec![f(S::DamageMin, 165), f(S::DamageMax, 250), p(S::CritChance, 36), f(S::AttackRating, 150), f(S::LifeOnHit, 30)], "La fin tire d'un mecanisme."));
    v.push(mk(10_524, "Chaos-Carreau", K::Crossbow, 100, vec![f(S::DamageMin, 185), f(S::DamageMax, 285), f(S::Dexterity, 78), p(S::CritChance, 38), f(S::AttackRating, 170), f(S::LifeOnHit, 35)], "Le carreau choisit sa cible."));

    // ── STAVES (10_600..) ──────────────────────────────────────────────────
    v.push(mk(10_600, "Baton d'Apprenti", K::Staff, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 5), f(S::Energy, 5), f(S::Mana, 15)], "Trop long, trop droit."));
    v.push(mk(10_601, "Chene", K::Staff, 10, vec![f(S::DamageMin, 4), f(S::DamageMax, 8), f(S::Energy, 8), p(S::CastSpeed, 6)], "Coupee a la lune ascendante."));
    v.push(mk(10_602, "Glade", K::Staff, 15, vec![f(S::DamageMin, 6), f(S::DamageMax, 12), f(S::Energy, 10), f(S::Mana, 30), p(S::CastSpeed, 8)], "Encore couverte de mousse."));
    v.push(mk(10_603, "Druide", K::Staff, 18, vec![f(S::DamageMin, 8), f(S::DamageMax, 15), f(S::Vitality, 8), f(S::Energy, 12), p(S::CastSpeed, 10)], "Murmure de bouche en bouche."));
    v.push(mk(10_604, "Sagesse d'Aube", K::Staff, 20, vec![f(S::DamageMin, 10), f(S::DamageMax, 18), f(S::Energy, 14), f(S::Mana, 40), p(S::CastSpeed, 12)], "Premiere clarte de l'esprit."));
    v.push(mk(10_605, "Os-Magus", K::Staff, 25, vec![f(S::DamageMin, 13), f(S::DamageMax, 22), f(S::Energy, 16), p(S::CastSpeed, 14)], "Os de necromant grave."));
    v.push(mk(10_606, "Esprit", K::Staff, 30, vec![f(S::DamageMin, 16), f(S::DamageMax, 28), f(S::Energy, 20), f(S::Mana, 50), p(S::CastSpeed, 16)], "Resonne avec celui qui le tient."));
    v.push(mk(10_607, "Necro-Magus", K::Staff, 35, vec![f(S::DamageMin, 20), f(S::DamageMax, 34), f(S::PoisonResist, 20), p(S::CastSpeed, 18)], "L'embaumeur l'a tenu jusqu'au bout."));
    v.push(mk(10_608, "Tombeau", K::Staff, 38, vec![f(S::DamageMin, 24), f(S::DamageMax, 40), f(S::Energy, 25), f(S::Mana, 60)], "Couvre les morts d'un mot."));
    v.push(mk(10_609, "Lich", K::Staff, 40, vec![f(S::DamageMin, 28), f(S::DamageMax, 46), f(S::Energy, 28), p(S::CastSpeed, 20), f(S::ColdResist, 25)], "Le lich y a laisse son chuchotement."));
    v.push(mk(10_610, "Magus-Garde", K::Staff, 45, vec![f(S::DamageMin, 32), f(S::DamageMax, 54), f(S::Energy, 30), f(S::Defense, 30)], "Insigne des magisters de cour."));
    v.push(mk(10_611, "Sceptre-Hauteur", K::Staff, 50, vec![f(S::DamageMin, 38), f(S::DamageMax, 62), f(S::Energy, 34), p(S::CastSpeed, 22)], "Pour parler haut, et plus loin."));
    v.push(mk(10_612, "Pouvoir", K::Staff, 55, vec![f(S::DamageMin, 44), f(S::DamageMax, 70), f(S::Energy, 38), f(S::Mana, 80)], "On le tient comme on tient un roi."));
    v.push(mk(10_613, "Conduit", K::Staff, 58, vec![f(S::DamageMin, 50), f(S::DamageMax, 78), p(S::CastSpeed, 24), f(S::LightningResist, 25)], "L'air autour vibre tout seul."));
    v.push(mk(10_614, "Couronne", K::Staff, 60, vec![f(S::DamageMin, 56), f(S::DamageMax, 88), f(S::Energy, 42), f(S::Mana, 100), p(S::CastSpeed, 26)], "Au-dessus, plus rien que le ciel."));
    v.push(mk(10_615, "Pyrose", K::Staff, 65, vec![f(S::DamageMin, 64), f(S::DamageMax, 105), f(S::FireResist, 30), p(S::CastSpeed, 26)], "La pointe a brule trois portes."));
    v.push(mk(10_616, "Cryomance", K::Staff, 70, vec![f(S::DamageMin, 72), f(S::DamageMax, 118), f(S::ColdResist, 30), f(S::Energy, 45)], "Gel a meme l'air autour."));
    v.push(mk(10_617, "Tempeste", K::Staff, 75, vec![f(S::DamageMin, 80), f(S::DamageMax, 130), f(S::LightningResist, 30), p(S::CastSpeed, 28)], "Le ciel s'y referme et s'y ouvre."));
    v.push(mk(10_618, "Drakestaff", K::Staff, 78, vec![f(S::DamageMin, 88), f(S::DamageMax, 142), f(S::Energy, 50), f(S::FireResist, 25)], "Os de drake taille en hampe."));
    v.push(mk(10_619, "Wyrm-Conduit", K::Staff, 80, vec![f(S::DamageMin, 96), f(S::DamageMax, 155), f(S::FireResist, 25), f(S::ColdResist, 25), p(S::CastSpeed, 30)], "Le wyrm parlait par lui."));
    v.push(mk(10_620, "Aether", K::Staff, 85, vec![f(S::DamageMin, 108), f(S::DamageMax, 178), f(S::Energy, 60), f(S::Mana, 150), p(S::CastSpeed, 32)], "Trop leger, comme une idee."));
    v.push(mk(10_621, "Chant-Etoile", K::Staff, 90, vec![f(S::DamageMin, 122), f(S::DamageMax, 200), f(S::Energy, 65), f(S::LifeOnHit, 25), p(S::CastSpeed, 34)], "Une etoile y dort encore."));
    v.push(mk(10_622, "Demonbinder", K::Staff, 95, vec![f(S::DamageMin, 138), f(S::DamageMax, 220), f(S::FireResist, 35), f(S::Energy, 70), p(S::CastSpeed, 36)], "Lie ce qui hurle."));
    v.push(mk(10_623, "Eternel", K::Staff, 98, vec![f(S::DamageMin, 152), f(S::DamageMax, 240), f(S::Energy, 75), f(S::Mana, 200), p(S::CastSpeed, 38)], "Plus vieux que celui qui le porte."));
    v.push(mk(10_624, "Chaos-Conduit", K::Staff, 100, vec![f(S::DamageMin, 170), f(S::DamageMax, 270), f(S::Energy, 85), f(S::Mana, 220), p(S::CastSpeed, 42), p(S::CritChance, 25)], "Le chaos passe par les deux bouts."));

    // ── WANDS (10_700..) ───────────────────────────────────────────────────
    v.push(mk(10_700, "Eclat", K::Wand, 5, vec![f(S::DamageMin, 1), f(S::DamageMax, 4), f(S::Energy, 4), p(S::CastSpeed, 8)], "Un eclat de l'autel du village."));
    v.push(mk(10_701, "Etincelle", K::Wand, 10, vec![f(S::DamageMin, 3), f(S::DamageMax, 7), f(S::Energy, 6), p(S::CritChance, 8)], "Au reveil, elle pique encore."));
    v.push(mk(10_702, "Brindille", K::Wand, 15, vec![f(S::DamageMin, 4), f(S::DamageMax, 10), f(S::Energy, 8), p(S::CastSpeed, 12)], "Tellement legere qu'elle plane."));
    v.push(mk(10_703, "Eclat-Bois", K::Wand, 18, vec![f(S::DamageMin, 6), f(S::DamageMax, 13), f(S::Energy, 10), f(S::Mana, 25)], "Coeur d'orme tendre."));
    v.push(mk(10_704, "Lien d'Aube", K::Wand, 20, vec![f(S::DamageMin, 8), f(S::DamageMax, 16), f(S::Energy, 12), p(S::CastSpeed, 14), p(S::CritChance, 10)], "Aube prise dans un manche."));
    v.push(mk(10_705, "Os-Lien", K::Wand, 25, vec![f(S::DamageMin, 10), f(S::DamageMax, 19), f(S::Energy, 14), p(S::CritChance, 12)], "Os de chouette taille a la lune."));
    v.push(mk(10_706, "Murmure", K::Wand, 30, vec![f(S::DamageMin, 13), f(S::DamageMax, 24), f(S::Energy, 16), p(S::CastSpeed, 16)], "On l'entend en collant l'oreille."));
    v.push(mk(10_707, "Ame", K::Wand, 35, vec![f(S::DamageMin, 16), f(S::DamageMax, 28), f(S::Mana, 50), p(S::CastSpeed, 18)], "Une ame y boude sa libellule."));
    v.push(mk(10_708, "Sepulcre", K::Wand, 38, vec![f(S::DamageMin, 18), f(S::DamageMax, 32), f(S::Energy, 20), f(S::PoisonResist, 20)], "Etoupe issue d'un linceul."));
    v.push(mk(10_709, "Cri-Esprit", K::Wand, 40, vec![f(S::DamageMin, 22), f(S::DamageMax, 38), f(S::Energy, 22), p(S::CastSpeed, 20), p(S::CritChance, 14)], "Crie pour les muets."));
    v.push(mk(10_710, "Conduit-Acier", K::Wand, 45, vec![f(S::DamageMin, 26), f(S::DamageMax, 44), f(S::Energy, 25), f(S::Defense, 25)], "Fer froid sur main chaude."));
    v.push(mk(10_711, "Tour", K::Wand, 50, vec![f(S::DamageMin, 30), f(S::DamageMax, 52), f(S::Energy, 28), f(S::Mana, 70)], "Une tour resumee dans un manche."));
    v.push(mk(10_712, "Edit", K::Wand, 55, vec![f(S::DamageMin, 35), f(S::DamageMax, 58), p(S::CastSpeed, 22), p(S::CritChance, 16)], "Promulguait la mort."));
    v.push(mk(10_713, "Conclave", K::Wand, 58, vec![f(S::DamageMin, 40), f(S::DamageMax, 66), f(S::Energy, 32), f(S::Mana, 80)], "Sept mains s'y sont posees."));
    v.push(mk(10_714, "Magister", K::Wand, 60, vec![f(S::DamageMin, 45), f(S::DamageMax, 74), f(S::Energy, 35), p(S::CastSpeed, 24), p(S::CritChance, 18)], "Pour celui qui devance la pensee."));
    v.push(mk(10_715, "Petite-Flamme", K::Wand, 65, vec![f(S::DamageMin, 52), f(S::DamageMax, 86), f(S::FireResist, 25), p(S::CritChance, 20)], "Tient une braise par habitude."));
    v.push(mk(10_716, "Eclat-Glace", K::Wand, 70, vec![f(S::DamageMin, 58), f(S::DamageMax, 96), f(S::ColdResist, 25), p(S::CastSpeed, 26)], "Glace meme le souffle."));
    v.push(mk(10_717, "Eclair", K::Wand, 75, vec![f(S::DamageMin, 65), f(S::DamageMax, 108), f(S::LightningResist, 25), p(S::CritChance, 22)], "Une etincelle naissante."));
    v.push(mk(10_718, "Petite-Drake", K::Wand, 78, vec![f(S::DamageMin, 72), f(S::DamageMax, 118), f(S::Energy, 42), f(S::FireResist, 22)], "Crochet sec d'un jeune drake."));
    v.push(mk(10_719, "Wyrm-Petit", K::Wand, 80, vec![f(S::DamageMin, 80), f(S::DamageMax, 128), f(S::Energy, 45), p(S::CastSpeed, 28)], "Le wyrm en a perdu trois."));
    v.push(mk(10_720, "Divinite", K::Wand, 85, vec![f(S::DamageMin, 90), f(S::DamageMax, 148), f(S::Energy, 55), p(S::CritChance, 24), p(S::CastSpeed, 30)], "Plume ravie a un saint."));
    v.push(mk(10_721, "Cherubin", K::Wand, 90, vec![f(S::DamageMin, 102), f(S::DamageMax, 168), f(S::LifeOnHit, 20), p(S::CastSpeed, 32)], "Le chant des cherubins l'a tordue."));
    v.push(mk(10_722, "Demon-Mineur", K::Wand, 95, vec![f(S::DamageMin, 115), f(S::DamageMax, 188), f(S::FireResist, 30), f(S::Energy, 60), p(S::CritChance, 28)], "Un demon mineur, mais reel."));
    v.push(mk(10_723, "Eternel-Mineur", K::Wand, 98, vec![f(S::DamageMin, 128), f(S::DamageMax, 205), f(S::Mana, 150), p(S::CastSpeed, 34), p(S::CritChance, 30)], "Petite eternite tenue par un doigt."));
    v.push(mk(10_724, "Chaos-Eclat", K::Wand, 100, vec![f(S::DamageMin, 145), f(S::DamageMax, 230), f(S::Energy, 70), p(S::CastSpeed, 38), p(S::CritChance, 32), f(S::LifeOnHit, 25)], "Un eclat du chaos."));

    // ── SCEPTERS (10_800..) ────────────────────────────────────────────────
    v.push(mk(10_800, "Prelat", K::Scepter, 5, vec![f(S::DamageMin, 2), f(S::DamageMax, 5), f(S::Energy, 4), f(S::Defense, 8)], "Pour benir et fendre."));
    v.push(mk(10_801, "Chapelain", K::Scepter, 10, vec![f(S::DamageMin, 4), f(S::DamageMax, 9), f(S::Energy, 6), f(S::Mana, 20)], "Toujours une priere prete."));
    v.push(mk(10_802, "Espoir", K::Scepter, 15, vec![f(S::DamageMin, 6), f(S::DamageMax, 12), f(S::Vitality, 6), f(S::Life, 20), p(S::CastSpeed, 8)], "Petite lumiere au creux du metal."));
    v.push(mk(10_803, "Lien-Loup", K::Scepter, 18, vec![f(S::DamageMin, 8), f(S::DamageMax, 15), f(S::Energy, 10), p(S::CastSpeed, 10)], "Le clerge des loups l'a beni."));
    v.push(mk(10_804, "Aube Mineure", K::Scepter, 20, vec![f(S::DamageMin, 10), f(S::DamageMax, 18), f(S::Energy, 12), f(S::Defense, 18), f(S::Mana, 35)], "Premier rayon dans la nef."));
    v.push(mk(10_805, "Reliquaire", K::Scepter, 25, vec![f(S::DamageMin, 13), f(S::DamageMax, 23), f(S::Energy, 15), f(S::Defense, 25)], "Une dent de saint au pommeau."));
    v.push(mk(10_806, "Saint-Os", K::Scepter, 30, vec![f(S::DamageMin, 16), f(S::DamageMax, 28), f(S::Energy, 18), p(S::CastSpeed, 14)], "Os benit, polit par la priere."));
    v.push(mk(10_807, "Ombre", K::Scepter, 35, vec![f(S::DamageMin, 20), f(S::DamageMax, 34), f(S::PoisonResist, 20), f(S::Energy, 22)], "L'ombre y dort comme un chien."));
    v.push(mk(10_808, "Sepulcrale", K::Scepter, 38, vec![f(S::DamageMin, 23), f(S::DamageMax, 39), f(S::Defense, 35), f(S::Mana, 50)], "Servait au scellement."));
    v.push(mk(10_809, "Lich-Reliquaire", K::Scepter, 40, vec![f(S::DamageMin, 27), f(S::DamageMax, 46), f(S::Energy, 25), f(S::ColdResist, 25), p(S::CastSpeed, 18)], "Un lich y a laisse un signet."));
    v.push(mk(10_810, "Croisade", K::Scepter, 45, vec![f(S::DamageMin, 32), f(S::DamageMax, 54), f(S::Energy, 28), f(S::Defense, 40)], "Marche avec la banniere."));
    v.push(mk(10_811, "Pretre-Guerre", K::Scepter, 50, vec![f(S::DamageMin, 38), f(S::DamageMax, 62), f(S::Strength, 25), f(S::Energy, 28)], "Pour celui qui prie en armure."));
    v.push(mk(10_812, "Croix", K::Scepter, 55, vec![f(S::DamageMin, 44), f(S::DamageMax, 70), f(S::Defense, 55), f(S::Vitality, 25)], "Pommeau cisele d'une croix."));
    v.push(mk(10_813, "Cathedrale", K::Scepter, 58, vec![f(S::DamageMin, 50), f(S::DamageMax, 80), f(S::Energy, 35), f(S::Mana, 90)], "Nef gravee sur le manche."));
    v.push(mk(10_814, "Indulgence", K::Scepter, 60, vec![f(S::DamageMin, 56), f(S::DamageMax, 88), f(S::Energy, 38), p(S::CastSpeed, 22), f(S::Defense, 60)], "Pardonne, et frappe quand meme."));
    v.push(mk(10_815, "Petit-Drake", K::Scepter, 65, vec![f(S::DamageMin, 64), f(S::DamageMax, 100), f(S::FireResist, 28), f(S::Energy, 40)], "Pommeau dans une dent juvenile."));
    v.push(mk(10_816, "Cendre", K::Scepter, 70, vec![f(S::DamageMin, 72), f(S::DamageMax, 115), f(S::FireResist, 30), p(S::CastSpeed, 24)], "Couvert d'une cendre tiede."));
    v.push(mk(10_817, "Glaciere", K::Scepter, 75, vec![f(S::DamageMin, 80), f(S::DamageMax, 128), f(S::ColdResist, 30), f(S::Energy, 45)], "Le pommeau givre toute la journee."));
    v.push(mk(10_818, "Foudre-Sceptre", K::Scepter, 78, vec![f(S::DamageMin, 88), f(S::DamageMax, 140), f(S::LightningResist, 30), p(S::CritChance, 22)], "Tient l'orage par la queue."));
    v.push(mk(10_819, "Wyrm-Sceptre", K::Scepter, 80, vec![f(S::DamageMin, 95), f(S::DamageMax, 152), f(S::FireResist, 25), f(S::ColdResist, 25), f(S::Energy, 48)], "Forme par les anneaux d'un wyrm."));
    v.push(mk(10_820, "Saint", K::Scepter, 85, vec![f(S::DamageMin, 108), f(S::DamageMax, 178), f(S::Energy, 58), f(S::Life, 80), p(S::CastSpeed, 30)], "Saint, mais pas tendre."));
    v.push(mk(10_821, "Archonte", K::Scepter, 90, vec![f(S::DamageMin, 122), f(S::DamageMax, 200), f(S::Energy, 65), f(S::Defense, 80), p(S::CastSpeed, 32)], "L'ordre eternel le reconnait."));
    v.push(mk(10_822, "Demon-Sceptre", K::Scepter, 95, vec![f(S::DamageMin, 138), f(S::DamageMax, 220), f(S::FireResist, 35), f(S::Energy, 70), p(S::CritChance, 26)], "Sept demons mineurs cousus dedans."));
    v.push(mk(10_823, "Eternel-Sceptre", K::Scepter, 98, vec![f(S::DamageMin, 152), f(S::DamageMax, 240), f(S::Energy, 75), f(S::Mana, 180), p(S::CastSpeed, 36)], "Tient la perpetuite par le manche."));
    v.push(mk(10_824, "Chaos-Sceptre", K::Scepter, 100, vec![f(S::DamageMin, 170), f(S::DamageMax, 270), f(S::Energy, 82), f(S::Mana, 220), p(S::CastSpeed, 40), p(S::CritChance, 28)], "Le chaos eleve sa propre eglise."));

    // ── HELMS (11_000..) ───────────────────────────────────────────────────
    v.push(mk(11_000, "Heaume du Novice", K::Helm, 5, vec![f(S::Defense, 12), f(S::Vitality, 4)], "Un peu trop grand, encore."));
    v.push(mk(11_001, "Capuche de Cuir", K::Helm, 10, vec![f(S::Defense, 18), f(S::Vitality, 6), f(S::Life, 15)], "Sent encore l'huile de tannerie."));
    v.push(mk(11_002, "Heaume du Garde", K::Helm, 15, vec![f(S::Defense, 28), f(S::Vitality, 10), f(S::FireResist, 12)], "Cabosse au front."));
    v.push(mk(11_003, "Masque du Loup", K::Helm, 18, vec![f(S::Defense, 35), f(S::Strength, 8), f(S::Life, 25)], "Le rictus de la bete y resta."));
    v.push(mk(11_004, "Heaume d'Aube", K::Helm, 20, vec![f(S::Defense, 42), f(S::Vitality, 14), f(S::FireResist, 15), f(S::ColdResist, 15)], "Frais a la tete au matin."));
    v.push(mk(11_005, "Crane", K::Helm, 25, vec![f(S::Defense, 52), f(S::Strength, 12), f(S::PoisonResist, 18)], "Un vrai. Repolish."));
    v.push(mk(11_006, "Cagoule Sepulcre", K::Helm, 30, vec![f(S::Defense, 62), f(S::Vitality, 18), f(S::ColdResist, 20)], "Tisse d'un linceul royal."));
    v.push(mk(11_007, "Masque Spectral", K::Helm, 35, vec![f(S::Defense, 75), f(S::Energy, 15), f(S::Mana, 40)], "Brille faiblement la nuit."));
    v.push(mk(11_008, "Heaume du Lich", K::Helm, 38, vec![f(S::Defense, 85), f(S::Vitality, 22), f(S::ColdResist, 25), f(S::PoisonResist, 25)], "Sourit derriere ta nuque."));
    v.push(mk(11_009, "Crane de Necromancien", K::Helm, 40, vec![f(S::Defense, 95), f(S::Energy, 20), f(S::Life, 50), f(S::ColdResist, 28)], "Avait deja une couronne."));
    v.push(mk(11_010, "Heaume d'Acier", K::Helm, 45, vec![f(S::Defense, 115), f(S::Vitality, 25), f(S::FireResist, 22), f(S::ColdResist, 22)], "Sorti d'une forge royale."));
    v.push(mk(11_011, "Champion", K::Helm, 50, vec![f(S::Defense, 135), f(S::Strength, 18), f(S::Vitality, 28), f(S::LightningResist, 25)], "Marque du tournoi noir."));
    v.push(mk(11_012, "Heaume du Chevalier", K::Helm, 55, vec![f(S::Defense, 155), f(S::Vitality, 32), f(S::FireResist, 25), f(S::PoisonResist, 25)], "Bouche close, voix porte."));
    v.push(mk(11_013, "Couronne d'Acier", K::Helm, 58, vec![f(S::Defense, 175), f(S::Strength, 22), f(S::AttackRating, 50), f(S::Life, 70)], "Pour qui regne par les armes."));
    v.push(mk(11_014, "Heaume du Roi", K::Helm, 60, vec![f(S::Defense, 200), f(S::Vitality, 40), f(S::FireResist, 28), f(S::ColdResist, 28), f(S::LightningResist, 28)], "Un seul roi, et son ombre."));
    v.push(mk(11_015, "Masque de Flamme", K::Helm, 65, vec![f(S::Defense, 225), f(S::FireResist, 40), f(S::Vitality, 35)], "On ne le retire jamais."));
    v.push(mk(11_016, "Masque de Glace", K::Helm, 70, vec![f(S::Defense, 250), f(S::ColdResist, 40), f(S::Vitality, 40)], "Givre encore tes cheveux."));
    v.push(mk(11_017, "Masque de Foudre", K::Helm, 75, vec![f(S::Defense, 280), f(S::LightningResist, 40), f(S::Energy, 25)], "Vibre dans les dents."));
    v.push(mk(11_018, "Heaume Drake", K::Helm, 78, vec![f(S::Defense, 310), f(S::Vitality, 45), f(S::FireResist, 35), f(S::ColdResist, 30)], "Forge d'ecaille verte."));
    v.push(mk(11_019, "Masque Wyrm", K::Helm, 80, vec![f(S::Defense, 340), f(S::FireResist, 30), f(S::ColdResist, 30), f(S::LightningResist, 30), f(S::Life, 100)], "Le wyrm est dedans. Encore."));
    v.push(mk(11_020, "Couronne Divine", K::Helm, 85, vec![f(S::Defense, 380), f(S::Vitality, 55), f(S::FireResist, 35), f(S::ColdResist, 35), f(S::LightningResist, 35), f(S::PoisonResist, 35)], "Plus belle qu'un ciel d'ete."));
    v.push(mk(11_021, "Aureole", K::Helm, 90, vec![f(S::Defense, 420), f(S::Energy, 50), f(S::Mana, 200), f(S::LifeOnHit, 20), f(S::LightningResist, 40)], "Reste droite meme apres ta mort."));
    v.push(mk(11_022, "Masque Demoniaque", K::Helm, 95, vec![f(S::Defense, 470), f(S::Strength, 50), f(S::FireResist, 45), f(S::Vitality, 60)], "Soude au visage du porteur."));
    v.push(mk(11_023, "Couronne Eternelle", K::Helm, 98, vec![f(S::Defense, 520), f(S::Vitality, 70), f(S::FireResist, 40), f(S::ColdResist, 40), f(S::LightningResist, 40), f(S::PoisonResist, 40)], "Sept rois s'y sont reflets."));
    v.push(mk(11_024, "Heaume du Chaos", K::Helm, 100, vec![f(S::Defense, 580), f(S::Vitality, 85), f(S::FireResist, 45), f(S::ColdResist, 45), f(S::LightningResist, 45), f(S::PoisonResist, 45), f(S::Life, 150)], "Le porteur cesse d'avoir un nom."));

    // ── BODY ARMOR (11_100..) ──────────────────────────────────────────────
    v.push(mk(11_100, "Toile du Novice", K::BodyArmor, 5, vec![f(S::Defense, 18), f(S::Vitality, 5)], "Un nom prete pour une toile prete."));
    v.push(mk(11_101, "Cuir de Loup", K::BodyArmor, 10, vec![f(S::Defense, 28), f(S::Vitality, 8), f(S::Life, 25)], "Marche encore dans son sommeil."));
    v.push(mk(11_102, "Plastron du Garde", K::BodyArmor, 15, vec![f(S::Defense, 42), f(S::Vitality, 12), f(S::FireResist, 15)], "Pour faire le poids au mur."));
    v.push(mk(11_103, "Cuir Sauvage", K::BodyArmor, 18, vec![f(S::Defense, 55), f(S::Strength, 10), f(S::Dexterity, 8)], "Sent la sueur d'une autre vie."));
    v.push(mk(11_104, "Plastron d'Aube", K::BodyArmor, 20, vec![f(S::Defense, 68), f(S::Vitality, 16), f(S::FireResist, 18), f(S::ColdResist, 18)], "Premier reflexe contre la lance."));
    v.push(mk(11_105, "Armure d'Os", K::BodyArmor, 25, vec![f(S::Defense, 85), f(S::Vitality, 20), f(S::PoisonResist, 22)], "Cliquete a chaque pas."));
    v.push(mk(11_106, "Linceul", K::BodyArmor, 30, vec![f(S::Defense, 100), f(S::Vitality, 25), f(S::ColdResist, 25)], "Tissu froid, peau froide."));
    v.push(mk(11_107, "Cotte Spectrale", K::BodyArmor, 35, vec![f(S::Defense, 120), f(S::Energy, 18), f(S::Mana, 50)], "Visible seulement de cote."));
    v.push(mk(11_108, "Armure Tombeau", K::BodyArmor, 38, vec![f(S::Defense, 140), f(S::Vitality, 30), f(S::ColdResist, 28), f(S::PoisonResist, 28)], "Couvercle decoupe en cotte."));
    v.push(mk(11_109, "Armure du Lich", K::BodyArmor, 40, vec![f(S::Defense, 160), f(S::Vitality, 35), f(S::ColdResist, 30), f(S::LightningResist, 25)], "Le lich y dort en attendant."));
    v.push(mk(11_110, "Cotte d'Acier", K::BodyArmor, 45, vec![f(S::Defense, 195), f(S::Vitality, 40), f(S::FireResist, 25)], "Maillons d'une fonderie disparue."));
    v.push(mk(11_111, "Mail du Croise", K::BodyArmor, 50, vec![f(S::Defense, 230), f(S::Strength, 25), f(S::Vitality, 45), f(S::FireResist, 28)], "Foi cousue dans le metal."));
    v.push(mk(11_112, "Plastron du Lord", K::BodyArmor, 55, vec![f(S::Defense, 270), f(S::Vitality, 50), f(S::FireResist, 30), f(S::ColdResist, 30)], "Tient mieux qu'un titre."));
    v.push(mk(11_113, "Carapace", K::BodyArmor, 58, vec![f(S::Defense, 310), f(S::Strength, 32), f(S::Vitality, 55), f(S::Life, 100)], "Lourde, mais elle te porte."));
    v.push(mk(11_114, "Bastion", K::BodyArmor, 60, vec![f(S::Defense, 350), f(S::Vitality, 60), f(S::FireResist, 32), f(S::ColdResist, 32), f(S::LightningResist, 32)], "Une cite tenue par toi seul."));
    v.push(mk(11_115, "Plastron de Flamme", K::BodyArmor, 65, vec![f(S::Defense, 395), f(S::FireResist, 45), f(S::Vitality, 50)], "Chaud, meme dans l'hiver."));
    v.push(mk(11_116, "Plastron de Glace", K::BodyArmor, 70, vec![f(S::Defense, 440), f(S::ColdResist, 45), f(S::Vitality, 55)], "Tu ne transpires plus."));
    v.push(mk(11_117, "Plastron de Foudre", K::BodyArmor, 75, vec![f(S::Defense, 490), f(S::LightningResist, 45), f(S::Energy, 30)], "Picote le porteur a tout coup."));
    v.push(mk(11_118, "Cuir de Drake", K::BodyArmor, 78, vec![f(S::Defense, 540), f(S::Vitality, 65), f(S::FireResist, 35), f(S::ColdResist, 35)], "Encore tiede au coeur."));
    v.push(mk(11_119, "Ecailles Wyrm", K::BodyArmor, 80, vec![f(S::Defense, 600), f(S::FireResist, 35), f(S::ColdResist, 35), f(S::LightningResist, 35), f(S::Life, 150)], "Le wyrm a glisse hors de ces ecailles."));
    v.push(mk(11_120, "Plastron d'Aether", K::BodyArmor, 85, vec![f(S::Defense, 660), f(S::Vitality, 75), f(S::FireResist, 40), f(S::ColdResist, 40), f(S::LightningResist, 40), f(S::PoisonResist, 40)], "Plus leger qu'une vraie pensee."));
    v.push(mk(11_121, "Plastron Seraph", K::BodyArmor, 90, vec![f(S::Defense, 730), f(S::Energy, 55), f(S::Mana, 200), f(S::LightningResist, 45), f(S::LifeOnHit, 25)], "Aile six fois pliee."));
    v.push(mk(11_122, "Plastron du Demon", K::BodyArmor, 95, vec![f(S::Defense, 810), f(S::Strength, 60), f(S::FireResist, 50), f(S::Vitality, 80)], "Lie a la peau du porteur."));
    v.push(mk(11_123, "Plastron Eternel", K::BodyArmor, 98, vec![f(S::Defense, 890), f(S::Vitality, 90), f(S::FireResist, 45), f(S::ColdResist, 45), f(S::LightningResist, 45), f(S::PoisonResist, 45)], "Habite par celui qui le porta."));
    v.push(mk(11_124, "Plastron du Chaos", K::BodyArmor, 100, vec![f(S::Defense, 990), f(S::Vitality, 100), f(S::FireResist, 50), f(S::ColdResist, 50), f(S::LightningResist, 50), f(S::PoisonResist, 50), f(S::Life, 200)], "Aucune lame ne se souviendra."));

    // ── GLOVES (11_200..) ──────────────────────────────────────────────────
    v.push(mk(11_200, "Gants de Cuir", K::Gloves, 5, vec![f(S::Defense, 8), f(S::Dexterity, 3)], "Coussinets uses."));
    v.push(mk(11_201, "Doigt-Fin", K::Gloves, 10, vec![f(S::Defense, 14), f(S::Dexterity, 6), p(S::AttackSpeed, 5)], "Pour les doigts qui parlent."));
    v.push(mk(11_202, "Patte de Loup", K::Gloves, 15, vec![f(S::Defense, 20), f(S::Strength, 6), p(S::CritChance, 6)], "Griffes encore enfoncees."));
    v.push(mk(11_203, "Gantelet du Garde", K::Gloves, 18, vec![f(S::Defense, 28), f(S::Dexterity, 10), f(S::AttackRating, 15)], "Pour serrer la lance."));
    v.push(mk(11_204, "Gants d'Aube", K::Gloves, 20, vec![f(S::Defense, 35), f(S::Dexterity, 12), p(S::AttackSpeed, 8), p(S::CritChance, 8)], "Premier reflexe du matin."));
    v.push(mk(11_205, "Doigts d'Os", K::Gloves, 25, vec![f(S::Defense, 45), f(S::Dexterity, 14), p(S::CritChance, 10)], "Os colles a la peau."));
    v.push(mk(11_206, "Gants d'Ombre", K::Gloves, 30, vec![f(S::Defense, 55), f(S::Dexterity, 18), p(S::AttackSpeed, 12)], "Les doigts passent partout."));
    v.push(mk(11_207, "Gants Spectraux", K::Gloves, 35, vec![f(S::Defense, 65), f(S::Energy, 15), p(S::AttackSpeed, 14)], "Souples comme l'air."));
    v.push(mk(11_208, "Gants Sepulcre", K::Gloves, 38, vec![f(S::Defense, 78), f(S::Dexterity, 22), p(S::CritChance, 14)], "On a coupe ces doigts deux fois."));
    v.push(mk(11_209, "Gants du Lich", K::Gloves, 40, vec![f(S::Defense, 88), f(S::Dexterity, 25), f(S::ColdResist, 22), p(S::AttackSpeed, 15)], "Toujours froids."));
    v.push(mk(11_210, "Gants d'Acier", K::Gloves, 45, vec![f(S::Defense, 105), f(S::Strength, 18), f(S::Dexterity, 22), p(S::CritChance, 16)], "Tres bien noues."));
    v.push(mk(11_211, "Gantelet du Champion", K::Gloves, 50, vec![f(S::Defense, 125), f(S::Dexterity, 28), p(S::AttackSpeed, 17), p(S::CritChance, 17)], "Tient l'arme mieux que le bras."));
    v.push(mk(11_212, "Gants de Croisade", K::Gloves, 55, vec![f(S::Defense, 145), f(S::FireResist, 20), f(S::Dexterity, 30), p(S::AttackSpeed, 18)], "Benis avant le mur."));
    v.push(mk(11_213, "Gants Cathedrale", K::Gloves, 58, vec![f(S::Defense, 165), f(S::Dexterity, 35), f(S::Energy, 20), p(S::CritChance, 18)], "Une nef sur le dos de la main."));
    v.push(mk(11_214, "Gants du Roi", K::Gloves, 60, vec![f(S::Defense, 185), f(S::Dexterity, 38), p(S::AttackSpeed, 20), p(S::CritChance, 20)], "Le roi savait s'en servir."));
    v.push(mk(11_215, "Gants de Flamme", K::Gloves, 65, vec![f(S::Defense, 210), f(S::FireResist, 30), p(S::AttackSpeed, 22)], "Plus tu frappes, plus c'est chaud."));
    v.push(mk(11_216, "Gants de Givre", K::Gloves, 70, vec![f(S::Defense, 240), f(S::ColdResist, 30), p(S::CritChance, 22)], "L'air sort en buee."));
    v.push(mk(11_217, "Gants d'Eclair", K::Gloves, 75, vec![f(S::Defense, 270), f(S::LightningResist, 30), p(S::AttackSpeed, 25)], "Les doigts paraissent flous."));
    v.push(mk(11_218, "Gants Drake", K::Gloves, 78, vec![f(S::Defense, 300), f(S::Dexterity, 45), f(S::FireResist, 25)], "Ecailles cousues sur cuir."));
    v.push(mk(11_219, "Gants Wyrm", K::Gloves, 80, vec![f(S::Defense, 330), f(S::Dexterity, 48), p(S::AttackSpeed, 26), p(S::CritChance, 24)], "Tient l'arme mieux que ton sang."));
    v.push(mk(11_220, "Gants Divins", K::Gloves, 85, vec![f(S::Defense, 365), f(S::Dexterity, 55), p(S::AttackSpeed, 28), p(S::CritChance, 26), f(S::LifeOnHit, 18)], "Sentent l'orage avant la pluie."));
    v.push(mk(11_221, "Gants Cherubin", K::Gloves, 90, vec![f(S::Defense, 405), f(S::Energy, 45), f(S::Mana, 100), p(S::AttackSpeed, 30)], "Six doigts, dans une paume."));
    v.push(mk(11_222, "Gants Demon", K::Gloves, 95, vec![f(S::Defense, 450), f(S::Strength, 50), f(S::FireResist, 40), p(S::CritChance, 30)], "Pas vraiment des gants."));
    v.push(mk(11_223, "Gants Eternels", K::Gloves, 98, vec![f(S::Defense, 500), f(S::Dexterity, 65), p(S::AttackSpeed, 32), p(S::CritChance, 32), f(S::LifeOnHit, 25)], "Ils ont deja tout casse une fois."));
    v.push(mk(11_224, "Gants du Chaos", K::Gloves, 100, vec![f(S::Defense, 555), f(S::Dexterity, 75), p(S::AttackSpeed, 35), p(S::CritChance, 35), f(S::LifeOnHit, 30)], "Aucune main n'y tient pareil."));

    // ── BOOTS (11_300..) ───────────────────────────────────────────────────
    v.push(mk(11_300, "Trotteurs", K::Boots, 5, vec![f(S::Defense, 8), p(S::MovementSpeed, 8)], "Trois villes, un manche."));
    v.push(mk(11_301, "Mocassins de Loup", K::Boots, 10, vec![f(S::Defense, 14), p(S::MovementSpeed, 12), f(S::Stamina, 20)], "Posent le pied sans crisser."));
    v.push(mk(11_302, "Bottes du Chasseur", K::Boots, 15, vec![f(S::Defense, 20), f(S::Dexterity, 8), p(S::MovementSpeed, 14)], "Les pistes commencent ici."));
    v.push(mk(11_303, "Bottes de Loup", K::Boots, 18, vec![f(S::Defense, 28), f(S::Vitality, 8), p(S::MovementSpeed, 15)], "Empreintes plus larges qu'un homme."));
    v.push(mk(11_304, "Coureur d'Aube", K::Boots, 20, vec![f(S::Defense, 35), p(S::MovementSpeed, 18), f(S::Stamina, 30)], "On les met une fois, on ne s'arrete plus."));
    v.push(mk(11_305, "Os-Marche", K::Boots, 25, vec![f(S::Defense, 45), p(S::MovementSpeed, 18), f(S::PoisonResist, 18)], "Cliquetent dans le noir."));
    v.push(mk(11_306, "Bottes Spectrales", K::Boots, 30, vec![f(S::Defense, 55), p(S::MovementSpeed, 20), f(S::ColdResist, 20)], "Trace, mais pas d'empreinte."));
    v.push(mk(11_307, "Bottes d'Ombre", K::Boots, 35, vec![f(S::Defense, 65), p(S::MovementSpeed, 22), f(S::Stamina, 40)], "Une ombre est cousue dedans."));
    v.push(mk(11_308, "Bottes du Tombeau", K::Boots, 38, vec![f(S::Defense, 75), f(S::Vitality, 18), p(S::MovementSpeed, 22)], "Empreintes en croix."));
    v.push(mk(11_309, "Bottes du Lich", K::Boots, 40, vec![f(S::Defense, 88), f(S::ColdResist, 25), p(S::MovementSpeed, 24)], "Pas un cri du parquet."));
    v.push(mk(11_310, "Bottes d'Acier", K::Boots, 45, vec![f(S::Defense, 105), f(S::Vitality, 22), p(S::MovementSpeed, 24)], "Pas un bruit sur la pierre."));
    v.push(mk(11_311, "Souliers Champion", K::Boots, 50, vec![f(S::Defense, 125), p(S::MovementSpeed, 26), p(S::AttackSpeed, 10), f(S::Stamina, 50)], "Champion deux fois en course."));
    v.push(mk(11_312, "Bottes Mur", K::Boots, 55, vec![f(S::Defense, 145), p(S::MovementSpeed, 28), f(S::FireResist, 25)], "Marchaient sur les remparts."));
    v.push(mk(11_313, "Bottes Bastion", K::Boots, 58, vec![f(S::Defense, 165), p(S::MovementSpeed, 28), f(S::Vitality, 28), f(S::Life, 80)], "Lourdes, mais elles tiennent."));
    v.push(mk(11_314, "Bottes du Roi", K::Boots, 60, vec![f(S::Defense, 185), p(S::MovementSpeed, 30), p(S::AttackSpeed, 12), f(S::Stamina, 60)], "Le sol s'aplanit sous elles."));
    v.push(mk(11_315, "Cendre-Pas", K::Boots, 65, vec![f(S::Defense, 210), f(S::FireResist, 30), p(S::MovementSpeed, 30)], "La cendre fume dans le sillage."));
    v.push(mk(11_316, "Glace-Marche", K::Boots, 70, vec![f(S::Defense, 240), f(S::ColdResist, 30), p(S::MovementSpeed, 32)], "Tu glisses sans te plaindre."));
    v.push(mk(11_317, "Eclair-Pas", K::Boots, 75, vec![f(S::Defense, 270), f(S::LightningResist, 30), p(S::MovementSpeed, 34)], "Tu arrives avant ta main."));
    v.push(mk(11_318, "Bottes Drake", K::Boots, 78, vec![f(S::Defense, 300), f(S::Vitality, 40), p(S::MovementSpeed, 35), f(S::FireResist, 25)], "Les talons soufflent du feu."));
    v.push(mk(11_319, "Bottes Wyrm", K::Boots, 80, vec![f(S::Defense, 330), f(S::FireResist, 30), f(S::ColdResist, 30), p(S::MovementSpeed, 36)], "Le wyrm les a portees a sec."));
    v.push(mk(11_320, "Vol Divin", K::Boots, 85, vec![f(S::Defense, 365), p(S::MovementSpeed, 38), f(S::Stamina, 90), f(S::LightningResist, 35)], "Le sol n'a plus son mot a dire."));
    v.push(mk(11_321, "Bottes Aile", K::Boots, 90, vec![f(S::Defense, 405), p(S::MovementSpeed, 40), f(S::Energy, 40), f(S::Mana, 80)], "Battent quand tu cours."));
    v.push(mk(11_322, "Bottes Demon", K::Boots, 95, vec![f(S::Defense, 450), f(S::Strength, 50), f(S::FireResist, 40), p(S::MovementSpeed, 42)], "Brulent le sol qu'elles foulent."));
    v.push(mk(11_323, "Bottes Eternel", K::Boots, 98, vec![f(S::Defense, 500), p(S::MovementSpeed, 44), f(S::Stamina, 110), f(S::Life, 120)], "Aucune fatigue ne s'y accroche."));
    v.push(mk(11_324, "Bottes du Chaos", K::Boots, 100, vec![f(S::Defense, 555), p(S::MovementSpeed, 48), f(S::Stamina, 130), f(S::Life, 150), p(S::AttackSpeed, 20)], "Chaque pas reecrit la route."));

    // ── BELTS (11_400..) ───────────────────────────────────────────────────
    v.push(mk(11_400, "Sangle", K::Belt, 5, vec![f(S::Defense, 8), f(S::Vitality, 4)], "Tres simple, tres utile."));
    v.push(mk(11_401, "Cuir de Loup", K::Belt, 10, vec![f(S::Defense, 14), f(S::Strength, 6), f(S::Life, 18)], "Pelage encore present."));
    v.push(mk(11_402, "Ceinture du Garde", K::Belt, 15, vec![f(S::Defense, 22), f(S::Vitality, 10), f(S::Stamina, 25)], "Boucle frappee aux armes."));
    v.push(mk(11_403, "Ceinture du Loup", K::Belt, 18, vec![f(S::Defense, 30), f(S::Strength, 12), f(S::Vitality, 12)], "Le porteur a le pas large."));
    v.push(mk(11_404, "Sash d'Aube", K::Belt, 20, vec![f(S::Defense, 36), f(S::Vitality, 14), f(S::Life, 35), f(S::FireResist, 15)], "Premier reflexe du matin."));
    v.push(mk(11_405, "Os-Lien", K::Belt, 25, vec![f(S::Defense, 48), f(S::Strength, 16), f(S::PoisonResist, 20)], "Os couses entre eux."));
    v.push(mk(11_406, "Ceinture d'Ombre", K::Belt, 30, vec![f(S::Defense, 58), f(S::Dexterity, 18), f(S::Stamina, 40)], "Bouge sans la toucher."));
    v.push(mk(11_407, "Ceinture Spectre", K::Belt, 35, vec![f(S::Defense, 70), f(S::Vitality, 22), f(S::ColdResist, 22)], "On la voit la nuit, seulement."));
    v.push(mk(11_408, "Sash Sepulcre", K::Belt, 38, vec![f(S::Defense, 82), f(S::Strength, 22), f(S::Life, 60)], "Cordon d'un cercueil noble."));
    v.push(mk(11_409, "Sash du Lich", K::Belt, 40, vec![f(S::Defense, 95), f(S::Vitality, 28), f(S::ColdResist, 25), f(S::Mana, 50)], "Une boucle gravee de runes."));
    v.push(mk(11_410, "Ceinture d'Acier", K::Belt, 45, vec![f(S::Defense, 115), f(S::Strength, 22), f(S::Vitality, 28)], "Pour soutenir le plastron."));
    v.push(mk(11_411, "Centurion", K::Belt, 50, vec![f(S::Defense, 135), f(S::Vitality, 35), f(S::FireResist, 25)], "Cent batailles sur le cuir."));
    v.push(mk(11_412, "Ceinture du Lord", K::Belt, 55, vec![f(S::Defense, 160), f(S::Vitality, 40), f(S::Life, 100)], "Boucle d'or, lame en flanc."));
    v.push(mk(11_413, "Sash Royal", K::Belt, 58, vec![f(S::Defense, 180), f(S::Strength, 32), f(S::Vitality, 45), f(S::Life, 120)], "Couronne posee sur la hanche."));
    v.push(mk(11_414, "Sash Bastion", K::Belt, 60, vec![f(S::Defense, 200), f(S::Vitality, 50), f(S::FireResist, 28), f(S::ColdResist, 28), f(S::LightningResist, 28)], "Tient ton corps en place."));
    v.push(mk(11_415, "Sash de Flamme", K::Belt, 65, vec![f(S::Defense, 225), f(S::FireResist, 35), f(S::Vitality, 45)], "Brule par les deux bouts."));
    v.push(mk(11_416, "Sash de Glace", K::Belt, 70, vec![f(S::Defense, 250), f(S::ColdResist, 35), f(S::Vitality, 50)], "Tu sors une vapeur avec chaque mot."));
    v.push(mk(11_417, "Sash de Foudre", K::Belt, 75, vec![f(S::Defense, 280), f(S::LightningResist, 35), f(S::Energy, 30)], "Pique a chaque mouvement."));
    v.push(mk(11_418, "Sash Drake", K::Belt, 78, vec![f(S::Defense, 310), f(S::Strength, 40), f(S::FireResist, 28), f(S::Life, 130)], "Ecailles encore tiedes."));
    v.push(mk(11_419, "Sash Wyrm", K::Belt, 80, vec![f(S::Defense, 340), f(S::FireResist, 30), f(S::ColdResist, 30), f(S::LightningResist, 30), f(S::Life, 150)], "Anneau d'un wyrm endormi."));
    v.push(mk(11_420, "Sash Divin", K::Belt, 85, vec![f(S::Defense, 380), f(S::Vitality, 65), f(S::FireResist, 38), f(S::ColdResist, 38), f(S::LightningResist, 38), f(S::PoisonResist, 38)], "Tient l'ame en place."));
    v.push(mk(11_421, "Sash Aurore", K::Belt, 90, vec![f(S::Defense, 420), f(S::Vitality, 75), f(S::Life, 180), f(S::LightningResist, 40)], "Premier rayon des heros."));
    v.push(mk(11_422, "Sash Demon", K::Belt, 95, vec![f(S::Defense, 470), f(S::Strength, 60), f(S::FireResist, 45), f(S::Life, 200)], "Lie a la chair du porteur."));
    v.push(mk(11_423, "Sash Eternel", K::Belt, 98, vec![f(S::Defense, 520), f(S::Vitality, 90), f(S::FireResist, 42), f(S::ColdResist, 42), f(S::LightningResist, 42), f(S::PoisonResist, 42)], "Six rois, six boucles."));
    v.push(mk(11_424, "Sash du Chaos", K::Belt, 100, vec![f(S::Defense, 580), f(S::Vitality, 100), f(S::FireResist, 50), f(S::ColdResist, 50), f(S::LightningResist, 50), f(S::PoisonResist, 50), f(S::Life, 250)], "Tient ce qui ne devrait plus tenir."));

    // ── SHIELDS (11_500..) ─────────────────────────────────────────────────
    v.push(mk(11_500, "Bouclier du Novice", K::Shield, 5, vec![f(S::Defense, 12), f(S::BlockChance, 4)], "Trop large d'un cote."));
    v.push(mk(11_501, "Bouclier de Cuir", K::Shield, 10, vec![f(S::Defense, 22), f(S::BlockChance, 6), f(S::Vitality, 6)], "Strie de coups."));
    v.push(mk(11_502, "Bouclier-Loup", K::Shield, 15, vec![f(S::Defense, 32), f(S::BlockChance, 8), f(S::Strength, 8)], "Gueule de bete au centre."));
    v.push(mk(11_503, "Bouclier du Garde-Village", K::Shield, 18, vec![f(S::Defense, 42), f(S::BlockChance, 10), f(S::Vitality, 10), f(S::Life, 25)], "Couvre la moitie du village."));
    v.push(mk(11_504, "Aspis d'Aube", K::Shield, 20, vec![f(S::Defense, 52), f(S::BlockChance, 12), f(S::FireResist, 12), f(S::ColdResist, 12)], "Premier rempart de l'aube."));
    v.push(mk(11_505, "Bouclier d'Os", K::Shield, 25, vec![f(S::Defense, 65), f(S::BlockChance, 13), f(S::PoisonResist, 18)], "Os emboites en spirale."));
    v.push(mk(11_506, "Bouclier d'Ombre", K::Shield, 30, vec![f(S::Defense, 78), f(S::BlockChance, 14), f(S::Vitality, 18)], "Coule plus qu'il ne se tient."));
    v.push(mk(11_507, "Bouclier Spectral", K::Shield, 35, vec![f(S::Defense, 92), f(S::BlockChance, 15), f(S::Energy, 18), f(S::ColdResist, 22)], "Tu vois a travers."));
    v.push(mk(11_508, "Bouclier Sepulcre", K::Shield, 38, vec![f(S::Defense, 105), f(S::BlockChance, 16), f(S::Vitality, 25), f(S::PoisonResist, 25)], "Recoupe au scellement."));
    v.push(mk(11_509, "Bouclier du Lich", K::Shield, 40, vec![f(S::Defense, 120), f(S::BlockChance, 17), f(S::ColdResist, 28), f(S::Mana, 50)], "Givre meme au soleil."));
    v.push(mk(11_510, "Bouclier d'Acier", K::Shield, 45, vec![f(S::Defense, 145), f(S::BlockChance, 18), f(S::Vitality, 30), f(S::FireResist, 22)], "Plein, pas un orifice."));
    v.push(mk(11_511, "Aspis", K::Shield, 50, vec![f(S::Defense, 170), f(S::BlockChance, 20), f(S::Strength, 25), f(S::Vitality, 35)], "Bord en bronze poli."));
    v.push(mk(11_512, "Bouclier du Chevalier", K::Shield, 55, vec![f(S::Defense, 195), f(S::BlockChance, 22), f(S::Vitality, 40), f(S::FireResist, 25)], "Devise gravee de l'envers."));
    v.push(mk(11_513, "Garde-Roi", K::Shield, 58, vec![f(S::Defense, 220), f(S::BlockChance, 23), f(S::Vitality, 45), f(S::FireResist, 28), f(S::ColdResist, 28)], "Couronne incrustee."));
    v.push(mk(11_514, "Rempart", K::Shield, 60, vec![f(S::Defense, 250), f(S::BlockChance, 25), f(S::Vitality, 50), f(S::Life, 100), f(S::FireResist, 28)], "Une portion de mur portee."));
    v.push(mk(11_515, "Bouclier de Flamme", K::Shield, 65, vec![f(S::Defense, 285), f(S::BlockChance, 25), f(S::FireResist, 40), f(S::Vitality, 45)], "Repousse meme l'ombre des flammes."));
    v.push(mk(11_516, "Bouclier de Glace", K::Shield, 70, vec![f(S::Defense, 320), f(S::BlockChance, 26), f(S::ColdResist, 40), f(S::Vitality, 50)], "Gele les fleches en vol."));
    v.push(mk(11_517, "Bouclier de Foudre", K::Shield, 75, vec![f(S::Defense, 360), f(S::BlockChance, 27), f(S::LightningResist, 40), f(S::Energy, 30)], "Frappe en arriere d'un eclair."));
    v.push(mk(11_518, "Bouclier Drake", K::Shield, 78, vec![f(S::Defense, 400), f(S::BlockChance, 28), f(S::FireResist, 30), f(S::ColdResist, 30), f(S::Vitality, 55)], "Ecailles plates emboitees."));
    v.push(mk(11_519, "Bouclier Wyrm", K::Shield, 80, vec![f(S::Defense, 440), f(S::BlockChance, 29), f(S::FireResist, 32), f(S::ColdResist, 32), f(S::LightningResist, 32)], "Une seule ecaille, une seule plaque."));
    v.push(mk(11_520, "Aile d'Ange", K::Shield, 85, vec![f(S::Defense, 490), f(S::BlockChance, 30), f(S::Vitality, 65), f(S::Life, 150), f(S::LightningResist, 38)], "Aile blanche, bord coupant."));
    v.push(mk(11_521, "Soleil", K::Shield, 90, vec![f(S::Defense, 540), f(S::BlockChance, 31), f(S::FireResist, 45), f(S::Vitality, 75), f(S::Life, 180)], "Brille meme en cave."));
    v.push(mk(11_522, "Garde-Demon", K::Shield, 95, vec![f(S::Defense, 600), f(S::BlockChance, 33), f(S::Strength, 60), f(S::FireResist, 48), f(S::Life, 220)], "Mache les attaques."));
    v.push(mk(11_523, "Bouclier Eternel", K::Shield, 98, vec![f(S::Defense, 660), f(S::BlockChance, 34), f(S::Vitality, 90), f(S::FireResist, 45), f(S::ColdResist, 45), f(S::LightningResist, 45)], "Sept guerres l'ont juste griffe."));
    v.push(mk(11_524, "Bouclier du Chaos", K::Shield, 100, vec![f(S::Defense, 730), f(S::BlockChance, 36), f(S::Vitality, 100), f(S::FireResist, 50), f(S::ColdResist, 50), f(S::LightningResist, 50), f(S::PoisonResist, 50)], "Aucune lame ne se souvient l'avoir touche."));

    // ── AMULETS (12_000..) ─────────────────────────────────────────────────
    v.push(mk(12_000, "Cordon", K::Amulet, 5, vec![f(S::Vitality, 4), f(S::Life, 15)], "Un cordon avec une promesse."));
    v.push(mk(12_001, "Pierre du Loup", K::Amulet, 10, vec![f(S::Strength, 5), f(S::Life, 20), f(S::FireResist, 8)], "Un caillou d'un autel paien."));
    v.push(mk(12_002, "Talisman du Loup", K::Amulet, 15, vec![f(S::Vitality, 8), f(S::Life, 30), f(S::ColdResist, 10)], "Le loup t'attend dehors."));
    v.push(mk(12_003, "Talisman du Gardien", K::Amulet, 18, vec![f(S::Vitality, 10), f(S::Defense, 15), f(S::Life, 40)], "Pour ne pas oublier sa garde."));
    v.push(mk(12_004, "Talisman d'Aube", K::Amulet, 20, vec![f(S::Vitality, 12), f(S::Life, 45), f(S::FireResist, 15), f(S::ColdResist, 15)], "Recue a la fin d'une nuit."));
    v.push(mk(12_005, "Talisman d'Os", K::Amulet, 25, vec![f(S::Vitality, 15), f(S::Life, 55), f(S::PoisonResist, 18)], "Os perce en son centre."));
    v.push(mk(12_006, "Linceul-Charme", K::Amulet, 30, vec![f(S::Vitality, 18), f(S::Life, 70), f(S::ColdResist, 22)], "Un noeud du linceul d'un saint."));
    v.push(mk(12_007, "Amulette Spectre", K::Amulet, 35, vec![f(S::Energy, 18), f(S::Mana, 50), f(S::LightningResist, 22)], "Sonne quand on l'agite."));
    v.push(mk(12_008, "Amulette Sepulcre", K::Amulet, 38, vec![f(S::Vitality, 22), f(S::Life, 90), f(S::PoisonResist, 25)], "Sceau d'un caveau scelle."));
    v.push(mk(12_009, "Amulette du Lich", K::Amulet, 40, vec![f(S::Energy, 22), f(S::Mana, 70), f(S::ColdResist, 28), f(S::MagicFind, 15)], "Le lich y a laisse sa carte."));
    v.push(mk(12_010, "Croix d'Acier", K::Amulet, 45, vec![f(S::Strength, 18), f(S::Vitality, 22), f(S::Life, 100), f(S::FireResist, 22)], "Forgee au feu d'une cathedrale."));
    v.push(mk(12_011, "Talisman Croise", K::Amulet, 50, vec![f(S::Strength, 22), f(S::Vitality, 28), f(S::FireResist, 25), f(S::PoisonResist, 25)], "Pour ceux qui marchent loin."));
    v.push(mk(12_012, "Sceau du Lord", K::Amulet, 55, vec![f(S::Vitality, 32), f(S::Life, 130), f(S::FireResist, 28), f(S::ColdResist, 28)], "Sigle interdit aux paysans."));
    v.push(mk(12_013, "Amulette Royale", K::Amulet, 58, vec![f(S::Vitality, 38), f(S::Life, 150), f(S::FireResist, 30), f(S::ColdResist, 30), f(S::LightningResist, 30)], "Reservee aux invites du roi."));
    v.push(mk(12_014, "Amulette Sainte", K::Amulet, 60, vec![f(S::Vitality, 42), f(S::Life, 170), f(S::MagicFind, 25), f(S::FireResist, 32)], "Encore tiede au cou."));
    v.push(mk(12_015, "Amulette de Flamme", K::Amulet, 65, vec![f(S::Vitality, 48), f(S::FireResist, 45), f(S::Life, 200)], "Chauffee a meme la peau."));
    v.push(mk(12_016, "Amulette de Glace", K::Amulet, 70, vec![f(S::Vitality, 55), f(S::ColdResist, 45), f(S::Life, 220)], "Une larme prise dans l'ambre froid."));
    v.push(mk(12_017, "Amulette de Foudre", K::Amulet, 75, vec![f(S::Energy, 50), f(S::LightningResist, 45), f(S::Mana, 150)], "Picote a chaque heure pleine."));
    v.push(mk(12_018, "Amulette Drake", K::Amulet, 78, vec![f(S::Vitality, 60), f(S::FireResist, 35), f(S::ColdResist, 35), f(S::Life, 250)], "Crochet d'une dent de drake."));
    v.push(mk(12_019, "Amulette Wyrm", K::Amulet, 80, vec![f(S::Vitality, 65), f(S::FireResist, 38), f(S::ColdResist, 38), f(S::LightningResist, 38), f(S::PoisonResist, 38)], "Le wyrm la porte parfois aux nuits chaudes."));
    v.push(mk(12_020, "Talisman Divin", K::Amulet, 85, vec![f(S::Vitality, 75), f(S::Life, 300), f(S::FireResist, 40), f(S::ColdResist, 40), f(S::LightningResist, 40), f(S::PoisonResist, 40)], "Tout porte bonheur, ou rien."));
    v.push(mk(12_021, "Etoile-Amulette", K::Amulet, 90, vec![f(S::Energy, 60), f(S::Mana, 250), f(S::LifeOnHit, 25), f(S::MagicFind, 40)], "Tout l'etoile dans la paume."));
    v.push(mk(12_022, "Amulette Demon", K::Amulet, 95, vec![f(S::Strength, 60), f(S::Vitality, 80), f(S::FireResist, 50), f(S::Life, 350)], "Ne se dissout pas dans le feu."));
    v.push(mk(12_023, "Amulette Eternelle", K::Amulet, 98, vec![f(S::Vitality, 95), f(S::Life, 400), f(S::FireResist, 45), f(S::ColdResist, 45), f(S::LightningResist, 45), f(S::PoisonResist, 45)], "Chaque main qui l'a tenue revient."));
    v.push(mk(12_024, "Amulette du Chaos", K::Amulet, 100, vec![f(S::Vitality, 110), f(S::Life, 500), f(S::FireResist, 50), f(S::ColdResist, 50), f(S::LightningResist, 50), f(S::PoisonResist, 50), f(S::MagicFind, 60)], "Rien ne pend droit avec elle."));

    // ── RINGS (12_100..) ───────────────────────────────────────────────────
    v.push(mk(12_100, "Anneau de Bois", K::Ring, 5, vec![f(S::Strength, 3), f(S::Dexterity, 3), f(S::Life, 10)], "Marie d'enfance."));
    v.push(mk(12_101, "Anneau de Cuivre", K::Ring, 10, vec![f(S::Vitality, 5), f(S::Life, 18), f(S::AttackRating, 12)], "Verdit autour du doigt."));
    v.push(mk(12_102, "Anneau du Loup", K::Ring, 15, vec![f(S::Strength, 8), f(S::Life, 25), p(S::CritChance, 4)], "Hurle au creux de la paume."));
    v.push(mk(12_103, "Anneau d'Argent", K::Ring, 18, vec![f(S::Energy, 8), f(S::Mana, 20), f(S::ColdResist, 10)], "Sang noir sur metal blanc."));
    v.push(mk(12_104, "Anneau d'Or", K::Ring, 20, vec![f(S::Vitality, 10), f(S::Life, 35), f(S::GoldFind, 25), f(S::FireResist, 12)], "Promesse de mariage non tenue."));
    v.push(mk(12_105, "Anneau d'Os", K::Ring, 25, vec![f(S::Strength, 10), f(S::PoisonResist, 18), f(S::LifeOnHit, 5)], "Coupe dans l'os d'un saint."));
    v.push(mk(12_106, "Anneau d'Ombre", K::Ring, 30, vec![f(S::Dexterity, 14), p(S::CritChance, 6), f(S::Stamina, 30)], "Disparait sous la peau."));
    v.push(mk(12_107, "Anneau Spectre", K::Ring, 35, vec![f(S::Energy, 14), f(S::Mana, 40), p(S::CastSpeed, 6)], "Pres de toi sans etre vu."));
    v.push(mk(12_108, "Anneau Sepulcre", K::Ring, 38, vec![f(S::Vitality, 18), f(S::Life, 70), f(S::ColdResist, 20)], "Sceau cisele d'un tombeau."));
    v.push(mk(12_109, "Anneau du Lich", K::Ring, 40, vec![f(S::Energy, 18), f(S::Mana, 55), f(S::ColdResist, 25), p(S::CastSpeed, 8)], "Le lich te le fera payer."));
    v.push(mk(12_110, "Anneau d'Acier", K::Ring, 45, vec![f(S::Strength, 15), f(S::Vitality, 18), f(S::Life, 80), f(S::FireResist, 20)], "Forge entre deux marteaux."));
    v.push(mk(12_111, "Anneau du Chevalier", K::Ring, 50, vec![f(S::Strength, 20), f(S::Vitality, 25), f(S::Defense, 30), f(S::FireResist, 22)], "Honneur enferme dans un metal."));
    v.push(mk(12_112, "Anneau du Lord", K::Ring, 55, vec![f(S::Vitality, 28), f(S::Life, 100), f(S::FireResist, 25), f(S::ColdResist, 25)], "Le sceau d'un duche disparu."));
    v.push(mk(12_113, "Anneau du Roi", K::Ring, 58, vec![f(S::Vitality, 32), f(S::Life, 120), p(S::CritChance, 8), f(S::MagicFind, 25)], "Lettre du tresor royal."));
    v.push(mk(12_114, "Anneau Saint", K::Ring, 60, vec![f(S::Vitality, 38), f(S::Life, 140), f(S::FireResist, 28), f(S::ColdResist, 28), f(S::LightningResist, 28)], "Une priere s'y glisse."));
    v.push(mk(12_115, "Anneau de Flamme", K::Ring, 65, vec![f(S::FireResist, 38), f(S::Life, 160), p(S::AttackSpeed, 10)], "Brule a chaque pulsation."));
    v.push(mk(12_116, "Anneau de Givre", K::Ring, 70, vec![f(S::ColdResist, 38), f(S::Life, 180), p(S::CritChance, 12)], "Donne le frisson, longtemps apres."));
    v.push(mk(12_117, "Anneau de Foudre", K::Ring, 75, vec![f(S::LightningResist, 38), f(S::Mana, 130), p(S::CastSpeed, 12)], "Pique le doigt qui le porte."));
    v.push(mk(12_118, "Anneau Drake", K::Ring, 78, vec![f(S::Vitality, 50), f(S::FireResist, 30), f(S::ColdResist, 30), f(S::Life, 200)], "Ecaille polie en cercle."));
    v.push(mk(12_119, "Anneau Wyrm", K::Ring, 80, vec![f(S::FireResist, 32), f(S::ColdResist, 32), f(S::LightningResist, 32), f(S::PoisonResist, 32), f(S::Life, 220)], "Le wyrm en a sept au meme doigt."));
    v.push(mk(12_120, "Anneau Divin", K::Ring, 85, vec![f(S::Vitality, 60), f(S::Life, 250), f(S::FireResist, 38), f(S::ColdResist, 38), f(S::LightningResist, 38), f(S::PoisonResist, 38)], "Aucune doit dieu ne l'a moules."));
    v.push(mk(12_121, "Anneau Aile", K::Ring, 90, vec![f(S::Dexterity, 55), p(S::AttackSpeed, 15), p(S::MovementSpeed, 12), f(S::LifeOnHit, 22)], "Pousse une plume contre la peau."));
    v.push(mk(12_122, "Anneau Demon", K::Ring, 95, vec![f(S::Strength, 55), f(S::FireResist, 45), f(S::Life, 300), p(S::CritChance, 18)], "Un nom y est grave a l'envers."));
    v.push(mk(12_123, "Anneau Eternel", K::Ring, 98, vec![f(S::Vitality, 80), f(S::Life, 350), f(S::FireResist, 42), f(S::ColdResist, 42), f(S::LightningResist, 42), f(S::PoisonResist, 42)], "Tous les doigts deviennent eternel."));
    v.push(mk(12_124, "Anneau du Chaos", K::Ring, 100, vec![f(S::Vitality, 95), f(S::Life, 400), f(S::FireResist, 50), f(S::ColdResist, 50), f(S::LightningResist, 50), f(S::PoisonResist, 50), f(S::MagicFind, 50), f(S::GoldFind, 50)], "On ne le sort plus."));

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_has_425_entries() {
        assert_eq!(uniques_table().len(), 425);
    }

    #[test]
    fn ids_are_unique() {
        let t = uniques_table();
        let mut ids: Vec<u32> = t.iter().map(|u| u.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), t.len(), "duplicate ids in unique table");
    }

    #[test]
    fn every_unique_has_a_name_and_mods() {
        for u in uniques_table() {
            assert!(!u.name.trim().is_empty(), "empty name on id {}", u.id);
            assert!(!u.mods.is_empty(), "no mods on id {}", u.id);
        }
    }

    #[test]
    fn level_reqs_in_range() {
        for u in uniques_table() {
            assert!((1..=100).contains(&u.min_ilvl), "level_req out of range: id {}", u.id);
        }
    }
}
