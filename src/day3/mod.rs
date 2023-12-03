use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");
    let res = get_part_number_sum(input);

    dbg!(res);
}

fn get_part_number_sum(input: &str) -> u32 {
    let board = parse(input);
    let part_numbers = board.get_part_numbers();

    part_numbers.iter().sum()
}

#[derive(Debug, PartialEq)]
struct Coord(i32, i32);

#[derive(Debug, PartialEq)]
struct NumberPos {
    number: u32,
    start_x: i32,
    end_x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Board {
    symbols: Vec<Coord>,
    numbers: Vec<NumberPos>,
}

impl Board {
    pub fn get_part_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|number_pos| self.is_near_symbol(number_pos))
            .map(|number| number.number)
            .collect()
    }

    pub fn is_near_symbol(&self, number_pos: &NumberPos) -> bool {
        //   ....
        //   .12.
        //   ....

        self.symbols.iter().any(|s| {
            let Coord(s_x, s_y) = s;

            (number_pos.start_x - 1 == *s_x && number_pos.y == *s_y) // symbol on the left
                || (number_pos.end_x + 1 == *s_x && number_pos.y == *s_y) // symbol on the right
                || (number_pos.start_x - 1 <= *s_x && number_pos.end_x + 1 >= *s_x && number_pos.y - 1 == *s_y) // symbol above
                || (number_pos.start_x - 1 <= *s_x && number_pos.end_x + 1 >= *s_x && number_pos.y + 1 == *s_y)
            // symbol below
        })
    }
}

fn parse(input: &str) -> Board {
    let lines = input.split("\n");

    let symbol_regex = Regex::new(r"[^0-9\.]").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();

    let symbols = lines
        .clone()
        .enumerate()
        .flat_map(|(i, l)| {
            symbol_regex
                .find_iter(l)
                .map(move |m| Coord(m.start() as i32, i as i32))
                .collect::<Vec<_>>()
        })
        .collect();

    let part_numbers = lines
        .enumerate()
        .flat_map(|(i, l)| {
            number_regex
                .find_iter(l)
                .map(move |m| NumberPos {
                    number: m.as_str().parse().unwrap(),
                    start_x: m.start() as i32,
                    end_x: m.end() as i32 - 1,
                    y: i as i32,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Board {
        symbols,
        numbers: part_numbers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parses_board() {
        let input = "...*......
..35..633.
......#$..
617*......";

        let expected = Board {
            symbols: vec![Coord(3, 0), Coord(6, 2), Coord(7, 2), Coord(3, 3)],
            numbers: vec![
                NumberPos {
                    number: 35,
                    start_x: 2,
                    end_x: 3,
                    y: 1,
                },
                NumberPos {
                    number: 633,
                    start_x: 6,
                    end_x: 8,
                    y: 1,
                },
                NumberPos {
                    number: 617,
                    start_x: 0,
                    end_x: 2,
                    y: 3,
                },
            ],
        };

        assert_eq!(expected, parse(input));
    }

    #[test]
    fn is_near_symbol_returns_if_near_symbol() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let board = parse(input);
        dbg!(&board);
        assert_eq!(
            board.is_near_symbol(&NumberPos {
                number: 58,
                start_x: 7,
                end_x: 8,
                y: 5
            }),
            false
        );
        assert_eq!(
            board.is_near_symbol(&NumberPos {
                number: 35,
                start_x: 2,
                end_x: 3,
                y: 2
            }),
            true
        );
        assert_eq!(
            board.is_near_symbol(&NumberPos {
                number: 633,
                start_x: 6,
                end_x: 8,
                y: 2
            }),
            true
        );
    }

    #[test]
    fn get_part_numbers_return_numbers() {
        let input = "...*......
..35..633.
......#...
617*......";

        let board = parse(input);
        assert_eq!(board.get_part_numbers(), vec![633, 617]);
    }

    #[test]
    fn get_part_number_sum_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(get_part_number_sum(input), 4361);
    }
}
