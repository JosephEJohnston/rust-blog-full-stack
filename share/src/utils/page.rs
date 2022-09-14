use serde::{Deserialize, Serialize};

#[allow(dead_code)]

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pagination<T> {
    pub page: i64,
    pub total_page: i64,
    pub page_size: i64,
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

impl <T> Pagination<T> {
    pub fn to_page<V, F>(self, func: F) -> Pagination<V>
    where
        F: Fn(T) -> V
    {
        Pagination {
            page: self.page,
            total_page: self.total_page,
            page_size: self.page_size,
            data: func(self.data),
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: i64,
    pub page_size: i64,
}

impl PageRequest {
    pub fn init(page_size: i64) -> PageRequest {
        PageRequest {
            page: 1,
            page_size,
        }
    }
}

