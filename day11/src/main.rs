use std::fs;

mod cosmos;
mod part1;

fn main() {
    let input_string: String = fs::read_to_string("./input.txt").expect("couldn't read file!");

    let cosmos_grid: Vec<Vec<char>> = cosmos::get_cosmos(input_string);

    let (expanded_galaxy, galaxies) = cosmos::expand_galaxy(&cosmos_grid);

    // part 1
    let result1: u32 = part1::calculate_distance_between_galaxies(&expanded_galaxy, galaxies);
    println!("{}", result1);

    // part 2
}
