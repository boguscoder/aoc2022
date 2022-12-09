use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");
}
