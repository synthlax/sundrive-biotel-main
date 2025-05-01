#[cfg(test)]
mod test{
    use std::time::Instant;
    use crate::io::fasta;
    
    #[test]
    fn test_fasta_read() {
        let start_time = Instant::now();
        println!("Started");
        println!("{:#?}", fasta::read_fasta("/workspaces/sundrive-biotel-main/server/src/input/1-M13F.seq").unwrap());
        let duration = start_time.elapsed();
        println!("Time to read moderate fasta: {:?}", duration);
    }
}

