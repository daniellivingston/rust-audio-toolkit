use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::BufReader;

// use aes::cipher::{AsyncStreamCipher, KeyIvInit};
// https://github.com/RustCrypto/block-modes/blob/master/cfb-mode/Cargo.toml
// type Aes128CfbDec = cfb_mode::Decryptor<aes::Aes128>;

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
    // Aes128CfbDec::new(&key.into(), &iv.into()).decrypt(&mut buf)
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

fn pad_zeroes<const A: usize, const B: usize>(arr: [u8; A]) -> [u8; B] {
    assert!(B >= A); //just for a nicer error message, adding #[track_caller] to the function may also be desirable
    let mut b = [0; B];
    b[..A].copy_from_slice(&arr);
    b
}

fn test_aes(buf: &mut Vec<u8>) {
    use hex_literal::hex;
    use aes::Aes128;
    use aes::cipher::KeyIvInit;
    use aes::cipher::AsyncStreamCipher;
    use cfb_mode::Decryptor;

    type Aes128CfbDec = Decryptor<Aes128>;

    let key = hex!("C53DB23870A1A2F71CAE64061FDD0E1157309DC85204D4C5BFDF25090DF2572C");
    let iv = hex!("E915AA018FEF71FC508132E4BB4CEB42");

    // let key = [0x42; 16];
    // let iv = [0x24; 16];

    Aes128CfbDec::new(&key.into(), &iv.into()).decrypt(&mut buf);
    // cipher_toc = AES.new(
    //   codecs.decode(ARC_KEY,'hex'),  # secret key (16 bytes)
    //   mode=AES.MODE_CFB,             # MODE_{EAX,CBC,CFB,OFB,OPENPGP}
    //   IV=codecs.decode(ARC_IV,'hex'),# initialization vector (16 bytes)
    //   segment_size=128)              # The number of bits the plaintext and 
    //                                  # ciphertext are segmented in. 
    //                                  # It must be a multiple of 8.
    //
    // let data = reader.read_bytes(toc_size);
    // let data = data.pad();
    // let decryption = cipher_toc.decrypt(&data);
}

fn parse_psarc(path: &std::path::PathBuf) -> PsarcHeader {
    let filename = String::from(path.to_string_lossy());
    println!("Filename: {:?}", filename);

    let f = File::open(&path).unwrap();
    let mut reader = BufReader::new(f);

    let header = PsarcHeader::read(&mut reader).unwrap();
    println!("{:?}", header);
    test_results(&header);
    //test_aes(&mut reader);
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
