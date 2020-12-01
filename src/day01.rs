pub fn part1(input: String) {
    let expense_report: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<i32>().unwrap())
        .collect();

    println!(
        "{}",
        find_operands_of_sum(2020, &expense_report)
            .iter()
            .product::<i32>()
    );
}

pub fn part2(input: String) {
    let expense_report: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<i32>().unwrap())
        .collect();

    for expense in &expense_report {
        let mut operands = find_operands_of_sum(2020 - expense, &expense_report);

        if operands.len() == 2 {
            operands.append(&mut vec![expense.clone()]);
            println!("{}", operands.iter().product::<i32>());
            break;
        }
    }
}

fn find_operands_of_sum(sum: i32, data: &Vec<i32>) -> Vec<i32> {
    for operand_a in data {
        let operand_b = sum - operand_a;

        if data.contains(&operand_b) {
            return vec![operand_a.clone(), operand_b];
        }
    }

    return vec![];
}
