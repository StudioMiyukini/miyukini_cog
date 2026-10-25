#!/usr/bin/env python3
"""Compteur d'appels interdits par C-10 hors tests (unwrap, expect, panic!, ...).

Utilisé par `scripts/compter-unwrap.sh`. Analyse lexicale légère du Rust :
- retire commentaires (y compris imbriqués), chaînes, chaînes brutes, littéraux
  caractère et octet, en conservant les sauts de ligne ;
- ignore tout item précédé d'un attribut `#[cfg(test)]` (ou `cfg(all(test, ...))`,
  `cfg(any(test, ...))`) ou `#[test]` / `#[tokio::test]` / `#[..::test]` ;
- suit les modules fichier `#[cfg(test)] mod x;` (ignorés) ;
- exclut les dossiers `tests/`, `benches/`, `examples/`, `fuzz/`, `target/` et `build.rs`.

Motifs comptés : `.unwrap()`, `.unwrap_err()`, `.expect(`, `.expect_err(`,
`panic!`, `todo!`, `unimplemented!`, `unreachable!`.

Usage : compter_unwrap.py RACINE [--zero crate1,crate2,...] [--detail]
Code de sortie : 1 si une crate de la liste « zéro » a au moins une occurrence,
2 si une crate de la liste est introuvable, 0 sinon.
"""

from __future__ import annotations

import argparse
import os
import re
import sys
from pathlib import Path

DOSSIERS_EXCLUS = {"tests", "benches", "examples", "fuzz", "target", ".git", "node_modules"}

MOTIFS = [
    ("unwrap", re.compile(r"\.\s*unwrap(?:_err)?\s*\(\s*\)")),
    ("expect", re.compile(r"(?<!\bself)\.\s*expect(?:_err)?\s*\(")),
    ("panic!", re.compile(r"(?<![A-Za-z0-9_])panic\s*!")),
    ("todo!", re.compile(r"(?<![A-Za-z0-9_])todo\s*!")),
    ("unimplemented!", re.compile(r"(?<![A-Za-z0-9_])unimplemented\s*!")),
    ("unreachable!", re.compile(r"(?<![A-Za-z0-9_])unreachable\s*!")),
]


