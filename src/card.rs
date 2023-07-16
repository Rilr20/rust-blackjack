pub struct Card {
    suit: char, // "heart,spade,diamond,clubs"
    value: String
}
impl Card {
    pub fn new(suit: char, value: String) -> Card {
        Card {suit, value}
    }
    pub fn get_suit(&self) -> char {
        self.suit
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
}