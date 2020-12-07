use std::collections::HashMap;

pub fn part1(input: String) {
    let bags = parse_bag_rules(input);
    let num_bags = get_num_bags_containing(&"shiny gold", &bags);

    println!("{}", num_bags);
}

pub fn part2(input: String) {
    let bags = parse_bag_rules(input);
    let num_bags = get_num_inner_bags(&"shiny gold", &bags);

    println!("{}", num_bags);
}

fn get_num_bags_containing(color: &str, bags: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut sum = 0;

    for bag in bags.keys() {
        if bag != color && bag_contains(color, bag, &bags) {
            sum += 1;
        }
    }

    sum
}

fn bag_contains(color: &str, bag: &str, bags: &HashMap<String, Vec<(usize, String)>>) -> bool {
    if bag == color {
        return true;
    }

    if let Some(inner_bags) = bags.get(bag) {
        for inner_bag in inner_bags {
            if bag_contains(&color, &inner_bag.1, bags) {
                return true;
            }
        }
    };

    return false;
}

fn get_num_inner_bags(color: &str, bags: &HashMap<String, Vec<(usize, String)>>) -> usize {
    if let Some(inner_bags) = bags.get(color) {
        return inner_bags.iter().fold(0, |sum, inner_bag| {
            sum + inner_bag.0 + inner_bag.0 * get_num_inner_bags(&inner_bag.1, &bags)
        });
    }

    return 0;
}

fn parse_bag_rules(input: String) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags = HashMap::new();

    input
        .lines()
        .filter(|line| !line.contains("no other"))
        .for_each(|line| {
            let parsed = line.split(" bags contain ").collect::<Vec<&str>>();

            let bag = parsed[0];
            let inner_bags = parsed[1]
                .replace(".", "")
                .replace(" bags", "")
                .replace(" bag", "")
                .split(", ")
                .map(|bag| {
                    let quantity = bag
                        .matches(char::is_numeric)
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let color = bag.split(' ').skip(1).collect::<Vec<&str>>().join(" ");

                    (quantity, color)
                })
                .collect::<Vec<(usize, String)>>();

            bags.insert(bag.to_string(), inner_bags);
        });

    bags
}
