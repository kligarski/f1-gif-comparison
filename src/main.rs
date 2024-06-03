use data_fetcher::fetch;
use gif_generator::generate_gif;

mod data_fetcher;
mod gif_generator;

const FRAMERATE: u32 = 20;

fn main() {
    match fetch(FRAMERATE, 2024, "Monaco", "PIA", "TSU", true) {
        Ok((d1, d2)) 
            => generate_gif(d1, d2, "animation.gif", FRAMERATE),
        Err(e) => println!("Unable to create a gif. Error: {}", e)
    }
}
