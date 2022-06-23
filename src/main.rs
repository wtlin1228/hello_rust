use std::collections::HashMap;

mod exercise;

use exercise::{company, pig_latin, statistics};

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

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    company::add_people_to_department(&mut company, "RD".to_string(), "Leo".to_string());
    company::add_people_to_department(&mut company, "RD".to_string(), "Una".to_string());
    company::add_people_to_department(&mut company, "RD".to_string(), "Leonerd".to_string());
    if let Some(rd) = company::get_people_from_department(&mut company, "RD".to_string()) {
        println!("{:?}", rd);
    }
}
