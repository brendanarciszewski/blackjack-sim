use std::error::Error;
use csv::Writer;

mod deck;
pub mod card;
pub use deck::{Deck, ShuffledDeck, PlayerHand};

pub fn output_to_csv(trials: i32, ace_val: i32) -> Result<(), Box<dyn Error>> {
	let mut wtr = Writer::from_path("./target/output.csv")?;
    wtr.write_record(&["Player's 1st Card",
                       "Player's 1st Card Value",
                       "Dealer's 1st Card",
                       "Dealer's 1st Card Value",
                       "Player's 2nd Card",
                       "Player's 2nd Card Value",
                       "Dealer's 2nd Card",
                       "Dealer's 2nd Card Value",
                       "Player's 2 Card Sum",
                       "Dealer's 2 Card Sum",
                       "Player's 3rd Card",
                       "Player's 3rd Card Value",
                       "Dealer's 3rd Card",
                       "Dealer's 3rd Card Value",
                       "Player's 3 Card Sum",
                       "Dealer's 3 Card Sum"])?;

    for _ in 0..trials {
        let mut deck = ShuffledDeck::new();
        let mut player = PlayerHand::new();
        let mut player_sums = vec![];
        let mut dealer = PlayerHand::new();
        let mut dealer_sums = vec![];
        for i in 0..3 {
            player.push(deck.pick().unwrap());
            dealer.push(deck.pick().unwrap());
            player_sums.push(player.sum(i, ace_val));
            dealer_sums.push(dealer.sum(i, ace_val));
        }

        wtr.write_record(&[format!("{:?}", player[0]),
                           format!("{}", player[0].val(ace_val)),
                           format!("{:?}", dealer[0]),
                           format!("{}", dealer[0].val(ace_val)),
                           format!("{:?}", player[1]),
                           format!("{}", player[1].val(ace_val)), 
                           format!("{:?}", dealer[1]),
                           format!("{}", dealer[1].val(ace_val)),
                           format!("{}", player_sums[1]),
                           format!("{}", dealer_sums[1]),
                           format!("{:?}", player[2]),
                           format!("{}", player[2].val(1)), 
                           format!("{:?}", dealer[2]),
                           format!("{}", dealer[2].val(1)),
                           format!("{}", player_sums[2]),
                           format!("{}", dealer_sums[2])])?;
    }
    Ok(())
}