use std::cmp::Ordering;

fn read_file() -> String {
    std::fs::read_to_string("src/day_4_input.txt").expect("Should be ther")
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    HasPaper,
    IsEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
type Grid = Vec<Vec<Cell>>;
fn into_grid(src: &str) -> Grid {
    src.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '@' => Cell::HasPaper,
                    _ => Cell::IsEmpty,
                })
                .collect()
        })
        .collect()
}

fn adjacent(column: usize, row: usize, grid: &Grid) -> Vec<(Cell, Direction)> {
    let min_row = row.checked_sub(1).unwrap_or(0);
    let max_row = if row + 1 < grid.len() {
        row + 1
    } else {
        grid.len() - 1
    };
    let mut cells = Vec::new();
    for curr_row in min_row..=max_row {
        let cells_in_row = &grid[curr_row];
        let min_column = column.checked_sub(1).unwrap_or(0);
        let max_column = if column + 1 < cells_in_row.len() {
            column + 1
        } else {
            cells_in_row.len() - 1
        };
        for curr_col in min_column..=max_column {
            let cell = cells_in_row[curr_col];
            cells.push((
                cell,
                match (curr_col.cmp(&column), curr_row.cmp(&row)) {
                    (Ordering::Equal, Ordering::Equal) => continue,
                    (Ordering::Less, Ordering::Equal) => Direction::Left,
                    (Ordering::Greater, Ordering::Equal) => Direction::Right,
                    (Ordering::Equal, Ordering::Less) => Direction::Up,
                    (Ordering::Equal, Ordering::Greater) => Direction::Down,
                    (Ordering::Less, Ordering::Less) => Direction::UpLeft,
                    (Ordering::Less, Ordering::Greater) => Direction::DownLeft,
                    (Ordering::Greater, Ordering::Less) => Direction::UpRight,
                    (Ordering::Greater, Ordering::Greater) => Direction::DownRight,
                },
            ));
        }
    }
    cells
}

pub fn rolls_of_paper_repetitive() -> usize {
    let src = read_file();
    let mut grid = into_grid(&src);

    let mut touch_count = 0;
    loop {
        let mut removable = Vec::new();
        for (row, cells) in grid.iter().enumerate() {
            for (column, &cell) in cells.iter().enumerate() {
                let Cell::HasPaper = cell else {
                    continue;
                };
                let adjacent = adjacent(column, row, &grid);
                let with_paper: usize = adjacent
                    .iter()
                    .map(|(cell, _)| matches!(cell, Cell::HasPaper) as usize)
                    .sum();
                if with_paper < 4 {
                    removable.push((column, row));
                    touch_count += 1;
                }
            }
        }
        if removable.is_empty() {
            break;
        }
        for (col, row) in removable {
            grid[row][col] = Cell::IsEmpty;
        }
    }
    touch_count
}

pub fn rolls_of_paper() -> usize {
    let src = read_file();
    let grid = into_grid(&src);
    let mut touch_count = 0;
    for (row, cells) in grid.iter().enumerate() {
        for (column, &cell) in cells.iter().enumerate() {
            let Cell::HasPaper = cell else {
                continue;
            };
            let adjacent = adjacent(column, row, &grid);
            let with_paper: usize = adjacent
                .iter()
                .map(|(cell, _)| matches!(cell, Cell::HasPaper) as usize)
                .sum();
            if with_paper < 4 {
                touch_count += 1;
            }
        }
    }
    touch_count
}
