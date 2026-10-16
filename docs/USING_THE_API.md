# Using the API

How the API behaves: what it promises, what it doesn't, and how to handle each response. For every endpoint and field, see the interactive docs at [`/api/v1/docs`](http://localhost:3000/api/v1/docs) or the raw OpenAPI 3.1 spec at [`/api/v1/openapi.json`](http://localhost:3000/api/v1/openapi.json). For code, see [`EXAMPLES.md`](EXAMPLES.md).

> **Unofficial project.** This API is an educational project. It is not affiliated with or endorsed by HBO, Max, Sony, Naughty Dog, or the series' creators. *The Last of Us* and related names belong to their respective owners.

## Basics

- **Read-only.** Every endpoint is a `GET` (`HEAD` also works). There is no authentication and nothing to sign up for.
- **JSON everywhere.** Every response, errors and unknown routes included, is `application/json`. The only exception is the HTML docs page at `/api/v1/docs`.
- **Versioned.** Public routes live under `/api/v1`. Fields may be added to `v1` responses, so ignore fields you don't know. Fields won't be removed or renamed, and their types won't change, without a new version.
- **CORS is open.** Any website can call the API with `fetch`. Responses send `Access-Control-Allow-Origin: *`. The API doesn't use cookies, so don't send credentials.

## Response shapes

A single resource is wrapped in `data`:

```json
{ "data": { "id": 1, "title": "When You're Lost in the Darkness", "...": "..." } }
```

A collection is a list in `data` plus pagination details in `meta`:

```json
{
  "data": [ { "id": 1, "...": "..." }, { "id": 2, "...": "..." } ],
  "meta": { "page": 1, "limit": 2, "total": 16, "total_pages": 8 }
}
```

### Values

- Dates are ISO 8601 strings: `"2023-01-15"`.
- Unknown values are `null`, never `""`. Nullable fields are still always present, so check for `null` rather than for a missing key.
- Episode `id`s follow broadcast order across the whole series, so `id` equals `overall_number` (1–16). Seasons are looked up by their `number`.
- `writers` lists names in credited order.
- Characters are looked up by `slug`, e.g. `joel-miller`. Names follow the TV series, so Ellie is `ellie`, not the games' "Ellie Williams".
- A character's `seasons` lists every season they appear in, each with a `role` (`main`, `recurring` or `guest`: their billing that season, which can change) and an `episode_count`.
- `viewer_count_millions` is `null` for now. The only per-episode figures are same-night US TV ratings, which badly understate how many people watched.
- `image` is `null` until properly licensed images are added. When it is set, it includes an `attribution` object; if you show the image, show `attribution.text` with it.
- `external_ids` holds the matching `imdb`, `wikidata` and `tmdb` ids, so you can look things up elsewhere under those services' terms. `tmdb` isn't filled in yet: use TMDB series id `100088` with the season and episode numbers.

## Pagination

Every collection is paginated with two query parameters:

| Parameter | Default | Allowed |
|---|---|---|
| `page` | `1` | `1` or more |
| `limit` | `20` | `1` to `100` |

`meta.total` counts every matching item, not just this page, and `meta.total_pages` is how many pages that makes. A page past the end isn't an error: it returns `"data": []` with the usual `meta`. When nothing matches, `total` and `total_pages` are both `0`.

To fetch everything, start at page 1 and stop after `meta.total_pages` (or when `data` comes back empty). With only 16 episodes, `?limit=100` gets them all at once.

Filters work alongside pagination. `GET /api/v1/episodes?season=2&limit=3` returns the first three episodes of season 2, and `meta.total` is `7`. An unknown season in the `season` filter gives an empty page, while `GET /api/v1/seasons/99/episodes` is a `404`, because there the season is the resource you asked for.

## Characters and episodes

`GET /api/v1/characters/{slug}/episodes` and `GET /api/v1/episodes/{id}/characters` link characters to the episodes they're **credited** in. That follows the on-screen credits, with two corrections: a character who is credited but doesn't appear is left out, and a character who appears without a credit is included. A credit can come from a flashback or a recap, so it doesn't always mean a big part in the episode.

`GET /api/v1/characters?role=main` matches characters with that billing in any season. Add `season`, as in `?season=1&role=main`, to match the billing in that season only.

## Errors

Every error has the same shape, whatever the status:

```json
{
  "error": {
    "status": 404,
    "code": "episode_not_found",
    "message": "Episode 99 was not found."
  }
}
```

- `status` repeats the HTTP status code.
- `code` is stable and machine-readable. Branch on it.
- `message` is a sentence for people. It may change, so don't parse it.

| Status | Code | When |
|---|---|---|
| `400` | `invalid_pagination` | `page` is `0`, or `limit` is outside 1–100 |
| `400` | `invalid_query_parameter` | A query value isn't a valid number, e.g. `?season=one` or `?page=-1` |
| `400` | `invalid_path_parameter` | A path id isn't a valid number, e.g. `/episodes/abc` |
| `404` | `season_not_found` | No season has that number |
| `404` | `episode_not_found` | No episode has that id |
| `404` | `character_not_found` | No character has that slug |
| `404` | `route_not_found` | The path doesn't exist |
| `500` | `internal_error` | Something went wrong on the server. Details are logged, never returned. Retrying later may help. |

## Spoilers

Episode and season summaries and character descriptions are original, short, and deliberately light on plot. They may still contain **mild spoilers for the episode, season or character they describe**, but they don't reveal major twists or deaths. Titles, dates and writing and directing credits are not considered spoilers. If you're building something for people who haven't seen the show, hide `summary` and `description` behind a click.

**Character appearances can be spoilers.** The list of episodes a character is credited in, and their `episode_count` per season, can hint at what happens to them, for example when a character stops appearing. Treat `/characters/{slug}/episodes`, `/episodes/{id}/characters` and `seasons` as revealing as the episodes they cover.

## Rate limits and availability

There are no rate limits, API keys or usage quotas right now, and no hosted instance yet: run the server locally (see the [README](../README.md#quick-start)). If a public instance is deployed, it may add rate limits; they will be documented here and reported with a `429` status and the usual error body. Please cache responses: the data changes only when a new migration ships.

## Data and attribution

Facts come from public sources and every row records where it came from. See [`SOURCES.md`](SOURCES.md).
