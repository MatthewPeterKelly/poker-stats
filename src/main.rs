mod card;

fn main() {
    for id in 0..52 {
        let card = card::Card { id };
        println!("Sorted:  {}", card);
    }

    print!("Five Cards: ");
    for my_card in card::Card::draw_five_cards() {
        print!("{} ", my_card);
    }
    println!("");

    print!("Seven Cards: ");
    for my_card in card::Card::draw_seven_cards() {
        print!("{} ", my_card);
    }
    println!("");
}
