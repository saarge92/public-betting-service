use shaku::module;
use crate::operation::user::UserService;
use crate::repository::{UserRepository};

module! {
    pub AppContainer {
        components = [UserRepository, UserService],
        providers = []
    }
}