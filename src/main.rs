use std::{fs::File, io::Read};

fn main() {
let mut file = File::open("data/INVADERS").unwrap();
let mut data = Vec::<u8>::new();
let _ =file.read_to_end(&mut data);
println!("{:?}", data);
}
