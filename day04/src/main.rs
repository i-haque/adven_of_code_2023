mod card;

use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use card::Card;

fn main() {
    let cards: String = fs::read_to_string("../input.txt").unwrap();

    let mut points: u32 = 0;
    let mut card_number: u32 = 1;
    let mut map: HashMap<u32, u32> = HashMap::new();

    for card in cards.lines() {
        let card_game: Card = Card::new(card.to_string());

        // calculate total points
        points += card_game.calculate_points();

        // getting win count for each card game
        let count: u32 = card_game.win_count();
        map.insert(card_number, count);
        card_number += 1;
    }

    // calculate total scratch cards
    let mut total_cards: u32 = 0;
    let mut q: VecDeque<u32> = VecDeque::new();
    for i in 1..card_number {
        q.push_back(i);
    }
    while !q.is_empty() {
        for _ in 0..q.len() {
            let mut num: u32 = q.pop_back().unwrap();
            let freq: u32 = map.get(&num).copied().unwrap();
            total_cards += 1;
            for _ in 0..freq {
                q.push_back(num + 1);
                num += 1;
            }
        }
    }
    println!("Part 1: {}", points);
    println!("Part 2: {}", total_cards);
}
