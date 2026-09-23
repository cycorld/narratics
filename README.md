# Narratics (내러틱스)

> **완결까지 흔들리지 않는 서사의 설계도**  
> High-Performance Local-First Novel & Worldbuilding CRDT Engine

---

## 1. 아키텍처 및 철학

- **100% 로컬 퍼스트 (Zero-Network Invariant):** 인터넷 연결이나 서버 계정 없이 단일 `.narr` 파일로 모든 집필, 세계관 관리, 스냅샷 보관이 100% 동작합니다.
- **바인더 트리 CRDT (Kleppmann Tree Move):** 폴더 및 씬의 임의 이동, 중첩, 분기 시 사이클(순환 참조)을 원천 차단하고 다중 기기에서 동일한 트리로 결정론적 수렴.
- **UTF-16 인덱스 기반 yrs 텍스트 엔진:** 웹 브라우저(TipTap/ProseMirror), 데스크톱(Tauri), 모바일 네이티브 환경 간 한글(3바이트 UTF-8) 및 이모지 경계 충돌 없는 무결점 씬 에디팅.
- **단일 파일 SQLite WAL 컨테이너 (`.narr`):** 스크리브너 식의 취약한 디렉터리 번들 충돌 위험을 제거하고, 단일 파일 단위 이동 및 초고속 WAL 영구 보존.

---

## 2. 모노레포 구조

```
narratics/
├── Cargo.toml                  # Workspace Root
├── crates/
│   └── engine-core/            # 핵심 Rust CRDT 엔진 & SQLite WAL 컨테이너
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── tree_crdt.rs    # Kleppmann Tree Move CRDT
│       │   ├── text_engine.rs  # yrs UTF-16 씬 텍스트 엔진 & 컴팩션
│       │   └── container.rs    # .narr SQLite WAL 단일 파일 입출력
│       └── tests/
│           └── adversarial_hardening.rs # 10대 실패 축 적대적 경화 테스트 (0 Defect)
├── README.md
└── AGENTS.md
```

---

## 3. 핵심 벤치마크 및 적대적 경화 검증 지표

10개 실패 축에 대한 적대적 스트레스 테스트 전원 통과:

| 검증 축 (Adversarial Axis) | 테스트 시나리오 | 판정 | 소요 시간 |
| :--- | :--- | :--- | :--- |
| **Cycle 1: Offline Invariant** | 완전 오프라인 격리 집필 및 재접속 검증 | **PASS** | 0.05s |
| **Cycle 2: Boundary & Negative** | 빈 문서 삭제, 음수/초과 범위 슬라이싱 방어 | **PASS** | 0.00s |
| **Cycle 3: Filesystem Hostility** | 특수문자, 따옴표, 깊은 중첩 경로 생성 및 플러시 | **PASS** | 0.05s |
| **Cycle 4: Deep Tree Cycle** | 50단계 조상-자손 순환 참조 이동 시도 사전 차단 | **PASS** | 0.00s |
| **Cycle 5: Poison CRDT Payload** | 랜덤 바이트, 절단된 패킷 디코딩 시 패닉 방어 | **PASS** | 0.06s |
| **Cycle 6: Korean & Astral Emoji** | 3바이트 한글 + 4바이트 이모지 임의 위치 절단/삽입 | **PASS** | 0.00s |
| **Cycle 7: 4-Peer Convergence** | 4대 기기 동시 트리 이동 및 편집의 100% 동일 수렴 | **PASS** | 0.00s |
| **Cycle 8: Tombstone Compaction** | 200회 연속 삭제/수정 후 1.1ms 스쿼시 & 바운디드 메모리 | **PASS** | 0.01s |
| **Cycle 9: SQLite WAL Contention** | 다중 커넥션 동시 트랜잭션 및 원자적 롤백 무결성 | **PASS** | 0.01s |
| **Cycle 10: Schema Parity** | JSON / Binary 와이어 포맷 라운드트립 무손실 | **PASS** | 0.00s |

---

## 4. 빌드 및 테스트

```bash
# 전체 워크스페이스 검증
cargo check

# 10대 적대적 경화 테스트 실행 (Release 모드 0.12초)
cargo test --release -- --nocapture
```
