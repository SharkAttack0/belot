#![allow(unused)]
use std::panic;
use std::usize;

mod deck_generation;
use deck_generation::*;

use general_enums::constants::*;
use strum::*;

mod general_enums;
use general_enums::{CardSuits::*, CardValue::*, GameMode::*, *};

mod bots;
use bots::bot;
use bots::get_vlastni;

mod bidding;
use crate::bidding::bidding::bid_phase;

mod user_input;
use user_input::*;

mod eval_points;
use eval_points::announcments::*;
use eval_points::basic::*;

mod print;
use print::*;

mod essentials;
use essentials::*;

mod test;

fn main() {
    run();
}

fn run() {
    //these are variables that must carry throughout games
    let mut points_total: [usize; 2] = [0, 0];
    let mut init_hand_index = 0;
    let mut hanging_points = 0;
    let mut bot_player_index = 0;

    //game loop
    loop {
        //these are variables that are needed for a game
        //most important var - keeps track of trick's winner
        let mut win_hand_index = init_hand_index;
        let mut points_game: [usize; 2];
        let mut points_from_announs: [usize; 2] = [0, 0];
        let mut point_decks: [Vec<Card>; 2] = [vec![], vec![]];
        //create first hands and deck
        let (mut hands, mut deck) = new_game();
        let mut cards_in_play: Vec<Card> = Vec::with_capacity(4);
        let (bidding, player_last_bid_index) = bid_phase(win_hand_index);
        let game_mode;

        let mut double_game_mode = false;
        let mut redouble_game_mode = false;

        match bidding {
            bidding::Bidding::Pass => {
                println!("\nAll players passed! Restarting...\n");
                init_hand_index = (init_hand_index + 1) % 4;
                println!("Enter any key to continue");
                user_input();
                continue;
            }
            bidding::Bidding::GameMode(bid_game_mode) => {
                println!("\nThe game mode is {:?}\n", bid_game_mode);
                game_mode = bid_game_mode;
            }
            bidding::Bidding::Double(bid_game_mode) => {
                double_game_mode = true;
                game_mode = bid_game_mode;
            }
            bidding::Bidding::ReDouble(bid_game_mode) => {
                redouble_game_mode = true;
                game_mode = bid_game_mode;
            }
        }

        hands = continue_game(hands, &mut deck, game_mode, &mut points_from_announs);

        //actual playing
        for _ in 0..hands[0].len() {
            let mut init_card: Card;
            //for for current turn
            //NOTE: make this a reference

            //turn starts
            for hand_index in 0..4 {
                let mut other_hands: Vec<Card> = vec![];

                //determine other_hands
                for index in 0..4 {
                    if index != win_hand_index {
                        other_hands.extend(&hands[index]);
                    }
                }

                let actual_player_index = (hand_index + win_hand_index) % 4;
                print_cards_in_play(&cards_in_play, win_hand_index);
                println!("player #{}", actual_player_index + 1);
                //finish this

                //remove this line
                bot_player_index = actual_player_index;

                if actual_player_index == bot_player_index {
                    let init_card_index = bot(
                        &hands[bot_player_index],
                        &other_hands,
                        &cards_in_play,
                        game_mode,
                    );
                    init_card = hands[actual_player_index][init_card_index];
                    hands[actual_player_index].remove(init_card_index);
                    cards_in_play.push(init_card);
                } else {
                    if hand_index == 0 {
                        //skip card validation if first card
                        let init_card_index = ask_play_card(&mut hands[win_hand_index]);
                        init_card = hands[win_hand_index][init_card_index];
                        hands[win_hand_index].remove(init_card_index);
                        cards_in_play.push(init_card);
                    } else {
                        init_card = cards_in_play[cards_compare(&cards_in_play, game_mode)];
                        cards_in_play.push(card_validation(
                            &mut hands[actual_player_index],
                            game_mode,
                            init_card,
                            &cards_in_play,
                            win_hand_index,
                        ));
                    }
                }
                //belot check
                if belot_check(
                    &hands[actual_player_index],
                    game_mode,
                    cards_in_play[hand_index],
                    init_card,
                ) {
                    points_from_announs[actual_player_index % 2] += 20;
                }
            }
            print_cards_in_play(&cards_in_play, win_hand_index);
            //win_hand_index is the same index as first player of next turn
            //cards_compare() returns winning card index, which represents
            //the number of positions from the initial card (win_hand_index)
            win_hand_index = (win_hand_index + cards_compare(&cards_in_play, game_mode)) % 4;
            println!(
                "Strongest card is the {:?} of {:?}",
                cards_in_play[win_hand_index].value, cards_in_play[win_hand_index].suit
            );

            //assume indexes 0 and 2 are of one team, as well as 1 and 3
            point_decks[win_hand_index % 2].append(&mut cards_in_play);
        } //round's over

        points_game = point_count(&point_decks, game_mode);

        println!("\nThe round is over!");
        println!(
            "Added 10 points to team #{} for getting last trick\n",
            (win_hand_index % 2) + 1
        );

        //add last 10 to team that got last trick
        match game_mode {
            NoTrumps => points_game[win_hand_index % 2] += 20,
            _ => {
                points_game[win_hand_index % 2] += 10;
                for index in 0..2 {
                    println!(
                        "Team #{}'s points from announcments: {}",
                        index + 1,
                        points_from_announs[index]
                    );
                    //add points from announcments
                    points_game[index] += points_from_announs[index];
                }
            }
        }

        //check for kapo
        check_kapo(&point_decks, &mut points_game);

        //print points from game
        println!();
        for index in 0..2 {
            println!(
                "Team #{}'s points from game: {}",
                index + 1,
                points_game[index]
            );
        }

        //vutrene check
        let double_vutrene = if double_game_mode {
            println!("We were playing double! Points doubled!");
            true
        } else if redouble_game_mode {
            println!("We were playing redouble! Points quadrupled!");
            true
        } else {
            false
        };
        if points_game[(player_last_bid_index + 1) % 2] > points_game[player_last_bid_index % 2] {
            points_game[(player_last_bid_index + 1) % 2] += points_game[player_last_bid_index % 2];
            points_game[player_last_bid_index % 2] = 0;
            println!("Team #{} is inside!", (player_last_bid_index % 2) + 1);
            println!(
                "Team #{} gets all points!",
                ((player_last_bid_index + 1) % 2) + 1
            );
        } else if double_vutrene {
            points_game[player_last_bid_index % 2] += points_game[(player_last_bid_index + 1) % 2];
            points_game[(player_last_bid_index + 1) % 2] = 0;
            println!("Team #{} is inside!", ((player_last_bid_index + 1) % 2) + 1);
            println!("Team #{} gets all points!", (player_last_bid_index % 2) + 1);
        }
        println!();

        //print points from game again
        for index in 0..2 {
            println!(
                "Team #{}'s points from game: {}",
                index + 1,
                points_game[index]
            );
        }
        println!();

        //round points of game
        round_points(&mut points_game, &mut points_total, game_mode);

        //check if game is hanging
        check_hanging_game(player_last_bid_index, hanging_points, &mut points_game);

        //check for winner
        if check_winner(&points_total) {
            break;
        }

        //move first player of next game with one
        init_hand_index = (init_hand_index + 1) % 4;
        println!("Enter any key to continue");
        user_input();
    }
}

