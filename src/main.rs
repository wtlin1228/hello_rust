mod utils;
use utils::{statistics, pig_latin};

fn main() {
    println!(
        "{:?}",
        statistics::get_median_and_mode(&vec![1, 5, 5, 3, 2, 2, 5])
    );
    println!(
        "{:?}",
        statistics::get_median_and_mode(&vec![1, 5, 5, 3, 2, 5])
    );
    println!(
        "{}",
        pig_latin::to_pig_latin(String::from("fiRst apPle नमस्ते"))
    );
}
