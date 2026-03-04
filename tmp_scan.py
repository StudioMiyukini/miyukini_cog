#!/usr/bin/env python3
"""Scan code health metrics for Miyukini COG workspace."""
import subprocess
import os
import json
from collections import defaultdict

BASE = os.path.dirname(os.path.abspath(__file__))

def rg(args, timeout=60):
    """Run ripgrep with given args."""
    cmd = ["rg"] + args
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, cwd=BASE)
        return r.stdout
    except Exception as e:
        return ""

def get_pkg_and_type(filepath):
    """Return (type, package_name) from filepath."""
    fp = filepath.replace("\\", "/")
    parts = fp.split("/")
    if len(parts) >= 2:
        return parts[0], parts[1]  # e.g. "crates", "miyucloud"
    return "unknown", "unknown"

def get_cfg_test_lines(root):
    """Get first #[cfg(test)] line number per file."""
    out = rg(["-n", "--type", "rust", r"#\[cfg\(test\)\]", root])
    result = {}
    for line in out.strip().split("\n"):
        if not line or line.startswith("Found"):
            continue
        # Format: path:linenum:#[cfg(test)]
        parts = line.split(":", 2)
        if len(parts) >= 2:
            fp = parts[0].replace("\\", "/")
            try:
                lnum = int(parts[1])
                if fp not in result:
                    result[fp] = lnum
            except ValueError:
                pass
    return result

def count_pattern_prod(root, pattern, cfg_test_map):
    """Count pattern occurrences in production code only."""
    # Get all matches with line numbers
    out = rg(["-n", "--type", "rust", pattern, root])

    per_pkg = defaultdict(int)
    total = 0
    per_file = defaultdict(int)

    for line in out.strip().split("\n"):
        if not line or line.startswith("Found"):
            continue
        # Format: path:linenum:content
        parts = line.split(":", 2)
        if len(parts) < 2:
            continue
        fp = parts[0].replace("\\", "/")
        try:
            lnum = int(parts[1])
        except ValueError:
            continue

        pkg_type, pkg_name = get_pkg_and_type(fp)

        # Skip test files (in tests/ directory)
        if "/tests/" in fp:
            continue

        # If file has #[cfg(test)], only count lines before it
        if fp in cfg_test_map:
            if lnum >= cfg_test_map[fp]:
                continue  # In test section

        per_pkg[f"{pkg_type}/{pkg_name}"] += 1
        total += 1

    return total, per_pkg

def count_rs_files(root):
    """Count .rs files per package."""
    out = rg(["--type", "rust", "--files", root])
    per_pkg = defaultdict(int)
    for line in out.strip().split("\n"):
        if not line:
            continue
        fp = line.replace("\\", "/")
        pkg_type, pkg_name = get_pkg_and_type(fp)
        per_pkg[f"{pkg_type}/{pkg_name}"] += 1
    return per_pkg

def has_tests(root):
    """Check which packages have #[test]."""
    out = rg(["--type", "rust", r"#\[test\]", "--files-with-matches", root])
    per_pkg = defaultdict(bool)
    for line in out.strip().split("\n"):
        if not line:
            continue
        fp = line.replace("\\", "/")
        pkg_type, pkg_name = get_pkg_and_type(fp)
        per_pkg[f"{pkg_type}/{pkg_name}"] = True
    return per_pkg

def count_unsafe_prod(root, cfg_test_map):
    """Count unsafe keyword in production code, excluding forbid(unsafe_code)."""
    # Get all unsafe matches
    out = rg(["-n", "--type", "rust", r"unsafe ", root])

    per_pkg = defaultdict(int)
    total = 0

    for line in out.strip().split("\n"):
        if not line or line.startswith("Found"):
            continue
        parts = line.split(":", 2)
        if len(parts) < 3:
            continue
        fp = parts[0].replace("\\", "/")
        try:
            lnum = int(parts[1])
        except ValueError:
            continue
        content = parts[2] if len(parts) > 2 else ""

        # Skip if it's the forbid directive
        if "forbid(unsafe_code)" in content:
            continue
        # Skip if it's a comment about unsafe
        if content.strip().startswith("//") or content.strip().startswith("*") or content.strip().startswith("///"):
            continue
        # Skip references to unsafe_code in lint sections
        if "unsafe_code" in content:
            continue

        pkg_type, pkg_name = get_pkg_and_type(fp)

        # Skip test files
        if "/tests/" in fp:
            continue

        # Skip test sections
        if fp in cfg_test_map:
            if lnum >= cfg_test_map[fp]:
                continue

        per_pkg[f"{pkg_type}/{pkg_name}"] += 1
        total += 1

    return total, per_pkg

