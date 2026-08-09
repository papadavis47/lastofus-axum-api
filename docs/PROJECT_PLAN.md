# The Last of Us REST API — Project Plan

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
- `image_url` (optional)

### Character

- `id`
- `name`
- `slug`
- `description`
- `role_type` (`main`, `recurring`, or `guest`)
- `first_season`
- `last_season`
- `image_url` (optional)

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
- SQLite for the first release
- SQLx for asynchronous database access and explicit SQL
- `tower-http` for CORS, tracing, compression, and request IDs
- `tracing` and `tracing-subscriber` for structured logs
- `thiserror` for typed application errors

SQLite keeps local development and deployment simple for a mostly static dataset. PostgreSQL can be considered later if operational needs justify it.

### Proposed source layout

```text
src/
├── main.rs
├── app.rs
├── config.rs
├── error.rs
├── state.rs
├── models/
│   ├── season.rs
│   ├── episode.rs
│   ├── character.rs
│   └── cast_member.rs
├── routes/
│   ├── health.rs
│   ├── seasons.rs
│   ├── episodes.rs
│   ├── characters.rs
│   └── cast.rs
└── db/
    └── mod.rs

migrations/
tests/
```

The initial implementation should avoid unnecessary enterprise-style layers. Handlers may query the database through a shared `SqlitePool` in `AppState`. Additional service or repository abstractions should only be introduced when they remove real complexity.

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

## Delivery roadmap

### Phase 1 — Axum foundation

- Start the Axum server.
- Add `/health` and basic JSON responses.
- Add request tracing and consistent error responses.
- Test the router without opening a network port.

Learning focus: routers, handlers, Tokio, responses, and HTTP status codes.

### Phase 2 — Static API prototype

- Add a small set of seasons and episodes as fixtures.
- Implement list and detail endpoints.
- Add path and query extractors.
- Finalize response conventions before database work.

Learning focus: Serde, extractors, filtering, and Rust ownership.

### Phase 3 — SQLite persistence

- Add SQLx and database migrations.
- Create normalized tables and foreign keys.
- Seed a small, verified dataset.
- Replace fixtures with database queries.
- Add pagination and filtering.

Learning focus: asynchronous SQL, application state, joins, and migrations.

### Phase 4 — Developer experience

- Publish an OpenAPI specification and interactive documentation.
- Add `curl`, JavaScript, Python, and Rust examples.
- Enable appropriately scoped CORS for browser exercises.
- Document rate limits, pagination, errors, and spoiler behavior.

### Phase 5 — Content expansion

- Complete character and cast relationships.
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
- Small SQLite seed dataset
- Filtering and pagination
- Consistent structured errors
- CORS configuration
- OpenAPI documentation
- Automated tests
- Data attribution and unofficial-project disclaimers

Characters and cast can be delivered immediately afterward, once routing, errors, persistence, and documentation conventions are stable.
