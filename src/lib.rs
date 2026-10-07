mod archive;
mod constants;
mod index;

pub use constants::{ARCHIVE_PATH, INDEX_PATH};
pub use index::seach_index_entries;

#[derive(Debug)]
pub struct IndexEntry {
    pub offset: u64,
    pub page_id: u64,
    pub title: String,
}
