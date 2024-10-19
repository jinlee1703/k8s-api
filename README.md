# k8s-api

&nbsp; 본 저장소는 k8s 환경에서 실행되도록 설계된 API 서버를 구현한 저장소이다. 백엔드 개발을 위한 언어로는 Rust를 사용하고, 데이터베이스로는 mySQL을 사용한다.

## Tech Stack

- **언어**: Rust 1.82.0
- **패키지 매니저**: Cargo 1.82.0
- **웹 프레임워크**: Actix-web 4.0
- **데이터베이스**: MySQL 9.0.1
- **ORM**: SQLx 0.6
- **직렬화**: Serde 1.0
- **환경 관리**: dotenv 0.15
- **로깅**: env_logger 0.9

## 의존성

본 프로젝트의 주요 의존성은 `./Cargo.toml`에 작성되어 있다.

```toml
[dependencies]
actix-web = "4.0"
sqlx = { version = "0.6", features = ["runtime-actix-native-tls", "mysql"] }
serde = { version = "1.0", features = ["derive"] }
dotenv = "0.15"
env_logger = "0.9"
```

## Build & Run

### 1. 저장소 복제

```bash
git clone https://github.com/jinlee1703/k8s-api.git
cd k8s-api
```

### 2. 환경 변수 설정

```plaintext
DATABASE_URL=mysql://username:password@localhost/dbname
```

### 3. 프로젝트 빌드

```bash
cargo build
```

### 4. 서버 실행

```bash
cargo run
```

&nbsp; 서버는 기본적으로 `http://localhost:8080/`으로 시작된다.

## Docker Build

...

## Deploy

...

## API Endpoints

- `GET` `/api/health`: 헬스 체크 API
- `GET` `/api/items`: 모든 항목 조회
- `POST` `/api/items`: 새 항목 생성
- `GET` `/api/items/{id}`: 특정(id) 항목 조회
- `PUT` `/api/items/{id}`: 특정(id) 항목 수정
- `DELETE` `/api/items/{id}`: 특정(id) 항목 삭제
