use std::borrow::Cow;

const REVERSE_COMPLEMENT_TABLE: [u8; 256] = {
    let mut table = [b'N'; 256];
    table[b'A' as usize] = b'T';
    table[b'T' as usize] = b'A';
    table[b'G' as usize] = b'C';
    table[b'C' as usize] = b'G';
    table[b'R' as usize] = b'Y';
    table[b'Y' as usize] = b'R';
    table[b'S' as usize] = b'S';
    table[b'W' as usize] = b'W';
    table[b'K' as usize] = b'M';
    table[b'M' as usize] = b'K';
    table[b'B' as usize] = b'V';
    table[b'D' as usize] = b'H';
    table[b'H' as usize] = b'D';
    table[b'V' as usize] = b'B';
    table[b'N' as usize] = b'N';

    // Also fill lowercase equivalents
    table[b'a' as usize] = b't';
    table[b't' as usize] = b'a';
    table[b'g' as usize] = b'c';
    table[b'c' as usize] = b'g';
    table[b'r' as usize] = b'y';
    table[b'y' as usize] = b'r';
    table[b's' as usize] = b's';
    table[b'w' as usize] = b'w';
    table[b'k' as usize] = b'm';
    table[b'm' as usize] = b'k';
    table[b'b' as usize] = b'v';
    table[b'd' as usize] = b'h';
    table[b'h' as usize] = b'd';
    table[b'v' as usize] = b'b';
    table[b'n' as usize] = b'n';

    table
};

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

    pub fn get_sequence(&self) -> &[u8] {
        &self.sequence
    }

    pub fn get_identifier(&self) -> &str {
        self.identifier
    }

    pub fn get_quality(&self) -> &[u8] {
        self.quality
    }

    pub fn reverse_complement_sequence(&self) -> Vec<u8> {
        self.sequence
            .iter()
            .rev()
            .map(|&base| REVERSE_COMPLEMENT_TABLE[base as usize])
            .collect()
    }

    pub fn reverse_sequence(&self) -> Vec<u8> {
        self.sequence.iter().rev().copied().collect()
    }

    pub fn complement_sequence(&self) -> Vec<u8> {
        self.sequence
            .iter()
            .map(|&base| REVERSE_COMPLEMENT_TABLE[base as usize])
            .collect()
    }

    fn count_bases(sequence: &[u8]) -> [usize; 128] {
        let mut base_counts = [0usize; 128];
        for &b in std::str::from_utf8(sequence).unwrap_or_default().as_bytes() {
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

pub fn sequence_to_str(sequence: &[u8]) -> &str {
    std::str::from_utf8(sequence).unwrap_or_default()
}

pub fn quality_to_str(quality: &[u8]) -> &str {
    std::str::from_utf8(quality).unwrap_or_default()
}
