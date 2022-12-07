use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn test_quad(iter: &(usize, &[u8]), size: usize) -> bool {
  let quad = iter.1;
  let set: HashSet<&u8> = HashSet::from_iter(quad);
  set.len() == size
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut contents = File::open(&args[2]).expect("no file found");
  let metadata = fs::metadata(&args[2]).expect("unable to read metadata");
  let mut buffer = vec![0; metadata.len() as usize];
  contents.read(&mut buffer).expect("buffer overflow");
  {
    // round 1
    let windows = buffer.windows(4);
    let found = windows.enumerate().find(|e| test_quad(e, 4)).unwrap().0 + 4;
    println!("{found}");
  }
  {
    // round 2
    let windows = buffer.windows(14);
    let found = windows.enumerate().find(|e| test_quad(e, 14)).unwrap().0 + 14;
    println!("{found}");
  }
}
