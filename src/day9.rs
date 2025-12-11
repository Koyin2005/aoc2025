use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
    fs::File,
};

fn read_file() -> String {
    std::fs::read_to_string("src/day_9_input.txt").expect("Day 9 input should be here")
}

fn red_tiles(src: String) -> Vec<(usize, usize)> {
    src.lines()
        .filter_map(|line| {
            let mut iter = line
                .split(",")
                .filter_map(|number| number.parse::<usize>().ok());
            let (Some(x), Some(y)) = (iter.next(), iter.next()) else {
                return None;
            };
            Some((x, y))
        })
        .collect()
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    NoColor,
    Red,
    Green,
}
pub fn largest_area() -> usize {
    let tile_positions = red_tiles(read_file());

    let pairs = {
        let mut pairs = Vec::new();
        let mut prev_pos = None;
        for &pos in tile_positions.iter() {
            if let Some(prev_pos) = prev_pos {
                pairs.push((prev_pos, pos));
            }
            prev_pos = Some(pos);
        }
        if let Some(pos) = prev_pos {
            pairs.push((pos, tile_positions[0]));
        }
        pairs
    };

    let max_x = tile_positions.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = tile_positions.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = tile_positions.iter().map(|&(_, y)| y).max().unwrap();

    fn print_grid(grid: &Vec<Vec<Tile>>) {
        use std::io::Write;
        let mut output = std::io::stdout();
        for row in grid {
            for tile in row {
                write!(
                    &mut output,
                    "{}",
                    match tile {
                        Tile::Green => "X",
                        Tile::NoColor => ".",
                        Tile::Red => "#",
                    }
                )
                .unwrap();
            }
            writeln!(&mut output).unwrap();
        }
    }
    let (grid, greens) = {
        let mut greens = BTreeMap::new();
        let mut grid = (0..=max_y)
            .map(|_| vec![Tile::NoColor; max_x + 1])
            .collect::<Vec<_>>();

        for &(x, y) in tile_positions.iter() {
            grid[y][x] = Tile::Red;
        }
        for ((prev_x, prev_y), (curr_x, curr_y)) in pairs {
            if prev_y == curr_y {
                let (start, end) = if prev_x < curr_x {
                    (prev_x, curr_x)
                } else {
                    (curr_x, prev_x)
                };
                for x in start + 1..end {
                    grid[curr_y][x] = Tile::Green;
                }
            } else if prev_x == curr_x {
                let (start, end) = if prev_y < curr_y {
                    (prev_y, curr_y)
                } else {
                    (curr_y, prev_y)
                };
                for y in start + 1..end {
                    grid[y][curr_x] = Tile::Green;
                }
            }
        }
        for (row, row_index) in grid[min_y..=max_y].as_mut().iter_mut().zip(min_y..=max_y) {
            let Some(min_x) = row.iter().position(|tile| *tile != Tile::NoColor) else {
                continue;
            };
            let Some(max_x) = row.iter().rposition(|tile| *tile != Tile::NoColor) else {
                continue;
            };

            let _ = greens.entry(row_index).or_insert(None).insert(min_x..max_x);
            row[min_x + 1..max_x].iter_mut().for_each(|tile| {
                if *tile != Tile::Red {
                    *tile = Tile::Green;
                }
            });
        }
        (grid, greens)
    };
    //print_grid(&grid);
    println!("GRID DONE");
    let mut max_area = 0;
    for (i, &(x_i, y_i)) in tile_positions.iter().enumerate() {
        for (j, &(x_j, y_j)) in tile_positions.iter().enumerate() {
            if i == j {
                continue;
            }
            let x_diff = x_i.abs_diff(x_j) + 1;
            let y_diff = y_i.abs_diff(y_j) + 1;
            let area = x_diff * y_diff;
            if area <= max_area {
                continue;
            }
            let y_min = y_i.min(y_j);
            let y_max = y_i.max(y_j);
            let x_min = x_i.min(x_j);
            let x_max = x_i.max(x_j);

            //Check four corners for emptiness
            if grid[y_min][x_min] == Tile::NoColor
                || grid[y_min][x_max] == Tile::NoColor
                || grid[y_max][x_min] == Tile::NoColor
                || grid[y_max][x_max] == Tile::NoColor
            {
                continue;
            }

            if !(y_min..y_max)
                .filter_map(|y| greens.get(&y).cloned().and_then(std::convert::identity))
                .all(|r| r.contains(&x_min) && r.contains(&x_max))
            {
                continue;
            }
            max_area = area;
        }
    }
    max_area
}
