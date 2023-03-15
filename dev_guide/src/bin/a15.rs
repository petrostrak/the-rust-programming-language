// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32),
}

impl Ticket {
    fn print_info(&self) {
        match self {
            Self::Backstage(name, price) => {
                println!("Ticket: Backstage, Name: {}, Price: {}", name, price)
            }
            Self::Vip(name, price) => println!("Ticket: VIP, Name: {}, Price: {}", name, price),
            Self::Standard(price) => println!("Ticket: Standard, Price: {}", price),
        }
    }
}

fn main() {
    let bs: Ticket = Ticket::Backstage(String::from("Pit"), 65);
    let vip: Ticket = Ticket::Vip(String::from("Maggie"), 105);
    let std: Ticket = Ticket::Standard(45);

    let tickets: Vec<Ticket> = vec![bs, vip, std];

    for ticket in tickets.iter() {
        ticket.print_info()
    }
}
