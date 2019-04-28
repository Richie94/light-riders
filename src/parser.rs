use board::Board;
use bot::Bot;
use types::*;

use std::io::{self, stderr, BufRead, Write};
use std::time::{Duration, Instant};

pub struct Parser<T: Bot> {
	bot: T,
	board : Board,
}

impl<T: Bot> Parser<T> {
	pub fn run(&mut self) {
		let stdin = io::stdin();
		let handle = stdin.lock();
		for line in handle.lines().map(|l| l.expect("stdin read failed")) {
			let command = line.split(' ').collect::<Vec<_>>();
			match command[0] {
				"action" => {
					self.handle_action(&command);
				}
				"update" => {
					self.handle_update(&command);
				}
				"settings" => {
					self.handle_setting(&command);
				}
				_ => panic!(),
			}
		}
	}

	fn handle_action(&mut self, command: &Vec<&str>) {
		let start = Instant::now();
		match command[1] {
			"move" => send_move(&self.bot.get_move(Duration::from_millis(command[2].parse::<u64>().unwrap()))),
			_ => panic!(),
		}

		let elapsed = start.elapsed();
		let in_ms = elapsed.as_secs() * 1000 + elapsed.subsec_nanos() as u64 / 1_000_000;

		writeln!(&mut stderr(), "Turn ended in {:?} ms ({:?})", in_ms, command).expect("Stderr problem");
	}

	fn handle_update(&mut self, command: &Vec<&str>) {
		match command[2] {
			"round" => self.bot.update_round(command[3].parse::<u8>().unwrap()),
			"field" => self.bot.update_board(command[3]),
			_ => panic!(),
		}
	}

	fn handle_setting(&mut self, command: &Vec<&str>) {
		self.bot.set_setting(match command[1] {
			"timebank" => {
				Setting::Timebank(Duration::from_millis(command[2].parse::<u64>().unwrap()))
			}
			"time_per_move" => {
				Setting::TimePerMove(Duration::from_millis(command[2].parse::<u64>().unwrap()))
			}
			"your_bot" => Setting::BotName(command[2].into()),
			"your_botid" => Setting::BotId(command[2].parse::<u8>().unwrap()),
			"field_width" => Setting::FieldWidth(command[2].parse::<u8>().unwrap()),
			"field_height" => Setting::FieldHeight(command[2].parse::<u8>().unwrap()),
			"player_names" => {
				let mut names = command[2].split(',');
				Setting::PlayerNames {
					player1: names.next().unwrap().into(),
					player2: names.next().unwrap().into(),
				}
			}
			_ => panic!(),
		});
	}
}

impl<T: Bot> From<T> for Parser<T> {
	fn from(bot: T) -> Self {
		Parser { bot: bot, board: Board::new() }
	}
}

fn send_move(m: &Move) {
	println!(
		"{}",
		match *m {
			Move::Up => "up",
			Move::Down => "down",
			Move::Left => "left",
			Move::Right => "right",
			Move::Pass => "pass"
		}
	);
}
