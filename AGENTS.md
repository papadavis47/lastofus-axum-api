# AGENTS.md

The main source of truth for coding agents (Amp, Claude Code, and others) working in this repository. Read this first, then `docs/PROJECT_PLAN.md` for the long-term vision.

_Last updated: 2026-10-15_

## What this project is

An unofficial, educational read-only REST API for HBO's *The Last of Us* TV series, written in Rust with Axum and backed by [Turso](https://docs.turso.tech). It has two purposes: to give learners a realistic API to consume, and to help the author learn Rust and Axum. Prefer clear, idiomatic code over clever abstractions. Add a layer (service, repository, trait) only when it removes real complexity.

The project is not affiliated with HBO, Sony, or Naughty Dog. Do not add copyrighted text or images; see "Data sourcing" and "Image policy" in `docs/PROJECT_PLAN.md`.

## Current state

| Phase (from `docs/PROJECT_PLAN.md`) | Status |
|---|---|
| 1 — Axum foundation | ✅ Done |
| 2 — Static API prototype | ✅ Done |
| 3 — Turso persistence | ✅ Done |
| 4 — Developer experience (OpenAPI, CORS, examples) | ⏭️ **Next** |
| 5 — Content expansion (characters, cast, search, images) | Not started |
| 6 — Deployment | Not started |

### Live endpoints

| Method & path | Notes |
|---|---|
| `GET /health` | `{"data": {"status": "ok"}}` (does not touch the database) |
| `GET /api/v1/seasons` | Paginated |
| `GET /api/v1/seasons/{season_number}` | `404 season_not_found` if missing |
| `GET /api/v1/seasons/{season_number}/episodes` | Paginated; `404` if the season is missing |
| `GET /api/v1/episodes` | Paginated; `?season=<number>` filter (an unknown season gives an empty page, not a 404) |
| `GET /api/v1/episodes/{id}` | `404 episode_not_found` if missing |
| anything else | `404 route_not_found` |

Pagination on every collection: `?page=` (≥ 1, default 1) and `?limit=` (1–100, default 20). Pages past the end return `"data": []`. Episodes are ordered by `overall_number`; episode `id` equals `overall_number` (1–16).

### Data

- Seeded by `migrations/0002_seed_seasons_1_and_2.sql`: seasons 1 (2023) and 2 (2025), all 16 episodes, with directors, writers in credited order, and runtimes.
- Every fact was verified against Wikipedia on 2026-10-15. Each row stores `source_name`, `source_url`, and `last_verified_at`; these are not exposed by the API yet.
- Season 1 episode 5's `release_date` is 2023-02-10, its online release on HBO Max (the broadcast moved to 2023-02-12 for the Super Bowl).
- `viewer_count_millions` and `image_url` are `null` everywhere. The only per-episode viewer figures are same-night US linear-TV ratings, which understate total viewing badly. No images are licensed yet.
- Season 3 is expected in 2027. Add it in a new migration once episodes air.

## Database (Turso)

