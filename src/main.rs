use std::error::Error;
use csv::Writer;

mod deck;
use deck::{Deck, DealerDeck, PlayerHand};

fn main() -> Result<(), Box<dyn Error>> {
    const TRIALS: i32 = 1000;
    const ACE_VAL: i32 = 11;
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

    for _ in 0..TRIALS {
        let mut deck = DealerDeck::new();
        let mut player = PlayerHand::new();
        let mut player_sums = vec![];
        let mut dealer = PlayerHand::new();
        let mut dealer_sums = vec![];
        for i in 0..3 {
            player.push(deck.pick().unwrap());
            dealer.push(deck.pick().unwrap());
            player_sums.push(player.sum(i, ACE_VAL));
            dealer_sums.push(dealer.sum(i, ACE_VAL));
        }

        wtr.write_record(&[format!("{:?}", player[0]),
                           format!("{}", player[0].val(ACE_VAL)),
                           format!("{:?}", dealer[0]),
                           format!("{}", dealer[0].val(ACE_VAL)),
                           format!("{:?}", player[1]),
                           format!("{}", player[1].val(ACE_VAL)), 
                           format!("{:?}", dealer[1]),
                           format!("{}", dealer[1].val(ACE_VAL)),
                           format!("{}", player_sums[1]),
                           format!("{}", dealer_sums[1]),
                           format!("{:?}", player[2]),
                           format!("{}", player[2].val(ACE_VAL)), 
                           format!("{:?}", dealer[2]),
                           format!("{}", dealer[2].val(ACE_VAL)),
                           format!("{}", player_sums[2]),
                           format!("{}", dealer_sums[2])])?;
    }
    Ok(())
}
