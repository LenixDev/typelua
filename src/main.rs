use lenix::Player;

fn main() {
	let score: i32 = 10;
	let _status: &str = if score > 5 { "win" } else { "lose" };

	let player: Player = Player::new(String::from("Lenix"), score);
	println!("{}", player.describe());
}
