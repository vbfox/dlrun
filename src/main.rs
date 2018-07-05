#[macro_use]
extern crate clap;

mod args;

fn main() {
    let matches = args::get().get_matches();
    println!("Hello world");
}
