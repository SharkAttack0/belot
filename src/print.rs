use crate::general_enums::Card;

pub fn print_cards_in_play(cards_in_play: &Vec<Card>, first_card_index: usize) {
    //prints cards_in_play depending on their count and first playing player
    for index in 0..cards_in_play.len() {
        println!(
            "\t\t\tp{}:{:?} {:?}",
            ((first_card_index + index) % 4) + 1,
            cards_in_play[index].value,
            cards_in_play[index].suit
        );
    }
    println!();
}

pub fn print_hand(hand: &Vec<Card>, label_cards: bool) {
    for (index, card) in hand.iter().enumerate() {
        if label_cards {
            println!("{}:\t{:?}\t{:?}", index + 1, card.value, card.suit);
        } else {
            println!("\t{:?}\t{:?}", card.value, card.suit);
        }
    }
    println!();
}
