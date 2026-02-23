# Allumina - Extraction Architecture UO : ServUO & ModernUO

> **Objectif** : Documenter l'architecture core des émulateurs Ultima Online (ServUO / ModernUO) pour extraction vers MGE (Allumina).
> **Sources** : Code source GitHub ServUO/ServUO (master) et modernuo/ModernUO (main).

---

## A) Hiérarchie des Entités

### Interface et Classe Racine

**Fichier** : `Server/IEntity.cs`

```csharp
public interface IEntity : IPoint3D, IComparable, IComparable<IEntity>
{
    Serial Serial { get; }
    Point3D Location { get; set; }
    Map Map { get; set; }
    bool NoMoveHS { get; set; }
    Direction Direction { get; set; }
    string Name { get; set; }
    int Hue { get; set; }
    bool Deleted { get; }
    void Delete();
    void ProcessDelta();
    void InvalidateProperties();
    void OnStatsQuery(Mobile m);
}

public class Entity : IEntity, IComparable<Entity>
{
    public Serial Serial { get; private set; }
    public Point3D Location { get; set; }
    public Map Map { get; set; }
    public bool Deleted { get; private set; }
    // ...
}
```

### Arbre d'héritage complet

```
IEntity (interface)
├── Entity (classe concrète minimale)
├── Mobile (Server/Mobile.cs ~12000 lignes)
│   ├── PlayerMobile (Scripts/Mobiles/PlayerMobile.cs)
│   └── BaseCreature (Scripts/Mobiles/BaseCreature.cs)
│       ├── BaseVendor
│       ├── BaseGuard
│       ├── BaseMount
│       ├── BaseEscortable
│       └── [centaines de créatures spécifiques]
├── Item (Server/Item.cs ~6000 lignes)
│   ├── BaseWeapon → BaseSword, BaseAxe, BasePoleArm, BaseRanged...
│   ├── BaseArmor → BaseShield, PlateChest, LeatherLegs...
│   ├── BaseClothing
│   ├── BaseJewel
│   ├── Container → Backpack, BankBox, Chest...
│   ├── BaseAddon / AddonComponent
│   ├── Gold, BankCheck
│   ├── SpellScroll
│   └── BaseMulti (héritage via Item)
│       ├── BaseHouse → SmallHouse, LargeHouse, Castle, Keep...
│       ├── BaseBoat → BaseGalleon, TokunoGalleon...
│       └── StaticHouse
└── BaseGuild (Server/Guilds/)
```

### Propriétés clés de Mobile

- `Skills` (collection de `Skill`, cap total configurable, défaut 7000 = 700.0)
- `RawStr`, `RawDex`, `RawInt` (stats brutes, cap par stat + cap total `StatCap`)
- `Hits`, `Stam`, `Mana` (pools dérivées des stats)
- `Backpack` (Container inventaire)
- `Direction`, `Location`, `Map`
- `NetState` (connexion réseau associée)
- `SkillLock` par skill (Up / Down / Locked)
- `StatLockType` par stat (Up / Down / Locked)

### Propriétés clés de Item

