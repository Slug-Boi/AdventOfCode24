use std::io;
use regex::Regex;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // let res = parse_mul(input);
    
    // println!("{}", res);

    let res = parse_mul_do(input);

    println!("{}", res);    
}   


fn parse_mul(str: String) -> i32 {
    let mut res = 0;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let cap: Vec<String> = re.captures_iter(&str).map(|cap| cap.get(0).unwrap().as_str().to_string()).collect();

    for i in cap {
        let split: Vec<&str> = i.split(',').collect();
        let a = split[0][4..].parse::<i32>().unwrap();
        let b = split[1].split(')').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        
        res = res + (a * b);
    };

    res
}

fn parse_mul_do(str: String) -> i32 {
    let mut toggle = true;

    let mut res = 0;

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let do_re = Regex::new(r"do()").unwrap();
    let dont_re = Regex::new(r"dont()").unwrap();
    let cap: Vec<String> = re.captures_iter(&str).map(|cap| cap.get(0).unwrap().as_str().to_string()).collect();

    for i in cap {
        if i == "don't()" {
            toggle = false;
        } else if i == "do()" {
            toggle = true;
        } else {
            if toggle {
                let split: Vec<&str> = i.split(',').collect();
                let a = split[0][4..].parse::<i32>().unwrap();
                let b = split[1].split(')').collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
                
                res = res + (a * b);
            }
        }
    };

    res
}