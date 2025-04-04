#[derive(Debug, PartialEq)]

pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        SimpleRng { state: seed }
    }

    // Linear Congruential Generator: (a * x + c) % m
    pub fn next(&mut self) -> u64 {
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        const M: u64 = 1 << 32;
        self.state = (A.wrapping_mul(self.state).wrapping_add(C)) % M;
        self.state
    }

    pub fn next_range(&mut self, min: u8, max: u8) -> u8 {
        min + (self.next() % ((max - min + 1) as u64)) as u8
    }
}

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }

    // Modify random to use SimpleRng
    pub fn random() -> Suit {
        let mut rng = SimpleRng::new(42); // Initialize RNG with a fixed seed, for test stability
        Suit::translate(rng.next_range(1, 4)) // Get a random Suit based on range 1 to 4
    }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            2..=10 => Rank::Number(value),
            _ => panic!("Invalid rank value"),
        }
    }

    // Modify random to use SimpleRng
    pub fn random() -> Rank {
        let mut rng = SimpleRng::new(42); // Initialize RNG with a fixed seed
        Rank::translate(rng.next_range(1, 13)) // Get a random Rank based on range 1 to 13
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// Function to check if the card is Ace of Spades
pub fn winner_card(card: Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}

/*
A standard deck of cards has 52 cards: 4 suits with 13 cards per suit. Represent the cards from a deck:

    Create an enum to represent the Suit.
    Implement the associated function random, which returns a random Suit (Heart, Diamond, Spade or Club).
    Create a Rank enum. For ace and face cards, it can be one of Ace, King, Queen or Jack. For the values from 2 to 10, it can have a Number value associated to a u8.
    Create an associated function to Rank called Random that returns a random Rank.
    Create a structure name Card which has the fields suit and rank.

Define:

    The associated function translate for Rank and Suit:
        For Suit, translate converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
        For Rank, translate converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
    The associated function random for Rank and Suit returns a random Rank and Suit respectively.
    Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.

Expected Functions and Structures

pub enum Suit {
}

pub enum Rank {
}

impl Suit {
    pub fn random() -> Suit {

    }

    pub fn translate(value: u8) -> Suit {
    }
}

impl Rank {
    pub fn random() -> Rank {
    }

    pub fn translate(value: u8) -> Rank {
    }
}

pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
}

Usage

Here is a program to test your function

use card_deck::*;

fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", your_card);

    if card_deck::winner_card(your_card) {
        println!("You are the winner!");
    }
}

And its output

$ cargo run
Your card is Card { suit: Club, rank: Ace }
$

*/