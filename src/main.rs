use std::error::Error;
use csv::Writer;

mod deck;
use deck::Deck;

fn main() -> Result<(), Box<dyn Error>> {
	const TRIALS: i32 = 1000;
	let mut wtr = Writer::from_path("out.csv")?;
	wtr.write_record(&["Player's 1st Card",
					   "Dealer's 1st Card",
					   "Player's 2nd Card",
					   "Dealer's 2nd Card",
					   "Player's Sum",
					   "Dealer's Sum"])?;
	for _ in 0..TRIALS {
		let mut deck = Deck::new();
		let mut player = vec![];
		let mut dealer = vec![];
		for _ in 0..2 {
			player.push(deck.pick().unwrap());
			dealer.push(deck.pick().unwrap());
		}
		let player_sum: i32 = player.iter().map(|a| a.val()).sum();
		let dealer_sum: i32 = dealer.iter().map(|a| a.val()).sum();
		let rec = vec![format!("{}", player[0].val()),
					   format!("{}", dealer[0].val()),
					   format!("{}", player[1].val()), 
					   format!("{}", dealer[1].val()),
					   format!("{}", player_sum),
					   format!("{}", dealer_sum)];
		wtr.write_record(&rec)?;
		//println!("{:?}\n{:?}\n", player, dealer);
	}
	Ok(())
}
