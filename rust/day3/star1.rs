use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let backPacks = input.split("\n");
    let mut points = 0;
    for val in backPacks {
        let length = val.len();
        let first : HashSet<char> = val.get(0..(length/2)).unwrap().chars().collect();
        let second : HashSet<char> = val.get((length/2)..length).unwrap().chars().collect();
        //let commonLetter = first.chars().any(|ch| second.contains(ch));
        let commonLetter = first.intersection(&second).next().unwrap();
        //print!("{}", commonLetter);

        match commonLetter {
            d if d.is_lowercase() => points += commonLetter.to_digit(36).unwrap() - 9,
            d if d.is_uppercase() => points += commonLetter.to_digit(36).unwrap() + 17,
            _ => points += 0,
        }
    }

    print!("{}", points)
}