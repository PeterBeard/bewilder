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


/// A Board has some (square) number of letter dice, usually 16, 25, or 36
#[derive(Clone, Debug)]
struct Board {
    squares: Vec<Die>,
}

impl Board {
    /// Create a new (empty) board
    pub fn new() -> Board {
        Board {
            squares: Vec::new(),
        }
    }

    /// Create a new board with the given dice
    pub fn with_dice(dice: Vec<Die>) -> Board {
        Board {
            squares: dice,
        }
    }

    /// Get the value at a particular position
    pub fn at(&self, pos: usize) -> char {
        self.squares[pos].value
    }

    /// Get the number of squares in this board
    pub fn len(&self) -> usize {
        self.squares.len()
    }

    /// Get the dimension of this board
    /// Returns the square root of the total length
    pub fn dim(&self) -> usize {
        (self.squares.len() as f32).sqrt() as usize
    }

    /// Shuffle the board
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();

        // Roll a die for each square
        for d in &mut self.squares {
            d.roll()
        }

        // Then shuffle the squares
        for i in 0..self.squares.len() {
            let j: usize = sample(&mut rng, 0..self.squares.len(), 1)[0];
            let tmp = self.squares[i].clone();
            self.squares[i] = self.squares[j].clone();
            self.squares[j] = tmp;
        }
    }
}

/// A Die has six letters, one on each face
#[derive(Clone, Debug)]
struct Die {
    letters: [char; 6],
    value: char,
}

impl Die {
    /// Create a new Die with the given letters
    pub fn new(letters: [char; 6]) -> Die {
        Die {
            letters: letters,
            value: '0',
        }
    }

    /// Roll this die
    pub fn roll(&mut self) {
        let mut rng = thread_rng();

        self.value = *sample(&mut rng, self.letters.iter(), 1)[0];
    }
}

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

/// Generate a 4x4 board
fn generate_board_4x4() -> Board {
    let dice: Vec<Die> = vec![
        Die::new(['A', 'A', 'C', 'I', 'O', 'T']),
        Die::new(['A', 'B', 'I', 'L', 'T', 'Y']),
        Die::new(['A', 'B', 'J', 'M', 'O', 'Q']),
        Die::new(['A', 'C', 'D', 'E', 'M', 'P']),
        Die::new(['A', 'C', 'E', 'L', 'R', 'S']),
        Die::new(['A', 'D', 'E', 'N', 'V', 'Z']),
        Die::new(['A', 'H', 'M', 'O', 'R', 'S']),
        Die::new(['B', 'I', 'F', 'O', 'R', 'X']),
        Die::new(['D', 'E', 'N', 'O', 'S', 'W']),
        Die::new(['D', 'K', 'N', 'O', 'T', 'U']),
        Die::new(['E', 'E', 'F', 'H', 'I', 'Y']),
        Die::new(['E', 'G', 'K', 'L', 'U', 'Y']),
        Die::new(['E', 'G', 'I', 'N', 'T', 'V']),
        Die::new(['E', 'H', 'I', 'N', 'P', 'S']),
        Die::new(['E', 'L', 'P', 'S', 'T', 'U']),
        Die::new(['G', 'I', 'L', 'R', 'U', 'W'])
    ];
    let mut board = Board::with_dice(dice);
    board.shuffle();
    board
}

