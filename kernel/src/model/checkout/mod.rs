use chrono::{DateTime, Utc};

use crate::model::id::{BookId, CheckoutId, UserId};

pub mod event;

pub struct Checkout {
    pub id: CheckoutId,
    pub checked_out_by: DateTime<Utc>,
    pub checked_out_at: UserId,
    pub returned_at: Option<DateTime<Utc>>,
    pub book: CheckoutBook,
}

pub struct CheckoutBook {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
}
