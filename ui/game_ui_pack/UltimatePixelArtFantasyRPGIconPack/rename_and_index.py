#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Renomme les icônes N.png en icon_NNNN.png (4 chiffres) et génère index.json.
Usage: exécuter depuis la racine du pack (UltimatePixelArtFantasyRPGIconPack).
Cible par défaut: Dark Outline/32X32 (5616 fichiers).
"""
from pathlib import Path
import json
import re

# Dossiers à traiter: (nom_relatif, taille pour tri)
# Décommenter les lignes pour traiter tous les styles/résolutions.
FOLDERS = [
    ("Dark Outline/32X32", "32"),
    # ("Dark Outline/16X16", "16"),
    # ("Dark Outline/24X24", "24"),
    # ("Dark Outline/64X64", "64"),
    # ("Light Outline/32X32", "32"),
    # ("Light Outline/16X16", "16"),
    # ("Light Outline/24X24", "24"),
    # ("Light Outline/64X64", "64"),
    # ("Classic/32X32", "32"),
    # ("Classic/16X16", "16"),
    # ("Classic/24X24", "24"),
    # ("Classic/64X64", "64"),
]

# Noms sémantiques et catégories pour les icônes analysées (id -> (name, category))
# Catégories du pack: conditions_states, attributes_stats, basic_menu, general_items_tools,
# gems_jewels, crafting_materials, monster_hunting_drops, food_crops_beverages,
# alchemy_materials, magic_potions, weapons, shields, armor_clothing, accessories, masks
KNOWN_ICONS = {
    1: ("poison_skull", "conditions_states", "Tête de mort / poison / danger (bulle)"),
    2: ("skull_warning", "conditions_states", "Crâne avertissement (bulle)"),
    50: ("metal_scraps", "crafting_materials", "Ferraille / pièces métalliques"),
    100: ("observation_eye", "attributes_stats", "Œil observation / détection (bulle)"),
    200: ("bread_food", "food_crops_beverages", "Pain / ration / nourriture"),
    500: ("mana", "magic_potions", "Mana / énergie magique"),
    1000: ("gem_cyan", "gems_jewels", "Gemme / cristal cyan"),
}


def main():
    base = Path(__file__).resolve().parent
    index = []
    num_pattern = re.compile(r"^(\d+)\.png$", re.IGNORECASE)

    for folder_rel, _ in FOLDERS:
        folder = base / folder_rel
        if not folder.is_dir():
            continue
        renamed_count = 0
        for f in sorted(folder.iterdir(), key=lambda p: (len(p.name), p.name)):
            if not f.suffix.lower() == ".png":
                continue
            m = num_pattern.match(f.name)
            if not m:
                continue
            n = int(m.group(1))
            new_name = f"icon_{n:04d}.png"
            if f.name == new_name:
                continue
            new_path = f.parent / new_name
            if new_path.exists():
                continue
            try:
                f.rename(new_path)
                renamed_count += 1
            except OSError as e:
                print(f"  skip {f}: {e}")
        if renamed_count:
            print(f"Renamed {renamed_count} files in {folder_rel}")

    # Construire l'index à partir d'un dossier de référence (ids 1..max)
    ref_folder = base / "Dark Outline/32X32"
    if ref_folder.is_dir():
        files = [f for f in ref_folder.iterdir() if f.suffix.lower() == ".png"]
        for f in sorted(files, key=lambda p: (len(p.name), p.name)):
            m = re.match(r"^icon_(\d+)\.png$", f.name, re.IGNORECASE) or num_pattern.match(f.name)
            if not m:
                continue
            n = int(m.group(1))
            entry = {
                "id": n,
                "file_renamed": f"icon_{n:04d}.png",
                "name": None,
                "category": None,
                "description_fr": None,
            }
            if n in KNOWN_ICONS:
                entry["name"], entry["category"], entry["description_fr"] = KNOWN_ICONS[n]
            index.append(entry)

    index.sort(key=lambda e: e["id"])
    out = base / "index.json"
    with open(out, "w", encoding="utf-8") as fp:
        json.dump(
            {
                "pack": "UltimatePixelArtFantasyRPGIconPack",
                "source": "Clockwork Raven Studios",
                "total_icons": len(index),
                "icons": index,
            },
            fp,
            ensure_ascii=False,
            indent=2,
        )
    print(f"Index written: {out} ({len(index)} entries)")


if __name__ == "__main__":
    main()
