use std::collections::HashSet;

fn parse_step(step: &str) -> impl Iterator<Item = (isize, isize)> {
  let mut chars = step.chars();

  let coord = match chars.next() {
      Some('U') => (0, -1),
      Some('D') => (0,  1),
      Some('L') => (-1, 0),
      Some('R') => (1, 0),
      _         => (0, 0)
  };

  let dist: usize  = chars.collect::<String>().parse().unwrap();

  std::iter::repeat(coord).take(dist)
}

fn parse_line(line: &str) -> Vec<(isize, isize)> {
  line.split(",").flat_map(parse_step).scan((0,0), |pos, step| {
      pos.0 += step.0;
      pos.1 += step.1;
      Some(pos.clone())
  }).collect()
}

fn manhattan_distance(pos: &(isize, isize)) -> usize {
  pos.0.abs() as usize + pos.1.abs() as usize
}

fn main() {
    let input = std::fs::read_to_string("input/day3.txt")
        .expect("Failed to read day3.txt");

    let mut lines = input.lines();
    let wire_input_1 = parse_line(lines.next().unwrap());
    let wire_input_2 = parse_line(lines.next().unwrap());

    let wire_1: HashSet<_> = wire_input_1.iter().cloned().collect();
    let wire_2: HashSet<_> = wire_input_2.iter().cloned().collect();

    let intersections = wire_1.intersection(&wire_2);

    let closest_intersection = intersections.clone().map(manhattan_distance).min();
    println!("Part One: {:?}", closest_intersection.unwrap());

    let quickest_intersection = intersections.map(|pos|
      wire_input_1.iter().position(|x| x == pos).unwrap() +
      wire_input_2.iter().position(|x| x == pos).unwrap() + 2
    ).min();

    println!("Part Two: {:?}", quickest_intersection.unwrap());
}
