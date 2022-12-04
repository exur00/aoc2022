use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    print!("{}",input);
    let elves : Vec<&str> = input.split("\n\n").collect();
    let mut result: Vec<i32> = Vec::new();
    let elveIter = elves.iter();
    print!("dit is laatste element: {}\n\n", elves.iter().last().unwrap());
    for val in elveIter {
        let sp : Vec<&str> = val.split("\n").collect();
        let spIter = sp.iter();
        let mut tmpRes : i32 = 0;
        for food in spIter {
            print!("{}\n",food);
            let unwrap = food.parse::<i32>();
            if(unwrap.is_ok()) {tmpRes += unwrap.unwrap()}
            //tmpRes += (food.parse::<i32>().unwrap());
        }
        //let sum = tmpRes.iter().sum::<i32>();
        print!("{}\n\n", tmpRes);
        result.push(tmpRes);
    }
    print!("{}", result.iter().max().unwrap());
}