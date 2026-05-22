use crate::api::UserHandler;
use crate::operation::user::UserService;
use crate::operation::user::auth_service::AuthService;
use crate::repository::UserRepository;
use shaku::module;

module! {
    pub AppContainer {
        components = [UserRepository, UserService, AuthService],
        providers = [
            UserHandler
        ]
    }
}
