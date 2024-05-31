pub mod announcments {
    use crate::cards_value;
    use crate::general_enums::{constants::*, CardSuits::*, CardValue::*, GameMode::*, *};
    use crate::sort_hand;

    pub fn belot_check(
        hand: &Vec<Card>,
        game_mode: GameMode,
        played_card: Card,
        init_card: Card,
    ) -> bool {
        //returns true if there is a belot (also checks if game mode allows it)
        //if the played_card is also the first played card, then init_card should be played_card
        if played_card.value == King || played_card.value == Queen {
            let second_belot_card_val = if played_card.value == King {
                Queen
            } else {
                King
            };

            let main_suit = match game_mode {
                NoTrumps => return false,
                AllTrumps => init_card.suit,
                OneTrump(trump_suit) => trump_suit,
            };

            if played_card.suit == main_suit {
                if hand.contains(&Card {
                    suit: played_card.suit,
                    value: second_belot_card_val,
                }) {
                    println!("Belot! Added 20 points");
                    return true;
                }
            }
        }
        false
    }
    use strum::IntoEnumIterator;

    fn check_cards_sequence(hand: &Vec<Card>) -> (Vec<usize>, Vec<Card>) {
        //sorts hand, returns 4 ints for cards in a highest row of same suit
        //also returns highest card of the highest sequence for each suit
        //DOESN'T WORK IN 1 CASE - IN CASE OF 2 SEQUENCES IN
        //SAME SUIT, WILL REGISTER ONLY 1 (excluding cases of quinte)
        let sort_way = REGULAR_ORDER;
        let hand = sort_hand(&mut hand.clone(), sort_way);
        let cards_actual_value = cards_value(&hand, sort_way);
        //final results
        let mut max_card_seqs: Vec<Card> = Vec::new();
        let mut hand_sequence_values = Vec::new();

        for spec_suit in CardSuits::iter() {
            //temp highest results
            let mut row_value: usize = 0;
            let mut temp_row_value: usize = 1;
            let mut temp_highest_card: Card = Card {
                value: Seven,
                suit: Clubs,
            };
            let mut highest_index = 1;
            for index in 0..hand.len() - 1 {
                if hand[index].suit == spec_suit && hand[index + 1].suit == spec_suit {
                    //check if current and next card are in a row
                    if cards_actual_value[index] == cards_actual_value[index + 1] - 1 {
                        temp_row_value += 1;
                        highest_index = index + 1;
                    } else {
                        //sequence ends, record results
                        if temp_row_value > row_value {
                            row_value = temp_row_value;
                            temp_highest_card = hand[index];
                        }
                        temp_row_value = 1;
                    }
                }
            }
            //do check again incase else case never occurs
            if temp_row_value > row_value {
                row_value = temp_row_value;
                temp_highest_card = hand[highest_index];
            }
            max_card_seqs.push(temp_highest_card);
            hand_sequence_values.push(row_value);
        }
        (hand_sequence_values, max_card_seqs)
    }

    //validating cards sequences
    pub fn card_sequences_validation(points_from_announs: &mut [usize; 2], hands: &[Vec<Card>; 4]) {
        let mut sequence_values: [Vec<usize>; 2] = [vec![], vec![]];
        let mut highest_sequences: [usize; 2] = [0, 0];
        let mut max_card_seq: [Vec<Card>; 2] = [vec![], vec![]];

        for index in 0..4 {
            let (temp_seq_cal, temp_max_card_seq) = check_cards_sequence(&hands[index]);
            //4 x 4 values (4 lenghts for each suit, for each hand)
            sequence_values[index % 2].extend(temp_seq_cal);
            //4 x 4 values (4 max card for each suit, for each hand)
            max_card_seq[index % 2].extend(temp_max_card_seq);
        }

        for index in 0..2 {
            for val in sequence_values[index].iter() {
                if highest_sequences[index] < *val {
                    highest_sequences[index] = *val;
                }
            }
        }
        let mut team_sequence_index = 2;
        let mut all_sequences_fall = false;
        if highest_sequences[0] >= 3 && highest_sequences[1] >= 3 {
            if highest_sequences[0] == highest_sequences[1] {
                //highest sequences are equal
                println!("The highest sequences are of equal length");
                let max_card_actual_val = [
                    cards_value(&max_card_seq[0], REGULAR_ORDER),
                    cards_value(&max_card_seq[1], REGULAR_ORDER),
                ];
                let mut highest_card: [usize; 2] =
                    [max_card_actual_val[0][0], max_card_actual_val[1][0]];
                for team_index in 0..2 {
                    for index in 0..sequence_values[team_index].len() {
                        if sequence_values[team_index][index] == highest_sequences[team_index] {
                            if highest_card[team_index] < max_card_actual_val[team_index][index] {
                                highest_card[team_index] = max_card_actual_val[team_index][index];
                            }
                        }
                    }
                }
                println!("0 - {:?} 1 - {:?}", highest_card[0], highest_card[1]);
                if highest_card[0] == highest_card[1] {
                    println!("Both teams' highest cards of highest sequences are equal!");
                    println!("Not counting anything");
                    all_sequences_fall = true;
                } else if highest_card[0] > highest_card[1] {
                    println!("Team #1 has higher card! Team #2's sequences are not counted!'");
                    team_sequence_index = 0;
                } else {
                    println!("Team #2 has higher card! Team #1's sequences are not counted!'");
                    team_sequence_index = 1;
                }
                //determine highest card values of each max length sequence
            } else {
                //one team has a higher sequence than other
                if highest_sequences[0] > highest_sequences[1] {
                    println!("Team #1 has longer card sequence");
                    println!("Team #2's card sequences don't count");
                    team_sequence_index = 0;
                } else {
                    println!("Team #2 has longer card sequence");
                    println!("Team #1's card sequences don't count");
                    team_sequence_index = 1;
                };
            }
        } else if highest_sequences[0] >= 3 {
            println!("Team #1 has a seq, but team #2 doesn't");
            team_sequence_index = 0;
        } else if highest_sequences[1] >= 3 {
            println!("Team #2 has a seq, but team #1 doesn't");
            team_sequence_index = 1;
        } else {
            println!("No team has seq");
            all_sequences_fall = true;
        }
        if !all_sequences_fall {
            points_from_announs[team_sequence_index] +=
                points_from_card_sequences(&sequence_values[team_sequence_index]);
        }
    }

    fn points_from_card_sequences(sequence_values: &Vec<usize>) -> usize {
        //takes vec of sequence lenghts, turns it into points and returns
        let mut points = 0;
        for val in sequence_values {
            match val {
                3 => points += 20,
                4 => points += 50,
                5 => points += 100,
                6 => points += 100,
                7 => points += 100,
                8 => points += 120,
                _ => (),
            }
        }
        points
    }

    pub fn check_carre(hand: &Vec<Card>, hand_index: usize, points_count: &mut [usize; 2]) {
        let mut value_times = [0, 0, 0, 0, 0, 0];
        for card in hand {
            match card.value {
                Seven => (),
                Eight => (),
                Nine => value_times[0] += 1,
                Ten => value_times[1] += 1,
                Jack => value_times[2] += 1,
                Queen => value_times[3] += 1,
                King => value_times[4] += 1,
                Ace => value_times[5] += 1,
            }
        }
        for (index, val_time) in value_times.iter().enumerate() {
            if *val_time == 4 {
                match index {
                    0 => {
                        println!("\tThis hand has a carré of Nines!");
                        points_count[hand_index % 2] += 150;
                    }
                    1 => {
                        println!("\tThis hand has a carré of Tens!");
                        points_count[hand_index % 2] += 100;
                    }
                    2 => {
                        println!("\tThis hand has a carré of Jacks!");
                        points_count[hand_index % 2] += 200;
                    }
                    3 => {
                        println!("\tThis hand has a carré of Queens!");
                        points_count[hand_index % 2] += 100;
                    }
                    4 => {
                        println!("\tThis hand has a carré of Kings!");
                        points_count[hand_index % 2] += 100;
                    }
                    5 => {
                        println!("\tThis hand has a carré of Aces!");
                        points_count[hand_index % 2] += 100;
                    }
                    _ => panic!("At check_carre() out of bounds index"),
                };
            }
        }
    }
}

