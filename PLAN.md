# PLAN.md

The working plan: what's done recently, what's next, and in what order. The long-term vision and roadmap live in [`docs/PROJECT_PLAN.md`](docs/PROJECT_PLAN.md), and the current state of the code in [`AGENTS.md`](AGENTS.md). Update this file when a step lands or the order changes.

_Last updated: 2026-10-16_

## Recently done

- **CI** (`.github/workflows/ci.yml`): `cargo fmt --check`, `clippy -D warnings` and `cargo test` on stable for every push to `main` and every pull request, plus `cargo check` on the MSRV, Rust 1.90.
- **Characters** (migrations 0005–0006): 34 characters from seasons 1 and 2, each with a billing per season (`main`, `recurring` or `guest`) and links to the episodes they're credited in.
  - `GET /api/v1/characters` (`?season=`, `?role=`), `/characters/{slug}`, `/characters/{slug}/episodes` (`?season=`)
  - `GET /api/v1/episodes/{id}/characters`

## Next steps

Roughly in order. Each step ships as its own pull request with migrations, OpenAPI annotations, integration tests, and updates to `README.md`, `docs/USING_THE_API.md`, `docs/EXAMPLES.md` and `docs/SOURCES.md` where they apply.

### 1. Cast members (Phase 5)

The actors who play the characters.

- **Schema:** `cast_members` (`id`, `slug`, `name`, `biography`, `image_id`, `wikidata_id`, `imdb_id`, `source_*`) and `character_cast` (`character_id`, `cast_member_id`, `season_id`), because casting can change between seasons and a character can have more than one actor, e.g. a younger version in a flashback.
- **Data:** the "X as Y" lines in each season article's "Cast and characters" section, cross-checked against the "Portrayed by" column of the characters article. Biographies are short and original. Wikidata and IMDb ids come from Wikidata (CC0), as for episodes.
- **Endpoints:** `GET /api/v1/cast` (`?season=`), `/cast/{slug}`, `/cast/{slug}/characters`. Add `portrayed_by: [{cast_member, season}]` to `Character`, loaded in one query like `seasons`.
- **Images:** cast members are the likeliest first licensed images. The season 1 article already uses Commons photos of Pedro Pascal and Bella Ramsey, so Commons is the place to start. Each image needs its licence checked on its Commons page and an `images` row with full attribution.

### 2. More episode filters

The plan proposes `GET /api/v1/episodes?director=ali-abbasi`. Directors and writers are plain strings today, so either:

- **Option A:** filter on a slugified `director` column (cheap, but writers stay strings), or
- **Option B (preferred):** a `people` table for crew, with `episode_directors`/`episode_writers` pointing at it, so `?director=` and `?writer=` take slugs and crew can be looked up later. Do this with care: it changes the `episode_writers` table, which needs a migration that moves the existing rows.

Keep `director` and `writers` in the episode response unchanged either way.

### 3. Provenance in responses

Every season, episode and character row records `source_name`, `source_url` and `last_verified_at`. Expose them as a `source: {name, url, last_verified_at}` object on each resource, mark the fields `required` in the schema, and update `docs/SOURCES.md`.

### 4. Search

`GET /api/v1/search?q=ellie`, returning matches across episodes (title), characters (name) and, once they exist, cast members (name), grouped by type, e.g. `{"data": {"episodes": [...], "characters": [...]}}`. Start with case-insensitive `LIKE` over a handful of columns; full-text search is only worth it if the dataset grows a lot. Validate `q` (1–100 characters) and return `400 invalid_query_parameter` otherwise.

### 5. More characters and data

- One-off guests the characters article doesn't cover (Robert, Dr. Neuman, Hanrahan, Burton, Elise Park and others), using the guest lists in the season articles.
- Wikidata ids for characters, where the TV characters have their own items.
- Season 3 when it airs (expected 2027): the season, its episodes, Yara and Lev, and updated billing for returning characters, all in new migrations.

### 6. Deployment (Phase 6)

- A container image (multi-stage build; the Turso engine makes the first build slow, so cache dependencies).
- Run `cargo run --bin migrate` as a deploy step, before new servers start.
- Request ids and structured (JSON) logs.
- A periodic `pull` for synced replicas, so servers pick up new data without a restart.
- Add the public URL to the OpenAPI `servers` list, `README.md` and `docs/EXAMPLES.md`.
- Caching headers (`Cache-Control`, `ETag`) and rate limiting if public traffic needs them; document any limits in `docs/USING_THE_API.md` with a `429` error code.

### Small follow-ups

- Dependabot (or Renovate) for Cargo and GitHub Actions updates.
- An `/api/v1/characters?q=` name filter if search takes a while to arrive.

## Decisions made

- **Billing is per season.** The project plan's single `role_type`, `first_season` and `last_season` became a `seasons: [{season, role, episode_count}]` list, because billing changes between seasons (Tommy is a guest in season 1 and main in season 2). First and last seasons are the first and last entries.
- **Episode links follow credits**, with "credited but does not appear" removed and uncredited appearances added. They're documented as possible spoilers.
- **Both directions are exposed:** `/characters/{slug}/episodes` and `/episodes/{id}/characters`.
- **Names follow the TV series**, so the slug is `ellie`, not the games' `ellie-williams`.
