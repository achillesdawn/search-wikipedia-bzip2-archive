use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use eyre::Context;
use tracing::info;

use crate::archive::api::Page;

mod api;

pub fn archive_read(archive_path: PathBuf, offset: u64) -> eyre::Result<String> {
    let mut file = std::fs::File::open(archive_path).expect("could not open archive path");

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
        let archive_base_path =
            std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive");

        let archive_path =
            archive_base_path.join("enwiki-20220901-pages-articles-multistream.xml.bz2");

        let xml_string = archive_read(archive_path, 583381911).unwrap();

        dbg!(xml_string);
    }

    #[test]
    fn test_archive_parse() {
        tracing_subscriber::fmt().init();

        let archive_base_path =
            std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive");

        let archive_path =
            archive_base_path.join("enwiki-20220901-pages-articles-multistream.xml.bz2");

        let xml_string = archive_read(archive_path, 583381911).unwrap();

        let pages: Vec<Page> = quick_xml::de::from_str(&xml_string).unwrap();

        for page in pages.iter() {
            if page.id == 142488 {
                dbg!(page);
            }
        }
    }
}
