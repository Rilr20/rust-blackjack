use crate::Card;
use color_print::{self, cprint, cprintln};
use rand::prelude::*;

pub struct Deck {
    size: u32,
    cards: Vec<Card>,
}
impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            size: 52,
            cards: Self::create_cards(),
        };
        deck
    }
    fn create_cards() -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        let suits = ['♥', '♦', '♣', '♠'];
        for suit in suits {
            for i in 1..=13 {
                match i {
                    1 => {
                        cards.push(Card::new(suit, String::from("A")));
                    }
                    11 => {
                        cards.push(Card::new(suit, String::from("J")));
                    }
                    12 => {
                        cards.push(Card::new(suit, String::from("Q")));
                    }
                    13 => {
                        cards.push(Card::new(suit, String::from("K")));
                    }
                    _ => {
                        cards.push(Card::new(suit, i.to_string()));
                    }
                }
            }
        }
        return cards;
    }
    pub fn hit(&mut self) -> Card {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..self.cards.len() - 1);
        self.cards.swap_remove(index)
    }
    pub fn print_deck(&self) {
        for card in &self.cards {
            if card.get_suit() == '♥' || card.get_suit() == '♦' {
                cprint!("<r>{}{}</r>", card.get_suit(), card.get_value());
            } else {
                cprint!("<rev>{}{}</rev>", card.get_suit(), card.get_value());
            }
        }
    }
    pub fn print_cards_in_hand(&self, cards: &Vec<Card>) {
        for card in cards {
            if card.get_suit() == '♥' || card.get_suit() == '♦' {
                cprint!("<r>{}{}</r>", card.get_suit(), card.get_value());
            } else {
                cprint!("<rev>{}{}</rev>", card.get_suit(), card.get_value());
            }
        }
    }
    pub fn print_card(&self, card: &Card) {
        if card.get_suit() == '♥' || card.get_suit() == '♦' {
            cprint!("<r>{}{}</r>", card.get_suit(), card.get_value());
        } else {
            cprint!("<rev>{}{}</rev>", card.get_suit(), card.get_value());
        }
    }
    pub fn get_size(self) -> u32 {
        self.size
    }
    pub fn hand_sum(&self, cards: &Vec<Card>) -> u32 {
        let mut sum = 0;

        for card in cards {
            card.get_value();
            sum = sum + card.get_value().parse().unwrap_or(10);
        }
        sum
    }
}
