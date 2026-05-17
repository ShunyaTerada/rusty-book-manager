use kernel::model::book::{event::CreateBook, Book};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for CreateBook {
   fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest { 
            title, 
            author, 
            isbn, 
            description 
        } = value;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}
