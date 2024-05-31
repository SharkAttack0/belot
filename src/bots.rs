use std::cmp::max;

use rand::*;
use strum::IntoEnumIterator;

use crate::general_enums::{constants::*, GameMode::*, *};
use crate::{card_value_trump, cards_compare, cards_value, cards_value_trump};

//NOTE: create function which determines what the bot should bid

pub fn bot(
    hand: &Vec<Card>,
    other_cards: &Vec<Card>,
    cards_in_play: &Vec<Card>,
    game_mode: GameMode,
) -> usize {
    //return a card's index of its hand to be played
    let vlastni = get_vlastni(hand, other_cards, game_mode);

    if cards_in_play.is_empty() {
        //bot plays first
        if !vlastni.is_empty() {
            return hand
                .iter()
                .position(|&r| r == vlastni[random_from_zero(vlastni.len())])
                .unwrap();
        }
        //doesn't have vlastni at first trick, play lowest card
        return min_val_pos(&cards_value_trump(&hand, game_mode));
    } else {
        //bot responds to play
        let valid_cards = valid_cards(hand, game_mode, cards_in_play);
        //get strongest and weakest card's indexes in hand
        let valid_cards_val = cards_value_trump(&valid_cards, game_mode);
        let mut min_valid_hand_index = 1000;
        let mut max_valid_hand_index = 1000;

        let max_valid_card = valid_cards[max_val_pos(&valid_cards_val)];
        for (index, card) in hand.iter().enumerate() {
            if max_valid_card == *card {
                max_valid_hand_index = index;
                break;
            }
        }

        let min_valid_card = valid_cards[min_val_pos(&valid_cards_val)];
        for (index, card) in hand.iter().enumerate() {
            if min_valid_card == *card {
                min_valid_hand_index = index;
                break;
            }
        }

        //determine if bot can take the trick

        //i dont know what i was thinking with this:
        // for card in valid_cards.iter() {
        //     if vlastni.contains(card) {
        //         return hand.iter().position(|&r| r == *card).unwrap();
        //     }
        // }

        //bot can't take the trick
        let strongest_card_index = cards_compare(cards_in_play, game_mode);

        if cards_in_play.len() == 2 {
            if strongest_card_index == 1 {
                //teammate might take, evaluate if teammate will take for sure
                if get_vlastni_bool(&vec![cards_in_play[1]], other_cards, game_mode) {
                    //teammate takes for sure - give strongest
                    return max_valid_hand_index;
                }
            } else {
                //opponents will take the trick for sure, give weakest
                return min_valid_hand_index;
            }
        } else if cards_in_play.len() == 3 {
            if strongest_card_index == 2 {
                //teammate takes, give strongest valid card
                return max_valid_hand_index;
            } else {
                //opponent takes , give weakest valid card
                return min_valid_hand_index;
            }
        }
        //one card is played - opponent's, and can't be taken - give weakest
        return min_valid_hand_index;
    }
}

fn max_val_pos(values: &Vec<usize>) -> usize {
    //give highest's value's index
    let mut max_val = 0;
    let mut max_index = 0;
    for index in 0..values.len() {
        if max_val > values[index] {
            max_val = values[index];
            max_index = index;
        }
    }
    max_index
}

fn min_val_pos(values: &Vec<usize>) -> usize {
    //give lowest's value's index
    let mut min_val = 0;
    let mut min_index = 0;
    for index in 0..values.len() {
        if min_val < values[index] {
            min_val = values[index];
            min_index = index;
        }
    }
    min_index
}

pub fn get_vlastni(hand: &Vec<Card>, other_cards: &Vec<Card>, game_mode: GameMode) -> Vec<Card> {
    let mut vlastni_cards: Vec<Card> = vec![];
    let mut hand_local = hand.clone();

    for (index, cur_suit) in CardSuits::iter().enumerate() {
        let other_max_suit = card_max_suit(other_cards, &game_mode, cur_suit);
        let hand_max_suit = card_max_suit(&hand_local, &game_mode, cur_suit);
        match hand_max_suit {
            None => continue,
            Some(max_hand_card) => match other_max_suit {
                None => {
                    //there are no other cards of suit - all vlastni
                    for card in hand.iter() {
                        if card.suit == cur_suit {
                            vlastni_cards.push(*card);
                        }
                    }
                }
                Some(max_other_card) => {
                    //hand and other hands have cards of suit - compare
                    let mut cur_max_hand_card = max_hand_card;
                    let mut vlastni_counter = 0;

                    let other_card_val = card_value_trump(&max_other_card, game_mode);

                    loop {
                        let main_card_val = card_value_trump(&cur_max_hand_card, game_mode);

                        if main_card_val > other_card_val {
                            vlastni_cards.push(cur_max_hand_card);
                            vlastni_counter += 1;
                        } else {
                            //this code also counts vlastni to be non-vlastni cards which will become vlastni
                            //when the cards of suit are played from the highest one descending
                            // let mut other_cards_counter = 0;
                            // for card in other_cards.iter() {
                            //     if card.suit == cur_suit {
                            //         other_cards_counter += 1;
                            //     }
                            // }
                            // if vlastni_counter >= other_cards_counter {
                            //     //vlastni's number is higher than total of other's cards
                            //     //=> all vlastni
                            //     for card in hand_local.iter() {
                            //         if card.suit == cur_suit && !vlastni_cards.contains(card) {
                            //             vlastni_cards.push(*card);
                            //         }
                            //     }
                            // }
                            break;
                        }
                        //prepare next biggest card
                        hand_local.remove(
                            hand_local
                                .iter()
                                .position(|&r| r == cur_max_hand_card)
                                .unwrap(),
                        );

                        match card_max_suit(&hand_local, &game_mode, cur_suit) {
                            Some(max_hand_card) => cur_max_hand_card = max_hand_card,
                            None => break,
                        };
                    }
                }
            },
        }
    }
    vlastni_cards
}

