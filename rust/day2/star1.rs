use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    //print!("{}",input);
    let games : Vec<&str> = input.split("\n").collect();
    let mut points = 0;
    for val in games {
        let mut iter = val.chars();
        let first = iter.next().unwrap().to_digit(36).unwrap() - 9;
        let second = iter.nth(1).unwrap().to_digit(36).unwrap() - 32;
        points += second;
        print!("{},{}\n", first,second);
         if first == second {
             points += 3;
        } else if (9 + second - first)%3 == 1{
             points += 6;
             print!("won\n");
         }

    print!("{}", points)
}