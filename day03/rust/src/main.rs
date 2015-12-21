fn slurp(filename : &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    return s;
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    use std::collections::HashSet;

    let s = slurp("../input.txt");
    let mut visited = HashSet::new();
    let mut santa = Point { x: 0, y: 0 };
    let mut robot = Point { x: 0, y: 0 };
    visited.insert(santa);
    for (n, c) in s.chars().enumerate() {
        let mut pos;
        if n % 2 == 0 {
            pos = &mut santa;
        } else {
            pos = &mut robot;
        }
        match c {
            '>' => pos.x += 1,
            '<' => pos.x -= 1,
            '^' => pos.y += 1,
            'v' => pos.y -= 1,
            _ => println!("Invalid input"),
        }
        visited.insert(*pos);
    }
    println!("{}", visited.len());
}
