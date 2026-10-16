//! Wrappers around Axum's extractors whose rejections render as [`AppError`]
//! instead of Axum's default plain-text bodies.

use axum::extract::{FromRequestParts, Path, Query};

use crate::error::AppError;

#[derive(FromRequestParts)]
#[from_request(via(Path), rejection(AppError))]
pub struct ApiPath<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(AppError))]
pub struct ApiQuery<T>(pub T);
