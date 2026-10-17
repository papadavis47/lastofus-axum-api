//! The OpenAPI document. Paths and their schemas come from the `#[utoipa::path]`
//! attributes on the handlers registered in `app.rs`; this adds the top-level
//! information around them.

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "The Last of Us API",
        description = "An unofficial, educational, read-only API for HBO's *The Last of Us* \
            TV series. It is not affiliated with or endorsed by HBO, Sony or Naughty Dog.\n\n\
            Every collection is paginated with `page` and `limit`. Every error, whatever its \
            status, has the same `{\"error\": {status, code, message}}` body. Episode summaries \
            are short and original, but may contain mild spoilers for the episode they describe.",
        license(name = "MIT", identifier = "MIT"),
    ),
    tags(
        (name = "seasons", description = "Seasons of the series and their episodes."),
        (name = "episodes", description = "Every episode, in broadcast order."),
        (name = "characters", description = "Characters, their billing in each season, and the episodes they're credited in."),
        (name = "health", description = "Service status."),
    ),
)]
pub struct ApiDoc;

/// The interactive documentation page. It loads Scalar from a CDN and points it
/// at the spec served next to it.
pub const DOCS_HTML: &str = include_str!("docs.html");
