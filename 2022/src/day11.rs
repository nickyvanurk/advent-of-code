extern crate queues;
use queues::*;

pub fn part1(input: &str) -> u128 {
    // let monkey_data = input.split("\n\n");
    // let mut monkeys: Vec<Monkey> = vec!{};

    // for data in monkey_data {
    //     let mut items: Queue<u128> = queue![];
    //     let mut operation = "".to_string();
    //     let mut test = 0;
    //     let mut throw_to = (0, 0);

    //     for line in data.lines().skip(1) {
    //         if line.contains("Starting items") {
    //             let v: Vec<u128> = line.split(':').nth(1).unwrap().trim().split(", ").map(|s| s.parse::<u128>().unwrap()).collect();
    //             for item in v {
    //                 items.add(item);
    //             }
    //         } else if line.contains("Operation") {
    //             operation = line.split("= old ").nth(1).unwrap().to_string();
    //         } else if line.contains("Test") {
    //             test = line.split("divisible by ").nth(1).unwrap().parse::<u128>().unwrap();
    //         } else if line.contains("If true") {
    //             throw_to.0 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
    //         } else if line.contains("If false") {
    //             throw_to.1 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
    //         }
    //     }

    //     monkeys.push(Monkey { items, operation, test, throw_to, inspection_count: 0 });
    // }

    // for _ in 0..20 {
    //     for idx in 0..monkeys.len() {
    //         while let Ok(mut item) = monkeys[idx].items.remove() {
    //             let t = monkeys[idx].operation.split_whitespace().collect::<Vec<&str>>();
    //             let instruction = t[0];
    //             let value = t[1];

    //             if value.parse::<u128>().is_ok() {
    //                 if instruction == "*" {
    //                     item *= value.parse::<u128>().unwrap();
    //                 } else if instruction == "+" {
    //                     item += value.parse::<u128>().unwrap();
    //                 }
    //             } else {
    //                 item *= item;
    //             }

    //             item /= 3;

    //             if item % monkeys[idx].test == 0 {
    //                 let idx = monkeys[idx].throw_to.0 as usize;
    //                 monkeys[idx].items.add(item);
    //             } else {
    //                 let idx = monkeys[idx].throw_to.1 as usize;
    //                 monkeys[idx].items.add(item);
    //             }

    //             monkeys[idx].inspection_count += 1;
    //         }
    //     }
    // }

    // let mut inspection_counts = vec![];
    // for monkey in monkeys {
    //     inspection_counts.push(monkey.inspection_count);
    // }

    // inspection_counts.sort();
    // inspection_counts = inspection_counts.into_iter().rev().collect();

    // inspection_counts[0] * inspection_counts[1]
    0
}

pub fn part2(input: &str) -> u128 {
    let monkey_data = input.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec!{};

    for data in monkey_data {
        let mut items: Queue<String> = queue![];
        let mut operation = "".to_string();
        let mut test = 0;
        let mut throw_to = (0, 0);

        for line in data.lines().skip(1) {
            if line.contains("Starting items") {
                let v: Vec<String> = line.split(':').nth(1).unwrap().trim().split(", ").map(|s| s.to_string()).collect();
                for item in v {
                    items.add(item);
                }
            } else if line.contains("Operation") {
                operation = line.split("= old ").nth(1).unwrap().to_string();
            } else if line.contains("Test") {
                test = line.split("divisible by ").nth(1).unwrap().parse::<u128>().unwrap();
            } else if line.contains("If true") {
                throw_to.0 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            } else if line.contains("If false") {
                throw_to.1 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            }
        }

        monkeys.push(Monkey { items, operation, test, throw_to, inspection_count: 0 });
    }

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while let Ok(mut item) = monkeys[idx].items.remove() {
                let t = monkeys[idx].operation.split_whitespace().collect::<Vec<&str>>();
                let instruction = t[0];
                let value = t[1];

                if value != "old" {
                    if instruction == "*" {
                        item = multiply(item, value.to_string());
                    } else if instruction == "+" {
                        item = sum(item, value.to_string());
                    }
                } else {
                    item = multiply(item.to_string(), item.to_string());
                }

                if modulo(item.to_string(), monkeys[idx].test as u32) == 0 {
                    let idx = monkeys[idx].throw_to.0 as usize;
                    monkeys[idx].items.add(item.to_string());
                } else {
                    let idx = monkeys[idx].throw_to.1 as usize;
                    monkeys[idx].items.add(item.to_string());
                }

                monkeys[idx].inspection_count += 1;
            }
        }
    }

    let mut inspection_counts = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inspection_count);
    }

    inspection_counts.sort();
    inspection_counts = inspection_counts.into_iter().rev().collect();

    inspection_counts[0] * inspection_counts[1]
}

struct Monkey {
    items: Queue<String>,
    operation: String,
    test: u128,
    throw_to: (u128, u128),
    inspection_count: u128,
}

fn multiply(num1: String, num2: String) -> String {
    let len1 = num1.len();
    let len2 = num2.len();
    if len1 == 0 || len2 == 0 {
        return "0".to_string();
    }

    let mut result = vec![0; len1 + len2];

    let mut i_n1 = 0;
    let mut i_n2 = 0;

    for i in (0..len1).rev() {
        let mut carry = 0;
        let n1 = num1.chars().nth(i).unwrap().to_digit(10).unwrap();

        i_n2 = 0;

        for j in (0..len2).rev() {
            let n2 = num2.chars().nth(j).unwrap().to_digit(10).unwrap();
            let sum = n1*n2 + result[i_n1 + i_n2] + carry;
            carry = sum / 10;
            result[i_n1 + i_n2] = sum % 10;
            i_n2 += 1;
        }

        if carry > 0 {
            result[i_n1 + i_n2] += carry;
        }

        i_n1 += 1;
    }

    let mut i = result.len() as i32 - 1;
    while i > 0 && result[i as usize] == 0 {
        i -= 1;
    }

    if i == -1 {
        return "0".to_string();
    }

    let mut s = "".to_string();

    while i >= 0 {
        s = vec![s, result[i as usize].to_string()].concat();
        i -= 1;
    }

    s
}

fn sum(num1: String, num2: String) -> String {
    let mut str1 = num1;
    let mut str2 = num2;
    if str1.len() > str2.len() {
        let t = str1;
        str1 = str2;
        str2 = t;
        // std::mem::swap(&mut str1, &mut str2);
    }

    let mut s = "".to_string();

    let n1 = str1.len();
    let n2 = str2.len();

    str1 = str1.chars().rev().collect::<String>();
    str2 = str2.chars().rev().collect::<String>();

    let mut carry = 0;
    for i in 0..n1 {
        let sum = str1.chars().nth(i).unwrap().to_digit(10).unwrap() + str2.chars().nth(i).unwrap().to_digit(10).unwrap() + carry;
        s = vec![s, (sum % 10).to_string()].concat();
        carry = sum / 10;
    }

    for i in n1..n2 {
        let sum = str2.chars().nth(i).unwrap().to_digit(10).unwrap() + carry;
        s = vec![s, (sum % 10).to_string()].concat();
        carry = sum / 10;
    }

    if carry != 0 {
        s = vec![s, carry.to_string()].concat();
    }

    s.chars().rev().collect::<String>()
}

fn modulo(num: String, a: u32) -> u32 {
    let mut res = 0;

    for i in 0..num.len() {
        res = (res * 10 + num.chars().nth(i).unwrap().to_digit(10).unwrap()) % a;
    }

    res
}