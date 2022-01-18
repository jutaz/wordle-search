use dialoguer::{theme::ColorfulTheme, Input};
use std::{process};
use std::collections::HashMap;
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
    must_have: Vec<(usize, char)>,
}

fn parse_into_args (guess: String, ignored_chars: &Vec<char>, incorrect_position_chars: &mut HashMap<usize, Vec<char>>) -> SearchArgs {
    let mut result: Vec<String> = Vec::new();
    let split = guess.split("");
    let mut must_have: Vec<(usize, char)> = Vec::new();

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
            must_have.push((pos, guess.chars().nth(pos).unwrap()));

            let empty_vec = Vec::new();

            let to_remove: &Vec<char> = match incorrect_position_chars.get(&pos) {
                Some(chars) => chars,
                None => &empty_vec,
            };

            let mut current_pos_chars_to_match = chars_to_match
                .to_string()
                .clone();

            for chr in to_remove {
                current_pos_chars_to_match = current_pos_chars_to_match.replace(
                    *chr,
                    ""
                );
            }

            result.push(
                current_pos_chars_to_match
                .clone()
                .replace(guess.chars().nth(pos).unwrap(), "")
            );

            skip = true;
        } else if char.to_owned() == "?" {

            let empty_vec = Vec::new();

            let to_remove: &Vec<char> = match incorrect_position_chars.get(&(&pos-1)) {
                Some(chars) => chars,
                None => &empty_vec,
            };

            let mut current_pos_chars_to_match = chars_to_match
                .to_string()
                .clone();

            for chr in to_remove {
                current_pos_chars_to_match = current_pos_chars_to_match.replace(
                    *chr,
                    ""
                );
            }

            result.push(current_pos_chars_to_match.to_string());
        } else {
            result.push(char.to_owned());
        }
    }

    return SearchArgs {
        result,
        must_have
    };
}

fn get_must_haves(incorrect_position_chars: &mut HashMap<usize, Vec<char>>, must_have: Vec<(usize, char)>) -> Result<Vec<char>, Box<dyn std::error::Error>> {

    for (pos, chr) in must_have.iter() {
        match incorrect_position_chars.get_mut(&*pos) {
            Some(chars) => {
                chars.push(*chr);
            },
            None => {
                incorrect_position_chars.insert(*pos, vec![*chr]);
            }
        }
    }

    let must_haves: Vec<char> = incorrect_position_chars
        .clone()
        .into_values()
        .flatten()
        .collect();

    Ok(must_haves)
}

fn main() {
    println!("Enter a word, substituting `?` for unknown characters.");
    println!("If there's a yellow letter, prefix it with `*`.");

    let mut rm_mode = false;
    let mut ignored_chars: Vec<char> = Vec::new();
    let mut incorrect_position_chars = HashMap::new();

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
                            incorrect_position_chars.clear();
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
                    let args = parse_into_args(cmd, &ignored_chars, &mut incorrect_position_chars);
                    if args.result.len() != 5 {
                        println!("Must be 5 slots.");
                        continue;
                    }

                    match get_must_haves(&mut incorrect_position_chars, args.must_have) {
                        Err(_) => {
                            println!("Some error!.");
                        },
                        Ok(must_haves) => {
                            println!("must_haves, {:?}", must_haves);
                            match crate::dict_new::search(args.result, must_haves) {
                                Err(err) => {
                                    println!("Error, {:?}", err);
                                    process::exit(1);
                                },
                                Ok(count) => {
                                    match count {
                                        0 => continue,
                                        1 => {
                                            println!("Found answer, Clearing state.");
                                            ignored_chars.clear();
                                            incorrect_position_chars.clear();
                                        },
                                        _ => continue,
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
