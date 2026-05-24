use crate::api::UserController;
use crate::api::wallet::controller::WalletController;
use crate::operation::user::UserService;
use crate::operation::user::auth_service::AuthService;
use crate::operation::wallet::create::wallet_service::CreateWalletService;
use crate::repository::UserRepository;
use crate::repository::wallet::WalletRepository;
use shaku::module;

module! {
    pub AppContainer {
        components = [UserRepository, UserService, AuthService, CreateWalletService, WalletRepository],
        providers = [
            UserController,
            WalletController
        ]
    }
}
