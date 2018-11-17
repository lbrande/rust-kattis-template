macro_rules! scanln {
    ($($type:ty),+) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        ($(line.next().unwrap().parse::<$type>().unwrap(),)+)
    })
}

fn main() {
    let _ = scanln!(i32);
}
