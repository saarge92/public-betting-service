use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWalletDto {
    #[validate(custom(function = "validate_iso_currency"))]
    pub currency: String,
}

fn validate_iso_currency(currency: &str) -> Result<(), ValidationError> {
    let clean_currency = currency.trim().to_uppercase();

    if iso_currency::Currency::from_code(&clean_currency).is_some() {
        Ok(())
    } else {
        let mut err = ValidationError::new("Неверно указана валюта");
        err.message = Some(std::borrow::Cow::Borrowed(
            "Предоставленный код валюты не соответствует стандарту ISO 4217",
        ));
        Err(err)
    }
}
