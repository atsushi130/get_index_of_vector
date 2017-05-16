# Get index of vector
"get index of vector" is Vector<T> with GetIndex<T> trait.

## Cargo.toml
Add dependencies get_index_of_vector.
```
[dependencies]
get_index_of_vector = "1.0.0"
```

## Usage
Return type of `Vector<T>.get_index(&self, &char)` is IndexResult used Result Pattern. So pattern match is available.
```rust
// import crete.
extern crete get_index_of_vector;
use get_index_of_vector::IndexResult;
use get_index_of_vector::GetIndex;

...

let characters = vec!['A', 'B', 'C', 'D', 'E'];
let character  = 'C';
match characters.get_index(&character) {
    IndexResult::Exist(index) => println!("index: {}, character: {}", index, characters[index]),
    IndexResult::None         => println!("Character is not exist.")
}
// ndex: 2, character: C
```

## Sample Code
```
$ cd src
$ cargo build
$ cargo run
```

## Setting up a Rust Development Environment (for Darwin)
```sh
$ brew install rust
$ rustc -V
rustc 1.17.0

$ cargo -V
cargo 0.18.0-dev (fe7b0cd 2017-04-24)
```
