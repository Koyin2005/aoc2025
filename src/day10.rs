use std::{
    collections::{BTreeMap, BinaryHeap, HashSet, VecDeque},
    env::var,
    fs::File,
    usize,
};

use microlp::Problem;

fn read_file() -> String {
    std::fs::read_to_string("src/day_10_input.txt").expect("day 10 input should be in src folder")
}

#[derive(Debug)]
struct MachineSpec {
    on_state: Vec<bool>,
    button_mappings: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}
fn parse_machine_specs(src: String) -> Vec<MachineSpec> {
    src.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let on_state = iter
                .next()
                .iter()
                .flat_map(|source| {
                    source.chars().filter_map(|c| match c {
                        '[' | ']' => None,
                        '.' => Some(false),
                        '#' => Some(true),
                        _ => unreachable!("First part should not contain this"),
                    })
                })
                .collect::<Vec<_>>();

            let mut button_mapping = Vec::new();
            let mut iter = iter.peekable();
            while let Some(source) = iter.peek()
                && source.starts_with('(')
            {
                let Some(source) = source
                    .strip_prefix('(')
                    .and_then(|src| src.strip_suffix(')'))
                else {
                    break;
                };
                iter.next();
                let nums = source
                    .split(',')
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                button_mapping.push(nums);
            }
            let joltage = iter
                .filter_map(|src| {
                    let src = src.trim().strip_prefix('{')?;
                    let src = src.strip_suffix('}')?;

                    Some(src.split(','))
                })
                .flatten()
                .filter_map(|src| src.parse().ok())
                .collect();
            MachineSpec {
                on_state,
                button_mappings: button_mapping,
                joltage,
            }
        })
        .collect()
}

fn find_fewest(
    target_state: &[usize],
    state: Vec<usize>,
    button_mappings: &Vec<Vec<usize>>,
) -> usize {
    let mut q = VecDeque::from([(0, state.clone())]);
    let mut states = 0;
    let mut seen = HashSet::new();
    while let Some((count, state)) = q.pop_front() {
        use std::io::Write;
        let _ = writeln!(
            &mut File::create("src/day_10_output.txt").unwrap(),
            "{}",
            states
        );
        if state == target_state {
            return count;
        }
        states += 1;
        let mut new_states = Vec::new();
        'a: for (_, lights) in button_mappings.iter().enumerate() {
            let mut new_state = state.clone();
            for &light in lights {
                new_state[light] += 1;
                if new_state[light] > target_state[light] {
                    continue 'a;
                }
            }
            let next_state = (count + 1, new_state);
            if seen.insert(next_state.clone()) {
                new_states.push(next_state);
            }
        }
        q.extend(new_states);
    }
    0
}
pub fn fewest_button_presses() -> usize {
    let src = read_file();
    let spec = parse_machine_specs(src);
    println!("{:?}", spec);

    spec.into_iter()
        .map(|x| {
            let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);

            let mut effect_map = vec![Vec::new(); x.on_state.len()];
            for (button, mapping) in x.button_mappings.iter().enumerate() {
                for &light in mapping {
                    effect_map[light].push(button);
                }
            }
            let button_vars = (0..x.button_mappings.len())
                .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
                .collect::<Vec<_>>();
            println!("{:?}", effect_map);
            for (joltage, effect) in x.joltage.iter().zip(effect_map) {
                problem.add_constraint(
                    effect
                        .into_iter()
                        .map(|button| (button_vars[button], 1.0f64)),
                    microlp::ComparisonOp::Eq,
                    *joltage as f64,
                );
            }
            let value = problem.solve().expect("It should be solvable");
            println!("{:?} {}", value.objective(), value.objective().round());
            /*
               (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
               0    1    2    3     4     5
               {a,b,c,d}
               (b4 + b5) = 3
               (b1 + b5) = 5
               (b2 + b3 + b4) = 4
               (b0 + b1 + b3) = 7
               b0 + b1 + b2 + b3
            */

            value.objective().round() as usize
        })
        .sum()
}
