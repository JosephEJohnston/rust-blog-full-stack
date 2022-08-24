use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TagHttp {
    pub id: i64,

    pub user_id: i64,

    pub name: String,
}
