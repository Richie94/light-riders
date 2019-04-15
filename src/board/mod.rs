use std::fmt;
use types::Move;

use std::collections::{HashMap, HashSet};
use std::io::{stderr, Write};
use std::cmp::max;
use std::time::Instant;
use std::os::raw::c_float;

const BLOCKED_VALUE: i32 = 100;

#[derive(Clone, Debug)]
pub struct Board {
    b: [[Cell; 16]; 16],
    player_position: [(u8, u8); 2],
    adjacents: HashMap<(u8, u8), HashSet<(u8, u8)>>,
    best_turn: Move,
    desired_depth: u8,
    turn: i32,
    score_options: Vec<(Move, f64)>,
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
    pub fn set_best_turn(&mut self, turn: Move) {
        self.best_turn = turn;
    }
    pub fn get_score_options(&self) -> Vec<(Move, f64)> {
        self.score_options.clone()
    }

    pub fn get_player_position(&self, player_id : u8) -> (u8, u8) {
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

    pub fn mini_max(&mut self, player_id: u8, depth: u8, end_game : bool) -> f64 {
        let mut legal_moves = self.legal_moves(player_id);

        if (depth < 1) | (legal_moves.len() < 1) {
            return self.get_score(player_id, end_game).0 + (self.desired_depth - depth) as f64;
        }

        let player = self.player_position[player_id as usize];
        let enemy = self.player_position[((player_id + 1) % 2) as usize];
        let mut max_value: f64 = -100000.0;

        while !legal_moves.is_empty() {
            let chosen_move = legal_moves.remove(0);
            self.execute_move(chosen_move, player_id, false);
            let mut value : f64;
            if end_game {
                value = self.mini_max(player_id, depth - 1, end_game);
            } else {
                value = -self.mini_max((player_id + 1) % 2, depth - 1, end_game);
            }
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

        return max_value;
    }

    pub fn get_metrics(&mut self, current_position: (u8, u8)) -> ([[i32; 16]; 16], u32) {
        let mut reachable_points: HashSet<(u8, u8)> = HashSet::new();
        reachable_points.insert(current_position);
        let mut node_edges: Vec<u32> = vec![];
        let mut newly_added: HashSet<(u8, u8)> = self.get_adjacent(current_position, true);

        let mut depth_count = 0;

        let mut distance_matrix  = [[BLOCKED_VALUE; 16]; 16];

        while !newly_added.is_empty() {
            let last_round_added = newly_added.clone();
            depth_count += 1;
            newly_added = HashSet::new();

            for point in &last_round_added {
                reachable_points.insert(*point);
            }

            for point in &last_round_added {
                let adjacent_to_point: HashSet<(u8, u8)> = self.get_adjacent(*point, true);
                let length: u32 = adjacent_to_point.len() as u32;
                let value = (length as c_float / 2.0) + 1 as c_float;
                //node_edges.push(value.floor() as u32);

                for adjacent_point in adjacent_to_point {
                    distance_matrix[adjacent_point.0 as usize][adjacent_point.1 as usize] = depth_count;
                    match reachable_points.get(&adjacent_point) {
                        Some(_a) => {}
                        None => {
                            newly_added.insert(adjacent_point);
                            node_edges.push(1);
                        }
                    }
                }
            }
        }
        (distance_matrix, node_edges.iter().sum())
    }

    pub fn get_metrics_for_player(&mut self, player_id: u8) -> ([[i32; 16]; 16], u32) {
        let position = self.player_position[player_id as usize];
        self.get_metrics(position)
    }

    // Executes a move (expects a legal move)
    pub fn execute_move(&mut self, next_move: Move, player_id: u8, reverse: bool) {
        let all_moves = vec![Move::Up, Move::Down, Move::Right, Move::Left];
        let player = self.player_position[player_id as usize];
        let delta = self.move_to_position(next_move, player_id, reverse);

        for direction_move in &all_moves {
            let point_to_update = self.move_to_position(*direction_move, player_id, reverse);
            self.get_adjacent(point_to_update, false);
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
            let point_to_update = self.move_to_position(*direction_move, player_id, reverse);
            self.get_adjacent(point_to_update, false);
        }
    }

    pub fn get_score(&mut self, player_id: u8, endgame_mode : bool) -> (f64, bool) {
        let my_score_tuple: ([[i32; 16]; 16], u32) = self.get_metrics_for_player(player_id);
        let my_distances = my_score_tuple.0;
        let enemy_id = (player_id + 1) % 2;
        let enemy_score_tuple : ([[i32; 16]; 16], u32) = self.get_metrics_for_player(enemy_id);
        let enemy_distances = enemy_score_tuple.0;
        if endgame_mode {
            return ((my_score_tuple.1) as f64, false)
        }

        let mut my_score = 0.0;

        let mut we_can_meet = false;

        for col in 0..16 {
            for row in 0..16 {
                let adjacent_points = self.adjacents.get(&(col as u8, row as u8));
                let mut cell_value = 1.0;
                match adjacent_points {
                    Some(points) => {
                        if points.len() > 2 {
                            cell_value += 0.1;
                        }
                    }
                    None => {}
                }

                if my_distances[col][row] == BLOCKED_VALUE || enemy_distances[col][row] == BLOCKED_VALUE {
                    cell_value += 0.1;
                }

                if my_distances[col][row] < enemy_distances[col][row] {
                    my_score += cell_value;
                } else if my_distances[col][row] > enemy_distances[col][row]{
                    my_score -= cell_value;
                }

                if we_can_meet == false && my_distances[col][row] < BLOCKED_VALUE && enemy_distances[col][row] < BLOCKED_VALUE {
                    we_can_meet = true;
                }
            }
        }
        (my_score, we_can_meet)
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
            desired_depth: 8,
            turn: 0,
            score_options: vec![],
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
