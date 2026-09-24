<div align="center">

# 📖 Narratics (내러틱스)

**완결까지 흔들리지 않는 서사의 설계도 — 고성능 로컬 퍼스트 집필 스튜디오 & CRDT 엔진**

[![License: PolyForm Noncommercial 1.0.0](https://img.shields.io/badge/License-PolyForm%20Noncommercial%201.0.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](Cargo.toml)
[![Zero-Defect](https://img.shields.io/badge/Adversarial%20Hardening-13%2F13%20PASS-brightgreen.svg)](crates/engine-core/tests/adversarial_hardening.rs)
[![E2E UX Gate](https://img.shields.io/badge/E2E%20UX%20Gate-5%2F5%20PASS-brightgreen.svg)](tests/e2e_ux_automated_gate.py)
[![Platforms](https://img.shields.io/badge/Platform-Web%20%7C%20macOS%20%7C%20Windows%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-lightgrey.svg)](#architecture)

**[ English ](README.md)** • **[ 한국어 ](README.ko.md)**

</div>

---

## 1. 비전 및 핵심 철학

내러틱스(Narratics)는 기존 서사 저작 도구(Scrivener, Notion, Google Docs 등)의 동기화 불안정, 서식 왜곡, 벤더 종속 한계를 극복하기 위해 설계된 **프로덕션급 로컬 퍼스트 장편 소설 집필 스튜디오 및 세계관 엔진**입니다.

- **100% 로컬 퍼스트 (Zero-Network Invariant):** 외부 인터넷 연결이나 클라우드 계정 없이 단일 `.narr` SQLite WAL 파일로 모든 집필, 세계관 관리, 스냅샷 보관이 100% 자율 동작합니다.
- **순환 방지 트리 CRDT (Kleppmann Tree Move):** 폴더 및 씬의 임의 이동, 중첩, 분기 시 순환 참조(Cycle)를 원천 차단하고 다중 기기 환경에서 결정론적으로 수렴합니다.
- **UTF-16 인덱싱 기반 yrs 텍스트 엔진:** 웹 브라우저(TipTap), 데스크톱(Tauri), 모바일 네이티브 환경 간 3바이트 한글, 고대 문자, 4바이트 이모지 절단 버그를 완벽히 해결했습니다.
- **단일 파일 SQLite WAL 컨테이너 (`.narr`):** 스크리브너 식의 취약한 디렉터리 번들 충돌을 방지하고, 100만 자 이상의 장편 원고도 원자적(Atomic) 트랜잭션으로 안전하게 보존합니다.
- **자동화된 인체공학 UI & 제로 수동 QA:** 45% 시선 높이 타자기 수직 스크롤 락, `@` 멘션 로어북 자동완성, 코르크보드 장 연동, 분할 뷰(Split-view) 레퍼런스 인스펙터를 기본 탑재했습니다.

---

## 2. 모노레포 아키텍처

```
narratics/
├── Cargo.toml                       # 워크스페이스 루트 매니페스트
├── LICENSE                          # PolyForm Noncommercial License 1.0.0
├── apps/
│   └── desktop/                     # Tauri v2 크로스플랫폼 데스크톱 앱 (macOS/Win/Linux)
│       ├── Cargo.toml
│       ├── tauri.conf.json
│       ├── src/                     # 네이티브 IPC 커맨드 & 데스크톱 상태 관리
│       └── ui/                      # 단일 소스 원칙 웹 스튜디오 셸
├── crates/
│   ├── engine-core/                 # 순수 오프라인 Rust CRDT 코어 엔진 & 컨테이너
│   │   ├── src/tree_crdt.rs         # Kleppmann Tree Move CRDT
│   │   ├── src/text_engine.rs       # yrs UTF-16 씬 텍스트 엔진 & 컴팩션
│   │   └── src/container.rs         # .narr 단일 파일 SQLite WAL 입출력
│   ├── backend-sync/                # Axum AOT 동기화 서버 & 반응형 웹 스튜디오
│   │   ├── src/app_state.rs         # 다권 라이브러리, CRUD & Typst PDF 조판 컴파일러
│   │   ├── src/web_ui.rs            # TipTap 에디터, 코르크보드, 로어북 & 분할 뷰
│   │   ├── src/landing_ui.rs        # 프로덕션 랜딩 페이지 및 다운로드 허브
│   │   └── tests/regression_suite.rs# 백엔드 5대 전수 회귀 테스트 스위트
│   └── mobile-bridge/               # C-FFI / JNI / Swift 네이티브 모바일 브리지
│       ├── bindings/android/        # Kotlin JNI 바인딩
│       └── bindings/ios/            # Swift 브리지 인터페이스
├── docs/                            # 검증 스크린샷, 벤치마크 및 규격서
├── scripts/
│   ├── regression_gate.sh           # 제로-디펙트 빌드/테스트/E2E 통합 게이트
│   └── sync_desktop_ui.py           # 웹 스튜디오 코드를 데스크톱 셸에 동기화
└── tests/
    └── e2e_ux_automated_gate.py     # 헤드리스 브라우저 E2E 사용자 인터랙션 게이트
```

---

## 3. 13대 실패 축 적대적 경화 검증 지표

모든 릴리즈 후보는 13개 축에 대한 적대적 스트레스 테스트(`crates/engine-core/tests/adversarial_hardening.rs`)를 통과해야 합니다:

| 검증 축 | 테스트 시나리오 및 불변식 | 판정 | 소요 시간 |
| :--- | :--- | :---: | :---: |
| **Cycle 1: 오프라인 불변성** | 대량 작성 중 완전한 네트워크 단절, 크래시 재시작, 스냅샷 보존 | **PASS** | 0.05s |
| **Cycle 2: 경계 및 음수 방어** | 빈 문서 삭제, 음수/초과 범위 슬라이싱 방어 | **PASS** | 0.00s |
| **Cycle 3: 파일시스템 적대성** | 특수문자, 따옴표, 깊은 중첩 경로 생성 및 플러시 | **PASS** | 0.05s |
| **Cycle 4: 깊은 순환 트리 방어** | 50단계 조상-자손 순환 참조 이동 시도 사전 차단 | **PASS** | 0.00s |
| **Cycle 5: 손상 CRDT 페이로드** | 랜덤 바이트, 절단된 패킷 디코딩 시 패닉 방어 | **PASS** | 0.06s |
| **Cycle 6: 한글 및 이모지 경계** | 3바이트 한글 + 4바이트 이모지 임의 위치 절단/삽입 | **PASS** | 0.00s |
| **Cycle 7: 4대 기기 동시 수렴** | 4대 기기 동시 트리 이동 및 편집의 100% 동일 수렴 | **PASS** | 0.00s |
| **Cycle 8: 툼스톤 컴팩션** | 200회 연속 삭제/수정 후 1.1ms 스쿼시 & 바운디드 메모리 | **PASS** | 0.01s |
| **Cycle 9: SQLite WAL 경합** | 다중 커넥션 동시 트랜잭션 및 원자적 롤백 무결성 | **PASS** | 0.01s |
| **Cycle 10: 스키마 패리티** | JSON / Binary 와이어 포맷 라운드트립 무손실 | **PASS** | 0.00s |
| **Cycle 11: 100만 자 스트레스** | 1,000,000자 대작 원고 스트리밍 시 레이턴시 튐 방어 | **PASS** | 0.02s |
| **Cycle 12: 인젝션 공격 방어** | 악의적인 로어 ID 및 SQL 인젝션 시도 중화 | **PASS** | 0.01s |
| **Cycle 13: 휴지통 격리 불변식** | 삭제된 챕터/씬의 안전 격리 및 불변성 보장 | **PASS** | 0.00s |

---

## 4. 제로 수동 QA: 브라우저 E2E 인터랙션 자동 게이트

사용자가 수동으로 UI를 테스트하는 부하를 0으로 줄이기 위해 `tests/e2e_ux_automated_gate.py`가 실제 브라우저 이벤트와 DOM 지오메트리를 자동 실측합니다:

1. **`@` 멘션 키보드 탐색 검증:** `@` 타이핑 $\to$ 팝업 노출 $\to$ `ArrowDown` 하이라이트 이동 $\to$ `Enter` 본문 삽입 $\to$ 팝업 닫힘 검증.
2. **타자기 모드 45% 시선 높이 락:** 30줄 연속 타이핑 후 브라우저 커서가 화면 높이의 **45%($\pm 5\%$)** 위치에 정확히 유지되는지 실측.
3. **장 간 씬 이동 영속성 검증:** 인스펙터 드롭다운 및 드래그 이동 $\to$ SQLite WAL 반영 $\to$ 바인더 트리 갱신 확인.
4. **코르크보드 장 자동 동기화 검증:** 사이드바 선택 시 코르크보드가 해당 장으로 자동 전환되고 활성 씬 카드가 하이라이트되는지 확인.
5. **실행 취소(Undo) 토스트 검증:** 삭제 후 6초 타이머 및 복구 콜백 트리거 확인.

---

## 5. 빠른 시작 및 개발 가이드

### 요구사항
- **Rust:** `1.80+` (stable)
- **Python:** `3.10+` (E2E 브라우저 테스트 실행용)
- **Typst:** (선택사항, 출판급 PDF 조판 익스포트용)

### 빌드 및 검증

```bash
# 리포지토리 클론
git clone https://github.com/cycorld/narratics.git
cd narratics

# 워크스페이스 컴파일 검사
cargo check --workspace

# 22대 단위 테스트 및 적대적 경화 스위트 실행
cargo test --workspace -- --nocapture

# 제로-디펙트 회귀 게이트 전수 실행 (E2E UX 테스트 포함)
./scripts/regression_gate.sh
```

### 웹 스튜디오 로컬 실행

```bash
# Axum 웹 스튜디오 서버 실행
cargo run --release -p narratics-backend-sync

# 브라우저에서 http://localhost:3901/ 접속
```

---

## 6. 라이선스 및 상용 계약 안내

본 프로젝트는 **[PolyForm Noncommercial License 1.0.0](LICENSE)** 라이선스를 따릅니다.

### 라이선스 요약:
- **개인적·비영리적 사용:** **완전 무상(100% Free).** 개인 연구, 집필, 교육, 비영리 단체, 오픈소스 개발, 취미 활동 목적은 제한 없이 무료로 사용, 수정, 배포할 수 있습니다.
- **상용 서비스 및 영리 목적:** 상용 SaaS 호스팅, 유료 서비스 연동, 기업 내 영리 목적의 활용은 **별도의 유료 상용 라이선스 계약**이 필요합니다.

상용 라이선스 계약 및 파트너십 문의:
- **최용철 (Charles Choi)** (`cycorld@martian.link`)
- **엔트로피패러독스 주식회사 (Entropy Paradox, Inc.)**
