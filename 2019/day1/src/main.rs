use std::io::Error;
use std::io::ErrorKind;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
  println!("{}", day1().unwrap());
}

fn day1() -> Result<i64, Error> {
  let file = File::open("./input/day1.txt")?;
  let br = BufReader::new(file);

  let mut total_fuel = 0;

  for line in br.lines() {
      let line = line?;

      let mass = line.trim().parse::<i64>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
      let mut fuel = mass / 3 - 2;

      // Part 1
      // total_fuel += fuel;

      // Part 2
      while fuel > 0 {
          total_fuel += fuel;
          fuel = fuel / 3 - 2;
      }
  }

  Ok(total_fuel)
}
