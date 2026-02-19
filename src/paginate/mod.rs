mod paginated;
pub use paginated::*;

pub trait PaginatedParams {
    /// the page field of the actual param. pages start at 0.
    fn page(&self) -> u64;

    /// the page_size field of the actual param
    fn param_page_size(&self) -> u64;

    /// Set the page (0-based)
    fn set_page(&mut self, page: u64);

    fn set_page_size(&mut self, page_size: u64);

    fn new_default(page: u64, page_size: u64) -> Self
    where
        Self: Sized + Default,
    {
        let mut this = Self::default();

        this.set_page(page);
        this.set_page_size(page_size);

        this
    }
    /// limits results between 1 and 100 by default
    fn page_size(&self) -> u64 {
        self.param_page_size().clamp(1, 100)
    }
}

#[macro_export]
macro_rules! paginated {
    ($name:ident) => {
        impl $crate::paginate::PaginatedParams for $name {
            fn page(&self) -> u64 {
                self.page.unwrap_or_default()
            }
            fn param_page_size(&self) -> u64 {
                self.page_size.unwrap_or(10)
            }
            fn set_page(&mut self, page: u64) {
                self.page = Some(page);
            }
            fn set_page_size(&mut self, page_size: u64) {
                self.page_size = Some(page_size);
            }
        }
    };
}
