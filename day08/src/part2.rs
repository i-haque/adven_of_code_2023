// use std::collections::{HashMap, VecDeque};

// pub fn ghost_steps(directions: &Vec<char>, map: &HashMap<String, [String; 2]>) -> u32 {
//     let mut q: VecDeque<String> = VecDeque::new();
//     let mut arr: Vec<String> = Vec::new();
//     for (key, _) in map {
//         if key.ends_with("A") {
//             q.push_back(key.to_owned());
//         }
//         if key.ends_with("Z") {
//             arr.push(key.to_string());
//         }
//     }

//     println!("{:?}", q);
//     println!("{:?}", arr);

//     let size: usize = directions.len();
//     let mut index: usize = 0;
//     let mut steps: u32 = 0;

//     while !q.is_empty() {
//         let n: usize = q.len();
//         let mut all_ends_with_z: bool = true;
//         for i in 0..n {
//             if !q[i].ends_with("Z") {
//                 all_ends_with_z = false;
//                 break;
//             }
//         }
//         if all_ends_with_z {
//             return steps;
//         }
//         for _ in 0..n {
//             let key: String = q.pop_front().unwrap();
//             let val: &[String; 2] = map.get(&key).unwrap();
//             if directions[index] == 'L' {
//                 q.push_back(val[0].to_string());
//             } else {
//                 q.push_back(val[1].to_string());
//             }
//         }
//         index = (index + 1) % size;
//         steps += 1;
//     }

//     steps
// }
