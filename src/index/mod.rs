use std::{io::Read, time::Instant};

use boyer_moore_magiclen::BMByte;
use tracing::{debug, info};

use crate::{INDEX_PATH, IndexEntry};

pub const BUFFER_SIZE: usize = 1024 * 8;

fn needle_find_entries(needle: &BMByte, data_buffer: &str) -> Vec<IndexEntry> {
    let mut entries = Vec::new();

    if needle.find_first_in(data_buffer).is_none() {
        return entries;
    }

    for line in data_buffer.lines() {
        let mut split = line.split(":");

        if let (Some(offset), Some(inner_offset), Some(title)) =
            (split.next(), split.next(), split.next())
            && needle.find_first_in(title).is_some()
        {
            let entry = IndexEntry {
                offset: offset.parse().unwrap(),
                page_id: inner_offset.parse().unwrap(),
                title: title.to_owned(),
            };

            entries.push(entry);
        }
    }

    entries
}

pub fn seach_index_entries(query: &str) -> Vec<IndexEntry> {
    let file = std::fs::File::open(INDEX_PATH.clone()).expect("could not open archive path");

    let mut decompresor = bzip2::read::MultiBzDecoder::new(file);

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut last_idx = BUFFER_SIZE;

    let now = Instant::now();

    let needle = boyer_moore_magiclen::BMByte::from(query).unwrap();

    let mut entries = Vec::new();

    while let Ok(n) = decompresor.read(&mut buffer[BUFFER_SIZE - last_idx..]) {
        if n == 0 {
            // EOF
            break;
        }

        for (idx, b) in buffer.iter().enumerate().rev() {
            if *b == 10u8 {
                last_idx = idx;

                break;
            }
        }

        if let Ok(s) = std::str::from_utf8(&buffer[..last_idx]) {
            let new_entries = needle_find_entries(&needle, s);

            for entry in new_entries.iter() {
                info!(entry = entry.title);
            }

            entries.extend(new_entries);
        }

        buffer.copy_within(last_idx.., 0);
    }

    let elapsed = now.elapsed().as_secs();

    debug!("{} seconds elapsed", elapsed);

    entries
}
