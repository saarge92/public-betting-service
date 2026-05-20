use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// ==========================================
// ВОТ ОНА, МАГИЯ RUST: Создаем красивые имена
// ==========================================
pub type User = Model;            // Теперь User — это синоним Model
pub type UserEntity = Entity;      // UserEntity — синоним Entity
pub type UserActiveModel = ActiveModel; // UserActiveModel — синоним ActiveModel
pub type UserColumn = Column;      // UserColumn — синоним Column