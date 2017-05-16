extern crate get_index_of_vector;
use get_index_of_vector::IndexResult;
use get_index_of_vector::GetIndex;

fn main() {
    let characters = vec!['A','B', 'C', 'D', 'E'];
    match characters.get_index(&'A') {
        IndexResult::Exist(index) => println!("index: {}, value: {}", index, characters[index]),
        IndexResult::None         => println!("None")
    }
}

