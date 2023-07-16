mod deck;
use deck::Deck;
mod card;
use card::Card;
use std::io;
use color_print::{self, cprintln};


fn main() {
    let mut deck = Deck::new();
    let mut start = true;
    let mut player_total = 0;
    let mut house_total = 0;
    let mut player_cards: Vec<Card> = Vec::new();
    let mut house_cards: Vec<Card> = Vec::new();

    let card = deck.hit();
    house_cards.push(card);
    let card = deck.hit();
    house_cards.push(card);

    let card = deck.hit();
    player_cards.push(card);
    let card = deck.hit();
    player_cards.push(card);

    let mut input_text = String::new();
    // println!("H: Hit");
    // println!("S: Stay");
    // println!("Q: Quit");
    // println!("Help: show Commands");
    game_board(&deck, &player_cards, &house_cards, false);

    while start {
        print!("Write something: ");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        print!("{}", input_text.trim());
        println!("");
        match input_text.to_lowercase().trim() {
            "h" => {
                println!("You chose to hit.");
                let card = deck.hit();
                deck.print_card(&card);
                player_cards.push(card);
                // println!("your total: {}", player_total);
                game_board(&deck, &player_cards, &house_cards, false);
            }
            "s" => {
                println!("You chose to stay.");
                // println!("your total: {}", player_total);
                let housesum = deck.hand_sum(&house_cards);
                house_draws(housesum, &mut house_cards, &mut deck);
                start = false;
                game_board(&deck, &player_cards, &house_cards, true);
            }
            "q" => {
                println!("You chose to quit.");
                start = false;
            }
            "help" => {
                println!("H: Hit");
                println!("S: Stay");
                println!("Q: Quit");
                println!("Help: show Commands");
            }
            _ => {
                println!("Not a command");
            }
        }
        
        input_text.clear();
    }
}
fn house_draws(mut house_total: u32, house_cards: &mut Vec<Card>, deck: &mut Deck) {
    house_total = deck.hand_sum(&house_cards);
    while house_total <= 16 {
        let card = deck.hit();
        house_cards.push(card);
        house_total = deck.hand_sum(&house_cards);
    }
}
fn game_board(deck: &Deck, player_cards:&Vec<Card>, house_cards:&Vec<Card>, reveal:bool) {
    println!("----------------------------");
    println!("-----------HOUSE------------");
    if !reveal {
        print!("            ");
        deck.print_card(&house_cards[0]);
        cprintln!("<s>??</s>");
        println!("House Total: {}", house_cards[0].get_value().parse().unwrap_or(10));
    } else {
        let spaces = " ".repeat((28 - player_cards.len() * 2) / 2);
        print!("{}", spaces); // 15 lång när det är 0 karatärer
        deck.print_cards_in_hand(&house_cards);
        println!("");
            
        let housesum = deck.hand_sum(&house_cards);
        println!("House Total: {}", housesum.to_string());
    }
    println!("-----------PLAYER-----------");
    let spaces = " ".repeat((28 - player_cards.len() * 2) / 2);
    print!("{}", spaces);
    deck.print_cards_in_hand(player_cards);
    println!("");
    let playersum = deck.hand_sum(&player_cards);
    println!("Player Total: {}", playersum.to_string());
    println!("----------------------------");
    cprintln!("<rev>H</rev>it,       <rev>S</rev>tand,       <rev>Q</rev>uit");
}

