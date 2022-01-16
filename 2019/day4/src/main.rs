use regex::Regex;

fn main() {
    let mut match_count = 0;
    let re = Regex::new(r"^0*1*2*3*4*5*6*7*8*9*$").unwrap();

    for i in 284639..748759 {
        let num = i.to_string();

        if re.is_match(&num) {
            let mut prev_char: char = num.chars().next().unwrap();
            let mut has_same_digits = false;

            for (i, c) in num.chars().enumerate() {
                if i == 0 { continue; }

                if c == prev_char {
                    has_same_digits = true;
                }

                prev_char = c;
            }

            if has_same_digits {
                match_count += 1;
            }
        }
    }

    println!("Part One: {}", match_count);

    match_count = 0;

    for i in 284639..748759 {
        let num = i.to_string();

        if re.is_match(&num) {
            let mut prev_char: char = num.chars().next().unwrap();
            let mut same_digits_count = 1;

            for (i, c) in num.chars().enumerate() {
                if i == 0 { continue; }

                if c == prev_char {
                    same_digits_count += 1;
                } else {
                    if same_digits_count == 2 {
                        break;
                    }

                    same_digits_count = 1;
                }

                prev_char = c;
            }

            if same_digits_count == 2 {
                match_count += 1;
            }
        }
    }

    println!("Part Two: {}", match_count);
}