def neutraliser(src: str) -> str:
    """Remplace commentaires et littéraux par des espaces (sauts de ligne gardés)."""
    out = []
    i, n = 0, len(src)

    def blanc(segment: str) -> str:
        return "".join("\n" if c == "\n" else " " for c in segment)

    while i < n:
        c = src[i]
        if c == "/" and i + 1 < n and src[i + 1] == "/":
            j = src.find("\n", i)
            j = n if j < 0 else j
            out.append(blanc(src[i:j]))
            i = j
        elif c == "/" and i + 1 < n and src[i + 1] == "*":
            prof, j = 1, i + 2
            while j < n and prof > 0:
                if src.startswith("/*", j):
                    prof += 1
                    j += 2
                elif src.startswith("*/", j):
                    prof -= 1
                    j += 2
                else:
                    j += 1
            out.append(blanc(src[i:j]))
            i = j
        elif c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            # chaînes brutes r"..", r#".."#, br"..", b"..", b'..'
            m = re.match(r'(b?r)(#*)"', src[i:])
            if m:
                fin = '"' + m.group(2)
                j = src.find(fin, i + len(m.group(0)))
                j = n if j < 0 else j + len(fin)
                out.append(blanc(src[i:j]))
                i = j
                continue
            if c == "b" and i + 1 < n and src[i + 1] in "\"'":
                out.append(" ")
                i += 1
                continue
            out.append(c)
            i += 1
        elif c == '"':
            j = i + 1
            while j < n and src[j] != '"':
                j += 2 if src[j] == "\\" else 1
            j = min(j + 1, n)
            out.append(blanc(src[i:j]))
            i = j
        elif c == "'":
            # littéral caractère ou durée de vie
            m = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F_]+\}|.)|[^\\'\n])'", src[i:])
            if m:
                out.append(blanc(m.group(0)))
                i += len(m.group(0))
            else:
                out.append(c)
                i += 1
        else:
            out.append(c)
            i += 1
    return "".join(out)


RE_ATTR_TEST = re.compile(
    r"#\s*\[\s*(?:cfg\s*\(\s*(?:test|all\s*\(\s*test\b[^\]]*|any\s*\(\s*test\b[^\]]*)\s*\)"
    r"|(?:[A-Za-z_][A-Za-z0-9_]*\s*::\s*)*test(?:_case|_log)?\s*(?:\([^\]]*\))?)\s*\]"
)


def fin_attributs(code: str, i: int) -> int:
    """Saute espaces et attributs `#[...]` à partir de i."""
    n = len(code)
    while True:
        while i < n and code[i].isspace():
            i += 1
        if code.startswith("#[", i) or (code.startswith("#", i) and code[i + 1 : i + 2].isspace()):
            j = code.find("[", i)
            prof = 0
            while j < n:
                if code[j] == "[":
                    prof += 1
                elif code[j] == "]":
                    prof -= 1
                    if prof == 0:
                        break
                j += 1
            i = j + 1
        else:
            return i


def fin_item(code: str, i: int) -> int:
    """Fin de l'item commençant à i : premier `;` hors délimiteurs, ou bloc `{}` équilibré."""
    n = len(code)
    prof = 0
    while i < n:
        c = code[i]
        if c in "([":
            prof += 1
        elif c in ")]":
            prof -= 1
        elif c == ";" and prof == 0:
            return i + 1
        elif c == "{":
            p = 1
            j = i + 1
            while j < n and p > 0:
                if code[j] == "{":
                    p += 1
                elif code[j] == "}":
                    p -= 1
                j += 1
            # `impl X for Y {}` / `fn f() {}` / `mod m {}` : bloc final de l'item
            if prof == 0:
                return j
            i = j
            continue
        i += 1
    return n


LINT_VERS_TYPE = {
    "unwrap_used": "unwrap",
    "expect_used": "expect",
    "panic": "panic!",
    "todo": "todo!",
    "unimplemented": "unimplemented!",
    "unreachable": "unreachable!",
}
RE_ALLOW = re.compile(r"#\s*(!?)\s*\[\s*allow\s*\(([^\]]*)\)\s*\]")


def portees_allow(code: str) -> dict[str, list[tuple[int, int]]]:
    """Portées des `#[allow(clippy::...)]` (item suivant) et `#![allow(...)]` (fichier)."""
    portees: dict[str, list[tuple[int, int]]] = {}
    for m in RE_ALLOW.finditer(code):
        lints = re.findall(r"clippy\s*::\s*([a-z_]+)", m.group(2))
        types = [LINT_VERS_TYPE[l] for l in lints if l in LINT_VERS_TYPE]
        if not types:
            continue
        if m.group(1):
            portee = (0, len(code))
        else:
            portee = (m.start(), fin_item(code, fin_attributs(code, m.end())))
        for t in types:
            portees.setdefault(t, []).append(portee)
    return portees


def analyser_fichier(chemin: Path, exclus: set[Path]) -> list[tuple[int, str, str, bool]]:
    try:
        brut = chemin.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return []
    code = neutraliser(brut)
    masque = list(code)
    for m in RE_ATTR_TEST.finditer(code):
        debut = m.start()
        corps = fin_attributs(code, m.end())
        mm = re.match(r"(?:pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;", code[corps:])
        if mm:
            attrs = code[debut:corps]
            chemin_attr = re.search(r'path\s*=', attrs)
            nom = mm.group(1)
            base = chemin.parent
            cands = [base / f"{nom}.rs", base / nom / "mod.rs"]
            if chemin.name not in ("mod.rs", "lib.rs", "main.rs"):
                cands += [base / chemin.stem / f"{nom}.rs", base / chemin.stem / nom / "mod.rs"]
            if chemin_attr:
                pm = re.search(r'path\s*=\s*"([^"]*)"', brut[debut:corps])
                if pm:
                    cands.append(base / pm.group(1))
            for c in cands:
                exclus.add(c.resolve())
        fin = fin_item(code, corps)
        for k in range(debut, fin):
            if masque[k] != "\n":
                masque[k] = " "
    code = "".join(masque)
    portees = portees_allow(code)
    lignes_src = brut.splitlines()
    res = []
    for type_, motif in MOTIFS:
        for m in motif.finditer(code):
            ligne = code.count("\n", 0, m.start()) + 1
            texte = lignes_src[ligne - 1].strip() if ligne - 1 < len(lignes_src) else ""
            justifie = any(a <= m.start() < b for a, b in portees.get(type_, []))
            res.append((ligne, type_, texte, justifie))
    return res


def trouver_crates(racine: Path) -> dict[str, Path]:
    crates: dict[str, Path] = {}
    for dossier, sous, fichiers in os.walk(racine):
        sous[:] = [s for s in sous if s not in DOSSIERS_EXCLUS and not s.startswith(".")]
        if "Cargo.toml" in fichiers:
            p = Path(dossier)
            try:
                txt = (p / "Cargo.toml").read_text(encoding="utf-8", errors="replace")
            except OSError:
                continue
            m = re.search(r"^\[package\][^\[]*?^name\s*=\s*\"([^\"]+)\"", txt, re.M | re.S)
            if m:
                crates[m.group(1)] = p
    return crates


def fichiers_crate(dossier: Path, autres: set[Path]) -> list[Path]:
    out = []
    for d, sous, fichiers in os.walk(dossier):
        dp = Path(d)
        sous[:] = [
            s for s in sous
            if s not in DOSSIERS_EXCLUS and not s.startswith(".") and (dp / s).resolve() not in autres
        ]
        for f in fichiers:
            if f.endswith(".rs") and not (dp == dossier and f == "build.rs"):
                out.append(dp / f)
    return sorted(out)


def compter_crate(dossier: Path, autres: set[Path]) -> list[tuple[Path, int, str, str, bool]]:
    fichiers = fichiers_crate(dossier, autres)
    exclus: set[Path] = set()
    resultats = {}
    # deux passes : la première découvre les modules fichier de test
    for f in fichiers:
        resultats[f] = analyser_fichier(f, exclus)
    # propagation : sous-modules d'un module de test exclus
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
    out = []
    for f in fichiers:
        if f.resolve() in exclus:
            continue
        for ligne, type_, texte, justifie in resultats[f]:
            out.append((f, ligne, type_, texte, justifie))
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("racine")
    ap.add_argument("--zero", default="", help="crates devant être à zéro (séparées par des virgules)")
    ap.add_argument("--detail", action="store_true", help="affiche chaque occurrence")
    ap.add_argument("--crate", action="append", default=[], help="restreint à ces crates")
    args = ap.parse_args()

    racine = Path(args.racine).resolve()
    crates = trouver_crates(racine)
    dossiers = {p.resolve() for p in crates.values()}
    zero = [c for c in args.zero.split(",") if c]
    cibles = args.crate or sorted(crates)

    comptes = []
    details = {}
    introuvables = []
    for nom in cibles:
        if nom not in crates:
            introuvables.append(nom)
            continue
        d = crates[nom]
        occ = compter_crate(d, dossiers - {d.resolve()})
        details[nom] = occ
        comptes.append((sum(1 for o in occ if not o[4]), sum(1 for o in occ if o[4]), nom))

    comptes.sort(key=lambda x: (-x[0], -x[1], x[2]))
    print(f"{'compte':>6}  {'allow':>5}  crate   (allow = occurrences sous #[allow(clippy::...)] justifié)")
    for n, a, nom in comptes:
        if n == 0 and a == 0 and not args.crate:
            continue
        marque = " [zéro exigé]" if nom in zero else ""
        print(f"{n:6d}  {a:5d}  {nom}{marque}")
        if args.detail:
            for f, ligne, type_, texte, justifie in details[nom]:
                tag = " (allow)" if justifie else ""
                print(f"                {f.relative_to(racine)}:{ligne}: {type_}{tag}: {texte}")
    total = sum(n for n, _, _ in comptes)
    total_a = sum(a for _, a, _ in comptes)
    print(f"{total:6d}  {total_a:5d}  TOTAL ({sum(1 for n, _, _ in comptes if n)} crates non nulles)")
    sys.stdout.flush()

    code = 0
    for nom in [z for z in zero if not args.crate or z in args.crate]:
        if nom not in crates:
            print(f"ERREUR : crate « zéro exigé » introuvable : {nom}", file=sys.stderr)
            code = 2
            continue
        if nom not in details:
            details[nom] = compter_crate(crates[nom], dossiers - {crates[nom].resolve()})
        n = sum(1 for o in details[nom] if not o[4])
        if n:
            print(f"ÉCHEC : {nom} contient {n} occurrence(s) interdite(s) (C-10)", file=sys.stderr)
            code = code or 1
    for nom in introuvables:
        print(f"ERREUR : crate introuvable : {nom}", file=sys.stderr)
        code = 2
    return code


if __name__ == "__main__":
    sys.exit(main())
