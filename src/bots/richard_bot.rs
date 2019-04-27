use board::{Board, Cell};
use bot::Bot;
use std::time::*;
use types::*;
use std::io::{stderr, Write};
use std::collections::HashMap;

pub struct RichardBot {
    board: Option<Board>,
    id: u8,
    round: u8,
    min_depth: u8,
    current_depth: u8,
    max_depth: u8,
    timebank: Duration,
    transposition_table: HashMap<[[Cell; 16]; 16], f64>
}

impl RichardBot {
    pub fn new() -> Self {
        RichardBot {
            board: None,
            id: 0,
            round: 0,
            min_depth: 5,
            current_depth: 8,
            max_depth: 20,
            timebank: Duration::from_millis(0),
            transposition_table: HashMap::new()
        }
    }

    pub fn make_move_to_middle(&mut self) -> Move {
        let mut legal_moves = self.board.as_mut().unwrap().legal_moves(self.id);

        legal_moves.sort_by(|a, b| {
            let position_a = self.board.as_mut().unwrap().move_to_position(*a, self.id, false);
            let position_b = self.board.as_mut().unwrap().move_to_position(*b, self.id, false);

            let dist_a_to_enemy = ((position_a.0 - 8) as i8).abs() +
                ((position_a.1 - 8) as i8).abs();
            let dist_b_to_enemy = ((position_b.0 - 8) as i8).abs() +
                ((position_b.1 - 8) as i8).abs();

            dist_a_to_enemy.cmp(&dist_b_to_enemy)
        });

        legal_moves[0]
    }
}

impl Bot for RichardBot {
    fn get_move(&mut self, time_to_move: Duration) -> Move {
        self.timebank = time_to_move;
        let before = Instant::now();
        self.round += 1;
        if self.round < 3 {
            self.make_move_to_middle()
        } else {
            let board = self.board.as_mut().unwrap();

            board.reset_score_options();
            board.set_best_turn(Move::Pass);
            board.set_transpositions(self.transposition_table.clone());
            writeln!(&mut stderr(), "=== ROUND {} ===", self.round).expect("Stderr problem");
            board.get_territory(self.id, true);
            board.set_desired_depth(self.current_depth);
            board.mini_max(self.id, self.current_depth);

            writeln!(&mut stderr(), "Score Options: {:?}", board.get_score_options()).expect("Stderr problem");

            let elapsed = before.elapsed().as_millis();
            writeln!(&mut stderr(), "Elapsed {}, Timebank {:?}", elapsed, self.timebank).expect("Stderr problem");
            if (elapsed < 300) && (self.timebank.as_millis() > 6000) {
                if self.current_depth < self.max_depth {
                    self.current_depth += 1;
                }
            } else {
                if self.current_depth > self.min_depth {
                    if elapsed > 1000 {
                        self.current_depth -= 4;
                    } else {
                        self.current_depth -= 1;
                    }
                }
            }
            board.set_desired_depth(self.current_depth);
            self.transposition_table = board.get_transpositions();
            board.get_best_turn()
        }
    }


    fn update_round(&mut self, round: u8) {
        self.round = round;
    }

    fn update_board(&mut self, board: Board) {
        self.board = Some(board);
    }

    fn set_setting(&mut self, setting: Setting) {
        match setting {
            Setting::BotId(id) => self.id = id,
            _ => {}
        }
    }
}
