use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

/// A single resource: `{"data": ...}`.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

/// A page of a collection: `{"data": [...], "meta": {...}}`.
#[derive(Serialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

#[derive(Serialize)]
pub struct PageMeta {
    pub page: u32,
    pub limit: u32,
    pub total: usize,
    pub total_pages: usize,
}

/// `?page=&limit=` query parameters shared by every collection endpoint.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default)]
pub struct Pagination {
    pub page: u32,
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

    /// Slices an already-filtered collection into the requested page.
    /// Pages past the end return an empty `data` array, not an error.
    pub fn paginate<T: Clone>(self, items: &[T]) -> Paginated<T> {
        let limit = self.limit as usize;
        let start = (self.page as usize - 1).saturating_mul(limit);
        let data = items.iter().skip(start).take(limit).cloned().collect();

        Paginated {
            data,
            meta: PageMeta {
                page: self.page,
                limit: self.limit,
                total: items.len(),
                total_pages: items.len().div_ceil(limit),
            },
        }
    }
}
