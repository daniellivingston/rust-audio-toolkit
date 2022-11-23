use clap::Parser;
use binrw::BinRead;

use std::fs::File;
use std::io::{Read, BufReader};

use hex_literal::hex;
use aes::Aes256;
use cfb_mode::Decryptor;
use aes::cipher::{AsyncStreamCipher, KeyIvInit};
use binrw::io::Cursor;

static ARC_KEY: [u8; 32] = [0xC5, 0x3D, 0xB2, 0x38, 0x70, 0xA1, 0xA2, 0xF7, 0x1C, 0xAE, 0x64, 0x06, 0x1F, 0xDD, 0x0E, 0x11, 0x57, 0x30, 0x9D, 0xC8, 0x52, 0x04, 0xD4, 0xC5, 0xBF, 0xDF, 0x25, 0x09, 0x0D, 0xF2, 0x57, 0x2C];
static ARC_IV:  [u8; 16] = [0xE9, 0x15, 0xAA, 0x01, 0x8F, 0xEF, 0x71, 0xFC, 0x50, 0x81, 0x32, 0xE4, 0xBB, 0x4C, 0xEB, 0x42];

//let MAC_KEY = 0x9821330E34B91F70D0A48CBD625993126970CEA09192C0E6CDA676CC9838289D;
//let WIN_KEY = 0xCB648DF3D12A16BF71701414E69619EC171CCA5D2A142E3E59DE7ADDA18A3A30;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn pad_zeroes<const A: usize, const B: usize>(arr: [u8; A]) -> [u8; B] {
    assert!(B >= A); //just for a nicer error message, adding #[track_caller] to the function may also be desirable
    let mut b = [0; B];
    b[..A].copy_from_slice(&arr);
    b
}

fn u40_from_vec(bytes: &Vec<u8>, little_endian: bool) -> u64 {
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
#[br(big)]
#[allow(dead_code)]
struct TocTable {
    md5_hash: u128,
    block_offset: u32,
    #[br(count = 5, map = |bytes: Vec<u8>| u40_from_vec(&bytes, false))]
    uncompressed_size: u64,
    #[br(count = 5, map = |bytes: Vec<u8>| u40_from_vec(&bytes, false))]
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
}

#[derive(Debug)]
#[allow(dead_code)]
struct PlaystationArchive {
    header: PsarcHeader,
    toc: Vec<TocTable>
}

impl TocTable {
    pub fn from_bytes(bytes: Vec<u8>) -> TocTable {
        let mut bytes = bytes.clone();

        Decryptor::<Aes256>::new_from_slices(&ARC_KEY, &ARC_IV)
                           .expect("Invalid key or iv length")
                           .decrypt(&mut bytes);

        let mut buff: Cursor<Vec<u8>> = Cursor::new(bytes);

        TocTable::read(&mut buff).unwrap()
    }
}

impl PlaystationArchive {
    pub fn read(path: &std::path::Path) -> Self {
        let f = File::open(&path).expect("Failed to open file");
        let mut reader = BufReader::new(f);

        let header = PsarcHeader::read(&mut reader).expect("Failed to read PSARC header");

        let mut toc = Vec::<TocTable>::new();

        let chunk_size = header.toc_length as u64;
        let num_chunks = header.toc_entries as u64;
        let mut bytes: Vec<u8> = vec![];

        reader
            .take(chunk_size * num_chunks)
            .read_to_end(&mut bytes)
            .expect("Failed to read file");

        for i in 0..num_chunks {
            let x0 = (i * chunk_size) as usize;
            let x1 = ((i + 1) * chunk_size) as usize;

            let table = TocTable::from_bytes(bytes[x0..x1].to_vec());
            toc.push(table);
        }

        PlaystationArchive {
            header,
            toc
        }
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

    println!("Reading PSARC file: {:?}", path);

    let psarc = PlaystationArchive::read(&path);
    println!("{:?}", psarc);

    Ok(())
}
