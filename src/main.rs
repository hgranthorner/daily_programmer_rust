mod yahtzee_upper_section_scoring;
mod same_necklace;

#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use crate::yahtzee_upper_section_scoring::YahtzeeArray;

fn main() {
    same_necklace::same_necklace(String::from("a"), String::from("a"));
    yahtzee_upper_section_scoring::yahtzee_upper(YahtzeeArray::new());
    println!("Hello, world!");
}
