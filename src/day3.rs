fn read_file() -> String {
    std::fs::read_to_string("src/day_3_input.txt").expect("Should definitely be here")
}

fn banks_from_src(src: &str) -> Vec<Bank> {
    src.lines()
        .map(|line| Bank {
            batteries: line
                .trim()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|x| x as usize))
                .collect(),
        })
        .collect()
}
struct Bank {
    batteries: Vec<usize>,
}

pub fn total_output_joltage() -> usize {
    let src = read_file();
    let banks = banks_from_src(&src);
    let mut sum = 0;
    for bank in banks {
        let mut biggest_digit = None;
        let mut max_joltage = 0;
        for battery_joltage in bank.batteries {
            let is_start = biggest_digit.is_none();
            let current_max = biggest_digit.get_or_insert(battery_joltage);
            let next_joltage = *current_max * 10 + battery_joltage;
            if battery_joltage > *current_max {
                *current_max = battery_joltage;
            }
            if !is_start {
                max_joltage = max_joltage.max(next_joltage);
            }
        }
        sum += max_joltage;
    }
    sum
}

pub fn total_output_joltage_with_friction() -> usize {
    let src = read_file();
    let banks = banks_from_src(&src);
    let mut sum = 0;
    for bank in banks {
        let value_of = |digits: &Vec<usize>| {
            let value: usize = digits
                .iter()
                .rev()
                .enumerate()
                .map(|(i, &digit)| digit * (10usize.pow(i as u32)))
                .sum();
            value
        };
        let mut joltage_digits = Vec::<usize>::new();
        for (i, &joltage) in bank.batteries.iter().enumerate() {
            while let Some(digit) = joltage_digits.last()
                && *digit < joltage
                && (12 - joltage_digits.len()) < bank.batteries.len() - i
            {
                joltage_digits.pop();
            }
            if joltage_digits.len() < 12 {
                joltage_digits.push(joltage);
            }
        }
        sum += value_of(&joltage_digits);
    }
    sum
}
