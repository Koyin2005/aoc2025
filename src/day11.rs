use core::num;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    path,
};

fn read_file() -> String {
    std::fs::read_to_string("src/day_11_input.txt").expect("It should be there")
}

fn connections(src: &str) -> BTreeMap<&str, Vec<&str>> {
    let mut connections = src
        .lines()
        .filter_map(|line| {
            let mut iter = line.split(':');
            let Some(head) = iter.next() else {
                return None;
            };

            let Some(src) = iter.next() else {
                return None;
            };
            Some((head, src.split_whitespace().map(|src| src.trim()).collect()))
        })
        .collect::<BTreeMap<_, _>>();

    let set = connections
        .values()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>();
    for node in set {
        connections.entry(node).or_default();
    }
    connections
}

fn paths<'a, 'b, 'c>(
    m: &'c mut HashMap<(&'a str, bool, bool), usize>,
    connections: &'a BTreeMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    dac: bool,
    fft: bool,
) -> usize {
    if let Some(count) = m.get(&(current, dac, fft)) {
        return *count;
    }
    if current == "out" {
        return if dac && fft { 1 } else { 0 };
    }
    let dac = dac || current == "dac";
    let fft = fft || current == "fft";
    let all_paths = connections[&current]
        .iter()
        .map(|&next| paths(m, connections, next, dac, fft))
        .sum::<usize>();
    m.insert((current, dac, fft), all_paths);
    all_paths
}
pub fn diff_paths() -> usize {
    let src = read_file();
    let connections = connections(&src);
    let mut s = HashMap::new();
    let paths = paths(&mut s, &connections, "svr", false, false);
    println!("{:?}", s);
    paths
}
