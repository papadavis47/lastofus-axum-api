# AGENTS.md

The main source of truth for coding agents (Amp, Claude Code, and others) working in this repository. Read this first, then `docs/PROJECT_PLAN.md` for the long-term vision.

_Last updated: 2026-10-15_

## What this project is

An unofficial, educational read-only REST API for HBO's *The Last of Us* TV series, written in Rust with Axum. It has two purposes: to give learners a realistic API to consume, and to help the author learn Rust and Axum. Prefer clear, idiomatic code over clever abstractions. Add a layer (service, repository, trait) only when it removes real complexity.

The project is not affiliated with HBO, Sony, or Naughty Dog. Do not add copyrighted text or images; see "Data sourcing" and "Image policy" in `docs/PROJECT_PLAN.md`.

## Current state

| Phase (from `docs/PROJECT_PLAN.md`) | Status |
|---|---|
| 1 — Axum foundation | ✅ Done |
| 2 — Static API prototype | ✅ Done |
| 3 — SQLite persistence | ⏭️ **Next** |
| 4 — Developer experience (OpenAPI, CORS, examples) | Not started |
| 5 — Content expansion (characters, cast, search, images) | Not started |
| 6 — Deployment | Not started |

### Live endpoints

All data currently comes from in-memory fixtures in `src/fixtures.rs`.

| Method & path | Notes |
|---|---|
| `GET /health` | `{"data": {"status": "ok"}}` |
| `GET /api/v1/seasons` | Paginated |
| `GET /api/v1/seasons/{season_number}` | `404 season_not_found` if missing |
| `GET /api/v1/seasons/{season_number}/episodes` | Paginated; `404` if the season is missing |
| `GET /api/v1/episodes` | Paginated; `?season=<number>` filter (an unknown season gives an empty page, not a 404) |
| `GET /api/v1/episodes/{id}` | `404 episode_not_found` if missing |
| anything else | `404 route_not_found` |

Pagination on every collection: `?page=` (≥ 1, default 1) and `?limit=` (1–100, default 20). Pages past the end return `"data": []`.

### Fixture data

- Seasons 1 (2023, 9 episodes) and 2 (2025, 7 episodes).
- Episodes: Season 1 only (ids 1–9). Season 2 episodes are intentionally missing, so `/seasons/2/episodes` is empty for now.
- `runtime_minutes`, `viewer_count_millions`, and `image_url` are `null` everywhere.
- **Unverified.** Titles, dates, directors, and writers were entered without checking a source, because the session that wrote them could not reach Wikipedia. Verify every value before it becomes the Phase 3 seed.

## Code map

```text
src/lib.rs        Library crate root; tests/ imports from here
src/main.rs       Binary: tracing setup, Config, bind, serve, graceful shutdown
src/app.rs        router(state): /health, nested /api/v1 routes, fallback, TraceLayer
src/config.rs     Config::from_env() — PORT (default 3000), binds 0.0.0.0
src/error.rs      AppError {NotFound, BadRequest, Internal} → structured JSON error
src/extract.rs    ApiPath<T> / ApiQuery<T>: Path/Query whose rejections become AppError 400s
src/response.rs   ApiResponse<T>, Paginated<T>, PageMeta, Pagination (validate + paginate)
src/state.rs      AppState { seasons: Arc<[Season]>, episodes: Arc<[Episode]> }
src/fixtures.rs   Phase 2 in-memory data (to be replaced by SQLite)
src/models/       Season, Episode (Serialize structs matching the plan's data model)
src/routes/       health.rs, seasons.rs, episodes.rs — one async fn per endpoint
tests/api.rs      Integration tests driving the router with tower::ServiceExt::oneshot
docs/PROJECT_PLAN.md  Vision, data model, API conventions, roadmap
.agents/setup, .agents/resume  Amp orb lifecycle scripts (install/check the Rust toolchain)
```

## Conventions to follow

- **Responses.** A single resource is `{"data": ...}` (`ApiResponse`). A collection is `{"data": [...], "meta": {page, limit, total, total_pages}}` (`Paginated`). Collections are always paginated.
- **Errors.** Return `Result<_, AppError>` from handlers. Never return plain-text errors. Error codes are `snake_case` (`episode_not_found`, `invalid_pagination`, `invalid_path_parameter`, `invalid_query_parameter`, `route_not_found`, `internal_error`). Messages are full sentences, e.g. `"Episode 99 was not found."`.
- **Extractors.** Use `ApiPath` and `ApiQuery` (not Axum's `Path` and `Query`) so bad input gets a structured 400. A handler can take several `ApiQuery` extractors, e.g. a filter struct plus `Pagination`. Don't use `#[serde(flatten)]` with query structs: flatten breaks numeric parsing in `serde_urlencoded`.
- **Data values.** Unknown values are `null`, never `""`. Dates are ISO 8601 strings (`2023-01-15`). Human-facing lookups use slugs (`ellie-williams`), and episodes use numeric ids.
- **Routes.** Public routes live under `/api/v1`, nested in `app.rs`.
- **Tests.** Every endpoint gets integration tests in `tests/` covering success, 404s, and 400s. Tests never bind a port.
- **Style.** Rust 2024 edition. Keep `cargo fmt` and `cargo clippy --all-targets -- -D warnings` clean.

## Commands

```bash
cargo run                                    # serves on 0.0.0.0:3000
PORT=8080 RUST_LOG=debug cargo run           # custom port and log level
cargo test                                   # unit + integration tests
cargo fmt
cargo clippy --all-targets -- -D warnings
curl -s localhost:3000/api/v1/episodes?season=1 | jq
```

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` before every commit. All three must pass.

## Next steps (Phase 3: SQLite persistence)

1. Add `sqlx` (features: `runtime-tokio`, `sqlite`, `migrate`) and a `DATABASE_URL` setting in `config.rs`.
2. Create `migrations/` with normalized `seasons` and `episodes` tables and foreign keys. Store `writers` in a join table or a separate `episode_writers` table, not as a delimited string. Leave room for `source_url`, `source_name`, and `last_verified_at`.
3. Verify the fixture data against sources, write a seed migration, and add Season 2's episodes.
4. Change `AppState` to hold an `SqlitePool`. Move the in-memory filtering and pagination into SQL (`LIMIT`/`OFFSET` plus a `COUNT(*)`). Map `sqlx::Error` to `AppError::Internal`, log the error, and never leak its details to clients.
5. Delete `src/fixtures.rs`. Point tests at an in-memory SQLite database that is migrated and seeded per test.
6. Keep every existing response shape and test assertion unchanged. The tests are the contract.

After Phase 3: CORS (`tower-http` `cors`), OpenAPI documentation, and then characters and cast (Phases 4–5).

## History

Development started with Amp (the `.agents/` scripts are its orb setup/resume hooks) and Delta, and continued with Claude Code. Commits so far: cargo scaffold → Amp orb scripts → project plan → Phase 1 foundation → Phase 2 prototype.
