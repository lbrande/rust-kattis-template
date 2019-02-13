# Usage
Run `new.sh <name>` to create a new cargo project with rust-kattis-template set up.
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
## scan_str!
### Scan
```rust
let str = "6 3";
let (i, j): (u8, i32);
scan_str!(str, i, j);
```
### Scan and return rest
```rust
let str = "4 hello";
let (i, rest): (usize, String);
scan_str!(str, 2, i, rest);
```
### Scan into iterator
```rust
let str = "2 3 5 7 11";
let vec: Vec<i32> = scan_str!(str).collect();
```