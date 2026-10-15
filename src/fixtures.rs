//! Phase 2 in-memory dataset. Phase 3 replaces this with a SQLite seed.
//!
//! VERIFY BEFORE SEEDING: these values were entered from memory, not checked
//! against a source, so they must be verified (and given `source_url` and
//! `last_verified_at`) before becoming the Phase 3 seed. Values that could not
//! be confirmed are `None` (serialized as `null`) per the API conventions.
//! Season 2's episodes are deliberately not included yet.
//!
//! Summaries are original, deliberately light on plot, and may contain mild
//! spoilers for the episode they describe.

use crate::models::{Episode, Season};

pub fn seasons() -> Vec<Season> {
    vec![
        Season {
            id: 1,
            number: 1,
            title: "Season 1".into(),
            release_year: 2023,
            episode_count: 9,
            summary: "Twenty years after a fungal outbreak collapses society, a hardened \
                      smuggler agrees to escort a teenage girl across a ruined United States."
                .into(),
        },
        Season {
            id: 2,
            number: 2,
            title: "Season 2".into(),
            release_year: 2025,
            episode_count: 7,
            summary: "Years after the events of the first season, life in a rebuilt \
                      community is upended and the story turns toward Seattle."
                .into(),
        },
    ]
}

/// `(title, slug, release_date, director, writers, summary)`
type EpisodeRow = (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static [&'static str],
    &'static str,
);

pub fn episodes() -> Vec<Episode> {
    let season_one: [EpisodeRow; 9] = [
        (
            "When You're Lost in the Darkness",
            "when-youre-lost-in-the-darkness",
            "2023-01-15",
            "Craig Mazin",
            &["Craig Mazin", "Neil Druckmann"],
            "The outbreak begins, and two decades later Joel is handed an unusual cargo.",
        ),
        (
            "Infected",
            "infected",
            "2023-01-22",
            "Neil Druckmann",
            &["Craig Mazin"],
            "The journey out of the Boston quarantine zone proves more dangerous than planned.",
        ),
        (
            "Long, Long Time",
            "long-long-time",
            "2023-01-29",
            "Peter Hoar",
            &["Craig Mazin"],
            "A survivalist's quiet life changes when a stranger arrives at his fence.",
        ),
        (
            "Please Hold to My Hand",
            "please-hold-to-my-hand",
            "2023-02-05",
            "Jeremy Webb",
            &["Craig Mazin"],
            "Joel and Ellie reach Kansas City and run into a group hunting for them.",
        ),
        (
            "Endure and Survive",
            "endure-and-survive",
            "2023-02-10",
            "Jeremy Webb",
            &["Craig Mazin"],
            "New allies try to slip out of Kansas City before it is too late.",
        ),
        (
            "Kin",
            "kin",
            "2023-02-19",
            "Jasmila Žbanić",
            &["Craig Mazin"],
            "The search for Tommy leads to Wyoming and a hard conversation.",
        ),
        (
            "Left Behind",
            "left-behind",
            "2023-02-26",
            "Liza Johnson",
            &["Neil Druckmann"],
            "Ellie remembers a night in an abandoned mall with her best friend.",
        ),
        (
            "When We Are in Need",
            "when-we-are-in-need",
            "2023-03-05",
            "Ali Abbasi",
            &["Craig Mazin"],
            "During a harsh winter, Ellie meets the leader of a struggling community.",
        ),
        (
            "Look for the Light",
            "look-for-the-light",
            "2023-03-12",
            "Ali Abbasi",
            &["Craig Mazin"],
            "The journey reaches its destination, and Joel faces a choice.",
        ),
    ];

    season_one
        .into_iter()
        .zip(1..)
        .map(
            |((title, slug, release_date, director, writers, summary), number)| Episode {
                id: number,
                season_id: 1,
                number_in_season: number,
                overall_number: number,
                title: title.into(),
                slug: slug.into(),
                summary: summary.into(),
                runtime_minutes: None,
                release_date: release_date.into(),
                director: director.into(),
                writers: writers.iter().map(|&w| w.into()).collect(),
                viewer_count_millions: None,
                image_url: None,
            },
        )
        .collect()
}
