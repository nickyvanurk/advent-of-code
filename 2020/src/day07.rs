use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: String) {
    let bags = parse_bag_rules(input);
    let num_bags = get_num_bags_containing(&"shiny gold", &mut vec![], &bags);

    println!("{}", num_bags);
}

pub fn part2(input: String) {
    let bags = parse_bag_rules(input);
    let num_bags = get_num_bags_contained(&"shiny gold", 1, &bags);

    println!("{}", num_bags);
}

fn get_num_bags_containing(
    color: &str,
    colors: &mut Vec<String>,
    bags: &HashMap<String, Vec<(usize, String)>>,
) -> usize {
    bags.iter()
        .filter(|&(_, children)| children.iter().any(|(_, c)| c == color))
        .for_each(|(bag, _)| {
            get_num_bags_containing(bag, colors, bags);
            colors.push(bag.to_string());
        });

    colors.iter().unique().count()
}

fn get_num_bags_contained(
    color: &str,
    amount: usize,
    bags: &HashMap<String, Vec<(usize, String)>>,
) -> usize {
    bags.iter()
        .filter(|&(bag, _)| bag == color)
        .fold(0, |acc1, (_, children)| {
            acc1 + amount
                * children.iter().fold(0, |acc2, (quantity, bag)| {
                    acc2 + get_num_bags_contained(bag.as_str(), *quantity, bags) + quantity
                })
        })
}

fn parse_bag_rules(input: String) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags = HashMap::new();

    input
        .lines()
        .filter(|line| !line.contains("no other"))
        .for_each(|line| {
            let root_bag_re = Regex::new(r"^(\w*\s\w*)").unwrap();
            let child_bag_re = Regex::new(r"(\d)\s(\w*\s\w*)").unwrap();

            let bag = root_bag_re.captures(line).unwrap().get(0).unwrap().as_str();
            let mut children = vec![];

            for caps in child_bag_re.captures_iter(line) {
                let quantity = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let color = caps.get(2).map(|m| m.as_str()).unwrap();

                children.push((quantity, color.to_string()));
            }

            bags.insert(bag.to_string(), children);
        });

    bags
}
