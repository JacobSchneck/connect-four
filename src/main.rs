#![allow(
    unused_must_use,
    unused_variables,
    dead_code,
    unreachable_code,
)]

use std::io::{Write};

use connect_four::*;

fn get_line() -> String {
    let mut input_buffer = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input_buffer);
    input_buffer.trim().to_string()
}

fn play_game() {
    println!("Instructions: ");
    println!("Enter a number 0 - 6 to place a piece");
    println!("To exit the game enter 'q'");
    println!("Lets get Started!\n");

    let mut game = ConnectFour::new();
    println!("{}", game);
    let mut exit_condition = false;
    while !exit_condition {
        print!("Player {} take your turn: ", game.turn());
        let line = get_line();
        if let Ok(col) = line.parse::<usize>(){
            match game.take_turn(col) {
                Ok(res) => {
                    exit_condition = res;
                    println!("{}", game);
                    if res {
                        if game.turn() == Player::Red { 
                            let winner = Player::Black;
                            println!("Congratulations player {}, you won", winner);
                        } else {
                            let winner = Player::Red;
                            println!("Congratulations player {}, you won\n ", winner);
                        }
                        println!("To play the game again please type: 'p' ");
                        println!("To quit the game please type: 'q' ");
                    }
                },
                Err(e) => eprintln!("{}", e),
            };
        } else {
            match line.as_str() {
                "q" => {
                    exit_condition = true;
                    println!("Exiting to main menu");
                    println!("To start over a new game press: 'p' ");
                    println!("To quit the game altogether please type: 'q' ");

                },
                "help" => {
                    println!("\nInstructions: ");
                    println!("Enter a number 0 - 6 to place a piece");
                    println!("To exit the game enter 'q'\n");
                }
                _ => println!("Sorry, command does not exist. Please type: 'help' for help."),

            }
        }

        // match line.as_str() {

        // };
    }
}

fn main() {
    println!("Welcome to Connect Four");
    println!("To play the game please type: 'p' ");
    println!("To quit the game please type: 'q' ");

    loop {
        print!("> ");
        let line = get_line();
        // let result = execute(line);
        match line.as_str() {
            "p" => play_game(),
            "q" => break,
            "help" => println!("p => plays the game \nq => quits the game"),
            _ => println!("Sorry, command does not exist. Please type: 'help' for help."),
        }
    }
}