fn check_hanging_game(
    player_last_bid_index: usize,
    mut hanging_points: usize,
    points_game: &mut [usize; 2],
) {
    //check for hanging
    //NOTE: check for hanging happens AFTER rounding the score
    if points_game[0] == points_game[1] {
        hanging_points += points_game[0];
        println!("The game is hanging!");
        println!(
            "Added points to team #{} and {} are hanging for next game!",
            (player_last_bid_index % 2) + 1,
            hanging_points
        );
    //check to add hanging points
    } else if hanging_points != 0 {
        let team_win_index = if points_game[0] > points_game[1] {
            0
        } else {
            1
        };
        points_game[team_win_index] += hanging_points;
        println!(
            "Team #{} gets the {} hanging points!",
            team_win_index, hanging_points
        );
        //reset hanging points
        hanging_points = 0;
    }
}

fn check_winner(points_total: &[usize; 2]) -> bool {
    if points_total[0] >= 151 || points_total[1] >= 151 {
        //at least one team has >=151
        if points_total[0] < 151 {
            //one team is >= 151, other isn't - winner
            println!("Team #{} is the winner!", 2);
            return true;
        }
        if points_total[1] < 151 {
            //one team is >= 151, other isn't - winner
            println!("Team #{} is the winner!", 1);
            return true;
        }
        //both teams >=151
        if points_total[0] > points_total[1] {
            //one team has more points
            println!("Team #{} is the winner!", 1);
            return true;
        }
        if points_total[1] > points_total[0] {
            //one team has more points
            println!("Team #{} is the winner!", 2);
            return true;
        }
        //both team have equal points and are >=151
        println!("Both teams have >=151 but are equal! Starting another game...");

        false
    } else {
        false
    }
}

