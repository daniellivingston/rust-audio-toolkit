# AFX Song File Toolkit

This Crate is an attempt to parse various music-based audio input formats.

## Milestones

### v0.1.0

- [ ] Implement Rocksmith `.psarc` file format parser
  - [x] Read PSARC header
  - [ ] Read PSARC TOC Table
    - [x] Implement AES-CFB decryption
    - [x] Handle decrypted bytes -> ToC Table
    - [ ] Validate decryption data integrity
  - [ ] Read manifest file
    - [ ] Implement a `read_entry` function like the Python script has
    - [ ] Bring in ZLIB and connect it to `BufReader`

## References & Resources

- [PSARC PS3 EXTRACTOR](https://github.com/AlexAltea/psarc-tool)
- [Converting Guitar Pro to Rocksmith CDLC](https://jamesprestonblog.wordpress.com/2017/12/03/creating-a-custom-from-a-guitar-pro-file/)
- [RocksmithToolkitLib](https://github.com/rscustom/rocksmith-custom-song-toolkit/tree/master/RocksmithToolkitLib)
