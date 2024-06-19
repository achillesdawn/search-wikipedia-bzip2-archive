use std::{io::Read, time::Instant};

use boyer_moore_magiclen::BMByte;

#[derive(Debug)]
struct IndexEntry {
    offset: u64,
    inner_offset: u32,
    title: String,
}

fn parse_entries(data_buffer: &str, needle: &BMByte) {

    let contains = needle.find_first_in(data_buffer);

    if contains.is_none() {
        return;
    }

    for line in data_buffer.lines() {
        let mut split = line.split(":");
        if let (Some(offset), Some(inner_offset), Some(title)) =
            (split.next(), split.next(), split.next())
        {
            if needle.find_first_in(title).is_some() {
                let entry = IndexEntry {
                    offset: offset.parse().unwrap(),
                    inner_offset: inner_offset.parse().unwrap(),
                    title: title.to_owned(),
                };

                dbg!(entry);
            }
        }
    }
}

fn buffer_to_str(buffer: &[u8], needle: &BMByte) {
    if let Ok(s) = std::str::from_utf8(buffer) {
        parse_entries(s, needle);
    }
}

fn main() {
    let file =
        std::fs::File::open("enwiki-20220901-pages-articles-multistream-index.txt.bz2").unwrap();
        
    let mut decompresor = bzip2::read::MultiBzDecoder::new(file);

    const BUFFER_SIZE: usize = 1024*8;

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut last_idx = BUFFER_SIZE;

    let now = Instant::now();

    let needle = boyer_moore_magiclen::BMByte::from("Harmonic ").unwrap();

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

        buffer_to_str(&buffer[..last_idx], &needle);

        buffer.copy_within(last_idx.., 0);
    }

    let elapsed = now.elapsed().as_secs();
    println!("{} seconds elapsed", elapsed);
}
