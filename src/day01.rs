pub fn part1(input: String) {
    let expense_report: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<i32>().unwrap())
        .collect();

    for expense in &expense_report {
        let operand = 2020 - expense;

        if expense_report.contains(&operand) {
            println!("{} * {} = {}", expense, operand, expense * operand);
            break;
        }
    }
}
