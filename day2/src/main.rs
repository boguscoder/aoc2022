use std::env;
use std::fs;

fn shape_cost(shape: &str) -> i32 {
  match shape {
    "A" | "X" => 1, // rock
    "B" | "Y" => 2, // paper
    "C" | "Z" => 3, // scissors
    _ => panic!("Unexpected shape")
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[2])
    .expect("Should have been able to read the file");
  
  {
    // round 1
    fn round_outcome(round: &str) -> i32 {
      let their = shape_cost(&round[0..1]);
      let yours = shape_cost(&round[2..3]);
      
      match i32::abs(their - yours) {
        2 => yours + if their > yours {6} else {0},
        1 => yours + if their > yours {0} else {6},
        _ => yours + 3
      }
    }

    let rounds = contents.split('\n');
    let costs: i32 = rounds.map(round_outcome).sum(); 
    println!("{costs}");
  }

  {
    // round 2
    fn round_outcome(round: &str) -> i32 {
      let their = shape_cost(&round[0..1]);
      let outcome: &str = &round[2..3];
      
      match outcome {
        "X" => /*   los  */ if their > 1 {their - 1} else {3},
        "Y" => 3 /* draw */ + their,
        "Z" => 6 /* win! */ + if their < 3 {their + 1} else {1},
        _ => panic!("Unexpected outcome")
      }
    }

    let rounds = contents.split('\n');
    let costs: i32 = rounds.map(round_outcome).sum(); 
    println!("{costs}");
  }
}