use rand::Rng;

// Suit enum: Represents the four suits in a deck of cards.
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    // Randomly selects and returns a suit.
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=4) {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Unexpected value"),
        }
    }

    // Translates a number (1-4) to a corresponding suit.
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

// Rank enum: Represents the ranks in a deck, including Ace, Jack, Queen, King, and numbers 2-10.
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Rank {
    // Randomly selects and returns a rank.
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..=13) {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }

    // Translates a number (1-13) to a corresponding rank.
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n if n >= 2 && n <= 10 => Rank::Number(n),
            _ => panic!("Invalid rank value"),
        }
    }
}

// Card struct: Represents a single card in the deck, consisting of a rank and a suit.
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Function that checks if the card is the Ace of Spades.
pub fn winner_card(card: &Card) -> bool {
    match (card.rank, card.suit) {
        (Rank::Ace, Suit::Spade) => true,
        _ => false,
    }
}
