use core::num;
use std::collections::HashMap;

fn read_file() -> String {
    std::fs::read_to_string("src/day_6_input.txt").expect("It should be here")
}

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}
#[derive(Debug)]
struct Column<T> {
    data: Vec<T>,
    op: Operation,
}

fn columns(src: &str) -> Vec<Column<Vec<Option<usize>>>> {
    let last_line = src.lines().last().expect("Should have a last line");

    let (ops, indices): (Vec<_>, Vec<_>) = last_line
        .trim()
        .char_indices()
        .filter_map(|(i, c)| match c {
            '*' => Some((Operation::Multiply, i)),
            '+' => Some((Operation::Add, i)),
            _ => None,
        })
        .unzip();

    let line_count = src.lines().count();
    let rows = src
        .lines()
        .take(line_count - 1)
        .map(|line| line.trim_matches(|c| c == '\r' || c == '\n'))
        .map(|line| {
            let mut prev_index = None;
            let mut rows = Vec::new();
            for i in 0..=indices.len() {
                if let Some(prev) = prev_index {
                    if let Some(i) = indices.get(i).copied() {
                        rows.push(&line[prev..i - 1]);
                    } else {
                        rows.push(&line[prev..]);
                    }
                }
                prev_index = indices.get(i).copied();
            }
            rows
        })
        .collect::<Vec<_>>();
    let mut columns = ops
        .into_iter()
        .map(|op| Column {
            data: Vec::new(),
            op,
        })
        .collect::<Vec<_>>();
    for row in rows {
        for (i, txt) in row.into_iter().enumerate() {
            columns[i].data.push(
                txt.chars()
                    .map(|c| match c {
                        ' ' => None,
                        c => c.to_digit(10).map(|x| x as usize),
                    })
                    .collect(),
            );
        }
    }
    columns
}
pub fn grand_total() -> usize {
    let src = read_file();
    let columns = columns(&src);
    let mut new_columns = Vec::new();
    for column in columns {
        let Column { data, op } = column;

        let mut new_column = vec![0; data.len()];
        for digits in data {
            for (i, digit) in digits.iter().copied().rev().enumerate() {
                if let Some(digit) = digit {
                    new_column[i] *= 10;
                    new_column[i] += digit;
                }
            }
        }
        new_column.retain_mut(|x| *x != 0);
        new_columns.push(Column {
            data: new_column,
            op,
        });
    }
    println!("{:?}", new_columns);
    new_columns
        .into_iter()
        .map(|column| match column.op {
            Operation::Add => column.data.iter().sum::<usize>(),
            Operation::Multiply => column.data.iter().product(),
        })
        .sum()
}