The app uses the [Turso Database](https://docs.turso.tech/tursodb/quickstart) engine through the `turso` crate (0.8.x, pre-1.0). SQLx is **not** used: it has no Turso driver.

`Db` (`src/db/mod.rs`) has two modes, chosen by environment variables:

| Mode | When | Behaviour |
|---|---|---|
| `Db::Local` | `TURSO_DATABASE_URL` unset | Embedded file at `DATABASE_PATH` (default `lastofus.db`). The server runs pending migrations on startup. Tests use `Db::in_memory()`. |
| `Db::Synced` | `TURSO_DATABASE_URL` set | Local replica at `DATABASE_PATH` that syncs with Turso Cloud. On startup it downloads or `pull`s the remote database. The server never writes to it, and refuses to start if migrations are missing. |

Both modes hand out the same `turso::Connection` type, so handlers don't care which is in use. Each request opens its own connection with `state.db.connect().await?`.

**Migrations.** These are numbered `.sql` files in `migrations/`, registered in `MIGRATIONS` in `src/db/migrations.rs` and embedded with `include_str!`. Each one runs in a transaction with foreign keys enforced. Applied versions are recorded in `_migrations`, because `PRAGMA user_version` is read-only on Turso Cloud. Never edit an applied migration; add a new one, data changes included. `cargo run --bin migrate` applies them; in synced mode it then `push`es them to Turso Cloud, which is how schema and data changes reach production.

**Gotchas.**
- A replica file is tied to one remote. Delete the `DATABASE_PATH` file and its `-wal`, `-changes` and `-info` siblings if you change `TURSO_DATABASE_URL`.
- The server only pulls at startup. Restart it to pick up new data; periodic pulls aren't implemented.
- `*.db` and `*.db-*` are gitignored.

**Testing sync without an account.** Install the `tursodb` CLI and start a local sync server, then point `TURSO_DATABASE_URL` at it (no token needed):

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/tursodatabase/turso/releases/latest/download/turso_cli-installer.sh | sh
~/.turso/tursodb :memory: --sync-server 127.0.0.1:8080
TURSO_DATABASE_URL=http://127.0.0.1:8080 DATABASE_PATH=writer.db cargo run --bin migrate
TURSO_DATABASE_URL=http://127.0.0.1:8080 DATABASE_PATH=reader.db cargo run
```

**Real Turso Cloud.** Create the database with the Turso engine (`turso db create <name> --tursodb`, not plain `turso db create`, which makes a libSQL database). Set `TURSO_DATABASE_URL` and `TURSO_AUTH_TOKEN` from `turso db show <name> --url` and `turso db tokens create <name>`.

## Code map

```text
src/lib.rs             Library crate root; tests/ and src/bin/ import from here
src/main.rs            Server binary: tracing, Config, Db::open, migrate-or-check, serve, graceful shutdown
src/bin/migrate.rs     `cargo run --bin migrate`: apply migrations, push when synced
src/app.rs             router(state): /health, nested /api/v1 routes, fallback, TraceLayer
src/config.rs          Config / DatabaseConfig from env: PORT, DATABASE_PATH, TURSO_DATABASE_URL, TURSO_AUTH_TOKEN
src/db/mod.rs          Db {Local, Synced}; connect/push; query_all, query_optional, count helpers
src/db/migrations.rs   MIGRATIONS list; run() and pending()
src/error.rs           AppError {NotFound, BadRequest, Internal} → structured JSON; turso::Error → logged 500
src/extract.rs         ApiPath<T> / ApiQuery<T>: Path/Query whose rejections become AppError 400s
src/response.rs        ApiResponse<T>, Paginated<T>, PageMeta, Pagination (validate, offset, page)
src/state.rs           AppState { db: Db }
src/models/            Season, Episode: Serialize structs with COLUMNS + from_row(&turso::Row)
src/routes/            health.rs, seasons.rs, episodes.rs: handlers with their SQL inline
migrations/            0001 schema, 0002 seed (verified data)
tests/api.rs           Integration tests: fresh migrated in-memory Db per test, router via oneshot
README.md              Human-facing overview: quick start, endpoints, configuration
docs/PROJECT_PLAN.md   Vision, data model, API conventions, roadmap
.agents/setup, .agents/resume  Amp orb lifecycle scripts (install/check the Rust toolchain)
```

## Conventions to follow

- **Responses.** A single resource is `{"data": ...}` (`ApiResponse`). A collection is `{"data": [...], "meta": {page, limit, total, total_pages}}` (`Paginated`, built with `pagination.page(rows, total)`). Collections are always paginated.
- **Errors.** Return `Result<_, AppError>` from handlers. `?` on a `turso::Error` logs it and returns a generic `500 internal_error`, so never put database details in a response. Error codes are `snake_case` (`episode_not_found`, `invalid_pagination`, `invalid_path_parameter`, `invalid_query_parameter`, `route_not_found`, `internal_error`). Messages are full sentences, e.g. `"Episode 99 was not found."`.
- **SQL.** Write explicit SQL in the handler (or a small shared function like `episodes::fetch_page`). Always bind values as `?1, ?2…` parameters and never interpolate input; the only `format!` in SQL is for `Model::COLUMNS` constants. Filter and paginate in SQL (`LIMIT`/`OFFSET` plus a `COUNT(*)`). Avoid N+1 queries; see `load_writers`, which passes ids as a JSON array to `json_each`.
- **Models.** Integers are `i64` and reals `f64`, matching SQLite. Nullable columns are `Option<T>`. Each model has a `COLUMNS` constant and a `from_row` that reads columns in the same order.
- **Extractors.** Use `ApiPath` and `ApiQuery` (not Axum's `Path` and `Query`) so bad input gets a structured 400. Path ids and query numbers are `u32`, which rejects negatives. A handler can take several `ApiQuery` extractors, e.g. a filter struct plus `Pagination`. Don't use `#[serde(flatten)]` with query structs: flatten breaks numeric parsing in `serde_urlencoded`.
- **Data values.** Unknown values are `null`, never `""`. Dates are ISO 8601 strings (`2023-01-15`). Human-facing lookups use slugs (`ellie-williams`), and episodes use numeric ids. Every new fact needs a source; record it in the row's `source_*` and `last_verified_at` columns. Summaries must be original writing.
- **Routes.** Public routes live under `/api/v1`, nested in `app.rs`.
- **Tests.** Every endpoint gets integration tests in `tests/` covering success, 404s, and 400s. Tests use `Db::in_memory()` plus `migrations::run` and never bind a port or touch the network.
- **Style.** Rust 2024 edition. Keep `cargo fmt` and `cargo clippy --all-targets -- -D warnings` clean.

## Commands

```bash
cargo run                                    # serves on 0.0.0.0:3000 using ./lastofus.db (auto-migrated)
PORT=8080 RUST_LOG=debug cargo run           # custom port and log level
cargo run --bin migrate                      # apply migrations (and push, when TURSO_DATABASE_URL is set)
cargo test                                   # integration tests against in-memory Turso
cargo fmt
cargo clippy --all-targets -- -D warnings
curl -s 'localhost:3000/api/v1/episodes?season=2' | jq
```

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` before every commit. All three must pass. The first build compiles the Turso engine and takes a few minutes.

## Next steps (Phase 4: developer experience)

1. **CORS:** add `tower-http`'s `cors` feature and a `CorsLayer` allowing `GET` from any origin (it's a public, read-only API). Test it in `tests/`.
2. **OpenAPI:** generate a spec (e.g. `utoipa` + `utoipa-axum`) for every endpoint, with response and error schemas. Serve it at `/api/v1/openapi.json` with an interactive docs page.
3. **Examples:** `curl`, JavaScript (`fetch`), Python and Rust client examples in `docs/`.
4. **Behaviour docs:** document pagination, errors, spoiler policy and the unofficial-project disclaimer, and add a sources/attribution page or endpoint (the provenance columns already exist).

Later:
- Characters and cast tables and endpoints (Phase 5).
- A periodic `pull` for synced replicas.
- A container image and deployment (Phase 6).

## History

Development started with Amp (the `.agents/` scripts are its orb setup/resume hooks) and Delta, and continued with Claude Code. Commits so far: cargo scaffold → Amp orb scripts → project plan → Phase 1 foundation → Phase 2 prototype → docs (AGENTS.md, CLAUDE.md) → Phase 3 Turso persistence.
