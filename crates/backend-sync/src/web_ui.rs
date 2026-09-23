pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Narratics — Local-First Narrative Studio</title>
  <link rel="preconnect" href="https://cdn.jsdelivr.net" />
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/static/pretendard.min.css" />
  <style>
    :root {
      --bg: #ffffff;
      --bg-sidebar: #fbfbfa;
      --bg-hover: #f3f3f2;
      --bg-active: #ececec;
      --border: #e6e6e4;
      --text: #191919;
      --text-muted: #737373;
      --accent: #2563eb;
      --accent-light: #eff6ff;
      --danger: #dc2626;
      --success: #16a34a;
      --font-ui: "Pretendard", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --font-serif: "KoPub Batang", "Nanum Myeongjo", "Source Han Serif KR", "Noto Serif KR", Georgia, serif;
      --font-sans: "Pretendard", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    }

    [data-theme="dark"] {
      --bg: #121214;
      --bg-sidebar: #18181b;
      --bg-hover: #222226;
      --bg-active: #2a2a2f;
      --border: #2e2e34;
      --text: #f4f4f5;
      --text-muted: #a1a1aa;
      --accent: #3b82f6;
      --accent-light: #1e293b;
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
    }

    /* Top Navigation Header */
    header.topbar {
      height: 48px;
      border-bottom: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 0 16px;
      background: var(--bg);
      z-index: 10;
      flex-shrink: 0;
    }

    .brand-section {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    .brand-logo {
      font-weight: 700;
      font-size: 16px;
      letter-spacing: -0.5px;
      display: flex;
      align-items: center;
      gap: 6px;
      color: var(--text);
      text-decoration: none;
    }

    .brand-tag {
      font-size: 11px;
      padding: 2px 6px;
      border-radius: 4px;
      background: var(--accent-light);
      color: var(--accent);
      font-weight: 600;
      text-transform: uppercase;
    }

    .project-title-badge {
      font-weight: 500;
      color: var(--text-muted);
      border-left: 1px solid var(--border);
      padding-left: 12px;
      font-size: 13px;
    }

    .status-badge {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      font-size: 12px;
      padding: 3px 8px;
      border-radius: 9999px;
      border: 1px solid var(--border);
      color: var(--text-muted);
    }

    .status-dot {
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background: var(--success);
    }

    .status-dot.offline {
      background: #f59e0b;
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
      padding: 6px 10px;
      border-radius: 6px;
      border: 1px solid var(--border);
      background: var(--bg);
      color: var(--text);
      font-size: 12px;
      font-weight: 500;
      cursor: pointer;
      transition: background 0.15s ease, border-color 0.15s ease;
    }

    button.btn:hover {
      background: var(--bg-hover);
      border-color: var(--text-muted);
    }

    button.btn-primary {
      background: var(--accent);
      color: #ffffff;
      border-color: var(--accent);
    }

    button.btn-primary:hover {
      background: #1d4ed8;
      border-color: #1d4ed8;
    }

    /* Main Container (3 Panels) */
    .app-workspace {
      display: flex;
      flex: 1;
      overflow: hidden;
      position: relative;
    }

    /* Left Panel: Arc Binder */
    aside.binder-panel {
      width: 280px;
      border-right: 1px solid var(--border);
      background: var(--bg-sidebar);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
      user-select: none;
    }

    .binder-header {
      padding: 12px 14px;
      border-bottom: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: space-between;
    }

    .binder-header h2 {
      font-size: 12px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      color: var(--text-muted);
    }

    .binder-actions {
      display: flex;
      gap: 4px;
    }

    .binder-actions button {
      padding: 4px;
      border-radius: 4px;
      background: transparent;
      border: none;
      cursor: pointer;
      color: var(--text-muted);
    }

    .binder-actions button:hover {
      color: var(--text);
      background: var(--bg-hover);
    }

    .binder-tree {
      flex: 1;
      overflow-y: auto;
      padding: 8px 6px;
    }

    .tree-item {
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 6px 8px;
      border-radius: 6px;
      cursor: pointer;
      margin-bottom: 2px;
      font-size: 13px;
      transition: background 0.1s;
    }

    .tree-item:hover {
      background: var(--bg-hover);
    }

    .tree-item.active {
      background: var(--bg-active);
      font-weight: 600;
    }

    .tree-item.folder {
      font-weight: 600;
      margin-top: 4px;
      color: var(--text);
    }

    .tree-item.depth-1 {
      padding-left: 24px;
    }

    .tree-item.depth-2 {
      padding-left: 38px;
    }

    .tree-item-title {
      flex: 1;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .tree-item-count {
      font-size: 11px;
      color: var(--text-muted);
      font-variant-numeric: tabular-nums;
    }

    .tree-item-handle {
      opacity: 0;
      cursor: grab;
      color: var(--text-muted);
      display: flex;
    }

    .tree-item:hover .tree-item-handle {
      opacity: 0.8;
    }

    /* Center Panel: Studio Editor */
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
    }

    .scene-meta-group {
      display: flex;
      align-items: center;
      gap: 12px;
      flex: 1;
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
      width: 60%;
      font-family: inherit;
    }

    .scene-title-input:focus {
      border-color: var(--border);
      background: var(--bg-hover);
    }

    .studio-controls {
      display: flex;
      align-items: center;
      gap: 8px;
    }

    /* Stats Bar */
    .word-count-bar {
      padding: 6px 24px;
      border-bottom: 1px solid var(--border);
      background: var(--bg-sidebar);
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 12px;
      color: var(--text-muted);
    }

    .stat-pills {
      display: flex;
      gap: 16px;
    }

    .stat-pill strong {
      color: var(--text);
      font-weight: 600;
      margin-left: 2px;
    }

    .target-bar-wrap {
      display: flex;
      align-items: center;
      gap: 8px;
      width: 200px;
    }

    .target-progress {
      flex: 1;
      height: 6px;
      background: var(--border);
      border-radius: 3px;
      overflow: hidden;
    }

    .target-fill {
      height: 100%;
      background: var(--accent);
      width: 35%;
      transition: width 0.3s ease;
    }

    /* Editor Area */
    .editor-container {
      flex: 1;
      display: flex;
      overflow-y: auto;
      padding: 32px 48px;
      justify-content: center;
    }

    .editor-paper {
      width: 100%;
      max-width: 760px;
      min-height: 100%;
      display: flex;
      flex-direction: column;
    }

    textarea.manuscript-editor {
      width: 100%;
      flex: 1;
      border: none;
      outline: none;
      background: transparent;
      color: var(--text);
      font-family: var(--font-serif);
      font-size: 17px;
      line-height: 1.9;
      resize: none;
      white-space: pre-wrap;
      word-break: break-all;
      letter-spacing: -0.2px;
    }

    .scrivenings-view {
      display: none;
      width: 100%;
    }

    .scrivenings-section {
      margin-bottom: 32px;
      padding-bottom: 24px;
      border-bottom: 1px dashed var(--border);
    }

    .scrivenings-title {
      font-size: 16px;
      font-weight: 700;
      color: var(--text-muted);
      margin-bottom: 12px;
    }

    .scrivenings-content {
      font-family: var(--font-serif);
      font-size: 17px;
      line-height: 1.9;
      white-space: pre-wrap;
    }

    /* Right Panel: LoreDeck & Inspector */
    aside.inspector-panel {
      width: 300px;
      border-left: 1px solid var(--border);
      background: var(--bg-sidebar);
      display: flex;
      flex-direction: column;
      flex-shrink: 0;
    }

    .tab-nav {
      display: flex;
      border-bottom: 1px solid var(--border);
    }

    .tab-btn {
      flex: 1;
      padding: 10px;
      border: none;
      background: transparent;
      font-size: 12px;
      font-weight: 600;
      color: var(--text-muted);
      cursor: pointer;
      border-bottom: 2px solid transparent;
      transition: all 0.15s;
    }

    .tab-btn.active {
      color: var(--accent);
      border-bottom-color: var(--accent);
      background: var(--bg);
    }

    .tab-content {
      flex: 1;
      overflow-y: auto;
      padding: 12px;
      display: none;
    }

    .tab-content.active {
      display: block;
    }

    .lore-card {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 12px;
      margin-bottom: 10px;
      cursor: pointer;
      transition: border-color 0.15s, box-shadow 0.15s;
    }

    .lore-card:hover {
      border-color: var(--accent);
      box-shadow: 0 2px 8px rgba(0,0,0,0.04);
    }

    .lore-card-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 6px;
    }

    .lore-name {
      font-weight: 700;
      font-size: 14px;
    }

    .lore-category-tag {
      font-size: 11px;
      padding: 2px 6px;
      border-radius: 4px;
      background: var(--bg-hover);
      color: var(--text-muted);
    }

    .lore-content-text {
      font-size: 12px;
      color: var(--text-muted);
      line-height: 1.5;
    }

    .modal-overlay {
      position: fixed;
      inset: 0;
      background: rgba(0,0,0,0.4);
      display: none;
      align-items: center;
      justify-content: center;
      z-index: 100;
    }

    .modal-box {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 12px;
      width: 440px;
      max-width: 90%;
      padding: 20px;
      box-shadow: 0 10px 30px rgba(0,0,0,0.15);
    }

    .modal-title {
      font-size: 16px;
      font-weight: 700;
      margin-bottom: 12px;
    }

    .form-group {
      margin-bottom: 12px;
    }

    .form-group label {
      display: block;
      font-size: 12px;
      font-weight: 600;
      margin-bottom: 4px;
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
    }

    .form-group input:focus, .form-group textarea:focus {
      border-color: var(--accent);
    }

    .modal-actions {
      display: flex;
      justify-content: flex-end;
      gap: 8px;
      margin-top: 16px;
    }

    /* SVG icon utility */
    .icon {
      width: 16px;
      height: 16px;
      stroke-width: 2;
      stroke: currentColor;
      fill: none;
      stroke-linecap: round;
      stroke-linejoin: round;
      display: inline-block;
      vertical-align: middle;
    }
  </style>
