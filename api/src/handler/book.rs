use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use garde::Validate;
use kernel::model::{
    book::{
        BookListOptions,
        event::{DeleteBook, UpdateBook},
    },
    id::BookId,
};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::book::{
        BookListQuery, BookResponse, CreateBookRequest, PaginatedBookResponse, UpdateBookRequest,
        UpdateBookRequestWithIds,
    },
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
    Query(list): Query<BookListQuery>,
) -> AppResult<Json<PaginatedBookResponse>> {
    list.validate(&())?;
    let options: BookListOptions = list.into();

    registry
        .book_repository()
        .find_all(options)
        .await
        .map(|v| Json(v.into()))
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

pub async fn update_book(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(book_id): Path<BookId>,
    Json(req): Json<UpdateBookRequest>,
) -> AppResult<StatusCode> {
    req.validate(&())?;

    let book_req = UpdateBookRequestWithIds::new(book_id, user.id(), req);

    let event: UpdateBook = book_req.into();

    registry
        .book_repository()
        .update(event)
        .await
        .map(|_| StatusCode::OK)
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
