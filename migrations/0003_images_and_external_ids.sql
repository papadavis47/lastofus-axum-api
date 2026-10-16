-- Licensed images with attribution, and external ids for other databases.
--
-- Only freely licensed, public-domain, or explicitly permitted images belong
-- here (see "Image policy" in docs/PROJECT_PLAN.md). Every row carries the
-- attribution its licence requires, and the API returns it with the image.
--
-- External ids (TMDB, IMDb, Wikidata) let API users fetch artwork from those
-- services under their own terms, without this API redistributing it.
-- They start NULL; fill them in with sourced data migrations.

CREATE TABLE images (
    id               INTEGER PRIMARY KEY,
    url              TEXT    NOT NULL UNIQUE,
    width            INTEGER,
    height           INTEGER,
    alt_text         TEXT    NOT NULL,
    -- The page the licence was checked on, e.g. a Wikimedia Commons file page.
    source_page_url  TEXT    NOT NULL,
    creator          TEXT,
    license_name     TEXT    NOT NULL,
    license_url      TEXT,
    -- Ready-to-display credit line, e.g. "Photo by Jane Doe, CC BY-SA 4.0".
    attribution_text TEXT    NOT NULL,
    last_verified_at TEXT    NOT NULL
);

ALTER TABLE seasons ADD COLUMN image_id INTEGER REFERENCES images (id);
ALTER TABLE seasons ADD COLUMN tmdb_id INTEGER;
ALTER TABLE seasons ADD COLUMN imdb_id TEXT;
ALTER TABLE seasons ADD COLUMN wikidata_id TEXT;

-- Replaced by image_id: a bare URL can't carry attribution.
ALTER TABLE episodes DROP COLUMN image_url;
ALTER TABLE episodes ADD COLUMN image_id INTEGER REFERENCES images (id);
ALTER TABLE episodes ADD COLUMN tmdb_id INTEGER;
ALTER TABLE episodes ADD COLUMN imdb_id TEXT;
ALTER TABLE episodes ADD COLUMN wikidata_id TEXT;
