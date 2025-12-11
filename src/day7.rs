use std::{
    borrow::Cow,
    cell,
    collections::{HashMap, HashSet, hash_map::Entry},
    fs::File,
    io::Write,
};

fn read_file() -> String {
    std::fs::read_to_string("src/day_7_input.txt")
        .expect("`day_7_input.txt` should be in the src file")
}
#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Splitter,
    Beam,
    Laser,
}
fn grid(src: &str) -> Vec<Vec<Cell>> {
    src.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'S' => Cell::Beam,
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    _ => unreachable!("Can only have 'S', '.' and '^' as characters in map"),
                })
                .collect()
        })
        .collect()
}
fn print_grid(grid: &[Vec<Cell>], output: &mut dyn Write) {
    for row in grid {
        for cell in row {
            let _ = output.write_fmt(format_args!(
                "{}",
                match cell {
                    Cell::Beam => "S",
                    Cell::Empty => ".",
                    Cell::Laser => "|",
                    Cell::Splitter => "^",
                }
            ));
        }
        let _ = output.write(b"\n");
    }
    let _ = output.write(b"\n");
}

pub fn total_splits() -> usize {
    let src = read_file();
    let mut grid = grid(&src);
    let (row, column) = grid
        .iter()
        .enumerate()
        .find_map(|(row, cells)| {
            cells
                .iter()
                .position(|c| matches!(c, Cell::Beam))
                .map(|column| (row, column))
        })
        .expect("There should be an 'S' in the grid");

    let mut beam_positions = vec![(row + 1, column)];
    while !beam_positions.is_empty() {
        for &(row, column) in &beam_positions {
            grid[row][column] = Cell::Laser;
        }
        let mut new_beams = Vec::new();
        let mut beams_to_remove = HashSet::new();
        let mut seen_beams = HashSet::new();
        for (i, (row, col)) in beam_positions.iter_mut().enumerate() {
            let new_row = *row + 1;
            let new_row = if let Some(cells) = grid.get(new_row) {
                if matches!(cells[*col], Cell::Splitter) {
                    let left_col = *col - 1;
                    let right_col = *col + 1;
                    let left_pos = (new_row, left_col);
                    let right_pos = (new_row, right_col);
                    if cells.get(left_col).is_some() && seen_beams.insert(left_pos) {
                        new_beams.push(left_pos);
                    }
                    if cells.get(right_col).is_some() && seen_beams.insert(right_pos) {
                        new_beams.push(right_pos);
                    }
                    beams_to_remove.insert(i);
                    new_row
                } else {
                    new_row
                }
            } else {
                beams_to_remove.insert(i);
                new_row
            };
            *row = new_row;
        }
        let mut i = 0;
        beam_positions.retain(|_| {
            let should_remove = beams_to_remove.contains(&i);
            i += 1;
            !should_remove
        });
        beam_positions.extend(new_beams);
    }

    grid[1..]
        .chunks(2)
        .filter_map(|rows| {
            let [prev_row, curr_row] = rows else {
                return None;
            };
            Some((prev_row, curr_row))
        })
        .map(|(prev_row, curr_row)| {
            prev_row
                .iter()
                .zip(curr_row)
                .map(
                    |(&above_cell, &below_cell)| match (above_cell, below_cell) {
                        (Cell::Laser, Cell::Splitter) => 1,
                        _ => 0,
                    },
                )
                .sum::<usize>()
        })
        .sum()
}

fn solve(grid: &[Vec<Cell>], map: &mut HashMap<(u8, u8), usize>, (row, column): (u8, u8)) -> usize {
    match map.get(&(row, column)) {
        Some(value) => *value,
        None => {
            let value = if let Some(cells) = grid.get(row as usize) {
                let new_row = row + 1;
                if matches!(cells[column as usize], Cell::Splitter) {
                    solve(grid, map, (new_row, column - 1))
                        + solve(grid, map, (new_row, column + 1))
                } else {
                    solve(grid, map, (new_row, column))
                }
            } else {
                1
            };
            map.insert((row, column), value);
            value
        }
    }
}
fn sim_grid<'a>(
    grid: &'a [Vec<Cell>],
    (mut row, column): (u8, u8),
    new_beam_positions: &mut Vec<(u8, u8)>,
) {
    while (row as usize) < grid.len() {
        let mut left_beam = None;
        let mut right_beam = None;
        let new_row = {
            let new_row = row + 1;
            if let Some(cells) = grid.get(new_row as usize) {
                if matches!(cells[column as usize], Cell::Splitter) {
                    let left_col = column - 1;
                    let right_col = column + 1;
                    let left_pos = (new_row, left_col);
                    let right_pos = (new_row, right_col);
                    left_beam = cells.get(left_col as usize).map(|_| left_pos);
                    right_beam = cells.get(right_col as usize).map(|_| right_pos);
                    None
                } else {
                    Some(new_row)
                }
            } else {
                None
            }
        };
        match (left_beam, right_beam) {
            (None, None) => (),
            (Some(pos), None) | (None, Some(pos)) => {
                new_beam_positions.push(pos);
            }
            (Some(left_pos), Some(right_pos)) => {
                if left_pos == right_pos {
                    new_beam_positions.push(left_pos);
                } else {
                    new_beam_positions.push(left_pos);
                    new_beam_positions.push(right_pos);
                }
            }
        }
        let Some(new_row) = new_row else {
            break;
        };
        row = new_row;
    }
}
fn timeline_solution(grid: &[Vec<Cell>], row: usize, column: usize) -> usize {
    let mut timelines = 0;
    let mut new_positions = vec![(row as u8 + 1, column as u8)];
    while let Some((row, column)) = new_positions.pop() {
        let old_len = new_positions.len();
        sim_grid(grid, (row, column), &mut new_positions);
        let new_len = new_positions.len();
        let equal = old_len == new_len;
        timelines += equal as usize;
    }
    timelines
}
pub fn total_timelines() -> usize {
    let src = read_file();
    let grid = grid(&src);
    let grid = grid.as_slice();
    let (row, column) = grid
        .iter()
        .enumerate()
        .find_map(|(row, cells)| {
            cells
                .iter()
                .position(|c| matches!(c, Cell::Beam))
                .map(|column| (row, column))
        })
        .expect("There should be an 'S' in the grid");

    const OUTPUT_TO_FILE: bool = false;
    let mut output = OUTPUT_TO_FILE.then(|| match File::open("src/results_day_7.txt").ok() {
        Some(file) => file,
        None => File::create("src/results_day_7.txt").expect("This should always work"),
    });
    let mut map = HashMap::new();
    solve(grid, &mut map, (row as u8 + 1, column as u8))
}
