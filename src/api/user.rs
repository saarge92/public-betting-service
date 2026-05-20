use std::sync::Arc;
use shaku::{Component, Interface, Module, ModuleBuildContext};
use crate::operation::user::UserServiceTrait;

// 1. Создаем маркерный интерфейс для shaku
pub trait UserHandlerTrait: Interface + Send + Sync {}

// 2. Позволяем макросу самому сгенерировать все скрытые типы контекста
#[derive(Component)]
#[shaku(interface = UserHandlerTrait)]
pub struct UserHandler {
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceTrait>,
}

// 3. Реализуем пустой маркерный трейт
impl UserHandlerTrait for UserHandler {}