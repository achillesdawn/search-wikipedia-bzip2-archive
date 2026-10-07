use wiki_archive::collect_entries;

fn main() {
    tracing_subscriber::fmt().init();

    let archive_path =
        std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive");

    let index_path = archive_path.join("enwiki-20220901-pages-articles-multistream-index.txt.bz2");

    let entries = collect_entries(index_path);

    let entry = entries.get(5).expect("expected at least 6 elements");

    dbg!(entry);
}
