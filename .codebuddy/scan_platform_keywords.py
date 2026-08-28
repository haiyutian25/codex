"""Scan codex-rs for 'windows' / 'macos' keywords and export an MD report.

For every match, records: file, line number, line content, and the
crate/module the file belongs to (top-level directory under codex-rs).
"""

import pathlib
import re
from collections import defaultdict

ROOT = pathlib.Path("codex-rs")
OUT = pathlib.Path("platform-keyword-scan.md")
KEYWORDS = ["windows", "macos"]
# Source files worth scanning: Rust code + crate manifests (platform cfg
# dependencies live in Cargo.toml).
SUFFIXES = {".rs", ".toml"}
SKIP_DIRS = {"target", "node_modules", ".git"}

match_re = re.compile("|".join(KEYWORDS), re.IGNORECASE)


def module_of(path: pathlib.Path) -> str:
    rel = path.relative_to(ROOT)
    parts = rel.parts
    if len(parts) == 1:
        return "(codex-rs root)"
    return parts[0]


def scan():
    # module -> file -> list of (lineno, line_text, matched_keywords)
    results = defaultdict(lambda: defaultdict(list))
    file_count = 0
    for path in sorted(ROOT.rglob("*")):
        if not path.is_file() or path.suffix not in SUFFIXES:
            continue
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        file_count += 1
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for lineno, line in enumerate(text.splitlines(), start=1):
            found = sorted({m.group(0).lower() for m in match_re.finditer(line)})
            if found:
                results[module_of(path)][path].append((lineno, line.strip(), found))
    return results, file_count


def main():
    results, file_count = scan()

    total_matches = sum(
        len(hits) for files in results.values() for hits in files.values()
    )
    total_files = sum(len(files) for files in results.values())
    kw_totals = {kw: 0 for kw in KEYWORDS}
    for files in results.values():
        for hits in files.values():
            for _, _, kws in hits:
                for kw in kws:
                    kw_totals[kw] += 1

    lines = []
    lines.append("# codex-rs 平台关键词扫描报告（windows / macos）")
    lines.append("")
    lines.append("> 扫描对象：`codex-rs/` 下全部 `.rs` 与 `.toml` 文件（跳过 `target/`）")
    lines.append("> 匹配规则：大小写不敏感（覆盖 `Windows`、`macOS`、`WINDOWS` 等变体）")
    lines.append("")
    lines.append("## 一、总览")
    lines.append("")
    lines.append(f"- 扫描文件数：{file_count}")
    lines.append(f"- 命中文件数：{total_files}")
    lines.append(f"- 命中行数（总）：{total_matches}")
    for kw in KEYWORDS:
        lines.append(f"- `{kw}` 命中行数：{kw_totals[kw]}")
    lines.append("")
    lines.append("## 二、按模块汇总")
    lines.append("")
    lines.append("| 模块 | 命中文件数 | 命中行数 | windows | macos |")
    lines.append("|---|---:|---:|---:|---:|")
    for module in sorted(results):
        files = results[module]
        mod_hits = sum(len(h) for h in files.values())
        mod_kw = {kw: 0 for kw in KEYWORDS}
        for hits in files.values():
            for _, _, kws in hits:
                for kw in kws:
                    mod_kw[kw] += 1
        lines.append(
            f"| `{module}` | {len(files)} | {mod_hits} "
            f"| {mod_kw['windows']} | {mod_kw['macos']} |"
        )
    lines.append("")
    lines.append("## 三、逐文件明细")
    lines.append("")
    for module in sorted(results):
        files = results[module]
        mod_hits = sum(len(h) for h in files.values())
        lines.append(f"### 模块 `{module}`（{len(files)} 个文件 / {mod_hits} 行）")
        lines.append("")
        for path in sorted(files):
            hits = files[path]
            rel = path.as_posix()
            lines.append(f"#### `{rel}`（{len(hits)} 处）")
            lines.append("")
            lines.append("| 行号 | 关键词 | 内容 |")
            lines.append("|---:|---|---|")
            for lineno, text, kws in hits:
                escaped = (
                    text.replace("|", "\\|")
                    .replace("`", "'")[:160]
                )
                kw_cell = ", ".join(f"`{k}`" for k in kws)
                lines.append(f"| {lineno} | {kw_cell} | `{escaped}` |")
            lines.append("")

    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"scanned={file_count} files_with_hits={total_files} matches={total_matches}")
    print(f"report -> {OUT.resolve()}")


if __name__ == "__main__":
    main()
