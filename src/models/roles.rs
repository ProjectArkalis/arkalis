#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Roles {
    Admin,
    Uploader,
    User,
}

impl From<Roles> for String {
    fn from(value: Roles) -> Self {
        match value {
            Roles::Admin => "admin".to_string(),
            Roles::Uploader => "uploader".to_string(),
            Roles::User => "user".to_string(),
        }
    }
}

impl<'a> From<&'a str> for Roles {
    fn from(value: &'a str) -> Self {
        match value {
            "admin" => Roles::Admin,
            "uploader" => Roles::Uploader,
            "user" => Roles::User,
            _ => Roles::User,
        }
    }
}
