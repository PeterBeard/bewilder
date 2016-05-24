extern crate rand;
extern crate time;
extern crate getopts;

use rand::{thread_rng, sample};

use time::get_time;

use getopts::Options;

use std::io::{stdin, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::env;

/// Load a dictionary file into a useful structure
///
/// The resulting HashMap maps sorted strings to all of the words that contain those letters
/// e.g. dict['act'] = vec!['act', 'cat']
fn load_dictionary(filename: &str) -> HashMap<String, Vec<String>> {
    let fh = match File::open(filename) {
        Ok(handle) => handle,
        Err(e) => panic!("Failed to open dictionary {}: {}", filename, e),
    };
    let mut file = BufReader::new(&fh);
    let mut dict = HashMap::new();
    loop {
        let mut word = String::new();
        match file.read_line(&mut word) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                word = word.trim().to_uppercase();
                let sorted = sort_word(&word);
                if dict.contains_key(&sorted) {
                    let mut words: &mut Vec<String> = dict.get_mut(&sorted).unwrap();
                    words.push(word.to_string());
                } else {
                    let mut v: Vec<String> = Vec::new();
                    v.push(word.to_string());
                    dict.insert(sorted, v);
                }
            },
            Err(_) => {
                break;
            },
        }
    }
    dict
}

/// Put the letters in a word in alphabetical order
fn sort_word(word: &str) -> String {
    let mut sorted: Vec<char> = word.chars().collect();
    sorted.sort();
    sorted.into_iter().collect()
}

/// Generate a board using 16 letter dice
fn generate_board() -> [char; 16] {
    let mut rng = thread_rng();

    let dice = [
        ['A', 'A', 'C', 'I', 'O', 'T'],
        ['A', 'B', 'I', 'L', 'T', 'Y'],
        ['A', 'B', 'J', 'M', 'O', 'Q'],
        ['A', 'C', 'D', 'E', 'M', 'P'],
        ['A', 'C', 'E', 'L', 'R', 'S'],
        ['A', 'D', 'E', 'N', 'V', 'Z'],
        ['A', 'H', 'M', 'O', 'R', 'S'],
        ['B', 'I', 'F', 'O', 'R', 'X'],
        ['D', 'E', 'N', 'O', 'S', 'W'],
        ['D', 'K', 'N', 'O', 'T', 'U'],
        ['E', 'E', 'F', 'H', 'I', 'Y'],
        ['E', 'G', 'K', 'L', 'U', 'Y'],
        ['E', 'G', 'I', 'N', 'T', 'V'],
        ['E', 'H', 'I', 'N', 'P', 'S'],
        ['E', 'L', 'P', 'S', 'T', 'U'],
        ['G', 'I', 'L', 'R', 'U', 'W']
    ];
    let mut board: [char; 16] = ['1','1','1','1','1','1','1','1','1','1','1','1','1','1','1','1'];

    // Roll a die for each square
    for i in 0..16 {
        board[i] = *sample(&mut rng, dice[i].iter(), 1)[0];
    }

    // Then shuffle the squares
    for i in 0..16 {
        let j: usize = sample(&mut rng, 0..16, 1)[0];
        let tmp = board[i];
        board[i] = board[j];
        board[j] = tmp;
    }

    board
}

