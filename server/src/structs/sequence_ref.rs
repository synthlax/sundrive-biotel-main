use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SequenceRef {
    sequence_name: Option<String>,
    nucleotide_type: Option<String>,
    sequence: Option<String>,
    nucleotide_content: Option<HashMap<char, i32>>,
    melting_temperature: Option<f64>
}

impl SequenceRef {
    pub fn new(seq_name: String, seq: String, nucleo_type: String, nucleo_content: HashMap<char, i32>, melting_temp: f64) -> SequenceRef {
        SequenceRef {
            sequence_name: Some(seq_name),
            nucleotide_type: Some(nucleo_type),
            sequence: Some(seq),
            nucleotide_content: Some(nucleo_content),
            melting_temperature: Some(melting_temp),
        }
    }

    pub fn get_record_name(&self) -> &String {
        &self.sequence_name.as_ref().unwrap()
    }
    
    pub fn get_sequence(&self) -> &String {
        &self.sequence.as_ref().unwrap()
    }

    pub fn get_content(&self) -> &HashMap<char, i32> {
        &self.nucleotide_content.as_ref().unwrap()
    }
}
