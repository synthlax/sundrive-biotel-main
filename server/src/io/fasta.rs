use memmap2::MmapOptions;
use rayon::prelude::*;
use std::fs::File;

use crate::structs::genetic_schema::IUPACNucleotide;
use crate::structs::sequence_ref::Record;

pub fn read_fasta(path_dir: &str) -> Result<Vec<Record>> {
    let file = File::open(path_dir)?;
    let file_map = unsafe { MmapOptions::new().map(&file)? };
    let data = &file_map;

    Ok()
}

pub fn write_fasta() {}
