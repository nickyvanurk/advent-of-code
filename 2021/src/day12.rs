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

pub fn part2(input: &String) -> u16 {
    let input: Vec<Vec<&str>> = input.lines().map(|s| s.split('-').collect()).collect();


    0
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