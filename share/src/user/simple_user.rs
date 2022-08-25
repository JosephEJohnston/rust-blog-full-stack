use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleUserHttp {
    pub id: i64,

    pub name: String,
}
