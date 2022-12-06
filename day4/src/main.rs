#![feature(iter_array_chunks)]

use std::env;
use std::fs;

fn to_range(range_str: &str) -> [i32; 2] {
  let mut chunks = range_str.split('-').map(|e | e.parse::<i32>().unwrap());
  [chunks.next().unwrap(), chunks.next().unwrap()]
}

fn contains(line: &str) -> u32 {
  let chunks :Vec<&str> = line.split(',').collect();
  let ranges = [to_range(chunks[0]), to_range(chunks[1])];
  if (ranges[0][0] >= ranges[1][0] && ranges[0][1] <=ranges[1][1]) ||
    (ranges[1][0]>= ranges[0][0] && ranges[1][1] <=ranges[0][1]) {
    return 1;
  }
  0
} 

fn overlaps(line: &str) -> u32 {
  let chunks :Vec<&str> = line.split(',').collect();
  let ranges = [to_range(chunks[0]), to_range(chunks[1])];
  if (ranges[0][0] <= ranges[1][0] && ranges[0][1] >=ranges[1][0]) ||
    (ranges[1][0]<= ranges[0][0] && ranges[1][1] >= ranges[0][0]) {
    return 1;
  }
  0
} 

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");
  {
    // round 1
    let sum : u32 = contents.split('\n').map(contains).sum();
    println!("{sum}");
  }
  {
    // round 2
    let sum : u32 = contents.split('\n').map(overlaps).sum();
    println!("{sum}");
  }
}
