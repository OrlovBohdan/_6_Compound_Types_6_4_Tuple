#[test]

/*

fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, __, i64, __, __) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}
*/


fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

/*
Первый пропуск заполняется u16, так как второй элемент кортежа имеет тип u16.
Третий пропуск остается i64, так как это уже указано в определении кортежа.
Четвертый пропуск заполняется &str, так как строковый литерал "hello" имеет этот тип.
Пятый пропуск заполняется String, так как используется String::from(", world").
*/