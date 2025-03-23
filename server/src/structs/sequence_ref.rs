use std::collections::HashMap;

#[derive(Debug)]
pub struct Record {
    identifier: Option<String>,
    sequence: Option<String>,
    quality: Option<String>,
    base_contents: Result<HashMap<char, i32>, String>,
    melting_temperature: Result<f64, String>,
}

impl Record {
    pub fn new(identifier: String, sequence: String, quality: String,base_contents: HashMap<char, i32>, melting_temperature: f64) -> Record {
        Record {
            identifier: Some(identifier),
            sequence: Some(sequence),
            quality: Some(quality),
            base_contents: Ok(base_contents),
            melting_temperature: Ok(melting_temperature),
        }
    }

    pub fn get_identifier(&self) -> &String {
        let identifier = &self.identifier;
        identifier.as_ref().unwrap()
    }

    pub fn get_sequence(&self) -> &String {
        let sequence = &self.sequence;
        sequence.as_ref().unwrap()
    }

    pub fn get_quality(&self) -> &String {
        let quality = &self.quality;
        quality.as_ref().unwrap()
    }

    pub fn get_base_contents(&self) -> &HashMap<char, i32> {
        let base_contents = &self.base_contents;
        base_contents.as_ref().unwrap()
    }
    pub fn get_melting_temperature(&self) -> &f64 {
        let melting_temperature = &self.melting_temperature;
        melting_temperature.as_ref().unwrap()
    }

    fn calculate_melting_temperature(&self) -> f64 {
        let base_contents = self.get_base_contents();
        let mut melting_temperature = 0.0;

        melting_temperature
    }

}