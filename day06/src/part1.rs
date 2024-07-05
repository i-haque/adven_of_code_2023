pub fn process_data(input: &str) -> ([u64; 4], [u64; 4]) {
    let mut raw_data: Vec<u64> = Vec::with_capacity(8);
    for line in input.lines() {
        let data: Vec<&str> = line.split(" ").collect();
        for s in data {
            match s.parse::<u64>() {
                Ok(val) => raw_data.push(val),
                Err(_) => (),
            }
        }
    }

    let mut time: [u64; 4] = [0; 4];
    let mut distance: [u64; 4] = [0; 4];
    for i in 0..4 {
        time[i] = raw_data[i];
        distance[i] = raw_data[i + 4];
    }

    (time, distance)
}
