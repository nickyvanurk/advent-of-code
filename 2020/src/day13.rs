pub fn part1(input: &String) -> u32 {
    let earliest_depart_timestamp = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let bus_ids = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter(|&bus_id| bus_id != "x")
        .map(|bus_id| bus_id.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut shortest_time_before_departure = (0, 999999); // bus_id, time

    for bus_id in bus_ids.iter() {
        let time_before_departure = bus_id - earliest_depart_timestamp % bus_id;

        if time_before_departure < shortest_time_before_departure.1 {
            shortest_time_before_departure.0 = *bus_id;
            shortest_time_before_departure.1 = time_before_departure;
        }
    }

    shortest_time_before_departure.0 * shortest_time_before_departure.1
}

pub fn part2(input: &String) -> u64 {
    let bus_ids = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, bus_id)| bus_id != "x")
        .map(|(i, bus_id)| (i, bus_id.parse::<u64>().unwrap()))
        .collect::<Vec<(usize, u64)>>();

    let mut time = 0;
    let product = bus_ids.iter().fold(1, |acc, &(_, bus_id)| acc * bus_id);

    for &(idx, bus_id) in bus_ids.iter() {
        let pp = product / bus_id;
        time += (bus_id as i64 - idx as i64) * mod_inverse(pp, bus_id) as i64 * pp as i64;
    }

    time as u64 % product
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    for x in 1..m {
        if (a * x) % m == 1 {
            return x;
        }
    }

    0
}
