use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use eyre::Context;

pub fn archive_read(archive_path: PathBuf, offset: u64) -> eyre::Result<String> {
    let mut file = std::fs::File::open(archive_path).expect("could not open archive path");

    file.seek(SeekFrom::Start(offset))
        .wrap_err("could not seek to offset")?;

    let mut chunk_decoder = bzip2::read::BzDecoder::new(file);

    let mut buf = Vec::new();

    chunk_decoder
        .read_to_end(&mut buf)
        .wrap_err("could not read")?;

    let s = std::str::from_utf8(&buf)
        .wrap_err("could not read utf-8 bytes")?
        .to_string();

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_read() {
        // [src/main.rs:15:5] entry = IndexEntry {
        //     offset: 583381911,
        //     inner_offset: 142488,
        //     title: "Harmonic series (mathematics)",
        // }

        let archive_base_path =
            std::path::PathBuf::from("/run/media/miguel/Blue/backup/novaera/rust/wiki_archive");

        let archive_path =
            archive_base_path.join("enwiki-20220901-pages-articles-multistream.xml.bz2");

        let s = archive_read(archive_path, 583381911).unwrap();

        dbg!(s);
    }
}
