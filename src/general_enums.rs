use strum::*;
pub mod constants {
    use crate::CardValue;
    use crate::CardValue::*;

    pub const FIRST_CARD_DEALING_NUM: usize = 5;
    pub const SECOND_CARD_DEALING_NUM: usize = 3;
    pub const NO_TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Jack, Queen, King, Ten, Ace];
    pub const TRUMP_ORDER: [CardValue; 8] = [Seven, Eight, Queen, King, Ten, Ace, Nine, Jack];
    pub const REGULAR_ORDER: [CardValue; 8] = [Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
}

#[derive(Debug, PartialEq, EnumIter, Clone, Copy)]
pub enum GameMode {
    OneTrump(CardSuits),
    NoTrumps,
    AllTrumps,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuits,
}

#[derive(Debug, PartialEq, EnumIter, Copy, Clone, Default)]
pub enum CardSuits {
    #[default]
    Clubs = 0,
    Diamonds = 1,
    Hearts = 2,
    Spades = 3,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq)]
pub enum CardValue {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}
