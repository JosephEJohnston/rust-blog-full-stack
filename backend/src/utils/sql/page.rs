use share::utils::page::Pagination;

#[derive(Clone, Copy, Debug)]
pub struct PaginationSql {
    pub limit: i64,
    pub offset: i64,
}

impl From<Pagination> for PaginationSql {
    fn from(page: Pagination) -> Self {
        let size = page.page_size;

        PaginationSql {
            limit: size as i64,
            offset: ((page.page - 1) * size) as i64,
        }
    }
}