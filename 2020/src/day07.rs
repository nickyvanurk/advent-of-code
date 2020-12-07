use itertools::Itertools;

pub fn part1(input: String) {
    let parsed = input
        .lines()
        .filter(|&l| !l.contains("no other"))
        .map(|l| {
            l.match_indices("bag")
                .map(|(idx, _)| l[..idx].trim().split(' ').rev().take(2).collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let num_bag_colors = get_bags_containing(&"goldshiny", &mut vec![], &parsed);

    println!("{}", num_bag_colors);
}

pub fn part2(input: String) {
    // can use hashmap prolly

    let parsed = input
        .lines()
        .filter(|&l| !l.contains("no other"))
        .map(|l| {
            l.match_indices("bag")
                .map(|(idx, _)| {
                    let quantity = match l[..idx].matches(char::is_numeric).last() {
                        Some(n) => n.parse::<usize>().unwrap(),
                        None => 0,
                    };
                    let color = l[..idx].trim().split(' ').rev().take(2).collect::<String>();

                    (quantity, color)
                })
                .collect::<Vec<(usize, String)>>()
        })
        .collect::<Vec<Vec<(usize, String)>>>();

    println!("{}", get_bags_contained(&"goldshiny", 1, &parsed));
}

fn get_bags_contained(color: &str, amount: usize, bags: &Vec<Vec<(usize, String)>>) -> usize {
    let bag_tuples = bags
        .iter()
        .filter(|b| b.first().unwrap().1 == color)
        .map(|b| b.iter().skip(1).collect::<Vec<&(usize, String)>>())
        .collect::<Vec<Vec<&(usize, String)>>>();

    if bag_tuples.is_empty() {
        return 0;
    }

    let mut a = 0;

    for bag in bag_tuples {
        let q = bag.iter().fold(0, |acc, &(quantity, color)| {
            acc + get_bags_contained(color, *quantity, bags) + quantity
        });

        a += amount * q;
    }

    a
}

fn get_bags_containing(color: &str, colors: &mut Vec<String>, bags: &Vec<Vec<String>>) -> usize {
    let bag_colors: Vec<&String> = bags
        .iter()
        .filter(|b| {
            let position = b.iter().position(|c| c == color);

            match position {
                Some(idx) => idx != 0,
                None => false,
            }
        })
        .map(|b| b.first().unwrap())
        //.inspect(|v| println!("{:?}", v))
        .collect();

    for color in bag_colors {
        colors.push(color.to_string());
        get_bags_containing(color, colors, bags);
    }

    colors.iter().unique().count()
}
