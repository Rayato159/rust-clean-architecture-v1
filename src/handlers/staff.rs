use crate::{models::item::StaffAdding, usecases::staff::StaffUsecase};
use axum::{http::StatusCode, response::IntoResponse, Json};
use opentelemetry::trace::Span;
use opentelemetry::trace::Tracer;
use std::sync::Arc;
use tracing_span::tracing_execution;

#[tracing_execution]
pub async fn staff_adding(
    Json(body): Json<StaffAdding>,
    staff_usecase: Arc<StaffUsecase>,
) -> impl IntoResponse {
    match staff_usecase.adding(body).await {
        Ok(r) => (StatusCode::CREATED, Json(r)).into_response(),
        Err(e) => e.error().into_response(),
    }
}
