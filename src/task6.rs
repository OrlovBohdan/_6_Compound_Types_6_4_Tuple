#[test]

/*

fn main() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply(__);

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
*/


fn main() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply((2,3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

/*
Чтобы код работал корректно, вам нужно передать кортеж с двумя целыми числами в функцию sum_multiply.
Поскольку ожидается, что сумма этих чисел равна 5, а произведение равно 6, можно использовать числа 2 и 3

*/