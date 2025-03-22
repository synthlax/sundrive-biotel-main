use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader, Result}
};
use crate::structs::genetic_schema::IUPACNucleotide;
use crate::structs::sequence_ref::SequenceRef;

pub fn read_fasta(path_dir: &str) -> Result<Vec<SequenceRef>> {
    let file: File = File::open(path_dir)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut records = vec![];
    let mut last_name = String::new();
    let mut last_seq= String::new();
    let mut last_nucleo_type = String::new();
    let mut last_melt_temp = 0.0;
    let mut last_nucleo_content: HashMap<char, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('>') {
            if !last_name.is_empty() {
                records.push(SequenceRef::new(last_name, last_seq,last_nucleo_type, last_nucleo_content, last_melt_temp));
                last_name = String::new();
                last_seq = String::new();
                last_nucleo_type = String::new();
                last_melt_temp = 0.0;
                last_nucleo_content = HashMap::new();
            }
            else{
                last_name = line[1..].to_string();
            }
        } else {
            last_seq.push_str(&line);
            last_nucleo_type = IUPACNucleotide::get_nucleotide_type(&line).unwrap_or("INVALID".to_string());
        }
    }
    Ok(records)
}
