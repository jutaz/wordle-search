use dialoguer::{theme::ColorfulTheme, Input};
use std::{process};

mod dict_new;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];

struct SearchArgs {
    result: Vec<String>,
    must_have: Vec<char>,
}

fn parse_into_args (guess: String, ignored_chars: &Vec<char>) -> SearchArgs {
    let mut result: Vec<String> = Vec::new();
    let split = guess.split("");
    let mut must_have: Vec<char> = Vec::new();

    let mut skip = false;

    let chars_to_match: String = format!(
        "[{}]",
        ASCII_LOWER
            .clone()
            .iter()
            .filter(|chr| !ignored_chars.contains(chr))
            .cloned()
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>()
    );

    for (pos, char) in split.enumerate() {
        if skip || char.to_owned() == "" {
            skip = false;
            continue;
        }

        if char.to_owned() == "*" {
            must_have.push(guess.chars().nth(pos).unwrap());
            result.push(
                chars_to_match
                .to_string()
                .clone()
                .replace(guess.chars().nth(pos).unwrap(), "")
            );
            skip = true;
        } else if char.to_owned() == "?" {
            result.push(chars_to_match.to_string());
        } else {
            result.push(char.to_owned());
        }
    }

    return SearchArgs {
        result,
        must_have
    };
}

fn main() {
    println!("Enter a word, substituting `?` for unknown characters.");
    println!("If there's a yellow letter, prefix it with `*`.");

    let mut rm_mode = false;
    let mut ignored_chars: Vec<char> = Vec::new();

    loop {
        if let Ok(cmd) = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt(match rm_mode {
                true => "Remove chars",
                false => "Wordle solver."
            })
            .interact_text()
        {
            match cmd.chars().nth(0).unwrap() {
                ':' => {
                    match cmd.as_ref() {
                        ":q" => process::exit(0),
                        ":rm" => {
                            rm_mode = true;
                            continue;
                        },
                        ":c" => {
                            println!("Clearing state.");
                            ignored_chars.clear();
                        }
                        _ => println!("Unknown command.")
                    }
                },
                _ => {
                    if rm_mode {
                        for (_pos, char) in cmd.split("").enumerate() {
                            if "" != char {
                                ignored_chars.push(char.chars().nth(0).unwrap());
                            }
                        }

                        rm_mode = false;
                        continue;
                    }
                    let args = parse_into_args(cmd, &ignored_chars);
                    if args.result.len() != 5 {
                        println!("Must be 5 slots.");
                        continue;
                    }

                    match crate::dict_new::search(args.result, args.must_have) {
                        Err(err) => {
                            println!("Error, {:?}", err);
                            process::exit(1);
                        },
                        Ok(count) => {
                            match count {
                                0 => continue,
                                1 => process::exit(0),
                                _ => continue,
                            }
                        },
                    }
                }
            }
        }
    }
}
