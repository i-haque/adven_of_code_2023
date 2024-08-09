use std::fs;

mod pattern;
use pattern::Pattern;

fn main() {
    let input_string: String = fs::read_to_string("./input.txt").expect("couldn't read file!");

    let mut total_patterns: u32 = 1;
    for line in input_string.lines() {
        if line.len() == 0 {
            total_patterns += 1;
        }
    }

    let mut patterns: Vec<Vec<Vec<char>>> = vec![vec![]; total_patterns as usize];
    let mut i: usize = 0;
    for line in input_string.lines() {
        if line.len() == 0 {
            i += 1;
        } else {
            patterns[i].push(line.chars().collect());
        }
    }

    // part1
    let mut row_sum: u32 = 0;
    let mut col_sum: u32 = 0;
    for pattern in patterns {
        let p: Pattern = Pattern::new(pattern);
        let (mirror_row, mirror_col) = p.summarize();
        if mirror_row != -1 {
            row_sum += mirror_row as u32;
        }
        if mirror_col != -1 {
            col_sum += mirror_col as u32;
        }
    }
    println!("part1: {}", row_sum * 100 + col_sum);

    // part 2
}
