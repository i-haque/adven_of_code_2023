use std::fs;

mod part1;
mod part2;
mod util;

fn main() {
    let input: String = fs::read_to_string("../input.txt").expect("couldn't read the file");

    // part 1
    let (part1_time, part2_distance) = part1::process_data(&input);
    let mut part1_res: u64 = 1;
    for i in 0..4 {
        part1_res *= util::ways_to_beat_record_distance(part1_time[i], part2_distance[i]);
    }
    println!("part1: {}", part1_res);

    // part 2
    let (part2_time, part2_distance) = part2::process_data(&input);
    let part2_res: u64 = util::ways_to_beat_record_distance(part2_time, part2_distance);
    println!("part2: {}", part2_res);
}
