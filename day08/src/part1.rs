use std::collections::HashMap;

pub fn calculate_steps(directions: &Vec<char>, map: &HashMap<String, [String; 2]>) -> u32 {
    let size: usize = directions.len();
    let mut index: usize = 0;
    let mut steps: u32 = 0;

    let mut key: String = String::from("AAA");
    while key != String::from("ZZZ") {
        let val: &[String; 2] = map.get(&key).unwrap();
        if directions[index] == 'L' {
            key = val[0].to_owned();
        } else {
            key = val[1].to_owned();
        }
        index = (index + 1) % size;
        steps += 1;
    }
    steps
}
