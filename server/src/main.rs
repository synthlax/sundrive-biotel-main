use io::fasta;
use std::time::Instant;

pub mod error;
pub mod io;
pub mod structs;
mod tests;

fn main() {
    let start_time = Instant::now();
    println!("Started");
    println!(
        "{:#?}",
        fasta::read_fasta(
            "C:/Users/angel/Documents/GitHub/sundrive-biotel-main/server/src/input/1-M13F.seq"
        )
        .unwrap()
    );
    // let _ = fasta::read_fasta("/config/workspace/sundrive-biotel-main/server/src/input/1-M13F.seq");
    let duration = start_time.elapsed();
    println!("Time to read moderate fasta: {:?}", duration);
}
