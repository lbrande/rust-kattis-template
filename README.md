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
let string = "6 3";
let (i, j): (u8, i32);
scan_str!(string, i, j);
```
### Scan and return rest
```rust
let string = "4 hello";
let (i, rest): (usize, String);
scan_str!(string, 2, i, rest);
```
### Scan into iterator
```rust
let string = "2 3 5 7 11";
let vec: Vec<i32> = scan_str!(string).collect();
```
## readln!
```rust
let line = readln!();
```
## read_lns!
```rust
let lines = readlns!();
```