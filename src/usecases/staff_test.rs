#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use mockall::predicate::eq;

    use crate::{
        entities::items::Items as ItemsEntity,
        models::item::{Category, StaffAdding},
        repositories::items::MockItemsRepository,
        time_helper::TimerHelper,
        usecases::staff::StaffUsecase,
    };

    #[tokio::test]
    async fn adding_test() {
        let mut items_repository_mock = MockItemsRepository::new();
        let timer_helper = TimerHelper::Mock.creation();

        let req = StaffAdding {
            name: "wooden staff".to_string(),
        };

        items_repository_mock
            .expect_find_by_name()
            .with(eq(req.name.clone()))
            .times(1)
            .returning(|_| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        items_repository_mock
            .expect_insert()
            .with(eq(ItemsEntity::new(
                req.name.clone(),
                Category::Staff.to_string(),
                Arc::clone(&timer_helper),
            )))
            .returning(|_| Box::pin(async { Ok(1) }));

        items_repository_mock
            .expect_find_by_id()
            .with(eq(1))
            .times(1)
            .returning(|_| {
                Box::pin(async {
                    let t = TimerHelper::Mock.creation();
                    Ok(ItemsEntity {
                        id: Some(1),
                        name: "wooden staff".to_string(),
                        category: Category::Staff.to_string(),
                        created_at: t.now(),
                        updated_at: t.now(),
                    })
                })
            });

        let staff_usecase = StaffUsecase::creation(Arc::new(items_repository_mock), timer_helper);

        let result = match staff_usecase.adding(req).await {
            Ok(r) => r,
            Err(_) => panic!("adding error"),
        };

        let id = match Some(result.id) {
            Some(i) => i,
            None => panic!("id is None"),
        };

        assert_eq!(result.id, id);
        assert_eq!(result.name, "wooden staff");
    }
}
