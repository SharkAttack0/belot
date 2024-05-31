use crate::general_enums::{Card, CardSuits, CardValue};
pub fn card(value: CardValue, suit: CardSuits) -> Card {
    Card { value, suit }
}
#[cfg(test)]
mod bots_tests {
    mod vlastni {

        use crate::{bots::get_vlastni, test::card, Card, CardSuits::*, CardValue::*, GameMode::*};

        #[test]
        fn empty_all() {
            let result: Vec<Card> = vec![];
            assert_eq!(result, get_vlastni(&vec![], &vec![], NoTrumps));
        }

        #[test]
        fn alltrumps_game_mode() {
            let hand: &Vec<Card> = &vec![
                card(Seven, Clubs),
                card(Eight, Clubs),
                card(Nine, Spades),
                card(Ace, Diamonds),
                card(Jack, Hearts),
                card(Ten, Clubs),
                card(Queen, Hearts),
                card(Ten, Diamonds),
            ];
            let other_cards: &Vec<Card> = &vec![
                card(Jack, Clubs),
                card(Eight, Spades),
                card(Seven, Spades),
                card(Nine, Hearts),
                card(Queen, Clubs),
            ];
            let result: Vec<Card> = vec![
                card(Ace, Diamonds),
                card(Ten, Diamonds),
                card(Jack, Hearts),
                card(Nine, Spades),
            ];
            assert_eq!(result, get_vlastni(hand, other_cards, AllTrumps));
        }

        #[test]
        fn full_hand() {
            let hand: &Vec<Card> = &vec![
                card(Seven, Clubs),
                card(Eight, Clubs),
                card(Nine, Spades),
                card(Ace, Diamonds),
                card(King, Hearts),
                card(Ten, Clubs),
                card(Queen, Hearts),
                card(Ten, Diamonds),
            ];
            let other_cards: &Vec<Card> = &vec![
                card(King, Clubs),
                card(Eight, Spades),
                card(Seven, Spades),
                card(Jack, Hearts),
                card(Queen, Clubs),
            ];
            let result: Vec<Card> = vec![
                card(Ten, Clubs),
                card(Ace, Diamonds),
                card(Ten, Diamonds),
                card(King, Hearts),
                card(Queen, Hearts),
                card(Nine, Spades),
            ];
            assert_eq!(result, get_vlastni(hand, other_cards, NoTrumps));
        }

        #[test]
        fn non_vlastni() {
            let hand: Vec<Card> = vec![card(Ten, Diamonds), card(Seven, Diamonds)];
            let other_cards: Vec<Card> = vec![card(Ace, Diamonds)];
            let vlastni_in_hand: Vec<Card> = vec![];
            assert_eq!(vlastni_in_hand, get_vlastni(&hand, &other_cards, NoTrumps));
        }

        #[test]
        fn no_other() {
            let hand = vec![
                card(Ace, Clubs),
                card(Seven, Clubs),
                card(Seven, Diamonds),
                card(Ten, Spades),
                card(Eight, Spades),
            ];

            let other_cards = vec![];
            let vlastni_in_hand = vec![
                card(Ace, Clubs),
                card(Seven, Clubs),
                card(Seven, Diamonds),
                card(Ten, Spades),
                card(Eight, Spades),
            ];
            assert_eq!(
                vlastni_in_hand,
                get_vlastni(&hand, &other_cards, crate::GameMode::NoTrumps)
            );
        }

        #[ignore]
        #[test]
        fn deduce_vlastni() {
            //this test checks for the commented part of the function (see it for info)
            let hand = vec![card(King, Clubs), card(Ace, Clubs)];
            let other_cards = vec![card(Ten, Clubs)];
            let vlastni_in_hand = vec![card(Ace, Clubs), card(King, Clubs)];
            assert_eq!(vlastni_in_hand, get_vlastni(&hand, &other_cards, NoTrumps));
        }
    }
    mod validate_valid_cards {
        use crate::{bots::valid_cards, cards_compare, test::card, Card, CardSuits::*, CardValue::*, GameMode::{self, *}};

        #[test]
        fn one_card_hand() {
            let game_mode: GameMode = OneTrump(Spades);
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Ace, Hearts),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Ace, Hearts),]);
        }

        #[test]
        fn two_valid_cards() {
            let game_mode: GameMode = OneTrump(Spades);
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Ace, Diamonds),
                card(Queen, Hearts),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), *hand);
        }

        #[test]
        fn two_non_valid_cards() {
            let game_mode: GameMode = OneTrump(Spades);
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Queen, Hearts),
                card(Eight, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Nine, Spades),]);
        }
        
        #[test]
        fn full_hand_no_trumps() {
            let game_mode: GameMode = NoTrumps;
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Queen, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Seven, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Queen, Hearts),card(Ten, Hearts),card(Seven, Hearts),]);
        }

        #[test]
        fn full_hand_all_trumps() {
            let game_mode: GameMode = AllTrumps;
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Jack, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Seven, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Jack, Hearts), card(Ten, Hearts),]);
        }

        #[test]
        fn full_hand_teammate_all_trumps() {
            let game_mode: GameMode = AllTrumps;
            let cards_in_play: &Vec<Card> = &vec![
                card(Seven, Hearts),
                card(King, Hearts),
                card(Seven, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Jack, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Eight, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Jack, Hearts), card(Ten, Hearts),]);
        }

        #[test]
        fn full_hand_one_trump_trump() {
            let game_mode: GameMode = OneTrump(Clubs);
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Queen, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Jack, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Seven, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Ten, Clubs),]);
        }

        #[test]
        fn full_hand_one_trump() {
            let game_mode: GameMode = OneTrump(Diamonds);
            let cards_in_play: &Vec<Card> = &vec![
                card(King, Hearts),
                card(Ace, Spades),
                card(Queen, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Jack, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Seven, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Jack, Hearts),card(Ten, Hearts),card(Seven, Hearts),]);
        }

        #[test]
        fn full_hand_one_trump_teammate() {
            let game_mode: GameMode = OneTrump(Hearts);
            let cards_in_play: &Vec<Card> = &vec![
                card(Ace, Spades),
                card(King, Hearts),
                card(Queen, Clubs),
            ];

            let hand = &vec![  
                card(Nine, Spades),
                card(Jack, Hearts),
                card(Eight, Clubs),
                card(Ten, Hearts),
                card(Seven, Hearts),
                card(Ace, Diamonds),
                card(Ten, Clubs),
            ];

            assert_eq!(valid_cards(hand, game_mode, &cards_in_play), vec![card(Jack, Hearts),
            card(Ten, Hearts),
            card(Seven, Hearts),
            ]);

        }
    }
}


mod general_checks {
    mod belot {
        use crate::{bots::get_vlastni, test::card, Card, CardSuits::*, CardValue::*, GameMode::*};
        use crate::eval_points::announcments::belot_check;
        #[test]
        fn one_trump_true() {
            let hand = &vec![card(King, Spades), card(Queen, Spades)];
            //assert_eq!(belot_check(hand, OneTrump(Spades), played_card, init_card));
        }

        #[test]
        fn one_trump_false() {

        }

        #[test]
        fn all_trump_true() {

        }
        
        #[test]
        fn all_trump_false() {

        }

        fn no_trump() {

        }

        fn first_player() {

        }

        fn full_hand() {

        }
    }
}