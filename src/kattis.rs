#![allow(unused_macros)]
macro_rules! scanstr {
    ($str:expr,$($var:ident),+) => {
        let mut str = $str.split_whitespace();
        $($var = str.next().unwrap().parse().unwrap();)+
    };
    ($str:expr,$max:expr,$($var:ident),+) => {
        let mut str = $str.splitn($max, char::is_whitespace);
        $($var = str.next().unwrap().parse().unwrap();)+
    };
    ($str:expr) => (
        $str.split_whitespace().map(|s| s.parse().unwrap())
    )
}

macro_rules! scanln {
    ($($var:ident),+) => {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        scanstr!(line, $($var),+);
    };
    ($max:expr,$($var:ident),+) => {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        scanstr!(line, $max, $($var),+);
    };
    () => ({
        let mut line:String = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.clone().split_whitespace().map(|s| s.parse().unwrap())
    })
}