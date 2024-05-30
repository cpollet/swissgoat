use std::io::{stdout, Write};
include!(concat!(env!("OUT_DIR"), "/goats.rs"));

fn main() {
    let goat = rand::random::<usize>() % GOATS.len();
    stdout().write(GOATS[goat]).unwrap();
    stdout().flush().unwrap()
}
