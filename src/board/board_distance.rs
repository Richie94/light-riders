use std::collections::VecDeque;
use std::fmt;

use board::Board;

#[derive(Clone, Debug)]
pub struct BoardDistance {
    player0: [[u8; 16]; 16],
    player1: [[u8; 16]; 16],
}

impl BoardDistance {
    /// Returns the controlled area by player 0 and player 1
    pub fn controlled_area(&self) -> (u8, u8) {
        self.player0
            .iter()
            .flat_map(|x| x) // In rust 1.29 use flatten
            .zip(self.player1.iter().flat_map(|x| x)) // In rust 1.29 use flatten
            .map(|(d0, d1)| *d1 as i32 - *d0 as i32)
            .filter(|x| *x != 0)
            .fold(
                (0, 0),
                |(p0, p1), x| {
                    if x > 0 {
                        (p0 + 1, p1)
                    } else {
                        (p0, p1 + 1)
                    }
                },
            )
    }
}

impl<'a> From<&'a Board> for BoardDistance {
    fn from(board: &'a Board) -> BoardDistance {
        let mut board_distance = [[[255; 16]; 16]; 2];

        let mut queue = VecDeque::new();

        let p0 = board.get_player0_position();
        let p1 = board.get_player1_position();

        // x, y, id, iteration
        queue.push_back((p0.0, p0.1, 0, 0));
        queue.push_back((p1.0, p1.1, 1, 0));

        while !queue.is_empty() {
            let (row, col, id, iteration) = queue.pop_front().unwrap();

            if !Board::in_bounds(row, col)
                || (!board.cell_is_empty(row, col) && iteration > 0)
                || board_distance[id as usize][row as usize][col as usize] != 255
            {
                continue;
            }

            board_distance[id as usize][row as usize][col as usize] = iteration;

            queue.push_back((row + 1, col, id, iteration + 1));
            queue.push_back((row - 1, col, id, iteration + 1));
            queue.push_back((row, col + 1, id, iteration + 1));
            queue.push_back((row, col - 1, id, iteration + 1));
        }

        BoardDistance {
            player0: board_distance[0],
            player1: board_distance[1],
        }
    }
}

impl fmt::Display for BoardDistance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.player0.iter() {
            for cell in row {
                write!(f, "{:#3} ", cell)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n");

        for row in self.player1.iter() {
            for cell in row {
                write!(f, "{:#3} ", cell)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}
