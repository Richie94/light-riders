use board::Board;
use bot::Bot;
use std::time::*;
use types::*;

pub struct StarterBot {
	rng_state: u64,
	board: Option<Board>,
	id: u32,
	round: u32,
}

impl StarterBot {
	pub fn new() -> Self {
		StarterBot {
			rng_state: SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.unwrap()
				.as_secs(),
			board: None,
			id: 0,
			round: 0,
		}
	}
}

impl Bot for StarterBot {
	fn get_move(&mut self) -> Move {
		self.rng_state = (1664525 * self.rng_state + 1013904223) % 4294967296;
		let moves: Vec<Move> = self
			.board
			.as_ref()
			.iter()
			.flat_map(|b| b.legal_moves(self.id))
			.collect();
		if moves.len() > 0 {
			moves
				.get(self.rng_state as usize % moves.len())
				.map(|m| (*m).clone())
				.unwrap()
		} else {
			Move::Up
		}
	}

	fn update_round(&mut self, round: u32) {
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
