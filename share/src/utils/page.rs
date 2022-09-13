use serde::{Deserialize, Serialize};

#[allow(dead_code)]

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub page: i32,
    pub total_page: i32,
    pub page_size: i32,
}

impl Pagination {
    pub fn init(page_size: i32) -> Pagination {
        Pagination {
            page: 1,
            total_page: 0,
            page_size
        }
    }
}

