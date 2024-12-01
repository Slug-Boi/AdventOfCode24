use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    

    let (mut left, mut right):(Vec<i32>,Vec<i32>) = std::io::stdin().lines().map(|x| x.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap()).unzip();

    left.sort();
    right.sort();

    let sum = left.iter().zip(right.iter()).map(|(x,y)| (x-y).abs()).sum::<i32>();

    println!("{}",sum);

    let mut map = HashMap::new();
    

    right.into_iter().for_each(|x| {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    });

    let mut count_sum = 0;

    left.into_iter().for_each(|x| {
        count_sum += map.get(&x).unwrap_or(&0) * x;
    });


    println!("{}",count_sum);
}
