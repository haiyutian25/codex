#!/usr/bin/env python3
"""Analyze the module structure of the codex-rs Rust workspace.

Reads the Cargo workspace members, each crate's metadata and Rust module
declarations (`mod foo;` / `pub mod foo;`), recursively builds the module
tree of every crate, and writes a Markdown report to
`codex-rs-module-structure.md` in the workspace root.

Usage:  python analyze_codex_modules.py
"""

from __future__ import annotations

import re
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent
WS = ROOT / "codex-rs"
OUT = ROOT / "codex-rs-module-structure.md"
MAX_DEPTH = 12

MOD_DECL = re.compile(r"^\s*(pub(?:\s*\([^)]*\))?\s+)?mod\s+([A-Za-z_]\w*)\s*([;{])")
PATH_ATTR = re.compile(r'#\[path\s*=\s*"([^"]+)"\]')


# --------------------------------------------------------------------------- #
# TOML helpers
# --------------------------------------------------------------------------- #
def load_toml(text: str) -> dict:
    try:
        import tomllib

        return tomllib.loads(text)
    except Exception:
        return {}


def parse_workspace_members(ws_toml_text: str) -> list[str]:
    data = load_toml(ws_toml_text)
    members = data.get("workspace", {}).get("members", [])
    if members:
        return members
    m = re.search(r"members\s*=\s*\[(.*?)\]", ws_toml_text, re.S)
    return re.findall(r'"([^"]+)"', m.group(1)) if m else []


def parse_crate_meta(cargo_toml: Path) -> tuple[str | None, str | None, str | None]:
    """Return (package_name, description, lib_path) for a crate."""
    try:
        text = cargo_toml.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return None, None, None
    data = load_toml(text)
    if data:
        pkg = data.get("package", {})
        lib_path = data.get("lib", {}).get("path")
        return pkg.get("name"), pkg.get("description"), lib_path
    # Fallback: crude regex parsing of the [package] section.
    name = desc = None
    in_pkg = False
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("["):
            in_pkg = s == "[package]"
            continue
        if in_pkg:
            m = re.match(r'name\s*=\s*"([^"]+)"', s)
            if m and name is None:
                name = m.group(1)
            m = re.match(r'description\s*=\s*"([^"]+)"', s)
            if m and desc is None:
                desc = m.group(1)
    return name, desc, None


# --------------------------------------------------------------------------- #
# File helpers
# --------------------------------------------------------------------------- #
def count_lines(path: Path) -> int:
    try:
        with path.open(encoding="utf-8", errors="replace") as fh:
            return sum(1 for _ in fh)
    except OSError:
        return 0


def rel(path: Path) -> str:
    try:
        return path.relative_to(ROOT).as_posix()
    except ValueError:
        return path.as_posix()


# --------------------------------------------------------------------------- #
# Module tree extraction
# --------------------------------------------------------------------------- #
def scan_mod_declarations(file: Path):
    """Yield (name, visibility, is_inline, custom_path, is_test) per `mod` decl."""
    try:
        lines = file.read_text(encoding="utf-8", errors="replace").splitlines()
    except OSError:
        return
    for idx, line in enumerate(lines):
        m = MOD_DECL.match(line)
        if not m:
            continue
        vis = (m.group(1) or "").strip() or "private"
        name = m.group(2)
        is_inline = m.group(3) == "{"
        custom, attrs = None, []
        for back in range(idx - 1, max(idx - 8, -1), -1):
            prev = lines[back].strip()
            pm = PATH_ATTR.search(prev)
            if pm:
                custom = pm.group(1)
                attrs.append(prev)
                continue
            if prev.startswith("#[") or prev.startswith("]") or prev == "":
                attrs.append(prev)
                continue
            break
        is_test = "cfg(test)" in "".join(attrs).replace(" ", "")
        yield name, vis, is_inline, custom, is_test


def resolve_child_file(parent_file: Path, modname: str, custom: str | None) -> Path | None:
    if custom:
        cand = (parent_file.parent / custom)
        return cand if cand.exists() else None
    if parent_file.name in ("mod.rs", "lib.rs", "main.rs"):
        base = parent_file.parent
    else:
        base = parent_file.with_suffix("")
    for cand in (base / f"{modname}.rs", base / modname / "mod.rs"):
        if cand.is_file():
            return cand
    return None


def build_module_tree(file: Path, name: str, vis: str, depth: int, seen: set) -> dict:
    node = {
        "name": name,
        "vis": vis,
        "file": file,
        "lines": count_lines(file) if file else 0,
        "inline": file is None,
        "test": False,
        "children": [],
    }
    if file is None or depth >= MAX_DEPTH:
        return node
    key = file.resolve()
    if key in seen:
        return node
    seen.add(key)
    for cname, cvis, inline, custom, is_test in scan_mod_declarations(file):
        if inline:
            child = {
                "name": cname, "vis": cvis, "file": None, "lines": 0,
                "inline": True, "test": is_test, "children": [],
            }
        else:
            child_file = resolve_child_file(file, cname, custom)
            child = build_module_tree(child_file, cname, cvis, depth + 1, seen)
            child["test"] = is_test
        node["children"].append(child)
    return node


