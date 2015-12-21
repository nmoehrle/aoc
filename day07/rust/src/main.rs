extern crate regex;

fn slurp(filename : &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(filename).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    return s;
}

#[derive(Clone)]
enum Signal {
    Value(u16),
    Gate(String, String, String),
    Wire(String, String), 
    Mod(String, String, u8),
}

fn main() {
    use std::collections::HashMap;
    use regex::Regex;

    let mut signals = HashMap::new(); 

    let s = slurp("../input.txt");
    let re_gate = Regex::new(r"^([a-z]+) (AND|OR) ([a-z]+) -> ([a-z]+)$").unwrap();
    let re_value = Regex::new(r"^(\d+) -> ([a-z]+)$").unwrap();
    let re_wire = Regex::new(r"^(?:(1 AND|NOT) )?([a-z]+) -> ([a-z]+)$").unwrap();
    let re_mod = Regex::new(r"^([a-z]+) (RSHIFT|LSHIFT) (\d+) -> ([a-z]+)$").unwrap();
    for line in s.lines() {
        match re_gate.captures(line) {
            Some(cap) => {
                let a = cap.at(1).unwrap().to_string();
                let op = cap.at(2).unwrap().to_string();
                let b = cap.at(3).unwrap().to_string();
                let o = cap.at(4).unwrap().to_string();
                signals.insert(o, Signal::Gate(a, op, b));
            }
            None => (),
        };
        match re_value.captures(line) {
            Some(cap) => {
                let v = cap.at(1).unwrap_or("0").parse::<u16>().unwrap();
                let o = cap.at(2).unwrap().to_string();
                signals.insert(o, Signal::Value(v));
            }
            None => (),
        };
        match re_wire.captures(line) {
            Some(cap) => {
                let op = cap.at(1).unwrap_or("").to_string();
                let a = cap.at(2).unwrap().to_string();
                let o = cap.at(3).unwrap().to_string();
                signals.insert(o, Signal::Wire(op, a));
            }
            None => (),
        };
        match re_mod.captures(line) {
            Some(cap) => {
                let a = cap.at(1).unwrap().to_string();
                let op = cap.at(2).unwrap().to_string();
                let p = cap.at(3).unwrap().parse::<u8>().unwrap();
                let o = cap.at(4).unwrap().to_string();
                signals.insert(o, Signal::Mod(a, op, p));
            }
            None => (),
        };
    }
   
    signals.insert("b".to_string(), Signal::Value(956));

    fn output(o : &String, mut signals : &mut HashMap<String, Signal>) -> u16 {
        let signal = signals.get(o).unwrap().clone();
        let ret = match signal {
            Signal::Value(v) => v,
            Signal::Gate(ref a, ref op, ref b) => {
                let oa = output(&a, &mut signals); 
                let ob = output(&b, &mut signals); 
                match op.as_ref() {
                    "AND" => oa & ob,
                    "OR" => oa | ob,
                    _ => panic!("Invalid input"),
                }
            },
            Signal::Mod(ref a, ref op, ref p) => {
                let oa = output(&a, &mut signals);
                match op.as_ref() {
                    "RSHIFT" => oa >> p,
                    "LSHIFT" => oa << p,
                    _ => panic!("Invalid input"),
                }
            },
            Signal::Wire(ref op, ref a) => {
                let oa = output(&a, &mut signals);
                match op.as_ref() {
                    "NOT" => !oa,
                    "1 AND" => 1 & oa,
                    "" => oa,
                    _ => panic!("Invalid input"),
                }
            },
        };
        signals.insert(o.clone(), Signal::Value(ret));
        return ret;
    };

    println!("{}", output(&"a".to_string(), &mut signals));
}
