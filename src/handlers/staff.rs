use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{models::item::StaffAdding, tracer::tracing_span, usecases::staff::StaffUsecase};

pub async fn staff_adding(
    Json(body): Json<StaffAdding>,
    staff_usecase: Arc<StaffUsecase>,
) -> impl IntoResponse {
    match staff_usecase.adding(body).await {
        Ok(r) => {
            tracing_span("staff_adding".to_string(), None);
            (StatusCode::CREATED, Json(r)).into_response()
        }
        Err(e) => {
            tracing_span("staff_adding".to_string(), Some(e.error().error));
            e.error().into_response()
        }
    }
}
