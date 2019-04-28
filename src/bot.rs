use types::*;
use std::time::Duration;

pub trait Bot {
	fn get_move(&mut self, time_to_move : Duration) -> Move;
	fn update_round(&mut self, round: u8);
	fn update_board(&mut self, text: &str);
	fn set_setting(&mut self, setting: Setting);
}
