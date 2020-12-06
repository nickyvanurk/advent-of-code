pub fn part1(instructions: String) {
    let floor = instructions.matches("(").count() - instructions.matches(")").count();

    println!("{}", floor);
}

pub fn part2(instructions: String) {
    let mut floor = 0;
    let mut position = 0;

    for (i, direction) in instructions.chars().enumerate() {
        floor += if direction == '(' { 1 } else { -1 };

        if floor == -1 {
            position = i + 1;
            break;
        }
    }

    println!("{}", position);
}
