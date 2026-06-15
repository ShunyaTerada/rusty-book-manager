use async_trait::async_trait;
use derive_new::new;
use kernel::model::checkout::event::{CreateCheckout, UpdateReturned};
use kernel::model::id::{BookId, UserId};
use kernel::{model::checkout::Checkout, repository::checkout::CheckoutRepository};
use shared::error::{AppError, AppResult};

use crate::database::ConnectionPool;
use crate::database::model::checkout::{CheckoutRow, CheckoutStateRow, ReturnedCheckoutRow};

#[derive(new)]
pub struct CheckoutRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl CheckoutRepository for CheckoutRepositoryImpl {
    async fn create(&self, event: CreateCheckout) -> AppResult<()> {
        todo!()
    }
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()> {
        todo!()
    }
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>> {
        let rows = sqlx::query_as!(
            CheckoutRow,
            r#"
                SELECT 
                    c.checkout_id, 
                    c.book_id,
                    c.user_id, 
                    c.checked_out_at, 
                    b.title,
                    b.author,
                    b.isbn
                FROM checkouts AS c
                JOIN books AS b USING(book_id)
                ORDER BY c.checked_out_at DESC
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(AppError::SpecificOperationError)?;

        Ok(rows.into_iter().map(Checkout::from).collect())
    }
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>> {
        todo!()
    }
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>> {
        todo!()
    }
}
