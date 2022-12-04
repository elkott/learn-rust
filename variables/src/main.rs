fn main() {
    let x: u32 = 5;
    const COUNT: u32 = 100_000; // const cannot be mutated.

    println!("\n\n{} + {} = {}", x, COUNT, sum_two_nums(x, COUNT));

    // Variable shadowing.
    let x: i32 = 257;
    // Casting the first function argument to U32.
    println!("\n\n{} + {} = {}", x, COUNT, sum_two_nums(x as u32, COUNT));

    // Scalar data types.
    let _int: i64 = 5; // Use '_' to avoid warning.
    let _flt: f64 = 1.5;
    let _ok: bool = true;
    let _my_chr: char = 's';

    // COMPUND TYPES

    //      Tuple
    let mytup: (&str, i32) = ("Diaa ElKott", 53);
    let (name, age) = mytup; // Destructuring a tuple.
    println!("\n\nTUPLE: name = {}, age = {}", name, age);
    println!("\n\nTUPLE: name = {}, age = {}", mytup.0, mytup.1);

    //      Array
    let _ages = [10, 20, 30, 50];

    // EXPRESSIONS
    let age = if mytup.1 < 125 { mytup.1 } else { 25 };
    println!("\n\nAGE: {}", age);

    // CONTROL FLOW

    //      Loop
    let mut x = 10;
    let mut sum = 0;
    loop {
        sum = sum_two_nums(sum, x);
        x += 1;
        if x == 15 {
            println!("\n\nLOOP ENDED AT SUM = {}", sum);
            break;
        }
    }

    //      Perform loop using a function.
    let increment = 1;
    let end_condition = 500;
    let count_result = loop_increment(x, increment, end_condition);

    println!("\n\nLOOP-INCREMENT enD VALUE: {}", count_result);

    //      FOR LOOP
    // x = 0;
    // for
}

fn sum_two_nums(x: u32, y: u32) -> u32 {
    x + y // Will return the sum since there is no ';'
}

fn loop_increment(x: u32, inc: u32, end_condition: u32) -> u32{
    let mut final_value = x;
    loop {
        final_value += inc;
        if !(final_value < end_condition) {
            break final_value;
        }
    };
    return final_value;
}