def find_crate_root(crate_dir: Path, lib_path: str | None) -> Path | None:
    if lib_path:
        p = crate_dir / lib_path
        if p.is_file():
            return p
    for cand in (crate_dir / "src" / "lib.rs", crate_dir / "src" / "main.rs"):
        if cand.is_file():
            return cand
    return None


# --------------------------------------------------------------------------- #
# Markdown rendering
# --------------------------------------------------------------------------- #
def render_tree(node: dict, depth: int = 0) -> list[str]:
    indent = "  " * depth
    flags = []
    if node["vis"] != "private":
        flags.append(node["vis"])
    if node["test"]:
        flags.append("cfg(test)")
    flag_str = f" *({', '.join(flags)})*" if flags else ""
    if node["inline"]:
        lines = [f"{indent}- `{node['name']}`{flag_str} — inline module"]
    else:
        f = rel(node["file"]) if node["file"] else "file not found"
        lines = [f"{indent}- `{node['name']}`{flag_str} — `{f}` ({node['lines']} lines)"]
    for child in node["children"]:
        lines.extend(render_tree(child, depth + 1))
    return lines


def count_nodes(node: dict) -> int:
    return 1 + sum(count_nodes(c) for c in node["children"])


def main() -> int:
    if not WS.is_dir():
        print(f"error: workspace dir not found: {WS}", file=sys.stderr)
        return 1

    ws_toml = WS / "Cargo.toml"
    members = parse_workspace_members(ws_toml.read_text(encoding="utf-8"))

    crates = []
    for member in members:
        crate_dir = WS / member
        cargo_toml = crate_dir / "Cargo.toml"
        if not cargo_toml.is_file():
            continue
        pkg_name, desc, lib_path = parse_crate_meta(cargo_toml)
        root_file = find_crate_root(crate_dir, lib_path)
        rs_files = sorted(crate_dir.rglob("*.rs"))
        rs_lines = sum(count_lines(p) for p in rs_files)
        tree = None
        if root_file is not None:
            tree = build_module_tree(root_file, root_file.stem, "crate-root", 0, set())
        crates.append({
            "name": pkg_name or member,
            "member": member,
            "dir": crate_dir,
            "desc": desc or "",
            "root": root_file,
            "rs_files": len(rs_files),
            "rs_lines": rs_lines,
            "tree": tree,
        })

    crates.sort(key=lambda c: c["member"])

    all_rs = list(WS.rglob("*.rs"))
    total_rs_lines = sum(count_lines(p) for p in all_rs)
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")

    md: list[str] = []
    md.append("# Codex Rust Workspace — Module Structure")
    md.append("")
    md.append(f"> Auto-generated by `analyze_codex_modules.py` on {now}.")
    md.append("")
    md.append("## Overview")
    md.append("")
    md.append(f"- Workspace root: `{rel(WS)}`")
    md.append(f"- Workspace members (crates): **{len(crates)}**")
    md.append(f"- Total `.rs` files: **{len(all_rs)}**")
    md.append(f"- Total `.rs` lines: **{total_rs_lines:,}**")
    md.append("")
    md.append("## Crates")
    md.append("")
    md.append("| Crate | Path | Description | `.rs` files | Lines | Modules | Root |")
    md.append("|---|---|---|---:|---:|---:|---|")
    for c in crates:
        nmods = count_nodes(c["tree"]) - 1 if c["tree"] else 0
        desc = c["desc"].replace("|", "\\|")
        if len(desc) > 80:
            desc = desc[:77] + "..."
        root = rel(c["root"]) if c["root"] else "—"
        md.append(
            f"| `{c['name']}` | `{rel(c['dir'])}` | {desc or '—'} | "
            f"{c['rs_files']} | {c['rs_lines']:,} | {nmods} | `{root}` |"
        )
    md.append("")
    md.append("## Module trees")
    md.append("")
    md.append(
        "Visibility legend: *private* = no marker, otherwise `pub` / `pub(crate)` / "
        "`pub(super)` etc. *cfg(test)* marks test-only modules. Line counts refer to "
        "the module's own source file."
    )
    md.append("")
    for c in crates:
        md.append(f"### `{c['name']}` — `{rel(c['dir'])}`")
        md.append("")
        if c["desc"]:
            md.append(f"> {c['desc']}")
            md.append("")
        if c["tree"] is None:
            md.append("_No `src/lib.rs` / `src/main.rs` found._")
            md.append("")
            continue
        md.append(f"- Crate root: `{rel(c['root'])}` ({c['tree']['lines']} lines)")
        md.append(f"- Modules declared: {count_nodes(c['tree']) - 1}")
        md.append("")
        if c["tree"]["children"]:
            md.append("```text")
            lines = []
            for child in c["tree"]["children"]:
                lines.extend(render_tree(child))
            md.extend(lines)
            md.append("```")
        else:
            md.append("_No module declarations._")
        md.append("")

    OUT.write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"Wrote {rel(OUT)}")
    print(f"crates={len(crates)} rs_files={len(all_rs)} rs_lines={total_rs_lines}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
