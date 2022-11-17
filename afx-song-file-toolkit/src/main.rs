use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

// https://www.psdevwiki.com/ps3/PlayStation_archive_(PSARC)
// https://github.com/GaticusHax/libPSARC/blob/development/libPSARC-Static/Source/PSARC/Header.cs
// https://docs.rs/binrw/latest/binrw/
// https://jam1.re/blog/binread-a-declarative-rust-binary-parsing-library
#[derive(BinRead, Debug)]
#[br(little, magic = b"PSARC")]
struct PsarcHeader {

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
    let buffer = std::fs::read(&path).unwrap();

    let identifier = std::str::from_utf8(&buffer[0..4]).unwrap();
    let version = as_u32_be(&buffer[4..8].try_into().unwrap());
    let compression = std::str::from_utf8(&buffer[8..12]).unwrap();

    let toc_size = as_u32_be(&buffer[12..16].try_into().unwrap());
    let toc_entry_size = as_u32_be(&buffer[16..20].try_into().unwrap());;
    let entry_count = as_u32_be(&buffer[20..24].try_into().unwrap());;
    let block_size = as_u32_be(&buffer[24..28].try_into().unwrap());;
    let archive_flags = as_u32_be(&buffer[28..32].try_into().unwrap());;

    println!("Filename: {:?}", filename);
    println!("Identifier: {:?}", identifier);
    println!("Version: {}", version);
    println!("Compression: {:?}", compression);
    println!("TOC Size: {}", toc_size);
    println!("TOC Entry Size: {}", toc_entry_size);
    println!("Entry Count: {}", entry_count);
    println!("Block Size: {}", block_size);
    println!("Archive Flags: {}", archive_flags);

    // Next up: implement primary contents reading; decryption; flags
    // https://github.com/kokolihapihvi/Rocksmith2014PsarcLib/blob/06a078a872b7d9d8c0b7d0c58ca76543fdc095d4/Psarc/PsarcTOC.cs#L8

    PsarcFile {
        filename: String::from(path.to_string_lossy())
    }
}

fn main() -> std::io::Result<()> {
    // let args = Cli::parse(); // Implement later
    let path: std::path::PathBuf = [
        "/Users/livingston/dev/playground/realtime-audio-rs",
        "afx-song-file-toolkit",
        "bin/psarc/dlc/",
        "karmapolice_m.psarc"
    ].iter().collect();

    let _ = parse_psarc(&path);

    Ok(())
}
