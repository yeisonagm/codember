use std::io::{self, Write};

mod challenge_01;

fn menu(challenges: u8) -> u8 {
    println!("Restos de codember 2023👌");
    for i in 1..=challenges {
        println!("CHALLENGE 0{}", i);
    }
    
    let mut challenge = 0;
    
    // Read input
    while challenge == 0 || challenge > challenges{
        let mut input = String::new();
        print!(">>>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        challenge = input.trim().parse::<u8>().unwrap_or(0);
    }

    challenge
}

fn main() {
    let challenges = 5;
    let input = menu(challenges);
    
    println!("challenge {}", input);
}
