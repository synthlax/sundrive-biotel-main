use crate::structs::genetic_schema::IUPACNucleotide;
use crate::structs::sequence_ref::Record;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn read_fasta(path_dir: &str) -> Result<Vec<Record>> {
    let file= File::open(path_dir)?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();
    let mut last_identifier = String::new();
    let mut last_seq = String::new();
    let mut last_nucleo_type = String::new();
    let mut last_melt_temp = 0.0;
    let mut last_nucleo_content: HashMap<char, i32> = IUPACNucleotide::get_bases_map();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('>') {
            if !last_identifier.is_empty() {
                records.push(Record::new(
                    last_identifier,
                    last_seq,
                    last_nucleo_type,
                    last_nucleo_content,
                    last_melt_temp
                ));
                last_identifier = String::new();
                last_seq = String::new();
                last_nucleo_type = String::new();
                last_nucleo_content = HashMap::new();
                last_melt_temp = 0.0;
            } 
            else {
                last_identifier = line.strip_prefix('>').unwrap().to_string().to_owned();
            }
        } else {
            last_seq.push_str(&line);
            last_nucleo_content = IUPACNucleotide::count_nucleotides(&line).unwrap();
            last_nucleo_type = IUPACNucleotide::get_nucleotide_type(&line).unwrap_or("INVALID".to_string());
        }
    }

    if !last_identifier.is_empty() {
        records.push(Record::new(
            last_identifier,
            last_seq,
            last_nucleo_type,
            last_nucleo_content,
            last_melt_temp
        ));
    }

    Ok(records)
}

pub fn write_fasta() {

}
