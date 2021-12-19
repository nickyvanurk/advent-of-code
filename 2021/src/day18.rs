use std::collections::VecDeque;
use regex::Regex;

pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let mut prev_line = String::from(input.nth(0).unwrap());

    for line in input {
        let mut l = String::from('[');
        l.push_str(&prev_line[0..]);
        l.push_str(",");
        l.push_str(line);
        l.push_str("]");
        
        // println!("{:?}", l);
        
        loop {
            let exploded = explode(&mut l);
            if exploded { continue; }
            let splitted = split(&mut l);
            if splitted { continue; }

            if !exploded && !splitted {
                println!("{:?}", l);
                break;
            }
        }

        prev_line = l;
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn split(line: &mut String) -> bool {
    let re = Regex::new(r"\d{2,}").unwrap();
    for cap in re.captures_iter(&line.clone()[0..]) {
        println!("S --- input {:?}", line);

        let num: f32 = (&cap[0]).parse::<f32>().unwrap();
        *line = line.clone().replacen(&format!("{}", &cap[0]), &format!("[{},{}]", (num / 2_f32).floor(), (num / 2_f32).ceil()), 1);
        
        println!("S --- digit {:?}", &format!("{}", &cap[0]));
        println!("S --- result {:?}", line);
        println!("");

        return true;
    }

    false
}

fn explode(line: &mut String) -> bool {
    let mut start_idx = 0;
    let mut depth = 0;

    let mut numbers: VecDeque<u32> = VecDeque::new();
    let re = Regex::new(r"\d+").unwrap();
    for cap in re.captures_iter(&line.clone()[0..]) {
        numbers.push_back(cap[0].parse::<u32>().unwrap());
    }

    let mut prev_numbers = vec![];
    let mut digit = String::new();

    for (i, c) in line.clone().chars().enumerate() {    
        if c == '[' {
            start_idx = i;
            depth += 1;
        }

        if c == ']' {
            depth -= 1;

            let n2 = format!("{}", prev_numbers.last().unwrap_or(&0)).len();
            let n1 = format!("{}", prev_numbers.iter().nth(prev_numbers.len() - 2).unwrap_or(&0)).len();
            
            // println!("n1: {}, n2: {}, depth: {}", n1, n2, depth);

            if (i - start_idx) == (n1+n2+2) && depth >= 4 {
                println!("E --- input {}", line);

                let n2 = prev_numbers.pop().unwrap();
                let n1 = prev_numbers.pop().unwrap();
                let s = format!("[{},{}]", n1, n2);
                let line_clone = line.clone();
                *line = String::from(&line[0..start_idx]);
                line.push_str(&line_clone[start_idx..].replacen(&s[0..], "0", 1));

                println!("E --- digits {}, prevs; {:?}", s, prev_numbers);
                println!("{}", line);
                // println!("{:?}", &line[..=i]);
                // println!("{:?}", numbers);

                let mut prev_num = 0;
                if let Some(prev) = prev_numbers.last() {
                    if let Some(pos) = String::from(&line[0..start_idx]).rfind(&format!("{}", prev)) {
                        let mut l = String::from(&line[..start_idx]);
                        l.replace_range(pos..start_idx, &format!("{}", prev + n1)[0..]);
                        l.push_str(&line[pos+format!("{}", prev).len()..]);
                        *line = l;
                        prev_num = prev + n1;
                    }
                }

                // a little bug here where if the explosion result too far to the left,
                // it adds a number on the right to the wrong target
                if let Some(next) = numbers.pop_front() {
                    let prev_num_size = format!("{}", prev_num).len();
                    println!("{:?}", prev_num_size);

                    println!("E FFFF--- {}", &line[0..start_idx+1+prev_num_size]);
                    println!("E TTTT--- {}", &line[start_idx+s.len()-3-prev_num_size..]);
                    

                    // bug is here, with differen regular int lengths

                    let mut l = String::from(&line[0..start_idx+1+prev_num_size]);
                    l.push_str(&line[start_idx+s.len()-3-prev_num_size..].replacen(&format!("{}", next), &format!("{}", next + n2)[0..], 1));
                    *line = l;
                }

                println!("E --- result {}", line);
                println!("");

                return true
            }
        }

        if c.is_digit(10) {
            if &line[i+1..=i+1] == "," || &line[i+1..=i+1] == "]" {
                let d = numbers.pop_front().unwrap();
                digit.push(c);
                prev_numbers.push(digit.parse::<u32>().unwrap());

                if digit.parse::<u32>().unwrap() != d {
                    // println!("Error parsing digit{}", digit);
                    return false
                }
                digit.clear();
            } else {
                digit.push(c);
            }
        }
    }

    false
}