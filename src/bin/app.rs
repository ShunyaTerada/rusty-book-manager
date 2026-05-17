use std::net::{Ipv4Addr, SocketAddr};

use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::{
    book::build_book_routers,
    health::build_health_check_routers
};
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

//後々ログの初期化などの他の関数をmain関数内に挟むため、今のうちにサーバー起動分だけを分離しておく。
async fn bootstrap() -> Result<()> {
    // creat AppConfig
    let app_config = AppConfig::new()?;
    // データベースの接続を行う。コネクションプールを取り出しておく。
    let pool = connect_database_with(&app_config.database);

    //AppRegistryを生成する。
    let registry = AppRegistry::new(pool);

    //build_health_check_routers関数を呼び出す。AppRegistyをRouterに登録しておく。
    let app = Router::new()
        .merge(build_health_check_routers())
        .merge(build_book_routers())
        .with_state(registry);

    //サーバーを起動する
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listener on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
