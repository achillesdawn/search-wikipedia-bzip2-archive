use wiki_archive::seach_index_entries;

fn main() {
    tracing_subscriber::fmt().init();

    let entries = seach_index_entries("Harmonic ");

    let entry = entries.get(5).expect("expected at least 6 elements");

    dbg!(entry);
}
