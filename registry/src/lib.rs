use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::{book::BookRepositryImpl,health::HealthCheckRepositoryImpl}};
use kernel::{model::book::Book, repository::{book::BookRepositry,health::HealthCheckRepository}};


//DIコンテナの役割を果たす構造体定義する。Cloneは後ほどaxum側で必要になるため。
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
    book_repository: Arc<dyn BookRepositry>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        //依存解決を行う。関数内で手書きする。
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let book_repository = Arc::new(BookRepositryImpl::new(pool.clone()));
        Self {
            health_check_repository,
            book_repository
        }
    }

    //依存解決したインスタンスを返すメソッドを定義する。
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepositry> {
        self.book_repository.clone()
    }
}
