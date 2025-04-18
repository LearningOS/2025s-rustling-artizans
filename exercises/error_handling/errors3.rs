// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// 

use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
       println!("You can't afford that many!");
       Err("You can't afford that many!".into())
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
        // Ok(format!("Hi! My name is {}", tokens))
        Ok( println!("You now have {} tokens.", tokens))
       
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
