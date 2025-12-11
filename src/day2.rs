use std::fmt::Formatter;

fn read_file() -> String {
    std::fs::read_to_string("src/day_2_input.txt").expect("The file should be here")
}

fn id_ranges(src: &String) -> Vec<((usize, &str), (usize, &str))> {
    src.split(",")
        .filter_map(|range| {
            let index = range.find('-')?;
            let (first, second) = ((&range[0..index]).trim(), (&range[index + 1..]).trim());
            Some(((first.parse().ok()?, first), (second.parse().ok()?, second)))
        })
        .collect()
}

pub fn total_invalid_ids_new_rule() -> usize {
    let mut sum = 0;
    let source = read_file();
    let ranges = id_ranges(&source);
    for ((first, _), (last, _)) in ranges {
        for num in first..=last {
            let digits = num
                .to_string()
                .into_bytes()
                .into_iter()
                .map(|x| x as usize)
                .collect::<Vec<_>>();
            let mut invalid = false;
            for sequence in (1..=digits.len() / 2).map(|i| &digits[0..i]) {
                let mut digits = digits.as_slice();
                if digits.len() % sequence.len() != 0 {
                    continue;
                }
                while let Some((first, rest)) = digits.split_at_checked(sequence.len())
                    && first == sequence
                {
                    digits = rest;
                }
                invalid |= digits.is_empty();
            }
            if invalid {
                sum += num;
            }
        }
    }
    sum
}

pub fn total_invalid_ids() -> usize {
    let mut sum = 0;
    let source = read_file();
    let ranges = id_ranges(&source);
    for ((first, _), (last, _)) in ranges {
        for i in first..=last {
            let num = i;
            if num.ilog10() % 2 == 0 {
                continue;
            }
            let pow_ten_half = (num.ilog10() + 1) / 2;

            let ten_powed_by_half = 10usize.pow(pow_ten_half);
            let first_half = num / ten_powed_by_half;
            let second_half = num % ten_powed_by_half;

            if first_half == second_half {
                sum += num;
            }
        }
    }
    sum
}
