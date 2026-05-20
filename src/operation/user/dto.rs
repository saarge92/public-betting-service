use validator::Validate;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(email(message = "Некорректный формат email"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 32,
        message = "Пароль должен быть от 8 до 32 символов"
    ))]
    pub password_raw: String,

    #[validate(length(
        min = 2,
        message = "Имя пользователя должно содержать минимум 2 символа"
    ))]
    pub username: String,


    #[validate(length(
        min = 2,
        max = 100,
        message = "Имя должно содержать минимум 2 символа"
    ))]
    pub first_name: Option<String>,

    #[validate(length(
        min = 2,
        max = 100,
        message = "Имя должно содержать минимум 2 символа"
    ))]
    pub last_name: Option<String>,
}