pub struct RegisterUserDto {
    pub username: String,
    pub email: String,
    pub password_raw: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}