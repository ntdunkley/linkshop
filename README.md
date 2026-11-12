# Linkshop

Linkshop is a lightweight, multi-tenant web platform designed for small businesses that operate primarily through WhatsApp (e.g. home cooks, independent bakers, makers, and local service providers). It provides each business with a dedicated, mobile-first public page, a shareable link, and an interactive menu/catalogue where customers can browse items and submit order requests.

For detailed architecture, user personas, MVP scope, and product roadmap, refer to the [Project Plan](docs/project-plan.md).

---

## Architecture Overview

Linkshop is organized as a Cargo workspace:

- **`crates/linkshop`**: Core web backend built with [Actix Web](https://actix.rs/) and [SQLx](https://github.com/launchbadge/sqlx) for PostgreSQL.
- **`crates/oidc-login`**: OIDC / Google OAuth authentication crate for tenant owner management.
- **`docker-compose.yml`**: Multi-container setup for local PostgreSQL backing services and full-stack runs.

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (1.85+ with 2024 edition support)
- [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/)

### 1. Configure Environment

Copy the example environment file:

```bash
cp .env.example .env
```

Default connection strings point to local PostgreSQL on port `5432` with databases `linkshop` (application) and `linkshop_test` (test suite).

---

## Running the Project Locally

You can run Linkshop locally using either of the following workflows:

### Option A: Local Cargo + Dockerized PostgreSQL (Recommended)

This workflow gives you the fastest feedback loop with incremental compilation and IDE diagnostics.

1. **Start PostgreSQL in the background:**

   ```bash
   docker compose up -d postgres
   ```

   *Note: On first startup, the init script automatically creates both `linkshop` and `linkshop_test` databases.*

2. **Run the Linkshop server:**

   ```bash
   cargo run -p linkshop
   ```

   The application automatically runs pending database migrations on startup and starts listening on `http://127.0.0.1:8080`.

3. **Verify server health:**

   ```bash
   curl http://127.0.0.1:8080/health
   # Response: {"status":"ok"}
   ```

---

### Option B: Full Stack via Docker Compose

To build and run both PostgreSQL and the Linkshop application inside containers:

```bash
docker compose up --build
```

The application container waits for PostgreSQL to pass its healthcheck before launching. The server will be accessible on `http://127.0.0.1:8080`.

To stop containers:

```bash
docker compose down
```

---

## Running Tests

Linkshop includes both unit tests and database integration tests. Integration tests execute against the `linkshop_test` database.

1. Ensure PostgreSQL is running:

   ```bash
   docker compose up -d postgres
   ```

2. Run the test suite:

   ```bash
   cargo test
   ```

---

## Code Quality & Guidelines

Before committing changes, ensure your code complies with project conventions defined in [AGENTS.md](AGENTS.md):

```bash
cargo fmt --check
cargo clippy --all-targets
```
