use std::{io::Write, process};

pub fn halt() {
    println!("HALT detected");
    std::io::stdout().flush().expect("failed to flush");
    process::exit(1);
}