pub fn get_vlastni_bool(hand: &Vec<Card>, other_cards: &Vec<Card>, game_mode: GameMode) -> bool {
    for (index, cur_suit) in CardSuits::iter().enumerate() {
        let other_max_suit = card_max_suit(other_cards, &game_mode, cur_suit);
        let max_suit = card_max_suit(hand, &game_mode, cur_suit);
        match max_suit {
            None => continue,
            Some(max_hand_card) => match other_max_suit {
                None => {
                    //there are no other cards of suit - all vlastni
                    for card in hand.iter() {
                        if card.suit == cur_suit {
                            return true;
                        }
                    }
                }
                Some(max_other_card) => {
                    //hand and other hands have cards of suit - compare
                    let mut cur_max_hand_card = max_hand_card;

                    let other_card_val = cards_value_trump(other_cards, game_mode);
                    let main_card_val = cards_value_trump(hand, game_mode);

                    if main_card_val > other_card_val {
                        return true;
                    }
                }
            },
        }
    }
    false
}

fn card_max_suit(cards: &Vec<Card>, game_mode: &GameMode, suit: CardSuits) -> Option<Card> {
    //takes vec of cards and returns the strongest card of specified suit
    //if vec has no card of suit, that value is None
    let mut max_card_suit: Option<Card> = None;
    let mut cards_suit: Vec<Card> = vec![];

    for card in cards.iter() {
        if card.suit == suit {
            cards_suit.push(*card);
        }
    }

    if !cards_suit.is_empty() {
        max_card_suit = Some(cards_suit[cards_compare(&cards_suit, *game_mode)]);
    }
    max_card_suit
}

fn random_from_zero(lenght: usize) -> usize {
    //returns random int from 0 to lenght - 1
    let mut rng = rand::thread_rng();
    rng.gen_range(0..lenght)
}

pub fn valid_cards(hand: &Vec<Card>, game_mode: GameMode, cards_in_play: &Vec<Card>) -> Vec<Card> {
    //returns vec of valid cards
    //(do NOT use for 1st card of turn, since 1st card doesn't have limits)
    //strongest_card must be the current strongest card
    let win_hand_index = cards_compare(&cards_in_play, game_mode);
    let strongest_card = cards_in_play[win_hand_index];

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

    let mut valid_cards: Vec<Card> = vec![];

    for card in hand.iter() {
        let card_val = TRUMP_ORDER.iter().position(|&r| r == card.value).unwrap();
        //init checks, valid for every game mode
        if has_init_suit {
            if card.suit != strongest_card.suit {
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
                                break;
                            }
                        }
                        if has_trump {
                            //no init_suit, but has trump (this is case 2 from cards_compare())
                            if card.suit == trump_suit {
                                valid_cards.push(*card);
                                continue;
                            }
                            continue;
                        }
                        valid_cards.push(*card);
                        continue;
                    }
                }
                _ => (),
            }
            valid_cards.push(*card);
            continue;
        }

        match game_mode {
            NoTrumps => (),
            AllTrumps => {
                if has_higher_value {
                    if card_val > init_card_val {
                        valid_cards.push(*card);
                        continue;
                    }
                    continue;
                }
                valid_cards.push(*card);
                continue;
            }
            OneTrump(trump_suit) => {
                //trump case
                if strongest_card.suit == trump_suit {
                    //in case of 2 cards in play - 1st card is teammate, skip this check
                    //in case of 3 cards in play - 2nd card is teammate, skip this check
                    if (cards_in_play.len() == 2 && strongest_card == cards_in_play[0])
                        || (cards_in_play.len() == 3 && strongest_card == cards_in_play[1])
                    {
                        valid_cards.push(*card);
                        continue;
                    }
                    if has_higher_value {
                        if card_val > init_card_val {
                            valid_cards.push(*card);
                            continue;
                        }
                        continue;
                    }
                    valid_cards.push(*card);
                    continue;
                }
            }
        }
        valid_cards.push(*card);
        continue;
    }
    valid_cards
}
