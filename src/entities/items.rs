use crate::{
    models::{
        error::{APIError, IntoErrorResponse},
        item::{Category, Item as ItemModel},
    },
    time_helper::IntoTimerHelperShared,
};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow, PartialEq)]
pub struct Items {
    pub id: Option<i32>,
    pub name: String,
    pub category: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Items {
    pub fn new(name: String, category: String, t: IntoTimerHelperShared) -> Self {
        Self {
            id: None,
            name,
            category,
            created_at: t.now(),
            updated_at: t.now(),
        }
    }

    pub fn to_model(&self) -> Result<ItemModel, Box<dyn IntoErrorResponse>> {
        let category = match self.get_category() {
            Some(category) => category,
            None => {
                return Err(Box::new(APIError::InvalidCategory(self.category.clone())));
            }
        };

        Ok(ItemModel {
            id: self.id.unwrap(),
            name: self.name.to_string(),
            category,
        })
    }

    pub fn get_category(&self) -> Option<Category> {
        match self.category.as_str() {
            "Staff" => Some(Category::Staff),
            "Sword" => Some(Category::Sword),
            _ => None,
        }
    }
}
