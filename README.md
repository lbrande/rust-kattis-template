# Usage
1. Place `kattis.rs` into the same directory as `main.rs` (or clone this repository).
2. Add these lines to the top of `main.rs`:
    ```rust
    #[macro_use]
    mod kattis;
    ```
# Examples
## scanln!
```rust
let (i, j): (u8, i32);
scanln!(i, j);
```
### Scan with rest
```rust
let (i, rest): (usize, String);
scanln!(2, i, rest);
```
### Scan all
```rust
let vec: Vec<i32> = scanln!().collect();
```
## scanstr!
```rust
let str = "6 3";
let (i, j): (u8, i32);
scanstr!(str, i, j);
```
### Scan with rest
```rust
let str = "4 hello";
let (i, rest): (usize, String);
scanstr!(str, 2, i, rest);
```
### Scan all
```rust
let str = "2 3 5 7 11";
let vec: Vec<i32> = scanstr!(str).collect();
```