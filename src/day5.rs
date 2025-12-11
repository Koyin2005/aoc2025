use std::ops::{Range, RangeInclusive};

fn read_file() -> String {
    std::fs::read_to_string("src/day_5_input.txt").expect("It should be here")
}

fn fresh_ranges_and_ids(src: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let mut lines = src.lines();
    let mut fresh_ranges = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let mut numbers = line.split('-');
        let first = numbers
            .next()
            .expect("There should be a first number")
            .parse::<usize>()
            .expect("Should be a number");
        let second = numbers
            .next()
            .expect("There should be a second number")
            .parse::<usize>()
            .expect("Should be a number");
        fresh_ranges.push(first..=second);
    }
    let ids = lines
        .filter_map(|line| line.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();
    (fresh_ranges, ids)
}
pub fn fresh_ingredients() -> usize {
    let src = read_file();
    let (ranges, available) = fresh_ranges_and_ids(&src);

    available
        .into_iter()
        .filter(|n| ranges.iter().any(|range| range.contains(n)))
        .count()
}

pub fn fresh_ingredients_in_total() -> usize {
    /*
        --(---)----(-(--



    */
    let src = read_file();
    let (ranges, _) = fresh_ranges_and_ids(&src);
    let mut points = ranges
        .into_iter()
        .flat_map(|range| [(*range.start(), 1isize), (*range.end() + 1, -1isize)])
        .collect::<Vec<_>>();

    points.sort();
    let mut paren_counter = 0isize;
    let mut prev_point = 0;
    let ranges = points
        .iter()
        .copied()
        .map(|(point, offset)| {
            let val = (prev_point, point, paren_counter);
            prev_point = point;
            paren_counter += offset;
            val
        })
        .filter_map(|(prev, curr, parens)| (prev != curr && parens > 0).then(|| prev..curr))
        .collect::<Vec<_>>();

    let ranges = {
        let mut all_ranges: Vec<_> = Vec::<Range<usize>>::new();
        for range in ranges {
            if let Some(prev_range) = all_ranges.last().cloned()
                && prev_range.end == range.start
            {
                all_ranges.pop();
                all_ranges.push(prev_range.start..range.end);
                continue;
            }

            all_ranges.push(range);
        }
        all_ranges
    };
    ranges
        .into_iter()
        .map(|range| range.end - range.start)
        .sum()
}
