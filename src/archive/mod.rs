use std::io::{Read, Seek, SeekFrom};

use eyre::Context;

use crate::archive::api::Page;
use crate::{ARCHIVE_PATH, IndexEntry};

mod api;

pub fn find_in_archive(index: IndexEntry) -> eyre::Result<Page> {
    let xml_string = archive_read_chunk(index.offset)?;

    let pages = parse_xml_string(xml_string)?;

    for page in pages {
        if page.id == index.page_id {
            return Ok(page);
        }
    }

    eyre::bail!("could not find index entry")
}

fn parse_xml_string(xml_string: String) -> eyre::Result<Vec<Page>> {
    quick_xml::de::from_str(&xml_string).wrap_err("could not parse xml string")
}

fn archive_read_chunk(offset: u64) -> eyre::Result<String> {
    let mut file = std::fs::File::open(ARCHIVE_PATH.clone()).expect("could not open archive path");

    file.seek(SeekFrom::Start(offset))
        .wrap_err("could not seek to offset")?;

    let mut chunk_decoder = bzip2::read::BzDecoder::new(file);

    let mut buf = Vec::new();

    chunk_decoder
        .read_to_end(&mut buf)
        .wrap_err("could not read")?;

    let xml_string = std::str::from_utf8(&buf)
        .wrap_err("could not read utf-8 bytes")?
        .to_string();

    Ok(xml_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_read() {
        let xml_string = archive_read_chunk(583381911).unwrap();

        dbg!(xml_string);
    }

    #[test]
    fn test_archive_parse() {
        tracing_subscriber::fmt().init();

        let xml_string = archive_read_chunk(583381911).unwrap();

        let pages: Vec<Page> = quick_xml::de::from_str(&xml_string).unwrap();

        for page in pages.iter() {
            if page.id == 142488 {
                dbg!(page);
            }
        }
    }
}
