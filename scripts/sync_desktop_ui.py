#!/usr/bin/env python3
"""
Synchronizes the single-source-of-truth HTML studio from crates/backend-sync/src/web_ui.rs
into apps/desktop/ui/index.html for native Tauri desktop packaging.
"""

import sys
from pathlib import Path

def main():
    root = Path(__file__).resolve().parent.parent
    web_ui_path = root / "crates" / "backend-sync" / "src" / "web_ui.rs"
    dest_path = root / "apps" / "desktop" / "ui" / "index.html"
    dest_path.parent.mkdir(parents=True, exist_ok=True)

    text = web_ui_path.read_text(encoding="utf-8")
    start_marker = 'pub const INDEX_HTML: &str = r###"'
    end_marker = '"###;'
    start_pos = text.find(start_marker)
    if start_pos == -1:
        print("ERROR: Start marker not found in web_ui.rs", file=sys.stderr)
        sys.exit(1)
    start_pos += len(start_marker)
    end_pos = text.find(end_marker, start_pos)
    if end_pos == -1:
        print("ERROR: End marker not found in web_ui.rs", file=sys.stderr)
        sys.exit(1)

    html = text[start_pos:end_pos]
    dest_path.write_text(html, encoding="utf-8")
    print(f"✓ Desktop UI synchronized: {dest_path} ({len(html):,} bytes)")

if __name__ == "__main__":
    main()
