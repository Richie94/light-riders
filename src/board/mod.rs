use std::fmt;
use types::Move;

use std::collections::{HashMap, HashSet};
use std::io::{stderr, Write};

#[derive(Clone, Debug)]
pub struct Board {
    b: [[Cell; 16]; 16],
    player_position: [(u8, u8); 2],
    adjacents: HashMap<(u8, u8), HashSet<(u8, u8)>>,
    best_turn: Move,
    desired_depth: u8,
    turn: i32,
    score_options: Vec<(Move, f64)>,
    transposition_table: HashMap<[[Cell; 16]; 16], f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    pub fn set_best_turn(&mut self, turn: Move) {
        self.best_turn = turn;
    }

    pub fn get_transpositions(&self) -> HashMap<[[Cell; 16]; 16], f64> {
        self.transposition_table.clone()
    }
    pub fn set_transpositions(&mut self, trans: HashMap<[[Cell; 16]; 16], f64>) {
        self.transposition_table = trans;
    }

    pub fn get_score_options(&self) -> Vec<(Move, f64)> {
        self.score_options.clone()
    }

    pub fn get_player_position(&self, player_id: u8) -> (u8, u8) {
        self.player_position[player_id as usize]
    }

    pub fn reset_score_options(&mut self) {
        self.score_options = vec![];
    }

    pub fn set_desired_depth(&mut self, new_depth: u8) {
        writeln!(&mut stderr(), "Set Depth to: {:?}", new_depth).expect("Stderr problem");
        self.desired_depth = new_depth;
    }

    pub fn in_bounds(row: u8, col: u8) -> bool {
        row < 16 && col < 16
    }

    pub fn move_to_position(&self, chosen_move: Move, player_id: u8, reverse: bool) -> (u8, u8) {
        let player = self.player_position[player_id as usize];
        match chosen_move {
            Move::Up => {
                if !reverse {
                    return (player.0 - 1, player.1);
                } else {
                    return (player.0 + 1, player.1);
                }
            }
            Move::Down => {
                if !reverse {
                    return (player.0 + 1, player.1);
                } else {
                    return (player.0 - 1, player.1);
                }
            }
            Move::Left => {
                if !reverse {
                    return (player.0, player.1 - 1);
                } else {
                    return (player.0, player.1 + 1);
                }
            }
            Move::Right => {
                if !reverse {
                    return (player.0, player.1 + 1);
                } else {
                    return (player.0, player.1 - 1);
                }
            }
            _ => return (0, 0)
        }
    }

