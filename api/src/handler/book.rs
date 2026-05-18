use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use registry::AppRegistry;
use thiserror::Error;
use uuid::Uuid;

use crate::model::book::{BookResponse, CreateBookRequest};
