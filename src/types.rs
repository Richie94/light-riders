use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum Move {
	Up, Down, Left, Right
}

#[derive(Clone, Debug)]
pub enum Setting {
	Timebank(Duration),
	TimePerMove(Duration),
	BotName(String),
	BotId(u8),
	FieldWidth(u8),
	FieldHeight(u8),
	PlayerNames { player1: String, player2: String },
}
