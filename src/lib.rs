mod archive;
mod index;

use std::{path::PathBuf, sync::LazyLock};

pub use index::seach_index_entries;

pub const BUFFER_SIZE: usize = 1024 * 8;

pub static ARCHIVE_BASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive")
});

pub static ARCHIVE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| ARCHIVE_BASE_PATH.join("enwiki-20220901-pages-articles-multistream.xml.bz2"));

#[derive(Debug)]
pub struct IndexEntry {
    pub offset: u64,
    pub page_id: u64,
    pub title: String,
}
