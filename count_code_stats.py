"""统计 codex-rs 下各类型文件的个数与行数（排除 .md 与构建产物目录）。

用法: python count_code_stats.py
"""
import os
import sys
from collections import defaultdict

ROOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "codex-rs")
SKIP_DIRS = {"target", ".git", "node_modules", "dist", "build"}
EXCLUDE_EXT = {".md"}


def count_lines(path: str) -> int:
    """按字节统计行数：换行符个数，末行无换行时补 1。"""
    with open(path, "rb") as f:
        data = f.read()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def main() -> None:
    # ext -> [文件数, 行数]
    stats: dict = defaultdict(lambda: [0, 0])

    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
        for name in filenames:
            ext = os.path.splitext(name)[1].lower() or "(无扩展名)"
            if ext in EXCLUDE_EXT:
                continue
            path = os.path.join(dirpath, name)
            stats[ext][0] += 1
            stats[ext][1] += count_lines(path)

    total_files = sum(v[0] for v in stats.values())
    total_lines = sum(v[1] for v in stats.values())

    width = max(len(ext) for ext in stats) + 2
    print(f"{'类型':<{width}}{'文件数':>8}{'行数':>12}")
    print("-" * (width + 20))
    for ext, (files, lines) in sorted(stats.items(), key=lambda kv: (-kv[1][1], kv[0])):
        print(f"{ext:<{width}}{files:>8}{lines:>12,}")
    print("-" * (width + 20))
    print(f"{'合计':<{width}}{total_files:>8}{total_lines:>12,}")


if __name__ == "__main__":
    sys.exit(main())
