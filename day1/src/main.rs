use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");
  
  {
    // task 1: best sum
    let elves = contents.split("\n\n");

    let max : i32 = elves.map(
      |elf| elf.split('\n').map(|entry| entry.parse::<i32>().unwrap()).sum()
    ).max().unwrap();
    println!("{max}");
  }

  {
    // task 2: sum of top 3
    let elves = contents.split("\n\n");

    let mut elven_sums : Vec<i32> = elves.map(
      |elf| elf.split('\n').map(|entry| entry.parse::<i32>().unwrap()).sum()
    ).collect();
    elven_sums.sort();
    elven_sums.reverse();
    
    let max3 : i32 = elven_sums.iter().take(3).sum();
    println!("{max3}")
  }
}
