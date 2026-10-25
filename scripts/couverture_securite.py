#!/usr/bin/env python3
"""Couverture des lignes hors tests, par crate, à partir d'un rapport LCOV.

Utilisé par `scripts/couverture-securite.sh` (contrainte C-62). Ne mesure rien
lui-même : lit le LCOV produit par `cargo llvm-cov report --lcov`.

Lignes exclues du calcul (réutilise l'analyse lexicale de compter_unwrap.py) :
- items sous `#[cfg(test)]` / `#[test]` / `#[tokio::test]` ... ;
- modules fichier `#[cfg(test)] mod x;` et leurs sous-modules ;
- dossiers `tests/`, `benches/`, `examples/`, `fuzz/`, et `build.rs`.

Usage : couverture_securite.py RACINE LCOV --crates a,b,c [--seuil 80]
Code de sortie : 0 si toutes les crates atteignent le seuil, 1 sinon,
2 si une crate est introuvable ou absente du rapport.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import compter_unwrap as cu  # noqa: E402  (analyse lexicale partagée)


def lignes_test(chemin: Path, exclus: set[Path]) -> set[int]:
    """Numéros de ligne (1-based) couverts par un item de test ; alimente `exclus`."""
    try:
        brut = chemin.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return set()
    code = cu.neutraliser(brut)
    res: set[int] = set()
    for m in cu.RE_ATTR_TEST.finditer(code):
        debut = m.start()
        corps = cu.fin_attributs(code, m.end())
        mm = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;", code[corps:])
        if mm:
            nom = mm.group(1)
            base = chemin.parent
            cands = [base / f"{nom}.rs", base / nom / "mod.rs"]
            if chemin.name not in ("mod.rs", "lib.rs", "main.rs"):
                cands += [base / chemin.stem / f"{nom}.rs", base / chemin.stem / nom / "mod.rs"]
            pm = re.search(r'path\s*=\s*"([^"]*)"', brut[debut:corps])
            if pm:
                cands.append(base / pm.group(1))
            for c in cands:
                exclus.add(c.resolve())
        fin = cu.fin_item(code, corps)
        l1 = code.count("\n", 0, debut) + 1
        l2 = code.count("\n", 0, fin) + 1
        res.update(range(l1, l2 + 1))
    return res


def lire_lcov(chemin: Path) -> dict[Path, dict[int, int]]:
    """{fichier résolu: {ligne: compteur}} ; plusieurs blocs pour un fichier sont fusionnés."""
    out: dict[Path, dict[int, int]] = {}
    courant: dict[int, int] | None = None
    with chemin.open(encoding="utf-8", errors="replace") as f:
        for ligne in f:
            ligne = ligne.strip()
            if ligne.startswith("SF:"):
                courant = out.setdefault(Path(ligne[3:]).resolve(), {})
            elif ligne.startswith("DA:") and courant is not None:
                no, nb = ligne[3:].split(",")[:2]
                no_i = int(no)
                courant[no_i] = max(courant.get(no_i, 0), int(float(nb)))
            elif ligne == "end_of_record":
                courant = None
    return out


def mesurer_crate(dossier: Path, autres: set[Path], lcov: dict[Path, dict[int, int]]):
    """(lignes instrumentées, lignes exécutées, [(fichier, total, exécutées)])."""
    fichiers = cu.fichiers_crate(dossier, autres)
    exclus: set[Path] = set()
    masques = {f: lignes_test(f, exclus) for f in fichiers}
    # sous-modules d'un module de test : exclus aussi
    changement = True
    while changement:
        changement = False
        for f in fichiers:
            rf = f.resolve()
            if rf in exclus:
                continue
            for e in list(exclus):
                parent = e.parent / e.stem if e.name != "mod.rs" else e.parent
                if parent in rf.parents:
                    exclus.add(rf)
                    changement = True
                    break
    total = touche = 0
    detail = []
    for f in fichiers:
        rf = f.resolve()
        if rf in exclus or rf not in lcov:
            continue
        da = {no: n for no, n in lcov[rf].items() if no not in masques[f]}
        t, h = len(da), sum(1 for n in da.values() if n > 0)
        if t:
            detail.append((f, t, h))
        total += t
        touche += h
    return total, touche, detail


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("racine")
    ap.add_argument("lcov")
    ap.add_argument("--crates", required=True)
    ap.add_argument("--seuil", type=float, default=80.0)
    ap.add_argument("--detail", action="store_true", help="couverture par fichier")
    args = ap.parse_args()

    racine = Path(args.racine).resolve()
    crates = cu.trouver_crates(racine)
    dossiers = {p.resolve() for p in crates.values()}
    lcov = lire_lcov(Path(args.lcov))
    code = 0

    print(f"{'crate':<22} {'lignes':>7} {'couvertes':>9} {'%':>7}  état (seuil {args.seuil:g} %)")
    print("-" * 64)
    g_t = g_h = 0
    for nom in [c for c in args.crates.split(",") if c]:
        if nom not in crates:
            print(f"{nom:<22} {'?':>7} {'?':>9} {'?':>7}  INTROUVABLE")
            code = 2
            continue
        d = crates[nom]
        t, h, detail = mesurer_crate(d, dossiers - {d.resolve()}, lcov)
        if t == 0:
            print(f"{nom:<22} {0:>7} {0:>9} {'-':>7}  ABSENTE DU RAPPORT")
            code = 2
            continue
        pct = 100.0 * h / t
        ok = pct >= args.seuil
        print(f"{nom:<22} {t:>7} {h:>9} {pct:>6.1f}%  {'ok' if ok else 'SOUS LE SEUIL'}")
        if not ok and code == 0:
            code = 1
        if args.detail:
            for f, ft, fh in sorted(detail, key=lambda x: x[2] / x[1]):
                print(f"    {100.0 * fh / ft:6.1f}%  {ft - fh:>5} non couv.  {f.relative_to(racine)}")
        g_t += t
        g_h += h
    print("-" * 64)
    if g_t:
        print(f"{'TOTAL':<22} {g_t:>7} {g_h:>9} {100.0 * g_h / g_t:>6.1f}%")
    return code


if __name__ == "__main__":
    sys.exit(main())
