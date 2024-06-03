use data_fetcher::fetch;
use gif_generator::generate_gif;
use std::env;
use std::process;

mod data_fetcher;
mod gif_generator;

const FRAMERATE: u32 = 20;

fn usage_error(args: &Vec<String>) {
    eprintln!("Usage: {} <framerate> <year> <country> <driver1> <driver2>", args[0]);
    eprintln!("Please use 3-letter abbrievation of the driver, e.g. HAM, VER, ...");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        usage_error(&args);
    }

    let framerate = args[1].parse::<u32>();
    let year = args[2].parse::<u32>();
    if framerate.is_err() || year.is_err() {
        usage_error(&args);
    }

    match fetch(framerate.unwrap(), year.unwrap(), 
        args[3].as_str(), args[4].as_str(), args[5].as_str(), false) {
        Ok((d1, d2)) 
            => generate_gif(d1, d2, "animation.gif", FRAMERATE),

        Err(e) => {
            eprintln!("Unable to create a gif. Error: {}", e);
            process::exit(3);
        }
    }
}
