# Cinner (pronounced "Sinner") is an utility to write I/O operation in C++ style

### It features C++ like style for `cin` and `cout`
<br>

---

## Why?

- Because I'm bored of writing `std::io::stdin().read_line(&mut holder).trim().parse::<T>()` and `println!("{}", <variable>)` all the time

- Because I didn't find any other library that does this

- Because I didn't have anything to do

- Because ~~I'm a C++ programmer~~ I like C++ style

### Also, I wanted to learn more "advanced" Rust lmao

### I learnt:

- `Operator overloading` with `traits` (I'm still not sure if I did it right)

- The importance of `.trim()` (I forgot to add it in the first version and it took >5hrs to find the bug) 

- Using `trait bounds` in `generic functions`

- `Variadics` (they're so cool) 

- Publishing crates to `crates.io`

# Usage

- Add `cinner = "<version-no>"` to your `Cargo.toml` file

### You can use:

```rust
use cinner::{cin, cout};

fn main(){
    let mut i = 0;
    let _ = cin >> &mut i;
    let _ = cout << i << "\n";
}
```
### As opposed to:
```rust
fn main(){
    let mut holder = String::new();
    let i = std::io::stdin().read_line(&mut holder).trim().parse::<i32>().unwrap();
    println!("{}", i);
}
```

## Also it's recursive, so you can do:
```rust
use cinner::{cin, cout};

fn main(){
    let mut i = 0;
    let mut j = 0_f32;
    let _ = cin >> &mut i >> &mut j;
    let _ = cout << i << "\n";
}
```

## If you have any suggestions, `DCDA`


© 2022 Sarad (Chronos Ltd.)
