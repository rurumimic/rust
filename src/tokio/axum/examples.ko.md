# axum `examples` 안내서

- github: [axum](https://github.com/tokio-rs/axum): web framework
  - [examples](https://github.com/tokio-rs/axum/tree/main/examples)

이 문서는 `examples/` 폴더의 예제를 "무엇을 보여주는지"보다 "실제 프로젝트에 언제, 왜, 어떤 순서로 가져가야 하는지" 관점에서 정리한 문서다. 목표는 axum으로 웹 서버를 만드는 사람이 처음에는 최소한의 것만 배우고, 필요할 때만 점진적으로 고급 예제로 넘어가도록 돕는 것이다.

## 먼저 알아둘 기준

### 거의 항상 중요한 설정

- `Router::new()`와 `route`, `nest`, `merge`로 라우팅을 조합하는 방식
- `tokio::net::TcpListener::bind`와 `axum::serve(listener, app)`로 서버를 띄우는 기본 구조
- `Path`, `Query`, `Json`, `Form`, `State` 같은 기본 extractor
- `with_state`로 앱 상태를 주입하는 방식
- `IntoResponse` 또는 `Result<_, _>`로 응답/에러를 일관되게 만드는 방식
- `tower_http::trace::TraceLayer` 또는 최소한의 tracing 설정
- 요청 본문 제한, 타임아웃, graceful shutdown 같은 운영 안전장치
- 테스트 가능한 `app()` 함수 분리

### 자주 중요하지만 모든 프로젝트에 필요한 것은 아닌 설정

- `CorsLayer`, `CompressionLayer`, `RequestId` 전파
- 정적 파일 서빙, 템플릿 엔진
- DB pool, 커스텀 extractor, validator
- JWT/OAuth, WebSocket/SSE, TLS
- Prometheus metrics

### 대부분 예제에서 "지금은 덜 중요"한 것

- 데모용 HTML/JS 자산 파일
- 로그 포맷 세부값, `EnvFilter` 문자열 튜닝
- 예제용 하드코딩 포트와 샘플 데이터
- self-signed 인증서 파일 자체
- `println!`, `dbg!` 같은 시연 코드
- hyper/TLS 저수준 연결 루프 보일러플레이트

## 권장 도입 순서

### 1단계. 최소 서버와 기본 API

1. `hello-world`
2. `readme`
3. `routes-and-handlers-close-together`
4. `form`
5. `todos`

이 단계에서 익혀야 할 것:

- 라우트 선언
- 기본 extractor
- JSON/Form 응답
- 간단한 상태 공유
- REST 스타일 핸들러 분해

### 2단계. 에러 처리, 로깅, 테스트

1. `anyhow-error-response`
2. `error-handling`
3. `tracing-aka-logging`
4. `testing`
5. `global-404-handler`

이 단계에서 익혀야 할 것:

- 앱 에러를 HTTP 응답으로 변환하는 방식
- 요청 단위 로그/스팬
- `Router`를 직접 테스트하는 방식
- 공통 fallback

### 3단계. 운영 기본기

1. `graceful-shutdown`
2. `request-id`
3. `cors`
4. `compression`
5. `validator`

이 단계에서 익혀야 할 것:

- 서버 종료 시 요청 처리 보존
- 로그 상관관계용 request id
- 브라우저 연동 시 CORS
- 큰 응답/요청의 압축 처리
- 입력 검증 계층 분리

### 4단계. 실전형 서비스 조합

1. `key-value-store`
2. `dependency-injection`
3. `versioning`
4. `handle-head-request`
5. `static-file-server`

이 단계에서 익혀야 할 것:

- 미들웨어, 제한, 인증, 중첩 라우팅 조합
- 서비스 추상화와 테스트 친화 구조
- API 버저닝
- HEAD/GET 차별화
- SPA/정적 리소스 서빙

### 5단계. 프로젝트 성격에 맞는 가지치기

- HTML 서버면 `templates`, `templates-minijinja`, `multipart-form`, `stream-to-file`
- JSON API 서버면 `sqlx-postgres` 또는 `tokio-postgres`, `jwt`
- 소셜 로그인 필요하면 `oauth`
- 실시간이면 `websockets`, `chat`, `testing-websockets`, `sse`
- 게이트웨이/프록시면 `reqwest-response`, `reverse-proxy`, `http-proxy`

### 6단계. 인프라/저수준/특수 환경

- `serve-with-hyper`
- `unix-domain-socket`
- `tls-rustls`, `tls-graceful-shutdown`
- `low-level-rustls`, `low-level-native-tls`, `low-level-openssl`
- `simple-router-wasm`
- `websockets-http2`

이 단계는 "필요할 때만" 보는 편이 좋다. 대부분의 axum 앱은 여기까지 가지 않아도 충분하다.

## 빠른 추천

### 일반적인 JSON API 서버

추천 순서:

- `hello-world`
- `readme`
- `todos`
- `error-handling`
- `tracing-aka-logging`
- `testing`
- `validator`
- `sqlx-postgres` 또는 `tokio-postgres`
- `jwt`
- `graceful-shutdown`

### 서버 렌더링 HTML 앱

추천 순서:

- `hello-world`
- `form`
- `templates` 또는 `templates-minijinja`
- `static-file-server`
- `multipart-form`
- `stream-to-file`
- `tls-rustls`

### 실시간 기능이 있는 앱

추천 순서:

- `readme`
- `todos`
- `tracing-aka-logging`
- `websockets`
- `chat`
- `testing-websockets`
- `sse`

### 프록시/게이트웨이/인프라형 앱

추천 순서:

- `reqwest-response`
- `reverse-proxy`
- `http-proxy`
- `serve-with-hyper`
- `request-id`
- `prometheus-metrics`

## 예제 카탈로그

난이도 기준:

- `하`: 초반에 바로 읽어도 됨
- `중하`: 기본기 후 바로 활용 가능
- `중`: 프로젝트 뼈대가 잡힌 뒤 적합
- `중상`: 특정 요구사항이 있을 때만 권장
- `상`: 저수준 네트워크/런타임 이해 필요

활용 빈도 기준:

- `높음`: 많은 웹 서비스에서 자주 필요
- `중간`: 특정 종류의 앱에서 자주 필요
- `낮음`: 특수 요구사항일 때만 필요

### A. 시작점과 기본 라우팅

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `hello-world` | 시작점 | axum 최소 실행 구조 이해 | 단일 GET, HTML 응답 | `axum`, `tokio` | axum 시작점 | `Router`, `route`, `TcpListener`, `axum::serve` | `Html` 문자열 내용 | 하 | 높음 |
| `readme` | 시작점 | README 수준의 최소 JSON API | GET, POST, `Json`, 상태코드 반환 | `axum`, `serde` | JSON API 첫 진입 | `Json<T>`, `IntoResponse`, `StatusCode` | tracing 초기화 방식 | 하 | 높음 |
| `routes-and-handlers-close-together` | 구조 | 라우트와 핸들러를 가까이 배치 | `merge`로 작은 라우터 조합 | `axum` | 파일/모듈 구조를 고민할 때 | 작은 `Router` 단위로 합치는 방식 | 헬퍼 `route()` 함수 자체 | 하 | 중간 |
| `form` | 입력 처리 | HTML form과 `Form` extractor 이해 | GET 폼, POST form decode | `axum`, `serde` | 브라우저 폼 처리 | `Form<T>`, `app()` 분리, 테스트 | 예제 HTML 마크업 | 하 | 중간 |
| `todos` | 기본 CRUD | 실전형 REST 구조 입문 | 목록/생성/수정/삭제, 상태 공유 | `axum`, `tower`, `tower-http`, `uuid` | 간단한 API 서버 뼈대 | `Path`, `Query`, `Json`, `State`, `ServiceBuilder` | 인메모리 DB 구현 세부 | 중하 | 높음 |
| `global-404-handler` | 라우팅 | 공통 404 응답 | `fallback` 핸들러 | `axum` | 공통 에러 페이지/메시지 필요 | `fallback` | 데모 HTML | 하 | 중간 |
| `handle-head-request` | HTTP 세부 | HEAD를 GET과 다르게 처리 | `Method` extractor, HEAD 최적화 | `axum`, `http` | body 생성 비용이 클 때 | GET이 HEAD도 처리한다는 점 | 샘플 헤더 값 | 중하 | 중간 |
| `versioning` | API 설계 | 경로 기반 API 버전 추출 | 커스텀 `FromRequestParts` | `axum` | `/v1/...` 스타일 API | 버전을 extractor로 캡슐화 | 버전 enum 표기 자체 | 중하 | 중간 |

### B. 에러 처리와 입력 계층

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `anyhow-error-response` | 에러 처리 | `anyhow`를 앱 에러로 감싸기 | `From<E>`, `IntoResponse` | `axum`, `anyhow` | 빠르게 공통 에러 래핑할 때 | `AppError(anyhow::Error)` 패턴 | 에러 메시지 포맷 | 하 | 높음 |
| `error-handling` | 에러 처리 | 앱 에러, extractor 에러, 내부 로깅 분리 | 커스텀 JSON extractor, middleware logging | `axum`, `tower-http` | API 에러 형식을 일관화할 때 | `IntoResponse`로 앱 에러 표준화, response extension 로깅 | 데모용 `time_library` | 중 | 높음 |
| `customize-extractor-error` | extractor | extractor rejection 커스터마이징 비교 | `WithRejection`, 수동 `FromRequest`, derive `FromRequest` | `axum`, `axum-extra`, `thiserror` | 입력 오류 형식을 서비스 표준으로 맞출 때 | 세 가지 접근의 트레이드오프 | 예제별 `origin` 필드 | 중 | 중간 |
| `customize-path-rejection` | extractor | `Path` 파싱 실패를 더 자세히 노출 | `ErrorKind` 분기, JSON 에러 응답 | `axum`, `serde` | path 파라미터 오류를 친절하게 보여줄 때 | `FromRequestParts`로 `Path` 감싸기 | 모든 `ErrorKind` 분기 세부 | 중상 | 중간 |
| `parse-body-based-on-content-type` | extractor | `Content-Type`에 따라 JSON/Form 분기 | 커스텀 `FromRequest` | `axum` | 동일 endpoint가 여러 입력 형식을 받아야 할 때 | 헤더 보고 extractor 선택 | 지원 미디어 타입 수 확장 | 중 | 중간 |
| `consume-body-in-extractor-or-middleware` | body 처리 | 본문을 먼저 읽고 다시 넣는 패턴 | body buffering, 재구성 | `axum`, `http-body-util` | 서명 검증, 감사 로그, body inspection 필요 시 | body를 소비하면 다시 넣어야 한다는 점 | 디버그 출력 | 중상 | 중간 |
| `validator` | 검증 | extractor + 검증 계층 결합 | `ValidatedForm<T>`, `validator` 통합 | `axum`, `validator`, `thiserror` | 입력 검증을 핸들러 밖으로 뺄 때 | extractor 안에서 decode 후 validate | 예제 메시지 문자열 | 중하 | 높음 |

### C. 관측성과 운영 미들웨어

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `tracing-aka-logging` | 로깅 | 요청 로그와 tracing span 커스터마이징 | `TraceLayer`, `MatchedPath` | `tower-http`, `tracing` | 운영 로그를 구조화할 때 | `TraceLayer::new_for_http()`, matched path 기록 | 각 callback의 빈 구현체 | 중하 | 높음 |
| `request-id` | 로깅/추적 | request id 생성 및 응답 전파 | `SetRequestIdLayer`, `PropagateRequestIdLayer` | `tower-http`, `tracing` | 로그 상관관계, 게이트웨이 연동 | request id를 span과 응답에 같이 넣기 | 헤더 이름 상수 | 중하 | 높음 |
| `print-request-response` | 디버깅 | 요청/응답 본문을 직접 출력 | body collect 후 재구성 | `axum`, `http-body-util` | 개발 중 body 확인이 급할 때 | body를 버퍼링하면 스트리밍 이점이 사라진다는 점 | 출력 형식 | 중 | 중간 |
| `compression` | 성능/전송 | 요청 역압축 + 응답 압축 | `RequestDecompressionLayer`, `CompressionLayer` | `tower-http` | 큰 JSON/text payload 처리 | `Content-Encoding`, `Accept-Encoding` 흐름 | README의 curl 샘플 | 중하 | 중간 |
| `cors` | 브라우저 연동 | 다른 origin에서 API 호출 허용 | `CorsLayer` | `tower-http` | 프론트와 API origin이 다를 때 | 허용 origin/method/header 지정 | 프론트/백엔드 두 서버 데모 구성 | 중하 | 높음 |
| `graceful-shutdown` | 운영 안정성 | 종료 시 기존 요청 마무리 | `with_graceful_shutdown`, `TimeoutLayer` | `axum`, `tower-http`, `tokio::signal` | 배포/재시작 시 요청 유실 방지 | shutdown signal + timeout 조합 | `/forever` 데모 route | 중하 | 높음 |
| `prometheus-metrics` | 모니터링 | Prometheus scrape용 메트릭 노출 | custom middleware, `/metrics` 분리 포트 | `metrics`, `metrics-exporter-prometheus` | 표준 메트릭 수집 필요 시 | path/method/status 라벨링, 공개 포트 분리 | upkeep task 세부 | 중 | 중간 |
| `auto-reload` | 개발 환경 | 재컴파일 시 리스너 유지 | `listenfd`, `systemfd`, `cargo-watch` | `listenfd` | 개발 속도 개선 | 기존 listener 재사용 패턴 | 툴 설치 명령 자체 | 중상 | 중간 |

### D. 서비스 구성, 구조화, 파일/템플릿

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `dependency-injection` | 구조 | 상태 안에 서비스 의존성 주입 | trait object vs generic 비교 | `axum`, `uuid` | 저장소/서비스 추상화가 필요할 때 | `State<AppState>` 안에 repo 넣는 패턴 | generic 버전의 타입 복잡도 | 중 | 중간 |
| `key-value-store` | 종합 예제 | 작은 서비스 안에 미들웨어/제한/인증 결합 | body limit, timeout, compression, admin routes | `axum`, `tower`, `tower-http` | 여러 운영 concern을 한 번에 묶는 법을 볼 때 | route별 layer, nested routes, 상태 공유 | 인메모리 구현 디테일 | 중 | 높음 |
| `static-file-server` | 정적 자산 | 여러 방식의 정적 파일 서빙 비교 | `ServeDir`, `ServeFile`, fallback | `tower-http` | SPA/정적 리소스 제공 시 | `fallback_service`와 `nest_service` 차이 | 여러 포트로 데모를 동시에 띄우는 부분 | 중하 | 높음 |
| `templates` | SSR | Askama로 서버 렌더링 | `Template` derive, `IntoResponse` 래퍼 | `askama`, `axum` | Rust 타입 기반 템플릿 원할 때 | `HtmlTemplate<T>` 래퍼 | 템플릿 HTML 자체 | 중하 | 중간 |
| `templates-minijinja` | SSR | MiniJinja로 레이아웃/페이지 렌더링 | template env를 state로 주입 | `minijinja`, `axum` | Jinja 스타일 템플릿 선호 시 | `Environment`를 앱 상태로 넣는 방식 | 샘플 페이지 텍스트 | 중하 | 중간 |
| `multipart-form` | 업로드 | multipart 업로드와 body limit 설정 | `Multipart`, `DefaultBodyLimit`, `RequestBodyLimitLayer` | `axum`, `tower-http` | 파일 업로드 시작점 | 기본 제한 비활성화 후 별도 제한 부여 | 출력용 `println!` | 중하 | 중간 |
| `stream-to-file` | 업로드/저장 | 요청 스트림을 파일로 직접 저장 | `Multipart`, `StreamReader`, path validation | `axum`, `tokio-util`, `futures-util` | 큰 파일을 메모리에 다 올리지 않고 저장할 때 | 스트림을 `AsyncRead`로 변환, path 검증 | 데모 폼 HTML | 중 | 중간 |

### E. 테스트

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `testing` | 테스트 | 라우터 단위 테스트와 실제 서버 테스트 | `oneshot`, 실서버 spawn, `MockConnectInfo` | `tower`, `http-body-util` | axum 앱을 빠르게 검증할 때 | `app()` 분리, `Router`를 직접 서비스처럼 호출 | trace layer 존재 자체 | 중하 | 높음 |
| `testing-websockets` | 테스트 | WebSocket integration/unit test 비교 | 실제 ws client, channel 기반 mock sink/stream | `tokio-tungstenite`, `futures` | WS 로직을 테스트해야 할 때 | socket split 후 generic 함수로 분리 | 데모 echo 로직 | 중 | 중간 |

### F. 스트리밍과 실시간 통신

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `reqwest-response` | 스트리밍 | 외부 HTTP 응답을 그대로 스트리밍 | reqwest body를 axum `Body`로 전달 | `reqwest`, `tower-http` | 외부 API/파일 다운로드를 프록시할 때 | headers/status 유지 + `Body::from_stream` | 로컬 자기호출 데모 | 중 | 중간 |
| `sse` | 실시간 | Server-Sent Events 기본형 | `Sse`, keep-alive, 정적 자산 | `axum`, `axum-extra`, `tower-http` | 단방향 서버 푸시가 필요할 때 | `Sse::new(stream)`와 keep-alive | demo assets, test client 세부 | 중하 | 중간 |
| `websockets` | 실시간 | WebSocket 기본 패턴 전반 | upgrade, ping/pong, split send/recv | `axum`, `tower-http`, `futures-util` | 양방향 실시간 통신 시작점 | `WebSocketUpgrade`, `on_upgrade`, split 패턴 | 브라우저/Rust 클라이언트 예시 | 중 | 중간 |
| `chat` | 실시간 | broadcast 기반 다자 채팅 | username 관리, broadcast channel | `axum`, `tokio::sync::broadcast` | 채팅/알림 브로드캐스트 구조 참고 | shared state + send/recv task 분리 | demo HTML | 중 | 중간 |
| `websockets-http2` | 실시간/특수 | HTTP/2 WebSocket | TLS, connect protocol, broadcast | `axum-server`, `rustls`, `tokio` | HTTP/2 WS가 꼭 필요할 때 | `enable_connect_protocol()` | self-signed cert와 demo assets | 상 | 낮음 |

### G. 데이터 저장소

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `sqlx-postgres` | DB | SQLx + Postgres pool 사용 | `PgPool`, custom connection extractor | `sqlx` | async SQLx를 쓸 때 | pool을 `State`로 주입하거나 extractor로 분리 | 예제 쿼리 문자열 | 중 | 높음 |
| `tokio-postgres` | DB | raw tokio-postgres pool 패턴 | `bb8` pool, owned connection extractor | `tokio-postgres`, `bb8` | SQLx/Diesel 없이 직접 다룰 때 | pool/connection 분리 방식 | `select 1 + 1` 예제 쿼리 | 중 | 중간 |
| `tokio-redis` | DB/캐시 | Redis 연결 풀 패턴 | `bb8`, `AsyncCommands` | `redis`, `bb8` | 캐시나 세션 저장소가 필요할 때 | pool 검사 후 앱 시작, custom extractor | 예제 key `foo` | 중 | 중간 |
| `mongodb` | DB | Mongo CRUD | `Collection<T>`를 state로 주입 | `mongodb`, `tower-http` | Mongo 기반 서비스일 때 | collection을 상태로 넣는 방식 | 샘플 schema 자체 | 중 | 중간 |
| `diesel-postgres` | DB | Diesel sync + deadpool + migration | `embed_migrations`, `interact` | `diesel`, `deadpool-diesel` | Diesel 기반 조직/기존 코드베이스 | blocking Diesel을 pool `interact`로 분리 | demo schema macro | 중상 | 중간 |
| `diesel-async-postgres` | DB | Diesel async 버전 | async pool, migration, extractor | `diesel-async`, `diesel` | async Diesel을 선호할 때 | async connection pool과 custom extractor | demo schema macro | 중상 | 중간 |

### H. 인증, 인가, 보안

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `jwt` | 인증 | Bearer JWT 발급/검증 | `/authorize`, 커스텀 claims extractor | `jsonwebtoken`, `axum-extra` | API 토큰 인증이 필요할 때 | `FromRequestParts`로 claims 추출 | 데모 자격증명, 만료 시각 값 | 중 | 높음 |
| `oauth` | 인증 | Discord OAuth 로그인 + 세션 | CSRF, session cookie, optional user extractor | `oauth2`, `reqwest`, `async-session`, `axum-extra` | 소셜 로그인/서드파티 인증 필요 시 | CSRF 검증, 세션 저장, `User` extractor | Discord 전용 endpoint와 MemoryStore | 상 | 중간 |
| `tls-rustls` | TLS | rustls 기반 HTTPS + HTTP→HTTPS redirect | `axum_server::bind_rustls` | `axum-server`, `rustls` | 일반적인 HTTPS 서버 시작점 | 인증서 로드, redirect 서버 분리 | self-signed cert 파일 | 중 | 중간 |
| `tls-graceful-shutdown` | TLS/운영 | TLS + graceful shutdown 결합 | `Handle`, redirect 서버 종료 연동 | `axum-server`, `rustls` | HTTPS 서버를 운영 환경답게 종료할 때 | TLS 서버와 redirect 서버를 함께 종료 | 샘플 포트 값 | 중상 | 중간 |
| `low-level-rustls` | TLS/저수준 | tokio-rustls + hyper 수동 연결 | accept loop, handshake, hyper service | `tokio-rustls`, `hyper-util` | `axum_server`보다 더 세밀한 제어가 필요할 때 | TLS accept 후 hyper 연결 생성 | 인증서 파싱 세부 | 상 | 낮음 |
| `low-level-native-tls` | TLS/저수준 | native-tls 기반 저수준 HTTPS | 수동 accept loop | `tokio-native-tls`, `hyper-util` | OS TLS stack을 직접 써야 할 때 | low-level accept/serve 패턴 | 인증서 로드 보일러플레이트 | 상 | 낮음 |
| `low-level-openssl` | TLS/저수준 | OpenSSL 기반 저수준 HTTPS | 수동 handshake, hyper 연결 | `openssl`, `tokio-openssl`, `hyper-util` | OpenSSL 제약/기능이 필요한 환경 | handshake 후 `serve_connection_with_upgrades` | SSL builder 세부 옵션 | 상 | 낮음 |

### I. 프록시, 네트워크, 런타임 특수 사례

| 예제 | 카테고리 | 목적/의도 | 주요 기능 | 주요 라이브러리 | 언제/왜 쓰나 | 꼭 볼 것 | 지금은 덜 중요 | 난이도 | 빈도 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `reverse-proxy` | 프록시 | 들어온 요청을 다른 서버로 전달 | URI 재작성 후 hyper client 요청 | `hyper-util` | 간단한 reverse proxy가 필요할 때 | 기존 request를 재사용해 upstream 호출 | 로컬 데모 서버 | 중 | 중간 |
| `http-proxy` | 프록시/터널 | CONNECT 기반 HTTP proxy | upgrade, TCP tunnel, copy_bidirectional | `hyper`, `tokio` | 프록시 서버 자체를 만들 때 | `CONNECT` 처리와 tunnel | `/` hello route | 상 | 낮음 |
| `serve-with-hyper` | 런타임 | hyper 저수준 API로 axum 서빙 | 직접 accept loop, `ConnectInfo` 노출 | `hyper-util`, `tower` | axum 기본 `serve`보다 더 낮은 제어가 필요할 때 | tower service를 hyper service로 감싸는 법 | 반복되는 accept loop 보일러플레이트 | 상 | 낮음 |
| `unix-domain-socket` | 배포/런타임 | UDS로 axum 서빙 | custom `Connected`, UDS connect info | `tokio`, `hyper-util` | Nginx/systemd/socket activation과 UDS 연동 시 | `into_make_service_with_connect_info`와 UDS metadata | 테스트용 client handshake | 상 | 낮음 |
| `simple-router-wasm` | 서버리스/WASM | wasm 환경에서도 라우팅 사용 | request 하나를 router에 직접 전달 | `axum`, `tower-service` | serverless/wasm 타깃 실험 시 | `default-features = false`와 direct `router.call()` | 예제 `main()` 데모 | 상 | 낮음 |

## 어떤 예제를 우선 실무에 가져갈까

### 거의 모든 axum 프로젝트에서 우선 검토할 예제

- `readme`
- `todos`
- `error-handling`
- `tracing-aka-logging`
- `testing`
- `graceful-shutdown`

### 브라우저가 붙는 API 서버라면 거의 추가로 볼 예제

- `cors`
- `request-id`
- `validator`
- `compression`

### 서버 렌더링/파일 업로드가 있으면 볼 예제

- `templates` 또는 `templates-minijinja`
- `static-file-server`
- `multipart-form`
- `stream-to-file`

### DB가 붙으면 하나만 골라 깊게 보면 되는 예제

- `sqlx-postgres`: async SQL + compile-time 친화
- `tokio-postgres`: raw driver 직접 제어
- `diesel-postgres` 또는 `diesel-async-postgres`: Diesel 생태계 사용 시
- `mongodb`: 문서 DB일 때
- `tokio-redis`: 캐시/세션용

### 특수 요구사항이 생겼을 때만 보는 예제

- `oauth`
- `websockets-http2`
- `serve-with-hyper`
- `http-proxy`
- `unix-domain-socket`
- `simple-router-wasm`
- `low-level-*`

## 읽는 법

### 처음에는 코드보다 구조를 먼저 본다

- 라우터가 어떻게 조합되는지
- 상태가 어디서 주입되는지
- 에러가 어디서 HTTP 응답으로 변환되는지
- 미들웨어가 전역인지 route별인지

### 그 다음에 "이 설정이 내 앱에도 필요한가"를 본다

- 거의 항상 필요하면 바로 채택
- 특정 상황에서만 필요하면 TODO 후보로만 저장
- 데모용 설정이면 그대로 복사하지 말고 의도만 가져간다

## 메모

- `examples/async-graphql`는 현재 이 저장소 안의 완전한 예제가 아니라 외부 예제 저장소 링크만 남아 있다.
- `low-level-*`, `serve-with-hyper`, `http-proxy`, `unix-domain-socket`, `simple-router-wasm`는 axum 학습용이라기보다 런타임/네트워크 세부 제어용 예제에 가깝다.
- 대부분의 axum 앱은 `readme`, `todos`, `error-handling`, `testing`, `tracing-aka-logging`, `graceful-shutdown`, 그리고 자신의 DB/인증 예제 하나 정도면 충분히 시작할 수 있다.
