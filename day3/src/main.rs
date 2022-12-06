#![feature(iter_array_chunks)]

use std::env;
use std::fs;
use std::collections::HashSet;

fn prio(item: char) -> i32 {
  match item {
    'a'..='z' => item as i32 - 97 +1,
    'A'..='Z' => item as i32 - 65 + 27,
    _ => panic!("wrong input")
  }
}

fn common_per_line(line: &str) -> i32 {
  let (split1, split2) = line.split_at(line.chars().count() / 2);
  let ruck1: HashSet<char> = HashSet::from_iter(split1.chars());
  let ruck2: HashSet<char> = HashSet::from_iter(split2.chars());

  let common = *ruck1.intersection(&ruck2).next().unwrap();
  prio(common)
}

fn common_per_trio(trio: [&str; 3]) -> i32 {
  let ruck1: HashSet<char> = HashSet::from_iter(trio[0].chars());
  let ruck2: HashSet<char> = HashSet::from_iter(trio[1].chars());
  let ruck3: HashSet<char> = HashSet::from_iter(trio[2].chars());

  let mut ruck1and2 = ruck1.intersection(&ruck2);
  let common = ruck1and2.find(|e| ruck3.contains(e)).unwrap();

  prio(*common)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");

  {
    // round 1
    let sum : i32 = contents.split('\n').map(common_per_line).sum();
    println!("{sum}");
  }

  {
    // round 2
    let sum : i32 = contents.split('\n').array_chunks().map(common_per_trio).sum();
    println!("{sum}");
  }
}