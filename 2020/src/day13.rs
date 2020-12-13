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

pub fn part2(input: &String) -> u32 {
    0
}
