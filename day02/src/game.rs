use std::cmp;

pub struct Game {
    pub game_string: String,
    pub game_sets_string: String,
}

impl Game {
    pub fn new(game: String) -> Self {
        let temp: Vec<&str> = game.split(": ").collect();
        Self {
            game_string: temp[0].to_owned(),
            game_sets_string: temp[1].to_owned(),
        }
    }

    pub fn get_game_id(&mut self) -> u32 {
        let game_data: Vec<&str> = self.game_string.split(" ").collect();
        game_data[1].parse::<u32>().unwrap()
    }

    pub fn is_valid(
        &mut self,
        red_ball_count: u32,
        green_ball_count: u32,
        blue_ball_count: u32,
    ) -> bool {
        let mut flag: bool = true;
        let game_sets: Vec<&str> = self.game_sets_string.split("; ").collect();
        'outer: for game_set in game_sets {
            let rounds: Vec<&str> = game_set.split(", ").collect();
            for round in rounds {
                let game_data: Vec<&str> = round.split(" ").collect();
                if game_data[1] == "red" && game_data[0].parse::<u32>().unwrap() > red_ball_count {
                    flag = false;
                    break 'outer;
                } else if game_data[1] == "green"
                    && game_data[0].parse::<u32>().unwrap() > green_ball_count
                {
                    flag = false;
                    break 'outer;
                } else if game_data[1] == "blue"
                    && game_data[0].parse::<u32>().unwrap() > blue_ball_count
                {
                    flag = false;
                    break 'outer;
                }
            }
        }
        flag
    }

    pub fn get_power(&mut self) -> u32 {
        let mut red_count: u32 = 0;
        let mut green_count: u32 = 0;
        let mut blue_count: u32 = 0;
        let game_sets: Vec<&str> = self.game_sets_string.split("; ").collect();
        for game_set in game_sets {
            let rounds: Vec<&str> = game_set.split(", ").collect();
            for round in rounds {
                let game_data: Vec<&str> = round.split(" ").collect();
                if game_data[1] == "red" {
                    red_count = cmp::max(red_count, game_data[0].parse::<u32>().unwrap());
                } else if game_data[1] == "green" {
                    green_count = cmp::max(green_count, game_data[0].parse::<u32>().unwrap());
                } else {
                    blue_count = cmp::max(blue_count, game_data[0].parse::<u32>().unwrap());
                }
            }
        }
        red_count * green_count * blue_count
    }
}
