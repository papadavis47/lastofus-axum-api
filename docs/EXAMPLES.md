# Client examples

Small, complete programs that call the API with `curl`, JavaScript, Python and Rust. Each one fetches an episode, lists a season's episodes page by page, and handles an error. They assume the server is running locally (`cargo run`) at `http://localhost:3000`; change `BASE_URL` to point elsewhere.

For the rules these examples follow (pagination, error codes, `null` values), see [`USING_THE_API.md`](USING_THE_API.md). The full schema is in the OpenAPI spec at [`/api/v1/openapi.json`](http://localhost:3000/api/v1/openapi.json), which you can also feed to a client generator such as [`openapi-typescript`](https://openapi-ts.dev) or [`openapi-generator`](https://openapi-generator.tech).

## curl

```bash
# One episode. -s hides the progress bar; jq pretty-prints (optional).
curl -s http://localhost:3000/api/v1/episodes/1 | jq

# Season 2's episodes, three per page, second page.
curl -s 'http://localhost:3000/api/v1/seasons/2/episodes?limit=3&page=2' | jq '.data[].title, .meta'

# Filter the episode list by season.
curl -s 'http://localhost:3000/api/v1/episodes?season=1&limit=100' | jq '.meta.total'

# A character, and the main characters of season 2.
curl -s http://localhost:3000/api/v1/characters/joel-miller | jq '.data.name, .data.seasons'
curl -s 'http://localhost:3000/api/v1/characters?season=2&role=main' | jq -r '.data[].name'

# Who's credited in season 1 episode 3, and which episodes Tess is in.
curl -s http://localhost:3000/api/v1/episodes/3/characters | jq -r '.data[].name'
curl -s http://localhost:3000/api/v1/characters/tess/episodes | jq -r '.data[].title'

# Errors are JSON too. -w prints the status code after the body.
curl -s -w '\n%{http_code}\n' http://localhost:3000/api/v1/episodes/99

# Just the titles and air dates of every episode, as tab-separated text.
curl -s 'http://localhost:3000/api/v1/episodes?limit=100' \
  | jq -r '.data[] | [.overall_number, .release_date, .title] | @tsv'
```

## JavaScript (`fetch`)

Runs in a browser (CORS is open) or in Node.js 18+ (save as `example.mjs` and run `node example.mjs`).

```js
const BASE_URL = "http://localhost:3000/api/v1";

/** GETs a path and returns the parsed body, or throws the API's error. */
async function api(path) {
  const response = await fetch(`${BASE_URL}${path}`);
  const body = await response.json();
  if (!response.ok) {
    // Every error has the same shape: { error: { status, code, message } }.
    const error = new Error(body.error.message);
    error.code = body.error.code;
    error.status = body.error.status;
    throw error;
  }
  return body;
}

/** Follows `meta.total_pages` to collect every item in a collection. */
async function fetchAll(path, limit = 100) {
  const items = [];
  for (let page = 1; ; page++) {
    const separator = path.includes("?") ? "&" : "?";
    const { data, meta } = await api(`${path}${separator}page=${page}&limit=${limit}`);
    items.push(...data);
    if (page >= meta.total_pages) return items;
  }
}

const { data: episode } = await api("/episodes/1");
console.log(`${episode.title} (${episode.release_date}), written by ${episode.writers.join(" & ")}`);

const seasonTwo = await fetchAll("/seasons/2/episodes", 3);
for (const ep of seasonTwo) {
  // Nullable fields are always present; check for null, not undefined.
  const runtime = ep.runtime_minutes === null ? "unknown" : `${ep.runtime_minutes} min`;
  console.log(`S2E${ep.number_in_season}: ${ep.title} (${runtime})`);
}

try {
  await api("/episodes/99");
} catch (error) {
  if (error.code === "episode_not_found") {
    console.log(`Not found: ${error.message}`);
  } else {
    throw error;
  }
}
```

In TypeScript, generate types from the spec instead of writing them by hand:

```bash
npx openapi-typescript http://localhost:3000/api/v1/openapi.json -o lastofus.d.ts
```

## Python

Uses only the standard library (Python 3.9+). Save as `example.py` and run `python3 example.py`. With [`requests`](https://requests.readthedocs.io) or [`httpx`](https://www.python-httpx.org), `api()` shrinks to a couple of lines, but the logic is the same.

```python
import json
import urllib.error
import urllib.request

BASE_URL = "http://localhost:3000/api/v1"


class ApiError(Exception):
    def __init__(self, status, code, message):
        super().__init__(message)
        self.status = status
        self.code = code


def api(path):
    """GETs a path and returns the parsed body, or raises the API's error."""
    try:
        with urllib.request.urlopen(BASE_URL + path) as response:
            return json.load(response)
    except urllib.error.HTTPError as response:
        # Every error has the same shape: {"error": {status, code, message}}.
        error = json.load(response)["error"]
        raise ApiError(error["status"], error["code"], error["message"]) from None


def fetch_all(path, limit=100):
    """Follows meta.total_pages to collect every item in a collection."""
    items, page = [], 1
    separator = "&" if "?" in path else "?"
    while True:
        body = api(f"{path}{separator}page={page}&limit={limit}")
        items.extend(body["data"])
        if page >= body["meta"]["total_pages"]:
            return items
        page += 1


episode = api("/episodes/1")["data"]
print(f"{episode['title']} ({episode['release_date']}), written by {' & '.join(episode['writers'])}")

for ep in fetch_all("/seasons/2/episodes", limit=3):
    # Unknown values are None (JSON null), never "".
    runtime = "unknown" if ep["runtime_minutes"] is None else f"{ep['runtime_minutes']} min"
    print(f"S2E{ep['number_in_season']}: {ep['title']} ({runtime})")

try:
    api("/episodes/99")
except ApiError as error:
    if error.code != "episode_not_found":
        raise
    print(f"Not found: {error}")
```

## Rust

Uses [`reqwest`](https://docs.rs/reqwest), [`serde`](https://serde.rs) and [`tokio`](https://tokio.rs). Create a project with `cargo new lastofus-client`, then:

```bash
cargo add reqwest --features json
cargo add serde --features derive
cargo add tokio --features macros,rt-multi-thread
```

Put this in `src/main.rs` and `cargo run`. Structs only need the fields you use; serde ignores the rest, so new fields added to the API won't break your client.

```rust
use serde::{Deserialize, de::DeserializeOwned};

const BASE_URL: &str = "http://localhost:3000/api/v1";

#[derive(Deserialize)]
struct Single<T> {
    data: T,
}

#[derive(Deserialize)]
struct Page<T> {
    data: Vec<T>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
    total_pages: u64,
}

#[derive(Deserialize)]
struct Episode {
    title: String,
    number_in_season: u32,
    release_date: String,
    writers: Vec<String>,
    // Unknown values are null, so nullable fields are Options.
    runtime_minutes: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct ApiError {
    status: u16,
    code: String,
    message: String,
}

#[derive(Deserialize)]
struct ErrorBody {
    error: ApiError,
}

// The fields are only read by `Debug`, when `main` returns an error.
#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    Api(ApiError),
    Http(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err)
    }
}

/// GETs a path and decodes the body as `T`, or as the API's error.
async fn api<T: DeserializeOwned>(client: &reqwest::Client, path: &str) -> Result<T, Error> {
    let response = client.get(format!("{BASE_URL}{path}")).send().await?;
    if response.status().is_success() {
        Ok(response.json().await?)
    } else {
        // Every error has the same shape: {"error": {status, code, message}}.
        Err(Error::Api(response.json::<ErrorBody>().await?.error))
    }
}

/// Follows `meta.total_pages` to collect every item in a collection.
async fn fetch_all<T: DeserializeOwned>(
    client: &reqwest::Client,
    path: &str,
    limit: u32,
) -> Result<Vec<T>, Error> {
    let mut items = Vec::new();
    let mut page: u64 = 1;
    loop {
        let body: Page<T> = api(client, &format!("{path}?page={page}&limit={limit}")).await?;
        items.extend(body.data);
        if page >= body.meta.total_pages {
            return Ok(items);
        }
        page += 1;
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let Single { data: episode } = api::<Single<Episode>>(&client, "/episodes/1").await?;
    println!(
        "{} ({}), written by {}",
        episode.title,
        episode.release_date,
        episode.writers.join(" & ")
    );

    for ep in fetch_all::<Episode>(&client, "/seasons/2/episodes", 3).await? {
        let runtime = ep
            .runtime_minutes
            .map_or("unknown".to_string(), |minutes| format!("{minutes} min"));
        println!("S2E{}: {} ({runtime})", ep.number_in_season, ep.title);
    }

    match api::<Single<Episode>>(&client, "/episodes/99").await {
        Err(Error::Api(error)) if error.code == "episode_not_found" => {
            println!("Not found ({}): {}", error.status, error.message);
        }
        other => {
            other?;
        }
    }

    Ok(())
}
```
