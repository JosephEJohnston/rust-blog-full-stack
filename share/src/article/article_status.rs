use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArticleStatusHttp {
    pub id: i64,

    pub status: i8,
}