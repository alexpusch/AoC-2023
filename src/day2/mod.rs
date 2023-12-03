use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");
    let rules = GameRule {
        max_reds: 12,
        max_greens: 13,
        max_blues: 14,
    };

    let part1 = get_possible_games(input, rules);
    dbg!(part1);

    let part2 = get_games_power(input);
    dbg!(part2);
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    reveals: Vec<BallSet>,
}

impl Game {
    fn parse_str(input: &str) -> Self {
        let regex = Regex::new(r"Game (\d+): (.*)").unwrap();
        let parts = regex.captures(input).unwrap();

        let game_id: u32 = parts.get(1).unwrap().as_str().parse().unwrap();
        let details = parts.get(2).unwrap().as_str();
        let reveals = details
            .split(";")
            .map(BallSet::parse_str)
            .collect::<Vec<_>>();

        Game {
            id: game_id,
            reveals,
        }
    }

    fn get_minimal_set(&self) -> BallSet {
        let min_reds = self.reveals.iter().map(|r| r.reds).max().unwrap_or(0);
        let min_blues = self.reveals.iter().map(|r| r.blues).max().unwrap_or(0);
        let min_greens = self.reveals.iter().map(|r| r.greens).max().unwrap_or(0);

        BallSet {
            reds: min_reds,
            blues: min_blues,
            greens: min_greens,
        }
    }
}

#[derive(PartialEq, Debug)]
struct BallSet {
    reds: u32,
    blues: u32,
    greens: u32,
}

impl BallSet {
    fn parse_str(input: &str) -> Self {
        let blues_regex = Regex::new(r"(\d+) blue").unwrap();
        let blues = blues_regex
            .captures(input)
            .and_then(|c| c.get(1))
            .and_then(|d| d.as_str().parse::<u32>().ok())
            .unwrap_or(0);

        let reds_regex = Regex::new(r"(\d+) red").unwrap();
        let reds = reds_regex
            .captures(input)
            .and_then(|c| c.get(1))
            .and_then(|d| d.as_str().parse::<u32>().ok())
            .unwrap_or(0);

        let greens_regex = Regex::new(r"(\d+) green").unwrap();
        let greens = greens_regex
            .captures(input)
            .and_then(|c| c.get(1))
            .and_then(|d| d.as_str().parse::<u32>().ok())
            .unwrap_or(0);

        BallSet {
            blues,
            reds,
            greens,
        }
    }

    fn get_power(&self) -> u32 {
        self.reds * self.greens * self.blues
    }
}

struct GameRule {
    max_reds: u32,
    max_blues: u32,
    max_greens: u32,
}

impl GameRule {
    fn is_game_possible(&self, game: &Game) -> bool {
        game.reveals.iter().all(|r| {
            r.reds <= self.max_reds && r.greens <= self.max_greens && r.blues <= self.max_blues
        })
    }
}

fn parse_games(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.split("\n").map(Game::parse_str)
}

fn get_possible_games(input: &str, game_rules: GameRule) -> u32 {
    let games = parse_games(input);

    games
        .filter(|game| game_rules.is_game_possible(&game))
        .map(|game| game.id)
        .sum()
}

fn get_games_power(input: &str) -> u32 {
    let games = parse_games(input);

    games.map(|game| game.get_minimal_set().get_power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_games_valid_input_returns_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let actual = Game::parse_str(input);

        assert_eq!(
            actual,
            Game {
                id: 1,
                reveals: vec![
                    BallSet {
                        blues: 3,
                        reds: 4,
                        greens: 0
                    },
                    BallSet {
                        blues: 6,
                        reds: 1,
                        greens: 2
                    },
                    BallSet {
                        blues: 0,
                        reds: 0,
                        greens: 2
                    }
                ]
            }
        );
    }

    #[test]
    fn game_get_minimal_set() {
        let game = Game {
            id: 1,
            reveals: vec![
                BallSet {
                    blues: 3,
                    reds: 4,
                    greens: 0,
                },
                BallSet {
                    blues: 6,
                    reds: 1,
                    greens: 2,
                },
                BallSet {
                    blues: 0,
                    reds: 0,
                    greens: 2,
                },
            ],
        };

        let min_set = game.get_minimal_set();

        assert_eq!(
            min_set,
            BallSet {
                reds: 4,
                greens: 2,
                blues: 6
            }
        )
    }

    #[test]
    fn part1_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let rules = GameRule {
            max_reds: 12,
            max_greens: 13,
            max_blues: 14,
        };

        assert_eq!(get_possible_games(input, rules), 8);
    }

    #[test]
    fn part2_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(get_games_power(input), 2286);
    }
}
