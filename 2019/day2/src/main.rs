use std::io::Error;
use std::io::ErrorKind;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn main() {
    day2().unwrap();
}

fn day2() -> Result<(), Error> {
  let file = File::open("./input/day2.txt")?;
  let br = BufReader::new(file);

  let mut numbers = Vec::new();

  for line in br.lines() {
      let line = line?;
      let number = line.trim().parse::<i64>().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
      numbers.push(number);
  }

  // Part 1
  // let mut i = 0;
  // while i <= numbers.len() {
  //     let opcode = numbers[i];
  //     let var1 = numbers[numbers[i+1] as usize];
  //     let var2 = numbers[numbers[i+2] as usize];

  //     let result = match opcode {
  //         1 => var1 + var2,
  //         2 => var1 * var2,
  //         99 => break,
  //         _ => continue
  //     };

  //     let result_dest = numbers[i+3] as usize;
  //     numbers[result_dest] = result;

  //     i += 4;
  // }

  // Part 2
  let original_numbers = numbers.to_vec();
  for i in 0..100 {
    for j in 0..100 {
      numbers = original_numbers.clone();
      numbers[1] = i;
      numbers[2] = j;

      let mut k = 0;

      while k < numbers.len() {
        let opcode = numbers[k];
        let param1 = numbers[numbers[k+1] as usize];
        let param2 = numbers[numbers[k+2] as usize];

        let result = match opcode {
            1 => param1 + param2,
            2 => param1 * param2,
            99 => break,
            _ => {
              println!{"Unknown opcode: {}, {}", opcode, k};
              k += 4;
              break
            }
        };


        let result_dest = numbers[k+3] as usize;

        if result_dest >= numbers.len() {
          println!("Result index out of bounds: {}", result_dest);
          break;
        }

        numbers[result_dest] = result;

        if numbers[0] == 19690720 {
          println!("100 * noun({}) + verb({}) = {}", i, j, 100 * i + j);
          break;
        }

        k += 4;
      }
    }
  }

  Ok(())
}
