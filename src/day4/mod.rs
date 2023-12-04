use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");
    let res = get_sum_scores(input);

    dbg!(res);

    let res = part_2(input);
    dbg!(res);
}

#[derive(PartialEq, Debug)]
struct Card {
    id: u32,
    numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

impl Card {
    fn from_str(line: &str) -> Self {
        let card_regex = Regex::new(r"Card\s+(\d+): ([\d ]+) \| ([\d ]+)").unwrap();
        let number_regex = Regex::new(r"(\d+)").unwrap();
        let captures = card_regex.captures(line).unwrap();
        let id = captures
            .get(1)
            .and_then(|d| d.as_str().parse::<u32>().ok())
            .unwrap();

        let numbers = captures.get(3).unwrap().as_str();
        let numbers = number_regex
            .find_iter(numbers)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let winning_numbers = captures.get(2).unwrap().as_str();
        let winning_numbers = number_regex
            .find_iter(winning_numbers)
            .map(|n| n.as_str().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        Card {
            id,
            numbers,
            winning_numbers,
        }
    }

    fn get_winning_numbers_len(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }

    fn get_score(&self) -> u32 {
        let n_winning = self.get_winning_numbers_len();

        if n_winning == 0 {
            0
        } else {
            2u32.pow(n_winning - 1)
        }
    }
}

fn get_sum_scores(input: &str) -> u32 {
    let cards = input.split("\n").map(Card::from_str).collect::<Vec<_>>();

    cards.iter().map(|c| c.get_score()).sum()
}

fn part_2(input: &str) -> u32 {
    let cards = input.split("\n").map(Card::from_str).collect::<Vec<_>>();
    let cards_len = cards.len() as u32;

    let mut copies_by_card = cards.iter().map(|c| (c.id, 1)).collect::<HashMap<_, _>>();

    for card in cards {
        let w = card.get_winning_numbers_len();
        let copies = copies_by_card.get(&card.id).unwrap().clone();

        for _ in 0..copies {
            for next_card_i in (card.id + 1)..(min(card.id + 1 + w, cards_len as u32 + 1)) {
                copies_by_card
                    .entry(next_card_i)
                    .and_modify(|n| *n = *n + 1u32);
            }
        }
    }

    copies_by_card.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_from_str_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let expected = Card {
            id: 1,
            numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
        };

        assert_eq!(expected, Card::from_str(input));
    }

    #[test]
    fn card_get_score() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from_str(input);

        assert_eq!(card.get_score(), 8);
    }

    #[test]
    fn get_sum_scores_woks() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(get_sum_scores(input), 13);
    }

    #[test]
    fn part_2_wth() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_2(input), 30);
    }
}
