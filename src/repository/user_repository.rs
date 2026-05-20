use crate::domain::{User, UserActiveModel, UserColumn, UserEntity};
use crate::operation::user::RegisterUserDto;
use sea_orm::prelude::{Uuid, async_trait};
use sea_orm::sea_query::prelude::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DbConn, DbErr, EntityTrait, QueryFilter, Set,
};
use shaku::{Component, Interface};

#[async_trait::async_trait]
pub trait UserRepositoryTrait: Interface + Send + Sync {
    async fn create_user(
        &self,
        user: RegisterUserDto,
        password_hash: String,
    ) -> Result<User, DbErr>;

    async fn find_user_by_username_or_email(
        &self,
        username: String,
        email: String,
    ) -> Result<Option<User>, DbErr>;
}

#[derive(Component)]
#[shaku(interface = UserRepositoryTrait)]
pub struct UserRepository {
    #[shaku(default)]
    db: DbConn,
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

    async fn find_user_by_username_or_email(
        &self,
        username: String,
        email: String,
    ) -> Result<Option<User>, DbErr> {
        UserEntity::find()
            .filter(
                Condition::any() // Переключаем режим на "ИЛИ"
                    .add(UserColumn::Username.eq(username))
                    .add(UserColumn::Email.eq(email)),
            )
            .one(&self.db)
            .await
    }
}
