use std::sync::Arc;

use crate::{
    entities::items::Items as ItemsEntity,
    models::{
        error::{APIError, IntoErrorResponse},
        item::{Category, Item, StaffAdding},
    },
    repositories::items::SharedItemsRepository,
    time_helper::IntoTimerHelperShared,
};

pub struct StaffUsecase {
    items_repository: SharedItemsRepository,
    timer_helper: IntoTimerHelperShared,
}

impl StaffUsecase {
    pub fn creation(
        items_repository: SharedItemsRepository,
        timer_helper: IntoTimerHelperShared,
    ) -> Arc<Self> {
        Arc::new(Self {
            items_repository,
            timer_helper,
        })
    }

    pub async fn adding(&self, staff: StaffAdding) -> Result<Item, Box<dyn IntoErrorResponse>> {
        if let Ok(_) = self.items_repository.find_by_name(staff.name.clone()).await {
            return Err(Box::new(APIError::ItemAlreadyExists(staff.name.clone())));
        };

        let id = match self
            .items_repository
            .insert(ItemsEntity::new(
                staff.name.clone(),
                Category::Staff.to_string(),
                Arc::clone(&self.timer_helper),
            ))
            .await
        {
            Ok(id) => id,
            Err(e) => return Err(Box::new(APIError::AddingItemError(e))),
        };

        let staff_entity = match self.items_repository.find_by_id(id).await {
            Ok(r) => r,
            Err(_) => return Err(Box::new(APIError::ItemNotFound(id))),
        };

        Ok(match staff_entity.to_model() {
            Ok(r) => r,
            Err(e) => return Err(e),
        })
    }
}
