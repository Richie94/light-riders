use types::*;
use board::Board;

pub trait Bot {
	fn get_move(&mut self) -> Move;
	fn update_round(&mut self, round: u8);
	fn update_board(&mut self, board: Board);
	fn set_setting(&mut self, setting: Setting);
}
