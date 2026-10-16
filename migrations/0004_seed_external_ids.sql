-- External ids for seasons 1 and 2 and their 16 episodes.
--
-- Wikidata and IMDb ids come from Wikidata (CC0), checked on 2026-10-16: each
-- episode item is "part of the series" The Last of Us (Q87131973) with its
-- season and number recorded, and its IMDb id (P345). The IMDb ids were
-- cross-checked against IMDb's title lookup; every title and year matches.
--
-- Notes:
-- * IMDb has no ids for seasons, so seasons.imdb_id stays NULL.
-- * tmdb_id stays NULL. TMDB's website shows only internal hex ids, and its
--   integer API ids can't be verified without an API key. Clients can still
--   address TMDB by series id 100088 plus season and episode numbers.
-- * Data changes ship as new migrations; never edit this file once applied.

UPDATE seasons SET wikidata_id = 'Q107549810' WHERE id = 1;
UPDATE seasons SET wikidata_id = 'Q116482194' WHERE id = 2;

UPDATE episodes SET wikidata_id = 'Q116169494', imdb_id = 'tt11957006' WHERE id = 1;
UPDATE episodes SET wikidata_id = 'Q116169501', imdb_id = 'tt14500884' WHERE id = 2;
UPDATE episodes SET wikidata_id = 'Q116178387', imdb_id = 'tt14500888' WHERE id = 3;
UPDATE episodes SET wikidata_id = 'Q116178388', imdb_id = 'tt14500890' WHERE id = 4;
UPDATE episodes SET wikidata_id = 'Q116178389', imdb_id = 'tt14659486' WHERE id = 5;
UPDATE episodes SET wikidata_id = 'Q116178390', imdb_id = 'tt14659496' WHERE id = 6;
UPDATE episodes SET wikidata_id = 'Q116178391', imdb_id = 'tt15747172' WHERE id = 7;
UPDATE episodes SET wikidata_id = 'Q116178392', imdb_id = 'tt15747176' WHERE id = 8;
UPDATE episodes SET wikidata_id = 'Q116178393', imdb_id = 'tt15747178' WHERE id = 9;
UPDATE episodes SET wikidata_id = 'Q133867384', imdb_id = 'tt26469967' WHERE id = 10;
UPDATE episodes SET wikidata_id = 'Q133867385', imdb_id = 'tt31804612' WHERE id = 11;
UPDATE episodes SET wikidata_id = 'Q133867386', imdb_id = 'tt32550882' WHERE id = 12;
UPDATE episodes SET wikidata_id = 'Q133867387', imdb_id = 'tt32550883' WHERE id = 13;
UPDATE episodes SET wikidata_id = 'Q133867388', imdb_id = 'tt32550885' WHERE id = 14;
UPDATE episodes SET wikidata_id = 'Q133867389', imdb_id = 'tt32550886' WHERE id = 15;
UPDATE episodes SET wikidata_id = 'Q133867390', imdb_id = 'tt32550887' WHERE id = 16;
