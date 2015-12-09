extern crate regex;

fn main () {
    use std::io::prelude::*;
    use std::fs::File;
    use regex::Regex;

    let mut f = File::open("../input.txt").unwrap();
    let mut str = String::new();
    f.read_to_string(&mut str).unwrap();

    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut res = 0;
    for cap in re.captures_iter(&str) {
        let l = cap.at(1).unwrap().parse::<i32>().unwrap();
        let w = cap.at(2).unwrap().parse::<i32>().unwrap();
        let h = cap.at(3).unwrap().parse::<i32>().unwrap();
        let areas = vec![l * w, w * h, h * l];
        let slack = areas.iter().min().unwrap();
        let paper = areas.iter().map(|&x| 2 * x).fold(0, std::ops::Add::add);
        res += paper + slack;
    }

    println!("{}", res);
}
