use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let pairs = input.split("\n");
    let mut points = 0;
    for val in pairs {
        let mut iter = val.chars();
        let mut ranges : Vec<u32> = Vec::new();
        let nums : Vec<&str> = val.split(|c: char| !c.is_numeric()).collect();
        assert!(nums.len() == 4);
        //let ranges : Vec<u32> = Vec::new();
        for num in nums {ranges.push(num.parse::<u32>().unwrap())}
        print!("{}-{},{}-{}\n",ranges.get(0).unwrap(),ranges.get(1).unwrap(),ranges.get(2).unwrap(),ranges.get(3).unwrap());
        // let mut next = iter.next();
        // while next.is_some() {
        //     let ch = next.unwrap();
        //     if ch.is_numeric() {
        //         ranges.push(ch.to_digit(10).unwrap())
        //     }
        //     next = iter.next();
        // }
        if (ranges.get(0).unwrap() <= ranges.get(2).unwrap()) && (ranges.get(1).unwrap() >= ranges.get(3).unwrap()) {points += 1;}
        else if (ranges.get(0).unwrap() >= ranges.get(2).unwrap()) && (ranges.get(1).unwrap() <= ranges.get(3).unwrap()) {points += 1;}
    }

    print!("{}", points)
}