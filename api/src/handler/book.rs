use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use hyper::StatusCode;
use kernel::model::{book::event::DeleteBook, id::BookId};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::book::{BookResponse, CreateBookRequest, UpdateBookRequest, UpdateBookRequestWithIds},
};

pub async fn register_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> AppResult<StatusCode> {
    registry
        .book_repository()
        .create(req.into(), user.id())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    State(registry): State<AppRegistry>,
) -> AppResult<Json<Vec<BookResponse>>> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
        .map(Json)
}

pub async fn show_book(
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<BookResponse>> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|bc| match bc {
            Some(bc) => Ok(Json(bc.into())),
            None => Err(AppError::EntityNotFound(
                "The specific book was not found".into(),
            )),
        })
}

pub async fn delete_book(
    user: AuthorizedUser,
    Path(book_id): Path<BookId>,
    State(registry): State<AppRegistry>,
) -> AppResult<StatusCode> {
    let event = DeleteBook {
        book_id,
        requested_user: user.id(),
    };
    registry
        .book_repository()
        .delete(event)
        .await
        .map(|_| StatusCode::NO_CONTENT)
}
