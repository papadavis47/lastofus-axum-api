# Sources and attribution

Where the API's data comes from, and what you need to credit if you reuse it.

> This is an unofficial, educational project. It is not affiliated with or endorsed by HBO, Max, Sony, Naughty Dog, or the series' creators. *The Last of Us* and related names belong to their respective owners.

## Facts

Season and episode facts (titles, release dates, directors, writers and runtimes) were checked one at a time against these Wikipedia articles on 2026-10-15:

| Data | Source |
|---|---|
| Season 1 and its 9 episodes | [The Last of Us season 1](https://en.wikipedia.org/wiki/The_Last_of_Us_season_1) |
| Season 2 and its 7 episodes | [The Last of Us season 2](https://en.wikipedia.org/wiki/The_Last_of_Us_season_2) |
| Episode runtimes | Each episode's own Wikipedia article |

Nothing is scraped automatically. Every season and episode row in the database records its `source_name`, `source_url` and `last_verified_at`. The API doesn't return these fields yet; exposing them is planned for Phase 5.

Notes on specific values:

- **Season 1, episode 5** has a `release_date` of `2023-02-10`, when it was released online on HBO Max. Its TV broadcast moved to 2023-02-12 because of the Super Bowl, which is the date Wikipedia's episode table shows.
- **`viewer_count_millions`** is `null` everywhere. The only per-episode figures are same-night US TV ratings, which badly understate total viewing.

## External ids

The `imdb` and `wikidata` ids in `external_ids` come from [Wikidata](https://www.wikidata.org/wiki/Q87131973), whose data is released under [CC0](https://creativecommons.org/publicdomain/zero/1.0/) (no attribution required). They were taken on 2026-10-16, and the IMDb ids were cross-checked against IMDb's title lookup. No other IMDb content is used. IMDb has no ids for seasons, so seasons' `imdb` is `null`.

`tmdb` is `null` everywhere: TMDB's integer ids can't be verified without an API key. TMDB's series id for the show is `100088`, so you can request `/tv/100088/season/{season}/episode/{episode}` from TMDB under [its terms](https://www.themoviedb.org/api-terms-of-use).

## Summaries

Every season and episode summary is original writing for this project, released with the code under the [MIT License](../LICENSE). They're short, light on plot, and may contain mild spoilers for the episode or season they describe.

## Images

There are no images yet, so every `image` is `null`. Only images whose licence allows redistribution (for example from Wikimedia Commons, or in the public domain) will be added, never HBO stills, posters or screenshots. Each image will include an `attribution` object with a ready-to-display credit line (`text`), the `creator`, the `license` and its `license_url`, and the `source_url` where the licence was checked. **If you display an image, display its `attribution.text` with it.** That's what the image's licence requires, not just a courtesy.

## Reusing the data

Facts generally aren't copyrightable, and the summaries are MIT-licensed, so you can use the API's responses in your own projects. A link back to this repository is appreciated but not required. Wikipedia's article text is not copied into the API; if you use Wikipedia directly, follow its [CC BY-SA licence](https://en.wikipedia.org/wiki/Wikipedia:Text_of_the_Creative_Commons_Attribution-ShareAlike_4.0_International_License).

## Corrections

Found a wrong fact? Open an issue with a link to a reliable source. Fixes ship as new database migrations, with the source recorded on the row.
