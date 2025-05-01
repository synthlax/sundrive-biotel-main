use memmap2::{Mmap, MmapOptions};
use rayon::prelude::*;
use super::sequence_io::*;
use std::{clone, collections::HashMap, fs::File, io::Error};

use crate::structs::sequence_ref::Record;

pub fn read_fasta(path_dir: &str) -> Result<Vec<Record>, Error> {
    let file = File::open(path_dir)?;
    let file_map = unsafe { MmapOptions::new().map(&file)? };
    let data = file_map;
    let identifier_positions = get_identifier_positions(&data, '>');
    let ranges = get_sequence_ranges(&data, identifier_positions);
    let records = get_records(&data, ranges, "FASTA");
    let mut new_records = Vec::new();
    for record in records {
        new_records.push(Record::new(record.0, record.1, record.2, HashMap::new(), 0.0));
    }
    Ok(new_records)
}

pub fn write_fasta() {}
