use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    #[serde(rename = "jwt_secret_env")]
    pub jwt_secret: String,
}

pub fn load_config() -> AppConfig {
    envy::from_env::<AppConfig>().expect("Критическая ошибка: не удалось загрузить .env")
}
