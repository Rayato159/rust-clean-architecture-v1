use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{models::item::StaffAdding, usecases::staff::StaffUsecase};

pub async fn staff_adding(
    Json(body): Json<StaffAdding>,
    staff_usecase: Arc<StaffUsecase>,
) -> impl IntoResponse {
    let staff = match staff_usecase.adding(body).await {
        Ok(r) => r,
        Err(e) => return e.error().into_response(),
    };

    (StatusCode::CREATED, Json(staff)).into_response()
}
