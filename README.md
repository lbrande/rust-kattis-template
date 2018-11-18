# Usage
1. Place `kattis.rs` into the same directory as `main.rs` (or clone and use this repository).
2. Add these lines to the top of `main.rs`:
    ```rust
    #[macro_use]
    mod kattis;
    ```
3. Include `kattis.rs` when submitting to Kattis.
# Examples
## scanln!
### Scan
```rust
let (i, j): (u8, i32);
scanln!(i, j);
```
### Scan and return rest
```rust
let (i, rest): (usize, String);
scanln!(2, i, rest);
```
### Scan into iterator
```rust
let vec: Vec<i32> = scanln!().collect();
```
## scanstr!
###Scan
```rust
let str = "6 3";
let (i, j): (u8, i32);
scanstr!(str, i, j);
```
### Scan and return rest
```rust
let str = "4 hello";
let (i, rest): (usize, String);
scanstr!(str, 2, i, rest);
```
### Scan into iterator
```rust
let str = "2 3 5 7 11";
let vec: Vec<i32> = scanstr!(str).collect();
```