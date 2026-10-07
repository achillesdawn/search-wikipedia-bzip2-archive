mod archive;
mod index;

pub use index::seach_index_entries;

pub const BUFFER_SIZE: usize = 1024 * 8;

#[derive(Debug)]
pub struct IndexEntry {
    pub offset: u64,
    pub page_id: u32,
    pub title: String,
}
