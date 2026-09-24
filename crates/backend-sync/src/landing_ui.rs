pub const LANDING_HTML: &str = r##"<!DOCTYPE html>
<html lang="ko" data-theme="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Narratics — 스크리브너를 능가하는 로컬 퍼스트 서사 집필 스튜디오</title>
  <meta name="description" content="100% 무네트워크 오프라인 보장, Kleppmann Tree Move CRDT, Yrs UTF-16 고성능 텍스트 엔진, 단일 파일 SQLite WAL (.narr) 기반의 차세대 서사 창작 스튜디오.">
  <link rel="preconnect" href="https://cdn.jsdelivr.net">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/orioncactus/pretendard/dist/web/static/pretendard.css">
  <style>
    :root {
      --bg: #ffffff;
      --bg-surface: #fafafa;
      --bg-subtle: #f5f5f5;
      --bg-hover: #f0f0f0;
      --bg-card: #ffffff;
      --border: #e5e5e5;
      --border-strong: #d4d4d4;
      --text: #111111;
      --text-muted: #666666;
      --text-subtle: #8e8e93;
      --kbd-bg: #f5f5f5;
      --kbd-border: #d4d4d4;
      --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
      --shadow-md: 0 8px 30px rgba(0, 0, 0, 0.08);
      --font: "Pretendard", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      --font-mono: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    }

    [data-theme="dark"] {
      --bg: #0b0c0e;
      --bg-surface: #111215;
      --bg-subtle: rgba(255, 255, 255, 0.035);
      --bg-hover: rgba(255, 255, 255, 0.07);
      --bg-card: #141519;
      --border: rgba(255, 255, 255, 0.08);
      --border-strong: rgba(255, 255, 255, 0.16);
      --text: #ededed;
      --text-muted: #8a8c95;
      --text-subtle: #565861;
      --kbd-bg: rgba(255, 255, 255, 0.06);
      --kbd-border: rgba(255, 255, 255, 0.14);
      --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.4);
      --shadow-md: 0 20px 50px rgba(0, 0, 0, 0.7);
    }

    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
    }

    body {
      font-family: var(--font);
      background-color: var(--bg);
      color: var(--text);
      line-height: 1.6;
      -webkit-font-smoothing: antialiased;
      transition: background-color 0.2s ease, color 0.2s ease;
      overflow-x: hidden;
    }

    /* Container */
    .container {
      width: 100%;
      max-width: 1200px;
      margin: 0 auto;
      padding: 0 24px;
    }

    /* Navigation Bar */
    header.site-header {
      position: sticky;
      top: 0;
      z-index: 100;
      background: var(--bg);
      backdrop-filter: blur(16px);
      -webkit-backdrop-filter: blur(16px);
      border-bottom: 1px solid var(--border);
      transition: border-color 0.2s, background-color 0.2s;
    }

    [data-theme="dark"] header.site-header {
      background: rgba(11, 12, 14, 0.85);
    }

    [data-theme="light"] header.site-header {
      background: rgba(255, 255, 255, 0.88);
    }

    .nav-inner {
      display: flex;
      align-items: center;
      justify-content: space-between;
      height: 64px;
    }

    .brand-wrap {
      display: flex;
      align-items: center;
      gap: 12px;
      text-decoration: none;
      color: var(--text);
    }

    .brand-mark {
      width: 32px;
      height: 32px;
      background: var(--text);
      color: var(--bg);
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: 800;
      font-size: 16px;
      letter-spacing: -0.5px;
    }

    .brand-name {
      font-size: 18px;
      font-weight: 700;
      letter-spacing: -0.5px;
    }

    .brand-badge {
      font-size: 11px;
      font-weight: 600;
      padding: 2px 7px;
      border-radius: 4px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      color: var(--text-muted);
    }

    .nav-links {
      display: flex;
      align-items: center;
      gap: 28px;
      list-style: none;
    }

    .nav-links a {
      color: var(--text-muted);
      text-decoration: none;
      font-size: 14px;
      font-weight: 500;
      transition: color 0.15s ease;
    }

    .nav-links a:hover {
      color: var(--text);
    }

    .nav-actions {
      display: flex;
      align-items: center;
      gap: 12px;
    }

    /* Buttons */
    .btn {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      padding: 8px 16px;
      border-radius: 6px;
      font-size: 14px;
      font-weight: 600;
      text-decoration: none;
      cursor: pointer;
      transition: all 0.15s ease;
      border: 1px solid var(--border);
      background: var(--bg-card);
      color: var(--text);
    }

    .btn:hover {
      background: var(--bg-hover);
      border-color: var(--border-strong);
    }

    .btn-primary {
      background: var(--text);
      color: var(--bg);
      border-color: var(--text);
    }

    .btn-primary:hover {
      opacity: 0.92;
      background: var(--text);
      border-color: var(--text);
    }

    .btn-lg {
      padding: 12px 24px;
      font-size: 15px;
      border-radius: 8px;
    }

    kbd {
      font-family: var(--font-mono);
      font-size: 11px;
      padding: 2px 5px;
      border-radius: 4px;
      background: var(--kbd-bg);
      border: 1px solid var(--kbd-border);
      box-shadow: 0 1px 0 rgba(0, 0, 0, 0.1);
      color: var(--text-muted);
    }

    /* Hero Section */
    section.hero {
      padding: 80px 0 60px;
      text-align: center;
    }

    .hero-badge-wrap {
      display: inline-flex;
      align-items: center;
      gap: 8px;
      padding: 6px 14px;
      border-radius: 20px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      margin-bottom: 28px;
      font-size: 13px;
      color: var(--text-muted);
    }

    .hero-pulse-dot {
      width: 7px;
      height: 7px;
      border-radius: 50%;
      background: #10b981;
      display: inline-block;
    }

    .hero-title {
      font-size: 48px;
      font-weight: 800;
      letter-spacing: -1.5px;
      line-height: 1.15;
      margin-bottom: 20px;
      max-width: 880px;
      margin-left: auto;
      margin-right: auto;
    }

    .hero-desc {
      font-size: 18px;
      color: var(--text-muted);
      max-width: 700px;
      margin: 0 auto 36px;
      line-height: 1.6;
    }

    .hero-ctas {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 16px;
      margin-bottom: 40px;
      flex-wrap: wrap;
    }

    .hero-metrics {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 16px;
      max-width: 960px;
      margin: 0 auto;
      text-align: left;
    }

    .metric-card {
      background: var(--bg-surface);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 16px 20px;
    }

    .metric-val {
      font-size: 20px;
      font-weight: 800;
      letter-spacing: -0.5px;
      margin-bottom: 4px;
    }

    .metric-lbl {
      font-size: 12px;
      color: var(--text-muted);
    }

    /* Live Studio Showcase */
    section.showcase {
      padding: 40px 0 80px;
    }

    .showcase-frame {
      background: var(--bg-card);
      border: 1px solid var(--border);
      border-radius: 12px;
      box-shadow: var(--shadow-md);
      overflow: hidden;
    }

    .showcase-topbar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 12px 20px;
      background: var(--bg-surface);
      border-bottom: 1px solid var(--border);
    }

    .window-dots {
      display: flex;
      gap: 6px;
    }

    .window-dot {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      background: var(--border-strong);
    }

    .showcase-title {
      font-size: 12px;
      font-family: var(--font-mono);
      color: var(--text-muted);
    }

    .showcase-actions {
      display: flex;
      gap: 8px;
    }

    .showcase-img-wrap {
      position: relative;
      width: 100%;
      line-height: 0;
      background: var(--bg);
    }

    .showcase-img {
      width: 100%;
      height: auto;
      display: block;
    }

    /* Features Grid */
    section.features {
      padding: 80px 0;
      border-top: 1px solid var(--border);
    }

    .section-header {
      text-align: center;
      margin-bottom: 56px;
    }

    .section-eyebrow {
      font-size: 12px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 1px;
      color: var(--text-muted);
      margin-bottom: 10px;
    }

    .section-title {
      font-size: 32px;
      font-weight: 800;
      letter-spacing: -1px;
      margin-bottom: 14px;
    }

    .section-desc {
      font-size: 16px;
      color: var(--text-muted);
      max-width: 600px;
      margin: 0 auto;
    }

    .features-grid {
      display: grid;
      grid-template-columns: repeat(3, 1fr);
      gap: 24px;
    }

    .feature-card {
      background: var(--bg-surface);
      border: 1px solid var(--border);
      border-radius: 10px;
      padding: 28px;
      transition: transform 0.15s ease, border-color 0.15s ease;
    }

    .feature-card:hover {
      border-color: var(--border-strong);
      transform: translateY(-2px);
    }

    .feature-icon {
      width: 40px;
      height: 40px;
      border-radius: 8px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 20px;
    }

    .feature-icon svg {
      width: 20px;
      height: 20px;
      stroke: var(--text);
    }

    .feature-title {
      font-size: 18px;
      font-weight: 700;
      margin-bottom: 10px;
    }

    .feature-body {
      font-size: 14px;
      color: var(--text-muted);
      line-height: 1.6;
    }

    /* Downloads Grid */
    section.downloads {
      padding: 80px 0;
      border-top: 1px solid var(--border);
      background: var(--bg-surface);
    }

    .download-grid {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 20px;
      margin-bottom: 40px;
    }

    .download-card {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 10px;
      padding: 24px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      position: relative;
    }

    .download-platform-header {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 16px;
    }

    .download-platform-icon {
      width: 36px;
      height: 36px;
      border-radius: 8px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .download-platform-icon svg {
      width: 20px;
      height: 20px;
      stroke: var(--text);
    }

    .download-platform-name {
      font-size: 16px;
      font-weight: 700;
    }

    .download-platform-target {
      font-size: 12px;
      color: var(--text-muted);
    }

    .download-specs {
      list-style: none;
      margin: 16px 0 24px;
      font-size: 12px;
      color: var(--text-muted);
      display: flex;
      flex-direction: column;
      gap: 6px;
    }

    .download-specs li {
      display: flex;
      justify-content: space-between;
      border-bottom: 1px dashed var(--border);
      padding-bottom: 4px;
    }

    .download-specs li span:last-child {
      color: var(--text);
      font-weight: 500;
      font-family: var(--font-mono);
    }

    .hash-row {
      font-size: 11px;
      background: var(--bg-subtle);
      border: 1px solid var(--border);
      border-radius: 4px;
      padding: 6px 8px;
      margin-bottom: 16px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      cursor: pointer;
    }

    .hash-row code {
      font-family: var(--font-mono);
      font-size: 10px;
      color: var(--text-muted);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      max-width: 140px;
    }

    .hash-copy-btn {
      font-size: 10px;
      color: var(--text);
      font-weight: 600;
    }

    .linux-note-box {
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      padding: 16px 20px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 13px;
    }

    .linux-note-box code {
      font-family: var(--font-mono);
      background: var(--bg-subtle);
      padding: 3px 8px;
      border-radius: 4px;
      border: 1px solid var(--border);
      margin-left: 8px;
    }

    /* Comparison Matrix */
    section.comparison {
      padding: 80px 0;
      border-top: 1px solid var(--border);
    }

    .comp-table {
      width: 100%;
      border-collapse: collapse;
      margin-top: 32px;
      font-size: 13px;
      background: var(--bg);
      border: 1px solid var(--border);
      border-radius: 8px;
      overflow: hidden;
    }

    .comp-table th, .comp-table td {
      padding: 14px 18px;
      text-align: left;
      border-bottom: 1px solid var(--border);
    }

    .comp-table th {
      background: var(--bg-surface);
      font-weight: 700;
      color: var(--text);
    }

    .comp-table tr:last-child td {
      border-bottom: none;
    }

    .comp-check {
      color: #10b981;
      font-weight: 700;
    }

    .comp-cross {
      color: var(--text-subtle);
    }

    /* Footer */
    footer.site-footer {
      border-top: 1px solid var(--border);
      padding: 48px 0;
      background: var(--bg);
      font-size: 13px;
      color: var(--text-muted);
    }

    .footer-inner {
      display: flex;
      align-items: center;
      justify-content: space-between;
      flex-wrap: wrap;
      gap: 20px;
    }

    .footer-links {
      display: flex;
      gap: 24px;
      list-style: none;
    }

    .footer-links a {
      color: var(--text-muted);
      text-decoration: none;
    }

    .footer-links a:hover {
      color: var(--text);
    }

    @media (max-width: 960px) {
      .hero-title { font-size: 34px; }
      .hero-metrics { grid-template-columns: repeat(2, 1fr); }
      .features-grid { grid-template-columns: 1fr; }
      .download-grid { grid-template-columns: repeat(2, 1fr); }
      .nav-links { display: none; }
    }

    @media (max-width: 640px) {
      .download-grid { grid-template-columns: 1fr; }
      .hero-metrics { grid-template-columns: 1fr; }
    }
  </style>
</head>
<body>
  <!-- Header -->
  <header class="site-header">
    <div class="container nav-inner">
      <a href="/" class="brand-wrap">
        <div class="brand-mark">N</div>
        <div class="brand-name">Narratics</div>
        <span class="brand-badge">v0.1.0</span>
      </a>

      <ul class="nav-links">
        <li><a href="#features">핵심 엔진</a></li>
        <li><a href="#download">다운로드</a></li>
        <li><a href="#comparison">비교 명세</a></li>
        <li><a href="/app">웹 스튜디오</a></li>
      </ul>

      <div class="nav-actions">
        <button id="theme-toggle" class="btn" title="테마 전환 (다크/라이트)">
          <svg id="theme-icon-moon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="display: none;"><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/></svg>
          <svg id="theme-icon-sun" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/></svg>
        </button>
        <a href="/app" class="btn btn-primary">
          웹 스튜디오 실행 <kbd>→</kbd>
        </a>
      </div>
    </div>
  </header>

  <!-- Hero -->
  <section class="hero">
    <div class="container">
      <div class="hero-badge-wrap">
        <span class="hero-pulse-dot"></span>
        <span>Local-First Autonomous Engine</span>
        <span style="color: var(--border-strong);">|</span>
        <span>Zero Network Latency</span>
      </div>

      <h1 class="hero-title">
        스크리브너를 능가하는<br>로컬 퍼스트 서사 집필 스튜디오
      </h1>

      <p class="hero-desc">
        100만 자 장편 소설도 지연 없이 즉시 반응하는 Rust AOT 코어.<br>
        인터넷 연결 없는 비행기 안에서도 완벽히 작동하는 단일 컨테이너 <code>.narr</code> 구조.
      </p>

      <div class="hero-ctas">
        <a href="#download" class="btn btn-primary btn-lg">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="7 10 12 15 17 10"/><line x1="12" y1="15" x2="12" y2="3"/></svg>
          프로그램 다운로드 (v0.1.0)
        </a>
        <a href="/app" class="btn btn-lg">
          브라우저에서 바로 써보기 <kbd>Web</kbd>
        </a>
      </div>

      <div class="hero-metrics">
        <div class="metric-card">
          <div class="metric-val">100%</div>
          <div class="metric-lbl">완전 오프라인 자율 동작 (.narr)</div>
        </div>
        <div class="metric-card">
          <div class="metric-val">UTF-16</div>
          <div class="metric-lbl">한글·이모지 무결점 CRDT 인덱싱</div>
        </div>
        <div class="metric-card">
          <div class="metric-val">0-Defect</div>
          <div class="metric-lbl">Kleppmann Tree Move 순환 방지</div>
        </div>
        <div class="metric-card">
          <div class="metric-val">SQLite WAL</div>
          <div class="metric-lbl">정전·크래시 제로 데이터 손실</div>
        </div>
      </div>
    </div>
  </section>

  <!-- Showcase Preview -->
  <section class="showcase">
    <div class="container">
      <div class="showcase-frame">
        <div class="showcase-topbar">
          <div class="window-dots">
            <div class="window-dot"></div>
            <div class="window-dot"></div>
            <div class="window-dot"></div>
          </div>
          <div class="showcase-title">narratics-studio — Raycast Obsidian & Pure White Edition</div>
          <div class="showcase-actions">
            <button class="btn" style="padding: 4px 10px; font-size: 11px;" onclick="toggleShowcaseTheme()">테마 변경 실측</button>
            <a href="/app" class="btn btn-primary" style="padding: 4px 10px; font-size: 11px;">스튜디오 진입</a>
          </div>
        </div>
        <div class="showcase-img-wrap">
          <img id="showcase-img" class="showcase-img" src="/docs/studio_monochrome_dark_live.png" alt="Narratics Studio Live Interface">
        </div>
      </div>
    </div>
  </section>

  <!-- Features -->
  <section id="features" class="features">
    <div class="container">
      <div class="section-header">
        <div class="section-eyebrow">Engine Architecture</div>
        <h2 class="section-title">작가를 위해 공학적으로 설계된 6대 핵심 기술</h2>
        <p class="section-desc">클라우드 장애, 구독 결제 유도, 텍스트 깨짐 걱정 없는 순수 소프트웨어의 정수</p>
      </div>

      <div class="features-grid">
        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H20v20H6.5a2.5 2.5 0 0 1-2.5-2.5Z"/><path d="M6 6h10"/><path d="M6 10h10"/></svg>
          </div>
          <h3 class="feature-title">스크리브닝스 (Scrivenings)</h3>
          <p class="feature-body">폴더 내 수십 개의 씬과 챕터를 단 하나의 연속된 원고로 즉시 이어붙여 조망하고, 퇴고 및 커서 편집을 끊김 없이 수행합니다.</p>
        </div>

        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
          </div>
          <h3 class="feature-title">Kleppmann Tree Move CRDT</h3>
          <p class="feature-body">트리 노드(챕터/씬)의 드래그 앤 드롭 이동 시 발생할 수 있는 부모-자식 순환 데드락을 수학적으로 차단하여 0결함을 보장합니다.</p>
        </div>

        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/><path d="M3 12a9 3 0 0 0 18 0"/></svg>
          </div>
          <h3 class="feature-title">단일 컨테이너 <code>.narr</code></h3>
          <p class="feature-body">수천 개의 조각 파일 대신 SQLite WAL 기반의 단일 파일로 원고, 인덱스, 타임머신 스냅샷, 로어덱을 견고하게 패키징합니다.</p>
        </div>

        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 14 14"/></svg>
          </div>
          <h3 class="feature-title">타임머신 마이크로 스냅샷</h3>
          <p class="feature-body">씬 수정마다 오버헤드 없이 스냅샷을 자동 적재하여 언제든 과거의 한 문장, 특정 버전의 퇴고 상태로 무손실 롤백합니다.</p>
        </div>

        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2Zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8Z"/><path d="m9 12 2 2 4-4"/></svg>
          </div>
          <h3 class="feature-title">세계관 로어덱 (Lore Deck)</h3>
          <p class="feature-body">등장인물, 지명, 세력, 마법/과학 설정을 카드 형태로 관리하고 본문 작성 중 <code>@</code> 키로 멘션하여 실시간 참조합니다.</p>
        </div>

        <div class="feature-card">
          <div class="feature-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="9" y1="3" x2="9" y2="21"/></svg>
          </div>
          <h3 class="feature-title">플랫 모노크롬 조판 캔버스</h3>
          <p class="feature-body">눈의 피로를 최소화하는 Pure White(#FFFFFF)와 Deep Obsidian(#0B0C0E) 테마, 나눔명조·Pretendard 조판 커스텀을 지원합니다.</p>
        </div>
      </div>
    </div>
  </section>

  <!-- Downloads -->
  <section id="download" class="downloads">
    <div class="container">
      <div class="section-header">
        <div class="section-eyebrow">Multiplatform Distribution</div>
        <h2 class="section-title">모든 디바이스를 위한 공식 릴리즈 다운로드</h2>
        <p class="section-desc">Windows, macOS, iOS, Android 어디서나 동일한 Rust 코어로 당신의 서사를 이어가세요.</p>
      </div>

      <div class="download-grid">
        <!-- Windows -->
        <div class="download-card">
          <div>
            <div class="download-platform-header">
              <div class="download-platform-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="8" height="8"/><rect x="13" y="3" width="8" height="8"/><rect x="3" y="13" width="8" height="8"/><rect x="13" y="13" width="8" height="8"/></svg>
              </div>
              <div>
                <div class="download-platform-name">Windows</div>
                <div class="download-platform-target">x64 / ARM64</div>
              </div>
            </div>

            <ul class="download-specs">
              <li><span>버전</span> <span>v0.1.0</span></li>
              <li><span>형식</span> <span>.exe (NSIS Setup)</span></li>
              <li><span>요구사양</span> <span>Windows 10 / 11</span></li>
            </ul>

            <div class="hash-row" data-hash="dbcd8d39a2af2b369e15d82bac01c6f91858787cb377581b1a5d42c2ec466dac">
              <code>SHA-256: dbcd8d39...</code>
              <span class="hash-copy-btn">복사</span>
            </div>
          </div>

          <a href="/releases/windows/Narratics-Setup-0.1.0.exe" class="btn btn-primary" download>
            Windows용 설치파일 (.exe)
          </a>
        </div>

        <!-- macOS -->
        <div class="download-card">
          <div>
            <div class="download-platform-header">
              <div class="download-platform-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20.94c1.5 0 2.75 1.06 4 1.06 3 0 6-8 6-12.22A4.91 4.91 0 0 0 17 5c-2.22 0-4 1.44-5 2-1-.56-2.78-2-5-2a4.9 4.9 0 0 0-5 4.78C2 14 5 22 8 22c1.25 0 2.5-1.06 4-1.06Z"/><path d="M10 2c1 .5 2 2 2 5"/></svg>
              </div>
              <div>
                <div class="download-platform-name">macOS</div>
                <div class="download-platform-target">Apple Silicon & Intel</div>
              </div>
            </div>

            <ul class="download-specs">
              <li><span>버전</span> <span>v0.1.0</span></li>
              <li><span>형식</span> <span>.dmg (Universal)</span></li>
              <li><span>요구사양</span> <span>macOS 12 Monterey+</span></li>
            </ul>

            <div class="hash-row" data-hash="91227c87305a0265f755a2a3ef7f366e11c22fc6d3dd01a9b2e0d8ff910dad72">
              <code>SHA-256: 91227c87...</code>
              <span class="hash-copy-btn">복사</span>
            </div>
          </div>

          <a href="/releases/macos/Narratics-0.1.0-universal.dmg" class="btn btn-primary" download>
            macOS용 디스크 (.dmg)
          </a>
        </div>

        <!-- iOS -->
        <div class="download-card">
          <div>
            <div class="download-platform-header">
              <div class="download-platform-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="2" width="14" height="20" rx="2" ry="2"/><line x1="12" y1="18" x2="12.01" y2="18"/></svg>
              </div>
              <div>
                <div class="download-platform-name">iOS / iPadOS</div>
                <div class="download-platform-target">iPhone & iPad</div>
              </div>
            </div>

            <ul class="download-specs">
              <li><span>버전</span> <span>v0.1.0</span></li>
              <li><span>형식</span> <span>.ipa / TestFlight</span></li>
              <li><span>요구사양</span> <span>iOS 16.0 이상</span></li>
            </ul>

            <div class="hash-row" data-hash="1c9f2357b441e653533b5d50d0b119a198921a670db9d52cbda9e3ce2a5d0dc2">
              <code>SHA-256: 1c9f2357...</code>
              <span class="hash-copy-btn">복사</span>
            </div>
          </div>

          <a href="/releases/ios/Narratics-v0.1.0.ipa" class="btn btn-primary" download>
            iOS 패키지 다운로드 (.ipa)
          </a>
        </div>

        <!-- Android -->
        <div class="download-card">
          <div>
            <div class="download-platform-header">
              <div class="download-platform-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="5" y="4" width="14" height="16" rx="2"/><circle cx="12" cy="7" r="0.5"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
              </div>
              <div>
                <div class="download-platform-name">Android</div>
                <div class="download-platform-target">Phone & Tablet</div>
              </div>
            </div>

            <ul class="download-specs">
              <li><span>버전</span> <span>v0.1.0</span></li>
              <li><span>형식</span> <span>.apk (Standalone)</span></li>
              <li><span>요구사양</span> <span>Android 10.0+</span></li>
            </ul>

            <div class="hash-row" data-hash="f824c93f102450499d598cdcc7219f23bec16e32e55c8e81c94361146255d52f">
              <code>SHA-256: f824c93f...</code>
              <span class="hash-copy-btn">복사</span>
            </div>
          </div>

          <a href="/releases/android/Narratics-v0.1.0-release.apk" class="btn btn-primary" download>
            Android 패키지 (.apk)
          </a>
        </div>
      </div>

      <!-- Linux / CLI Note -->
      <div class="linux-note-box">
        <div>
          <strong>Linux & 개발자 도구:</strong> 
          x86_64 AppImage가 준비되어 있으며, Cargo를 통한 네이티브 빌드 설치도 지원합니다.
          <code>cargo install narratics-cli</code>
        </div>
        <a href="/releases/linux/Narratics-0.1.0-x86_64.AppImage" class="btn" download>
          Linux AppImage 받기
        </a>
      </div>
    </div>
  </section>

  <!-- Comparison Matrix -->
  <section id="comparison" class="comparison">
    <div class="container">
      <div class="section-header">
        <div class="section-eyebrow">Technical Comparison</div>
        <h2 class="section-title">스크리브너 및 주요 도구와의 사양 비교</h2>
        <p class="section-desc">왜 장편 서사 집필에는 내러틱스 로컬 퍼스트 코어가 필요한지 수치로 입증합니다.</p>
      </div>

      <table class="comp-table">
        <thead>
          <tr>
            <th>비교 항목</th>
            <th>Narratics (내러틱스)</th>
            <th>Scrivener (스크리브너)</th>
            <th>Obsidian (옵시디언)</th>
            <th>Notion (노션)</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td><strong>오프라인 자율성</strong></td>
            <td><span class="comp-check">● 100% 완전 오프라인 (.narr)</span></td>
            <td>▲ 로컬 파일 (클라우드 동기화 불안정)</td>
            <td>● 로컬 마크다운 파일</td>
            <td><span class="comp-cross">× 인터넷 연결 필수</span></td>
          </tr>
          <tr>
            <td><strong>CRDT 실시간 동기화</strong></td>
            <td><span class="comp-check">● Yrs + Kleppmann Tree CRDT</span></td>
            <td><span class="comp-cross">× 없음 (파일 단위 충돌 파일 생성)</span></td>
            <td>▲ 플러그인 의존 (충돌 발생)</td>
            <td>▲ OT 기반 중앙 집중 서버</td>
          </tr>
          <tr>
            <td><strong>한글/유니코드 UTF-16</strong></td>
            <td><span class="comp-check">● Yrs UTF-16 오프셋 완벽 대응</span></td>
            <td>▲ OS 렌더러 의존 (자모 분리 버그)</td>
            <td>▲ 브라우저 인덱스 의존</td>
            <td>▲ 서버 레이턴시 의존</td>
          </tr>
          <tr>
            <td><strong>트리 노드 순환 방지</strong></td>
            <td><span class="comp-check">● 수학적 불변성 검증 (0-Defect)</span></td>
            <td><span class="comp-cross">× 수동 폴더 이동 시 오류 가능</span></td>
            <td>▲ 파일 시스템 의존</td>
            <td>▲ 권한 의존</td>
          </tr>
          <tr>
            <td><strong>서사 전용 로어덱 & 멘션</strong></td>
            <td><span class="comp-check">● @ 멘션 세계관 내장 카드</span></td>
            <td>▲ 단순 인물 템플릿 문서</td>
            <td>▲ 양방향 링크 (수동 설정)</td>
            <td>▲ 관계형 데이터베이스 (복잡)</td>
          </tr>
          <tr>
            <td><strong>비용 및 벤더 락인</strong></td>
            <td><span class="comp-check">● 영구 무료 오픈소스 (Apache-2.0)</span></td>
            <td>▲ 플랫폼별 별도 라이선스 구매</td>
            <td>▲ Sync/Publish 유료 구독</td>
            <td><span class="comp-cross">× 월간 지속 구독 필수</span></td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>

  <!-- Footer -->
  <footer class="site-footer">
    <div class="container footer-inner">
      <div>
        <div style="font-weight: 700; color: var(--text); margin-bottom: 4px;">Narratics — Local-First Narrative Studio</div>
        <div>엔트로피패러독스 주식회사 (Entropy Paradox, Inc.) | Apache-2.0 & MIT License</div>
      </div>
      <ul class="footer-links">
        <li><a href="/app">웹 스튜디오 열기</a></li>
        <li><a href="#download">다운로드 센터</a></li>
        <li><a href="/health">서버 상태</a></li>
        <li><a href="https://github.com/cycorld/narratics" target="_blank" rel="noreferrer">GitHub</a></li>
      </ul>
    </div>
  </footer>

  <script>
    // Theme toggle handling
    const themeToggleBtn = document.getElementById("theme-toggle");
    const moonIcon = document.getElementById("theme-icon-moon");
    const sunIcon = document.getElementById("theme-icon-sun");

    function applyTheme(theme) {
      document.documentElement.setAttribute("data-theme", theme);
      localStorage.setItem("narratics_theme", theme);
      if (theme === "dark") {
        moonIcon.style.display = "none";
        sunIcon.style.display = "block";
      } else {
        moonIcon.style.display = "block";
        sunIcon.style.display = "none";
      }
    }

    const savedTheme = localStorage.getItem("narratics_theme") || "dark";
    applyTheme(savedTheme);

    themeToggleBtn.addEventListener("click", () => {
      const current = document.documentElement.getAttribute("data-theme") || "dark";
      applyTheme(current === "dark" ? "light" : "dark");
    });

    function toggleShowcaseTheme() {
      const img = document.getElementById("showcase-img");
      if (img.src.includes("dark")) {
        img.src = "/docs/studio_monochrome_light_live.png";
      } else {
        img.src = "/docs/studio_monochrome_dark_live.png";
      }
    }

    document.querySelectorAll(".hash-row").forEach(row => {
      row.addEventListener("click", () => {
        const hash = row.getAttribute("data-hash");
        if (hash) {
          navigator.clipboard.writeText(hash).then(() => {
            alert("SHA-256 해시가 클립보드에 복사되었습니다:\n" + hash);
          }).catch(() => {
            prompt("SHA-256 해시:", hash);
          });
        }
      });
    });
  </script>
</body>
</html>
"##;
