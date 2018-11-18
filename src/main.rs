macro_rules! scanln {
    ($($var:ident),+) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        $($var = line.next().unwrap().parse().unwrap();)+
    })
}

fn main() {
    let i: i32;
    scanln!(i);
    println!("{}", i);
}
