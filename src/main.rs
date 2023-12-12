use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();
    let responses = vec![
        "Yes",
        "No",
        "Maybe",
        "Ask again later",
        "Cannot predict now",
        "Don't count on it",
        "Most likely",
        "Outlook not so good",
    ];
    println!("Welcome to the Magic 8-Ball!");

    loop {
        print!("Ask a yes/no question (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut question = String::new();
        io::stdin().read_line(&mut question).unwrap();

        let question = question.trim();

        if question.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }
        if question.is_empty() {
            println!("Please enter a question.");
            continue;
        }
        let response_index = rng.gen_range(0..responses.len());
        let response = responses[response_index];
        println!("Magic 8-Ball says: {}", response);
    }
}