#[derive(Debug)]
pub struct PaginatedList<T> {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub item: Vec<T>,
}

impl<T> PaginatedList<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.item
    }
}
