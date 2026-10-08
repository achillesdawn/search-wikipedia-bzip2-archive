use tracing::Level;
use wiki_archive::{find_in_archive, search_all_index_entries};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let entries = search_all_index_entries("Harmonic series", 10);

    let entry = entries.get(0).expect("expected at least 1 elements");

    let page = find_in_archive(entry);

    dbg!(page);
}
