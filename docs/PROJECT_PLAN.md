# The Last of Us REST API — Project Plan

## Current status

_Last updated: 2026-10-16_

| Phase | Status |
|---|---|
| 1 — Axum foundation | ✅ Done |
| 2 — Static API prototype | ✅ Done |
| 3 — Turso persistence | ✅ Done (seasons 1–2 and all 16 episodes, verified) |
| 4 — Developer experience | ✅ Done: CORS, OpenAPI spec and Scalar docs, client examples, behaviour and sources docs; image schema and `image`/`external_ids` fields (migrations 0003–0004) |
| 5 — Content expansion | 🚧 In progress: characters, per-season billing and episode links done (migrations 0005–0006); cast next. See [`PLAN.md`](../PLAN.md) |
| 6 — Deployment | Not started |

See [`AGENTS.md`](../AGENTS.md) for the live endpoint list, code layout, and known gaps.

## Vision

Build an unofficial, educational REST API containing curated factual information about HBO's *The Last of Us* television series. The API is intended for students and beginner-to-intermediate developers who want to practice consuming a realistic API while the project itself provides an opportunity to learn Rust and the [Axum](https://docs.rs/axum/latest/axum/) framework.

The project is not affiliated with or endorsed by HBO, Max, Sony, Naughty Dog, or the series' creators.

## Goals

- Provide a predictable, well-documented JSON API that is approachable for learners.
- Model useful relationships among seasons, episodes, characters, and cast members.
- Demonstrate idiomatic Axum concepts such as routers, handlers, extractors, shared state, responses, and Tower middleware.
- Offer filtering, pagination, search, and structured errors for frontend practice.
- Maintain clear source attribution and use only content and images that can legally be redistributed.

## Non-goals for the first release

- User accounts, authentication, or user-submitted content.
- A public administration interface or full content-management system.
- Reviews, user ratings, awards, or production trivia.
- Automatic scraping from Wikipedia, IMDb, HBO, or other websites.
- Hosting copyrighted promotional stills, posters, logos, or screenshots without permission.

## MVP resources

The first release will focus on:

1. Seasons
2. Episodes
3. Characters
4. Cast members

### Proposed endpoints

```text
GET /health

GET /api/v1/seasons
GET /api/v1/seasons/{season_number}
GET /api/v1/seasons/{season_number}/episodes

GET /api/v1/episodes
GET /api/v1/episodes/{id}
GET /api/v1/episodes/{id}/characters

GET /api/v1/characters
GET /api/v1/characters/{slug}
GET /api/v1/characters/{slug}/episodes

GET /api/v1/cast
GET /api/v1/cast/{slug}

GET /api/v1/search?q=ellie
```

Example filters:

```text
GET /api/v1/episodes?season=1
GET /api/v1/episodes?director=ali-abbasi
GET /api/v1/characters?role=main
GET /api/v1/cast?season=2
GET /api/v1/episodes?page=1&limit=20
```

Human-facing resources should use URL-safe slugs such as `ellie-williams`. Numeric IDs can remain the internal database identifiers.

## Initial data model

### Season

- `id`
- `number`
- `title`
- `release_year`
- `episode_count`
- `summary`
- `image` (optional, see "Image policy")
- `external_ids` (`tmdb`, `imdb`, `wikidata`)

### Episode

- `id`
- `season_id`
- `number_in_season`
- `overall_number`
- `title`
- `slug`
- `summary`
- `runtime_minutes`
- `release_date`
- `director`
- `writers`
- `viewer_count_millions` (optional)
- `image` (optional, see "Image policy")
- `external_ids` (`tmdb`, `imdb`, `wikidata`)

### Character

- `id`
- `name`
- `slug`
- `description`
- `seasons`: one entry per season with `role` (`main`, `recurring`, or `guest`) and `episode_count`. This replaced the original `role_type`, `first_season` and `last_season`, because billing changes between seasons.
- `image` (optional, see "Image policy")

### Cast member

- `id`
- `name`
- `slug`
- `biography`
- `image_url` (optional)

Join tables such as `character_cast` and `episode_characters` will represent many-to-many relationships. Character-to-cast relationships should support a season or date range because casting can change over time.

## API conventions

### Successful resource

```json
{
  "data": {
    "id": 1,
    "title": "When You're Lost in the Darkness",
    "season": 1,
    "episode": 1
  }
}
```

### Paginated collection

