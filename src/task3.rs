#[test]

/*
// Fix the error
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}
*/


// Fix the error
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

/*
В Rust есть ограничение на количество элементов в кортеже — он может содержать не более 12 элементов.
Чтобы исправить ошибку с "слишком длинным кортежем", нужно уменьшить количество элементов в кортеже до 12 или меньше.
*/