</head>
<body>

  <!-- Top Navigation -->
  <header class="topbar">
    <div class="brand-section">
      <a href="/" class="brand-logo">
        <svg class="icon" viewBox="0 0 24 24"><path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path><path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path></svg>
        <span>Narratics</span>
      </a>
      <span class="brand-tag">Local-First</span>
      <span class="project-title-badge" id="project-meta-title">달빛 아래의 크로니클.narr</span>
      <div class="status-badge">
        <div class="status-dot" id="sync-status-dot"></div>
        <span id="sync-status-text">SQLite WAL 동기화 활성</span>
      </div>
    </div>

    <div class="top-actions">
      <button class="btn" id="btn-theme-toggle" title="다크 모드 전환">
        <svg class="icon" viewBox="0 0 24 24"><circle cx="12" cy="12" r="5"></circle><path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"></path></svg>
        <span>테마</span>
      </button>
      <button class="btn" id="btn-peer-simulate" title="새 탭에서 동시 편집 시뮬레이션">
        <svg class="icon" viewBox="0 0 24 24"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"></path><circle cx="9" cy="7" r="4"></circle><path d="M22 21v-2a4 4 0 0 0-3-3.87"></path><path d="M16 3.13a4 4 0 0 1 0 7.75"></path></svg>
        <span>피어 시뮬레이션</span>
      </button>
      <button class="btn btn-primary" id="btn-export-quick">
        <svg class="icon" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
        <span>원고 내보내기</span>
      </button>
    </div>
  </header>

  <!-- Main 3-Panel Workspace -->
  <div class="app-workspace">

    <!-- Left: Arc Binder -->
    <aside class="binder-panel">
      <div class="binder-header">
        <h2>바인더 (Binder)</h2>
        <div class="binder-actions">
          <button id="btn-add-folder" title="새 챕터/폴더 추가">
            <svg class="icon" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><line x1="9" y1="14" x2="15" y2="14"></line></svg>
          </button>
          <button id="btn-add-scene" title="새 씬 추가">
            <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline><line x1="12" y1="18" x2="12" y2="12"></line><line x1="9" y1="15" x2="15" y2="15"></line></svg>
          </button>
        </div>
      </div>
      <div class="binder-tree" id="binder-tree-container">
        <!-- Dynamic Tree Items -->
      </div>
    </aside>

    <!-- Center: The Studio -->
    <main class="studio-panel">
      <div class="studio-header">
        <div class="scene-meta-group">
          <input type="text" class="scene-title-input" id="current-scene-title" value="1화: 밤안개를 가르는 돛" />
        </div>
        <div class="studio-controls">
          <button class="btn" id="btn-font-toggle">
            <span id="font-style-label">명조체 (Serif)</span>
          </button>
          <button class="btn" id="btn-toggle-scrivenings">
            <svg class="icon" viewBox="0 0 24 24"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"></rect><line x1="3" y1="9" x2="21" y2="9"></line><line x1="9" y1="21" x2="9" y2="9"></line></svg>
            <span id="scrivenings-label">스크리브닝스 모드</span>
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
        </div>
        <div class="target-bar-wrap">
          <span>목표: <strong>1,000자</strong></span>
          <div class="target-progress">
            <div class="target-fill" id="stat-target-fill"></div>
          </div>
        </div>
      </div>

      <!-- Editor Canvas -->
      <div class="editor-container">
        <div class="editor-paper">
          <textarea id="manuscript-text-editor" class="manuscript-editor" placeholder="이곳에 서사를 펼치세요..."></textarea>
          <div id="scrivenings-container" class="scrivenings-view"></div>
        </div>
      </div>
    </main>

    <!-- Right: LoreDeck & Inspector -->
    <aside class="inspector-panel">
      <div class="tab-nav">
        <button class="tab-btn active" data-tab="tab-lore">로어덱 (세계관)</button>
        <button class="tab-btn" data-tab="tab-snapshots">스냅샷</button>
      </div>

      <div class="tab-content active" id="tab-lore">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
          <input type="text" id="lore-search-input" placeholder="인물·설정 검색..." style="flex:1; padding:6px 8px; border:1px solid var(--border); border-radius:6px; background:var(--bg); color:var(--text); font-size:12px; margin-right:6px;" />
          <button class="btn" id="btn-add-lore" style="padding:4px 8px;">+ 추가</button>
        </div>
        <div id="lore-list-container">
          <!-- Lore Cards -->
        </div>
      </div>

      <div class="tab-content" id="tab-snapshots">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
          <span style="font-size:12px; color:var(--text-muted);">원고 시점 복원</span>
          <button class="btn btn-primary" id="btn-create-snapshot" style="padding:4px 8px;">+ 스냅샷</button>
        </div>
        <div id="snapshot-list-container">
          <div class="lore-card">
            <div class="lore-card-header">
              <span class="lore-name">초고 완성 스냅샷</span>
              <span class="lore-category-tag">자동</span>
            </div>
            <p class="lore-content-text">2026-09-24 00:00 · 3개 씬 1,240자</p>
          </div>
        </div>
      </div>
    </aside>

  </div>

  <!-- Modal for Adding Folder / Scene -->
  <div class="modal-overlay" id="node-modal">
    <div class="modal-box">
      <div class="modal-title" id="node-modal-title">새 항목 추가</div>
      <div class="form-group">
        <label for="node-name-input">명칭</label>
        <input type="text" id="node-name-input" placeholder="예: 제3장: 결전의 날" />
      </div>
      <div class="form-group">
        <label for="node-parent-select">상위 폴더</label>
        <select id="node-parent-select">
          <option value="root">최상위 (Root)</option>
        </select>
      </div>
      <div class="modal-actions">
        <button class="btn" id="btn-node-cancel">취소</button>
        <button class="btn btn-primary" id="btn-node-save">생성</button>
      </div>
    </div>
  </div>

  <!-- Modal for Adding Lore -->
  <div class="modal-overlay" id="lore-modal">
    <div class="modal-box">
      <div class="modal-title">새 세계관 설정 추가</div>
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
        <label for="lore-aliases-input">별칭 / 이명 (쉼표 구분)</label>
        <input type="text" id="lore-aliases-input" placeholder="예: 은빛 마녀, 제국 감찰관" />
      </div>
      <div class="form-group">
        <label for="lore-desc-input">설명 및 설정 메모</label>
        <textarea id="lore-desc-input" rows="4" placeholder="캐릭터의 성격, 외모, 비밀, 복선 등을 기록하세요."></textarea>
      </div>
      <div class="modal-actions">
        <button class="btn" id="btn-lore-cancel">취소</button>
        <button class="btn btn-primary" id="btn-lore-save">저장</button>
      </div>
    </div>
  </div>

  <script>
    // Client State
    const clientId = "client_" + Math.random().toString(36).substring(2, 9);
    let ws = null;
    let state = {
      title: "달빛 아래의 크로니클",
      author: "최용철",
      binder: [],
      active_scene_id: "",
      scenes: {},
      lore: []
    };
    let isScriveningsMode = false;
    let debounceTimer = null;
    let currentModalType = "scene"; // 'folder' or 'scene'

    // DOM References
    const projectTitleEl = document.getElementById("project-meta-title");
    const syncDot = document.getElementById("sync-status-dot");
    const syncText = document.getElementById("sync-status-text");
    const binderTreeEl = document.getElementById("binder-tree-container");
    const sceneTitleInput = document.getElementById("current-scene-title");
    const editorTextarea = document.getElementById("manuscript-text-editor");
    const scriveningsContainer = document.getElementById("scrivenings-container");
    const statCharsWithSpace = document.getElementById("stat-chars-with-space");
    const statCharsNoSpace = document.getElementById("stat-chars-no-space");
    const statWords = document.getElementById("stat-words");
    const statPages = document.getElementById("stat-manuscript-pages");
    const statFill = document.getElementById("stat-target-fill");
    const loreListContainer = document.getElementById("lore-list-container");

    // Init WebSocket
    function connectWs() {
      const proto = location.protocol === "https:" ? "wss:" : "ws:";
      const wsUrl = `${proto}//${location.host}/ws`;
      ws = new WebSocket(wsUrl);

      ws.onopen = () => {
        syncDot.className = "status-dot";
        syncText.textContent = "SQLite WAL 동기화 활성 (Online)";
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
        syncText.textContent = "오프라인 모드 (로컬 캐시 보존됨)";
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
        // Update binder count
        const b = state.binder.find(x => x.id === msg.scene_id);
        if (b) b.word_count = msg.word_count;

        if (state.active_scene_id === msg.scene_id && msg.client_id !== clientId) {
          // If remote update arrived for currently open scene, sync editor cursor-safely
          const start = editorTextarea.selectionStart;
          const end = editorTextarea.selectionEnd;
          editorTextarea.value = msg.text;
          editorTextarea.setSelectionRange(start, end);
          updateWordCount(msg.text);
        }
        renderBinder();
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

    // Render Everything
    function renderAll() {
      projectTitleEl.textContent = `${state.title}.narr`;
      renderBinder();
      loadActiveScene();
      renderLore();
    }

    // Render Binder Tree
    function renderBinder() {
      binderTreeEl.innerHTML = "";

      // Group by chapters
      const rootFolders = state.binder.filter(b => b.is_folder);
      const rootScenes = state.binder.filter(b => !b.is_folder && (!b.parent || b.parent === "root"));

      rootFolders.forEach(folder => {
        const folderDiv = document.createElement("div");
        folderDiv.className = "tree-item folder depth-0";
        folderDiv.innerHTML = `
          <svg class="icon" viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>
          <span class="tree-item-title">${escapeHtml(folder.title)}</span>
        `;
        binderTreeEl.appendChild(folderDiv);

        // Child scenes
        const children = state.binder.filter(b => b.parent === folder.id);
        children.forEach(scene => {
          const sceneDiv = document.createElement("div");
          const isActive = scene.id === state.active_scene_id;
          sceneDiv.className = `tree-item depth-1 ${isActive ? "active" : ""}`;
          sceneDiv.innerHTML = `
            <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
            <span class="tree-item-title">${escapeHtml(scene.title)}</span>
            <span class="tree-item-count">${scene.word_count}자</span>
          `;
          sceneDiv.onclick = () => selectScene(scene.id);
          binderTreeEl.appendChild(sceneDiv);
        });
      });

      rootScenes.forEach(scene => {
        const sceneDiv = document.createElement("div");
        const isActive = scene.id === state.active_scene_id;
        sceneDiv.className = `tree-item depth-0 ${isActive ? "active" : ""}`;
        sceneDiv.innerHTML = `
          <svg class="icon" viewBox="0 0 24 24"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path><polyline points="14 2 14 8 20 8"></polyline></svg>
          <span class="tree-item-title">${escapeHtml(scene.title)}</span>
          <span class="tree-item-count">${scene.word_count}자</span>
        `;
        sceneDiv.onclick = () => selectScene(scene.id);
        binderTreeEl.appendChild(sceneDiv);
      });
    }

    function selectScene(sceneId) {
      if (state.active_scene_id === sceneId) return;
      state.active_scene_id = sceneId;
      renderBinder();
      loadActiveScene();
    }

    function loadActiveScene() {
      const scene = state.scenes[state.active_scene_id];
      if (!scene) return;

      sceneTitleInput.value = scene.title;
      editorTextarea.value = scene.text || "";
      updateWordCount(scene.text || "");

      if (isScriveningsMode) {
        renderScrivenings();
      }
    }

    // Word Count calculation
    function updateWordCount(text) {
      const charsWithSpace = text.length;
      const charsNoSpace = text.replace(/\s+/g, "").length;
      const words = text.trim() ? text.trim().split(/\s+/).length : 0;
      const manuscriptPages = (charsWithSpace / 200).toFixed(1);

      statCharsWithSpace.textContent = charsWithSpace.toLocaleString();
      statCharsNoSpace.textContent = charsNoSpace.toLocaleString();
      statWords.textContent = words.toLocaleString();
      statPages.textContent = manuscriptPages;

      const progressPct = Math.min(100, Math.round((charsWithSpace / 1000) * 100));
      statFill.style.width = `${progressPct}%`;
    }

    // Keystroke Debounced Sync
    editorTextarea.addEventListener("input", () => {
      const newText = editorTextarea.value;
      updateWordCount(newText);

      // Instant local state update
      if (state.scenes[state.active_scene_id]) {
        state.scenes[state.active_scene_id].text = newText;
        state.scenes[state.active_scene_id].word_count = newText.length;
      }
      const b = state.binder.find(x => x.id === state.active_scene_id);
      if (b) b.word_count = newText.length;
      renderBinder();

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

      // Also persist to REST endpoint as dual guarantee
      try {
        await fetch(`/api/scenes/${sceneId}`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ text: text })
        });
      } catch (e) {
        console.warn("Rest save fallback error:", e);
      }
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
        renderBinder();

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

    // Render Lore List
    function renderLore(filter = "") {
      loreListContainer.innerHTML = "";
      const filtered = state.lore.filter(l => 
        !filter || l.name.includes(filter) || l.category.includes(filter) || l.content.includes(filter)
      );

      filtered.forEach(lore => {
        const card = document.createElement("div");
        card.className = "lore-card";
        card.innerHTML = `
          <div class="lore-card-header">
            <span class="lore-name">${escapeHtml(lore.name)}</span>
            <span class="lore-category-tag">${escapeHtml(lore.category)}</span>
          </div>
          <p class="lore-content-text">${escapeHtml(lore.content)}</p>
        `;
        loreListContainer.appendChild(card);
      });
    }

    document.getElementById("lore-search-input").addEventListener("input", (e) => {
      renderLore(e.target.value.trim());
    });

    // Scrivenings Mode Toggle
    const btnToggleScrivenings = document.getElementById("btn-toggle-scrivenings");
    const scriveningsLabel = document.getElementById("scrivenings-label");
    btnToggleScrivenings.addEventListener("click", () => {
      isScriveningsMode = !isScriveningsMode;
      if (isScriveningsMode) {
        editorTextarea.style.display = "none";
        scriveningsContainer.style.display = "block";
        scriveningsLabel.textContent = "단일 씬 모드";
        renderScrivenings();
      } else {
        editorTextarea.style.display = "block";
        scriveningsContainer.style.display = "none";
        scriveningsLabel.textContent = "스크리브닝스 모드";
      }
    });

    function renderScrivenings() {
      scriveningsContainer.innerHTML = "";
      state.binder.filter(b => !b.is_folder).forEach(scene => {
        const sec = document.createElement("div");
        sec.className = "scrivenings-section";
        sec.innerHTML = `
          <div class="scrivenings-title"># ${escapeHtml(scene.title)}</div>
          <div class="scrivenings-content">${escapeHtml(state.scenes[scene.id]?.text || "")}</div>
        `;
        scriveningsContainer.appendChild(sec);
      });
    }

    // Font toggle
    let isSerif = true;
    const btnFontToggle = document.getElementById("btn-font-toggle");
    const fontLabel = document.getElementById("font-style-label");
    btnFontToggle.addEventListener("click", () => {
      isSerif = !isSerif;
      if (isSerif) {
        editorTextarea.style.fontFamily = "var(--font-serif)";
        fontLabel.textContent = "명조체 (Serif)";
      } else {
        editorTextarea.style.fontFamily = "var(--font-sans)";
        fontLabel.textContent = "고딕체 (Sans)";
      }
    });

    // Dark theme toggle
    const btnThemeToggle = document.getElementById("btn-theme-toggle");
    let isDark = false;
    btnThemeToggle.addEventListener("click", () => {
      isDark = !isDark;
      document.documentElement.setAttribute("data-theme", isDark ? "dark" : "light");
    });

    // Peer simulation button
    document.getElementById("btn-peer-simulate").addEventListener("click", () => {
      window.open(location.href, "_blank", "width=1200,height=800");
    });

    // Export Quick
    document.getElementById("btn-export-quick").addEventListener("click", () => {
      let fullManuscript = `# ${state.title}\n저자: ${state.author}\n\n`;
      state.binder.filter(b => !b.is_folder).forEach(b => {
        fullManuscript += `\n## ${b.title}\n\n${state.scenes[b.id]?.text || ""}\n\n`;
      });
      const blob = new Blob([fullManuscript], { type: "text/markdown;charset=utf-8" });
      const a = document.createElement("a");
      a.href = URL.createObjectURL(blob);
      a.download = `${state.title}_원고전체.md`;
      a.click();
    });

    // Tab navigation in Inspector
    document.querySelectorAll(".tab-btn").forEach(btn => {
      btn.addEventListener("click", () => {
        document.querySelectorAll(".tab-btn").forEach(b => b.classList.remove("active"));
        document.querySelectorAll(".tab-content").forEach(c => c.classList.remove("active"));
        btn.classList.add("active");
        document.getElementById(btn.dataset.tab).classList.add("active");
      });
    });

    // Modal Handling
    const nodeModal = document.getElementById("node-modal");
    const nodeModalTitle = document.getElementById("node-modal-title");
    const nodeNameInput = document.getElementById("node-name-input");
    const nodeParentSelect = document.getElementById("node-parent-select");

    document.getElementById("btn-add-folder").addEventListener("click", () => {
      currentModalType = "folder";
      nodeModalTitle.textContent = "새 챕터/폴더 추가";
      nodeNameInput.value = "";
      updateParentSelect();
      nodeModal.style.display = "flex";
      nodeNameInput.focus();
    });

    document.getElementById("btn-add-scene").addEventListener("click", () => {
      currentModalType = "scene";
      nodeModalTitle.textContent = "새 씬 추가";
      nodeNameInput.value = "";
      updateParentSelect();
      nodeModal.style.display = "flex";
      nodeNameInput.focus();
    });

    function updateParentSelect() {
      nodeParentSelect.innerHTML = `<option value="root">최상위 (Root)</option>`;
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
      loreModal.style.display = "flex";
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
