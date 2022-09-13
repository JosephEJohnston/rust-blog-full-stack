use serde::{Deserialize, Serialize};
use crate::utils::page::Pagination;
use crate::utils::status::StatusOptions;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ListArticleOptions {
    pub user_id: i64,
    pub status: StatusOptions,
    pub page: Pagination,
}

