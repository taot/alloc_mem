use clap::Parser;
use std::io::stdin;
use std::{thread, time};
use serde::Serialize;
use serde_yaml;

fn pause() {
    println!("Press Enter to continue...");
    stdin().read_line(&mut String::new()).unwrap();
}

fn ratio_parser(s: &str) -> Result<f32, String> {
    let r = s.parse::<f32>().map_err(|e| format!("{e}"))?;
    if r < 0.0 || r > 1.0 {
        return Err("ratio must be between 0 and 1".to_string());
    }
    Ok(r)
}

#[derive(Parser, Debug, Serialize)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(
        short = 'm',
        long,
        default_value_t = 1,
        value_name = "M",
        help = "Size of individual memory blocks to allocate in MBs"
    )]
    block_size: usize,

    #[arg(
        short = 'e',
        long,
        default_value_t = 0,
        value_name = "N",
        help = "Time between allocations in ms"
    )]
    interval: u64,

    #[arg(
        short = 'f',
        long,
        default_value_t = 1.0,
        value_parser = ratio_parser,
        value_name = "ratio",
        help = "Touch fill ratio (between 0 and 1), how much of the committed memory gets touched per each memory block allocated"
    )]
    touch_ratio: f32,

    #[arg(
        short = 'x',
        long,
        value_name = "S",
        help = "Stop allocating once memory committed reaches this value in MBs. Does not stop if not specified"
    )]
    stop: Option<usize>,

    #[arg(
        short = 'w',
        long,
        help = "Break execution before allocation starts and wait for a key to be pressed. Useful to see initial overhead of the process"
    )]
    wait: bool,

    #[arg(
        short = 'v',
        long,
        help = "Verbose mode"
    )]
    verbose: bool,
}

const BASE_SIZE_OF_BLOCK: usize = 256 * 1024; // the number of i32's that takes 1MB

fn main() {
    let args = Cli::parse();
    println!("Welcome to memory game!");

    if args.verbose {
        println!("-----------------------");
        println!("{}", serde_yaml::to_string(&args).unwrap());
    }

    if args.wait {
        pause();
    }

    let mut list: Vec<Vec<i32>> = Vec::new();

    let mut allocated = 0;
    let mut touched = 0;

    while args.stop.is_none() || allocated < args.stop.unwrap() {
        let size = BASE_SIZE_OF_BLOCK * args.block_size;
        let mut block = Vec::<i32>::with_capacity(size);
        allocated += args.block_size;

        if ((allocated as f32) * args.touch_ratio).floor() > (touched as f32) {
            block.resize(size, 0);
            touched += args.block_size;
        }

        list.push(block);

        if args.interval > 0 {
            thread::sleep(time::Duration::from_millis(args.interval));
        }
    }

    println!("Memory allocation complete. {} MB allocated, {} MB touched.", allocated, touched);
    println!("Press Ctrl-C to exit");
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
