use std;
use std::fmt;
use types::Move;

mod board_distance;

pub use self::board_distance::BoardDistance;
use std::collections::{HashMap, HashSet, LinkedList};
use std::io::{stderr, Write};
use std::iter::FromIterator;
use std::cmp::max;
use std::time::Instant;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Board {
    b: [[Cell; 16]; 16],
    player_position: [(u8, u8); 2],
    adjacents: HashMap<(u8, u8), HashSet<(u8, u8)>>,
    best_turn: Move,
    desired_depth: u8,
    turn : i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Player0,
    Player1,
    Wall,
}

impl Board {
    pub fn get_best_turn(&self) -> Move {
        self.best_turn
    }

    pub fn get_desired_depth(&self) -> u8 {
        self.desired_depth
    }
    pub fn get_player0_position(&self) -> (u8, u8) {
        self.player_position[0]
    }

    pub fn get_player1_position(&self) -> (u8, u8) {
        self.player_position[1]
    }

    pub fn in_bounds(row: u8, col: u8) -> bool {
        row < 16 && col < 16
    }

    pub fn get_cell(&self, row: u8, col: u8) -> Cell {
        self.b[row as usize][col as usize]
    }

    pub fn cell_is_empty(&self, row: u8, col: u8) -> bool {
        self.get_cell(row, col) == Cell::Empty
    }

    pub fn legal_moves(&self, player: u8) -> Vec<Move> {
        let (row, col) = self.player_position[player as usize];

        let mut moves = vec![];

        if !(Board::in_bounds(row, col)) {
            return moves
        }

        if col > 0 && self.b[row as usize][(col - 1) as usize] == Cell::Empty {
            moves.push(Move::Left);
        }

        if col < 15 && self.b[row as usize][(col + 1) as usize] == Cell::Empty {
            moves.push(Move::Right);
        }

        if row > 0 && self.b[(row - 1) as usize][col as usize] == Cell::Empty {
            moves.push(Move::Up);
        }

        if row < 15 && self.b[(row + 1) as usize][col as usize] == Cell::Empty {
            moves.push(Move::Down);
        }

        moves
    }


    pub fn get_adjacent(&mut self, point: (u8, u8), get_stored: bool) -> HashSet<(u8, u8)> {
        if !get_stored || !self.adjacents.contains_key(&point) {
            let mut adjacent_fields = HashSet::new();
            let col = point.0;
            let row = point.1;

            if col > 0 && self.b[row as usize][(col - 1) as usize] == Cell::Empty {
                adjacent_fields.insert((col - 1, row));
            }

            if col < 15 && self.b[row as usize][(col + 1) as usize] == Cell::Empty {
                adjacent_fields.insert((col + 1, row));
            }

            if row > 0 && self.b[(row - 1) as usize][col as usize] == Cell::Empty {
                adjacent_fields.insert((col, row - 1));
            }

            if row < 15 && self.b[(row + 1) as usize][col as usize] == Cell::Empty {
                adjacent_fields.insert((col, row + 1));
            }

            self.adjacents.insert(point, adjacent_fields);
        }
        let a: Option<&HashSet<(u8, u8)>> = self.adjacents.get(&point);

        match a {
            Some(a) => a.clone(),
            None => HashSet::new()
        }
    }

    pub fn mini_max(&mut self, player_id: u8, depth: u8, alpha: i32, beta: i32) -> i32 {
        let mut legal_moves = self.legal_moves(player_id);

        let player = self.player_position[player_id as usize];
        let enemy = self.player_position[((player_id + 1) % 2) as usize];


        let mut local_alpha = alpha;
        if (depth < 1) | (legal_moves.len() < 1) {
            return self.get_score(player_id);
        }

        let mut max_value: i32 = -100000;

        //legal_moves.sort_by(|a,b| )
//
//        // todo: sort moves
//
        while legal_moves.len() > 0 {
            let chosen_move = legal_moves.remove(0);
            self.execute_move(chosen_move, player_id, false);
            let value: i32 = -self.mini_max(((player_id + 1) % 2), depth - 1, -beta, -local_alpha);
            self.execute_move(chosen_move, player_id, true);
            //writeln!(&mut stderr(), "Calculated {}, depth: {}", value, depth).expect("Stderr problem");

            if value >= max_value {
                max_value = value;

                if depth == self.desired_depth {
                    //writeln!(&mut stderr(), "Set best turn {}", chosen_move).expect("Stderr problem");
                    self.best_turn = chosen_move;
                }
            }

            local_alpha = max(value, alpha);

            if local_alpha >= beta {
                break;
            }
        }

        self.get_score(player_id)
    }

