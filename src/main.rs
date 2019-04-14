use std::error::Error;
use blackjack_sim::output_to_csv;

fn main() -> Result<(), Box<dyn Error>> {
    const TRIALS: i32 = 1000;
    const ACE_VAL: i32 = 11;
    output_to_csv(TRIALS, ACE_VAL)?;
    Ok(())
}
