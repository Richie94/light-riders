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
	max_depth: u8,
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
			max_depth: 20
		}
	}
}

impl Bot for RichardBot {
	fn get_move(&mut self) -> Move {
		self.board.as_mut().unwrap().reset_score_options();
		self.rng_state = (1664525 * self.rng_state + 1013904223) % 4294967296;
		self.round = self.round + 1;
		writeln!(&mut stderr(), "=== ROUND {} ===", self.round).expect("Stderr problem");
		let possible_moves: Vec<Move> = self
			.board
			.as_ref()
			.iter()
			.flat_map(|b| b.legal_moves(self.id))
			.collect();


		let depth = self.board.as_ref().unwrap().get_desired_depth();
		self.board.as_mut().unwrap().mini_max(self.id, depth, -10000, 10000);
		writeln!(&mut stderr(), "Score Options: {:?}", self.board.as_ref().unwrap().get_score_options()).expect("Stderr problem");
		self.board.as_mut().unwrap().get_best_turn()

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
