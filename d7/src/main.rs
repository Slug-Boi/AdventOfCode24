use std::io;
use rayon::prelude::*;
use itertools::Itertools;

// https://www.researchgate.net/profile/Graham-Hutton-2/publication/246269864_FUNCTIONAL_PEARLS_The_countdown_problem/links/0c96053be57c017f1d000000/FUNCTIONAL-PEARLS-The-countdown-problem.pdf
fn main() {
    let _lines: Result<Vec<String>, std::io::Error> = io::stdin().lines().collect();
    // parse this input 190: 10 19
    let lines = _lines.unwrap().iter().map(|line| {
        let mut parts = line.split(": ");
        let target = parts.next().unwrap().parse::<i64>().unwrap();
        let numbers = parts.next().unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        (numbers, target)
    }).collect::<Vec<(Vec<i64>, i64)>>();

    let results = lines.par_iter().map(|(line, target)| {
        countdown(line.clone()[1..].to_vec(), *target, line[0])
    }).collect::<Vec<Option<Vec<i64>>>>();

    let mut sum = 0;

    for result in results {
        match result {
            Some(result) => {
                // sum all the result target values
                sum += result[0];

            },
            None => {
                //println!("Impossible");
            }
        }
    }

    println!("Sum: {}", sum);
}

fn countdown(line: Vec<i64>, target: i64, acc: i64) -> Option<Vec<i64>> {
    if line.len() == 0 {
        if acc == target {
            return Some(vec![acc]);
        } else {
            return None;
        }
    }
    
    let head = line[0];
    let tail = &line[1..];
    countdown(tail.to_vec(), target, acc + head)
        .or_else(|| countdown(tail.to_vec(), target, acc * head).or_else(|| countdown(tail.to_vec(), target, (acc.to_string() + &head.to_string()).parse::<i64>().unwrap())))
        .map(|mut x| {
            x.push(head);
            x
        })


}
