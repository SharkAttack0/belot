use strum::IntoEnumIterator;

use crate::general_enums::{constants::*, CardSuits::*, CardValue::*, GameMode::*, *};
use crate::{ask_play_card, card_sequences_validation, print_cards_in_play};

pub fn sort_hand(hand: &mut Vec<Card>, sort_way: [CardValue; 8]) -> Vec<Card> {
    //takes hand, returns sorted (by suit, by value, weakest to strongest)
    let mut card_regular_value: Vec<usize> = cards_value(&hand, sort_way);

    for index in 0..hand.len() {
        let mut smallest_card = card_regular_value[index];
        let mut temp_j = index;
        for j in index + 1..hand.len() {
            if smallest_card > card_regular_value[j] {
                smallest_card = card_regular_value[j];

                temp_j = j;
            }
        }
        let temp = hand[index];
        hand[index] = hand[temp_j];
        hand[temp_j] = temp;
        let temp_card_regular_value = card_regular_value[index];
        card_regular_value[index] = card_regular_value[temp_j];
        card_regular_value[temp_j] = temp_card_regular_value;
    }

    let mut sorted_suit_hand: Vec<Card> = Vec::new();

    for spec_suit in CardSuits::iter() {
        for card in hand.iter() {
            if card.suit != spec_suit {
                continue;
            }
            sorted_suit_hand.push(*card);
        }
    }
    sorted_suit_hand
}

pub fn sort_hand_gamemode(hand: &mut Vec<Card>, game_mode: GameMode) -> Vec<Card> {
    //sorts hand based on the game mode (by suit, by value, weakest to strongest)
    let mut sorted_hand: Vec<Card> = vec![];
    match game_mode {
        NoTrumps => sorted_hand = sort_hand(hand, NO_TRUMP_ORDER),
        AllTrumps => sorted_hand = sort_hand(hand, TRUMP_ORDER),
        //can make it to sort trump suit in trump order
        OneTrump(trump_suit) => {
            //sort in no trump order first
            sorted_hand = sort_hand(hand, NO_TRUMP_ORDER);
            //remove trump cards
            let mut trump_cards: Vec<Card> = Vec::new();
            for card_index in 0..sorted_hand.len() {
                let card = sorted_hand[card_index];
                if card.suit == trump_suit {
                    trump_cards.push(card);
                }
            }
            //remove trump cards from hand
            sorted_hand.retain(|card| card.suit != trump_suit);
            //push properly sorted trump cards
            //NOTE: trump cards are being added at end of sequence always
            //(although accidental, its actually nice)
            sorted_hand.extend(sort_hand(&mut trump_cards, TRUMP_ORDER));
        }
    }
    sorted_hand
}

pub fn cards_compare(cards_in_play: &Vec<Card>, game_mode: GameMode) -> usize {
    //takes vec of Cards, returns strongest's index according to rules and game mode
    let mut card_strongest_index: usize = 0;
    let cards_val = cards_value_trump(&cards_in_play, game_mode);
    let mut temp_card_strongest_value = cards_val[0];
    let mut trump_played = false;
    let mut init_suit = cards_in_play[0].suit;

    //compare each card, in one trump case compare only when needed (otherwise incorrect result)
    for (index, card) in cards_in_play.iter().enumerate() {
        match game_mode {
            OneTrump(trump_suit) => {
                if trump_played == false && card.suit == trump_suit {
                    //case 2 - No trumps played, current card is trump, DON'T compare
                    trump_played = true;
                    temp_card_strongest_value = cards_val[index];
                    card_strongest_index = index;
                    init_suit = trump_suit;
                    continue;
                }
                if trump_played == true && card.suit != trump_suit {
                    //case 3 - Trump played, current card non-trump, DON'T compare
                    continue;
                }
            }
            _ => (),
        }

        if card.suit == init_suit {
            if temp_card_strongest_value < cards_val[index] {
                temp_card_strongest_value = cards_val[index];
                card_strongest_index = index;
            }
        }
    }
    card_strongest_index
}

