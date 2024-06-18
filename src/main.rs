use std::io::Read;

#[derive(Debug)]
struct IndexEntry {
    offset: u64,
    inner_offset: u32,
    title: String,
}

fn parse_entries(data_buffer: &str) {
    for line in data_buffer.lines() {
        let mut split = line.split(":");
        if let (Some(offset), Some(inner_offset), Some(title)) =
            (split.next(), split.next(), split.next())
        {
            let entry = IndexEntry {
                offset: offset.parse().unwrap(),
                inner_offset: inner_offset.parse().unwrap(),
                title: title.to_owned(),
            };
            if entry.title.contains("Rust") {
                dbg!(entry);
            }
        }
    }
}

fn buffer_to_str(buffer: &[u8]) {
    let s = std::str::from_utf8(buffer).unwrap();
    dbg!(s);
}

fn main() {
    let file =
        std::fs::File::open("enwiki-20220901-pages-articles-multistream-index.txt.bz2").unwrap();
    let mut decompresor = bzip2::read::MultiBzDecoder::new(file);

    const BUFFER_SIZE: usize = 1000;

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut iteration = 0usize;
    let mut last_idx = BUFFER_SIZE;

    while let Ok(n) = decompresor.read(&mut buffer[BUFFER_SIZE - last_idx..]) {
        if n == 0 {
            println!("End of file");
            break;
        }

        for (idx, b) in buffer.iter().enumerate().rev() {
            if *b == 10u8 {
                last_idx = idx;
                break;
            }
        }

        buffer_to_str(&buffer[..last_idx]);

        buffer.copy_within(last_idx.., 0);


        iteration += 1;
        if iteration == 3 {
            break;
        }
    }
}
