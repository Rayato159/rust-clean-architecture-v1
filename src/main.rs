use axum::{
    error_handling::HandleErrorLayer,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    BoxError, Router,
};
use rust_clean_architecture_v1::{
    database, handlers::staff::staff_adding, models::error::ErrorResponse,
    repositories::staff::StaffRepository, setting::Setting, time_helper::TimerHelper,
    tracer::init_tracer, usecases::staff::StaffUsecase,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::TcpListener, signal};
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    trace::TraceLayer,
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    init_tracer();

    let setting = Setting::new().unwrap();
    info!("setting has been loaded.");

    let db_pool = database::conn_getting(Arc::clone(&setting)).await.unwrap();
    info!("database connection has been established.");

    let staff_repository = StaffRepository::creation(db_pool.clone());
    let timer_helper = TimerHelper::Directly.creation();
    let staff_usecase =
        StaffUsecase::creation(Arc::clone(&staff_repository), Arc::clone(&timer_helper));

    // build our application with a single route
    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(RequestBodyLimitLayer::new(
            (setting.server.body_limit * 1024 * 1024)
                .try_into()
                .unwrap(),
        ))
        .route("/", get(health_check))
        .route(
            "/items/staff",
            post({
                let usecase = Arc::clone(&staff_usecase);
                move |body| staff_adding(body, usecase)
            }),
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(
                    setting.server.timeout.try_into().unwrap(),
                ))),
        )
        .fallback(not_found);

    let addr = SocketAddr::from(([0, 0, 0, 0], setting.server.port as u16));

    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Server running on port {}", setting.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn not_found() -> impl IntoResponse {
    ErrorResponse {
        error: "Endpoint not found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    }
    .into_response()
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Starting graceful shutdown");
        },
        _ = terminate => {},
    }
}
