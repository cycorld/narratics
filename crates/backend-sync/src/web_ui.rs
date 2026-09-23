pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="ko" data-theme="dark">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Narratics — 로컬 퍼스트 서사 집필 스튜디오</title>
  <link rel="preconnect" href="https://cdn.jsdelivr.net" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/static/pretendard.min.css" />
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Nanum+Myeongjo:wght@400;700;800&family=Noto+Serif+KR:wght@400;600;700&display=swap" />
  <style>
    :root {
      --bg: #ffffff;
      --bg-sidebar: #fafafa;
      --bg-subtle: #f5f5f5;
      --bg-hover: #f0f0f0;
      --bg-active: #e5e5e5;
      --bg-card: #ffffff;
      --border: #e5e5e5;
      --border-strong: #d4d4d4;
      --text: #111111;
      --text-muted: #666666;
      --text-subtle: #8e8e93;
      --accent: #111111;
      --accent-hover: #262626;
      --accent-light: #f5f5f5;
      --accent-border: #e5e5e5;
      --danger: #ef4444;
      --danger-light: #fef2f2;
      --success: #10b981;
      --success-light: #f0fdf4;
      --warning: #f59e0b;
      --warning-light: #fffbeb;
      --font-ui: "Pretendard", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --font-serif: "Nanum Myeongjo", "Noto Serif KR", "KoPub Batang", Georgia, serif;
      --font-sans: "Pretendard", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --paper-max-width: 760px;
      --editor-font-size: 17px;
      --editor-line-height: 1.9;
    }

    [data-theme="dark"] {
      --bg: #0b0c0e;
      --bg-sidebar: #111215;
      --bg-subtle: rgba(255, 255, 255, 0.035);
      --bg-hover: rgba(255, 255, 255, 0.065);
      --bg-active: rgba(255, 255, 255, 0.10);
      --bg-card: #15161a;
      --border: rgba(255, 255, 255, 0.075);
      --border-strong: rgba(255, 255, 255, 0.15);
      --text: #ededed;
      --text-muted: #8a8c95;
      --text-subtle: #565861;
      --accent: #ffffff;
      --accent-hover: #e5e5e5;
      --accent-light: rgba(255, 255, 255, 0.08);
      --accent-border: rgba(255, 255, 255, 0.18);
      --danger: #ef4444;
      --danger-light: rgba(239, 68, 68, 0.15);
      --success: #10b981;
      --success-light: rgba(16, 185, 129, 0.15);
      --warning: #f59e0b;
      --warning-light: rgba(245, 158, 11, 0.15);
    }

    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      font-family: var(--font-ui);
      background: var(--bg);
      color: var(--text);
      display: flex;
      flex-direction: column;
      height: 100vh;
      overflow: hidden;
      font-size: 14px;
      line-height: 1.5;
      -webkit-font-smoothing: antialiased;
      -moz-osx-font-smoothing: grayscale;
    }

    /* Top Navigation Header */
    header.topbar {
      height: 52px;
      border-bottom: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 16px;
      background: rgba(255, 255, 255, 0.90);
      backdrop-filter: blur(16px);
      -webkit-backdrop-filter: blur(16px);
      z-index: 20;
      flex-shrink: 0;
      transition: all 0.2s ease;
    }

    .brand-section {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .brand-logo {
      font-weight: 700;
      font-size: 16px;
      letter-spacing: -0.4px;
      display: flex;
      align-items: center;
      gap: 8px;
      color: var(--text);
      text-decoration: none;
    }

    .brand-badge {
      font-size: 10px;
      padding: 2px 6px;
      border-radius: 4px;
      background: var(--accent-light);
      color: var(--accent);
      font-weight: 700;
      letter-spacing: 0.5px;
      border: 1px solid var(--accent-border);
    }

    .project-title-container {
      display: flex;
      align-items: center;
      gap: 8px;
      border-left: 1px solid var(--border);
      padding-left: 12px;
      margin-left: 4px;
    }

    .project-title-input {
      font-size: 13px;
      font-weight: 600;
      color: var(--text);
      background: transparent;
      border: 1px solid transparent;
      padding: 2px 6px;
      border-radius: 4px;
      outline: none;
      transition: all 0.15s;
    }

    .project-title-input:hover, .project-title-input:focus {
      background: var(--bg-hover);
      border-color: var(--border-strong);
    }

    .storage-status-pill {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      font-size: 12px;
      padding: 3px 10px;
      border-radius: 9999px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      color: var(--text-muted);
      cursor: help;
    }

    .status-dot {
      width: 7px;
      height: 7px;
      border-radius: 50%;
      background: var(--success);
      box-shadow: 0 0 0 2px var(--success-light);
      transition: all 0.3s;
    }

    .status-dot.offline {
      background: var(--warning);
      box-shadow: 0 0 0 2px var(--warning-light);
    }

    .status-dot.saving {
      background: var(--warning);
      animation: pulse 1s infinite;
    }

    @keyframes pulse {
      0% { transform: scale(0.9); opacity: 0.7; }
      50% { transform: scale(1.2); opacity: 1; }
      100% { transform: scale(0.9); opacity: 0.7; }
    }

    .top-actions {
      display: flex;
      align-items: center;
      gap: 8px;
    }

    button.btn {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      padding: 6px 12px;
      border-radius: 6px;
      border: 1px solid var(--border);
      background: var(--bg);
      color: var(--text);
      font-size: 12px;
      font-weight: 500;
      cursor: pointer;
      user-select: none;
      transition: background 0.15s ease, border-color 0.15s ease, color 0.15s ease;
    }

    button.btn:hover {
      background: var(--bg-hover);
      border-color: var(--border-strong);
    }

    button.btn:active {
      background: var(--bg-active);
    }

    button.btn-primary {
      background: #111111;
      color: #ffffff;
      border-color: #111111;
      font-weight: 600;
      box-shadow: 0 1px 2px rgba(0, 0, 0, 0.08);
    }

    button.btn-primary:hover {
      background: #262626;
      border-color: #262626;
    }

    button.btn-active {
      background: var(--bg-hover);
      border-color: var(--border-strong);
      color: var(--text);
      font-weight: 600;
    }

    /* Main Workspace Layout */
    .app-workspace {
      display: flex;
      flex: 1;
      overflow: hidden;
      position: relative;
    }

    /* Left Panel: Arc Binder */
    aside.binder-panel {
      width: 290px;
      border-right: 1px solid var(--border);
      background: var(--bg-sidebar);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
      user-select: none;
      transition: width 0.25s cubic-bezier(0.4, 0, 0.2, 1), margin-left 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    }

    aside.binder-panel.collapsed {
      margin-left: -290px;
    }

    .binder-header {
      padding: 12px 14px;
      border-bottom: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .binder-title-group {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .binder-header h2 {
      font-size: 11px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.8px;
      color: var(--text-muted);
    }

    .binder-actions {
      display: flex;
      align-items: center;
      gap: 4px;
    }

    .icon-btn {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      width: 26px;
      height: 26px;
      border-radius: 5px;
      background: transparent;
      border: 1px solid transparent;
      cursor: pointer;
      color: var(--text-muted);
      transition: all 0.15s;
    }

    .icon-btn:hover {
      background: var(--bg-hover);
      color: var(--text);
      border-color: var(--border);
    }

    .binder-search-wrap {
      padding: 8px 12px;
      border-bottom: 1px solid var(--border);
      position: relative;
      display: flex;
      align-items: center;
    }

    .binder-search-input {
      width: 100%;
      padding: 6px 10px 6px 28px;
      font-size: 12px;
      border: 1px solid var(--border);
      border-radius: 6px;
      background: var(--bg);
      color: var(--text);
      outline: none;
      background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='14' height='14' viewBox='0 0 24 24' fill='none' stroke='%2394a3b8' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Ccircle cx='11' cy='11' r='8'%3E%3C/circle%3E%3Cline x1='21' y1='21' x2='16.65' y2='16.65'%3E%3C/line%3E%3C/svg%3E");
      background-repeat: no-repeat;
      background-position: 8px center;
      transition: border-color 0.15s;
    }

    .binder-search-input:focus {
      border-color: var(--accent);
    }

    .binder-tree {
      flex: 1;
      overflow-y: auto;
      padding: 8px 8px;
    }

    .binder-folder-wrap {
      margin-bottom: 6px;
    }

    .tree-item {
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 6px 8px;
      border-radius: 6px;
      border: 1px solid transparent;
      cursor: pointer;
      font-size: 13px;
      transition: background 0.12s ease, border-color 0.12s ease;
      position: relative;
    }

    .tree-item:hover {
      background: var(--bg-hover);
    }

    .tree-item.active {
      background: var(--bg-hover);
      border: 1px solid var(--border-strong);
      color: var(--text);
      font-weight: 600;
      box-shadow: 0 1px 2px rgba(0, 0, 0, 0.04);
    }

    .tree-item.folder {
      font-weight: 600;
      color: var(--text);
      padding: 7px 8px;
    }

    .tree-item.folder .folder-arrow {
      transition: transform 0.2s;
    }

    .tree-item.folder.collapsed .folder-arrow {
      transform: rotate(-90deg);
    }

    .tree-item.depth-1 {
      padding-left: 26px;
      margin-bottom: 2px;
    }

    .tree-item-title {
      flex: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      font-size: 13px;
    }

    .status-pill-badge {
      font-size: 10px;
      padding: 1px 5px;
      border-radius: 4px;
      font-weight: 600;
      border: 1px solid var(--border);
      background: var(--bg-subtle);
      color: var(--text-muted);
      cursor: pointer;
      user-select: none;
    }

    .status-pill-badge.status-초고 {
      background: var(--bg-subtle);
      color: var(--text-muted);
      border: 1px solid var(--border);
    }

    .status-pill-badge.status-수정중 {
      background: var(--warning-light);
      color: var(--warning);
      border: 1px solid rgba(245, 158, 11, 0.3);
    }

    .status-pill-badge.status-퇴고완료 {
      background: var(--success-light);
      color: var(--success);
      border: 1px solid rgba(16, 185, 129, 0.3);
    }

    .status-pill-badge.status-탈고 {
      background: #111111;
      color: #ffffff;
      border: 1px solid #111111;
    }

    [data-theme="dark"] .status-pill-badge.status-탈고 {
      background: rgba(255, 255, 255, 0.16);
      color: #ffffff;
      border: 1px solid rgba(255, 255, 255, 0.28);
    }

    .tree-item-count {
      font-size: 11px;
      color: var(--text-subtle);
      font-variant-numeric: tabular-nums;
    }

    .tree-item-tools {
      display: none;
      align-items: center;
      gap: 2px;
    }

    .tree-item:hover .tree-item-tools {
      display: flex;
    }

    .tree-item-btn {
      width: 20px;
      height: 20px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      background: transparent;
      border: none;
      color: var(--text-muted);
      border-radius: 4px;
      cursor: pointer;
    }

    .tree-item-btn:hover {
      background: var(--bg-active);
      color: var(--text);
    }

    .binder-footer {
      padding: 10px 14px;
      border-top: 1px solid var(--border);
      background: var(--bg-sidebar);
      font-size: 11px;
      color: var(--text-muted);
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    /* Center Panel: The Manuscript Studio */
    main.studio-panel {
      flex: 1;
      display: flex;
      flex-direction: column;
      background: var(--bg);
      overflow: hidden;
      position: relative;
    }

    .studio-header {
      padding: 8px 24px;
      border-bottom: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: space-between;
      background: var(--bg);
      flex-shrink: 0;
      gap: 16px;
    }

    .scene-meta-group {
      display: flex;
      align-items: center;
      gap: 12px;
      flex: 1;
      min-width: 0;
    }

    .scene-title-input {
      font-size: 18px;
      font-weight: 700;
      border: 1px solid transparent;
      background: transparent;
      color: var(--text);
      outline: none;
      padding: 4px 8px;
      border-radius: 6px;
      flex: 1;
      max-width: 500px;
      font-family: inherit;
      transition: all 0.15s;
    }

    .scene-title-input:hover, .scene-title-input:focus {
      border-color: var(--border);
      background: var(--bg-hover);
    }

    .status-select-wrap {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    .status-select-wrap select {
      font-size: 12px;
      padding: 4px 8px;
      border-radius: 6px;
      border: 1px solid var(--border);
      background: var(--bg);
      color: var(--text);
      outline: none;
      cursor: pointer;
    }

    .studio-controls {
      display: flex;
      align-items: center;
      gap: 6px;
    }

    /* Korean Word Count & Metrics Bar */
    .word-count-bar {
      padding: 7px 24px;
      border-bottom: 1px solid var(--border);
      background: rgba(250, 250, 250, 0.90);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 12px;
      color: var(--text-muted);
      flex-shrink: 0;
      user-select: none;
    }

    .stat-pills {
      display: flex;
      gap: 16px;
      align-items: center;
    }

    .stat-pill strong {
      color: var(--text);
      font-weight: 600;
      margin-left: 2px;
    }

    .target-bar-wrap {
      display: flex;
      align-items: center;
      gap: 10px;
    }

    .target-progress {
      width: 140px;
      height: 6px;
      background: var(--border-strong);
      border-radius: 3px;
      overflow: hidden;
    }

    .target-fill {
      height: 100%;
      background: var(--accent);
      width: 0%;
      transition: width 0.3s ease;
    }

    .autosave-notice {
      display: flex;
      align-items: center;
      gap: 5px;
      font-size: 11px;
      color: var(--text-subtle);
    }

    /* Editor Canvas & Typography Container */
    .editor-container {
      flex: 1;
      display: flex;
      overflow-y: auto;
      padding: 40px 32px 120px 32px;
      justify-content: center;
      position: relative;
    }

    .editor-paper {
      width: 100%;
      max-width: var(--paper-max-width);
      min-height: 100%;
      display: flex;
      flex-direction: column;
      position: relative;
    }

    textarea.manuscript-editor {
      width: 100%;
      flex: 1;
      border: none;
      outline: none;
      background: transparent;
      color: var(--text);
      font-family: var(--font-serif);
      font-size: var(--editor-font-size);
      line-height: var(--editor-line-height);
      resize: none;
      white-space: pre-wrap;
      word-break: break-all;
      letter-spacing: -0.15px;
      padding-bottom: 240px;
    }

    /* Typewriter mode: keeps center scrolling */
    textarea.manuscript-editor.typewriter-mode {
      scroll-behavior: smooth;
    }

    /* Scrivenings Continuous View */
    .scrivenings-view {
      display: none;
      width: 100%;
    }

    .scrivenings-section {
      margin-bottom: 48px;
      padding-bottom: 32px;
      border-bottom: 1px solid var(--border);
    }

    .scrivenings-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;
      padding-bottom: 8px;
      border-bottom: 1px dashed var(--border);
    }

    .scrivenings-title {
      font-size: 17px;
      font-weight: 700;
      color: var(--text);
    }

    .scrivenings-meta {
      font-size: 12px;
      color: var(--text-muted);
    }

    .scrivenings-content {
      font-family: var(--font-serif);
      font-size: var(--editor-font-size);
      line-height: var(--editor-line-height);
      white-space: pre-wrap;
      word-break: break-all;
      color: var(--text);
    }

    /* Right Panel: LoreDeck & Inspector */
    aside.inspector-panel {
      width: 320px;
      border-left: 1px solid var(--border);
      background: var(--bg-sidebar);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
      transition: width 0.25s cubic-bezier(0.4, 0, 0.2, 1), margin-right 0.25s cubic-bezier(0.4, 0, 0.2, 1);
    }

    aside.inspector-panel.collapsed {
      margin-right: -320px;
    }

    .tab-nav {
      display: flex;
      border-bottom: 1px solid var(--border);
      background: var(--bg);
    }

    .tab-btn {
      flex: 1;
      padding: 11px 4px;
      border: none;
      background: transparent;
      font-size: 12px;
      font-weight: 600;
      color: var(--text-muted);
      cursor: pointer;
      border-bottom: 2px solid transparent;
      transition: all 0.15s;
      text-align: center;
    }

    .tab-btn:hover {
      color: var(--text);
      background: var(--bg-subtle);
    }

    .tab-btn.active {
      color: var(--text);
      border-bottom-color: var(--text);
      background: var(--bg);
    }

    .tab-content {
      flex: 1;
      overflow-y: auto;
      padding: 14px;
      display: none;
    }

    .tab-content.active {
      display: flex;
      flex-direction: column;
      gap: 12px;
    }

    .category-filter-chips {
      display: flex;
      gap: 4px;
      overflow-x: auto;
      padding-bottom: 4px;
      scrollbar-width: none;
    }

    .filter-chip {
      font-size: 11px;
      padding: 3px 8px;
      border-radius: 12px;
      border: 1px solid var(--border);
      background: var(--bg);
      color: var(--text-muted);
      cursor: pointer;
      white-space: nowrap;
      transition: all 0.12s;
    }

    .filter-chip:hover {
      border-color: var(--border-strong);
      color: var(--text);
    }

    .filter-chip.active {
      background: var(--text);
      border-color: var(--text);
      color: var(--bg);
      font-weight: 600;
    }

    .lore-card {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 12px;
      cursor: pointer;
      transition: border-color 0.15s, box-shadow 0.15s;
    }

    .lore-card:hover {
      border-color: var(--border-strong);
      box-shadow: 0 2px 8px rgba(0,0,0,0.05);
    }

    .lore-card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 6px;
    }

    .lore-name {
      font-weight: 700;
      font-size: 13px;
      color: var(--text);
    }

    .lore-category-tag {
      font-size: 10px;
      padding: 2px 6px;
      border-radius: 4px;
      background: var(--bg-subtle);
      color: var(--text-muted);
      border: 1px solid var(--border);
    }

    .lore-aliases {
      display: flex;
      gap: 4px;
      flex-wrap: wrap;
      margin-bottom: 6px;
    }

    .lore-alias-pill {
      font-size: 10px;
      color: var(--text-subtle);
      background: var(--bg-hover);
      padding: 1px 5px;
      border-radius: 3px;
    }

    .lore-content-text {
      font-size: 12px;
      color: var(--text-muted);
      line-height: 1.5;
      display: -webkit-box;
      -webkit-line-clamp: 3;
      -webkit-box-orient: vertical;
      overflow: hidden;
    }

    /* Snapshots tab styles */
    .snapshot-card {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 12px;
      display: flex;
      flex-direction: column;
      gap: 6px;
    }

    .snapshot-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .snapshot-label {
      font-weight: 600;
      font-size: 13px;
      color: var(--text);
    }

    .snapshot-time {
      font-size: 11px;
      color: var(--text-subtle);
    }

    .snapshot-actions {
      display: flex;
      justify-content: flex-end;
      gap: 6px;
      margin-top: 4px;
    }

    /* Floating Autocomplete for @mentions */
    .mention-popup {
      position: absolute;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      box-shadow: 0 8px 24px rgba(0,0,0,0.12);
      width: 220px;
      max-height: 200px;
      overflow-y: auto;
      z-index: 50;
      display: none;
    }

    .mention-item {
      padding: 8px 10px;
      font-size: 12px;
      cursor: pointer;
      display: flex;
      justify-content: space-between;
      align-items: center;
      border-bottom: 1px solid var(--border);
    }

    .mention-item:last-child {
      border-bottom: none;
    }

    .mention-item:hover, .mention-item.selected {
      background: var(--bg-hover);
      color: var(--text);
      font-weight: 600;
    }

    .mention-cat {
      font-size: 10px;
      color: var(--text-muted);
    }

    /* Typography Settings Dropdown */
    .typo-dropdown-menu {
      position: absolute;
      top: 48px;
      right: 120px;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 12px;
      box-shadow: 0 10px 30px rgba(0,0,0,0.12);
      width: 260px;
      z-index: 40;
      display: none;
      flex-direction: column;
      gap: 12px;
    }

    .typo-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 12px;
    }

    .typo-btn-group {
      display: flex;
      gap: 2px;
      background: var(--bg-subtle);
      padding: 2px;
      border-radius: 6px;
    }

    .typo-btn {
      padding: 3px 8px;
      font-size: 11px;
      background: transparent;
      border: none;
      color: var(--text-muted);
      border-radius: 4px;
      cursor: pointer;
    }

    .typo-btn.active {
      background: var(--bg);
      color: var(--text);
      font-weight: 700;
      box-shadow: 0 1px 3px rgba(0,0,0,0.08);
    }

    /* Modals */
    .modal-overlay {
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.45);
      backdrop-filter: blur(2px);
      display: none;
      align-items: center;
      justify-content: center;
      z-index: 100;
    }

    .modal-box {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 12px;
      width: 480px;
      max-width: 92%;
      padding: 22px;
      box-shadow: 0 20px 40px rgba(0,0,0,0.2);
    }

    .modal-title {
      font-size: 16px;
      font-weight: 700;
      margin-bottom: 14px;
      color: var(--text);
    }

    .form-group {
      margin-bottom: 14px;
    }

    .form-group label {
      display: block;
      font-size: 12px;
      font-weight: 600;
      margin-bottom: 6px;
      color: var(--text-muted);
    }

    .form-group input, .form-group textarea, .form-group select {
      width: 100%;
      padding: 8px 10px;
      border: 1px solid var(--border);
      border-radius: 6px;
      background: var(--bg);
      color: var(--text);
      font-size: 13px;
      outline: none;
      transition: border-color 0.15s;
    }

    .form-group input:focus, .form-group textarea:focus, .form-group select:focus {
      border-color: var(--border-strong);
    }

    .modal-actions {
      display: flex;
      justify-content: flex-end;
      gap: 8px;
      margin-top: 20px;
    }

    /* SVG Icon Utility */
    .icon {
      width: 15px;
      height: 15px;
      stroke-width: 1.8;
      stroke: currentColor;
      fill: none;
      stroke-linecap: round;
      stroke-linejoin: round;
      display: inline-block;
      vertical-align: middle;
      flex-shrink: 0;
    }

    /* Raycast Signature kbd badge */
    kbd {
      font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", monospace;
      font-size: 10px;
      font-weight: 600;
      padding: 1px 5px;
      border-radius: 4px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      color: var(--text-muted);
      box-shadow: 0 1px 0 rgba(0, 0, 0, 0.15);
      display: inline-flex;
      align-items: center;
      line-height: 1.3;
      user-select: none;
    }

    [data-theme="dark"] kbd {
      background: rgba(255, 255, 255, 0.08);
      border: 1px solid rgba(255, 255, 255, 0.13);
      color: #9da0aa;
      box-shadow: 0 1px 0 rgba(0, 0, 0, 0.4);
    }

    /* Raycast Dark Theme Elevated Surfaces & Glass */
    [data-theme="dark"] header.topbar {
      background: rgba(11, 12, 14, 0.85);
      backdrop-filter: blur(16px);
      -webkit-backdrop-filter: blur(16px);
      border-bottom: 1px solid rgba(255, 255, 255, 0.07);
    }

    [data-theme="dark"] .word-count-bar {
      background: rgba(17, 18, 21, 0.75);
      backdrop-filter: blur(8px);
      -webkit-backdrop-filter: blur(8px);
      border-bottom: 1px solid rgba(255, 255, 255, 0.06);
    }

    [data-theme="dark"] aside.binder-panel,
    [data-theme="dark"] aside.inspector-panel {
      background: #111215;
      border-color: rgba(255, 255, 255, 0.07);
    }

    [data-theme="dark"] .binder-header,
    [data-theme="dark"] .binder-search-wrap,
    [data-theme="dark"] .binder-footer,
    [data-theme="dark"] .studio-header,
    [data-theme="dark"] .tab-nav {
      border-color: rgba(255, 255, 255, 0.07);
      background: transparent;
    }

    [data-theme="dark"] .tree-item {
      color: #b0b2ba;
      border: 1px solid transparent;
    }

    [data-theme="dark"] .tree-item:hover {
      background: rgba(255, 255, 255, 0.05);
      color: #ededed;
    }

    [data-theme="dark"] .tree-item.active {
      background: rgba(255, 255, 255, 0.085);
      border: 1px solid rgba(255, 255, 255, 0.1);
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.35);
      color: #ffffff;
      font-weight: 600;
    }

    [data-theme="dark"] .tree-item.folder {
      color: #ededed;
    }

    /* Raycast Pill Badges */
    [data-theme="dark"] .status-pill-badge.status-초고 {
      background: rgba(255, 255, 255, 0.06);
      color: #8a8c95;
      border: 1px solid rgba(255, 255, 255, 0.09);
    }

    [data-theme="dark"] .status-pill-badge.status-수정중 {
      background: rgba(245, 158, 11, 0.12);
      color: #fbbf24;
      border: 1px solid rgba(245, 158, 11, 0.28);
    }

    [data-theme="dark"] .status-pill-badge.status-퇴고완료 {
      background: rgba(16, 185, 129, 0.12);
      color: #34d399;
      border: 1px solid rgba(16, 185, 129, 0.28);
    }

    [data-theme="dark"] .status-pill-badge.status-탈고 {
      background: rgba(99, 102, 241, 0.15);
      color: #a5b4fc;
      border: 1px solid rgba(99, 102, 241, 0.32);
    }

    /* Raycast Cards */
    [data-theme="dark"] .lore-card,
    [data-theme="dark"] .snapshot-card {
      background: #15161a;
      border: 1px solid rgba(255, 255, 255, 0.075);
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    }

    [data-theme="dark"] .lore-card:hover,
    [data-theme="dark"] .snapshot-card:hover {
      background: #1a1b21;
      border-color: rgba(255, 255, 255, 0.16);
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.5);
    }

    /* Raycast Buttons */
    [data-theme="dark"] button.btn {
      background: rgba(255, 255, 255, 0.045);
      border: 1px solid rgba(255, 255, 255, 0.085);
      color: #ededed;
      box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    }

    [data-theme="dark"] button.btn:hover {
      background: rgba(255, 255, 255, 0.085);
      border-color: rgba(255, 255, 255, 0.16);
    }

    [data-theme="dark"] button.btn-primary {
      background: #ffffff;
      border-color: #ffffff;
      color: #0b0c0e;
      font-weight: 600;
      box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    }

    [data-theme="dark"] button.btn-primary:hover {
      background: #e5e5e5;
      border-color: #e5e5e5;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.5);
    }

    [data-theme="dark"] button.btn-active {
      background: rgba(255, 255, 255, 0.12);
      border-color: rgba(255, 255, 255, 0.22);
      color: #ffffff;
    }

    [data-theme="dark"] .icon-btn:hover {
      background: rgba(255, 255, 255, 0.08);
      border-color: rgba(255, 255, 255, 0.1);
      color: #ededed;
    }

    /* Raycast Search Inputs */
    [data-theme="dark"] .binder-search-input,
    [data-theme="dark"] #lore-search-input,
    [data-theme="dark"] .form-group input,
    [data-theme="dark"] .form-group textarea,
    [data-theme="dark"] .form-group select,
    [data-theme="dark"] #split-reference-select {
      background: rgba(255, 255, 255, 0.04);
      border: 1px solid rgba(255, 255, 255, 0.08);
      color: #ededed;
    }

    [data-theme="dark"] .binder-search-input:focus,
    [data-theme="dark"] #lore-search-input:focus,
    [data-theme="dark"] .form-group input:focus,
    [data-theme="dark"] .form-group textarea:focus,
    [data-theme="dark"] .form-group select:focus {
      background: rgba(255, 255, 255, 0.07);
      border-color: rgba(255, 255, 255, 0.35);
      box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.08);
    }

    /* Raycast Tabs */
    [data-theme="dark"] .tab-btn.active {
      color: #ffffff;
      border-bottom-color: #ffffff;
      background: transparent;
    }

    [data-theme="dark"] .filter-chip {
      background: rgba(255, 255, 255, 0.04);
      border-color: rgba(255, 255, 255, 0.08);
      color: #8a8c95;
    }

    [data-theme="dark"] .filter-chip.active {
      background: rgba(255, 255, 255, 0.16);
      border-color: rgba(255, 255, 255, 0.28);
      color: #ffffff;
    }

    /* Raycast Modals and Popups */
    [data-theme="dark"] .modal-overlay {
      background: rgba(0, 0, 0, 0.72);
      backdrop-filter: blur(8px);
      -webkit-backdrop-filter: blur(8px);
    }

    [data-theme="dark"] .modal-box,
    [data-theme="dark"] .mention-popup,
    [data-theme="dark"] .typo-dropdown-menu {
      background: #141519;
      border: 1px solid rgba(255, 255, 255, 0.11);
      box-shadow: 0 24px 60px -12px rgba(0, 0, 0, 0.85), 0 0 0 1px rgba(255, 255, 255, 0.06);
    }

    [data-theme="dark"] .typo-btn-group {
      background: rgba(255, 255, 255, 0.05);
    }

    [data-theme="dark"] .typo-btn.active {
      background: rgba(255, 255, 255, 0.12);
      color: #ffffff;
    }

    /* Raycast Manuscript & Typing Canvas */
    [data-theme="dark"] textarea.manuscript-editor {
      color: #e4e5ea;
      caret-color: #ffffff;
    }

    [data-theme="dark"] textarea.manuscript-editor::selection {
      background: rgba(255, 255, 255, 0.18);
    }

    [data-theme="dark"] .scrivenings-content {
      color: #e4e5ea;
    }

    [data-theme="dark"] .status-select-wrap select {
      background: rgba(255, 255, 255, 0.04);
      border: 1px solid rgba(255, 255, 255, 0.09);
      color: #ededed;
    }

    [data-theme="dark"] .scene-title-input:hover,
    [data-theme="dark"] .scene-title-input:focus {
      background: rgba(255, 255, 255, 0.05);
      border-color: rgba(255, 255, 255, 0.14);
    }

    [data-theme="dark"] .target-progress {
      background: rgba(255, 255, 255, 0.08);
    }

    /* Raycast Sleek Scrollbar */
    ::-webkit-scrollbar {
      width: 6px;
      height: 6px;
    }
    ::-webkit-scrollbar-track {
      background: transparent;
    }
    ::-webkit-scrollbar-thumb {
      background: rgba(0, 0, 0, 0.15);
      border-radius: 9999px;
    }
    [data-theme="dark"] ::-webkit-scrollbar-thumb {
      background: rgba(255, 255, 255, 0.12);
    }
    [data-theme="dark"] ::-webkit-scrollbar-thumb:hover {
      background: rgba(255, 255, 255, 0.22);
    }

    [data-theme="dark"] #split-reference-content {
      background: #15161a;
      border-color: rgba(255, 255, 255, 0.08);
      color: #e4e5ea;
    }

    /* Zen Focus Mode */
    body.zen-focus-active header.topbar {
      opacity: 0.15;
    }
    body.zen-focus-active header.topbar:hover {
      opacity: 1;
    }
    body.zen-focus-active .word-count-bar {
      opacity: 0.15;
    }
    body.zen-focus-active .word-count-bar:hover {
      opacity: 1;
    }
  </style>
