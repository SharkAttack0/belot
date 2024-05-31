use std::io;
use crate::print_hand;
use crate::Card;

pub fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input = String::from(input.trim());
    input
}

pub fn user_input_to_int(max_allowed_int: usize) -> usize {
    let mut input_int;
    loop {
        let input = user_input();
        let input_to_int = input.parse::<usize>();
        match input_to_int {
            Ok(int) => input_int = int,
            Err(_) => {
                println!("Error: invalid input!");
                continue;
            }
        }
        if input_int == 0 || input_int > max_allowed_int {
            println!("Error: invalid number!");
            continue;
        }
        break;
    }
    //subtract one so that input represents actual order #
    input_int - 1
}

pub fn ask_play_card(hand: &mut Vec<Card>) -> usize {
    let ans_int;
    println!("Choose a card:");
    print_hand(hand, true);
    ans_int = user_input_to_int(hand.len());
    ans_int
}