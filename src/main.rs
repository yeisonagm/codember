use std::io::{self, Write};

mod challenge_01;
mod challenge_02;
mod challenge_03;

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

fn run_challenge(challenge: u8) {
    match challenge {
        1 => challenge_01::solve(),
        2 => challenge_02::solve(),
        3 => challenge_03::solve(),
        _ => println!("I do not add the challenge to the execution."),
    }
    println!();
}

fn main() {
    let challenges = 3;
    run_challenge(menu(challenges));
}
