# The Last of Us API

An unofficial, educational REST API for HBO's *The Last of Us* TV series, built with Rust, [Axum](https://github.com/tokio-rs/axum) and [Turso](https://docs.turso.tech).

It serves seasons and episodes as predictable JSON, with pagination, filtering and structured errors. It's meant for students and developers who want a realistic API to practise against. It's also a project for learning Rust and Axum.

> This project is not affiliated with or endorsed by HBO, Max, Sony, Naughty Dog, or the series' creators. Facts are checked against public sources, and all summaries are original writing.

## What's in it

- **Seasons 1 and 2** and all **16 episodes**: titles, release dates, directors, writers in credited order, and runtimes.
- **34 characters**, with their billing in each season (main, recurring or guest) and the episodes they're credited in.
- **Sourced data.** Every fact was checked against Wikipedia, and each row records its source and the date it was checked.
- **Light spoilers.** Episode summaries are short and light on plot, but they may contain mild spoilers for the episode they describe.

Cast members (the actors) and search are next; see [`PLAN.md`](PLAN.md).

## Quick start

You need [Rust](https://www.rust-lang.org/tools/install) 1.90 or newer (2024 edition). CI checks 1.90 and the latest stable.

```bash
git clone https://github.com/papadavis47/lastofus-axum-api.git
cd lastofus-axum-api
cargo run
```

The server listens on `http://localhost:3000`. On first start it creates a local database file, `lastofus.db`, and loads the data, so there's nothing else to set up. The first build compiles the Turso database engine and takes a few minutes.

```bash
curl http://localhost:3000/api/v1/episodes/1
```

```json
{
  "data": {
    "id": 1,
    "season_id": 1,
    "number_in_season": 1,
    "overall_number": 1,
    "title": "When You're Lost in the Darkness",
    "slug": "when-youre-lost-in-the-darkness",
    "summary": "The outbreak begins, and two decades later Joel is handed an unusual cargo.",
    "runtime_minutes": 81,
    "release_date": "2023-01-15",
    "director": "Craig Mazin",
    "writers": ["Craig Mazin", "Neil Druckmann"],
    "viewer_count_millions": null,
    "image": null,
    "external_ids": { "tmdb": null, "imdb": "tt11957006", "wikidata": "Q116169494" }
  }
}
```

## Documentation

With the server running:

- **Interactive docs:** <http://localhost:3000/api/v1/docs>, where you can browse every endpoint and field and send requests.
- **OpenAPI 3.1 spec:** <http://localhost:3000/api/v1/openapi.json>, for Postman, Insomnia or a client generator such as `openapi-typescript`.

And in this repository:

- [`docs/USING_THE_API.md`](docs/USING_THE_API.md): response shapes, pagination, errors, spoilers and rate limits.
- [`docs/EXAMPLES.md`](docs/EXAMPLES.md): complete clients in `curl`, JavaScript, Python and Rust.
- [`docs/SOURCES.md`](docs/SOURCES.md): where the data comes from and how to credit it.

## Endpoints

| Method & path | Description |
|---|---|
| `GET /health` | Health check |
| `GET /api/v1/seasons` | List seasons |
| `GET /api/v1/seasons/{season_number}` | One season, e.g. `/seasons/1` |
| `GET /api/v1/seasons/{season_number}/episodes` | A season's episodes |
| `GET /api/v1/episodes` | List episodes; filter with `?season=2` |
| `GET /api/v1/episodes/{id}` | One episode by id (1–16, in broadcast order) |
| `GET /api/v1/episodes/{id}/characters` | The characters credited in an episode |
| `GET /api/v1/characters` | List characters; filter with `?season=2` and `?role=main` (`main`, `recurring` or `guest`) |
| `GET /api/v1/characters/{slug}` | One character, e.g. `/characters/joel-miller` |
| `GET /api/v1/characters/{slug}/episodes` | The episodes a character is credited in; filter with `?season=` |
| `GET /api/v1/openapi.json` | OpenAPI spec |
| `GET /api/v1/docs` | Interactive docs (HTML) |

All public routes are versioned under `/api/v1`.

CORS is open: any website can call the API from the browser with `fetch`. Responses (errors included) send `Access-Control-Allow-Origin: *`, and only `GET` and `HEAD` are allowed. The API doesn't use cookies or credentials, so don't send them.

### Pagination

Every list endpoint is paginated with `?page=` (default `1`) and `?limit=` (default `20`, maximum `100`):

```bash
curl 'http://localhost:3000/api/v1/episodes?season=2&page=1&limit=2'
```

```json
{
  "data": [ { "id": 10, "title": "Future Days", "...": "..." }, { "id": 11, "title": "Through the Valley", "...": "..." } ],
  "meta": { "page": 1, "limit": 2, "total": 7, "total_pages": 4 }
}
```

A page past the end returns an empty `data` array.

### Errors

Errors always use the same JSON shape, with an HTTP status, a machine-readable `code`, and a readable `message`:

```json
{
  "error": {
    "status": 404,
    "code": "episode_not_found",
    "message": "Episode 99 was not found."
  }
}
```

| Status | Codes |
|---|---|
| `400` | `invalid_pagination`, `invalid_path_parameter`, `invalid_query_parameter` |
| `404` | `season_not_found`, `episode_not_found`, `character_not_found`, `route_not_found` |
| `500` | `internal_error` (details are logged, never returned) |

### Conventions

- Dates are ISO 8601 (`2023-01-15`).
- Unknown values are `null`, never empty strings. For example, `viewer_count_millions` and `image` are `null` until reliable figures and properly licensed images are available.
- Seasons and episodes have an `image` field. When it's set, it's an object with `url`, `width`, `height`, `alt` and an `attribution` object (`text`, `creator`, `license`, `license_url`, `source_url`). If you display the image, show its `attribution.text`; the image's licence requires it.
- `external_ids` holds the matching `tmdb`, `imdb` and `wikidata` ids, so you can fetch artwork from those services yourself, under their terms. This API only serves images it is allowed to redistribute. Episodes have IMDb and Wikidata ids, and seasons have Wikidata ids. TMDB ids aren't filled in yet; use TMDB series id `100088` with the season and episode numbers (`/tv/100088/season/1/episode/1`).

## Configuration

Everything is configured with environment variables:

| Variable | Default | Purpose |
|---|---|---|
| `PORT` | `3000` | Port to listen on (binds `0.0.0.0`) |
| `DATABASE_PATH` | `lastofus.db` | Local database file |
| `TURSO_DATABASE_URL` | _unset_ | Turso Cloud database URL; turns on synced mode |
| `TURSO_AUTH_TOKEN` | _unset_ | Turso Cloud auth token |
| `RUST_LOG` | `info` | Log level, e.g. `debug` |

## Database

The API uses [Turso Database](https://docs.turso.tech/tursodb/quickstart), an SQLite-compatible engine written in Rust, through the [`turso`](https://crates.io/crates/turso) crate. It can run in two modes:

- **Local (default):** an embedded database file. The server applies any pending migrations when it starts. Tests use a fresh in-memory database each.
- **Synced:** when `TURSO_DATABASE_URL` is set, the server keeps a local copy of a Turso Cloud database. It downloads the latest data when it starts and answers every request from that copy. The server never writes to it.

The schema and data live in numbered SQL files in [`migrations/`](migrations/). To apply them to Turso Cloud:

```bash
turso db create lastofus --tursodb          # use the Turso engine, not libSQL
export TURSO_DATABASE_URL="$(turso db show lastofus --url)"
export TURSO_AUTH_TOKEN="$(turso db tokens create lastofus)"

cargo run --bin migrate                     # apply migrations and push them to the cloud
cargo run                                   # serve from a synced local copy
```

The server picks up new data when it restarts. You can try synced mode without an account by running a local sync server; see [`AGENTS.md`](AGENTS.md#database-turso).

## Development

```bash
cargo test                                  # integration tests (no network or ports needed)
cargo fmt
cargo clippy --all-targets -- -D warnings
```

All three must pass before each commit.

```text
src/
├── main.rs          server: config, database, migrations, graceful shutdown
├── bin/migrate.rs   `cargo run --bin migrate`
├── app.rs           router
├── config.rs        environment variables
├── db/              database handle, query helpers, migration runner
├── routes/          handlers for health, seasons and episodes
├── models/          Season and Episode
├── openapi/         OpenAPI document info and the docs page
├── response.rs      response wrappers and pagination
├── error.rs         structured errors
└── extract.rs       path/query extractors with structured 400s
migrations/          schema and seed data
tests/api.rs         integration tests
```

More docs:

- [`docs/USING_THE_API.md`](docs/USING_THE_API.md), [`docs/EXAMPLES.md`](docs/EXAMPLES.md) and [`docs/SOURCES.md`](docs/SOURCES.md): for API users.
- [`docs/PROJECT_PLAN.md`](docs/PROJECT_PLAN.md): the vision, data model, API conventions and full roadmap.
- [`AGENTS.md`](AGENTS.md): detailed notes on the current state and code conventions, written for coding agents and useful for anyone working on the code.

## Roadmap

| Phase | Status |
|---|---|
| 1. Axum foundation | ✅ Done |
| 2. Static API prototype | ✅ Done |
| 3. Turso persistence | ✅ Done |
| 4. Developer experience: CORS, OpenAPI docs, client examples | ✅ Done |
| 5. Content expansion: characters, cast, search, licensed images | Planned |
| 6. Deployment | Planned |

## Data sources

See [`docs/SOURCES.md`](docs/SOURCES.md) for details. Facts are checked one at a time against public sources, mainly the Wikipedia articles for [season 1](https://en.wikipedia.org/wiki/The_Last_of_Us_season_1) and [season 2](https://en.wikipedia.org/wiki/The_Last_of_Us_season_2), and each episode's own article for its runtime. Nothing is scraped automatically, and no IMDb content is used. The IMDb and Wikidata ids come from Wikidata, which is CC0. Summaries are original. No copyrighted promotional images are included.

## License

The code is released under the [MIT License](LICENSE). The license covers this project's code and original writing only. *The Last of Us* and related names belong to their respective owners.
