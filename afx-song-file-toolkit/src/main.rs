use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::BufReader;

use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use hex_literal::hex;

// https://github.com/RustCrypto/block-modes/blob/master/cfb-mode/Cargo.toml
type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
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
    #[br(count = 5, map = |bytes: Vec<u8>| test(&bytes, false))]
    uncompressed_size: u64,
    #[br(count = 5, map = |bytes: Vec<u8>| test(&bytes, false))]
    file_offset: u64
}

#[derive(BinRead, Debug)]
#[br(big, magic = b"PSAR")]
#[allow(dead_code)]
struct PsarcHeader {
    version: Version,

    #[br(count = 4, map = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned())]
    compression_type: String,

    #[br(map = |x: u32| x - 32)]
    toc_length: u32,
    toc_entry_size: u32,
    toc_entries: u32,
    block_size: u32,
    archive_flags: u32,

    toc_table: TocTable
}

fn aes_cfb() {
    let key = 0x12;
    let iv  = 0x12;
    Aes128CfbDec::new(&key.into(), &iv.into()).decrypt(&mut buf)
}

fn test_results(header: &PsarcHeader) {
    println!("Version: {}.{} == 65540", header.version.major, header.version.minor);
    println!("Compression Type: {} == 'zlib'", header.compression_type);
    println!("TOC Length: {} == 824 (856)", header.toc_length);
    println!("TOC Entry Size: {} == 30", header.toc_entry_size);
    println!("TOC Entries: {} == 21", header.toc_entries);
    println!("Block Size: {} == 65536", header.block_size);
    println!("Archive Flags: {} == 4", header.archive_flags);

    // toc table: should be an iterable
    // all data below here is corrupted and must be uncompressed
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

    let header = PsarcHeader::read(&mut reader).unwrap();
    println!("{:?}", header);
    test_results(&header);
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