# === MAIN ===
print("Scanning crates/...")
cfg_test_crates = get_cfg_test_lines("crates")
print(f"  Found {len(cfg_test_crates)} files with #[cfg(test)] in crates/")

print("Scanning apps/...")
cfg_test_apps = get_cfg_test_lines("apps")
print(f"  Found {len(cfg_test_apps)} files with #[cfg(test)] in apps/")

cfg_test_all = {**cfg_test_crates, **cfg_test_apps}

print("Counting unwrap()...")
unwrap_total_c, unwrap_c = count_pattern_prod("crates", r"\.unwrap\(\)", cfg_test_crates)
unwrap_total_a, unwrap_a = count_pattern_prod("apps", r"\.unwrap\(\)", cfg_test_apps)

print("Counting panic!...")
panic_total_c, panic_c = count_pattern_prod("crates", r"panic!\(", cfg_test_crates)
panic_total_a, panic_a = count_pattern_prod("apps", r"panic!\(", cfg_test_apps)

print("Counting .expect(...")
expect_total_c, expect_c = count_pattern_prod("crates", r"\.expect\(", cfg_test_crates)
expect_total_a, expect_a = count_pattern_prod("apps", r"\.expect\(", cfg_test_apps)

print("Counting unsafe...")
unsafe_total_c, unsafe_c = count_unsafe_prod("crates", cfg_test_crates)
unsafe_total_a, unsafe_a = count_unsafe_prod("apps", cfg_test_apps)

print("Counting .rs files...")
rs_c = count_rs_files("crates")
rs_a = count_rs_files("apps")

print("Checking tests...")
tests_c = has_tests("crates")
tests_a = has_tests("apps")

# Aggregate results
results = {
    "crates": {},
    "apps": {}
}

# All crate names
all_crates = set()
for d in [unwrap_c, panic_c, expect_c, unsafe_c, rs_c, tests_c]:
    for k in d:
        if k.startswith("crates/"):
            all_crates.add(k)
for k in rs_c:
    if k.startswith("crates/"):
        all_crates.add(k)

for pkg in sorted(all_crates):
    name = pkg.split("/", 1)[1]
    results["crates"][name] = {
        "unwrap": unwrap_c.get(pkg, 0),
        "panic": panic_c.get(pkg, 0),
        "expect": expect_c.get(pkg, 0),
        "unsafe": unsafe_c.get(pkg, 0),
        "rs_files": rs_c.get(pkg, 0),
        "has_tests": tests_c.get(pkg, False)
    }

all_apps = set()
for d in [unwrap_a, panic_a, expect_a, unsafe_a, rs_a, tests_a]:
    for k in d:
        if k.startswith("apps/"):
            all_apps.add(k)
for k in rs_a:
    if k.startswith("apps/"):
        all_apps.add(k)

for pkg in sorted(all_apps):
    name = pkg.split("/", 1)[1]
    results["apps"][name] = {
        "unwrap": unwrap_a.get(pkg, 0),
        "panic": panic_a.get(pkg, 0),
        "expect": expect_a.get(pkg, 0),
        "unsafe": unsafe_a.get(pkg, 0),
        "rs_files": rs_a.get(pkg, 0),
        "has_tests": tests_a.get(pkg, False)
    }

# Totals
total_unwrap = unwrap_total_c + unwrap_total_a
total_panic = panic_total_c + panic_total_a
total_expect = expect_total_c + expect_total_a
total_unsafe = unsafe_total_c + unsafe_total_a

print(f"\n=== TOTALS ===")
print(f"unwrap_total_prod = {total_unwrap}")
print(f"panic_total_prod = {total_panic}")
print(f"expect_total_prod = {total_expect}")
print(f"unsafe_total_prod = {total_unsafe}")

zero_tests_crates = [n for n, d in results["crates"].items() if not d["has_tests"]]
zero_tests_apps = [n for n, d in results["apps"].items() if not d["has_tests"]]
print(f"packages_zero_tests = {len(zero_tests_crates) + len(zero_tests_apps)}")

# Output JSON for post-processing
print("\n=== JSON_START ===")
print(json.dumps(results, indent=2))
print("=== JSON_END ===")

# Print top 10
print("\n=== TOP10 ===")
all_pkgs = []
for name, d in results["crates"].items():
    score = d["unwrap"] + d["panic"] + d["expect"]
    all_pkgs.append((score, name, "crate", d))
for name, d in results["apps"].items():
    score = d["unwrap"] + d["panic"] + d["expect"]
    all_pkgs.append((score, name, "app", d))

all_pkgs.sort(reverse=True)
for score, name, ptype, d in all_pkgs[:10]:
    print(f"  {score:4d} | {ptype:5s} | {name:30s} | u={d['unwrap']:3d} p={d['panic']:3d} e={d['expect']:3d}")
