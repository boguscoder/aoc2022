use std::env;
use std::fs;
use std::collections::HashSet;

fn common_item_prio(line: &str) -> i32 {
  let (split1, split2) = line.split_at(line.chars().count() / 2);
  let ruck1: HashSet<char> = HashSet::from_iter(split1.chars());
  let ruck2: HashSet<char> = HashSet::from_iter(split2.chars());

  let common = ruck1.intersection(&ruck2).next().unwrap();

  match common {
    'a'..='z' => *common as i32 - 97 +1,
    'A'..='Z' => *common as i32 - 65 + 27,
    _ => panic!("wrong input")
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");

  {
    // round 1
    let sum : i32 = contents.split('\n').map(common_item_prio).sum();
    println!("{}", sum);
  }
}