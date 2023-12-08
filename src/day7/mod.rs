use std::{char, cmp::Ordering, collections::HashMap};

pub fn solve() {
    let input = include_str!("./input.txt");
    // let res = get_score(input);

    // dbg!(res);

    let res = get_score_jokers(input);

    dbg!(res);
}

fn parse(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            let bid = parts[1].parse::<u32>().unwrap();

            (Hand::from_str(parts[0]), bid)
        })
        .collect()
}

fn parse_jokers(input: &str) -> Vec<(HandJoker, u32)> {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" ").collect::<Vec<_>>();
            let bid = parts[1].parse::<u32>().unwrap();

            (HandJoker::from_str(parts[0]), bid)
        })
        .collect()
}

fn get_score(input: &str) -> u32 {
    let mut hands_and_bids = parse(input);

    hands_and_bids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    hands_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (hand, bid))| acc + bid * (i + 1) as u32)
}

fn get_score_jokers(input: &str) -> u32 {
    let mut hands_and_bids = parse_jokers(input);

    hands_and_bids.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    dbg!(&hands_and_bids);

    hands_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + bid * (i + 1) as u32)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Ord)]
struct Card(u32);

impl Card {
    fn from_char(c: char) -> Self {
        let value = match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        };

        Card(value)
    }
}

#[derive(Debug, PartialEq)]
struct Hand(Vec<Card>);

impl Hand {
    fn from_str(input: &str) -> Self {
        let cards = input.chars().map(Card::from_char).collect::<Vec<_>>();

        Hand(cards)
    }

    fn to_sets(&self) -> HashMap<Card, u32> {
        self.0.iter().fold(HashMap::new(), |mut acc, cur| {
            acc.entry(cur.clone())
                .and_modify(|c| *c = *c + 1)
                .or_insert(1);

            acc
        })
    }

    fn rank(&self) -> u32 {
        let sets = self.to_sets();

        let mut sorted_sets = sets.iter().collect::<Vec<_>>();
        sorted_sets.sort_by(|a, b| b.1.cmp(&a.1));

        match (sorted_sets.get(0), sorted_sets.get(1), sorted_sets.get(2)) {
            (Some((_, 5)), None, None) => 0,
            (Some((_, 4)), Some((_, 1)), None) => 1,
            (Some((_, 3)), Some((_, 2)), None) => 2,
            (Some((_, 3)), Some((_, 1)), Some((_, 1))) => 3,
            (Some((_, 2)), Some((_, 2)), Some((_, 1))) => 4,
            (Some((_, 2)), Some((_, 1)), Some((_, 1))) => 5,
            _ => 6,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank = self.rank();
        let other_rank = other.rank();

        match rank.cmp(&other_rank) {
            Ordering::Equal => self.0.partial_cmp(&other.0),
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug, PartialEq)]
struct HandJoker(Vec<CardJoker>);

impl HandJoker {
    fn from_str(input: &str) -> Self {
        let cards = input.chars().map(CardJoker::from_char).collect::<Vec<_>>();

        Self(cards)
    }

    fn to_sets(&self) -> HashMap<CardJoker, u32> {
        let mut sets = self.0.iter().fold(HashMap::new(), |mut acc, cur| {
            acc.entry(cur.clone())
                .and_modify(|c| *c = *c + 1)
                .or_insert(1);

            acc
        });

        if let Some(jokers) = sets.get(&CardJoker(0)) {
            if *jokers < 5 {
                let jokers = jokers.clone();
                sets.remove(&CardJoker(0));

                let max_set = sets.iter_mut().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

                *max_set.1 = *max_set.1 + jokers;
            }
        }

        sets
    }

    fn rank(&self) -> u32 {
        let sets = self.to_sets();

        let mut sorted_sets = sets.iter().collect::<Vec<_>>();
        sorted_sets.sort_by(|a, b| b.1.cmp(&a.1));

        // dbg!(&sorted_sets);

        match (sorted_sets.get(0), sorted_sets.get(1), sorted_sets.get(2)) {
            (Some((_, 5)), None, None) => 0,
            (Some((_, 4)), Some((_, 1)), None) => 1,
            (Some((_, 3)), Some((_, 2)), None) => 2,
            (Some((_, 3)), Some((_, 1)), Some((_, 1))) => 3,
            (Some((_, 2)), Some((_, 2)), Some((_, 1))) => 4,
            (Some((_, 2)), Some((_, 1)), Some((_, 1))) => 5,
            _ => 6,
        }
    }
}

impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank = self.rank();
        let other_rank = other.rank();

        match rank.cmp(&other_rank) {
            Ordering::Equal => self.0.partial_cmp(&other.0),
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Less => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Clone, Ord)]
struct CardJoker(u32);

impl CardJoker {
    fn from_char(c: char) -> Self {
        let value = match c {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => c.to_digit(10).unwrap(),
        };

        CardJoker(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_parse_works() {
        let input = "32T3K";

        let expected = Hand(vec![Card(3), Card(2), Card(10), Card(3), Card(13)]);

        assert_eq!(expected, Hand::from_str(input));
    }

    #[test]
    fn hand_cmp_works() {
        let hand0 = Hand::from_str("54321");
        let hand1 = Hand::from_str("64321");
        let hand2 = Hand::from_str("32T3K");
        let hand3 = Hand::from_str("KK677");
        let hand4 = Hand::from_str("T55J5");
        let hand5 = Hand::from_str("12112");
        let hand6 = Hand::from_str("12111");
        let hand7 = Hand::from_str("AAAAA");

        let hand8 = Hand::from_str("KTJJT");
        let hand9 = Hand::from_str("KK677");

        assert!(hand0 < hand1, "high card wins");
        assert!(hand1 < hand2, "pair wins");
        assert!(hand2 < hand3, "two pair wins");
        assert!(hand3 < hand4, "triple wins");
        assert!(hand4 < hand5, "full house");
        assert!(hand5 < hand6, "four of a kind wins");
        assert!(hand6 < hand7, "five wins");
        assert!(hand8 < hand9);
    }

    #[test]
    fn hand_joker_cmp_works() {
        assert!(HandJoker::from_str("AAT12") < HandJoker::from_str("AAJ12"));
        assert!(HandJoker::from_str("AJ523") < HandJoker::from_str("A5523"));
        assert!(HandJoker::from_str("11112") < HandJoker::from_str("JJJJJ"));
        assert!(HandJoker::from_str("AAA22") < HandJoker::from_str("AJJ2A"));
        assert!(HandJoker::from_str("TKAKT") < HandJoker::from_str("TQ77J"));
    }

    #[test]
    fn get_score_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let expected = get_score(input);

        assert_eq!(expected, 6440);
    }

    #[test]
    fn get_score_jokers_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let expected = get_score_jokers(input);

        assert_eq!(expected, 5905);
    }
}
