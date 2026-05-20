use shaku::module;
use crate::api::UserHandler;
use crate::repository::{UserRepository};
use crate::operation::user::{UserService};

module! {
    pub AppContainer {
        components = [UserRepository, UserService],
        providers = [
            UserHandler
        ]
    }
}