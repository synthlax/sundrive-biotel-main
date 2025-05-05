use std::borrow::Cow;

use memmap2::Mmap;
use rayon::prelude::*;

use crate::structs::sequence_ref::Record;

#[derive(Debug)]
pub struct FileData<'a> {
    pub records: Vec<Record<'a>>,
    pub mmap: Mmap,
}

pub fn get_identifier_positions(data: &[u8], symbol: char) -> Vec<usize> {
    let mut identifier_positions = Vec::with_capacity(1024);
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
            let end = identifier_positions
                .get(i + 1)
                .copied()
                .unwrap_or(data.len());
            (start, end)
        })
        .collect()
}

pub fn find_first_newline(data: &[u8]) -> usize {
    data.iter()
        .position(|&b| b == b'\n' || b == b'\r')
        .unwrap_or(data.len())
}

pub fn remove_newlines(data: &[u8]) -> Vec<u8> {
    let mut new_data = Vec::with_capacity(data.len());
    for &b in data {
        if b != b'\n' && b != b'\r' {
            new_data.push(b);
        }
    }
    new_data
}

pub fn get_sequence<'a>(record_chunk: &'a [u8], format_type: &str) -> Option<(&'a [u8], usize)> {
    let start = find_first_newline(record_chunk) + 1;
    match format_type {
        "FASTA" | "PLAIN" => Some((&record_chunk[start..], record_chunk.len())),
        "FASTQ" => {
            let mut symbol_position = find_first_newline(record_chunk) + 1;
            while symbol_position < record_chunk.len() {
                if record_chunk[symbol_position] == b'+'
                    && (symbol_position == 0 || record_chunk[symbol_position - 1] == b'\n')
                {
                    break;
                }
                symbol_position += 1;
            }
            Some((
                &record_chunk[start..symbol_position.saturating_sub(1)],
                symbol_position + 1,
            ))
        }
        _ => None,
    }
}

pub fn get_records<'a>(
    data: &'a [u8],
    ranges: Vec<(usize, usize)>,
    format_type: &str,
) -> Vec<Record<'a>> {
    ranges
        .par_iter()
        .map(|&(start, end)| {
            let record_chunk = &data[start..end];
            let identifier = &record_chunk[1..find_first_newline(record_chunk)];
            let identifier_str = std::str::from_utf8(identifier).unwrap_or_default();
            let (sequence_raw, quality_offset) =
                get_sequence(record_chunk, format_type).unwrap_or((&[], record_chunk.len()));
            let sequence = if sequence_raw.contains(&b'\n') {
                Cow::Owned(remove_newlines(sequence_raw))
            } else {
                Cow::Borrowed(sequence_raw)
            };
            let quality = &record_chunk[quality_offset..];

            Record::new(identifier_str, sequence, quality)
        })
        .collect()
}
