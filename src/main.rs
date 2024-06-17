use std::io::Read;

#[derive(Debug)]
struct IndexEntry {
    offset: u64,
    inner_offset: u16,
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
            dbg!(entry);
        }
    }
}

fn main() {
    let file =
        std::fs::File::open("enwiki-20220901-pages-articles-multistream-index.txt.bz2").unwrap();
    let mut decompresor = bzip2::read::MultiBzDecoder::new(file);

    let mut buffer = [0u8; 1000];
    let mut data_buffer = String::with_capacity(1000);
    let mut iteration = 0usize;

    while let Ok(n) = decompresor.read(&mut buffer) {
        if n == 0 {
            println!("End of file");
            break;
        }

        let mut last_idx = 0usize;

        for (idx, b) in buffer.iter().enumerate().rev() {
            if *b == 10u8 {
                last_idx = idx;
                break;
            }
        }

        data_buffer.push_str(std::str::from_utf8(&buffer[..last_idx + 1]).unwrap());
        parse_entries(&data_buffer);

        data_buffer.clear();
        data_buffer.push_str(std::str::from_utf8(&buffer[last_idx + 1..]).unwrap());

        iteration += 1;
        if iteration == 3 {
            break;
        }
    }
}
