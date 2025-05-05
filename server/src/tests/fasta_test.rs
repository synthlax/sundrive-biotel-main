#[cfg(test)]
mod test {
    use crate::io::fasta;
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
                    record.get_sequence(),
                    record.get_quality()
                )
            )
        }
        let duration = start_time.elapsed();
        println!("Time to read the_big_one.seq: {:?}", duration);
    }
}
