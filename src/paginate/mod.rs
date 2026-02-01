mod paginated;
pub use paginated::*;

// mod params;
// pub use params::*;

pub trait PaginatedParams {
    /// the page field of the actual param (should be 1 minimum)
    fn param_page(&self) -> u64 {
        1
    }
    /// the page_size field of the actual param
    fn param_page_size(&self) -> u64 {
        10
    }

    /// Set the page (should be 1 minimum)
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
    /// returns page after decrementing by 1. so page 0 and 1 from deserialize
    ///
    /// passed are the same. 2 is 1, etc. pages start at zero for db
    fn page(&self) -> u64 {
        self.param_page().saturating_sub(1)
    }
    /// limits results between 1 and 100 by default
    fn page_size(&self) -> u64 {
        self.param_page_size().clamp(1, 100)
    }
}

#[macro_export]
macro_rules! paginated {
    ($name:ident) => {
        impl PaginatedParams for $name {
            fn param_page(&self) -> u64 {
                self.page.unwrap_or(1)
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
