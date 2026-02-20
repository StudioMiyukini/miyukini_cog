# Scaffold des 99 crates des 16 genre packs - MGE AI-Native
$ErrorActionPreference = "Stop"
$root = "c:\Users\miyuk\Documents\Cursor\Miyukini_COG\mge"
$cratesRoot = "$root\crates"

# Structure: @( @{dir, prefix, deps}, @(modules) )
$packs = @(
    @{
        dir = "rpg"
        prefix = "rpg"
        domain = "rpg"
        deps = @("mge-ecs", "mge-event")
        modules = @("combat", "stats", "inventory", "quest", "progression", "dialogue", "ai")
    },
    @{
        dir = "massive-battle"
        prefix = "mb"
        domain = "massive_battle"
        deps = @("mge-ecs", "mge-event")
        modules = @("formation", "unit", "morale", "tactics", "supply", "siege")
    },
    @{
        dir = "social"
        prefix = "social"
        domain = "social"
        deps = @("mge-ecs", "mge-event")
        modules = @("relationship", "faction", "reputation", "need", "schedule", "personality", "gossip", "household")
    },
    @{
        dir = "rts"
        prefix = "rts"
        domain = "rts"
        deps = @("mge-ecs", "mge-event")
        modules = @("selection", "production", "resource", "building", "unit-ai", "minimap", "fog-of-war", "tech")
    },
    @{
        dir = "grand-strategy"
        prefix = "gs"
        domain = "grand_strategy"
        deps = @("mge-ecs", "mge-event")
        modules = @("diplomacy", "economy", "trade", "military", "population", "religion", "culture", "province", "decision", "cb")
    },
    @{
        dir = "puzzle"
        prefix = "puzzle"
        domain = "puzzle"
        deps = @("mge-ecs", "mge-event")
        modules = @("block", "match", "swap", "combo", "timer", "hint", "board", "tile", "goal")
    },
    @{
        dir = "sandbox"
        prefix = "sb"
        domain = "sandbox"
        deps = @("mge-ecs", "mge-event")
        modules = @("agent", "need", "world", "crafting", "building", "terrain", "wildlife", "season", "weather")
    },
    @{
        dir = "platformer"
        prefix = "pl"
        domain = "platformer"
        deps = @("mge-ecs", "mge-event")
        modules = @("movement", "jump", "collision", "camera", "checkpoint", "hazard")
    },
    @{
        dir = "shooter"
        prefix = "sh"
        domain = "shooter"
        deps = @("mge-ecs", "mge-event")
        modules = @("weapon", "aim", "ammo", "target", "health")
    },
    @{
        dir = "roguelike"
        prefix = "rl"
        domain = "roguelike"
        deps = @("mge-ecs", "mge-event")
        modules = @("procgen", "item", "permadeath", "floor")
    },
    @{
        dir = "racing"
        prefix = "race"
        domain = "racing"
        deps = @("mge-ecs", "mge-event")
        modules = @("vehicle", "track", "lap", "ai-driver")
    },
    @{
        dir = "factory"
        prefix = "factory"
        domain = "factory"
        deps = @("mge-ecs", "mge-event")
        modules = @("conveyor", "machine", "recipe", "logistics")
    },
    @{
        dir = "idle"
        prefix = "idle"
        domain = "idle"
        deps = @("mge-ecs", "mge-event")
        modules = @("prestige", "multiplier", "upgrade", "offline", "producer")
    },
    @{
        dir = "tycoon"
        prefix = "tycoon"
        domain = "tycoon"
        deps = @("mge-ecs", "mge-event")
        modules = @("customer", "employee", "facility", "revenue")
    },
    @{
        dir = "visual-novel"
        prefix = "vn"
        domain = "visual_novel"
        deps = @("mge-ecs", "mge-event")
        modules = @("choice", "script", "character", "save", "scene", "branch")
    },
    @{
        dir = "tcg"
        prefix = "tcg"
        domain = "tcg"
        deps = @("mge-ecs", "mge-event")
        modules = @("deck", "hand", "card", "battle", "mana")
    }
)

