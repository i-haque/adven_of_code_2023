use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("./input.txt").unwrap();
    let mut res: u32 = 0;

    let mut map: HashMap<&str, char> = HashMap::with_capacity(9);
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');

    for line in data.lines() {
        res += util(line, &map);
    }
    println!("{}", res);
}

fn util(string: &str, map: &HashMap<&str, char>) -> u32 {
    let string_chars: Vec<char> = string.chars().collect();
    let mut num_string: String = String::new();

    // looking for 1st digit

    'outer: for i in 0..string.len() {
        if string_chars[i].is_digit(10) {
            num_string.push(string_chars[i]);
            break 'outer;
        }
        for (k, v) in map {
            let length = k.len();
            if (i + length) <= string.len() && &&string[i..(i + length)] == k {
                num_string.push(*v);
                break 'outer;
            }
        }
    }

    // looking for 2nd digit

    'outer: for i in (0..string.len()).rev() {
        if string_chars[i].is_digit(10) {
            num_string.push(string_chars[i]);
            break 'outer;
        }
        for (k, v) in map {
            let length = k.len();
            let lower_limit: usize = if (i as i32 - length as i32) <= 0 {
                0
            } else {
                i - length + 1
            };
            if &&string[lower_limit..(i + 1)] == k {
                num_string.push(*v);
                break 'outer;
            }
        }
    }

    if num_string.len() == 0 {
        return 0;
    }
    num_string.parse::<u32>().unwrap()
}
