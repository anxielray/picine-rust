use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn random() -> Suit {
        let value = rand::thread_rng().gen_range(1..=4);
        Suit::translate(value)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value for suit: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn random() -> Rank {
        let value = rand::thread_rng().gen_range(1..=13);
        Rank::translate(value)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid value for rank: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
