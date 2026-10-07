use wiki_archive::seach_index_entries;

fn main() {
    tracing_subscriber::fmt().init();

    let archive_base_path =
        std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive");

    let index_path =
        archive_base_path.join("enwiki-20220901-pages-articles-multistream-index.txt.bz2");

    let archive_path = archive_base_path.join("enwiki-20220901-pages-articles-multistream.xml.bz2");

    let entries = seach_index_entries(index_path);

    let entry = entries.get(5).expect("expected at least 6 elements");

    dbg!(entry);
}
