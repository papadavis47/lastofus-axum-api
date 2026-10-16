-- Characters, the seasons they're in, and the episodes they're credited in.
--
-- character_seasons.role is the character's billing for that season: main,
-- recurring or guest. episode_characters links a character to every episode
-- they're credited in or appear in uncredited.

CREATE TABLE characters (
    id               INTEGER PRIMARY KEY,
    slug             TEXT    NOT NULL UNIQUE,
    name             TEXT    NOT NULL,
    description      TEXT    NOT NULL,
    image_id         INTEGER REFERENCES images (id),
    source_name      TEXT,
    source_url       TEXT,
    last_verified_at TEXT
);

CREATE TABLE character_seasons (
    character_id INTEGER NOT NULL REFERENCES characters (id),
    season_id    INTEGER NOT NULL REFERENCES seasons (id),
    role         TEXT    NOT NULL CHECK (role IN ('main', 'recurring', 'guest')),
    PRIMARY KEY (character_id, season_id)
);

CREATE TABLE episode_characters (
    episode_id   INTEGER NOT NULL REFERENCES episodes (id),
    character_id INTEGER NOT NULL REFERENCES characters (id),
    PRIMARY KEY (episode_id, character_id)
);

CREATE INDEX episode_characters_character_id ON episode_characters (character_id);
