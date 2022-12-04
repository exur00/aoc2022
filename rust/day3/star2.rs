use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut backPacks = input.split("\n");
    let mut points = 0;
    let mut first = backPacks.next();
    let mut second = backPacks.next();
    let mut third = backPacks.next();
    while first.is_some() && second.is_some() && third.is_some() {
        let firstSet : HashSet<char> = first.unwrap().chars().collect();
        let secondSet : HashSet<char> = second.unwrap().chars().collect();
        let thirdSet : HashSet<char> = third.unwrap().chars().collect();
        //let firstIntersect : HashSet<char>= firstSet.intersection(&secondSet).cloned().collect();
        //let commonLetter = firstIntersect.intersection(&thirdSet).next().unwrap();
        let commonLetter = firstSet.intersection(&secondSet).cloned().collect::<HashSet<char>>().intersection(&thirdSet).next().unwrap().clone();

        match commonLetter {
            d if d.is_lowercase() => points += commonLetter.to_digit(36).unwrap() - 9,
            d if d.is_uppercase() => points += commonLetter.to_digit(36).unwrap() + 17,
            _ => points += 0,
        }

        first = backPacks.next();
        second = backPacks.next();
        third = backPacks.next();
    }

    print!("{}", points)
}