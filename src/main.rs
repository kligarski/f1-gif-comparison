use data_fetcher::fetch;

mod data_fetcher;

fn main() {
    let (d1, d2) = fetch(2023, "Japan", "VER", "HAM");
    
    println!("{} {}", d1.len(), d2.len());
}
