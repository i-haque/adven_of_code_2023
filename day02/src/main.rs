use std::fs;

mod balls;
mod game;

use balls::BallCount;
use game::Game;

fn main() {
    let games: String = fs::read_to_string("../input.txt").unwrap();

    let mut result: u32 = 0;
    let mut power_of_game: u32 = 0;

    let balls: BallCount = BallCount::new(12, 13, 14);

    for game in games.lines() {
        let mut game: Game = Game::new(game.to_string());
        let game_id = game.get_game_id();
        if game.is_valid(balls.red, balls.green, balls.blue) {
            result += game_id
        }
        power_of_game += game.get_power();
    }
    println!("Part1: {}", result);
    println!("Part2: {}", power_of_game);
}
