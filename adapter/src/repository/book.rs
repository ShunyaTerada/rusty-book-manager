use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::BookListOptions;
use kernel::model::book::event::{DeleteBook, UpdateBook};
use kernel::model::book::{Book, event::CreateBook};
use kernel::model::id::{BookId, UserId};
use kernel::model::list::PaginatedList;
use kernel::repository::book::BookRepository;
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;
use crate::database::model::book::{BookRow, PaginatedBookRow};

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook, user_id: UserId) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description, user_id)
                VALUES($1, $2, $3, $4, $5)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            user_id as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(())
    }

    async fn find_all(&self, option: BookListOptions) -> AppResult<PaginatedList<Book>> {
        let BookListOptions { offset, limit } = option;
        let paginated_book_rows: Vec<PaginatedBookRow> = sqlx::query_as!(
            PaginatedBookRow,
            r#"
                SELECT COUNT(*) OVER() AS "total!", book_id AS id
                FROM books
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let total = paginated_book_rows
            .first()
            .map(|r| r.total)
            .unwrap_or_default();

        let book_ids = paginated_book_rows
            .into_iter()
            .map(|r| r.id)
            .collect::<Vec<BookId>>();

        let rows: Vec<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
                SELECT 
                b.book_id, 
                b.title, 
                b.author, 
                b.isbn, 
                b.description,
                u.user_id AS owned_by,
                u.name AS owner_name
                FROM books AS b
                INNER JOIN users AS u USING(user_id)
                WHERE b.book_id = ANY($1)
                ORDER BY b.created_at DESC
            "#,
            &book_ids as _
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        let book_rows = rows.into_iter().map(Book::from).collect();

        Ok(PaginatedList {
            total,
            offset,
            limit,
            item: book_rows,
        })
    }

    async fn find_by_id(&self, book_id: BookId) -> AppResult<Option<Book>> {
        let row: Option<BookRow> = sqlx::query_as!(
            BookRow,
            r#"
            SELECT
                b.book_id AS book_id,
                b.title AS title,
                b.author AS author,
                b.isbn AS isbn,
                b.description AS description,
                u.user_id AS owned_by,
                u.name AS owner_name
            FROM books AS b
            INNER JOIN users AS u USING(user_id)
            WHERE b.book_id = $1
        "#,
            book_id as _
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(row.map(Book::from))
    }

    async fn update(&self, event: UpdateBook) -> AppResult<()> {
        let res = sqlx::query!(
            r#"
                UPDATE books
                SET
                    title = $1,
                    author = $2,
                    isbn = $3,
                    description = $4
                WHERE book_id = $5 AND user_id = $6
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
            event.book_id as _,
            event.requested_user as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "該当する書籍が見つかりません".to_string(),
            ));
        }

        Ok(())
    }

    async fn delete(&self, event: DeleteBook) -> AppResult<()> {
        let DeleteBook {
            book_id,
            requested_user,
        } = event;

        let res = sqlx::query!(
            r#"
                DELETE FROM books
                WHERE book_id = $1 AND user_id = $2
            "#,
            book_id as _,
            requested_user as _,
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        if res.rows_affected() < 1 {
            return Err(AppError::EntityNotFound(
                "該当する書籍が見つかりません".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    #[ignore = "booksテーブルにuser_idを追加したことをアプリケーション側に反映するまで保留"]
    async fn test_register_book(pool: sqlx::PgPool) -> AppResult<()> {
        //BookRepositryImplを初期化
        let repo = BookRepositoryImpl::new(ConnectionPool::new(pool));

        //投入するための蔵書データを作成
        let book = CreateBook {
            title: "Test Title".into(),
            author: "Test Author".into(),
            isbn: "Test ISBN".into(),
            description: "Test Description".into(),
        };

        //蔵書データを投入すると正常終了することを確認
        repo.create(book).await?;

        //蔵書の一覧を取得すると投入した一件だけ取得することを確認
        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        //蔵書の一覧の最初のデータから蔵書IDを取得し、
        //find_by_idメソッドでその蔵書データを取得できることを確認
        let book_id = res[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        //取得した蔵書データがCreateBookで投入した
        //蔵書データと一致することを確認
        let Book {
            id,
            title,
            author,
            isbn,
            description,
        } = res.unwrap();
        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");

        Ok(())
    }
}
