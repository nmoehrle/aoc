extern crate regex;

fn slurp(filename : &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    return s;
}

fn main() {
    use regex::Regex;
    use std::cmp::max;

    let s = slurp("../input.txt");
    let mut lights: [i32; 1000 * 1000] = [0; 1000 * 1000];
    
    let re = Regex::new(r"(.+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for cap in re.captures_iter(&s) {
        let cmd = cap.at(1).unwrap();
        let lx = cap.at(2).unwrap().parse::<u32>().unwrap(); 
        let ly = cap.at(3).unwrap().parse::<u32>().unwrap(); 
        let hx = cap.at(4).unwrap().parse::<u32>().unwrap(); 
        let hy = cap.at(5).unwrap().parse::<u32>().unwrap();
        assert!(lx <= hx && ly <= hy);
        let action : Box<Fn(&mut i32)> = match cmd {
        //    "turn on" => Box::new(|x| {*x = 1}),
        //    "turn off" => Box::new(|x| {*x = 0}),
        //    "toggle" => Box::new(|x| {*x = if *x == 1 { 0 } else { 1 }}),
            "turn on" => Box::new(|x| {*x += 1}),
            "turn off" => Box::new(|x| {*x = max(0, *x - 1)}),
            "toggle" => Box::new(|x| {*x += 2}),
            _ => return
        };
        for y in ly..(hy + 1) {
            for x in lx..(hx + 1) {
                action(&mut lights[(y * 1000 + x) as usize]);
            }
        }
    }
    println!("{}", lights.iter().fold(0, |a, &b| a + b))
}