```json
{
  "data": [],
  "meta": {
    "page": 1,
    "limit": 20,
    "total": 16,
    "total_pages": 1
  }
}
```

### Error

```json
{
  "error": {
    "status": 404,
    "code": "episode_not_found",
    "message": "Episode 99 was not found."
  }
}
```

Conventions:

- Use ISO 8601 dates such as `2023-01-15`.
- Represent unknown values as `null`, not empty strings.
- Use `200` for successful reads, `400` for invalid input, `404` for missing resources, and `500` for unexpected failures.
- Enforce a maximum page size.
- Clearly document whether summaries contain spoilers.
- Version public routes under `/api/v1`.

## Technical direction

### Stack

- Rust 2024 edition
- Axum and Tokio
- Serde and `serde_json`
- [Turso Database](https://docs.turso.tech/tursodb/quickstart) (SQLite-compatible, written in Rust) through the `turso` crate, with explicit SQL
- Turso Cloud for production, reached as a synced local replica (`turso::sync`)
- `tower-http` for CORS, tracing, compression, and request IDs
- `tracing` and `tracing-subscriber` for structured logs
- `thiserror` for typed application errors

Turso keeps the simplicity of SQLite for a mostly static dataset: development and tests use an embedded database file or `:memory:` with no server or account, while production reads from a local replica that pulls from Turso Cloud. SQLx was the original plan but has no Turso driver, so queries are written against the `turso` crate directly and migrations use a small built-in runner. The Turso engine is pre-1.0 (0.8.x); libSQL with the `libsql` crate is the conservative fallback if it causes problems.

### Source layout

Files marked _(planned)_ do not exist yet.

```text
src/
├── lib.rs          # library crate; integration tests import from here
├── main.rs         # thin binary: logging, config, serve, graceful shutdown
├── app.rs          # router construction
├── config.rs       # environment configuration (PORT, DATABASE_PATH, TURSO_*)
├── error.rs        # AppError and the structured error body
├── extract.rs      # ApiPath / ApiQuery extractors that reject with AppError
├── response.rs     # ApiResponse, Paginated, Pagination
├── state.rs        # AppState shared with handlers
├── bin/
│   └── migrate.rs  # applies migrations; pushes to Turso Cloud in synced mode
├── openapi/        # ApiDoc (spec info and tags) and the docs page HTML
├── models/
│   ├── season.rs
│   ├── episode.rs
│   ├── character.rs
│   └── cast_member.rs   (planned)
├── routes/
│   ├── health.rs
│   ├── seasons.rs
│   ├── episodes.rs
│   ├── characters.rs
│   └── cast.rs          (planned)
└── db/
    ├── mod.rs           # Db (local or synced), connections, query helpers
    └── migrations.rs    # embedded migration list and runner

migrations/              # numbered .sql files: schema, then seed data
tests/
└── api.rs
```

The initial implementation should avoid unnecessary enterprise-style layers. Handlers query the database directly through the `Db` handle in `AppState`, opening one connection per request. Additional service or repository abstractions should only be introduced when they remove real complexity.

## Data sourcing and attribution

- Use sources such as [Wikipedia's series page](https://en.wikipedia.org/wiki/The_Last_of_Us_(TV_series)) to verify individual facts, not as an automatically imported source of truth.
- Write original summaries instead of copying episode descriptions or biographies.
- Do not scrape or reproduce IMDb content.
- Track `source_url`, `source_name`, and `last_verified_at` where practical.
- Publish an attribution and sources page.
- Review the license of every reused text or media asset. Wikipedia text is offered under Creative Commons Attribution-ShareAlike and requires attribution when copied or adapted.

## Image policy

Images found through web search are not automatically safe to redistribute. The API should only use:

1. Wikimedia Commons images with compatible licenses.
2. Public-domain images.
3. Creator-contributed images with explicit permission.
4. Licensed-provider URLs whose terms permit API redistribution.

Store attribution alongside every image:

- `url`
- `source_page_url`
- `creator`
- `license_name`
- `license_url`
- `attribution_text`

The MVP can return `null` for image fields until suitable licensed material is available. It should not bundle HBO promotional stills, posters, logos, character artwork, or screenshots without permission.

Being non-commercial doesn't make those uses legal. Wikipedia's posters and title cards are non-free files used under Wikipedia's own per-article fair-use rationale, which doesn't extend to this API. TMDB's licence is non-transferable and can't be sublicensed, so the API must not pass TMDB images through to its users.

**Implementation (migration 0003).** Images live in an `images` table with `url`, `width`, `height`, `alt_text`, `source_page_url`, `creator`, `license_name`, `license_url`, `attribution_text` and `last_verified_at`. Resources point to them through a nullable `image_id`. The API returns an `image` object carrying its attribution, never a bare URL. Resources also expose `external_ids` (TMDB, IMDb, Wikidata) so clients can fetch artwork from those services under their own terms. Likely first images: freely licensed Commons photos of cast members, once the cast tables exist.

## Delivery roadmap

### Phase 1 — Axum foundation ✅

- Start the Axum server.
- Add `/health` and basic JSON responses.
- Add request tracing and consistent error responses.
- Test the router without opening a network port.

Learning focus: routers, handlers, Tokio, responses, and HTTP status codes.

### Phase 2 — Static API prototype ✅

- Add a small set of seasons and episodes as fixtures.
- Implement list and detail endpoints.
- Add path and query extractors.
- Finalize response conventions before database work.

Learning focus: Serde, extractors, filtering, and Rust ownership.

Delivered: both seasons and Season 1's nine episodes as fixtures, `?season=` filtering, pagination (`page` ≥ 1, `limit` 1–100, default 20), and structured `400`/`404` errors for invalid path, query, and pagination input. Pagination landed here rather than in Phase 3 so the response shape is settled before database work. The fixture values were later verified for the Phase 3 seed (episode 9 gained Neil Druckmann as co-writer), and the fixtures were removed.

### Phase 3 — Turso persistence ✅

- Add Turso and database migrations.
- Create normalized tables and foreign keys.
- Seed a small, verified dataset.
- Replace fixtures with database queries.
- Add pagination and filtering.

Learning focus: asynchronous SQL, application state, joins, and migrations.

Delivered: the `turso` crate in place of SQLx, embedded migrations with a `migrate` binary, normalized `seasons`, `episodes`, and `episode_writers` tables with foreign keys and provenance columns, and a seed of seasons 1–2 (16 episodes) verified against Wikipedia on 2026-10-15. Filtering and pagination run in SQL. Production reads from a Turso Cloud replica, and tests use a fresh in-memory database each.

### Phase 4 — Developer experience ✅

- Publish an OpenAPI specification and interactive documentation. ✅ OpenAPI 3.1 generated with `utoipa` at `/api/v1/openapi.json`; Scalar docs at `/api/v1/docs`.
- Add `curl`, JavaScript, Python, and Rust examples. ✅ [`docs/EXAMPLES.md`](EXAMPLES.md), each run against the live server.
- Enable appropriately scoped CORS for browser exercises. ✅ Any origin may `GET`/`HEAD`; no credentials; preflights cached for a day.
- Document rate limits, pagination, errors, and spoiler behavior. ✅ [`docs/USING_THE_API.md`](USING_THE_API.md); sources and attribution in [`docs/SOURCES.md`](SOURCES.md).

### Phase 5 — Content expansion (in progress)

- Complete character and cast relationships. Characters ✅ (34 characters, billing per season, episode links); cast next.
- Expand the verified episode dataset.
- Add licensed images with attribution.
- Add search, filming locations, creators, and production credits.
- Expose provenance and update timestamps.

### Phase 6 — Deployment

- Build a container image.
- Automate migrations during deployment.
- Add structured logging, request IDs, and health checks.
- Add caching and rate limiting if public traffic requires them.

## Testing strategy

Tests should focus on observable API behavior:

- The health endpoint returns `200`.
- Collections use the documented JSON shape.
- Known IDs and slugs return the expected records.
- Unknown resources return structured `404` errors.
- Invalid pagination returns `400`.
- Filters return only matching records.
- Database failures produce safe `500` responses.
- CORS behavior matches the public documentation.

Axum routers implement Tower services, so most integration tests can execute requests directly against the router without binding a TCP port.

## First release definition of done

- `/health` endpoint
- Versioned `/api/v1` routes
- Season and episode list/detail endpoints
- Small, verified Turso seed dataset
- Filtering and pagination
- Consistent structured errors
- CORS configuration
- OpenAPI documentation
- Automated tests
- Data attribution and unofficial-project disclaimers

Characters and cast can be delivered immediately afterward, once routing, errors, persistence, and documentation conventions are stable.
