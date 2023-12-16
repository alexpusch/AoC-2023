use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("./input.txt");
    let board = Board::from_str(input);

    let res = simulate(
        &board,
        Ray {
            x: 0,
            y: 0,
            dir: Dir::Right,
        },
    );
    dbg!(res);

    let res = simulate_all(&board);
    dbg!(res);
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    RightSlash,         // \
    LeftSlash,          // /
    VerticalSplitter,   // -
    HorizontalSplitter, // |
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '\\' => Tile::LeftSlash,
            '/' => Tile::RightSlash,
            '-' => Tile::HorizontalSplitter,
            '|' => Tile::VerticalSplitter,
            _ => panic!("bad input {c}"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl Board {
    fn from_str(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|l| l.trim().chars().map(Tile::from_char).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let height = tiles.len();
        let width = tiles[0].len();

        Self {
            tiles,
            height,
            width,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[y][x]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Ray {
    x: usize,
    y: usize,
    dir: Dir,
}

fn step(board: &Board, ray: Ray) -> Vec<Ray> {
    let tile = board.get_tile(ray.x, ray.y);

    let next_dirs = match tile {
        Tile::Empty => vec![ray.dir],
        Tile::LeftSlash => match ray.dir {
            // \
            Dir::Up => vec![Dir::Left],
            Dir::Left => vec![Dir::Up],
            Dir::Down => vec![Dir::Right],
            Dir::Right => vec![Dir::Down],
        },
        Tile::RightSlash => match ray.dir {
            // /
            Dir::Up => vec![Dir::Right],
            Dir::Left => vec![Dir::Down],
            Dir::Down => vec![Dir::Left],
            Dir::Right => vec![Dir::Up],
        },
        Tile::VerticalSplitter => match ray.dir {
            // |
            Dir::Up | Dir::Down => vec![ray.dir],
            Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
        },
        Tile::HorizontalSplitter => match ray.dir {
            // -
            Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
            Dir::Left | Dir::Right => vec![ray.dir],
        },
    };

    next_dirs
        .iter()
        .filter_map(|dir| match dir {
            Dir::Up if ray.y > 0 => Some(Ray {
                x: ray.x,
                y: ray.y - 1,
                dir: *dir,
            }),
            Dir::Left if ray.x > 0 => Some(Ray {
                x: ray.x - 1,
                y: ray.y,
                dir: *dir,
            }),
            Dir::Down if ray.y < board.height - 1 => Some(Ray {
                x: ray.x,
                y: ray.y + 1,
                dir: *dir,
            }),
            Dir::Right if ray.x < board.width - 1 => Some(Ray {
                x: ray.x + 1,
                y: ray.y,
                dir: *dir,
            }),
            _ => None,
        })
        .collect()
}

fn debug(board: &Board, ray: &Ray) {
    for (row_i, row) in board.tiles.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            let c = if ray.x == col_i && ray.y == row_i {
                match ray.dir {
                    Dir::Up => '^',
                    Dir::Left => '<',
                    Dir::Down => 'V',
                    Dir::Right => '>',
                }
            } else {
                match tile {
                    Tile::Empty => '.',
                    Tile::RightSlash => '/',
                    Tile::LeftSlash => '\\',
                    Tile::VerticalSplitter => '|',
                    Tile::HorizontalSplitter => '-',
                }
            };
            print!("{}", c);
        }
        println!();
    }
}

fn simulate_all(board: &Board) -> usize {
    let mut scores = Vec::new();

    for x in 0..board.width {
        let score = simulate(
            &board,
            Ray {
                x,
                y: 0,
                dir: Dir::Down,
            },
        );

        scores.push(score);

        let score = simulate(
            &board,
            Ray {
                x,
                y: board.height - 1,
                dir: Dir::Up,
            },
        );

        scores.push(score);
    }

    for y in 0..board.height {
        let score = simulate(
            &board,
            Ray {
                x: 0,
                y: y,
                dir: Dir::Right,
            },
        );

        scores.push(score);

        let score = simulate(
            &board,
            Ray {
                x: board.width - 1,
                y: y,
                dir: Dir::Left,
            },
        );

        scores.push(score);
    }

    *scores.iter().max().unwrap()
}

fn simulate(board: &Board, start_ray: Ray) -> usize {
    let mut visited = HashSet::new();

    let mut rays = vec![start_ray];

    while let Some(ray) = rays.pop() {
        if visited.contains(&ray) {
            continue;
        };

        visited.insert(ray.clone());

        // debug(&board, &ray);
        let next_rays = step(&board, ray);
        for next in next_rays {
            rays.push(next);
        }
    }

    visited
        .iter()
        .map(|r| (r.x, r.y))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_from_str_works() {
        let input = ".|.\\
.|-/";

        let board = Board::from_str(input);

        assert_eq!(
            board,
            Board {
                width: 4,
                height: 2,
                tiles: vec![
                    vec![
                        Tile::Empty,
                        Tile::VerticalSplitter,
                        Tile::Empty,
                        Tile::LeftSlash
                    ],
                    vec![
                        Tile::Empty,
                        Tile::VerticalSplitter,
                        Tile::HorizontalSplitter,
                        Tile::RightSlash
                    ]
                ]
            }
        )
    }

    #[test]
    fn simulate_works() {
        let input = ".|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....";

        let board = Board::from_str(input);

        let res = simulate(
            &board,
            Ray {
                x: 0,
                y: 0,
                dir: Dir::Right,
            },
        );

        assert_eq!(res, 46);
    }
    #[test]

    fn simulate_all_works() {
        let input = ".|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|....";

        let board = Board::from_str(input);

        let res = simulate_all(&board);

        assert_eq!(res, 51);
    }
}
