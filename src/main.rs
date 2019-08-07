use blackjack_sim::deck::{Deck, ShuffledDeck};
use csv::Writer;
use std::{
    error::Error,
    mem::{self, MaybeUninit},
    ptr,
};

fn main() -> Result<(), Box<dyn Error>> {
    static TRIALS: u32 = 10_000;
    static ACE_VAL: u8 = 11;
    static ACE_VAL_3RD_CARD: u8 = 1;

    let mut wtr = Writer::from_path("./target/output.csv")?;
    wtr.write_record(&[
        "Player's 1st Card",
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
        "Dealer's 3rd Card Value",
    ])?;

    const HANDS: usize = 2;
    const CELLS_PER_HAND_PER_DEAL: usize = 2;
    const DEALS: usize = 3;
    const TOTAL: usize = HANDS * DEALS * CELLS_PER_HAND_PER_DEAL;
    let data = ShuffledDeck::simulate(TRIALS, |mut deck| {
        let mut row: [MaybeUninit<String>; TOTAL] = unsafe { MaybeUninit::uninit().assume_init() };
        let row: [String; TOTAL] = {
            let mut i = 0;
            while i < TOTAL {
                let ace = if i < HANDS * 2 /* Deals */ * CELLS_PER_HAND_PER_DEAL {
                    ACE_VAL
                } else {
                    ACE_VAL_3RD_CARD
                };
                for _ in 0..HANDS {
                    let card = deck.pick().unwrap();
                    unsafe {
                        ptr::write(row[i].as_mut_ptr(), format!("{}", card));
                        ptr::write(
                            row[i + 1].as_mut_ptr(),
                            format!("{}", card.value().as_num(ace)),
                        )
                    }
                    i += 2;
                }
            }
            unsafe { mem::transmute(row) }
        };
        row
    });

    for row in data {
        wtr.write_record(&row)?;
    }
    println!("Finished");
    Ok(())
}
