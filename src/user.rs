use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn create_user() -> User {
        let mut new_user = User {
            name: String::new(),
            email: String::new(),
            password: String::new(),
        };
        new_user.name_input();
        new_user.email_input();
        new_user.password_input();
        new_user
    }
}
