#[cfg(test)]
mod test {
    use crate::{
        io::fasta,
        structs::sequence_ref::{quality_to_str, sequence_to_str},
    };
    use std::time::Instant;

    #[test]
    fn test_fasta_read() {
        let start_time = Instant::now();
        println!("Started");
        for record in fasta::read_fasta("src/input/the_big_one.seq")
            .unwrap()
            .records
        {
            println!(
                "{:#?}",
                (
                    record.get_identifier(),
                    sequence_to_str(record.get_sequence()),
                    sequence_to_str(record.reverse_sequence().as_slice()),
                    sequence_to_str(record.complement_sequence().as_slice()),
                    sequence_to_str(record.reverse_complement_sequence().as_slice()),
                    quality_to_str(record.get_quality())
                )
            )
        }
        let duration = start_time.elapsed();
        println!("Time to read the_big_one.seq: {:?}", duration);
    }
}
