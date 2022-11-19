use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[derive(BinRead, Debug)]
#[allow(dead_code)]
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

fn test(bytes: &Vec<u8>, little_endian: bool) -> u64 {
    match little_endian {
        true => {
            (bytes[0] as u64)
          | (bytes[1] as u64) << 8
          | (bytes[2] as u64) << 16
          | (bytes[3] as u64) << 24
          | (bytes[4] as u64) << 32
        }
        false => {
            (bytes[0] as u64) << 32
          | (bytes[1] as u64) << 24
          | (bytes[2] as u64) << 16
          | (bytes[3] as u64) << 8
          | (bytes[4] as u64)
        }
    }
}

fn test_results(header: &PsarcHeader) {
    /*
    (b'PSAR', 65540, b'zlib', 856, 30, 21, 65536, 4)

    MAGIC       = "PSAR" = header[0]
    VERSION     = 65540  = header[1]
    COMPRESSION = "zlib" = header[2]
    toc_size    = 824    = header[3] - 32
    BLOCK_SIZE  = 65536  = header[4]
    n_entries   = 21     = header[5]
    */

    println!("Version: {}.{}", header.version.major, header.version.minor);
    println!("Compression Type: {}", header.compression_type);
    println!("TOC Length: {}", header.toc_length);
    println!("TOC Entry Size: {}", header.toc_entry_size);
    println!("TOC Entries: {}", header.toc_entries);
    println!("Block Size: {}", header.block_size);
    println!("Archive Flags: {}", header.archive_flags);
    println!("TOC Table MD5 Hash: {}", header.toc_table.md5_hash);
    println!("TOC Table Block Offset: {}", header.toc_table.block_offset);
    println!("TOC Table Uncompressed Size: {:?}", header.toc_table.uncompressed_size);
    println!("TOC Table File Offset: {:?}", header.toc_table.file_offset);
}

fn parse_psarc(path: &std::path::PathBuf) -> PsarcHeader {
    let filename = String::from(path.to_string_lossy());
    println!("Filename: {:?}", filename);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);

    let header = PsarcHeader::read(&mut reader);
    println!("{:?}", header);

    let header = header.unwrap();
    println!("block offset = {:?}", header.toc_table.block_offset);
    println!("uncompressed size = {:?}", test(&header.toc_table.uncompressed_size, true));
    println!("file offset = {:?}", test(&header.toc_table.file_offset, true));

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
