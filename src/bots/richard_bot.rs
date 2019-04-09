use board::Board;
use bot::Bot;
use std::time::*;
use types::*;
use std::io::{stderr, Write};

pub struct RichardBot {
    rng_state: u64,
    board: Option<Board>,
    id: u8,
    round: u8,
    min_depth: u8,
    current_depth: u8,
    max_depth: u8,
    timebank: Duration,
    endgame_mode: bool,
}

impl RichardBot {
    pub fn new() -> Self {
        RichardBot {
            rng_state: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            board: None,
            id: 0,
            round: 0,
            min_depth: 5,
            current_depth: 8,
            max_depth: 20,
            timebank: Duration::from_millis(0),
            endgame_mode: false,
        }
    }

    pub fn make_move_to_enemy(&mut self) -> Move {
        let mut legal_moves = self.board.as_mut().unwrap().legal_moves(self.id);
        let enemy_position = self.board.as_mut().unwrap().get_player_position((self.id + 1) % 2);

        legal_moves.sort_by(|a, b| {
            let position_a = self.board.as_mut().unwrap().move_to_position(*a, self.id, false);
            let position_b = self.board.as_mut().unwrap().move_to_position(*b, self.id, false);

            let dist_a_to_enemy = ((position_a.0 - enemy_position.0) as i8).abs() +
                ((position_a.1 - enemy_position.1) as i8).abs();
            let dist_b_to_enemy = ((position_b.0 - enemy_position.0) as i8).abs() +
                ((position_b.1 - enemy_position.1) as i8).abs();

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
            self.make_move_to_enemy()
        } else {
            let board = self.board.as_mut().unwrap();

            if !self.endgame_mode && !board.get_score(self.id, false).1 {
                self.endgame_mode = true;
            }

            board.reset_score_options();
            board.set_best_turn(Move::Pass);
            self.rng_state = (1664525 * self.rng_state + 1013904223) % 4294967296;
            writeln!(&mut stderr(), "=== ROUND {} ===", self.round).expect("Stderr problem");
            let mut chosen_move: Move;
            board.set_desired_depth(self.current_depth);
            board.mini_max(self.id, self.current_depth, self.endgame_mode);

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
                        self.current_depth /= 2;
                    } else {
                        self.current_depth -= 1;
                    }
                }
            }
            board.set_desired_depth(self.current_depth);

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
