use std::collections::HashMap;

pub fn parse_input(input_string: String) -> (Vec<char>, HashMap<String, [String; 2]>) {
    let input_vec: Vec<&str> = input_string.split("\n").collect();

    let directions: Vec<char> = input_vec[0].chars().collect();

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for i in 2..input_vec.len() {
        let mut key: String = String::with_capacity(3);
        let mut val1: String = String::with_capacity(3);
        let mut val2: String = String::with_capacity(3);
        for ch in input_vec[i].chars() {
            if char::is_alphabetic(ch) {
                if key.len() < 3 {
                    key.push(ch);
                    continue;
                } else if val1.len() < 3 {
                    val1.push(ch);
                    continue;
                } else if val2.len() < 3 {
                    val2.push(ch);
                    continue;
                }
            }
        }
        map.insert(key, [val1, val2]);
    }
    (directions, map)
}
