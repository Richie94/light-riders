extern crate core;

mod board;
mod bot;
mod bots;
mod parser;
mod types;

use bots::richard_bot;

fn main() {
	let bot = richard_bot::RichardBot::new();
	let mut parser = parser::Parser::from(bot);
	parser.run();
}