/// Generate a 5x5 board
fn generate_board_5x5() -> Board {
    let dice: Vec<Die> = vec![
        Die::new(['A', 'A', 'A', 'F', 'R', 'S']),
        Die::new(['A', 'A', 'E', 'E', 'E', 'E']),
        Die::new(['A', 'A', 'F', 'I', 'R', 'S']),
        Die::new(['A', 'D', 'E', 'N', 'N', 'N']),
        Die::new(['A', 'E', 'E', 'E', 'E', 'M']),
        Die::new(['A', 'E', 'E', 'G', 'M', 'U']),
        Die::new(['A', 'E', 'G', 'M', 'N', 'N']),
        Die::new(['A', 'F', 'I', 'R', 'S', 'Y']),
        Die::new(['B', 'J', 'K', 'Q', 'X', 'Z']),
        Die::new(['C', 'C', 'E', 'N', 'S', 'T']),
        Die::new(['C', 'E', 'I', 'I', 'L', 'T']),
        Die::new(['C', 'E', 'I', 'L', 'P', 'T']),
        Die::new(['C', 'E', 'I', 'P', 'S', 'T']),
        Die::new(['D', 'D', 'H', 'N', 'O', 'T']),
        Die::new(['D', 'H', 'H', 'L', 'O', 'R']),
        Die::new(['D', 'H', 'L', 'N', 'O', 'R']),
        Die::new(['D', 'H', 'L', 'N', 'O', 'R']),
        Die::new(['E', 'I', 'I', 'I', 'T', 'T']),
        Die::new(['E', 'M', 'O', 'T', 'T', 'T']),
        Die::new(['E', 'N', 'S', 'S', 'S', 'U']),
        Die::new(['F', 'I', 'P', 'R', 'S', 'Y']),
        Die::new(['G', 'O', 'R', 'R', 'V', 'W']),
        Die::new(['I', 'P', 'R', 'R', 'R', 'Y']),
        Die::new(['N', 'O', 'O', 'T', 'U', 'W']),
        Die::new(['O', 'O', 'O', 'T', 'T', 'U']),
    ];
    let mut board = Board::with_dice(dice);
    board.shuffle();
    board
}

/// Display a board
fn display_board(board: &Board) {
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

    let dim = board.dim();
    let width = dim * 5 - 1;

    print!("{}", TL);
    for i in 0..width {
        if i % 5 == 4 {
            print!("{}", HLINE);
        } else {
            print!("{}", HDOWN);
        }
    }
    println!("{}", TR);

    for i in 0..board.len() {
        print!("{}", VLINE);
        if board.at(i) == 'Q' {
            print!(" Qu ");
        } else {
            print!(" {}  ", board.at(i));
        }
        if i % dim == (dim - 1) {
            println!("{}", VLINE);
            if i < board.len() - 1 {
                print!("{}", VRIGHT);
                for i in 0..width {
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
    for i in 0..width {
        if i % 5 == 4 {
            print!("{}", HUP);
        } else {
            print!("{}", HDOWN);
        }
    }
    println!("{}", BR);
}

/// Display a nice little scoreboard at the end of a game
fn display_score(words: &Vec<String>, dict: &HashMap<String, Vec<String>>, board: &Board) {
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
fn is_valid_word(word: &str, dict: &HashMap<String, Vec<String>>, board: &Board) -> bool {
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
    let mut visited: Vec<bool> = vec![false; board.len()];
    for pos in 0..board.len()-1 {
        if word_continues_from(pos, &mut visited, 0, board, &word) {
            return true;
        }
    }

    false
}

/// Determine whether a word continues from a given point
fn word_continues_from(pos: usize, visited: &mut [bool], curr_ch: usize, board: &Board, word: &str) -> bool {
    // If we made it past the end of the word, we're done
    if curr_ch >= word.len() {
        return true;
    }

    let chars: Vec<char> = word.chars().collect();
    if board.at(pos) == chars[curr_ch] {
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
fn find_all_words(dict: &HashMap<String, Vec<String>>, board: &Board) -> Vec<(String, u32)> {
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

/// Show command usage
fn display_help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [OPTION]\nPlay a game of Bewilder through the terminal.", program);
    
    print!("{}", opts.usage(&brief));

    println!("\nSee GitHub for source code and more docs: <https://github.com/PeterBeard/bewilder>");
}

fn main() {
    const MAX_TIME: i64 = 180;  // Default time limit is 3 minutes (180 s)
    const DEFAULT_DICT: &'static str = "/usr/share/dict/american-english";

    // Handle command line args
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("c", "coach", "show best possible words after game end");
    opts.optopt("d", "dict", "use a specific dictionary file", "FILE");
    opts.optopt("n", "numsquares", "generate an N by N board (default 4)", "N");
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

    let dict_file = match matches.opt_str("d") {
        Some(f) => { f },
        None => { DEFAULT_DICT.to_string() },
    };

    let dict = load_dictionary(&dict_file);

    let board = match matches.opt_str("n") {
        Some(n) => {
            if n == "4" {
                generate_board_4x4()
            } else if n == "5" {
                generate_board_5x5()
            } else {
                panic!("Invalid board size.");
            }
        },
        None => {
            generate_board_4x4()
        },
    };

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
