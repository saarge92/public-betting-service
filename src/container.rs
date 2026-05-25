use crate::api::UserController;
use crate::api::wallet::controller::WalletController;
use crate::operation::user::UserService;
use crate::operation::user::auth_service::AuthService;
use crate::operation::wallet::create::service::CreateWalletService;
use crate::operation::wallet::list::service::WalletListService;
use crate::repository::UserRepository;
use crate::repository::wallet::WalletRepository;
use shaku::module;

module! {
    pub AppContainer {
        components = [
            UserRepository,
            UserService,
            AuthService,
            CreateWalletService,
            WalletRepository,
            WalletListService
        ],
        providers = [
            UserController,
            WalletController
        ]
    }
}
