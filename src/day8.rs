use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    sync::TryLockResult,
};

fn read_file() -> String {
    std::fs::read_to_string("src/day_8_input.txt").expect("Should have day 8 input file")
}

fn positions(src: &str) -> Vec<(usize, usize, usize)> {
    src.lines()
        .map(|line| {
            let mut numbers = line.split(",").filter_map(|n| n.parse::<usize>().ok());
            numbers
                .next()
                .and_then(|x| {
                    numbers
                        .next()
                        .and_then(|y| numbers.next().map(|z| (x, y, z)))
                })
                .expect("Should have three numbers on each line")
        })
        .collect()
}

fn circuits(
    positions: &[(usize, usize, usize)],
    max_connections: usize,
) -> (Vec<Vec<usize>>, Option<(usize, usize)>) {
    let mut connections = Vec::new();
    for (i, p1) in positions.iter().enumerate() {
        for (j, p2) in positions.iter().enumerate() {
            if i == j {
                continue;
            }
            let (i, j) = (i.min(j), j.max(i));
            connections.push((
                p1.0.abs_diff(p2.0).pow(2)
                    + p1.1.abs_diff(p2.1).pow(2)
                    + p1.2.abs_diff(p2.2).pow(2),
                (i, j),
            ));
        }
    }
    connections.sort_by_key(|(distance, _)| *distance);
    connections.dedup();
    let connections = connections
        .into_iter()
        .map(|(_, edge)| edge)
        .collect::<Vec<_>>();

    let mut parents = Vec::from_iter((0usize..).take(positions.len()).map(|i| i));
    let mut breaker = None;
    for &(i, j) in &connections[..] {
        let root_i = root(i, &mut parents);
        let root_j = root(j, &mut parents);
        if root_i < root_j {
            parents[root_j] = root_i;
        } else {
            parents[root_i] = root_j;
        }
        if (0..positions.len())
            .map(|x| root(x, &mut parents))
            .collect::<BTreeSet<_>>()
            .len()
            <= 1
        {
            println!("{:?} {:?}", positions[i], positions[j]);
            breaker = Some((i, j));
            break;
        }
    }
    fn root(node: usize, parents: &mut [usize]) -> usize {
        let root = {
            let mut root = node;
            while let parent = parents[root]
                && root != parent
            {
                root = parent;
            }
            root
        };
        let mut node = node;
        while root != parents[node] {
            let parent = &mut parents[node];
            *parent = root;
            node = *parent;
        }
        root
    }
    let mut groups = BTreeMap::new();
    for node in 0..positions.len() {
        groups
            .entry(root(node, &mut parents))
            .or_insert(Vec::new())
            .push(node);
    }
    (groups.into_values().collect(), breaker)
}

pub fn product_of_size_of_largest_ciruits() -> usize {
    let src = read_file();
    let positions = positions(&src);
    let (_, breaker) = circuits(&positions, 1000);
    if let Some((i, j)) = breaker {
        positions[i].0 * positions[j].0
    } else {
        0
    }
}
