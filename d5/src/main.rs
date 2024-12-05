use std::io;
use regex::Regex;
use string_builder::Builder;
use rayon::prelude::*;

fn main() {
    let mut input_line = String::new();
    let mut rules = Builder::default();
    let mut prints: Vec<String> = Vec::new();

    loop {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        if input_line.trim().is_empty() {
            break;
        }
        let split: Vec<&str> = input_line.trim().split("|").collect();
        let re_str = format!("{}.+{}|", split[1], split[0]);

        rules.append(re_str.clone());
        //println!("{}", &re_str);
    }

    let rules = rules.string().unwrap();
    let rules = rules[..rules.len()-1].to_string();

    let regex = Regex::new(&rules).unwrap();


    loop {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        if input_line.trim().is_empty() {
            break;
        }
        prints.push(input_line.trim().to_string());
    }

    let result = prints.par_iter().map(|x| check_order(x.to_string(), &regex)).collect::<Vec<Option<String>>>();

    let sum = result.iter().filter_map(|x| {
                                                if let Some(ref s) = x {
                                                    let split: Vec<&str> = s.split(",").collect();
                                                    let value = split[((split.len() as f64 / 2.0).ceil() as usize)-1].parse::<i32>().ok();
                                                    if let Some(ref v) = value {
                                                        // println!("{:?}", (split.len() as f64 / 2.0).ceil());
                                                        // println!("{:?}", v);
                                                        // println!("{:?}", split);
                                                    }
                                                    value
                                                } else {
                                                    None
                                                }
                                            }).sum::<i32>();

    dbg!(sum); 

    let result2 = prints.par_iter().map(|x| day_2(x.to_string(), &regex)).collect::<Vec<Option<String>>>();

    let sum2 = result2.par_iter().filter_map(|x| {
        if let Some(ref s) = x {
            if day_2(s.to_string(), &regex).is_some() {
                let mut s = s.to_string();
                loop {
                    match day_2(s.clone(), &regex) {
                        Some(new_s) => s = new_s,
                        None => break,
                    }
                }
                let split: Vec<&str> = s.split(",").collect();
                let value = split[((split.len() as f64 / 2.0).ceil() as usize) - 1].parse::<i32>().ok();
                if let Some(ref v) = value {
                    // println!("{:?}", (split.len() as f64 / 2.0).ceil());
                    // println!("{:?}", v);
                    // println!("{:?}", split);
                }
                value
            } else {
                let split: Vec<&str> = s.split(",").collect();
                let value = split[((split.len() as f64 / 2.0).ceil() as usize) - 1].parse::<i32>().ok();
                if let Some(ref v) = value {
                    // println!("{:?}", (split.len() as f64 / 2.0).ceil());
                    // println!("{:?}", v);
                    // println!("{:?}", split);
                }
                value
            }
        } else {
            None
        }
    }).sum::<i32>();
    dbg!(sum2);
}

fn check_order(input: String, regex: &Regex) -> Option<String> {
    if regex.is_match(&input) {
        return None;
    } else {
        return Some(input);
    }
}

fn day_2(input: String, regex: &Regex) -> Option<String> {
    if regex.is_match(&input) {
        let mut matches = regex.find_iter(&input).next().unwrap();
        let split = input.split(matches.as_str()).collect::<Vec<&str>>();
        let matches_str = matches.as_str().to_string();
        let list = matches_str.split(",").collect::<Vec<&str>>();
        let mut str = String::new();
        // put last element in front and first element in back
        if !split[0].is_empty() {
            str.push_str(&split[0]);
        }

        str.push_str(&list[list.len()-1]);
        for i in 1..list.len()-1 {
            if list[i].is_empty() {
                continue;
            }
            str.push_str(",");
            str.push_str(&list[i]);
        }
        str.push_str(",");
        str.push_str(&list[0]);
        if !split[1].is_empty() {
            str.push_str(&split[1]);
        }

        return Some(str);
    } else {
        return None
    }
}