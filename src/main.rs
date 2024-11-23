mod getch;

use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login { name: String },
    Logout,
    Deposit { amount: f64 },
    Withdraw { amount: f64 },
    Transfer { amount: f64, to: String },
}

impl Commands {
    fn variants() -> &'static [&'static str] {
        &["login", "logout", "deposit", "withdraw", "transfer"]
    }
}

struct Balance {
    amount: f64,
}

impl Default for Balance {
    fn default() -> Self {
        Self { amount: 0.0 }
    }
}

// Let's have a state machine to handle the commands
struct StateMachine {
    logged_in_user: Option<String>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            logged_in_user: None,
        }
    }
}

fn get_or_create_balance<'a>(
    balances: &'a mut HashMap<String, Balance>,
    user: &str,
) -> &'a mut Balance {
    balances
        .entry(user.to_string())
        .or_insert_with(Balance::default)
}

fn main() {
    println!("Welcome to the Bank Bank Toot ATM! Type 'exit' to quit.");

    let mut state = StateMachine::new();
    let mut balances: HashMap<String, Balance> = HashMap::new();

    let commands = Commands::variants();

    loop {
        atm_rust::printterm("");

        let input = atm_rust::read_line_with_autocomplete(commands);
        if input == "exit" {
            break;
        }

        // Implement ctrl + D shortcut
        if input.is_empty() {
            break;
        }

        // Add a dummy program name to the start of the args
        let mut args = vec!["program".to_string()];
        args.extend(input.split_whitespace().map(String::from));
        let cli = Cli::try_parse_from(args);

        let is_logged_in = state.logged_in_user.is_some();
        if !is_logged_in {
            // Check command before login
            match &cli {
                Ok(cli) => match &cli.command {
                    Some(Commands::Login { name }) => {
                        state.logged_in_user = Some(name.clone());
                        println!("You are already logged in as {}", name);
                    }
                    None | Some(_) => {
                        println!("You are not logged in. Please `login` to continue");
                    }
                },
                Err(_e) => println!("Command not recognized"),
            }
            continue;
        }

        let user = state.logged_in_user.clone().unwrap();

        // Check command after login
        match &cli {
            Ok(cli) => match &cli.command {
                Some(Commands::Logout) => {
                    state.logged_in_user = None;
                    println!("Logging out");
                    break;
                }
                Some(Commands::Deposit { amount }) => {
                    let balance = get_or_create_balance(&mut balances, &user);
                    balance.amount += amount;

                    println!("Deposited ${}", amount);
                    println!("New balance: ${}", balance.amount);
                }

                Some(Commands::Withdraw { amount }) => {
                    let balance = get_or_create_balance(&mut balances, &user);
                    if balance.amount < *amount {
                        println!("Insufficient funds. Your balance is ${}", balance.amount);
                        continue;
                    }
                    balance.amount -= amount;
                    println!("Withdrew ${}", amount);
                    println!("New balance: ${}", balance.amount);
                }

                Some(Commands::Transfer { amount, to }) => {
                    let balance = get_or_create_balance(&mut balances, &user);
                    if balance.amount < *amount {
                        println!("Insufficient funds. Your balance is ${}", balance.amount);
                        continue;
                    }
                    balance.amount -= amount;
                    let new_balance = balance.amount;

                    let to_balance = get_or_create_balance(&mut balances, to);
                    to_balance.amount += amount;

                    println!("Transferred ${} to {}", amount, to);
                    println!("New balance: ${}", new_balance);
                }
                Some(Commands::Login { name }) => println!("You are already logged in as {}", name),
                None => println!("No command specified"),
            },
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("Goodbye!");
}
