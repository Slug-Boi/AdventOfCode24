use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // run over line of input and save coords of a char in a map of vectors
    let mut map: HashMap<char, Vec<(i32,i32)>> = HashMap::new();

    let mut line = String::new();
    let mut x = 0;
    let mut y = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    loop {
        line.clear();
        io::stdin().read_line(&mut line).unwrap();
        if line.len() > 0 {
            max_x = line.trim().len() as i32;
        }
        if line.trim().is_empty() {
            break;
        }

        for c in line.trim().chars() {
            if c != '.' {
                map.entry(c).or_insert(Vec::new()).push((x, y));
            }
            x += 1;
        }
        x = 0;
        y += 1;
        max_y = y;
    }

    let mut result = HashSet::new();

    for c in map.keys() {
        let set = vector(&map, *c, max_x, max_y);
        result.extend(set);
    }

    println!("{:?}", result.len());
}


fn vector(map: &HashMap<char, Vec<(i32,i32)>>, c: char, limit_x: i32, limit_y: i32) -> HashSet<(i32, i32)> {
    let list = map.get(&c).unwrap().to_vec();

    //dbg!(limit_x, limit_y);

    let mut new_vectors: Vec<(i32,i32)> = Vec::new();

    // create vectors for each pair of coords
    let mut vectors: Vec<(i32,i32)> = Vec::new();
    for v in &list {
        for v2 in &list {
            if v != v2 {
                let transform = (v2.0 - v.0, v2.1 - v.1);
                let mut mv2 = (v2.0, v2.1);
                loop {
                    if map.get(&c).unwrap().len() > 1 {
                        // add the antenna as a vector
                        new_vectors.push((mv2.0, mv2.1));
                    }
                    new_vectors.push((mv2.0 + transform.0, mv2.1 + transform.1));
                    mv2 = (mv2.0 + transform.0, mv2.1 + transform.1);
                    if mv2.0 < 0 || mv2.0 >= limit_x || mv2.1 < 0 || mv2.1 >= limit_y {
                        break;
                    }

                }
            }
        }
    }

    //dbg!(&new_vectors);

    //dbg!(&new_vectors);

    // check if the vectors are within the map limits

    let mut list_set: HashSet<(i32, i32)> = HashSet::new();

    for v in &new_vectors {
        let mut x = v.0;
        let mut y = v.1;

        //println!("{}: {} {}", c, x, y);

        if x >= 0 && x < limit_x && y >= 0 && y < limit_y {
            list_set.insert((x, y));
        }

    }

    //dbg!(&list_set);

    list_set

}