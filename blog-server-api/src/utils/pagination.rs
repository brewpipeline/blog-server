use blog_generic::entities::TotalOffsetLimitContainer;

pub struct Pagination {
    pub offset: u64,
    pub limit: u64,
}

impl Pagination {
    pub fn new(offset: Option<u64>, limit: Option<u64>, max_limit: u64) -> Self {
        Self {
            offset: offset.unwrap_or(0),
            limit: limit.unwrap_or(max_limit).min(max_limit),
        }
    }

    pub fn with_total(&self, total: u64) -> TotalOffsetLimitContainer {
        TotalOffsetLimitContainer {
            total,
            offset: self.offset,
            limit: self.limit,
        }
    }
}
