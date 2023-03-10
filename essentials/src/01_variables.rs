include!("utils.rs");

fn run_variablaes_demo() {
    println!("\nVARIABLES DEMO FUNCTION");

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

    //      Vector
    let mut my_vec = vec![1u32, 2, 3];
    println!("\n\nVECTOR - Initial: {:?}", my_vec);
    println!(
        "First item in vector is ONE: {}",
        first_element_equals_one(&mut my_vec)
    );
    my_vec.push(15);
    println!("VECTOR - Final: {:?}", my_vec);

    // PASS A REFERENCE TO A MUTBLE VARIABLE USING A FUNCTION
    let mut init_val: i8 = 10;
    println!("\n\nADD Two to {}", init_val);
    add_two(&mut init_val);
    println!("              -> {}", init_val);

    // EXPRESSIONS
    let age = if mytup.1 < 125 { mytup.1 } else { 25 };
    println!("\n\nAGE: {}", age);

    // CONTROL FLOW

    //      Loop
    let mut x = 10;

    let sum = loop {
        x += 1;
        if x == 15 {
            break x; // Break loop, and return the value of x.
        }
    };
    println!("\n\nLOOP ENDED AT SUM = {}", sum);

    //      Perform loop using a function.
    let increment = 1;
    let end_condition = 500;
    let count_result = loop_increment(x, increment, end_condition);

    println!("\n\nLOOP-INCREMENT END VALUE: {}", count_result);

    //     WHILE LOOP
    x = 0;
    while x != end_condition {
        x += increment;
    }
    println!("\n\nWHILE LOOP END VALUE: {}", x);

    //      FOR LOOP
    x = 15;
    let array = [10, 20, 30, 40, 50, 60, 70];

    for element in array.iter() {
        x += element;
    }
    println!("\n\nCURRENT VALUE OF X: {}", x);

    x = 15;
    for element in 1..11 {
        x += element;
    }
    println!("\n\nCURRENT VALUE OF X: {}", x);

    // =================
    // O W N E R S H I P
    // =================
    let var0 = 10;
    let var1 = var0; // Copy!

    println!("\n\nVAR0: {}", var0);
    println!("VAR1: {}", var1);

    let s0 = String::from("Hello");

    let s1 = s0; // MOVE!
    let s2 = s1.clone(); // CLONE

    println!("\n{}, World! - MOVED  \"Hello\"", s1);
    println!("{}, World! - CLONED \"Hello\"", s2);
}

fn first_element_equals_one(vector_in: &mut [u32]) -> bool {
    vector_in[0] == 1
}

fn add_two(input_var: &mut i8) {
    *input_var += 2;
}
