pub trait PageProcessor {
    fn create_for_page(page: &u64) -> Self;
    fn limit(&self) -> u64;
    fn offset(&self) -> u64;
}

pub struct DefaultPageProcessor<const LIMIT: u64 = 10> {
    page: u64,
}

impl<const LIMIT: u64> PageProcessor for DefaultPageProcessor<LIMIT> {
    fn create_for_page(page: &u64) -> Self {
        Self { page: *page }
    }
    fn limit(&self) -> u64 {
        LIMIT
    }
    fn offset(&self) -> u64 {
        let Some(real_page) = self.page.checked_sub(1) else {
            return 0;
        };
        let offset = real_page * LIMIT;
        offset
    }
}
