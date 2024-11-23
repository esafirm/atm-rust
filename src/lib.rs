mod getch;

use std::io::{self, Write};
use termion::color::{self, Bg};

/// Handle the input from the user
pub fn read_line_with_autocomplete(commands: &[&str]) -> String {
    let mut input = String::new();
    let getch = getch::Getch::new();

    loop {
        let c = getch.getch().unwrap() as char;
        match c {
            // Handle enter
            '\n' => {
                println!();
                break;
            }
            // Handle tab completion
            '\t' => {
                let suggestions: Vec<&str> = commands
                    .iter()
                    .filter(|&&cmd| cmd.starts_with(&input))
                    .cloned()
                    .collect();

                if suggestions.len() == 1 {
                    input = suggestions[0].to_string();
                    printterm(&input);
                } else if suggestions.len() > 1 {
                    println!();
                    for suggestion in suggestions {
                        print_suggestion(suggestion);
                    }
                    printterm(&input);
                }
            }
            // Handle backspace
            '\x08' | '\x7f' => {
                if !input.is_empty() {
                    input.pop();
                    clear_line();
                    printterm(&input);
                }
            }
            // Handle other characters
            _ => {
                input.push(c);
                print!("{}", c);
                io::stdout().flush().unwrap();
            }
        }
    }

    input
}

/// Print the ATM prompt
pub fn printterm(input: &str) {
    print!("\ratm> {}", input);
    io::stdout().flush().unwrap();
}

fn print_suggestion(suggestion: &str) {
    write!(
        io::stdout(),
        "{}{}{}\n",
        Bg(color::LightGreen),
        suggestion,
        Bg(color::Reset)
    )
    .unwrap();
}

fn clear_line() {
    print!("\x1B[2K\x1B[G");
    io::stdout().flush().unwrap();
}
