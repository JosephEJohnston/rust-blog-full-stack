use serde::{Deserialize, Serialize};

#[allow(dead_code)]

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pagination<T> {
    pub page: i32,
    pub total_page: i64,
    pub page_size: i32,
    pub data: T
}

/*impl <T> Pagination<T> {
    pub fn init(page_size: i32) -> Pagination<T> {
        Pagination {
            page: 1,
            total_page: 0,
            page_size,
            data: None
        }
    }
}
*/


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: i32,
    pub page_size: i32,
}

impl PageRequest {
    pub fn init(page_size: i32) -> PageRequest {
        PageRequest {
            page: 1,
            page_size,
        }
    }
}

