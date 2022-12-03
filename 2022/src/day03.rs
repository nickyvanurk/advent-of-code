pub fn part1(input: &str) -> u32 {
    let input = input.lines().map(|s| (&s[..s.len()/2], &s[s.len()/2..]));
    let mut sum = 0;

    for (a, b) in input {
        let mut freq1 = vec![0; 58];
        for c in a.chars() {
            freq1[c as usize - 'A' as usize] += 1;
        }

        let mut freq2 = vec![0; 58];
        for c in b.chars() {
            freq2[c as usize - 'A' as usize] += 1;
        }

        for i in 0..58 {
            if std::cmp::min(freq1[i], freq2[i]) > 0 {
                sum += if i > 25 { i - 6 - 25 } else { i + 27 } as u32;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for i in (0..input.len()).step_by(3) {
        let a = input[i];
        let b = input[i + 1];
        let c = input[i + 2];

        let mut freq1 = vec![0; 58];
        for c in a.chars() {
            freq1[c as usize - 'A' as usize] += 1;
        }

        let mut freq2 = vec![0; 58];
        for c in b.chars() {
            freq2[c as usize - 'A' as usize] += 1;
        }

        let mut freq3 = vec![0; 58];
        for c in c.chars() {
            freq3[c as usize - 'A' as usize] += 1;
        }

        for i in 0..58 {
            if std::cmp::min(freq1[i], std::cmp::min(freq2[i], freq3[i])) > 0 {
                sum += if i > 25 { i - 6 - 25 } else { i + 27 } as u32;
            }
        }
    }

    sum
}