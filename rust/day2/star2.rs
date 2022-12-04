use std::fs;

fn main() {
    let input = fs::read_to_string("test.txt").unwrap();
    //print!("{}",input);
    let games : Vec<&str> = input.split("\n").collect();
    let mut points = 0;
    for val in games {
        let mut iter = val.chars();
        let first :u32 = iter.next().unwrap().to_digit(36).unwrap() - 10;
        let outcome :u32 = match iter.nth(1).unwrap().to_digit(36).unwrap() - 32 {
            1 => 2,
            2 => 0,
            3 => 1,
            _ => 100,
        };
        let second :u32 = ((first + outcome) % 3)+1;
        points += second;
        print!("{},{},outcome:{}\n", first,second,outcome);
        // if first == second {
        //     points += 3;
        // } else if (9 + second - first)%3 == 1{
        //     points += 6;
        //     print!("won\n");
        // }
        if outcome == 0 {points += 3}
        if outcome == 1 {points += 6}
    }

    print!("{}", points)
}