/// Display a board
fn display_board(board: &[char]) {
    // Box drawing characters
    const TL: char = '\u{250f}';
    const TR: char = '\u{2513}';
    const BL: char = '\u{2517}';
    const BR: char = '\u{251b}';
    const HUP: char = '\u{253b}';
    const HDOWN:char = '\u{2501}';
    const HLINE: char = '\u{2533}';
    const VLINE: char = '\u{2503}';
    const VRIGHT: char = '\u{2523}';
    const VLEFT: char = '\u{252b}';
    const CROSS: char = '\u{254b}';

    print!("{}", TL);
    for i in 0..19 {
        if i % 5 == 4 {
            print!("{}", HLINE);
        } else {
            print!("{}", HDOWN);
        }
    }
    println!("{}", TR);

    for i in 0..16 {
        print!("{}", VLINE);
        if board[i] == 'Q' {
            print!(" Qu ");
        } else {
            print!(" {}  ", board[i]);
        }
        if i % 4 == 3 {
            println!("{}", VLINE);
            if i < 15 {
                print!("{}", VRIGHT);
                for i in 0..19 {
                    if i % 5 == 4 {
                        print!("{}", CROSS);
                    } else {
                        print!("{}", HDOWN);
                    }
                }
                println!("{}", VLEFT);
            }
        }
    }
    
    print!("{}", BL);
    for i in 0..19 {
        if i % 5 == 4 {
            print!("{}", HUP);
        } else {
            print!("{}", HDOWN);
        }
    }
    println!("{}", BR);
}

/// Display a nice little scoreboard at the end of a game
fn display_score(words: &Vec<String>, dict: &HashMap<String, Vec<String>>, board: &[char]) {
    // Calculate the scores and sort the words from highest to lowest score
    let mut scored_words: Vec<(&str, u32)> = Vec::with_capacity(words.len());
    for w in words {
        let s = score_word(&w);
        if s > 0 && is_valid_word(&w, &dict, &board) {
            scored_words.push((w, s));
        }
    }
    let total = scored_words.iter().fold(0u32, |sum, &(_, s)| { sum + s });
    scored_words.sort_by(|&(wa, a), &(wb, b)| {
        if a == b {
            wa.cmp(&wb)
        } else {
            b.cmp(&a)
        }
    });


    println!("");
    print!("\u{250c}");
    for _ in 0..11 {
        print!("\u{2500}");
    }
    print!("[ Final Score ]");
    for _ in 0..11 {
        print!("\u{2500}");
    }
    println!("\u{2510}");
    println!("\u{2502}                                     \u{2502}");

    for (w, s) in scored_words {
        println!("\u{2502} {:>16} : {:<16} \u{2502}", w, s);
    }

    println!("\u{2502}                                     \u{2502}");
    print!("\u{251c}");
    for _ in 0..37 {
        print!("\u{2500}");
    }
    println!("\u{2524}");
    println!("\u{2502}                                     \u{2502}");
    println!("\u{2502}      Total score : {:<16} \u{2502}", total);
    println!("\u{2502}                                     \u{2502}");

    print!("\u{2514}");
    for _ in 0..37 {
        print!("\u{2500}");
    }
    println!("\u{2518}");
}

/// Calculate the score of a word
fn score_word(word: &str) -> u32 {
    match word.len() {
        0 => 0,
        1 => 0,
        2 => 0,
        3 => 1,
        4 => 1,
        5 => 2,
        6 => 3,
        7 => 5,
        _ => 11
    }
}

/// Determine whether a word is valid
fn is_valid_word(word: &str, dict: &HashMap<String, Vec<String>>, board: &[char]) -> bool {
    // First normalize the word by converting to upper case and replacing "QU" with "Q"
    let word = word.to_uppercase().replace("QU", "Q");
    
    // Next make sure that the word is in the dictionary
    let sorted_w = sort_word(&word);
    if !dict.contains_key(&sorted_w) {
        return false;
    } else {
        if !dict.get(&sorted_w).unwrap().contains(&word) {
            return false;
        }
    }
    
    // Now try to find the word on the board
    let mut visited = [false; 16];
    for pos in 0..16 {
        if word_continues_from(pos, &mut visited, 0, board, &word) {
            return true;
        }
    }

    false
}

/// Determine whether a word continues from a given point
fn word_continues_from(pos: usize, visited: &mut [bool], curr_ch: usize, board: &[char], word: &str) -> bool {
    // If we made it past the end of the word, we're done
    if curr_ch >= word.len() {
        return true;
    }

    let chars: Vec<char> = word.chars().collect();
    if board[pos] == chars[curr_ch] {
        // Check all the neighbors (except the square we came from) to see if
        // any of them contains the next letter
        visited[pos] = true;
        let neighbors = get_neighbor_positions(pos);
        for npos in neighbors {
            if !visited[npos] && word_continues_from(npos, visited, curr_ch+1, board, word) {
                return true;
            }
        }
    }
    visited[pos] = false;

    false
}

