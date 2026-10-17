# AGENTS.md

The main source of truth for coding agents (Amp, Claude Code, and others) working in this repository. Read this first, then [`PLAN.md`](PLAN.md) for the ordered next steps and `docs/PROJECT_PLAN.md` for the long-term vision.

_Last updated: 2026-10-16_

## What this project is

An unofficial, educational read-only REST API for HBO's *The Last of Us* TV series, written in Rust with Axum and backed by [Turso](https://docs.turso.tech). It has two purposes: to give learners a realistic API to consume, and to help the author learn Rust and Axum. Prefer clear, idiomatic code over clever abstractions. Add a layer (service, repository, trait) only when it removes real complexity.

The project is not affiliated with HBO, Sony, or Naughty Dog. Do not add copyrighted text or images; see "Data sourcing" and "Image policy" in `docs/PROJECT_PLAN.md`.

## Current state

| Phase (from `docs/PROJECT_PLAN.md`) | Status |
|---|---|
| 1 — Axum foundation | ✅ Done |
| 2 — Static API prototype | ✅ Done |
| 3 — Turso persistence | ✅ Done |
| 4 — Developer experience (OpenAPI, CORS, examples) | ✅ Done |
| 5 — Content expansion (characters, cast, search, images) | 🚧 **In progress** (characters done; cast next, see `PLAN.md`) |
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
| `GET /api/v1/episodes/{id}/characters` | Paginated, alphabetical; `404 episode_not_found` if the episode is missing |
| `GET /api/v1/characters` | Paginated, alphabetical; `?season=<number>` and `?role=main\|recurring\|guest` (with `season`, the billing in that season; an unknown role is a `400`) |
| `GET /api/v1/characters/{slug}` | `404 character_not_found` if missing |
| `GET /api/v1/characters/{slug}/episodes` | Paginated, broadcast order; `?season=` filter; `404` if the character is missing |
| `GET /api/v1/openapi.json` | OpenAPI 3.1 spec for every route above (not for itself or `/docs`) |
| `GET /api/v1/docs` | Interactive Scalar docs (HTML; loads `@scalar/api-reference` pinned on jsDelivr) |
| anything else | `404 route_not_found` |

Every response, errors included, carries `Access-Control-Allow-Origin: *`. Preflight `OPTIONS` requests are answered by the CORS layer before routing; it allows only `GET` and `HEAD`, the `Accept` and `Content-Type` headers, and caches preflights for a day. No credentials are allowed.

Pagination on every collection: `?page=` (≥ 1, default 1) and `?limit=` (1–100, default 20). Pages past the end return `"data": []`. Episodes are ordered by `overall_number`; episode `id` equals `overall_number` (1–16).

### Data

- Seeded by `migrations/0002_seed_seasons_1_and_2.sql`: seasons 1 (2023) and 2 (2025), all 16 episodes, with directors, writers in credited order, and runtimes.
- Every fact was verified against Wikipedia on 2026-10-15. Each row stores `source_name`, `source_url`, and `last_verified_at`; these are not exposed by the API yet.
- Season 1 episode 5's `release_date` is 2023-02-10, its online release on HBO Max (the broadcast moved to 2023-02-12 for the Super Bowl).
- `viewer_count_millions` is `null` everywhere. The only per-episode viewer figures are same-night US linear-TV ratings, which understate total viewing badly.
- `migrations/0003_images_and_external_ids.sql` adds an `images` table (URL, size, alt text, and licence/attribution columns) that `seasons.image_id` and `episodes.image_id` point to, and `tmdb_id`, `imdb_id` and `wikidata_id` columns on both. It drops `episodes.image_url`. No images are licensed yet, so every `image` is `null`.
- `migrations/0004_seed_external_ids.sql` fills in Wikidata ids for both seasons and Wikidata and IMDb ids for all 16 episodes. They were taken from Wikidata (CC0) on 2026-10-16, and the IMDb ids were cross-checked against IMDb's title lookup. Seasons have no `imdb` id, because IMDb doesn't have season ids. `tmdb` is `null` everywhere: TMDB's integer ids can't be verified without an API key. Clients can reach TMDB with series id `100088` plus the season and episode numbers. Sources for these ids are recorded in the migration's header comment, not in `source_*` (those columns describe the Wikipedia facts).
- Seasons and episodes return `"image": null | {url, width, height, alt, attribution: {text, creator, license, license_url, source_url}}` and `"external_ids": {tmdb, imdb, wikidata}`.
- `migrations/0005_create_characters.sql` adds `characters` (slug, name, original description, `image_id`, `source_*`), `character_seasons` (billing per season: `main`/`recurring`/`guest`, with a CHECK) and `episode_characters`. `0006_seed_characters.sql` seeds 34 characters, checked against Wikipedia on 2026-10-16: the characters article's overview table for which characters and which episodes, and the season articles' cast sections for billing. The seed asserts (in its generator) that the two agree; the integration test `character_seasons_match_their_episodes` keeps them in step.
- Episode links follow on-screen credits, minus "credited but does not appear" (Tommy S2E4–5, Jesse S2E4) and plus uncredited appearances (Joel S2E5). Yara and Lev are season 3 only and left out. Names follow the season articles (`ellie`, not `ellie-williams`).
- Characters return `{id, slug, name, description, seasons: [{season, role, episode_count}], image}`. `seasons` is loaded in one query (`load_seasons`), like episode `writers`.
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
src/app.rs             router(state): OpenApiRouter + routes!() for documented handlers, split into
                       Router + spec; /api/v1/openapi.json, /api/v1/docs, fallback, CORS, tracing
