pub fn ways_to_beat_record_distance(time: u64, record_distance: u64) -> u64 {
    let mut ways: u64 = 0;
    for i in 0..time + 1 {
        let speed: u64 = i;
        let remaining_time: u64 = time - i;
        if speed * remaining_time > record_distance {
            ways += 1;
        }
    }
    ways
}