/// Get all of the squares that neighbor the given square
///
/// Returns a vec of the positions of the neighbors
fn get_neighbor_positions(pos: usize) -> Vec<usize> {
    // Convert the position to x, y coordinates
    let x = pos % 4;
    let y = pos / 4;

    let mut neighbors: Vec<usize> = Vec::new();
    if y > 0 {
        neighbors.push(pos-4);
    }
    if y < 3 {
        neighbors.push(pos+4);
    }

    if x > 0 {
        neighbors.push(pos-1);
        if y > 0 {
            neighbors.push(pos-5);
        }
        if y < 3 {
            neighbors.push(pos+3);
        }
    }
    if x < 3 {
        neighbors.push(pos+1);
        if y > 0 {
            neighbors.push(pos-3);
        }
        if y < 3 {
            neighbors.push(pos+5);
        }
    }
    neighbors
}

/// Search the board for all possible words
fn find_all_words(dict: &HashMap<String, Vec<String>>, board: &[char]) -> Vec<(String, u32)> {
    let mut found: Vec<(String, u32)> = Vec::new();
    for words in dict.values() {
        for w in words {
            let s = score_word(w);
            if s > 0 && !found.contains(&(w.to_string(), s)) && is_valid_word(&w, dict, board) {
                found.push((w.to_string(), s));
            }
        }
    }
    
    // Sort the words by score
    found.sort_by(|&(ref wa, a), &(ref wb, b)| {
        if a == b {
            wa.cmp(&wb)
        } else {
            b.cmp(&a)
        }
    });
    found
}

/// Show command usage and exit
fn display_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTION]\nPlay a game of Bewilder through the terminal.", program);
    
    print!("{}", opts.usage(&brief));

    println!("\nSee GitHub for source code and more docs: <https://github.com/PeterBeard/bewilder>");
}

fn main() {
    const MAX_TIME: i64 = 180;  // Default time limit is 3 minutes (180 s)
    const DICT_FILE: &'static str = "/usr/share/dict/american-english";

    // Handle command line args
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("c", "coach", "show best possible words after game end");
    opts.optflag("h", "help", "display this help and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => {
            panic!("Error parsing args: {}", e.to_string())
        },
    };

    if matches.opt_present("h") {
        display_help(&args[0], opts);
        std::process::exit(0);
    }

    println!("Welcome to Bewilder! Hang on a sec while I load the dictionary...\n");

    let dict = load_dictionary(DICT_FILE);

    let board = generate_board();
    display_board(&board);

    // Let the player enter words for until MAX_TIME
    let start = get_time().sec;
    let mut words: Vec<String> = Vec::new();
    println!("You have {} seconds to find as many words as you can! Type QQ to give up.", MAX_TIME);
    loop {
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => {
                let remaining = MAX_TIME - (get_time().sec - start);
                if remaining > 0 {
                    let w = input.trim().to_uppercase().to_string();
                    if w == "QQ" {
                        println!("You gave up with {} seconds left.", remaining);
                        break;
                    }
                    if !words.contains(&w) {
                        words.push(w);
                    } else {
                        println!("Already found {}", w);
                    }
                    if remaining % 10 == 0 {
                        println!("{} seconds remaining.", remaining);
                    }
                }
            },
            Err(error) => {
                println!("Not a word ({}).", error);
            },
        };

        if get_time().sec - start >= MAX_TIME {
            break;
        }
    }

    display_score(&words, &dict, &board);

    if matches.opt_present("c") {
        println!("\nLet me see what I can find...");

        let all_words = find_all_words(&dict, &board);
        println!("Here are the best words on this board:\n");

        for (w, s) in all_words {
            if s > 1 {
                println!("{:>16} : {:<16}", w, s);
            }
        }
    }
}
