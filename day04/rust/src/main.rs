extern crate crypto;

fn main() {
    use crypto::md5::Md5;
    use crypto::digest::Digest;

    let mut m = Md5::new();
    let secret = "yzbqklnj".to_string();
    let mut n = 0;
    loop {
        n += 1;
        let input = secret.clone() + &n.to_string();
        m.reset();
        m.input_str(&input);
        if m.result_str().starts_with("000000") {
            println!("{} md5: {}", input, m.result_str());
            break;
        }
    }
}
