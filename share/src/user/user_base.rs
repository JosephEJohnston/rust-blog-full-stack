use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHttp {
    pub id: i64,

    pub name: String,

    pub email: String,

    pub phone: String,

    pub encrypted_password: String,
}

