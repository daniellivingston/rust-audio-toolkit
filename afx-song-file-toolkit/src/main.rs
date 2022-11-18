use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(BinRead, Debug)]
struct Version {
    major: u16,
    minor: u16
}

#[derive(BinRead, Debug)]
#[allow(dead_code)]
struct TocTable {
    md5_hash: u128,
    block_offset: u32,
    #[br(count = 5)]
    uncompressed_size: Vec<u8>,
    #[br(count = 5)]
    file_offset: Vec<u8>
}

// https://www.psdevwiki.com/ps3/PlayStation_archive_(PSARC)
// https://github.com/GaticusHax/libPSARC/blob/development/libPSARC-Static/Source/PSARC/Header.cs
// https://docs.rs/binrw/latest/binrw/
// https://jam1.re/blog/binread-a-declarative-rust-binary-parsing-library
#[derive(BinRead, Debug)]
#[br(big, magic = b"PSAR")]
#[allow(dead_code)]
struct PsarcHeader {
    version: Version,

    #[br(count = 4, map = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned())]
    compression_type: String,

    toc_length: u32,
    toc_entry_size: u32,
    toc_entries: u32,
    block_size: u32,
    archive_flags: u32,

    toc_table: TocTable
}

fn parse_psarc(path: &std::path::PathBuf) -> PsarcHeader {
    let filename = String::from(path.to_string_lossy());
    println!("Filename: {:?}", filename);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);

    let header = PsarcHeader::read(&mut reader);
    println!("{:?}", header);

    let header = header.unwrap();

    return header;
}

fn main() -> std::io::Result<()> {
    // let args = Cli::parse(); // Implement later

    let mut path = std::env::current_dir().unwrap();
    [
        "afx-song-file-toolkit",
        "bin/psarc/dlc/",
        "karmapolice_m.psarc"
    ].map(|x| path.push(x));

    let _ = parse_psarc(&path);

    Ok(())
}
