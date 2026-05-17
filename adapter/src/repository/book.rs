use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::{event::CreateBook, Book};
use kernel::repository::book::BookRepositry;
use uuid::Uuid;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepositry for BookRepositryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        let rows: Vec<BookRow> = sqlx::query_as!(
           BoodRow,
            r#"
                SELECT book_id, title, author, isbn, description
                FROM books
                ORDERD BY create_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        Ok(rows.into_iter().map(Book::from).collect())
    }

    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        let rows: Option<BookRow> = sqlx::query_as!(
        BookRow,
        r#"
            SELECT
                book_id,
                title,
                author,
                isbn,
                description,
            FROM books
            WHERE book_id = $1
        "#,
        book_id
        )
        .fetch_optional(self.db.inner_ref())
        .await?;

        Ok(rows.map(Book::from))
    }
}
