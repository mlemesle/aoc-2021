use lib;

enum CharacterType {
    Parenthesis,
    Bracket,
    Brace,
    AngleBracket
}

fn get_chunk_error_score(chunk: &String) -> usize {
    let mut character_type_stack: Vec<CharacterType> = Vec::new();
    for c in chunk.chars() {
        match c {
            '(' => character_type_stack.push(CharacterType::Parenthesis),
            '{' => character_type_stack.push(CharacterType::Brace),
            '[' => character_type_stack.push(CharacterType::Bracket),
            '<' => character_type_stack.push(CharacterType::AngleBracket),
            ')' => if let Some(CharacterType::Parenthesis) = character_type_stack.pop() {} else {return 3}
            '}' => if let Some(CharacterType::Brace) = character_type_stack.pop() {} else {return 1197}
            ']' => if let Some(CharacterType::Bracket) = character_type_stack.pop() {} else {return 57}
            '>' => if let Some(CharacterType::AngleBracket) = character_type_stack.pop() {} else {return 25137}
            _ => println!("Unknown character {}", c)
        }
    }

    0
}

fn get_chunk_completion_score(chunk: &String) -> usize {
    let mut character_type_stack: Vec<CharacterType> = Vec::new();
    for c in chunk.chars() {
        match c {
            '(' => character_type_stack.push(CharacterType::Parenthesis),
            '{' => character_type_stack.push(CharacterType::Brace),
            '[' => character_type_stack.push(CharacterType::Bracket),
            '<' => character_type_stack.push(CharacterType::AngleBracket),
            ')' | '}' | ']' | '>' => {character_type_stack.pop().expect(&format!("Error trying to depop {}", c));},
            _ => println!("Unknown character {}", c)
        }
    }

    character_type_stack.iter().rev().fold(0, |agg, char| agg * 5 + match char {
        CharacterType::Parenthesis => 1,
        CharacterType::Bracket => 2,
        CharacterType::Brace => 3,
        CharacterType::AngleBracket => 4
    })
}

fn part1() {
    let error_score: usize = lib::input::<String>("input.txt")
        .map(|line| get_chunk_error_score(&line))
        .sum();
    
    println!("Error code is {}", error_score);
}

fn part2() {
    let mut completion_scores = lib::input::<String>("input.txt")
        .filter(|line| get_chunk_error_score(line) == 0)
        .map(|line| get_chunk_completion_score(&line))
        .collect::<Vec<usize>>();
    completion_scores.sort();

    println!("Completion code is {}", completion_scores[completion_scores.len() / 2]);
}

fn main() {
    part1();
    part2();
}