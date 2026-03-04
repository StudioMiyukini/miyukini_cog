#!/usr/bin/env python3
"""Scan code health metrics - exclude only tests/ dir files."""
import os
import json
from collections import defaultdict

BASE = os.path.dirname(os.path.abspath(__file__))

def scan_package(pkg_path):
    """Scan a package directory. Exclude files in tests/ dirs from prod counts."""
    metrics = {
        "unwrap": 0, "panic": 0, "expect": 0, "unsafe": 0,
        "rs_files": 0, "has_tests": False
    }

    for dirpath, dirnames, filenames in os.walk(pkg_path):
        for fn in filenames:
            if not fn.endswith(".rs"):
                continue

            filepath = os.path.join(dirpath, fn)
            rel = os.path.relpath(filepath, BASE).replace("\\", "/")
            metrics["rs_files"] += 1

            try:
                with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                    lines = f.readlines()
            except Exception:
                continue

            # Check for #[test] anywhere in file
            for line in lines:
                if "#[test]" in line:
                    metrics["has_tests"] = True
                    break

            # Is this in a tests/ directory?
            is_test_dir = "/tests/" in rel
            if is_test_dir:
                continue  # Skip for prod metrics

            # Count in ALL production lines (including #[cfg(test)] blocks)
            for line in lines:
                stripped = line.strip()
                is_comment = (stripped.startswith("//") or stripped.startswith("///")
                             or stripped.startswith("*") or stripped.startswith("/*"))

                if ".unwrap()" in line:
                    metrics["unwrap"] += 1
                if "panic!(" in line:
                    metrics["panic"] += 1
                if ".expect(" in line:
                    metrics["expect"] += 1
                if "unsafe " in line and not is_comment:
                    if "forbid(unsafe_code)" not in line and "unsafe_code" not in line:
                        metrics["unsafe"] += 1

    return metrics


# Main
results = {}

for root in ["crates", "apps"]:
    root_path = os.path.join(BASE, root)
    if not os.path.isdir(root_path):
        continue

    pkgs = sorted([d for d in os.listdir(root_path)
                   if os.path.isdir(os.path.join(root_path, d))])

    for pkg in pkgs:
        pkg_path = os.path.join(root_path, pkg)
        key = f"{root}/{pkg}"
        metrics = scan_package(pkg_path)
        results[key] = metrics

# Compute totals
crate_results = {k: v for k, v in results.items() if k.startswith("crates/")}
app_results = {k: v for k, v in results.items() if k.startswith("apps/")}

total_unwrap = sum(d["unwrap"] for d in results.values())
total_panic = sum(d["panic"] for d in results.values())
total_expect = sum(d["expect"] for d in results.values())
total_unsafe = sum(d["unsafe"] for d in results.values())
zero_tests = sum(1 for d in results.values() if not d["has_tests"])

crate_unwrap = sum(d["unwrap"] for d in crate_results.values())
crate_panic = sum(d["panic"] for d in crate_results.values())
crate_expect = sum(d["expect"] for d in crate_results.values())
app_unwrap = sum(d["unwrap"] for d in app_results.values())
app_panic = sum(d["panic"] for d in app_results.values())
app_expect = sum(d["expect"] for d in app_results.values())

print(f"=== TOTALS ===")
print(f"unwrap_total_prod = {total_unwrap}")
print(f"panic_total_prod = {total_panic}")
print(f"expect_total_prod = {total_expect}")
print(f"unsafe_total_prod = {total_unsafe}")
print(f"packages_zero_tests = {zero_tests}")
print(f"total_packages = {len(results)}")
print(f"")
print(f"crate_unwrap = {crate_unwrap}, crate_panic = {crate_panic}, crate_expect = {crate_expect}")
print(f"app_unwrap = {app_unwrap}, app_panic = {app_panic}, app_expect = {app_expect}")

# Top 10
print(f"\n=== TOP 10 MOST PROBLEMATIC ===")
ranked = sorted(results.items(),
                key=lambda x: x[1]["unwrap"] + x[1]["panic"] + x[1]["expect"],
                reverse=True)
for i, (pkg, d) in enumerate(ranked[:10], 1):
    score = d["unwrap"] + d["panic"] + d["expect"]
    print(f"  {i:2d}. {score:4d} | {pkg:40s} | u={d['unwrap']:3d} p={d['panic']:3d} e={d['expect']:3d} | rs={d['rs_files']:3d} tests={'Y' if d['has_tests'] else 'N'}")

# Packages without tests
print(f"\n=== PACKAGES WITHOUT TESTS ({zero_tests}) ===")
no_tests = sorted([k for k, v in results.items() if not v["has_tests"]])
for pkg in no_tests:
    d = results[pkg]
    print(f"  {pkg:40s} | rs={d['rs_files']:3d}")

# Save JSON
output = {
    "totals": {
        "unwrap_total_prod": total_unwrap,
        "panic_total_prod": total_panic,
        "expect_total_prod": total_expect,
        "unsafe_total_prod": total_unsafe,
        "packages_zero_tests": zero_tests,
    },
    "crates": {k.split("/",1)[1]: v for k, v in sorted(crate_results.items())},
    "apps": {k.split("/",1)[1]: v for k, v in sorted(app_results.items())},
}
with open(os.path.join(BASE, "tmp_scan_results.json"), "w") as f:
    json.dump(output, f, indent=2)
print(f"\nJSON saved to tmp_scan_results.json")
