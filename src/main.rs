use rand::{thread_rng, Rng};
use std::{io, str::FromStr};

#[derive(strum_macros::Display)]
enum Inputs {
    Rock,
    Paper,
    Scissor,
    Balls,
    Gun,
}

enum GameResult {
    Win,
    Draw,
    Lose,
}

impl std::str::FromStr for Inputs {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Inputs::Rock),
            "2" => Ok(Inputs::Paper),
            "3" => Ok(Inputs::Scissor),
            "pp" => Ok(Inputs::Balls),
            "gun" => Ok(Inputs::Gun),
            _ => Err(format!("{} is not an input!", s)),
        }
    }
}

fn determine_if_user_wins(user: Inputs, computer: Inputs) -> Result<GameResult, String> {
    match user {
        Inputs::Rock => match computer {
            Inputs::Scissor => Ok(GameResult::Win),
            Inputs::Rock => Ok(GameResult::Draw),
            _ => Ok(GameResult::Lose),
        },
        Inputs::Paper => match computer {
            Inputs::Rock => Ok(GameResult::Win),
            Inputs::Paper => Ok(GameResult::Draw),
            _ => Ok(GameResult::Lose),
        },
        Inputs::Scissor => match computer {
            Inputs::Paper => Ok(GameResult::Win),
            Inputs::Scissor => Ok(GameResult::Draw),
            _ => Ok(GameResult::Lose),
        },
        Inputs::Balls => Ok(GameResult::Win),
        Inputs::Gun => Ok(GameResult::Win),
    }
}

fn main() {
    let mut computer_wins = 0;
    let mut user_wins = 0;
    loop {
        println!("Stupid game rock paper sissors (i spelt it wrong ik)");
        let mut input = String::new();
        println!("========================");
        println!("WINS:");
        println!("Computer: {}", computer_wins);
        println!("User:     {}", user_wins);
        println!("========================");
        println!("Rock, paper, or scissor? Input number. Press 'Ctrl + C' to exit.");
        println!("    1. Rock\n    2. Paper\n    3. Scissor");
        let _n = io::stdin().read_line(&mut input);
        let input = input.trim();
        clearscreen::clear().expect("Failed to clear screen.");
        println!("You entered:       {}", input);
        let user_selection: Result<Inputs, String> = Inputs::from_str(input);

        if user_selection.is_err() {
            println!("Invalid/No input! Exiting...");
            return;
        }

        println!("You answered:      {}", user_selection.unwrap());
        let computer_rng = thread_rng().gen_range(1..4).to_string();
        let computer_selection: Inputs =
            Inputs::from_str(&computer_rng).expect("how did this fail???");
        println!("Computer answered: {}", computer_selection);

        match determine_if_user_wins(Inputs::from_str(input).unwrap(), computer_selection) {
            Ok(GameResult::Win) => {
                println!("You won!");
                user_wins += 1;
            }
            Ok(GameResult::Draw) => println!("Draw........"),
            Ok(GameResult::Lose) => {
                println!("You lost...");
                computer_wins += 1;
            }
            Err(_) => println!("An error occured when determining result."),
        }
    }
}
