#![allow(unused)]

use clap::Parser;
use log::{info, warn};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
