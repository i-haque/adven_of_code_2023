use std::collections::HashSet;

pub fn get_cosmos(input: String) -> Vec<Vec<char>> {
    let mut cosmos_grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let line_as_vec: Vec<char> = line.chars().collect();
        cosmos_grid.push(line_as_vec);
    }
    cosmos_grid
}

pub fn expand_galaxy(cosmos_grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, u32) {
    let mut empty_rows: HashSet<usize> = HashSet::new();
    for i in 0..cosmos_grid.len() {
        let mut galaxy_found: bool = false;
        for j in 0..cosmos_grid[0].len() {
            if cosmos_grid[i][j] == '#' {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            empty_rows.insert(i);
        }
    }

    let mut empty_cols: HashSet<usize> = HashSet::new();
    for j in 0..cosmos_grid[0].len() {
        let mut galaxy_found: bool = false;
        for i in 0..cosmos_grid.len() {
            if cosmos_grid[i][j] == '#' {
                galaxy_found = true;
                break;
            }
        }
        if !galaxy_found {
            empty_cols.insert(j);
        }
    }

    // expanding galaxy
    let mut expanded_galaxy: Vec<Vec<char>> = Vec::new();
    let mut galaxies: u32 = 0;
    for i in 0..cosmos_grid.len() {
        let mut row: Vec<char> = Vec::new();
        for j in 0..cosmos_grid[0].len() {
            if cosmos_grid[i][j] == '#' {
                galaxies += 1;
            }
            row.push(cosmos_grid[i][j]);
            if empty_cols.contains(&j) {
                row.push(cosmos_grid[i][j]);
            }
        }
        expanded_galaxy.push(row.clone());
        if empty_rows.contains(&i) {
            expanded_galaxy.push(row);
        }
    }
    (expanded_galaxy, galaxies)
}
