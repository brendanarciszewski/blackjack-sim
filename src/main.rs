use std::error::Error;
use csv::Writer;
use blackjack_sim::deck::{Deck, ShuffledDeck, PlayerHand};

fn main() -> Result<(), Box<dyn Error>> {
    static TRIALS: u32 = 10_000;
    static ACE_VAL: u32 = 11;
    static ACE_VAL_3RD_CARD: u32 = 1;
    
    let mut wtr = Writer::from_path("./target/output.csv")?;
    wtr.write_record(&["Player's 1st Card",
                       "Player's 1st Card Value",
                       "Dealer's 1st Card",
                       "Dealer's 1st Card Value",
                       "Player's 2nd Card",
                       "Player's 2nd Card Value",
                       "Dealer's 2nd Card",
                       "Dealer's 2nd Card Value",
                       "Player's 3rd Card",
                       "Player's 3rd Card Value",
                       "Dealer's 3rd Card",
                       "Dealer's 3rd Card Value"])?;
    

    let data = ShuffledDeck::simulate(TRIALS, |mut deck| {
        let mut player = PlayerHand::new();
        let mut dealer = PlayerHand::new();
        for _ in 0..3 {
            player.push(deck.pick().unwrap());
            dealer.push(deck.pick().unwrap());
        }
        [
            format!("{:?}", player[0]),
            format!("{}", player[0].value().as_num(ACE_VAL)),
            format!("{:?}", dealer[0]),
            format!("{}", dealer[0].value().as_num(ACE_VAL)),
            format!("{:?}", player[1]),
            format!("{}", player[1].value().as_num(ACE_VAL)),
            format!("{:?}", dealer[1]),
            format!("{}", dealer[1].value().as_num(ACE_VAL)),
            format!("{:?}", player[2]),
            format!("{}", player[2].value().as_num(ACE_VAL_3RD_CARD)),
            format!("{:?}", dealer[2]),
            format!("{}", dealer[2].value().as_num(ACE_VAL_3RD_CARD)),
        ]
    });

    for row in data {
        wtr.write_record(&row)?;
    }
    Ok(())
}
