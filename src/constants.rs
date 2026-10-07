use std::{path::PathBuf, sync::LazyLock};

pub static ARCHIVE_BASE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive")
});

pub static ARCHIVE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| ARCHIVE_BASE_PATH.join("enwiki-20220901-pages-articles-multistream.xml.bz2"));

pub static INDEX_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    ARCHIVE_BASE_PATH.join("enwiki-20220901-pages-articles-multistream-index.txt.bz2")
});
