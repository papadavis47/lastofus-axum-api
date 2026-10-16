use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::error::AppError;

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

/// A single resource: `{"data": ...}`.
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

/// A page of a collection: `{"data": [...], "meta": {...}}`.
#[derive(Serialize, ToSchema)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

#[derive(Serialize, ToSchema)]
pub struct PageMeta {
    /// The page returned, starting at 1.
    #[schema(example = 1)]
    pub page: u32,
    /// Items per page.
    #[schema(example = 20)]
    pub limit: u32,
    /// Items matching the request across every page.
    #[schema(example = 16)]
    pub total: u64,
    /// Pages needed to list every item; `0` when nothing matches.
    #[schema(example = 1)]
    pub total_pages: u64,
}

/// `?page=&limit=` query parameters shared by every collection endpoint.
#[derive(Debug, Clone, Copy, Deserialize, IntoParams)]
#[serde(default)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    /// Page number, starting at 1. Pages past the end return an empty `data` array.
    #[param(minimum = 1, default = 1)]
    pub page: u32,
    /// Items per page.
    #[param(minimum = 1, maximum = 100, default = 20)]
    pub limit: u32,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

impl Pagination {
    pub fn validate(self) -> Result<Self, AppError> {
        if self.page == 0 {
            return Err(AppError::bad_request(
                "invalid_pagination",
                "page must be 1 or greater.",
            ));
        }
        if self.limit == 0 || self.limit > MAX_PAGE_SIZE {
            return Err(AppError::bad_request(
                "invalid_pagination",
                format!("limit must be between 1 and {MAX_PAGE_SIZE}."),
            ));
        }
        Ok(self)
    }

    /// Rows to skip for this page, for SQL `OFFSET`.
    pub fn offset(self) -> i64 {
        (i64::from(self.page) - 1) * i64::from(self.limit)
    }

    /// Wraps one page of results with metadata. `total` counts every matching
    /// row, not just this page. Pages past the end have empty `data`.
    pub fn page<T>(self, data: Vec<T>, total: u64) -> Paginated<T> {
        Paginated {
            data,
            meta: PageMeta {
                page: self.page,
                limit: self.limit,
                total,
                total_pages: total.div_ceil(u64::from(self.limit)),
            },
        }
    }
}
