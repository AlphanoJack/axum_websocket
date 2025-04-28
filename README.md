# axum_websocket

Axum 웹소켓과 gRPC를 활용한 실시간 메시지 전송 시스템

## 프로젝트 개요

이 프로젝트는 Axum 웹소켓 서버와 gRPC를 사용하여 실시간 메시지 전송 시스템을 구현합니다. 주요 기능은 다음과 같습니다:

- 웹소켓 클라이언트와의 핸드쉐이크 및 연결 관리
- gRPC를 통한 메시지 수신
- 수신된 메시지를 연결된 웹소켓 클라이언트에게 전송

## 기술 스택

- Rust
- Axum (웹소켓 서버)
- Tonic (gRPC)
- Tokio (비동기 런타임)

## 프로젝트 구조

```
src/
├── main.rs           # 메인 애플리케이션 진입점
├── websocket.rs      # 웹소켓 핸들러 및 로직
└── grpc.rs           # gRPC 서비스 구현
```

## 시작하기

1. 프로젝트 클론
```bash
git clone [repository-url]
cd axum_websocket
```

2. 의존성 설치
```bash
cargo build
```

3. 서버 실행
```bash
cargo run
```

## API 엔드포인트

- 웹소켓 연결: `ws://localhost:3000/ws`
- gRPC 서비스: `localhost:50051`