fn check_kapo(point_decks: &[Vec<Card>; 2], points_game: &mut [usize; 2]) {
    //check for kapo, add points
    for index in 0..2 {
        if point_decks[index].is_empty() {
            println!(
                "Kapo! Team #{} gets 90 points extra!",
                ((index % 2) + 1) + 1
            );
            points_game[(index + 1) % 2] += 90;
        }
    }
}

fn round_points(points_total: &mut [usize; 2], points_game: &mut [usize; 2], game_mode: GameMode) {
    //round points according to game mode
    for index in 0..2 {
        let round_limit = match game_mode {
            OneTrump(_) => 6,
            AllTrumps => 4,
            NoTrumps => 5,
        };
        if points_game[index] % 10 >= round_limit {
            points_game[index] += 10;
        }
        points_total[index] += points_game[index] / 10;
        println!(
            "Team #{}'s total points: {}",
            index + 1,
            points_total[index]
        );
    } //points_game is now rounded
}

fn new_hands() -> [Vec<Card>; 4] {
    [vec![], vec![], vec![], vec![]]
}

fn add_cards(mut hands: [Vec<Card>; 4], deck: &mut Vec<Card>, num_add: usize) -> [Vec<Card>; 4] {
    //adds certain amount of cards to each hand and returns hands

    for index in 0..hands.len() {
        hands[index].extend(deck.iter().take(num_add));
        deck.drain(..num_add);
    }
    hands
}

fn new_game() -> ([Vec<Card>; 4], Vec<Card>) {
    //creates new empty hands
    //creates new shuffled deck
    //calls add_cards with FIRST_CARD_DEALING_NUM
    //prints hands and returns
    let mut hands = new_hands();
    let mut deck = generate_full_deck();
    hands = add_cards(hands, &mut deck, FIRST_CARD_DEALING_NUM);

    println!("\nStarting a new game!");
    println!("Added 5 cards to each hand:\n");
    for index in 0..4 {
        hands[index] = sort_hand(&mut hands[index], NO_TRUMP_ORDER);
        println!("player #{}:", index + 1);
        print_hand(&hands[index], false);
    }
    (hands, deck)
}

fn continue_game(
    mut hands: [Vec<Card>; 4],
    deck: &mut Vec<Card>,
    game_mode: GameMode,
    points_from_announs: &mut [usize; 2],
) -> [Vec<Card>; 4] {
    //adds 3 cards to each hand, prints and returns
    hands = add_cards(hands, deck, SECOND_CARD_DEALING_NUM);
    println!("\nContinuing game!");
    println!("Added 3 more cards to each hand. Good luck!\n");

    for index in 0..4 {
        //sorting hands' cards and printing them
        hands[index] = sort_hand_gamemode(&mut hands[index], game_mode);
        println!("player #{}:", index + 1);
        print_hand(&hands[index], false);
    }

    //not checking for carre and cards sequences in NoTrumps
    if game_mode != NoTrumps {
        for index in 0..4 {
            check_carre(&hands[index], index, points_from_announs);
        }
        card_sequences_validation(points_from_announs, &hands);
    }

    hands
}
