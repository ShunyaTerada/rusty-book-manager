use std::net::{Ipv4Addr, SocketAddr};
use adapter::database::connect_database_with;
use anyhow::Result;
use api::route::{book::build_book_routers, health::build_health_check_routers};
use axum::http::Method;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use tokio::net::TcpListener;
use shared::env::{which, Environment};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use anyhow::Context;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tower_http::LatencyUnit;
use tower_http::cors::{self, CorsLayer};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
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
        .layer(cors())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    //サーバーを起動する
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happend in server")       
        .inspect_err(|e| {
            tracing::error! (
            error.cause_chain = ?e,
            error.message = %e,
            "Unexpected error"
            )
        })
}

// CORS の設定を行う関数
fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

//ロガーを初期化する関数
fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    //ログレベル設定
    let env_filter = 
        EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    //ログの出力形式を設定
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}



