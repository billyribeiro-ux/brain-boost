use validator::ValidationError;

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.contains('@') && email.len() >= 3 {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_email"))
    }
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() >= 8 {
        Ok(())
    } else {
        Err(ValidationError::new("password_too_short"))
    }
}
