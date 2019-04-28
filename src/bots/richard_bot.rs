use board::{Board};
use bot::Bot;
use std::time::*;
use types::*;
use std::io::{stderr, Write};

pub struct RichardBot {
    pub board: Board,
    id: u8,
    round: u8,
    min_depth: u8,
    current_depth: u8,
    max_depth: u8,
    timebank: Duration
}

impl RichardBot {
    pub fn new() -> Self {
        RichardBot {
            board: Board::new(),
            id: 0,
            round: 0,
            min_depth: 5,
            current_depth: 8,
            max_depth: 20,
            timebank: Duration::from_millis(0)
        }
    }

    pub fn make_move_to_middle(&mut self) -> Move {
        let mut legal_moves = self.board.legal_moves(self.id);

        legal_moves.sort_by(|a, b| {
            let position_a = self.board.move_to_position(*a, self.id, false);
            let position_b = self.board.move_to_position(*b, self.id, false);

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
            self.board.reset_score_options();
            self.board.set_best_turn(Move::Pass);
            writeln!(&mut stderr(), "=== ROUND {} ===", self.round).expect("Stderr problem");
            self.board.get_territory(self.id, true);
            self.board.set_desired_depth(self.current_depth);
            self.board.mini_max(self.id, self.current_depth);

            writeln!(&mut stderr(), "Score Options: {:?}", self.board.get_score_options()).expect("Stderr problem");

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
            self.board.get_best_turn()
        }
    }


    fn update_round(&mut self, round: u8) {
        self.round = round;
    }

    fn update_board(&mut self, text: &str) {
        self.board.update(text);
    }

    fn set_setting(&mut self, setting: Setting) {
        match setting {
            Setting::BotId(id) => self.id = id,
            _ => {}
        }
    }
}
