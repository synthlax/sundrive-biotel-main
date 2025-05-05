use super::sequence_io::*;
use memmap2::MmapOptions;
use std::{env, fs::File, io::Error};

pub fn read_fasta(path_dir: &str) -> Result<FileData, Error> {
    let mut path = env::current_dir()?;
    path.push(path_dir);
    let file = File::open(path)?;
    let file_map = unsafe { MmapOptions::new().map(&file)? };
    let data: &'static [u8] = unsafe { std::mem::transmute::<&[u8], &'static [u8]>(&file_map) };
    let identifier_positions = get_identifier_positions(data, '>');
    let ranges = get_sequence_ranges(data, identifier_positions);
    let records = get_records(data, ranges, "FASTA");
    Ok(FileData {
        records,
        mmap: file_map,
    })
}

pub fn write_fasta() {}
