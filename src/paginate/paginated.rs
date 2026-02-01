use serde::{Deserialize, Serialize};

pub struct PaginationInfo {
    pub page: u64,
    pub page_size: u64,
    pub number_of_items: u64,
    pub number_of_pages: u64,
}
impl PaginationInfo {
    pub fn new(number_of_items: u64, number_of_pages: u64, page: u64, page_size: u64) -> Self {
        Self {
            page,
            page_size,
            number_of_items,
            number_of_pages,
        }
    }
}

pub struct PaginateBuilder<T> {
    pub data: T,
    pub page: u64,
    pub page_size: u64,
    pub num_items: u64,
    pub num_pages: u64,
}

impl<I> PaginateBuilder<I> {
    pub fn from_parts(data: I, info: PaginationInfo) -> Self {
        Self {
            data,
            page: info.page,
            page_size: info.page_size,
            num_items: info.number_of_items,
            num_pages: info.number_of_pages,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub page_size: u64,
    pub num_items: u64,
    pub num_pages: u64,
}

impl<T> Paginated<T> {
    pub fn from_parts(data: Vec<T>, info: PaginationInfo) -> Self {
        Self::new(
            data,
            info.number_of_items,
            info.number_of_pages,
            info.page,
            info.page_size,
        )
    }
    pub fn new(
        data: Vec<T>,
        number_of_items: u64,
        number_of_pages: u64,
        page: u64,
        page_size: u64,
    ) -> Self {
        Self {
            data,
            page,
            page_size,
            num_items: number_of_items,
            num_pages: number_of_pages,
        }
    }
}
