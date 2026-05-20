use crate::domain::{User, UserActiveModel, UserEntity, UserColumn};
use crate::operation::user::RegisterUserDto;
use sea_orm::prelude::{Uuid, async_trait};
use sea_orm::sea_query::prelude::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, DbConn, DbErr, EntityTrait, QueryFilter, Set};

#[async_trait::async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(
        &self,
        user: RegisterUserDto,
        password_hash: String,
    ) -> Result<User, DbErr>;

    async fn find_user_by_username_or_email(&self, username: String, email: String) -> Result<Option<User>, DbErr>;
}

pub struct UserRepository {
    db: DbConn,
}

impl UserRepository {
    pub fn new(db: DbConn) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn create_user(
        &self,
        user: RegisterUserDto,
        password_hash: String,
    ) -> Result<User, DbErr> {
        let new_user = UserActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(user.username.clone()),
            email: Set(user.email.clone()),
            password_hash: Set(password_hash),
            is_active: Set(true),
            created_at: Set(Utc::now().into()),
        };

        new_user.insert(&self.db).await
    }

    async fn find_user_by_username_or_email(&self, username: String, email: String) -> Result<Option<User>, DbErr> {
        UserEntity::find()
            .filter(
                Condition::any() // Переключаем режим на "ИЛИ"
                    .add(UserColumn::Username.eq(username))
                    .add(UserColumn::Email.eq(email))
            )
            .one(&self.db)
            .await
    }
}
