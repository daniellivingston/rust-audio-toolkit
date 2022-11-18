use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(BinRead, Debug)]
struct Version(u16, u16);

// https://www.psdevwiki.com/ps3/PlayStation_archive_(PSARC)
// https://github.com/GaticusHax/libPSARC/blob/development/libPSARC-Static/Source/PSARC/Header.cs
// https://docs.rs/binrw/latest/binrw/
// https://jam1.re/blog/binread-a-declarative-rust-binary-parsing-library
#[derive(BinRead, Debug)]
#[br(big, magic = b"PSAR")]
struct PsarcHeader {
    version: Version,
    #[br(count = 4)]
    compression: Vec<u8>,
    toc_length: u32,
    toc_entry_size: u32,
    toc_entries: u32,
    block_size: u32,
    archive_flags: u32
}

struct PsarcFile {
    filename: String,
}

// Read a 4 byte unsigned integer in big endian format
fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

// Read a 4 byte unsigned integer in little endian format
fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

fn parse_psarc(path: &std::path::PathBuf) -> PsarcFile {
    let filename = String::from(path.to_string_lossy());
    println!("Filename: {:?}", filename);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);

    let header = PsarcHeader::read(&mut reader);
    println!("{:?}", header);

    let header = header.unwrap();

    let compression = String::from_utf8_lossy(&header.compression);
    println!("Compression: {:?}", compression);

    PsarcFile {
        filename: String::from(path.to_string_lossy())
    }
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
