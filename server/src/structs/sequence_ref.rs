use std::borrow::Cow;
#[derive(Debug)]
pub struct Record<'a> {
    identifier: &'a str,
    sequence: Cow<'a, [u8]>,
    quality: &'a [u8],
    base_counts: [usize; 128],
    melting_temp: Option<f64>,
}

impl<'a> Record<'a> {
    pub fn new(identifier: &'a str, sequence: Cow<'a, [u8]>, quality: &'a [u8]) -> Self {
        let base_counts = Self::count_bases(&sequence);
        Self {
            identifier,
            sequence,
            quality,
            base_counts,
            melting_temp: None,
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }

    pub fn get_sequence(&self) -> &str {
        &std::str::from_utf8(&self.sequence).unwrap_or_default()
    }

    pub fn get_quality(&self) -> &str {
        std::str::from_utf8(self.quality).unwrap_or_default()
    }

    fn count_bases(sequence: &[u8]) -> [usize; 128] {
        let mut base_counts = [0usize; 128];
        for &b in std::str::from_utf8(&sequence)
            .unwrap_or_default()
            .as_bytes()
        {
            if b < 128 {
                base_counts[b as usize] += 1
            }
        }
        base_counts
    }

    fn calculate_melting_temperature() -> f64 {
        let melting_temperature = 0.0;

        melting_temperature
    }
}
