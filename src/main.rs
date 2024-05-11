use data_fetcher::fetch;
use gif_generator::generate_gif;

mod data_fetcher;
mod gif_generator;

fn main() {
    let (d1, d2) = fetch(2023, "Japan", "VER", "HAM");
    generate_gif(d1, d2, 512, 512);
}
