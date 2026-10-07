mod needle;

pub use needle::{collect_entries, needle_find_entries};

#[derive(Debug)]
pub struct IndexEntry {
    pub offset: u64,
    pub inner_offset: u32,
    pub title: String,
}