</head>
<body>

  <!-- Top Navigation -->
  <header class="topbar">
    <div class="brand-section">
      <button class="icon-btn" id="btn-toggle-binder" title="바인더 사이드바 토글 (Cmd/Ctrl+B)">
        <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="9" y1="3" x2="9" y2="21"></line></svg>
      </button>
      <a href="/" class="brand-logo">
        <svg class="icon" viewBox="0 0 24 24"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path></svg>
        <span>Narratics</span>
      </a>
      <span class="brand-badge">Local-First</span>
      
      <div class="project-title-container">
        <input type="text" class="project-title-input" id="project-meta-title" value="달빛 아래의 크로니클.narr" title="클릭하여 작품 제목 변경" />
      </div>

      <div class="storage-status-pill" title="모든 수정사항이 SQLite WAL 저널에 즉시 원자적으로 영구 기록됩니다.">
        <div class="status-dot" id="sync-status-dot"></div>
        <span id="sync-status-text">로컬 안전 저장됨</span>
      </div>
    </div>

    <div class="top-actions">
      <!-- Zen Focus Mode -->
      <button class="btn" id="btn-zen-mode" title="방해 없는 집중 집필 모드 (단축키: Esc 또는 F11)">
        <svg class="icon" viewBox="0 0 24 24"><path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"></path></svg>
        <span>집중 집필</span>
        <kbd>F11</kbd>
      </button>

      <!-- Typewriter Mode Toggle -->
      <button class="btn" id="btn-typewriter-mode" title="타자기 모드: 입력 중인 커서 줄을 화면 중앙에 고정">
        <svg class="icon" viewBox="0 0 24 24"><polyline points="4 7 4 4 20 4 20 7"></polyline><line x1="9" y1="20" x2="15" y2="20"></line><line x1="12" y1="4" x2="12" y2="20"></line></svg>
        <span>타자기</span>
      </button>

      <!-- Typography Settings -->
      <button class="btn" id="btn-open-typo-menu" title="서체, 글자 크기, 줄 간격 조절">
        <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>
        <span>조판</span>
      </button>

      <!-- Theme Switcher -->
      <button class="btn" id="btn-theme-toggle" title="다크/라이트 테마 전환">
        <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="5"></circle><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"></path></svg>
        <span>테마</span>
      </button>

      <!-- Export Button -->
      <button class="btn btn-primary" id="btn-export-open" title="원고 내보내기 및 출판 규격 조판">
        <svg class="icon" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
        <span>내보내기</span>
        <kbd style="background:rgba(0,0,0,0.28); border-color:rgba(255,255,255,0.25); color:#ffffff; margin-left:3px;">⌘E</kbd>
      </button>

      <!-- Inspector Toggle -->
      <button class="icon-btn" id="btn-toggle-inspector" title="인스펙터 사이드바 토글 (Cmd/Ctrl+I)">
        <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="15" y1="3" x2="15" y2="21"></line></svg>
      </button>
    </div>
  </header>

  <!-- Typography Floating Menu -->
  <div class="typo-dropdown-menu" id="typo-dropdown-menu">
    <div class="typo-row">
      <span>서체</span>
      <div class="typo-btn-group">
        <button class="typo-btn active" id="btn-font-serif">명조체</button>
        <button class="typo-btn" id="btn-font-sans">고딕체</button>
      </div>
    </div>
    <div class="typo-row">
      <span>글자 크기</span>
      <div class="typo-btn-group">
        <button class="typo-btn" data-size="15px">15</button>
        <button class="typo-btn active" data-size="17px">17</button>
        <button class="typo-btn" data-size="19px">19</button>
        <button class="typo-btn" data-size="21px">21</button>
      </div>
    </div>
    <div class="typo-row">
      <span>줄 간격</span>
      <div class="typo-btn-group">
        <button class="typo-btn" data-lh="1.7">1.7</button>
        <button class="typo-btn active" data-lh="1.9">1.9</button>
        <button class="typo-btn" data-lh="2.2">2.2</button>
      </div>
    </div>
    <div class="typo-row">
      <span>원고지 폭</span>
      <div class="typo-btn-group">
        <button class="typo-btn" data-width="680px">좁게</button>
        <button class="typo-btn active" data-width="760px">표준</button>
        <button class="typo-btn" data-width="920px">넓게</button>
      </div>
    </div>
  </div>

  <!-- Main 3-Panel Workspace -->
  <div class="app-workspace">

    <!-- Left: Arc Binder -->
    <aside class="binder-panel" id="binder-panel">
      <div class="binder-header">
        <div class="binder-title-group">
          <h2>바인더 (Binder)</h2>
        </div>
        <div class="binder-actions">
          <button class="icon-btn" id="btn-add-folder" title="새 챕터(Chapter) 폴더 추가">
            <svg class="icon" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><line x1="9" y1="14" x2="15" y2="14"></line></svg>
          </button>
          <button class="icon-btn" id="btn-add-scene" title="새 씬(Scene) 추가">
            <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="12" y1="18" x2="12" y2="12"></line><line x1="9" y1="15" x2="15" y2="15"></line></svg>
          </button>
        </div>
      </div>

      <!-- Quick Search inside Binder -->
      <div class="binder-search-wrap">
        <input type="text" class="binder-search-input" id="binder-search-input" placeholder="씬 또는 챕터 검색..." />
        <kbd style="position:absolute; right:20px; pointer-events:none;">/</kbd>
      </div>

      <div class="binder-tree" id="binder-tree-container">
        <!-- Dynamic Tree Items -->
      </div>

      <!-- Binder Aggregate Footer -->
      <div class="binder-footer">
        <span id="binder-total-scenes">총 0개 씬</span>
        <span id="binder-total-words">0자 (원고지 0.0매)</span>
      </div>
    </aside>

    <!-- Center: The Manuscript Studio -->
    <main class="studio-panel">
      <div class="studio-header">
        <div class="scene-meta-group">
          <input type="text" class="scene-title-input" id="current-scene-title" value="" placeholder="씬 제목을 입력하세요..." />
          <div class="status-select-wrap">
            <select id="current-scene-status-select" title="집필 진행 상태">
              <option value="초고">초고 (Draft)</option>
              <option value="수정중">수정중 (In Progress)</option>
              <option value="퇴고완료">퇴고완료 (Revised)</option>
              <option value="탈고">탈고 (Final)</option>
            </select>
          </div>
        </div>
        <div class="studio-controls">
          <button class="btn" id="btn-toggle-scrivenings">
            <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="3" y1="9" x2="21" y2="9"></line><line x1="9" y1="21" x2="9" y2="9"></line></svg>
            <span id="scrivenings-label">스크리브닝스 (연속 뷰)</span>
          </button>
        </div>
      </div>

      <!-- Live Korean Word & Character Count Bar -->
      <div class="word-count-bar">
        <div class="stat-pills">
          <span class="stat-pill">공백 포함: <strong id="stat-chars-with-space">0</strong>자</span>
          <span class="stat-pill">공백 제외: <strong id="stat-chars-no-space">0</strong>자</span>
          <span class="stat-pill">단어수: <strong id="stat-words">0</strong>개</span>
          <span class="stat-pill">원고지: <strong id="stat-manuscript-pages">0.0</strong>매 (200자)</span>
          <span class="stat-pill">예상 완독: <strong id="stat-reading-time">0분</strong></span>
        </div>
        <div class="target-bar-wrap">
          <div class="autosave-notice">
            <svg class="icon" style="width:12px; height:12px; color:var(--success);" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"></polyline></svg>
            <span id="autosave-label">방금 저장됨</span>
          </div>
          <span>목표 <strong id="stat-target-pct">0%</strong></span>
          <div class="target-progress">
            <div class="target-fill" id="stat-target-fill"></div>
          </div>
        </div>
      </div>

      <!-- Editor Canvas -->
      <div class="editor-container" id="editor-container">
        <div class="editor-paper">
          <textarea id="manuscript-text-editor" class="manuscript-editor" placeholder="이곳에 서사를 펼치세요...&#10;&#10;* 팁: 세계관 인물이나 설정을 삽입하려면 '@' 키를 입력하세요.&#10;* 방해 없는 집필을 원하시면 상단의 [집중 집필]을 누르세요."></textarea>
          <div id="scrivenings-container" class="scrivenings-view"></div>
        </div>
      </div>
    </main>

    <!-- Right: LoreDeck & Inspector -->
    <aside class="inspector-panel" id="inspector-panel">
      <div class="tab-nav">
        <button class="tab-btn active" data-tab="tab-lore">세계관 (로어덱)</button>
        <button class="tab-btn" data-tab="tab-snapshots">스냅샷 타임머신</button>
        <button class="tab-btn" data-tab="tab-split">분할 참고</button>
      </div>

      <!-- Tab 1: LoreDeck -->
      <div class="tab-content active" id="tab-lore">
        <div style="display:flex; justify-content:space-between; align-items:center; gap:8px;">
          <div style="position:relative; flex:1; display:flex; align-items:center;">
            <input type="text" id="lore-search-input" placeholder="인물·장소·설정 검색..." style="width:100%; padding:6px 28px 6px 10px; border:1px solid var(--border); border-radius:6px; background:var(--bg); color:var(--text); font-size:12px;" />
            <kbd style="position:absolute; right:8px; pointer-events:none;">@</kbd>
          </div>
          <button class="btn btn-primary" id="btn-add-lore" style="padding:5px 9px;">+ 설정</button>
        </div>

        <div class="category-filter-chips" id="lore-category-chips">
          <div class="filter-chip active" data-cat="all">전체</div>
          <div class="filter-chip" data-cat="등장인물">등장인물</div>
          <div class="filter-chip" data-cat="장소 및 세력">장소·세력</div>
          <div class="filter-chip" data-cat="세계관 설정">규칙·설정</div>
          <div class="filter-chip" data-cat="복선 및 아이템">아이템·복선</div>
        </div>

        <div id="lore-list-container" style="display:flex; flex-direction:column; gap:8px;">
          <!-- Dynamic Lore Cards -->
        </div>
      </div>

      <!-- Tab 2: Snapshots Time-Machine -->
      <div class="tab-content" id="tab-snapshots">
        <div style="display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-size:12px; font-weight:600;">퇴고 전 시점 보존</div>
            <div style="font-size:11px; color:var(--text-subtle);">언제든 이전 시점으로 되돌립니다.</div>
          </div>
          <button class="btn btn-primary" id="btn-create-snapshot" style="padding:5px 9px;">+ 스냅샷</button>
        </div>

        <div id="snapshot-list-container" style="display:flex; flex-direction:column; gap:8px;">
          <!-- Dynamic Snapshot Cards -->
        </div>
      </div>

      <!-- Tab 3: Split Reference View -->
      <div class="tab-content" id="tab-split">
        <div style="font-size:12px; font-weight:600; margin-bottom:6px;">집필 참고 패널</div>
        <p style="font-size:11px; color:var(--text-muted); margin-bottom:10px;">왼쪽에서 원고를 쓰면서 동시에 확인할 설정이나 이전 씬을 선택하세요.</p>
        <select id="split-reference-select" style="width:100%; padding:6px; font-size:12px; border:1px solid var(--border); border-radius:6px; background:var(--bg); color:var(--text); margin-bottom:12px;">
          <option value="">참고할 씬 또는 인물 선택...</option>
        </select>
        <div id="split-reference-content" style="background:var(--bg); border:1px solid var(--border); border-radius:8px; padding:12px; font-size:13px; line-height:1.7; font-family:var(--font-serif); white-space:pre-wrap; max-height:450px; overflow-y:auto;">
          참고할 대상을 위에서 선택하면 이곳에 상시 고정 표시됩니다.
        </div>
      </div>
    </aside>

  </div>

  <!-- Floating @-mention Popup -->
  <div class="mention-popup" id="mention-popup"></div>

  <!-- Modal for Adding Folder / Scene -->
  <div class="modal-overlay" id="node-modal">
    <div class="modal-box">
      <div class="modal-title" id="node-modal-title">새 항목 추가</div>
      <div class="form-group">
        <label for="node-name-input">명칭</label>
        <input type="text" id="node-name-input" placeholder="예: 제3장: 결전의 날" />
      </div>
      <div class="form-group">
        <label for="node-parent-select">상위 폴더 (챕터)</label>
        <select id="node-parent-select">
          <option value="root">최상위 (루트)</option>
        </select>
      </div>
      <div class="modal-actions">
        <button class="btn" id="btn-node-cancel">취소</button>
        <button class="btn btn-primary" id="btn-node-save">생성 완료</button>
      </div>
    </div>
  </div>

  <!-- Modal for Adding Lore -->
  <div class="modal-overlay" id="lore-modal">
    <div class="modal-box">
      <div class="modal-title" id="lore-modal-title">새 세계관 설정 카드 등록</div>
      <div class="form-group">
        <label for="lore-cat-input">구분 (카테고리)</label>
        <select id="lore-cat-input">
          <option value="등장인물">등장인물</option>
          <option value="장소 및 세력">장소 및 세력</option>
          <option value="세계관 설정">세계관 설정</option>
          <option value="복선 및 아이템">복선 및 아이템</option>
        </select>
      </div>
      <div class="form-group">
        <label for="lore-name-input">이름 / 표제어</label>
        <input type="text" id="lore-name-input" placeholder="예: 세레나" />
      </div>
      <div class="form-group">
        <label for="lore-aliases-input">별칭 / 이명 (쉼표 구분 - 본문 @호출 시 사용)</label>
        <input type="text" id="lore-aliases-input" placeholder="예: 은빛 마녀, 제국 감찰관" />
      </div>
      <div class="form-group">
        <label for="lore-desc-input">설정 및 상세 시트 (나이, 성격, 핵심 복선, 목표)</label>
        <textarea id="lore-desc-input" rows="5" placeholder="인물의 성격, 외모, 숨겨진 비밀, 서사적 목표 등을 기록하세요."></textarea>
      </div>
      <div class="modal-actions">
        <button class="btn" id="btn-lore-cancel">취소</button>
        <button class="btn btn-primary" id="btn-lore-save">저장</button>
      </div>
    </div>
  </div>

  <!-- Modal for Creating Snapshot -->
  <div class="modal-overlay" id="snapshot-modal">
    <div class="modal-box">
      <div class="modal-title">시점 복원 스냅샷 생성</div>
      <p style="font-size:12px; color:var(--text-muted); margin-bottom:12px;">현재 씬의 본문과 글자수를 안전하게 보관합니다. 대규모 퇴고 전 스냅샷을 남겨두세요.</p>
      <div class="form-group">
        <label for="snapshot-label-input">스냅샷 메모 / 버전 명칭</label>
        <input type="text" id="snapshot-label-input" placeholder="예: 2화 전투 씬 퇴고 직전" />
      </div>
      <div class="modal-actions">
        <button class="btn" id="btn-snapshot-cancel">취소</button>
        <button class="btn btn-primary" id="btn-snapshot-save">스냅샷 생성</button>
      </div>
    </div>
  </div>

  <!-- Modal for Exporting Manuscript -->
  <div class="modal-overlay" id="export-modal">
    <div class="modal-box" style="width: 520px;">
      <div class="modal-title">원고 조판 및 내보내기</div>
      <p style="font-size:12px; color:var(--text-muted); margin-bottom:16px;">작품 전체 원고를 원하는 표준 출판 포맷으로 즉시 추출합니다.</p>
      
      <div style="display:flex; flex-direction:column; gap:10px; margin-bottom:16px;">
        <div style="border:1px solid var(--border); border-radius:8px; padding:12px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-weight:600; font-size:13px;">표준 웹소설 텍스트 (.txt)</div>
            <div style="font-size:11px; color:var(--text-muted);">문피아, 카카오페이지, 노벨피아 연재 업로드용 화수별 텍스트</div>
          </div>
          <button class="btn" id="btn-export-txt">TXT 다운로드</button>
        </div>

        <div style="border:1px solid var(--border); border-radius:8px; padding:12px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-weight:600; font-size:13px;">통합 마크다운 원고 (.md)</div>
            <div style="font-size:11px; color:var(--text-muted);">전체 챕터 및 씬이 구조화된 단일 Markdown 문서</div>
          </div>
          <button class="btn" id="btn-export-md">MD 다운로드</button>
        </div>

        <div style="border:1px solid var(--border); border-radius:8px; padding:12px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-weight:600; font-size:13px;">Typst 단행본 조판 파일 (.typ)</div>
            <div style="font-size:11px; color:var(--text-muted);">국판/신국판 판형에 맞춘 출판용 벡터 조판 소스</div>
          </div>
          <button class="btn" id="btn-export-typ">Typst 다운로드</button>
        </div>

        <div style="border:1px solid var(--border); border-radius:8px; padding:12px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-weight:600; font-size:13px;">인쇄 및 PDF 내보내기</div>
            <div style="font-size:11px; color:var(--text-muted);">원고지 양식 및 A4 여백 맞춤 브라우저 인쇄</div>
          </div>
          <button class="btn" id="btn-export-print">인쇄/PDF 열기</button>
        </div>
      </div>

      <div class="modal-actions">
        <button class="btn" id="btn-export-close">닫기</button>
      </div>
    </div>
  </div>

  <script>
    // App State
    const clientId = "client_" + Math.random().toString(36).substring(2, 9);
    let ws = null;
    let state = {
      title: "달빛 아래의 크로니클",
      author: "최용철",
      binder: [],
      active_scene_id: "",
      scenes: {},
      lore: [],
      snapshots: []
    };
    let isScriveningsMode = false;
    let isZenMode = false;
    let isTypewriterMode = false;
    let currentLoreFilter = "all";
    let debounceTimer = null;
    let currentModalType = "scene";

    // DOM Elements
    const projectTitleEl = document.getElementById("project-meta-title");
    const syncDot = document.getElementById("sync-status-dot");
    const syncText = document.getElementById("sync-status-text");
    const binderPanel = document.getElementById("binder-panel");
    const inspectorPanel = document.getElementById("inspector-panel");
    const binderTreeEl = document.getElementById("binder-tree-container");
    const sceneTitleInput = document.getElementById("current-scene-title");
    const sceneStatusSelect = document.getElementById("current-scene-status-select");
    const editorTextarea = document.getElementById("manuscript-text-editor");
    const scriveningsContainer = document.getElementById("scrivenings-container");
    const statCharsWithSpace = document.getElementById("stat-chars-with-space");
    const statCharsNoSpace = document.getElementById("stat-chars-no-space");
    const statWords = document.getElementById("stat-words");
    const statPages = document.getElementById("stat-manuscript-pages");
    const statReadingTime = document.getElementById("stat-reading-time");
    const statTargetPct = document.getElementById("stat-target-pct");
    const statFill = document.getElementById("stat-target-fill");
    const autosaveLabel = document.getElementById("autosave-label");
    const loreListContainer = document.getElementById("lore-list-container");
    const snapshotListContainer = document.getElementById("snapshot-list-container");
    const typoDropdown = document.getElementById("typo-dropdown-menu");
    const mentionPopup = document.getElementById("mention-popup");

    // Init WebSocket
    function connectWs() {
      const proto = location.protocol === "https:" ? "wss:" : "ws:";
      const wsUrl = `${proto}//${location.host}/ws`;
      ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        syncDot.className = "status-dot";
        syncText.textContent = "로컬 안전 저장됨 (Online)";
      };

      ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          handleWsMessage(msg);
        } catch (e) {
          console.error("WS Parse Error:", e);
        }
      };

      ws.onclose = () => {
        syncDot.className = "status-dot offline";
        syncText.textContent = "오프라인 모드 (로컬 컨테이너 영구 보존)";
        setTimeout(connectWs, 2000);
      };

      ws.onerror = () => {
        ws.close();
      };
    }

    function handleWsMessage(msg) {
      if (msg.type === "init") {
        state = msg.state;
        renderAll();
      } else if (msg.type === "text_update") {
        if (state.scenes[msg.scene_id]) {
          state.scenes[msg.scene_id].text = msg.text;
          state.scenes[msg.scene_id].word_count = msg.word_count;
        }
        const b = state.binder.find(x => x.id === msg.scene_id);
        if (b) b.word_count = msg.word_count;

        if (state.active_scene_id === msg.scene_id && msg.client_id !== clientId) {
          const start = editorTextarea.selectionStart;
          const end = editorTextarea.selectionEnd;
          editorTextarea.value = msg.text;
          editorTextarea.setSelectionRange(start, end);
          updateWordCount(msg.text);
        }
        renderBinder();
        updateBinderFooter();
      } else if (msg.type === "tree_move") {
        fetchState();
      } else if (msg.type === "lore_update") {
        const idx = state.lore.findIndex(l => l.id === msg.lore.id);
        if (idx >= 0) {
          state.lore[idx] = msg.lore;
        } else {
          state.lore.push(msg.lore);
        }
        renderLore();
      } else if (msg.type === "snapshot_created") {
        if (!state.snapshots) state.snapshots = [];
        state.snapshots.unshift(msg.snapshot);
        renderSnapshots();
      } else if (msg.type === "scene_status_updated") {
        const b = state.binder.find(x => x.id === msg.scene_id);
        if (b) b.status = msg.status;
        if (state.scenes[msg.scene_id]) state.scenes[msg.scene_id].status = msg.status;
        if (state.active_scene_id === msg.scene_id) {
          sceneStatusSelect.value = msg.status;
        }
        renderBinder();
      }
    }

    async function fetchState() {
      try {
        const res = await fetch("/api/state");
        state = await res.json();
        renderAll();
      } catch (e) {
        console.error("Failed to fetch state:", e);
      }
    }

    // Render Master
    function renderAll() {
      projectTitleEl.value = `${state.title}.narr`;
      renderBinder();
      loadActiveScene();
      renderLore();
      renderSnapshots();
      updateSplitReferenceDropdown();
      updateBinderFooter();
    }

    // Render Binder
    function renderBinder(searchQuery = "") {
      binderTreeEl.innerHTML = "";
      const query = searchQuery.toLowerCase().trim();

      const rootFolders = state.binder.filter(b => b.is_folder);
      const rootScenes = state.binder.filter(b => !b.is_folder && (!b.parent || b.parent === "root"));

      let totalScenesCount = 0;
      let totalCharsCount = 0;

      // Group into folders
      rootFolders.forEach(folder => {
        const childScenes = state.binder.filter(b => b.parent === folder.id);
        const folderChars = childScenes.reduce((acc, c) => acc + (c.word_count || 0), 0);

        if (query && !folder.title.toLowerCase().includes(query) && !childScenes.some(c => c.title.toLowerCase().includes(query))) {
          return;
        }

        const folderWrap = document.createElement("div");
        folderWrap.className = "binder-folder-wrap";

        const folderHeader = document.createElement("div");
        folderHeader.className = "tree-item folder";
        folderHeader.innerHTML = `
          <svg class="icon folder-arrow" viewBox="0 0 24 24"><polyline points="6 9 12 15 18 9"></polyline></svg>
          <svg class="icon" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
          <span class="tree-item-title">${escapeHtml(folder.title)}</span>
          <span class="tree-item-count">${childScenes.length}화 · ${folderChars.toLocaleString()}자</span>
        `;

        const childContainer = document.createElement("div");
        childContainer.className = "folder-children";

        folderHeader.onclick = () => {
          folderHeader.classList.toggle("collapsed");
          childContainer.style.display = folderHeader.classList.contains("collapsed") ? "none" : "block";
        };

        folderWrap.appendChild(folderHeader);

        childScenes.forEach((scene, sIdx) => {
          if (query && !scene.title.toLowerCase().includes(query)) return;

          totalScenesCount++;
          totalCharsCount += (scene.word_count || 0);

          const isActive = scene.id === state.active_scene_id;
          const status = scene.status || "초고";
          const sceneDiv = document.createElement("div");
          sceneDiv.className = `tree-item depth-1 ${isActive ? "active" : ""}`;
          sceneDiv.innerHTML = `
            <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
            <span class="tree-item-title">${escapeHtml(scene.title)}</span>
            <span class="status-pill-badge status-${status}" title="진행 상태 변경">${status}</span>
            <span class="tree-item-count">${(scene.word_count || 0).toLocaleString()}자</span>
            <div class="tree-item-tools">
              <button class="tree-item-btn btn-reorder-up" title="위로 이동">▲</button>
              <button class="tree-item-btn btn-reorder-down" title="아래로 이동">▼</button>
            </div>
          `;

          sceneDiv.onclick = (e) => {
            if (e.target.closest(".status-pill-badge")) {
              cycleSceneStatus(scene.id, status);
              return;
            }
            if (e.target.closest(".btn-reorder-up")) {
              moveSceneRank(scene.id, folder.id, -1);
              return;
            }
            if (e.target.closest(".btn-reorder-down")) {
              moveSceneRank(scene.id, folder.id, 1);
              return;
            }
            selectScene(scene.id);
          };

          childContainer.appendChild(sceneDiv);
        });

        folderWrap.appendChild(childContainer);
        binderTreeEl.appendChild(folderWrap);
      });

      // Root loose scenes
      rootScenes.forEach((scene, sIdx) => {
        if (query && !scene.title.toLowerCase().includes(query)) return;

        totalScenesCount++;
        totalCharsCount += (scene.word_count || 0);

        const isActive = scene.id === state.active_scene_id;
        const status = scene.status || "초고";
        const sceneDiv = document.createElement("div");
        sceneDiv.className = `tree-item ${isActive ? "active" : ""}`;
        sceneDiv.innerHTML = `
          <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
          <span class="tree-item-title">${escapeHtml(scene.title)}</span>
          <span class="status-pill-badge status-${status}">${status}</span>
          <span class="tree-item-count">${(scene.word_count || 0).toLocaleString()}자</span>
        `;
        sceneDiv.onclick = (e) => {
          if (e.target.closest(".status-pill-badge")) {
            cycleSceneStatus(scene.id, status);
            return;
          }
          selectScene(scene.id);
        };
        binderTreeEl.appendChild(sceneDiv);
      });

      if (totalScenesCount === 0 && !query) {
        binderTreeEl.innerHTML = `
          <div style="padding: 24px 16px; text-align: center; color: var(--text-muted); font-size: 12px;">
            <p style="margin-bottom: 8px;">아직 작성된 씬이 없습니다.</p>
            <button class="btn btn-primary" id="btn-empty-add-scene">+ 새 씬 추가하기</button>
          </div>
        `;
        const btnEmpty = document.getElementById("btn-empty-add-scene");
        if (btnEmpty) btnEmpty.onclick = () => document.getElementById("btn-add-scene").click();
      }
    }

    function updateBinderFooter() {
      const scenes = state.binder.filter(b => !b.is_folder);
      const totalChars = scenes.reduce((acc, s) => acc + (s.word_count || 0), 0);
      const totalPages = (totalChars / 200).toFixed(1);
      document.getElementById("binder-total-scenes").textContent = `총 ${scenes.length}개 씬`;
      document.getElementById("binder-total-words").textContent = `${totalChars.toLocaleString()}자 (원고지 ${totalPages}매)`;
    }

    // Binder Search
    document.getElementById("binder-search-input").addEventListener("input", (e) => {
      renderBinder(e.target.value);
    });

    function selectScene(sceneId) {
      if (state.active_scene_id === sceneId) return;
      state.active_scene_id = sceneId;
      renderBinder(document.getElementById("binder-search-input").value);
      loadActiveScene();
    }

    function loadActiveScene() {
      const scene = state.scenes[state.active_scene_id];
      if (!scene) {
        // Pick first available scene if active is invalid
        const first = state.binder.find(b => !b.is_folder);
        if (first) {
          state.active_scene_id = first.id;
          loadActiveScene();
        }
        return;
      }

      sceneTitleInput.value = scene.title;
      sceneStatusSelect.value = scene.status || "초고";
      editorTextarea.value = scene.text || "";
      updateWordCount(scene.text || "");

      if (isScriveningsMode) {
        renderScrivenings();
      }
    }

    // Word & Korean Novel Count calculation
    function updateWordCount(text) {
      const charsWithSpace = text.length;
      const charsNoSpace = text.replace(/\s+/g, "").length;
      const words = text.trim() ? text.trim().split(/\s+/).length : 0;
      const manuscriptPages = (charsWithSpace / 200).toFixed(1);
      const readingMinutes = Math.max(1, Math.round(charsWithSpace / 500)); // ~500 chars/min for Korean webnovel

      statCharsWithSpace.textContent = charsWithSpace.toLocaleString();
      statCharsNoSpace.textContent = charsNoSpace.toLocaleString();
      statWords.textContent = words.toLocaleString();
      statPages.textContent = manuscriptPages;
      statReadingTime.textContent = `${readingMinutes}분`;

      const targetGoal = 1000;
      const progressPct = Math.min(100, Math.round((charsWithSpace / targetGoal) * 100));
      statTargetPct.textContent = `${progressPct}%`;
      statFill.style.width = `${progressPct}%`;
    }

    // Keystroke Debounced Sync
    editorTextarea.addEventListener("input", (e) => {
      const newText = editorTextarea.value;
      updateWordCount(newText);

      // Autosave indicator
      autosaveLabel.textContent = "저장 중...";
      syncDot.className = "status-dot saving";

      // Local state update
      if (state.scenes[state.active_scene_id]) {
        state.scenes[state.active_scene_id].text = newText;
        state.scenes[state.active_scene_id].word_count = newText.length;
      }
      const b = state.binder.find(x => x.id === state.active_scene_id);
      if (b) b.word_count = newText.length;
      renderBinder(document.getElementById("binder-search-input").value);
      updateBinderFooter();

      // Check for @mention trigger
      handleMentionTrigger(e);

      // Typewriter scrolling
      if (isTypewriterMode) {
        scrollTypewriter();
      }

      clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        saveSceneText(state.active_scene_id, newText);
      }, 250);
    });

    async function saveSceneText(sceneId, text) {
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: "text_update",
          scene_id: sceneId,
          text: text,
          word_count: text.length,
          client_id: clientId
        }));
      }

      try {
        await fetch(`/api/scenes/${sceneId}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ text: text })
        });
        autosaveLabel.textContent = "방금 저장됨";
        syncDot.className = "status-dot";
      } catch (e) {
        console.warn("Rest save fallback error:", e);
        autosaveLabel.textContent = "로컬 보존됨";
      }
    }

    // Typewriter centering
    function scrollTypewriter() {
      const textarea = editorTextarea;
      const lineHeight = parseFloat(getComputedStyle(textarea).lineHeight) || 32;
      const cursorPos = textarea.selectionStart;
      const textBeforeCursor = textarea.value.substring(0, cursorPos);
      const lines = textBeforeCursor.split("\n").length;
      const cursorY = lines * lineHeight;
      const container = document.getElementById("editor-container");
      const targetScroll = cursorY - container.clientHeight * 0.4;
      container.scrollTop = Math.max(0, targetScroll);
    }

    // @-Mention Autocomplete Handling
    function handleMentionTrigger(e) {
      const pos = editorTextarea.selectionStart;
      const text = editorTextarea.value;
      const lastAt = text.lastIndexOf("@", pos - 1);

      if (lastAt !== -1 && pos - lastAt <= 12) {
        const query = text.substring(lastAt + 1, pos).toLowerCase();
        const matches = state.lore.filter(l => 
          l.name.toLowerCase().includes(query) || (l.aliases && l.aliases.some(a => a.toLowerCase().includes(query)))
        );

        if (matches.length > 0) {
          showMentionPopup(matches, lastAt, pos);
          return;
        }
      }
      hideMentionPopup();
    }

    function showMentionPopup(matches, startPos, endPos) {
      mentionPopup.innerHTML = "";
      matches.slice(0, 6).forEach((item, idx) => {
        const div = document.createElement("div");
        div.className = `mention-item ${idx === 0 ? "selected" : ""}`;
        div.innerHTML = `
          <span>${escapeHtml(item.name)}</span>
          <span class="mention-cat">${escapeHtml(item.category)}</span>
        `;
        div.onclick = () => {
          insertMention(item.name, startPos, endPos);
        };
        mentionPopup.appendChild(div);
      });

      // Position near center
      const editorRect = editorTextarea.getBoundingClientRect();
      mentionPopup.style.left = `${editorRect.left + 80}px`;
      mentionPopup.style.top = `${editorRect.top + 120}px`;
      mentionPopup.style.display = "block";
    }

    function hideMentionPopup() {
      mentionPopup.style.display = "none";
    }

    function insertMention(name, startPos, endPos) {
      const before = editorTextarea.value.substring(0, startPos);
      const after = editorTextarea.value.substring(endPos);
      editorTextarea.value = `${before}${name}${after}`;
      const newCursor = startPos + name.length;
      editorTextarea.setSelectionRange(newCursor, newCursor);
      hideMentionPopup();
      editorTextarea.focus();
      editorTextarea.dispatchEvent(new Event("input"));
    }

    // Scene Status Dropdown
    sceneStatusSelect.addEventListener("change", async () => {
      const newStatus = sceneStatusSelect.value;
      const sceneId = state.active_scene_id;
      if (!sceneId) return;

      const scene = state.binder.find(b => b.id === sceneId);
      if (scene) scene.status = newStatus;
      if (state.scenes[sceneId]) state.scenes[sceneId].status = newStatus;
      renderBinder(document.getElementById("binder-search-input").value);

      await fetch(`/api/scenes/${sceneId}/status`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ status: newStatus })
      });
    });

    async function cycleSceneStatus(sceneId, currentStatus) {
      const statuses = ["초고", "수정중", "퇴고완료", "탈고"];
      const nextIdx = (statuses.indexOf(currentStatus) + 1) % statuses.length;
      const nextStatus = statuses[nextIdx];

      const scene = state.binder.find(b => b.id === sceneId);
      if (scene) scene.status = nextStatus;
      if (state.scenes[sceneId]) state.scenes[sceneId].status = nextStatus;
      if (state.active_scene_id === sceneId) {
        sceneStatusSelect.value = nextStatus;
      }
      renderBinder(document.getElementById("binder-search-input").value);

      await fetch(`/api/scenes/${sceneId}/status`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ status: nextStatus })
      });
    }

    // Scene Rank Reordering
    async function moveSceneRank(sceneId, parentId, direction) {
      const siblings = state.binder.filter(b => b.parent === parentId);
      const curIdx = siblings.findIndex(b => b.id === sceneId);
      if (curIdx < 0) return;
      const targetIdx = curIdx + direction;
      if (targetIdx < 0 || targetIdx >= siblings.length) return;

      const targetSibling = siblings[targetIdx];
      const curScene = siblings[curIdx];

      // Swap ranks
      const tempRank = curScene.rank;
      curScene.rank = targetSibling.rank;
      targetSibling.rank = tempRank;

      renderBinder(document.getElementById("binder-search-input").value);

      await fetch("/api/binder/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          child: curScene.id,
          parent: parentId,
          rank: curScene.rank,
          title: curScene.title
        })
      });
    }

    // Rename Scene Title
    sceneTitleInput.addEventListener("change", async () => {
      const newTitle = sceneTitleInput.value.trim();
      if (!newTitle) return;

      const scene = state.binder.find(b => b.id === state.active_scene_id);
      if (scene) {
        scene.title = newTitle;
        if (state.scenes[state.active_scene_id]) {
          state.scenes[state.active_scene_id].title = newTitle;
        }
        renderBinder(document.getElementById("binder-search-input").value);

        await fetch("/api/binder/move", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            child: scene.id,
            parent: scene.parent || "root",
            rank: scene.rank,
            title: newTitle
          })
        });
      }
    });

    // Rename Project Title
    projectTitleEl.addEventListener("change", async () => {
      let newTitle = projectTitleEl.value.trim();
      if (newTitle.endsWith(".narr")) {
        newTitle = newTitle.slice(0, -5);
      }
      if (!newTitle) return;
      state.title = newTitle;
      projectTitleEl.value = `${newTitle}.narr`;
    });

    // Scrivenings Mode Toggle
    const btnToggleScrivenings = document.getElementById("btn-toggle-scrivenings");
    const scriveningsLabel = document.getElementById("scrivenings-label");
    btnToggleScrivenings.addEventListener("click", () => {
      isScriveningsMode = !isScriveningsMode;
      if (isScriveningsMode) {
        editorTextarea.style.display = "none";
        scriveningsContainer.style.display = "block";
        scriveningsLabel.textContent = "단일 씬 집필";
        btnToggleScrivenings.classList.add("btn-active");
        renderScrivenings();
      } else {
        editorTextarea.style.display = "block";
        scriveningsContainer.style.display = "none";
        scriveningsLabel.textContent = "스크리브닝스 (연속 뷰)";
        btnToggleScrivenings.classList.remove("btn-active");
      }
    });

    function renderScrivenings() {
      scriveningsContainer.innerHTML = "";
      const scenes = state.binder.filter(b => !b.is_folder);
      scenes.forEach(scene => {
        const text = state.scenes[scene.id]?.text || "";
        const sec = document.createElement("div");
        sec.className = "scrivenings-section";
        sec.innerHTML = `
          <div class="scrivenings-header">
            <span class="scrivenings-title"># ${escapeHtml(scene.title)}</span>
            <span class="scrivenings-meta">${(scene.word_count || 0).toLocaleString()}자 · 상태: ${scene.status || "초고"}</span>
          </div>
          <div class="scrivenings-content">${escapeHtml(text)}</div>
        `;
        scriveningsContainer.appendChild(sec);
      });
    }

    // Zen Focus Mode Toggle
    const btnZenMode = document.getElementById("btn-zen-mode");
    btnZenMode.addEventListener("click", toggleZenMode);

    function toggleZenMode() {
      isZenMode = !isZenMode;
      document.body.classList.toggle("zen-focus-active", isZenMode);
      if (isZenMode) {
        binderPanel.classList.add("collapsed");
        inspectorPanel.classList.add("collapsed");
        btnZenMode.classList.add("btn-active");
      } else {
        binderPanel.classList.remove("collapsed");
        inspectorPanel.classList.remove("collapsed");
        btnZenMode.classList.remove("btn-active");
      }
    }

    // Typewriter Mode Toggle
    const btnTypewriter = document.getElementById("btn-typewriter-mode");
    btnTypewriter.addEventListener("click", () => {
      isTypewriterMode = !isTypewriterMode;
      editorTextarea.classList.toggle("typewriter-mode", isTypewriterMode);
      btnTypewriter.classList.toggle("btn-active", isTypewriterMode);
      if (isTypewriterMode) scrollTypewriter();
    });

    // Typography Dropdown
    const btnOpenTypo = document.getElementById("btn-open-typo-menu");
    btnOpenTypo.addEventListener("click", (e) => {
      e.stopPropagation();
      typoDropdown.style.display = typoDropdown.style.display === "flex" ? "none" : "flex";
    });
    document.addEventListener("click", (e) => {
      if (!typoDropdown.contains(e.target) && e.target !== btnOpenTypo) {
        typoDropdown.style.display = "none";
      }
    });

    // Font family toggle
    const btnFontSerif = document.getElementById("btn-font-serif");
    const btnFontSans = document.getElementById("btn-font-sans");
    btnFontSerif.onclick = () => {
      btnFontSerif.classList.add("active");
      btnFontSans.classList.remove("active");
      editorTextarea.style.fontFamily = "var(--font-serif)";
    };
    btnFontSans.onclick = () => {
      btnFontSans.classList.add("active");
      btnFontSerif.classList.remove("active");
      editorTextarea.style.fontFamily = "var(--font-sans)";
    };

    // Font size buttons
    document.querySelectorAll("[data-size]").forEach(btn => {
      btn.onclick = () => {
        document.querySelectorAll("[data-size]").forEach(b => b.classList.remove("active"));
        btn.classList.add("active");
        editorTextarea.style.fontSize = btn.dataset.size;
      };
    });

    // Line height buttons
    document.querySelectorAll("[data-lh]").forEach(btn => {
      btn.onclick = () => {
        document.querySelectorAll("[data-lh]").forEach(b => b.classList.remove("active"));
        btn.classList.add("active");
        editorTextarea.style.lineHeight = btn.dataset.lh;
      };
    });

    // Paper width buttons
    document.querySelectorAll("[data-width]").forEach(btn => {
      btn.onclick = () => {
        document.querySelectorAll("[data-width]").forEach(b => b.classList.remove("active"));
        btn.classList.add("active");
        document.querySelector(".editor-paper").style.maxWidth = btn.dataset.width;
      };
    });

    // Theme Switcher (Raycast Dark Mode Default)
    const btnThemeToggle = document.getElementById("btn-theme-toggle");
    let currentTheme = localStorage.getItem("narratics_theme") || "dark";
    document.documentElement.setAttribute("data-theme", currentTheme);
    let isDark = currentTheme === "dark";

    btnThemeToggle.addEventListener("click", () => {
      isDark = !isDark;
      currentTheme = isDark ? "dark" : "light";
      document.documentElement.setAttribute("data-theme", currentTheme);
      localStorage.setItem("narratics_theme", currentTheme);
    });

    // Sidebars Toggle
    document.getElementById("btn-toggle-binder").addEventListener("click", () => {
      binderPanel.classList.toggle("collapsed");
    });
    document.getElementById("btn-toggle-inspector").addEventListener("click", () => {
      inspectorPanel.classList.toggle("collapsed");
    });

    // Tab Navigation in Inspector
    document.querySelectorAll(".tab-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        document.querySelectorAll(".tab-btn").forEach(b => b.classList.remove("active"));
        document.querySelectorAll(".tab-content").forEach(c => c.classList.remove("active"));
        btn.classList.add("active");
        document.getElementById(btn.dataset.tab).classList.add("active");
      });
    });

    // Lore List Rendering
    function renderLore(searchFilter = "") {
      loreListContainer.innerHTML = "";
      const filter = searchFilter.toLowerCase().trim();

      const filtered = state.lore.filter(l => {
        const matchesCategory = currentLoreFilter === "all" || l.category === currentLoreFilter;
        if (!matchesCategory) return false;
        if (!filter) return true;
        return l.name.toLowerCase().includes(filter) || 
               l.content.toLowerCase().includes(filter) ||
               (l.aliases && l.aliases.some(a => a.toLowerCase().includes(filter)));
      });

      if (filtered.length === 0) {
        loreListContainer.innerHTML = `
          <div style="padding: 20px 12px; text-align: center; color: var(--text-muted); font-size: 12px;">
            등록된 세계관 설정이 없습니다.<br>
            [+ 설정]을 눌러 인물이나 장소를 추가하세요.
          </div>
        `;
        return;
      }

      filtered.forEach(lore => {
        const card = document.createElement("div");
        card.className = "lore-card";
        const aliasesHtml = (lore.aliases || []).map(a => `<span class="lore-alias-pill">${escapeHtml(a)}</span>`).join("");
        card.innerHTML = `
          <div class="lore-card-header">
            <span class="lore-name">${escapeHtml(lore.name)}</span>
            <span class="lore-category-tag">${escapeHtml(lore.category)}</span>
          </div>
          ${aliasesHtml ? `<div class="lore-aliases">${aliasesHtml}</div>` : ""}
          <p class="lore-content-text">${escapeHtml(lore.content)}</p>
        `;
        card.onclick = () => openLoreDetail(lore);
        loreListContainer.appendChild(card);
      });
    }

    // Lore filter chips
    document.querySelectorAll(".filter-chip").forEach(chip => {
      chip.addEventListener("click", () => {
        document.querySelectorAll(".filter-chip").forEach(c => c.classList.remove("active"));
        chip.classList.add("active");
        currentLoreFilter = chip.dataset.cat;
        renderLore(document.getElementById("lore-search-input").value);
      });
    });

    document.getElementById("lore-search-input").addEventListener("input", (e) => {
      renderLore(e.target.value);
    });

    function openLoreDetail(lore) {
      document.getElementById("lore-cat-input").value = lore.category;
      document.getElementById("lore-name-input").value = lore.name;
      document.getElementById("lore-aliases-input").value = (lore.aliases || []).join(", ");
      document.getElementById("lore-desc-input").value = lore.content;
      document.getElementById("lore-modal-title").textContent = "세계관 설정 카드 수정";
      loreModal.style.display = "flex";
    }

    // Snapshots Rendering
    function renderSnapshots() {
      snapshotListContainer.innerHTML = "";
      const snapshots = state.snapshots || [];

      if (snapshots.length === 0) {
        snapshotListContainer.innerHTML = `
          <div style="padding: 20px 12px; text-align: center; color: var(--text-muted); font-size: 12px;">
            저장된 스냅샷이 없습니다.<br>
            중요한 퇴고 전에 [+ 스냅샷]을 생성하세요.
          </div>
        `;
        return;
      }

      snapshots.forEach(snap => {
        const card = document.createElement("div");
        card.className = "snapshot-card";
        const dateStr = new Date(snap.created_at).toLocaleString("ko-KR", { month: "numeric", day: "numeric", hour: "2-digit", minute: "2-digit" });
        card.innerHTML = `
          <div class="snapshot-header">
            <span class="snapshot-label">${escapeHtml(snap.label)}</span>
            <span class="snapshot-time">${dateStr}</span>
          </div>
          <div style="font-size: 11px; color: var(--text-muted);">
            대상 씬: <strong>${escapeHtml(snap.target_id)}</strong> · ${(snap.word_count || 0).toLocaleString()}자
          </div>
          <div class="snapshot-actions">
            <button class="btn btn-rollback" style="padding:3px 8px; font-size:11px;" data-id="${snap.id}">이 시점으로 복원</button>
          </div>
        `;
        card.querySelector(".btn-rollback").onclick = () => rollbackSnapshot(snap);
        snapshotListContainer.appendChild(card);
      });
    }

    async function rollbackSnapshot(snap) {
      const res = await fetch(`/api/scenes/${snap.target_id}/restore`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ text: snap.content, content: snap.content })
      });

      if (res.ok) {
        autosaveLabel.textContent = `'${snap.label}' 시점으로 복원 완료`;
        fetchState();
      }
    }

    // Split Reference Dropdown
    function updateSplitReferenceDropdown() {
      const select = document.getElementById("split-reference-select");
      select.innerHTML = `<option value="">참고할 씬 또는 인물 선택...</option>`;

      // Add other scenes
      const sceneGroup = document.createElement("optgroup");
      sceneGroup.label = "원고 씬";
      state.binder.filter(b => !b.is_folder).forEach(s => {
        const opt = document.createElement("option");
        opt.value = `scene:${s.id}`;
        opt.textContent = `[씬] ${s.title}`;
        sceneGroup.appendChild(opt);
      });
      select.appendChild(sceneGroup);

      // Add lore cards
      const loreGroup = document.createElement("optgroup");
      loreGroup.label = "세계관 설정";
      state.lore.forEach(l => {
        const opt = document.createElement("option");
        opt.value = `lore:${l.id}`;
        opt.textContent = `[${l.category}] ${l.name}`;
        loreGroup.appendChild(opt);
      });
      select.appendChild(loreGroup);
    }

    document.getElementById("split-reference-select").addEventListener("change", (e) => {
      const val = e.target.value;
      const refBox = document.getElementById("split-reference-content");
      if (!val) {
        refBox.textContent = "참고할 대상을 위에서 선택하면 이곳에 상시 고정 표시됩니다.";
        return;
      }

      const [type, id] = val.split(":");
      if (type === "scene") {
        const s = state.scenes[id];
        refBox.textContent = s ? `# ${s.title}\n\n${s.text || "(내용 없음)"}` : "(내용 없음)";
      } else if (type === "lore") {
        const l = state.lore.find(x => x.id === id);
        if (l) {
          refBox.textContent = `[${l.category}] ${l.name}\n별칭: ${(l.aliases || []).join(", ") || "-"}\n\n${l.content}`;
        }
      }
    });

    // Modals Handling
    const nodeModal = document.getElementById("node-modal");
    const nodeModalTitle = document.getElementById("node-modal-title");
    const nodeNameInput = document.getElementById("node-name-input");
    const nodeParentSelect = document.getElementById("node-parent-select");

    document.getElementById("btn-add-folder").addEventListener("click", () => {
      currentModalType = "folder";
      nodeModalTitle.textContent = "새 챕터(Chapter) 폴더 추가";
      nodeNameInput.value = "";
      updateParentSelect();
      nodeModal.style.display = "flex";
      nodeNameInput.focus();
    });

    document.getElementById("btn-add-scene").addEventListener("click", () => {
      currentModalType = "scene";
      nodeModalTitle.textContent = "새 씬(Scene) 추가";
      nodeNameInput.value = "";
      updateParentSelect();
      nodeModal.style.display = "flex";
      nodeNameInput.focus();
    });

    function updateParentSelect() {
      nodeParentSelect.innerHTML = `<option value="root">최상위 (루트)</option>`;
      state.binder.filter(b => b.is_folder).forEach(folder => {
        const opt = document.createElement("option");
        opt.value = folder.id;
        opt.textContent = folder.title;
        nodeParentSelect.appendChild(opt);
      });
    }

    document.getElementById("btn-node-cancel").addEventListener("click", () => {
      nodeModal.style.display = "none";
    });

    document.getElementById("btn-node-save").addEventListener("click", async () => {
      const title = nodeNameInput.value.trim();
      if (!title) return;

      const parent = nodeParentSelect.value;
      const isFolder = currentModalType === "folder";
      const id = (isFolder ? "chap_" : "scene_") + Date.now().toString(36);
      const rank = (state.binder.length * 10 + 10).toString();

      await fetch("/api/binder/move", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          child: id,
          parent: parent,
          rank: rank,
          title: title
        })
      });

      if (!isFolder) {
        await saveSceneText(id, "");
        state.active_scene_id = id;
      }

      nodeModal.style.display = "none";
      fetchState();
    });

    // Lore Modal
    const loreModal = document.getElementById("lore-modal");
    document.getElementById("btn-add-lore").addEventListener("click", () => {
      document.getElementById("lore-modal-title").textContent = "새 세계관 설정 카드 등록";
      document.getElementById("lore-name-input").value = "";
      document.getElementById("lore-aliases-input").value = "";
      document.getElementById("lore-desc-input").value = "";
      loreModal.style.display = "flex";
      document.getElementById("lore-name-input").focus();
    });
    document.getElementById("btn-lore-cancel").addEventListener("click", () => {
      loreModal.style.display = "none";
    });
    document.getElementById("btn-lore-save").addEventListener("click", async () => {
      const category = document.getElementById("lore-cat-input").value;
      const name = document.getElementById("lore-name-input").value.trim();
      const aliases = document.getElementById("lore-aliases-input").value.split(",").map(s => s.trim()).filter(Boolean);
      const content = document.getElementById("lore-desc-input").value.trim();
      if (!name) return;

      const newLore = {
        id: "lore_" + Date.now().toString(36),
        category,
        name,
        aliases,
        content,
        updated_at: Date.now()
      };

      await fetch("/api/lore", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(newLore)
      });

      loreModal.style.display = "none";
      fetchState();
    });

    // Snapshot Modal
    const snapshotModal = document.getElementById("snapshot-modal");
    document.getElementById("btn-create-snapshot").addEventListener("click", () => {
      document.getElementById("snapshot-label-input").value = "";
      snapshotModal.style.display = "flex";
      document.getElementById("snapshot-label-input").focus();
    });
    document.getElementById("btn-snapshot-cancel").addEventListener("click", () => {
      snapshotModal.style.display = "none";
    });
    document.getElementById("btn-snapshot-save").addEventListener("click", async () => {
      const label = document.getElementById("snapshot-label-input").value.trim() || "수동 스냅샷";
      const sceneId = state.active_scene_id;
      const scene = state.scenes[sceneId];
      if (!scene) return;

      const record = {
        id: "snap_" + Date.now().toString(36),
        target_id: sceneId,
        label,
        content: scene.text || "",
        word_count: (scene.text || "").length,
        created_at: Date.now()
      };

      await fetch("/api/snapshots", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(record)
      });

      snapshotModal.style.display = "none";
      fetchState();
    });

    // Export Modal
    const exportModal = document.getElementById("export-modal");
    document.getElementById("btn-export-open").addEventListener("click", () => {
      exportModal.style.display = "flex";
    });
    document.getElementById("btn-export-close").addEventListener("click", () => {
      exportModal.style.display = "none";
    });

    // Export Handlers
    document.getElementById("btn-export-txt").addEventListener("click", () => {
      let fullTxt = `[${state.title}]\n저자: ${state.author}\n\n`;
      state.binder.filter(b => !b.is_folder).forEach((b, i) => {
        fullTxt += `=== ${i + 1}화. ${b.title} ===\n\n${state.scenes[b.id]?.text || ""}\n\n\n`;
      });
      downloadFile(`${state.title}_연재원고.txt`, fullTxt, "text/plain;charset=utf-8");
    });

    document.getElementById("btn-export-md").addEventListener("click", () => {
      let fullMd = `# ${state.title}\n\n> 저자: ${state.author}\n> 생성: Narratics 서사 스튜디오\n\n`;
      state.binder.filter(b => !b.is_folder).forEach(b => {
        fullMd += `\n## ${b.title}\n\n${state.scenes[b.id]?.text || ""}\n\n---\n`;
      });
      downloadFile(`${state.title}_통합원고.md`, fullMd, "text/markdown;charset=utf-8");
    });

    document.getElementById("btn-export-typ").addEventListener("click", () => {
      let fullTyp = `// Narratics Typst Book Template\n#set document(title: "${state.title}", author: "${state.author}")\n#set page(paper: "a5", margin: (x: 2cm, y: 2.5cm))\n#set text(font: "KoPub Batang", size: 10.5pt, lang: "ko")\n#set par(justify: true, leading: 0.9em, first-line-indent: 1em)\n\n#align(center + horizon)[\n  #text(22pt, weight: "bold")[${state.title}]\n  #v(1em)\n  #text(12pt)[${state.author} 지음]\n]\n#pagebreak()\n\n`;
      state.binder.filter(b => !b.is_folder).forEach(b => {
        fullTyp += `= ${b.title}\n\n${state.scenes[b.id]?.text || ""}\n\n#pagebreak()\n\n`;
      });
      downloadFile(`${state.title}_출판조판.typ`, fullTyp, "text/plain;charset=utf-8");
    });

    document.getElementById("btn-export-print").addEventListener("click", () => {
      window.print();
    });

    function downloadFile(filename, content, mime) {
      const blob = new Blob([content], { type: mime });
      const a = document.createElement("a");
      a.href = URL.createObjectURL(blob);
      a.download = filename;
      a.click();
    }

    // Keyboard Shortcuts
    document.addEventListener("keydown", (e) => {
      // Escape closes modals or toggles Zen mode
      if (e.key === "Escape") {
        if (nodeModal.style.display === "flex") { nodeModal.style.display = "none"; return; }
        if (loreModal.style.display === "flex") { loreModal.style.display = "none"; return; }
        if (snapshotModal.style.display === "flex") { snapshotModal.style.display = "none"; return; }
        if (exportModal.style.display === "flex") { exportModal.style.display = "none"; return; }
        if (mentionPopup.style.display === "block") { hideMentionPopup(); return; }
        toggleZenMode();
      }

      // F11 for Zen Mode
      if (e.key === "F11") {
        e.preventDefault();
        toggleZenMode();
      }

      // Cmd/Ctrl+B for Binder toggle
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "b") {
        e.preventDefault();
        binderPanel.classList.toggle("collapsed");
      }

      // Cmd/Ctrl+I for Inspector toggle
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "i") {
        e.preventDefault();
        inspectorPanel.classList.toggle("collapsed");
      }
    });

    function escapeHtml(str) {
      return (str || "").replace(/[&<>"']/g, m => ({
        "&": "&amp;",
        "<": "&lt;",
        ">": "&gt;",
        '"': "&quot;",
        "'": "&#039;"
      })[m]);
    }

    // Startup
    connectWs();
    fetchState();
  </script>
</body>
</html>
"#;