src/openapi/           ApiDoc (#[derive(OpenApi)]: info, tags) and docs.html (Scalar page)
src/config.rs          Config / DatabaseConfig from env: PORT, DATABASE_PATH, TURSO_DATABASE_URL, TURSO_AUTH_TOKEN
src/db/mod.rs          Db {Local, Synced}; connect/push; query_all, query_optional, count helpers
src/db/migrations.rs   MIGRATIONS list; run() and pending()
src/error.rs           AppError {NotFound, BadRequest, Internal} → structured JSON; turso::Error → logged 500
src/extract.rs         ApiPath<T> / ApiQuery<T>: Path/Query whose rejections become AppError 400s
src/response.rs        ApiResponse<T>, Paginated<T>, PageMeta, Pagination (validate, offset, page)
src/state.rs           AppState { db: Db }
src/models/            Season, Episode, Character (+ CharacterSeason, Role): Serialize + ToSchema structs with FROM + COLUMNS + from_row;
                       image.rs has Image/Attribution/ExternalIds and the image_columns! macro
src/routes/            health.rs, seasons.rs, episodes.rs, characters.rs: handlers with their SQL inline;
                       episodes::fetch_page(EpisodeQuery) and characters::fetch_page(CharacterQuery) are shared
migrations/            0001 schema, 0002 seed (verified data), 0003 images + external ids schema,
                       0004 external id data, 0005 characters schema, 0006 character data
tests/api.rs           Integration tests: fresh migrated in-memory Db per test, router via oneshot
README.md              Human-facing overview: quick start, endpoints, configuration
PLAN.md                Ordered next steps and recent decisions
.github/workflows/ci.yml  CI: fmt, clippy, test on stable; cargo check on the MSRV (1.90)
docs/PROJECT_PLAN.md   Vision, data model, API conventions, roadmap
docs/USING_THE_API.md  Client-facing behaviour: shapes, pagination, errors, spoilers, rate limits
docs/EXAMPLES.md       curl, JS fetch, Python and Rust clients (all run against a live server)
docs/SOURCES.md        Data sources, attribution and reuse
.agents/setup, .agents/resume  Amp orb lifecycle scripts (install/check the Rust toolchain)
```

## Conventions to follow

- **Responses.** A single resource is `{"data": ...}` (`ApiResponse`). A collection is `{"data": [...], "meta": {page, limit, total, total_pages}}` (`Paginated`, built with `pagination.page(rows, total)`). Collections are always paginated.
- **Errors.** Return `Result<_, AppError>` from handlers. `?` on a `turso::Error` logs it and returns a generic `500 internal_error`, so never put database details in a response. Error codes are `snake_case` (`episode_not_found`, `character_not_found`, `invalid_pagination`, `invalid_path_parameter`, `invalid_query_parameter`, `route_not_found`, `internal_error`). Messages are full sentences, e.g. `"Episode 99 was not found."`.
- **SQL.** Write explicit SQL in the handler (or a small shared function like `episodes::fetch_page`). Always bind values as `?1, ?2…` parameters and never interpolate input; the only `format!` in SQL is for `Model::COLUMNS` constants. Filter and paginate in SQL (`LIMIT`/`OFFSET` plus a `COUNT(*)`). Avoid N+1 queries; see `load_writers` and `load_seasons`, which pass ids as one JSON array (`db::json_ids`) to `json_each`. Optional filters use `(?1 IS NULL OR ...)`, and shared page queries take a small `*Query` struct (`EpisodeQuery`, `CharacterQuery`).
- **Models.** Integers are `i64` and reals `f64`, matching SQLite. Nullable columns are `Option<T>`. Each model has a `COLUMNS` constant and a `from_row` that reads columns in the same order. Models with an image also have a `FROM` constant (the table plus `LEFT JOIN images i`); select `COLUMNS` from it so the image columns resolve.
- **Images.** Only add images whose licence allows redistribution (Wikimedia Commons, public domain, or written permission; see "Image policy" in `docs/PROJECT_PLAN.md`). Never add HBO stills, posters or screenshots, and never pass TMDB images through the API: TMDB's licence can't be sublicensed. Fill in `external_ids` instead, so clients can fetch artwork themselves. Every `images` row needs `source_page_url`, `license_name` and `attribution_text`. Add images in new migrations.
- **Extractors.** Use `ApiPath` and `ApiQuery` (not Axum's `Path` and `Query`) so bad input gets a structured 400. Path ids and query numbers are `u32`, which rejects negatives. A handler can take several `ApiQuery` extractors, e.g. a filter struct plus `Pagination`. Don't use `#[serde(flatten)]` with query structs: flatten breaks numeric parsing in `serde_urlencoded`.
- **Data values.** Unknown values are `null`, never `""`. Dates are ISO 8601 strings (`2023-01-15`). Human-facing lookups use slugs (`ellie-williams`), and episodes use numeric ids. Every new fact needs a source; record it in the row's `source_*` and `last_verified_at` columns. Summaries must be original writing.
- **Routes.** Public routes live under `/api/v1`, nested in `app.rs`. Register every data route with `.routes(routes!(handler))` on the `OpenApiRouter`, never with plain `.route`, so it lands in the spec. `routes!` takes handlers for one path only.
- **OpenAPI.** Every handler gets `#[utoipa::path(...)]` with a `tag`, its params (`IntoParams` structs for queries, tuples for path ids), and every response it can return: `200` plus each `400`/`404`/`500` as `body = ErrorResponse`, with the possible error codes in the description. The doc comment's first line is the summary. Response models derive `ToSchema`; `Option` fields get `#[schema(required)]` because they are always serialized (as `null`). Doc comments on schema fields are public docs, so put implementation notes in `//` comments. The spec test in `tests/api.rs` lists every documented path; update it with new routes.
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

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` before every commit. All three must pass; CI runs the same checks (with `--locked`) on every pull request, plus `cargo check` on Rust 1.90. Keep `rust-version` in `Cargo.toml` and the MSRV job in `ci.yml` in step when a dependency raises it. The first build compiles the Turso engine and takes a few minutes.

## Next steps

[`PLAN.md`](PLAN.md) has the ordered list, with design notes. In short:

1. **Cast members** (`cast_members`, `character_cast` per season, `/cast` endpoints, `portrayed_by` on characters) and the first licensed images (Commons photos of cast).
2. **Episode filters** such as `?director=`, ideally via a `people` table for crew.
3. **Provenance:** a `source` object on every resource.
4. **Search:** `GET /api/v1/search?q=`.
5. More characters (one-off guests), then season 3 when it airs.
6. Deployment (Phase 6), including a periodic `pull` for synced replicas.

When an endpoint or behaviour changes, keep `docs/USING_THE_API.md`, `docs/EXAMPLES.md`, `docs/SOURCES.md`, `README.md` and `PLAN.md` in step with it.

## History

Development started with Amp (the `.agents/` scripts are its orb setup/resume hooks) and Delta, and continued with Claude Code. Commits so far: cargo scaffold → Amp orb scripts → project plan → Phase 1 foundation → Phase 2 prototype → docs (AGENTS.md, CLAUDE.md) → Phase 3 Turso persistence → images and external ids → CORS → OpenAPI, examples and behaviour docs (Phase 4) → CI → characters and episode links (Phase 5).
