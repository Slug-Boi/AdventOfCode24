use std::io;
use itertools::structs::TupleWindows;

fn main() {
    let input:Vec<Vec<i32>> = io::stdin().lines().map(
        |x| x.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()).collect();

    let window =  input.into_iter();

    let mut sum = 0;
        
    // window.for_each(|x| {
    //     if isSafe(x) {
    //         sum += 1;
    //     }
    // });
    //
    // println!("{}",sum);

    window.for_each(|x| {
        // try x if false try x+1 window brute force
        
        
        if isSafe(x.clone()) {
            sum += 1;
        } else {
            // try all permutations of removing an element from the vec and run isSafe on it
            for i in 0..x.len() {
                let mut temp = x.clone();
                temp.remove(i);
                if isSafe(temp) {
                    sum += 1;
                    break;
                }
            }
            }
        });
    println!("{}",sum);

}

fn isSafe(list:Vec<i32>) -> bool {

    let mut window1 =  list.windows(2).all(|x| x[0] < x[1]);
    
    let mut window2 =  list.windows(2).all(|x| x[0] > x[1]);

    let mut window3 = list.windows(2).all(|x| (x[0] - x[1]).abs() >= 1 && (x[0] - x[1]).abs() < 4);

    if window1 && window2 {
        return false;
    } else if window3 && (window1 || window2) {
        return true;
    } else {
        
        return false;
    }

    return false;

} 
