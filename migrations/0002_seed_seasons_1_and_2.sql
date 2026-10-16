-- Seed data for seasons 1 and 2.
--
-- Every fact (titles, dates, directors, writers, runtimes) was checked against
-- Wikipedia on 2026-10-15: the season articles for the episode tables and each
-- episode's article for its runtime. Summaries are original, deliberately light
-- on plot, and may contain mild spoilers for the episode they describe.
--
-- Notes:
-- * Season 1 episode 5 is dated 2023-02-10, when it was released online on
--   HBO Max ahead of its 2023-02-12 broadcast (moved for Super Bowl LVII).
--   Wikipedia's table lists the broadcast date.
-- * viewer_count_millions is NULL: the only per-episode figures available are
--   same-night US linear-TV ratings, which badly understate total viewership.
-- * Data changes ship as new migrations; never edit this file once applied.

INSERT INTO seasons (id, number, title, release_year, episode_count, summary, source_name, source_url, last_verified_at) VALUES
    (1, 1, 'Season 1', 2023, 9, 'Twenty years after a fungal outbreak collapses society, a hardened smuggler agrees to escort a teenage girl across a ruined United States.', 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (2, 2, 'Season 2', 2025, 7, 'Years after the events of the first season, life in a rebuilt community is upended and the story turns toward Seattle.', 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15');

INSERT INTO episodes (id, season_id, number_in_season, overall_number, title, slug, summary, runtime_minutes, release_date, director, viewer_count_millions, image_url, source_name, source_url, last_verified_at) VALUES
    (1, 1, 1, 1, 'When You''re Lost in the Darkness', 'when-youre-lost-in-the-darkness', 'The outbreak begins, and two decades later Joel is handed an unusual cargo.', 81, '2023-01-15', 'Craig Mazin', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (2, 1, 2, 2, 'Infected', 'infected', 'The journey out of the Boston quarantine zone proves more dangerous than planned.', 53, '2023-01-22', 'Neil Druckmann', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (3, 1, 3, 3, 'Long, Long Time', 'long-long-time', 'A survivalist''s quiet life changes when a stranger arrives at his fence.', 75, '2023-01-29', 'Peter Hoar', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (4, 1, 4, 4, 'Please Hold to My Hand', 'please-hold-to-my-hand', 'Joel and Ellie reach Kansas City and run into a group hunting for them.', 45, '2023-02-05', 'Jeremy Webb', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (5, 1, 5, 5, 'Endure and Survive', 'endure-and-survive', 'New allies try to slip out of Kansas City before it is too late.', 59, '2023-02-10', 'Jeremy Webb', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (6, 1, 6, 6, 'Kin', 'kin', 'The search for Tommy leads to Wyoming and a hard conversation.', 58, '2023-02-19', 'Jasmila Žbanić', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (7, 1, 7, 7, 'Left Behind', 'left-behind', 'Ellie remembers a night in an abandoned mall with her best friend.', 55, '2023-02-26', 'Liza Johnson', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (8, 1, 8, 8, 'When We Are in Need', 'when-we-are-in-need', 'During a harsh winter, Ellie meets the leader of a struggling community.', 51, '2023-03-05', 'Ali Abbasi', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (9, 1, 9, 9, 'Look for the Light', 'look-for-the-light', 'The journey reaches its destination, and Joel faces a choice.', 43, '2023-03-12', 'Ali Abbasi', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_1', '2026-10-15'),
    (10, 2, 1, 10, 'Future Days', 'future-days', 'Years later, life in Jackson has settled, but Joel and Ellie have grown apart.', 60, '2025-04-13', 'Craig Mazin', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (11, 2, 2, 11, 'Through the Valley', 'through-the-valley', 'Jackson faces a threat on a scale it has never seen.', 57, '2025-04-20', 'Mark Mylod', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (12, 2, 3, 12, 'The Path', 'the-path', 'In the aftermath, Ellie sets her sights on Seattle.', 56, '2025-04-27', 'Peter Hoar', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (13, 2, 4, 13, 'Day One', 'day-one', 'Ellie and Dina arrive in Seattle and find a city at war.', 53, '2025-05-04', 'Kate Herron', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (14, 2, 5, 14, 'Feel Her Love', 'feel-her-love', 'Ellie and Dina press deeper into Seattle as the danger grows.', 45, '2025-05-11', 'Stephen Williams', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (15, 2, 6, 15, 'The Price', 'the-price', 'A look back at Joel and Ellie''s relationship over the years.', 60, '2025-05-18', 'Neil Druckmann', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15'),
    (16, 2, 7, 16, 'Convergence', 'convergence', 'Ellie''s pursuit in Seattle reaches its breaking point.', 50, '2025-05-25', 'Nina Lopez-Corrado', NULL, NULL, 'Wikipedia', 'https://en.wikipedia.org/wiki/The_Last_of_Us_season_2', '2026-10-15');

INSERT INTO episode_writers (episode_id, position, name) VALUES
    (1, 1, 'Craig Mazin'),
    (1, 2, 'Neil Druckmann'),
    (2, 1, 'Craig Mazin'),
    (3, 1, 'Craig Mazin'),
    (4, 1, 'Craig Mazin'),
    (5, 1, 'Craig Mazin'),
    (6, 1, 'Craig Mazin'),
    (7, 1, 'Neil Druckmann'),
    (8, 1, 'Craig Mazin'),
    (9, 1, 'Craig Mazin'),
    (9, 2, 'Neil Druckmann'),
    (10, 1, 'Craig Mazin'),
    (11, 1, 'Craig Mazin'),
    (12, 1, 'Craig Mazin'),
    (13, 1, 'Craig Mazin'),
    (14, 1, 'Craig Mazin'),
    (15, 1, 'Neil Druckmann'),
    (15, 2, 'Halley Gross'),
    (15, 3, 'Craig Mazin'),
    (16, 1, 'Neil Druckmann'),
    (16, 2, 'Halley Gross'),
    (16, 3, 'Craig Mazin');