pub fn card_validation(
    hand: &mut Vec<Card>,
    game_mode: GameMode,
    strongest_card: Card,
    cards_in_play: &Vec<Card>,
    win_hand_index: usize,
) -> Card {
    //ask for card, check if valid, plays it if so, otherwise ask again
    //(do NOT use for 1st card of turn, since 1st card doesn't have limits)
    //strongest_card must be the current strongest card
    let mut has_init_suit = false;
    let mut has_higher_value = false;
    let init_card_val = TRUMP_ORDER
        .iter()
        .position(|&r| r == strongest_card.value)
        .unwrap();
    let card_val = cards_value(hand, TRUMP_ORDER);
    for (index, card) in hand.iter().enumerate() {
        if card.suit == strongest_card.suit {
            has_init_suit = true;
            if card_val[index] > init_card_val {
                has_higher_value = true;
            }
        }
    }
    let mut card_to_play: Card;
    let mut card_to_play_index: usize;

    loop {
        print_cards_in_play(cards_in_play, win_hand_index);
        card_to_play_index = ask_play_card(hand);
        card_to_play = hand[card_to_play_index];
        let card_to_play_val = TRUMP_ORDER
            .iter()
            .position(|&r| r == card_to_play.value)
            .unwrap();
        //init checks, valid for every game mode
        if has_init_suit {
            if card_to_play.suit != strongest_card.suit {
                println!("Card's suit doesn't match the required one!");
                continue;
            }
        } else {
            //extra check for onetrump case
            match game_mode {
                OneTrump(trump_suit) => {
                    if strongest_card.suit != trump_suit {
                        let mut has_trump = false;
                        for card in hand.iter() {
                            if card.suit == trump_suit {
                                has_trump = true;
                            }
                        }
                        if has_trump {
                            //no init_suit, but has trump (this is case 2 from cards_compare())
                            if card_to_play.suit == trump_suit {
                                break;
                            }
                            println!("Play a trump!");
                            continue;
                        }
                        break;
                    }
                }
                _ => (),
            }
            break;
        }

        match game_mode {
            NoTrumps => (),
            AllTrumps => {
                if has_higher_value {
                    if card_to_play_val > init_card_val {
                        break;
                    }
                    println!("Play a higher value trump!");
                    continue;
                }
                break;
            }
            OneTrump(trump_suit) => {
                //trump case
                if strongest_card.suit == trump_suit {
                    //in case of 2 cards in play - 1st card is teammate, skip this check
                    //in case of 3 cards in play - 2nd card is teammate, skip this check
                    if (cards_in_play.len() == 2 && strongest_card == cards_in_play[0])
                        || (cards_in_play.len() == 3 && strongest_card == cards_in_play[1])
                    {
                        break;
                    }
                    if has_higher_value {
                        if card_to_play_val > init_card_val {
                            break;
                        }
                        println!("Play a higher value trump!");
                        continue;
                    }
                    break;
                }
            }
        }
        break;
    }
    hand.remove(card_to_play_index);
    card_to_play
}

pub fn cards_value(hand: &Vec<Card>, sort_way: [CardValue; 8]) -> Vec<usize> {
    //returns ints of cards' values according to a specified ordering
    let mut cards_actual_value: Vec<usize> = Vec::new();
    for card in hand.iter() {
        cards_actual_value.push(sort_way.iter().position(|&r| r == card.value).unwrap());
    }
    cards_actual_value
}

pub fn cards_value_trump(cards: &Vec<Card>, game_mode: GameMode) -> Vec<usize> {
    //create vec with value of each card (value depending if its trump)
    let mut cards_val: Vec<usize> = vec![];

    for card in cards.iter() {
        match game_mode {
            OneTrump(trump_suit) => {
                if card.suit == trump_suit {
                    cards_val.push(TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap());
                } else {
                    cards_val.push(
                        NO_TRUMP_ORDER
                            .iter()
                            .position(|&r| r == card.value)
                            .unwrap(),
                    );
                }
            }
            NoTrumps => cards_val.push(
                NO_TRUMP_ORDER
                    .iter()
                    .position(|&r| r == card.value)
                    .unwrap(),
            ),
            AllTrumps => cards_val.push(TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap()),
        };
    }
    cards_val
}
pub fn card_value_trump(card: &Card, game_mode: GameMode) -> usize {
    //return value of card depending on the game mode

    match game_mode {
        OneTrump(trump_suit) => {
            if card.suit == trump_suit {
                TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap()
            } else {
                NO_TRUMP_ORDER
                    .iter()
                    .position(|&r| r == card.value)
                    .unwrap()
            }
        }
        NoTrumps => NO_TRUMP_ORDER
            .iter()
            .position(|&r| r == card.value)
            .unwrap(),

        AllTrumps => TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap(),
    }
}
