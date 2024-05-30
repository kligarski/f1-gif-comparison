use data_fetcher::fetch;
use gif_generator::generate_gif;

mod data_fetcher;
mod gif_generator;

fn main() {
    match fetch(2023, "Hungary", "HAM", "RUS", true) {
        Ok((d1, d2)) 
            => generate_gif(d1, d2, "animation.gif"),
        Err(e) => println!("Unable to create a gif. Error: {}", e)
    }
}
