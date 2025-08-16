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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_one_has_zero_offset() {
        let processor = DefaultPageProcessor::<10>::create_for_page(&1);
        assert_eq!(processor.limit(), 10);
        assert_eq!(processor.offset(), 0);
    }

    #[test]
    fn page_zero_does_not_underflow() {
        let processor = DefaultPageProcessor::<10>::create_for_page(&0);
        assert_eq!(processor.offset(), 0);
    }

    #[test]
    fn page_five_calculates_correct_offset() {
        let processor = DefaultPageProcessor::<10>::create_for_page(&5);
        assert_eq!(processor.offset(), 40);
    }

    #[test]
    fn custom_limit_is_respected() {
        let processor = DefaultPageProcessor::<20>::create_for_page(&2);
        assert_eq!(processor.limit(), 20);
        assert_eq!(processor.offset(), 20);
    }

    #[test]
    fn high_page_with_limit_one_avoids_overflow() {
        let processor = DefaultPageProcessor::<1>::create_for_page(&u64::MAX);
        assert_eq!(processor.offset(), u64::MAX - 1);
    }
}
