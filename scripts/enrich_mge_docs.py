#!/usr/bin/env python3
"""Enrichit les documents MGE Phase 3 (catégories 11-24) avec une structure standard."""

import os
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent / "docs" / "Miyukini_Game_Engine" / "points"
REF = "../../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md"

# Fichiers déjà enrichis (06-10)
DONE_CATS = {"06-progression", "07-combat", "08-degats-resistances-effets", "09-mort-resurrection", "10-loot-tables"}

TEMPLATE = """# {title}

**Catégorie :** {cat_num}. {cat_name}
**Description :** {desc}

## Contexte

{context}

**Rôle dans le moteur :** {role} Voir [MGE - Référence technique]({ref}).

---

## Spécifications techniques

{specs}

---

## Modèle de données / API

```rust
{sample_code}
```

---

## Références

- [Index catégorie](_index.md)
- [Index MGE](../_index.md)
"""

def get_cat_info(dirname: str) -> tuple[str, str]:
    mapping = {
        "11-inventaire-objets": ("11", "Inventaire et objets"),
        "12-economie-commerce": ("12", "Économie et commerce"),
        "13-social-groupes": ("13", "Social et groupes"),
        "14-siege-territoire": ("14", "Siège et territoire"),
        "15-reputation-criminalite": ("15", "Réputation et criminalité"),
        "16-magie-sorts": ("16", "Magie et sorts"),
        "17-montures-familiers": ("17", "Montures et familiers"),
        "18-logement-monde": ("18", "Logement et monde"),
        "19-quetes-missions": ("19", "Quêtes et missions"),
        "20-interface": ("20", "Interface"),
        "21-ia-bots": ("21", "IA et bots"),
        "22-reseau-serveurs": ("22", "Réseau et serveurs"),
        "23-systeme": ("23", "Système"),
        "24-meta-moderation": ("24", "Meta et modération"),
    }
    for k, v in mapping.items():
        if k in dirname:
            return v
    return ("?", "?")

def title_from_filename(name: str) -> str:
    return name.replace(".md", "").replace("-", " ").title()

def enrich_file(path: Path, desc: str) -> bool:
    rel = path.relative_to(BASE)
    cat_dir = rel.parts[0]
    if cat_dir in DONE_CATS:
        return False
    content = path.read_text(encoding="utf-8")
    if len(content.splitlines()) > 80:
        return False
    cat_num, cat_name = get_cat_info(cat_dir)
    title = title_from_filename(path.stem)
    ref_up = ("../" * (len(rel.parts))) + REF
    ctx = f"Point de la référence technique MGE. {desc}"
    role = "Voir le contexte du point."
    specs = "- Contraintes et paramètres à définir\n- Formules si applicable\n- Intégration KindMother pour la persistance"
    code = "// Structures et signatures à définir"
    out = TEMPLATE.format(
        title=title,
        cat_num=cat_num,
        cat_name=cat_name,
        desc=desc,
        context=ctx,
        role=role,
        ref=REF,
        specs=specs,
        sample_code=code,
    )
    path.write_text(out, encoding="utf-8")
    return True

def main():
    count = 0
    for cat_dir in sorted(BASE.iterdir()):
        if not cat_dir.is_dir() or cat_dir.name in DONE_CATS:
            continue
        idx_path = cat_dir / "_index.md"
        if not idx_path.exists():
            continue
        import re
        descs = {}
        for line in idx_path.read_text(encoding="utf-8").splitlines():
            if "](" in line and ".md)" in line:
                m = re.search(r'\]\(([^)]+\.md)\)\s*\|\s*([^|]*)$', line)
                if m:
                    descs[m.group(1)] = m.group(2).strip()
        for md in cat_dir.glob("*.md"):
            if md.name == "_index.md":
                continue
            desc = descs.get(md.name, "À définir.")
            if enrich_file(md, desc):
                count += 1
                print(f"Enriched: {md.relative_to(BASE)}")
    print(f"Total: {count} files enriched")

if __name__ == "__main__":
    main()
