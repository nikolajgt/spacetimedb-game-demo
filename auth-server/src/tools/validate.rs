
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}


pub fn is_secure_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(char::is_uppercase)
}