    pub fn get_score(&mut self, player_id: u8) -> i32 {
        let my_score: i32 = self.get_amount_of_reachable_points_for_player(player_id) as i32;
        let enemy_score: i32 = self.get_amount_of_reachable_points_for_player((player_id + 1) % 2) as i32;
        my_score - enemy_score
    }

    pub fn get_amount_of_reachable_points_for_player(&mut self, player_id: u8) -> u32 {
        let start = Instant::now();
        let mut reachable_points: HashSet<(u8, u8)> = HashSet::new();
        let mut node_edges: LinkedList<u32> = LinkedList::new();
        let current_position = self.player_position[player_id as usize];
        //let mut newly_added = self.get_adjacent(current_position, true);
        let mut newly_added: HashSet<(u8, u8)> = self.get_adjacent(current_position, true);

        let mut continue_loop: bool = true;

        while continue_loop {
            continue_loop = false;

            for point in &newly_added {
                reachable_points.insert(*point);
            }

            let last_round_added = newly_added.clone();
            newly_added = HashSet::new();

            for point in last_round_added {
                let adjacent_to_point : HashSet<(u8, u8)> = self.get_adjacent(point, true);
                let length: u32 = adjacent_to_point.len() as u32;
                let value = (length / 2) + 1; // todo floor
                node_edges.push_back(value);

                for neighbour in adjacent_to_point {
                    if !(reachable_points.contains(&neighbour)) {
                        newly_added.insert(neighbour);
                        continue_loop = true;
                    }
                }
            }
        }
        let result = node_edges.iter().sum();
        //writeln!(&mut stderr(), "Scoring {}", result).expect("Stderr problem");
        let elapsed = start.elapsed();
        let in_ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

        writeln!(&mut stderr(), "Took {:?} ms", in_ms).expect("Stderr problem");
        result
    }

    // Executes a move (expects a legal move)
    pub fn execute_move(&mut self, next_move: Move, player_id: u8, reverse: bool) {
        let player = self.player_position[player_id as usize];
        let mut delta;

        match next_move {
            Move::Up => {
                if !reverse{
                    delta = (player.0 - 1, player.1);

                } else {
                    delta = (player.0 + 1, player.1);
                }
            },
            Move::Down => {
                if !reverse {
                    delta = (player.0 + 1, player.1);
                } else {
                    delta = (player.0 - 1, player.1);
                }
            },
            Move::Left => {
                if !reverse {
                    delta = (player.0, player.1 - 1);
                } else {
                    delta = (player.0, player.1 + 1);
                }
            },
            Move::Right => {
                if !reverse {
                    delta = (player.0, player.1 + 1);
                } else {
                    delta = (player.0, player.1 - 1);
                }
            },
        }

        if reverse {
            self.b[player.0 as usize][player.1 as usize] = Cell::Empty;
        } else {
            self.b[player.0 as usize][player.1 as usize] = Cell::Wall;
        }

        if player_id == 0 {
            self.b[delta.0 as usize][delta.1 as usize] = Cell::Player0;
        } else {
            self.b[delta.0 as usize][delta.1 as usize] = Cell::Player1;
        }
        self.player_position[player_id as usize] = delta;
    }
}

impl<'a> From<&'a str> for Board {
    fn from(text: &'a str) -> Board {
        let mut row = 0;
        let mut col = 0;
        let mut b = [[Cell::Empty; 16]; 16];
        let adjacents = HashMap::new();
        let mut player_position = [(0, 0); 2];
        for c in text.split(',') {
            match c {
                "." => b[row as usize][col as usize] = Cell::Empty,
                "x" => b[row as usize][col as usize] = Cell::Wall,
                "0" => {
                    b[row as usize][col as usize] = Cell::Player0;
                    player_position[0] = (row, col);
                }
                "1" => {
                    b[row as usize][col as usize] = Cell::Player1;
                    player_position[1] = (row, col);
                }
                _ => unimplemented!(),
            }

            col = (col + 1) % 16;
            if col == 0 {
                row += 1;
            }
        }

        Board {
            b,
            player_position,
            adjacents,
            best_turn: Move::Up,
            desired_depth: 4,
            turn: 0
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.b.iter() {
            for cell in row {
                write!(f, "{} ", cell)?;
            }
            write!(f, "\n")?;
        }

        write!(f, "\n")
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Cell::Empty => write!(f, "."),
            &Cell::Player0 => write!(f, "0"),
            &Cell::Player1 => write!(f, "1"),
            &Cell::Wall => write!(f, "X"),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Move::Up => write!(f, "UP"),
            &Move::Down => write!(f, "DOWN"),
            &Move::Right => write!(f, "RIGHT"),
            &Move::Left => write!(f, "LEFT"),
        }
    }
}
