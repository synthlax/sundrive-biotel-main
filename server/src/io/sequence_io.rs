use memmap2::Mmap;
use rayon::prelude::*;

pub fn get_identifier_positions(data: &[u8], symbol: char) -> Vec<usize> {
    let mut identifier_positions = Vec::new();
    for (i, &b) in data.iter().enumerate() {
        if b == symbol as u8 && (i == 0 || data[i - 1] == b'\n') {
            identifier_positions.push(i);
        }
    }
    identifier_positions
}

pub fn get_sequence_ranges(data: &[u8], identifier_positions: Vec<usize>) -> Vec<(usize, usize)> {
    identifier_positions
        .iter()
        .enumerate()
        .map(|(i, &start)| {
            let end = if i < identifier_positions.len() - 1 {
                identifier_positions[i + 1]
            } else {
                data.len()
            };
            (start, end)
        })
        .collect()
}

pub fn find_newline(data: &[u8]) -> usize {
    let mut newline_pos = 0;
    for (i, &b) in data.iter().enumerate() {
        if b == b'\n' {
            newline_pos = i;
            break;
        }
    }
    newline_pos
}

pub fn remove_newlines(data: &[u8]) -> Vec<u8> {
    let mut new_data = Vec::new();
    for &b in data {
        if b != b'\n' {
            new_data.push(b);
        }
    }
    new_data
}

pub fn get_sequence<'a>(record_chunk: &'a [u8], format_type: &str) -> Option<(&'a [u8], usize)> {
    match format_type {
        "FASTA" => Some((&record_chunk[find_newline(record_chunk) + 1..], record_chunk.len())),
        "PLAIN" => Some((&record_chunk[find_newline(record_chunk) + 1..], record_chunk.len())),
        "FASTQ" => {
            let mut symbol_position = find_newline(record_chunk) + 1;
            while symbol_position < record_chunk.len() {
                if record_chunk[symbol_position] == b'+' {
                    break;
                }

                if let Some(newline) = record_chunk[symbol_position..]
                    .iter()
                    .position(|&b| b == b'\n')
                {
                    symbol_position += newline + 1;
                };
            }
            Some((&record_chunk[find_newline(record_chunk) + 1..symbol_position - 2], symbol_position))
        }
        _ => None
    }
}

pub fn get_records<'a>(data: &[u8], ranges: Vec<(usize, usize)>, format_type: &'a str) -> Vec<(String, String, String)> {
    ranges
        .par_iter()
        .map(|&(start, end)| {
            let record_chunk = &data[start..end];
            let identifier = &record_chunk[1..find_newline(record_chunk)];
            let sequence = &remove_newlines(get_sequence(record_chunk, format_type).unwrap().0)[..];
            let quality = &record_chunk[get_sequence(record_chunk, format_type).unwrap().1..];
            (String::from_utf8_lossy(identifier).to_string(), String::from_utf8_lossy(sequence).to_string(), String::from_utf8_lossy(quality).to_string())
        })
        .collect()
}
