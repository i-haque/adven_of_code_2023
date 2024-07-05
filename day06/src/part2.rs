pub fn process_data(input: &str) -> (u64, u64) {
    let mut raw_data: Vec<u64> = Vec::with_capacity(2);
    for line in input.lines() {
        let mut data: String = String::new();
        for ch in line.chars() {
            if ch.is_digit(10) {
                data.push(ch);
            }
        }
        raw_data.push(data.parse::<u64>().unwrap());
    }
    (raw_data[0], raw_data[1])
}
