use std::{
    fs::File,
    io::{BufReader, Read},
    time::Instant,
};

use memchr::{memmem::Finder, memrchr};
use tracing::{debug, info};

use crate::{INDEX_PATH, IndexEntry};

pub const BUFFER_SIZE: usize = 1024 * 256;

fn needle_find_entries(finder: &Finder, chunk: &[u8]) -> Vec<IndexEntry> {
    let mut entries = Vec::new();

    if finder.find(chunk).is_none() {
        return entries;
    }

    for line in chunk.split(|&b| b == b'\n') {
        let mut split = line.split(|&b| b == b':');

        if let (Some(offset_raw), Some(id_raw), Some(title_raw)) =
            (split.next(), split.next(), split.next())
            && finder.find(title_raw).is_some()
            && let (Ok(offset_str), Ok(id_str), Ok(title_str)) = (
                std::str::from_utf8(offset_raw),
                std::str::from_utf8(id_raw),
                std::str::from_utf8(title_raw),
            )
            && let (Ok(offset), Ok(page_id)) = (offset_str.parse(), id_str.parse())
        {
            entries.push(IndexEntry {
                offset,
                page_id,
                title: title_str.to_owned(),
            });
        }
    }

    entries
}

fn read_index<R: Read>(
    mut decompressor: bzip2::read::MultiBzDecoder<R>,
    finder: &Finder,
    limit: usize,
) -> Vec<IndexEntry> {
    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut last_idx = 0;

    let mut entries = Vec::new();

    while let Ok(n) = decompressor.read(&mut buffer[last_idx..]) {
        if n == 0 {
            if last_idx > 0 {
                entries.extend(needle_find_entries(finder, &buffer[..last_idx]));
            }

            break;
        }

        last_idx += n;

        // break on \n
        if let Some(last_nl) = memrchr(b'\n', &buffer[..last_idx]) {
            let complete_slice = &buffer[..last_nl];

            let new_entries = needle_find_entries(finder, complete_slice);

            for entry in &new_entries {
                info!(entry = entry.title);
            }

            entries.extend(new_entries);

            if entries.len() >= limit {
                break;
            }

            let remainder_len = last_idx - (last_nl + 1);

            buffer.copy_within(last_nl + 1..last_idx, 0);

            last_idx = remainder_len;
        }
    }

    entries
}

pub fn search_all_index_entries(query: &str, limit: usize) -> Vec<IndexEntry> {
    let file = File::open(INDEX_PATH.clone()).expect("could not open archive path");

    let buf_reader = BufReader::with_capacity(BUFFER_SIZE, file);

    let decompressor = bzip2::read::MultiBzDecoder::new(buf_reader);

    let now = Instant::now();

    let finder = Finder::new(query.as_bytes());

    let entries = read_index(decompressor, &finder, limit);

    let elapsed = now.elapsed().as_secs();

    debug!("{} seconds elapsed", elapsed);

    entries
}
