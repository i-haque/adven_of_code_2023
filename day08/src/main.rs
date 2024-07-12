use std::fs;

mod part1;
mod part2;
mod util;

fn main() {
    let input_string: String = fs::read_to_string("./input.txt").expect("couldn't read file..");
    let (directions, map) = util::parse_input(input_string);

    let result1: u32 = part1::calculate_steps(&directions, &map);
    println!("part1: {}", result1);

    // part 2 not solved :(
    // let result2: u32 = part2::ghost_steps(&directions, &map);
    // println!("part2: {}", result2);
}
