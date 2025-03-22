use std::time::Instant;
pub use io::fasta;

#[test]
fn main() {
    let start_time = Instant::now();
    println!("Started");
    print!("{:#?}", fasta::read_fasta("/config/workspace/sundrive-biotel-main/server/src/input/1-M13F.seq"));
    // let _ = fasta::read_fasta("/config/workspace/sundrive-biotel-main/server/src/input/1-M13F.seq");
    let duration = start_time.elapsed();
    println!("Time to read moderate fasta: {:?}", duration);
}