- `Serial` (identifiant unique persisté)
- `Parent` (Mobile ou Container)
- `Amount`, `Stackable`
- `Weight`, `Hue`, `ItemID`
- `Layer` (emplacement d'équipement)
- `LootType` (Regular, Newbied, Blessed, Cursed)

### Différences ServUO vs ModernUO

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Langage | C# ~6 / .NET Framework | C# 14 / .NET 10 |
| Entity base | `IEntity` → `Entity`, `Mobile`, `Item` | Même hiérarchie, + attributs source generator |
| Sérialisation | Manuelle `Serialize(GenericWriter)` | Auto via `[SerializableField]` + Source Generator |
| Collections | `Dictionary<Serial, Mobile/Item>` | Identique mais avec `GenericEntityPersistence<T>` |

---

## B) Système de Skills

### Fichier source : `Scripts/Misc/SkillCheck.cs`

### Formule de Gain (GetGainChance)

```csharp
private static double GetGainChance(Mobile from, Skill skill, double chance, bool success)
{
    // Étape 1 : Ratio d'espace restant dans le skill total cap
    double gc = (double)(from.Skills.Cap - from.Skills.Total) / from.Skills.Cap;

    // Étape 2 : Ratio d'espace restant dans le cap individuel du skill
    gc += (skill.Cap - skill.Base) / skill.Cap;
    gc /= 2;

    // Étape 3 : Bonus de difficulté (tâches plus difficiles = gain plus facile)
    gc += (1.0 - chance) * (success ? 0.5 : 0.0);
    gc /= 2;

    // Étape 4 : Facteur de gain spécifique au skill
    gc *= skill.Info.GainFactor;

    // Plancher minimum
    if (gc < 0.01)
        gc = 0.01;

    // Bonus 100% pour les pets contrôlés
    if (from is BaseCreature && ((BaseCreature)from).Controlled)
        gc += gc * 1.00;

    // Plafond maximum
    if (gc > 1.00)
        gc = 1.00;

    return gc;
}
```

### Décomposition de la formule

Soit :
- `T` = Skills.Total (somme de tous les skills × 10)
- `C` = Skills.Cap (cap total, défaut 7000)
- `B` = skill.Base (valeur actuelle du skill)
- `K` = skill.Cap (cap individuel, défaut 1000 = 100.0)
- `p` = chance (probabilité de succès de l'action)
- `s` = 1 si succès, 0 si échec
- `G` = GainFactor du skill

```
gc = [ (C - T) / C + (K - B) / K ] / 2
gc = [ gc + (1 - p) × (s × 0.5) ] / 2
gc = gc × G
gc = clamp(gc, 0.01, 1.00)
```

### SkillCheck principal

```csharp
public static bool CheckSkill(Mobile from, Skill skill, object obj, double chance)
{
    bool success = Utility.Random(100) <= (int)(chance * 100);
    double gc = GetGainChance(from, skill, chance, success);

    if (AllowGain(from, skill, obj))
    {
        if (from.Alive && (skill.Base < 10.0 || Utility.RandomDouble() <= gc || CheckGGS(from, skill)))
            Gain(from, skill);
    }
    return success;
}
```

**Logique** :
1. Calcul succès/échec de l'action (skill check proprement dit)
2. Calcul de la probabilité de gain
3. Si `skill.Base < 10.0` → gain automatique garanti
4. Sinon → jet aléatoire contre `gc`
5. Fallback GGS (Guaranteed Gain System) pour éviter le blocage

### Système Anti-Macro

- Tableau `UseAntiMacro[58]` : true/false par skill
- `AntiMacroExpire` = 5 minutes
- `Allowance` = 3 utilisations du même target/location
- `LocationSize` = 4 (grille de zones)
- Skills de combat (Tactics, Swords, etc.) = anti-macro **désactivé**
- Skills d'interaction (Anatomy, Healing, etc.) = anti-macro **activé**

### Guaranteed Gain System (GGS)

```csharp
// Cooldown basé sur skill level et total
private static readonly int[][] GGSTable = {
    new[] {1, 3, 5},      // 0.0 - 4.9
    new[] {4, 10, 18},     // 5.0 - 9.9
    // ... progression exponentielle ...
    new[] {618, 1662, 3060} // 115.0 - 119.9
};
// Colonnes: [<3500 total, 3500-6999, ≥7000] en minutes
```

### Gain de Stats

- Chance configurable : `PlayerChanceToGainStats` = 5%
- Délai minimum : `StatGainDelay` = 15 minutes entre gains
- Chaque skill a un `Primary` et `Secondary` stat
- 75% chance Primary, 25% Secondary quand les deux sont "Up"
- Auto-baisse d'une stat "Down" quand on atteint `StatCap`

### Caps standards UO

| Cap | Valeur |
|-----|--------|
| Skill individuel | 100.0 (120.0 avec Power Scroll) |
| Skill total | 700.0 (720.0 avec Stat Scroll) |
| Stat individuelle | 125 |
| Stat total | 225 |

---

## C) Système de Housing

### Fichier source : `Scripts/Multis/Houses/BaseHouse.cs` (~4900 lignes)

### Structure de classe

```csharp
public abstract class BaseHouse : BaseMulti
{
    public static int AccountHouseLimit { get; } = Config.Get("Housing.AccountHouseLimit", 1);
    public static double GlobalBonusStorageScalar => 1.4;
    public const int MaxCoOwners = 15;
    public static int MaxFriends => 140;
    public static int MaxBans => 140;
    
    // Propriétaire
    private Mobile m_Owner;
    
    // Listes d'accès
    private List<Mobile> m_CoOwners;
    private List<Mobile> m_Friends;
    private List<Mobile> m_Bans;
    private List<Mobile> m_Access;
    
    // Stockage
    private List<Item> m_LockDowns;      // Items verrouillés au sol
    private List<Item> m_Secures;         // Containers sécurisés
    private List<Item> m_Addons;          // Addons installés
    
    // Decay
    public DateTime LastRefreshed { get; set; }
    public bool RestrictDecay { get; set; }
    public virtual TimeSpan DecayPeriod => TimeSpan.FromDays(5.0);
}
```

### Héritage Housing

```
BaseMulti (hérite de Item)
└── BaseHouse (abstract)
    ├── HouseFoundation (maisons customisables)
    ├── SmallOldHouse, SmallShop, ...
    ├── TwoStoryHouse, Tower, ...
    ├── Castle, Keep
    └── StaticHouse
```

### Système de Decay

```csharp
public enum DecayType { Ageless, AutoRefresh, ManualRefresh, Condemned }
public enum DecayLevel { Ageless, LikeNew, Slightly, Somewhat, Fairly, Greatly, IDOC }
```

**Logique de decay** :
1. `DecayPeriod` = 5 jours entre chaque stade
2. GM+ accounts → `Ageless` automatique
3. Comptes inactifs → `Condemned` immédiat
4. Maison la plus récente d'un compte → `AutoRefresh` (pas de decay)
5. Autres maisons → `ManualRefresh` (refresh par visite du owner)
6. Dynamic Decay : stades progressifs avec durées aléatoires via `DynamicDecay`

**Stades** : `LikeNew` → `Slightly` → `Somewhat` → `Fairly` → `Greatly` → **IDOC** (In Danger Of Collapsing)

### Placement Multi-Tile

- `BaseMulti` hérite de `Item` avec un système de composants multi-tiles
- Les maisons occupent une grille de tiles définie par un fichier `.mul`
- Vérification de placement : terrain plat, pas de chevauchement, zones autorisées
- `HouseFoundation` permet la personnalisation via un éditeur

### Système de Lockdown/Secure

- **Lockdown** : Item posé au sol dans la maison, verrouillé en place
  - Ne peut être déplacé que par owner/co-owner
  - Ne decay pas avec la maison (récupéré à la destruction)
  - Limité par `MaxLockDowns` (dépend de la taille de la maison)

- **Secure** : Container transformé en coffre sécurisé
  - Access configurable : Owner, Co-Owners, Friends, Anyone
  - Limité par `MaxSecures`
  - Formule : `MaxSecures = BaseStorage * GlobalBonusStorageScalar`

### Contrôle d'accès

| Niveau | Droits |
|--------|--------|
| Owner | Tout (placer, détruire, lockdown, secure, access lists) |
| Co-Owner | Lockdown, secure, accès complet, gestion friends |
| Friend | Ouvrir portes, utiliser objets, téléporteurs |
| Access | Entrer dans la maison (temporaire) |
| Banned | Interdit d'entrer, éjecté automatiquement |

---

## D) Système de Timer

### ServUO : Timer Thread + Priority Buckets

**Fichier** : `Server/Timer.cs`

#### Architecture

```csharp
public enum TimerPriority
{
    EveryTick,       // 0ms  - chaque tick
    TenMS,           // 10ms
    TwentyFiveMS,    // 25ms
    FiftyMS,         // 50ms
    TwoFiftyMS,      // 250ms
    OneSecond,       // 1000ms
    FiveSeconds,     // 5000ms
    OneMinute        // 60000ms
}
```

**Modèle** :
- **Thread dédié** (`TimerThread.TimerMain()`) qui tourne en boucle
- 8 listes de timers, une par priorité
- Le thread évalue chaque bucket selon son intervalle de priorité
- Les timers prêts sont mis en **queue** (`m_Queue`)
- Le thread principal (`Slice()`) dépile et exécute `OnTick()` sur le game thread
- **Synchronisation** : `lock(m_Changed)` pour add/remove, `lock(m_Queue)` pour l'exécution
- `AutoResetEvent m_Signal` pour wake-up du thread timer

**Sélection automatique de priorité** :
```csharp
public static TimerPriority ComputePriority(TimeSpan ts)
{
    if (ts >= 10min)  return OneMinute;
    if (ts >= 30s)    return FiveSeconds;
    if (ts >= 10s)    return OneSecond;
    if (ts >= 5s)     return TwoFiftyMS;
    if (ts >= 2.5s)   return FiftyMS;
    if (ts >= 1s)     return TwentyFiveMS;
    if (ts >= 0.5s)   return TenMS;
    return EveryTick;
}
```

**API principale** :
```csharp
Timer.DelayCall(TimeSpan delay, TimerCallback callback);
Timer.DelayCall(TimeSpan delay, TimeSpan interval, int count, TimerCallback callback);
timer.Start();
timer.Stop();
protected virtual void OnTick();
```

### ModernUO : Timer Wheel (Hierarchical)

**Fichiers** : `Projects/Server/Timer/Timer.cs` + `Timer.TimerWheel.cs`

#### Architecture Timer Wheel

```csharp
// Constantes de la roue
const int _ringSizePowerOf2 = 12;           // 4096 slots par anneau
const int _ringSize = 1 << 12;              // = 4096
const int _ringLayers = 3;                  // 3 anneaux hiérarchiques
const int _tickRatePowerOf2 = 3;            // résolution = 8ms
const int _tickRate = 1 << 3;               // = 8ms
const long _maxDuration = tickRate << (12*3 - 1); // ~137 milliards ms
```

**Structure** :
- 3 anneaux hiérarchiques de 4096 slots chacun
- Résolution de base : **8ms**
- Anneau 0 : 8ms × 4096 = ~33 secondes max
- Anneau 1 : ~33s × 4096 = ~37 heures max
- Anneau 2 : ~37h × 4096 = ~17 ans max
- **Linked list doublement chaînée** dans chaque slot (pas de `List<Timer>`)
- **Pas de thread séparé** : `Slice(tickCount)` appelé sur le game loop
- **Pas de locks** : single-threaded, toutes opérations sur le game thread

**Insertion** (`AddTimer`) :
```csharp
private static void AddTimer(Timer timer, long delay)
{
    // Trouve l'anneau approprié basé sur la durée
    for (var i = 0; i < _ringLayers; i++)
    {
        var resolution = 1L << resolutionPowerOf2;
        var max = 1L << nextResolutionPowerOf2;
        
        if (delay < max || lastRing)
        {
            // Calcul du slot dans l'anneau
            var slot = (delay >> resolutionPowerOf2) + ringIndex;
            // Insertion en tête de la linked list
            timer.Attach(_rings[i][slot]);
            _rings[i][slot] = timer;
            return;
        }
        delay -= offsetDelay;
    }
}
```

**Exécution** (`Turn`) :
1. Avancer l'index de chaque anneau
2. Quand un anneau fait un tour complet → cascade vers l'anneau supérieur
3. Détacher la chaîne du slot courant
4. Pour chaque timer de la chaîne :
   - Si dans anneau > 0 et `remaining > 0` → **promote** vers anneau inférieur
   - Sinon → `Execute(timer)` → appelle `OnTick()`

### Comparaison Timer

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Structure | 8 listes + thread dédié | 3 anneaux × 4096 slots, single-thread |
| Résolution | Variable (0ms à 60s par bucket) | 8ms uniforme |
| Synchronisation | `lock` + `AutoResetEvent` | Aucun lock (single-threaded) |
| Insertion | O(1) add to list | O(1) linked list head insert |
| Évaluation | Scan linéaire de chaque bucket | O(1) avancement d'index |
| Memory | `List<Timer>` + allocation entries | Linked list intrusive (pas d'alloc) |
| TimerPriority | 8 niveaux explicites | **Supprimé** - inutile |
| Précision | Dépend du bucket (10ms-60s) | 8ms constant |

---

## E) Système de Crafting

### Fichiers sources
- `Scripts/Services/Craft/Core/CraftSystem.cs`
- `Scripts/Services/Craft/Core/CraftItem.cs`
- `Scripts/Services/Craft/Def*.cs` (un par métier)

### Architecture CraftSystem

```csharp
public abstract class CraftSystem
{
    // Propriétés du système de craft
    public double Delay { get; }              // Délai animation
    public CraftItemCol CraftItems { get; }   // Collection de recettes
    public CraftGroupCol CraftGroups { get; } // Groupes UI
    public CraftSubResCol CraftSubRes { get; }  // Sous-ressources (types de métal, bois...)
    public CraftSubResCol CraftSubRes2 { get; } // Sous-ressources secondaires
    
    public bool CanEnhance { get; set; }
    public bool Repair { get; set; }
    public bool MarkOption { get; set; }
    
    // Abstraits - chaque métier implémente
    public abstract SkillName MainSkill { get; }
    public abstract double GetChanceAtMin(CraftItem item);
    public abstract void InitCraftList();
}
```

### Systèmes de craft concrets

| Classe | Skill | Fichier |
|--------|-------|---------|
| `DefAlchemy` | Alchemy | DefAlchemy.cs |
| `DefBlacksmithy` | Blacksmithy | DefBlacksmithy.cs |
| `DefBowFletching` | Fletching | DefBowFletching.cs |
| `DefCarpentry` | Carpentry | DefCarpentry.cs |
| `DefCartography` | Cartography | DefCartography.cs |
| `DefCooking` | Cooking | DefCooking.cs |
| `DefGlassblowing` | Alchemy | DefGlassblowing.cs |
| `DefInscription` | Inscribe | DefInscription.cs |
| `DefMasonry` | Carpentry | DefMasonry.cs |
| `DefTailoring` | Tailoring | DefTailoring.cs |
| `DefTinkering` | Tinkering | DefTinkering.cs |

### CraftItem - Définition d'une recette

```csharp
public class CraftItem
{
    public Type ItemType { get; }
    public CraftResCol Resources { get; }     // Ressources requises
    public CraftSkillCol Skills { get; }      // Skills requis (min/max)
    
    public bool NeedHeat { get; set; }        // Nécessite forge/feu
    public bool NeedOven { get; set; }        // Nécessite four
    public bool NeedWater { get; set; }       // Nécessite eau
    public bool NeedMill { get; set; }        // Nécessite moulin
    
    public bool UseAllRes { get; set; }       // Craft multi-unités
    public bool ForceNonExceptional { get; set; }
    public bool ForceExceptional { get; set; }
    
    public Recipe Recipe { get; set; }        // Recette à découvrir
    public int Mana { get; set; }             // Coût en mana
    public int Hits { get; set; }             // Coût en HP
    public int Stam { get; set; }             // Coût en stamina
}
```

### Enregistrement d'une recette (exemple)

```csharp
// Dans DefBlacksmithy.InitCraftList()
int index = AddCraft(
    typeof(Katana),                    // Type de l'item produit
    "Bladed Weapons",                  // Groupe
    "Katana",                          // Nom
    SkillName.Blacksmith,             // Skill requis
    44.1,                              // MinSkill
    94.1,                              // MaxSkill
    typeof(IronIngot),                 // Type de ressource
    "Iron Ingot",                      // Nom ressource
    8                                  // Quantité
);
SetNeedHeat(index, true);            // Nécessite une forge
```

### Formule de Succès (GetSuccessChance)

```csharp
// Interpolation linéaire entre min et max skill
double chance = craftSystem.GetChanceAtMin(this)
    + ((valMainSkill - minMainSkill) / (maxMainSkill - minMainSkill)
       * (1.0 - craftSystem.GetChanceAtMin(this)));
```

**Variables** :
- `GetChanceAtMin(item)` = chance au skill minimum (typiquement 0.0 ou 0.5)
- `valMainSkill` = skill actuel du joueur
- `minMainSkill` = skill minimum de la recette
- `maxMainSkill` = skill maximum de la recette

**Exemple** : Katana (min=44.1, max=94.1), joueur à 100 Blacksmith, ChanceAtMin=0.5
```
chance = 0.5 + ((100 - 44.1) / (94.1 - 44.1)) * (1.0 - 0.5)
       = 0.5 + (55.9 / 50.0) * 0.5
       = 0.5 + 0.559
       = 1.059 → clampé à 1.0 (100%)
```

### Formule Exceptional (GetExceptionalChance)

```csharp
public double GetExceptionalChance(CraftSystem system, double chance, Mobile from)
{
    double bonus = 0.0;
    // Bonus talisman, tablier de chef, établi...
    
    switch (system.ECA)
    {
        case CraftECA.ChanceMinusSixty:
            chance -= 0.6;  // Standard : chance - 60%
            break;
        case CraftECA.FiftyPercentChanceMinusTenPercent:
            chance = chance * 0.5 - 0.1;  // Cooking
            break;
        case CraftECA.ChanceMinusSixtyToFourtyFive:
            // Scale de 60% à 45% entre skill 95 et 100
            double offset = 0.60 - ((skill - 95.0) * 0.03);
            offset = clamp(offset, 0.45, 0.60);
            chance -= offset;
            break;
    }
    return chance + bonus;
}
```

**En pratique (standard)** :
- Skill 100, recette max 94.1 → success chance = 100%
- Exceptional = 100% - 60% = **40%** de base

### Interface ICraftable

```csharp
public interface ICraftable
{
    int OnCraft(int quality, bool makersMark, Mobile from,
                CraftSystem craftSystem, Type typeRes, ITool tool,
                CraftItem craftItem, int resHue);
}
```

Les items implémentent `ICraftable` pour recevoir les bonus de qualité, matériaux spéciaux, etc.

### Quality

| Qualité | Valeur | Signification |
|---------|--------|---------------|
| 1 | Normal | Craft standard |
| 2 | Exceptional | Bonus stats, peut être marqué |

---

## F) Persistance / World Save

### ServUO : Fichiers binaires indexés

**Fichier** : `Server/World.cs`

#### Format de sauvegarde

```
Saves/
├── Mobiles/
│   ├── Mobiles.idx    # Index : [typeID, serial, position, length]
│   ├── Mobiles.tdb    # Type database : [count, typeName...]
│   └── Mobiles.bin    # Données binaires sérialisées
├── Items/
│   ├── Items.idx
│   ├── Items.tdb
│   └── Items.bin
└── Guilds/
    ├── Guilds.idx
    └── Guilds.bin
```

#### Sérialisation manuelle

```csharp
// Chaque classe implémente :
public override void Serialize(GenericWriter writer)
{
    base.Serialize(writer);
    writer.Write(0); // version
    writer.Write(m_Name);
    writer.Write(m_HitPoints);
}

public override void Deserialize(GenericReader reader)
{
    base.Deserialize(reader);
    int version = reader.ReadInt();
    m_Name = reader.ReadString();
    m_HitPoints = reader.ReadInt();
}
```

#### Versioning avec fallthrough

```csharp
switch (version)
{
    case 2: someNewField = reader.ReadBool(); goto case 1;
    case 1: someField = reader.ReadInt(); goto case 0;
    case 0: baseFeld = reader.ReadString(); break;
}
```

#### Process de sauvegarde ServUO

1. `World.Save()` → set `Saving = true`
2. Sérialise tous les Mobiles dans `Mobiles.bin` avec index
3. Sérialise tous les Items dans `Items.bin` avec index
4. Sérialise les Guilds
5. Écrit les fichiers index `.idx` et types `.tdb`
6. `ManualResetEvent` pour la synchro écriture disque
7. **Single-threaded** pour la sérialisation, l'écriture disque peut être asynchrone

#### Chargement

1. Lit `.tdb` → résout les types via réflexion (`ConstructorInfo`)
2. Lit `.idx` → crée les objets vides via `ctor.Invoke(new object[]{ serial })`
3. Lit `.bin` → désérialise chaque objet à sa position dans le fichier

### ModernUO : Sérialisation parallèle + Source Generator

**Fichiers** : `Projects/Server/World/World.cs`, `Projects/Server/Serialization/GenericPersistence.cs`

#### Architecture de sauvegarde

```csharp
public static class World
{
    public static bool UseMultiThreadedSaves { get; private set; } // défaut: true
    
    internal static SerializationThreadWorker[] _threadWorkers;
    // Nombre de threads = max(ProcessorCount - 1, 1)
}
```

#### Pipeline de sauvegarde ModernUO

```
WorldState.Running
    → Save()
        → WorldState.PendingSave
        → Preserialize() [ThreadPool]
            → AllocateHeap() pour chaque worker
            → WakeSerializationThreads()
        → Snapshot() [Game Thread]
            → WorldState.Saving
            → NetState.FlushAll()
            → Persistence.SerializeAll()  ← round-robin distribution
            → PauseSerializationThreads()
        → WriteFiles() [ThreadPool]
            → WorldState.WritingSave
            → WriteSnapshot pour chaque persistence
            → MoveDirectoryContents(temp → Saves)
        → FinishWorldSave()
            → WorldState.Running
            → PostWorldSaveAll()
```

#### Sérialisation automatique (Source Generator)

```csharp
// ModernUO - déclaration avec attributs
[SerializationGenerator(0)]
public partial class ExampleItem : Item
{
    [SerializableField(0)]
    private string _name;
    
    [SerializableField(1)]
    private int _hitPoints;
}
// Le Source Generator crée automatiquement Serialize/Deserialize
// + fichiers de migration (ExampleItem.v0.json)
```

#### Distribution round-robin

```csharp
internal static void PushToCache(IGenericSerializable e)
{
    _threadWorkers[_threadId++].Push(e);
    if (_threadId == _threadWorkers.Length)
        _threadId = 0;
}
```

### Comparaison Persistance

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Format | .idx + .tdb + .bin | .bin par système, snapshot path |
| Sérialisation | Manuelle (Serialize/Deserialize) | Auto (Source Generator) + manuelle |
| Threading save | Single-threaded | Multi-threaded (round-robin workers) |
| Écriture disque | Directe (asynchrone possible) | Snapshot → temp → move atomique |
| Migration | Version switch/goto | Fichiers JSON de migration auto |
| Mémoire | Stream directement | Memory-mapped files + heap workers |
| Reader/Writer | `GenericWriter`/`GenericReader` (concrets) | `IGenericWriter`/`IGenericReader` (interfaces) |
| Safety | Queue add/delete pendant save | WorldState machine + PostDeserialize |

---

## G) Networking

### ServUO : Modèle Packet classique

**Fichier** : `Server/Network/PacketHandlers.cs`

#### Enregistrement des handlers

```csharp
static PacketHandlers()
{
    m_Handlers = new PacketHandler[0x100]; // 256 packet IDs
    
    Register(0x00, 104, false, CreateCharacter);  // Création perso
    Register(0x02, 7,   true,  MovementReq);      // Déplacement
    Register(0x03, 0,   true,  AsciiSpeech);      // Parole
    Register(0x05, 5,   true,  AttackReq);         // Attaque
    Register(0x06, 5,   true,  UseReq);            // Double-click
    Register(0x07, 7,   true,  LiftReq);           // Pick up item
    Register(0x08, 15,  true,  DropReq);           // Drop item
    Register(0x34, 10,  true,  MobileQuery);       // Stats query
    Register(0x6C, 19,  true,  TargetResponse);    // Target
    Register(0x72, 5,   true,  SetWarMode);        // War mode
    Register(0x73, 2,   false, PingReq);           // Ping
    Register(0x80, 62,  false, AccountLogin);      // Login compte
    Register(0x91, 65,  false, GameLogin);         // Login jeu
    Register(0xAD, 0,   true,  UnicodeSpeech);     // Parole Unicode
    Register(0xBF, 0,   true,  ExtendedCommand);   // Commandes étendues
    // ...
}
```

**Signature** : `Register(packetID, length, ingame, handler)`
- `packetID` : octet identifiant le packet
- `length` : taille fixe (0 = variable)
- `ingame` : true si nécessite un Mobile connecté
- `handler` : méthode statique de traitement

#### Structure de connexion

```
Client UO → TCP → NetState → PacketReader → PacketHandler → Game Logic
                                                          ← Packet (sortant)
                                                          ← NetState.Send()
```

- `NetState` : représente une connexion client unique
  - `Mobile` : le personnage associé
  - `Account` : le compte
  - `Send(Packet)` : envoie un packet au client
  - Cycle de vie : Connect → AccountLogin → GameLogin → Play → Disconnect

#### Envoi de packets (sortants)

```csharp
// Pattern ServUO : objets Packet avec Acquire/Release
Packet p = new UnicodeMessage(serial, body, type, hue, font, lang, name, text);
p.Acquire();
foreach (var ns in NetState.Instances)
    ns.Send(p);
p.Release();
NetState.FlushAll();
```

#### Packets étendus et encodés

```csharp
// 3 niveaux de dispatch :
m_Handlers[0x100]                    // Handlers standard (0x00-0xFF)
m_ExtendedHandlersLow[0x100]        // Handlers étendus bas (packet 0xBF)
m_ExtendedHandlersHigh              // Handlers étendus hauts (Dictionary)
m_EncodedHandlersLow[0x100]         // Handlers encodés bas
m_EncodedHandlersHigh               // Handlers encodés hauts (Dictionary)
```

### ModernUO : Pipeline moderne

#### Différences architecturales

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Socket I/O | Threads .NET classiques | `epoll`/`wepoll`/`kqueue` (PollGroup) |
| Packets sortants | Classes `Packet` heap-allocated | `stackalloc byte[]` sur la stack |
| Format sortant | `Packet.Compile()` → byte[] | `OutgoingMessagePackets.CreateMessage(Span<byte>)` |
| Envoi | `NetState.Send(Packet)` | `ns.Send(ReadOnlySpan<byte>)` |
| Flush | `NetState.FlushAll()` | `NetState.FlushAll()` (zero-copy) |
| Reader | `PacketReader` (classe) | `SpanReader` (ref struct, zero-alloc) |

```csharp
// ModernUO - envoi zero-allocation
var length = OutgoingMessagePackets.GetMaxMessageLength(text);
var buffer = stackalloc byte[length].InitializePacket();
length = OutgoingMessagePackets.CreateMessage(buffer, ...);
ns.Send(buffer[..length]);
```

#### Login Flow (identique conceptuellement)

```
1. Client → 0x80 AccountLogin (nom, mot de passe, clé)
2. Server → Server List
3. Client → 0x5D PlayCharacter (sélection serveur)
4. Client → 0x91 GameLogin (auth key, nom, mot de passe)
5. Server → Character List
6. Client → 0x5D PlayCharacter (sélection personnage)
7. Server → Login Confirm, Map, Mobile Update, Skills, etc.
```

#### Autorité serveur

Le serveur est **100% autoritaire** :
- Tous les mouvements sont validés côté serveur (speed checks, pathfinding)
- Les actions de combat sont calculées côté serveur (damage, hit chance)
- L'inventaire est géré côté serveur (anti-dupe : Serial unique + validation)
- Le client n'est qu'un **afficheur** : il envoie des intentions, reçoit des résultats
- Packets comme `Resynchronize` (0x22) permettent au client de re-syncer en cas de désync

---

## Résumé des fichiers clés

### ServUO (`github.com/ServUO/ServUO`)

| Fichier | Contenu |
|---------|---------|
| `Server/IEntity.cs` | Interface et classe Entity racine |
| `Server/Mobile.cs` | Classe Mobile (~12000 lignes) |
| `Server/Item.cs` | Classe Item (~6000 lignes) |
| `Server/Timer.cs` | Système Timer thread-based |
| `Server/World.cs` | Persistence / World Save |
| `Server/Network/PacketHandlers.cs` | Handlers de packets réseau |
| `Scripts/Misc/SkillCheck.cs` | Système de skills et formules de gain |
| `Scripts/Multis/Houses/BaseHouse.cs` | Système de housing |
| `Scripts/Services/Craft/Core/CraftSystem.cs` | Base du crafting |
| `Scripts/Services/Craft/Core/CraftItem.cs` | Logique de craft d'un item |
| `Scripts/Services/Craft/Def*.cs` | Définitions par métier |

### ModernUO (`github.com/modernuo/ModernUO`)

| Fichier | Contenu |
|---------|---------|
| `Projects/Server/Timer/Timer.cs` | Timer core (wheel-based) |
| `Projects/Server/Timer/Timer.TimerWheel.cs` | Implémentation Timer Wheel |
| `Projects/Server/World/World.cs` | Persistence multi-threaded |
| `Projects/Server/Serialization/GenericPersistence.cs` | Persistence générique |
| `Projects/Server/Network/` | Networking zero-alloc |
| `Projects/UOContent/` | Contenu gameplay (miroir Scripts/) |
