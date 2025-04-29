use memmap2::Mmap;
use rayon::prelude::*;

pub fn get_identifier_positions(data: Mmap, symbol: char) -> Vec<usize> {
    let mut identifier_positions = Vec::new();
    for (i, &b) in data.iter().enumerate() {
        if b == symbol as u8 && (i == 0 || data[i - 1] == b'\n') {
            identifier_positions.push(i);
        }
    }
    identifier_positions
}

pub fn get_sequence_ranges(data: Mmap, identifier_positions: Vec<usize>) -> Vec<(usize, usize)> {
    let ranges: Vec<_> = identifier_positions
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
        .collect();
    ranges
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
    let mut new_data: Vec<u8> = Vec::with_capacity(data.len());
    for &b in data {
        if b != b'\n' {
            new_data.push(b);
        }
    }
    new_data
}

pub fn get_sequence(record_chunk: &'static [u8], format_type: &str) -> Option<&'static [u8]> {
    match format_type {
        "FASTA" => Some(&record_chunk[find_newline(record_chunk) + 1..]),
        "PLAIN" => Some(&record_chunk[find_newline(record_chunk) + 1..]),
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
            Some(&record_chunk[find_newline(record_chunk) + 1..symbol_position - 1])
        }
        _ => None,
    }
}

pub fn get_record_information(data: Mmap, ranges: Vec<(usize, usize)>, format_type: &str) {
    ranges
        .par_iter()
        .map(|&(start, end)| {
            let record_chunk = &data[start..end];
            let identifier = &record_chunk[1..find_newline(record_chunk)];
            let sequence = remove_newlines(get_sequence(record_chunk, format_type).unwrap());
        })
        .collect();
}
