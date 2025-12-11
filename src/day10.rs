use std::{collections::{BTreeMap, BinaryHeap, HashSet, VecDeque}, fs::File, usize};

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
                && source.starts_with('('){
                
                let Some(source) = source.strip_prefix('(').and_then(|src| src.strip_suffix(')')) else{
                    break;
                };
                iter.next();
                let nums = source
                    .split(',')
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                button_mapping.push(nums);
                
            }
            let joltage = iter.filter_map(|src|{
                let src = src.trim().strip_prefix('{')?;
                let src = src.strip_suffix('}')?;

                Some(src.split(','))
            }).flatten().filter_map(|src| src.parse().ok()).collect();
            MachineSpec {
                on_state,
                button_mappings: button_mapping,
                joltage
            }
        })
        .collect()
}

fn find_fewest(
    target_state: &[usize],
    state: Vec<usize>,
    button_mappings: &Vec<Vec<usize>>,
) -> usize {
    let mut q = VecDeque::from([
        (0,state.clone())
    ]);
    let mut states = 0;
    let mut seen = HashSet::new();
    let mut distance = BTreeMap::from([((0usize,state.clone()),0usize)]);
    while let Some((count,state)) = q.pop_front() {
        use std::io::Write;
        let _ = writeln!(&mut File::create("src/day_10_output.txt").unwrap(),"{}",states);
        if state == target_state{
            return count;
        }
        states += 1;
        let mut new_states = Vec::new();
        'a:for (_,lights) in button_mappings.iter().enumerate(){
            let mut new_state = state.clone();
            for &light in lights{
                new_state[light] += 1;
                if new_state[light] > target_state[light]{
                    continue 'a;
                }
            }
            let next_state = (count+1,new_state);
            if seen.insert(next_state.clone()){
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
            let state = vec![0; x.on_state.len()];
            let pressed = find_fewest(
                &x.joltage,
                state,
                &x.button_mappings,
            );
            pressed
        })
        .sum()
}
