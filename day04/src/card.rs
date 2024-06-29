use std::collections::HashSet;

pub struct Card {
    pub winning_numbers: HashSet<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn new(card_details: String) -> Self {
        let temp: Vec<&str> = card_details.split(" ").collect();
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        let mut numbers: Vec<u32> = Vec::new();
        let mut partition: bool = false;
        for num in temp {
            if num == "|" {
                partition = true;
            }
            match num.parse::<u32>() {
                Ok(n) => {
                    if !partition {
                        winning_numbers.insert(n);
                    } else {
                        numbers.push(n);
                    }
                }
                Err(_) => (),
            }
        }
        Self {
            winning_numbers,
            numbers,
        }
    }

    pub fn calculate_points(&self) -> u32 {
        let mut points: u32 = 0;
        for num in &self.numbers {
            if self.winning_numbers.contains(num) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }

    pub fn win_count(&self) -> u32 {
        let mut count: u32 = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1;
            }
        }
        count
    }
}