    pub fn legal_moves_point(&self, row: u8, col: u8) -> Vec<Move> {
        let mut moves = vec![];

        if !(Board::in_bounds(row, col)) {
            return moves;
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

    pub fn legal_moves(&self, player: u8) -> Vec<Move> {
        let (row, col) = self.player_position[player as usize];
        self.legal_moves_point(row, col)
    }


    pub fn get_adjacent(&mut self, point: (u8, u8), get_stored: bool) -> HashSet<(u8, u8)> {
        if !Board::in_bounds(point.0, point.1) {
            return HashSet::new();
        }

        if !get_stored || !self.adjacents.contains_key(&point) {
            let mut adjacent_fields = HashSet::new();
            let row = point.0;
            let col = point.1;

            if col > 0 && self.b[row as usize][(col - 1) as usize] == Cell::Empty {
                adjacent_fields.insert((row, col - 1));
            }

            if col < 15 && self.b[row as usize][(col + 1) as usize] == Cell::Empty {
                adjacent_fields.insert((row, col + 1));
            }

            if row > 0 && self.b[(row - 1) as usize][col as usize] == Cell::Empty {
                adjacent_fields.insert((row - 1, col));
            }

            if row < 15 && self.b[(row + 1) as usize][col as usize] == Cell::Empty {
                adjacent_fields.insert((row + 1, col));
            }

            self.adjacents.insert(point, adjacent_fields);
        }

        match self.adjacents.get(&point) {
            Some(a) => a.clone(),
            None => HashSet::new()
        }
    }

    pub fn get_territory(&mut self, player_id: u8, print: bool) -> f64 {
        let enemy_id = ((player_id + 1) % 2) as usize;
        let mut score_matrix = [[0.0; 16]; 16];
        let mut my_new_cells = HashSet::new();
        my_new_cells.insert(self.player_position[player_id as usize]);
        let mut enemy_new_cells = HashSet::new();
        enemy_new_cells.insert(self.player_position[enemy_id]);

        while !my_new_cells.is_empty() || !enemy_new_cells.is_empty() {
            let enemy_cells_last_round = enemy_new_cells.clone();
            enemy_new_cells = HashSet::new();

            for point in &enemy_cells_last_round {
                if score_matrix[point.0 as usize][point.1 as usize] == 0.0 {
                    let adjacent_to_point: HashSet<(u8, u8)> = self.get_adjacent(*point, true);
                    let neighbour_count = adjacent_to_point.len();
                    let point_score = -1.0 - (neighbour_count - 2) * 0.05;
                    score_matrix[point.0 as usize][point.1 as usize] = point_score;
                    for adjacent_point in adjacent_to_point {
                        if score_matrix[adjacent_point.0 as usize][adjacent_point.1 as usize] == 0.0 {
                            enemy_new_cells.insert(adjacent_point);
                        }
                    }
                }
            }

            let my_cells_last_round = my_new_cells.clone();
            my_new_cells = HashSet::new();

            for point in &my_cells_last_round {
                if score_matrix[point.0 as usize][point.1 as usize] == 0.0 {
                    let adjacent_to_point: HashSet<(u8, u8)> = self.get_adjacent(*point, true);
                    let neighbour_count = adjacent_to_point.len();
                    let point_score = 1.0 + (neighbour_count - 2) * 0.05;
                    score_matrix[point.0 as usize][point.1 as usize] = point_score;

                    for adjacent_point in adjacent_to_point {
                        if score_matrix[adjacent_point.0 as usize][adjacent_point.1 as usize] == 0.0 {
                            my_new_cells.insert(adjacent_point);
                        }
                    }
                }
            }
        }

        let mut score = 0.0;
        for col in 0..16 {
            for row in 0..16 {
                score += score_matrix[col][row];
            }
        }

        if print {
            writeln!(&mut stderr(), "score_matrix {:?}", score_matrix).expect("Stderr problem");
        }

        score
    }

    pub fn mini_max(&mut self, player_id: u8, depth: u8) -> f64 {
        let mut legal_moves = self.legal_moves(player_id);

        // if we have that situation in our transposition table
        if self.transposition_table.contains_key(&self.b) {
            //writeln!(&mut stderr(), "used transp table in depth {}", depth).expect("Stderr problem");
            match self.transposition_table.get(&self.b) {
                Some(a) => a,
                None => &0.0
            };
        }

        if (depth == 0) | (legal_moves.len() == 0) {
            let mut score = self.get_score(player_id);
            //writeln!(&mut stderr(), "player {} depth {} score {} legal moves: {}", player_id, depth, score, legal_moves.len()).expect("Stderr problem");
            if legal_moves.len() == 0 {
                score -= 1000.0;
            }

            let return_score = score + (self.desired_depth - depth) as f64;
            self.transposition_table.insert(self.b, return_score);
            return return_score;
        }

        let player = self.player_position[player_id as usize];
        let enemy = self.player_position[((player_id + 1) % 2) as usize];
        let mut max_value: f64 = -100000.0;

        while !legal_moves.is_empty() {
            let chosen_move = legal_moves.remove(0);
            self.execute_move(chosen_move, player_id, false);
            let value: f64 = -self.mini_max((player_id + 1) % 2, depth - 1);
            self.execute_move(chosen_move, player_id, true);

            if value > max_value {
                max_value = value;

                if depth == self.desired_depth {
                    self.best_turn = chosen_move;
                }
            }

            if depth == self.desired_depth {
                self.score_options.push((chosen_move, value));
            }
        }
        self.transposition_table.insert(self.b, max_value);
        return max_value;
    }

    // Executes a move (expects a legal move)
    pub fn execute_move(&mut self, next_move: Move, player_id: u8, reverse: bool) {
        //writeln!(&mut stderr(), "simulate move {} to {:?} (r:{}) {:?}", player_id, next_move, reverse, self.b).expect("Stderr problem");
        let all_moves = vec![Move::Up, Move::Down, Move::Right, Move::Left];
        let player = self.player_position[player_id as usize];
        let delta = self.move_to_position(next_move, player_id, reverse);

        let mut points_to_update = HashSet::new();

        for direction_move in &all_moves {
            points_to_update.insert(self.move_to_position(*direction_move, player_id, reverse));
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

        for direction_move in &all_moves {
            points_to_update.insert(self.move_to_position(*direction_move, player_id, reverse));
        }

        for point_to_update in points_to_update {
            self.get_adjacent(point_to_update, false);
        }
    }

    pub fn get_score(&mut self, player_id: u8) -> f64 {
        return self.get_territory(player_id, false) as f64;
    }
}


impl<'a> From<&'a str> for Board {
    fn from(text: &'a str) -> Board {
        let mut row = 0;
        let mut col = 0;
        let mut b = [[Cell::Empty; 16]; 16];
        let adjacents = HashMap::new();
        let mut player_position = [(0, 0); 2];

        //let sit = ".,.,.,.,.,.,.,x,x,x,x,x,x,x,x,x,.,x,x,.,x,x,.,x,x,.,.,x,x,.,x,x,.,x,x,0,x,x,x,.,x,.,.,x,x,x,x,x,.,x,x,.,x,.,x,.,x,.,.,x,.,x,.,x,x,x,x,.,x,.,x,.,x,.,.,x,x,x,x,x,x,x,x,x,x,.,x,.,x,.,.,x,x,.,x,x,x,x,x,x,x,.,x,.,x,.,.,x,x,.,.,x,x,x,x,x,x,.,x,.,x,.,.,x,x,x,x,x,x,x,x,x,.,.,x,.,x,.,x,x,.,x,x,x,x,x,x,x,.,.,x,.,x,.,x,x,x,1,x,x,x,x,x,x,.,.,x,.,x,.,.,x,x,.,x,x,x,x,x,x,.,.,x,.,x,.,.,x,.,.,x,x,x,x,x,x,.,x,x,.,x,x,x,x,x,x,x,x,x,x,x,x,.,.,.,.,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,.,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x";

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
            desired_depth: 8,
            turn: 0,
            score_options: vec![],
            transposition_table: HashMap::new(),
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
            Move::Up => write!(f, "UP"),
            Move::Down => write!(f, "DOWN"),
            Move::Right => write!(f, "RIGHT"),
            Move::Left => write!(f, "LEFT"),
            Move::Pass => write!(f, "Pass"),
        }
    }
}
