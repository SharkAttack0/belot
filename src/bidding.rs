use crate::general_enums::{self, CardSuits::*, GameMode::*};
#[derive(Debug, PartialEq)]
pub enum Bidding {
    Pass,
    Double(general_enums::GameMode),
    ReDouble(general_enums::GameMode),
    GameMode(general_enums::GameMode),
}

pub mod bidding {
    use super::Bidding;
    use crate::general_enums::{self, CardSuits::*, GameMode::*};
    use crate::user_input_to_int;

    pub fn bid_phase(init_player: usize) -> (Bidding, usize) {
        //ask players to bid, returns game mode when passes game's conditions
        //returns index of last player who bid (required for unrelated checks)

        let mut last_game_mode_index = 0;
        let mut current_player = init_player;
        let mut pass_counter = 0;
        let mut last_player_who_bid = 4;
        let mut double_bid = false;
        loop {
            println!("player #{}", current_player + 1);
            let current_bid;
            //check if bidding has occured
            if last_game_mode_index == 0 {
                println!("No one has bid yet");
                current_bid = ask_bid(7);
            } else {
                println!(
                    "Current bid: {} from player #{}",
                    match last_game_mode_index {
                        1 => "All Trumps",
                        2 => "No Trumps",
                        3 => "Spades",
                        4 => "Hearts",
                        5 => "Diamonds",
                        6 => "Clubs",
                        _ => panic!("at bidding() user input out of bounds!"),
                    },
                    last_player_who_bid + 1
                );

                current_bid = ask_bid(last_game_mode_index);
            };
            //check if not pass
            if current_bid != 0 {
                last_game_mode_index = current_bid;
                last_player_who_bid = current_player;
                pass_counter = 0;
            } else {
                pass_counter += 1;
                //check 3 passes after bid
                if last_game_mode_index != 0 && pass_counter == 3 {
                    break;
                }
                //check 4 passes for no bid - restart game
                if last_game_mode_index == 0 && pass_counter == 4 {
                    break;
                }
            }

            //move player to ask with one
            current_player = (current_player + 1) % 4;
        }

        (
            match last_game_mode_index {
                0 => Bidding::Pass,
                1 => Bidding::GameMode(AllTrumps),
                2 => Bidding::GameMode(NoTrumps),
                3 => Bidding::GameMode(OneTrump(Spades)),
                4 => Bidding::GameMode(OneTrump(Hearts)),
                5 => Bidding::GameMode(OneTrump(Diamonds)),
                6 => Bidding::GameMode(OneTrump(Clubs)),
                _ => panic!("at bidding() user input out of bounds!"),
            },
            last_player_who_bid,
        )
    }

    fn ask_bid(last_game_mode_index: usize) -> usize {
        //prints possible bidding options and returns user input
        let game_mode_string = [
            "Pass",
            "AllTrumps",
            "NoTrumps",
            "Spades",
            "Hearts",
            "Diamonds",
            "Clubs",
        ];
        println!("\nSelect your bid: ");
        for index in 0..last_game_mode_index {
            println!("{}:\t{}", index + 1, game_mode_string[index]);
        }
        println!();
        if last_game_mode_index > 1 {
            println!("{}: \tDouble", last_game_mode_index + 1);
            return user_input_to_int(last_game_mode_index + 1);
        }

        user_input_to_int(last_game_mode_index)
    }
}

