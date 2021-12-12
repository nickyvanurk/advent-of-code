use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let input: Vec<Vec<&str>> = input.lines().map(|s| s.split('-').collect()).collect();
    let mut cavern: HashMap::<&str, Vec<&str>> = HashMap::new();
    
    for entry in &input {
        (*cavern.entry(entry[0]).or_insert(vec![])).push(entry[1]);
        (*cavern.entry(entry[1]).or_insert(vec![])).push(entry[0]);
    }

    get_paths("start", &mut vec![], &cavern)
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<Vec<&str>> = input.lines().map(|s| s.split('-').collect()).collect();
    let mut cavern: HashMap::<&str, Vec<&str>> = HashMap::new();
    
    for entry in &input {
        (*cavern.entry(entry[0]).or_insert(vec![])).push(entry[1]);
        (*cavern.entry(entry[1]).or_insert(vec![])).push(entry[0]);
    }

    get_paths2("start", &mut vec![], &cavern)
}

fn get_paths<'a>(start: &'a str, visited: &mut Vec<&'a str>, cavern: &HashMap::<&str, Vec<&'a str>>) -> u32 {
    let mut paths = 0;

    visited.push(start);

    if start == "end" {
        return 1;
    }

    for &cave in &cavern[start] {
        if cave == "start" || (cave.chars().all(char::is_lowercase) && visited.contains(&cave)) {
            continue;
        }

        paths += get_paths(cave, visited, cavern);
        visited.pop();
    }

    paths
}

fn get_paths2<'a>(start: &'a str, visited: &mut Vec<&'a str>, cavern: &HashMap::<&str, Vec<&'a str>>) -> u32 {
    let mut paths = 0;

    visited.push(start);

    if start == "end" {
        return 1;
    }

    for &cave in &cavern[start] {
        if cave == "start" {
            continue;
        }

        let small_caves = visited.iter().map(|c| *c).filter(|&c| c.chars().all(char::is_lowercase)).collect::<Vec<&str>>();
        let duplicate = small_caves.len() != small_caves.iter().unique().count();
        let is_small_cave = cave.chars().all(char::is_lowercase);
        
        if duplicate && is_small_cave && visited.contains(&cave) {
            continue;
        }

        paths += get_paths2(cave, visited, cavern);
        visited.pop();
    }

    paths
}