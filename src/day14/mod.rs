use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Write,
    hash::{Hash, Hasher},
    str::FromStr,
};

const ROUND: char = 'O';
const CUBE: char = '#';

pub fn solve() {
    let input = include_str!("./input.txt");
    let mut board: Board = input.parse().unwrap();
    board.tilt_north();

    dbg!(board.get_load());

    let board: Board = input.parse().unwrap();

    let times = 1_000_000_000;

    dbg!(tilt_cycles(board, times));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SparseCell {
    symbol: char,
    row: usize,
    col: usize,
}

impl ToString for SparseCell {
    fn to_string(&self) -> String {
        self.symbol.to_string()
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Board {
    width: usize,
    height: usize,

    rows: Vec<Vec<SparseCell>>,
    columns: Vec<Vec<SparseCell>>,
}

impl<'a> std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = self.rows.iter().flatten().collect::<Vec<_>>();
        f.write_char('\n')?;
        for row_i in 0..self.height {
            for col_i in 0..self.width {
                if let Some(cell) = cells.iter().find(|c| c.row == row_i && c.col == col_i) {
                    f.write_str(&cell.to_string())?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl<'a> FromStr for Board {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.split('\n');
        let height = lines.clone().count();
        let width = lines.clone().next().unwrap().len();

        let mut rows = Vec::new();
        let mut columns = Vec::new();

        for _ in 0..height {
            rows.push(Vec::new());
        }

        for _ in 0..width {
            columns.push(Vec::new());
        }

        for (row_i, line) in lines.enumerate() {
            for (col_i, char) in line.trim().chars().enumerate() {
                if char == ROUND || char == CUBE {
                    let cell = SparseCell {
                        symbol: char,
                        row: row_i,
                        col: col_i,
                    };

                    rows[row_i].push(cell.clone());
                    columns[col_i].push(cell.clone());
                }
            }
        }

        Ok(Board {
            rows,
            columns,
            width,
            height,
        })
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        let rows = vec![Vec::new(); height];
        let columns = vec![Vec::new(); width];

        Board {
            width,
            height,
            rows,
            columns,
        }
    }

    fn add(&mut self, cell: SparseCell) {
        self.rows[cell.row].push(cell.clone());
        self.columns[cell.col].push(cell);
    }

    fn tilt_north(&mut self) -> Self {
        let mut board = Board::new(self.width, self.height);

        for col in self.columns.iter() {
            let mut cur_fall = 0;
            for cell in col.iter() {
                let cur_loc = match cell.symbol {
                    ROUND => cur_fall,
                    CUBE => cell.row,
                    _ => panic!("bad input"),
                };

                let new_cell = SparseCell {
                    symbol: cell.symbol,
                    row: cur_loc,
                    col: cell.col,
                };

                cur_fall = cur_loc + 1;

                board.add(new_cell);
            }
        }

        board
    }

    fn tilt_south(&mut self) -> Self {
        let mut board = Board::new(self.width, self.height);

        for col in self.columns.iter() {
            let mut cur_fall = self.height - 1;
            for cell in col.iter().rev() {
                let cur_loc = match cell.symbol {
                    ROUND => cur_fall,
                    CUBE => cell.row,
                    _ => panic!("bad input"),
                };

                let new_cell = SparseCell {
                    symbol: cell.symbol,
                    row: cur_loc,
                    col: cell.col,
                };

                if cur_loc != 0 {
                    cur_fall = cur_loc - 1;
                }
                board.add(new_cell);
            }
        }

        board
    }

    fn tilt_west(&mut self) -> Self {
        let mut board = Board::new(self.width, self.height);

        for row in self.rows.iter() {
            let mut cur_fall = 0;
            for cell in row.iter() {
                let cur_loc = match cell.symbol {
                    ROUND => cur_fall,
                    CUBE => cell.col,
                    _ => panic!("bad input"),
                };

                let new_cell = SparseCell {
                    symbol: cell.symbol,
                    row: cell.row,
                    col: cur_loc,
                };

                cur_fall = cur_loc + 1;

                board.add(new_cell);
            }
        }

        board
    }

    fn tilt_east(&mut self) -> Self {
        let mut board = Board::new(self.width, self.height);

        for row in self.rows.iter() {
            let mut cur_fall = self.width - 1;
            for cell in row.iter().rev() {
                let cur_loc = match cell.symbol {
                    ROUND => cur_fall,
                    CUBE => cell.col,
                    _ => panic!("bad input"),
                };

                let new_cell = SparseCell {
                    symbol: cell.symbol,
                    row: cell.row,
                    col: cur_loc,
                };

                if cur_loc != 0 {
                    cur_fall = cur_loc - 1;
                }
                board.add(new_cell);
            }
        }

        board
    }

    pub fn get_load(&self) -> u32 {
        let cells = self.rows.iter().flatten().collect::<Vec<_>>();

        cells
            .iter()
            .filter(|cell| cell.symbol == ROUND)
            .map(|cell| (self.height - cell.row) as u32)
            .sum()
    }
}

fn tilt_cycles(mut board: Board, times: usize) -> u32 {
    let mut boards = HashMap::new();
    let mut loads: Vec<u32> = Vec::new();

    // .... (cycle start ... cycle end)(...)(...)...(..times
    // .....A................B
    // matching board to {times} cycles = (times - A) % (B - A)
    for i in 0..times / 4 {
        board = board.tilt_north();
        board = board.tilt_west();
        board = board.tilt_south();
        board = board.tilt_east();

        let mut hasher = DefaultHasher::new();
        board.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some(cycle_i) = boards.get(&hash) {
            let cycle_len = i - cycle_i;
            let last_cycle_reminder = (times - cycle_i) % (cycle_len);
            return loads
                .get(cycle_i + last_cycle_reminder as usize - 1)
                .unwrap()
                .clone();
        }

        boards.insert(hash, i);
        loads.push(board.get_load());
    }

    board.get_load()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn board_from_str_woks() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let board: Board = input.parse().unwrap();

        dbg!(board);
    }

    #[test]
    fn board_get_load_works() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut board: Board = input.parse().unwrap();
        dbg!(&board);

        let board = board.tilt_north();

        dbg!(&board);
        assert_eq!(board.get_load(), 136);
    }

    #[test]
    fn tilt_cycles_works() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let board: Board = input.parse().unwrap();

        assert_eq!(tilt_cycles(board, 1_000_000_000), 64);
    }
}
