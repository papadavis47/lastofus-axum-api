-- Characters from seasons 1 and 2, their billing per season, and the episodes
-- they're credited in.
--
-- Checked against Wikipedia on 2026-10-16:
-- * Characters and episodes: the overview table in "Characters of The Last of
--   Us (TV series)", which marks each character's credit in every episode.
--   It covers the characters with their own section in that article; one-off
--   guests such as Robert or Hanrahan are not included yet.
-- * Billing (main, recurring or guest) per season: the "Cast and characters"
--   sections of the season 1 and season 2 articles.
-- * Names follow the season articles, e.g. "Ellie" rather than the games'
--   "Ellie Williams".
-- Descriptions are original and avoid plot twists.
--
-- Notes:
-- * episode_characters follows on-screen credits. Credits the table marks as
--   "credited but does not appear" (Tommy in season 2 episodes 4 and 5, Jesse
--   in season 2 episode 4) are left out. Joel's uncredited appearance in
--   season 2 episode 5 is included. A credit can come from a flashback or
--   recap, e.g. Henry in season 1 episode 6.
-- * Yara and Lev are in the table for season 3 only, so they are left out
--   until season 3 is added.
-- * Data changes ship as new migrations; never edit this file once applied.

INSERT INTO characters (id, slug, name, description, source_name, source_url, last_verified_at) VALUES
    (1, 'joel-miller', 'Joel Miller', 'A hardened smuggler in the Boston quarantine zone who reluctantly agrees to escort a teenage girl across the country.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (2, 'ellie', 'Ellie', 'A defiant, quick-witted teenager whose immunity to the infection could matter to everyone left alive.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (3, 'tommy-miller', 'Tommy Miller', 'Joel''s younger brother, a former Firefly who helps run the community in Jackson, Wyoming.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (4, 'dina', 'Dina', 'A free-spirited resident of Jackson with a close bond with Ellie.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (5, 'jesse', 'Jesse', 'A dependable member of the Jackson community who puts other people first.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (6, 'abby', 'Abby', 'A Washington Liberation Front soldier in Seattle and a former Firefly, driven by an old grievance.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (7, 'owen', 'Owen', 'A Washington Liberation Front soldier and former Firefly, gentler than his work demands.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (8, 'nora', 'Nora', 'A Washington Liberation Front medic and former Firefly.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (9, 'mel', 'Mel', 'A doctor with the Washington Liberation Front and a former Firefly.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (10, 'sarah', 'Sarah', 'Joel''s teenage daughter, living with him in Austin, Texas, when the outbreak begins.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (11, 'marlene', 'Marlene', 'The leader of the Fireflies, a resistance movement opposed to military rule.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (12, 'tess', 'Tess', 'Joel''s smuggling partner, respected and feared in the Boston quarantine zone.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (13, 'ratna-pertiwi', 'Ratna Pertiwi', 'A mycology professor in Jakarta who is called in to examine an alarming case early in the outbreak.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (14, 'bill', 'Bill', 'A wary survivalist who prepared for the collapse and has turned his small town into a fortress.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (15, 'frank', 'Frank', 'A traveller whose arrival changes life in Bill''s fortified town.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (16, 'henry-burrell', 'Henry Burrell', 'A man hiding from the authorities in Kansas City with his younger brother, Sam.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (17, 'sam', 'Sam', 'Henry''s artistic, deaf younger brother.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (18, 'kathleen-coghlan', 'Kathleen Coghlan', 'The leader of the revolutionary movement that has taken over Kansas City.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (19, 'perry', 'Perry', 'Kathleen''s second-in-command in Kansas City.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (20, 'maria', 'Maria', 'Tommy''s wife and a co-leader of the community in Jackson, Wyoming.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (21, 'marlon', 'Marlon', 'A hunter who lives in the Wyoming wilderness with his wife, Florence.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (22, 'florence', 'Florence', 'Marlon''s wife, who shares their remote home in Wyoming.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (23, 'riley-abel', 'Riley Abel', 'Ellie''s best friend from the military boarding school in Boston.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (24, 'david', 'David', 'A preacher who leads a struggling community through a harsh winter.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (25, 'james', 'James', 'David''s right-hand man.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (26, 'anna', 'Anna', 'Ellie''s mother.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (27, 'seth', 'Seth', 'A former police officer who runs a bar in Jackson.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (28, 'manny', 'Manny', 'A Washington Liberation Front soldier and former Firefly with an easygoing manner.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (29, 'kat', 'Kat', 'A resident of Jackson and Ellie''s ex.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (30, 'gail', 'Gail', 'A therapist in Jackson who counsels Joel.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (31, 'isaac-dixon', 'Isaac Dixon', 'The leader of the Washington Liberation Front and a former FEDRA sergeant.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (32, 'janowicz', 'Janowicz', 'A FEDRA soldier stationed in Seattle.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (33, 'javier-miller', 'Javier Miller', 'Joel and Tommy''s father, a police officer.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16'),
    (34, 'eugene-lynden', 'Eugene Lynden', 'Gail''s husband, a resident of Jackson.', 'Wikipedia', 'https://en.wikipedia.org/wiki/Characters_of_The_Last_of_Us_(TV_series)', '2026-10-16');

INSERT INTO character_seasons (character_id, season_id, role) VALUES
    (1, 1, 'main'),         -- Joel Miller
    (1, 2, 'main'),         -- Joel Miller
    (2, 1, 'main'),         -- Ellie
    (2, 2, 'main'),         -- Ellie
    (3, 1, 'guest'),        -- Tommy Miller
    (3, 2, 'main'),         -- Tommy Miller
    (4, 2, 'main'),         -- Dina
    (5, 2, 'main'),         -- Jesse
    (6, 2, 'recurring'),    -- Abby
    (7, 2, 'recurring'),    -- Owen
    (8, 2, 'recurring'),    -- Nora
    (9, 2, 'recurring'),    -- Mel
    (10, 1, 'guest'),       -- Sarah
    (11, 1, 'guest'),       -- Marlene
    (12, 1, 'recurring'),   -- Tess
    (13, 1, 'guest'),       -- Ratna Pertiwi
    (14, 1, 'guest'),       -- Bill
    (15, 1, 'guest'),       -- Frank
    (16, 1, 'guest'),       -- Henry Burrell
    (17, 1, 'guest'),       -- Sam
    (18, 1, 'guest'),       -- Kathleen Coghlan
    (19, 1, 'guest'),       -- Perry
    (20, 1, 'guest'),       -- Maria
    (20, 2, 'recurring'),   -- Maria
    (21, 1, 'guest'),       -- Marlon
    (22, 1, 'guest'),       -- Florence
    (23, 1, 'guest'),       -- Riley Abel
    (24, 1, 'guest'),       -- David
    (25, 1, 'guest'),       -- James
    (26, 1, 'guest'),       -- Anna
    (27, 2, 'recurring'),   -- Seth
    (28, 2, 'recurring'),   -- Manny
    (29, 2, 'guest'),       -- Kat
    (30, 2, 'recurring'),   -- Gail
    (31, 2, 'guest'),       -- Isaac Dixon
    (32, 2, 'guest'),       -- Janowicz
    (33, 2, 'guest'),       -- Javier Miller
    (34, 2, 'guest');       -- Eugene Lynden

INSERT INTO episode_characters (episode_id, character_id) VALUES
    -- Episode 1: Joel Miller, Ellie, Tommy Miller, Sarah, Marlene, Tess
    (1, 1), (1, 2), (1, 3), (1, 10), (1, 11), (1, 12),
    -- Episode 2: Joel Miller, Ellie, Tess, Ratna Pertiwi
    (2, 1), (2, 2), (2, 12), (2, 13),
    -- Episode 3: Joel Miller, Ellie, Tess, Bill, Frank
    (3, 1), (3, 2), (3, 12), (3, 14), (3, 15),
    -- Episode 4: Joel Miller, Ellie, Henry Burrell, Sam, Kathleen Coghlan, Perry
    (4, 1), (4, 2), (4, 16), (4, 17), (4, 18), (4, 19),
    -- Episode 5: Joel Miller, Ellie, Henry Burrell, Sam, Kathleen Coghlan, Perry
    (5, 1), (5, 2), (5, 16), (5, 17), (5, 18), (5, 19),
    -- Episode 6: Joel Miller, Ellie, Tommy Miller, Sarah, Henry Burrell, Maria, Marlon, Florence
    (6, 1), (6, 2), (6, 3), (6, 10), (6, 16), (6, 20), (6, 21), (6, 22),
    -- Episode 7: Joel Miller, Ellie, Riley Abel
    (7, 1), (7, 2), (7, 23),
    -- Episode 8: Joel Miller, Ellie, David, James
    (8, 1), (8, 2), (8, 24), (8, 25),
    -- Episode 9: Joel Miller, Ellie, Marlene, Anna
    (9, 1), (9, 2), (9, 11), (9, 26),
    -- Episode 10: Joel Miller, Ellie, Tommy Miller, Dina, Jesse, Abby, Owen, Nora, Mel, Maria, Seth, Manny, Kat, Gail
    (10, 1), (10, 2), (10, 3), (10, 4), (10, 5), (10, 6), (10, 7), (10, 8), (10, 9), (10, 20), (10, 27), (10, 28), (10, 29), (10, 30),
    -- Episode 11: Joel Miller, Ellie, Tommy Miller, Dina, Jesse, Abby, Owen, Nora, Mel, Maria, Seth, Manny
    (11, 1), (11, 2), (11, 3), (11, 4), (11, 5), (11, 6), (11, 7), (11, 8), (11, 9), (11, 20), (11, 27), (11, 28),
    -- Episode 12: Ellie, Tommy Miller, Dina, Jesse, Maria, Seth, Manny, Gail
    (12, 2), (12, 3), (12, 4), (12, 5), (12, 20), (12, 27), (12, 28), (12, 30),
    -- Episode 13: Ellie, Dina, Isaac Dixon, Janowicz
    (13, 2), (13, 4), (13, 31), (13, 32),
    -- Episode 14: Joel Miller, Ellie, Dina, Jesse, Nora
    (14, 1), (14, 2), (14, 4), (14, 5), (14, 8),
    -- Episode 15: Joel Miller, Ellie, Tommy Miller, Dina, Jesse, Maria, Seth, Kat, Gail, Javier Miller, Eugene Lynden
    (15, 1), (15, 2), (15, 3), (15, 4), (15, 5), (15, 20), (15, 27), (15, 29), (15, 30), (15, 33), (15, 34),
    -- Episode 16: Ellie, Tommy Miller, Dina, Jesse, Abby, Owen, Mel, Manny, Isaac Dixon
    (16, 2), (16, 3), (16, 4), (16, 5), (16, 6), (16, 7), (16, 9), (16, 28), (16, 31);