pub mod basic {
    use crate::general_enums::{constants::*, CardSuits::*, CardValue::*, GameMode::*, *};

    enum PointsOrder {
        NoTrumps,
        AllTrumps,
    }

    pub fn point_count(point_decks: &[Vec<Card>; 2], game_mode: GameMode) -> [usize; 2] {
        //takes 2 decks and transforms cards into points for each team
        let mut points_from_decks: [usize; 2] = [0, 0];
        for index in 0..2 {
            for card in point_decks[index].iter() {
                let points_order = match game_mode {
                    NoTrumps => PointsOrder::NoTrumps,
                    AllTrumps => PointsOrder::AllTrumps,
                    OneTrump(trump_suit) => {
                        if card.suit == trump_suit {
                            PointsOrder::AllTrumps
                        } else {
                            PointsOrder::NoTrumps
                        }
                    }
                };

                //This is 1 of 2 ways to do it:
                //2nd way is to make arrays of each team's points, then add them
                //(this is in the case that you need arrays of points for some reason)
                points_from_decks[index] += match points_order {
                    PointsOrder::NoTrumps => match card.value {
                        Seven => 0,
                        Eight => 0,
                        Nine => 0,
                        Jack => 2,
                        Queen => 3,
                        King => 4,
                        Ten => 10,
                        Ace => 11,
                    },
                    PointsOrder::AllTrumps => match card.value {
                        Seven => 0,
                        Eight => 0,
                        Queen => 3,
                        King => 4,
                        Ten => 10,
                        Ace => 11,
                        Nine => 14,
                        Jack => 20,
                    },
                }
            }
        }

        //in case of No trumps: multiply points by 2
        match game_mode {
            NoTrumps => {
                points_from_decks[0] *= 2;
                points_from_decks[1] *= 2;
            }
            _ => (),
        }
        points_from_decks
    }
}