function Get-DepPath($dep, $packDir) {
    if ($packDir -match "^[^/]+$") {
        return "../" * 2 + $dep
    }
    return "../" * (2 + ($packDir -split "/").Count) + $dep
}

# Path depuis pack vers root crates: crates/rpg/ => ../.., crates/rpg/mge-rpg-x => ../..
$relativeToCrates = "../.."

$allMembers = @()
foreach ($pack in $packs) {
    $packDir = "$cratesRoot\$($pack.dir)"
    if (-not (Test-Path $packDir)) {
        New-Item -ItemType Directory -Path $packDir -Force | Out-Null
    }
    foreach ($mod in $pack.modules) {
        $crateName = "mge-$($pack.prefix)-$mod"
        $crateDir = "$packDir\$crateName"
        $srcDir = "$crateDir\src"
        
        New-Item -ItemType Directory -Path $srcDir -Force | Out-Null
        
        $depLines = ($pack.deps | ForEach-Object { "$_ = { path = `"$relativeToCrates/$_`" }" }) -join "`n"
        $cargoToml = @"
[package]
name = "$crateName"
version = "0.1.0"
edition = "2021"
description = "MGE $($pack.domain) — $mod module"

[dependencies]
$depLines

[lints.rust]
unsafe_code = "forbid"
"@
        
        $idBase = "mge.$($pack.prefix).$mod.v1"
        $domain = $pack.domain
        
        $libRs = @"
//! @id $idBase
//! @role plugin
//! @layer plugin
//! @domain $domain
//! @do scaffold_$($mod)_module

pub mod components;
pub mod systems;
pub mod events;

pub use components::*;
pub use events::*;
"@
        
        $componentsRs = @"
//! @id $idBase.components
//! @role data
//! @layer plugin
//! @domain $domain

use mge_ecs::Component;

// TODO: composants à définir
"@
        
        $systemsRs = @"
//! @id $idBase.systems
//! @role system
//! @layer plugin
//! @domain $domain

// TODO: systèmes à implémenter
"@
        
        $eventsRs = @"
//! @id $idBase.events
//! @role event
//! @layer plugin
//! @domain $domain

// TODO: événements à définir
"@
        
        $indexMd = @"
Plugin: $crateName
Version: v1
Domain: $domain

Components:
- (à définir)

Systems:
- (à implémenter)

Events:
- (à définir)

Hot path: no
Headless safe: yes
AI-Native Score: 0/10
"@
        
        Set-Content -Path "$crateDir\Cargo.toml" -Value $cargoToml -Encoding UTF8
        Set-Content -Path "$srcDir\lib.rs" -Value $libRs -Encoding UTF8
        Set-Content -Path "$srcDir\components.rs" -Value $componentsRs -Encoding UTF8
        Set-Content -Path "$srcDir\systems.rs" -Value $systemsRs -Encoding UTF8
        Set-Content -Path "$srcDir\events.rs" -Value $eventsRs -Encoding UTF8
        Set-Content -Path "$crateDir\index.md" -Value $indexMd -Encoding UTF8
        
        $wsPath = "crates/$($pack.dir)/$crateName"
        $allMembers += "    `"$wsPath`""
        
        Write-Host "Created $crateName"
    }
}

# Update workspace Cargo.toml
$wsContent = Get-Content "$root\Cargo.toml" -Raw
$memberList = ($allMembers | ForEach-Object { $_ }) -join ",`n"
$wsContent = $wsContent -replace "  # packs genre ajoutés incrémentalement", "`n$memberList"
Set-Content -Path "$root\Cargo.toml" -Value $wsContent -Encoding UTF8
Write-Host "Updated workspace Cargo.toml with $($allMembers.Count) new members"
