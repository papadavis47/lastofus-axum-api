-- Seasons, episodes, and episode writers.
-- source_* and last_verified_at record where each row's facts were checked.

CREATE TABLE seasons (
    id               INTEGER PRIMARY KEY,
    number           INTEGER NOT NULL UNIQUE,
    title            TEXT    NOT NULL,
    release_year     INTEGER NOT NULL,
    episode_count    INTEGER NOT NULL,
    summary          TEXT    NOT NULL,
    source_name      TEXT,
    source_url       TEXT,
    last_verified_at TEXT
);

CREATE TABLE episodes (
    id                    INTEGER PRIMARY KEY,
    season_id             INTEGER NOT NULL REFERENCES seasons (id),
    number_in_season      INTEGER NOT NULL,
    overall_number        INTEGER NOT NULL UNIQUE,
    title                 TEXT    NOT NULL,
    slug                  TEXT    NOT NULL UNIQUE,
    summary               TEXT    NOT NULL,
    runtime_minutes       INTEGER,
    release_date          TEXT    NOT NULL,
    director              TEXT    NOT NULL,
    viewer_count_millions REAL,
    image_url             TEXT,
    source_name           TEXT,
    source_url            TEXT,
    last_verified_at      TEXT,
    UNIQUE (season_id, number_in_season)
);

CREATE INDEX episodes_season_id ON episodes (season_id);

-- Writers keep their credited order via `position`.
CREATE TABLE episode_writers (
    episode_id INTEGER NOT NULL REFERENCES episodes (id),
    position   INTEGER NOT NULL,
    name       TEXT    NOT NULL,
    PRIMARY KEY (episode_id, position)
);
