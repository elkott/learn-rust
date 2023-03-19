///
/// A function to demonstarte hte Rust ownership concepts.
///
fn run_ownership_demo() {
    // Variables on the stack.
    let mut var0: u32 = 10; // Mutable variable.
    let var1: &mut u32 = &mut var0; // Mutable reference to V0. Changing VAR1 will change VAR0.

    let mut var2: u32 = 10;
    let mut var3: u32 = var2; // Copy; variables on the stack may be copied.
    var2 += 10;
    var3 += 1;

    increment_and_print_u32(var1, String::from("VAR1+1: ")); // Note how a value is passed by mutable reference.
    print_variable(&var0, String::from("VAR0_1: "));
    print_variable(&var2, String::from("VAR2:\t"));
    print_variable(&var3, String::from("VAR3:\t"));

    let str_lit: &str = "Hello"; // String literal - created on the stack.
    let str_lit_cp = str_lit; // String literals can be copied.

    print_variable(&str_lit_cp, String::from("STR_LIT_CP:\t"));

    //      Array.
    let array = [10, 20, 30, 40, 50, 60, 70];
    let array_2 = array;

    print_variable(&array, String::from("\nARRAY:\t"));
    print_variable(&array_2, String::from("ARRAY2:\t"));

    print_variable(&array, String::from("\nARRAY:\t"));

    // Variables on the heap.

    //      String.
    let s0 = String::from("Hello");

    let s1 = s0; // MOVE; data were moved to s1.
    let s2 = s1.clone() + &String::from(", World!"); // CLONE.

    print_variable(&s1, String::from("\nS1:\t"));

    print_variable(&s2, String::from("S2:\t")); // One may use ANY number of
    print_variable(&s2, String::from("S2:\t")); // immutable references!

    //      Vector.
    let my_vec = vec![1, 2, 3];
    let my_vec_mv = my_vec; // MOVE!

    print_variable(&my_vec_mv, String::from("\nVEC MOVE:\t"));

    //          =============================
    //               S L I C E   T Y P E
    //          =============================
    let hello_world = String::from("Hello, World!");
    print_variable(&hello_world, String::from("\nFull0:\t"));

    let hello_world = &hello_world[..];
    print_variable(&hello_world, String::from("Full1:\t"));

    let hello = &hello_world[0..5]; // From index 0 to index 5, 5 is EXCLUSIVE.
    let world = &hello_world[7..13]; // From index 7 to index 13, 13 is EXCLUSIVE.
    print_variable(&hello, String::from("\nSLIC1:\t"));
    print_variable(&world, String::from("SLIC2:\t"));

    let hello = &hello_world[..5];
    let world = &hello_world[7..];
    print_variable(&hello, String::from("\nSLIC1:\t"));
    print_variable(&world, String::from("SLIC2:\t"));

    let wrd1 = return_first_word(hello_world);
    print_variable(&wrd1, String::from("\n1ST W:\t"));

    //      Mutable string literal.
    let mut new_str_literal = return_first_word(hello_world);

    print_variable(
        &return_first_word(new_str_literal),
        String::from("\n1ST W1:\t"),
    );

    new_str_literal = "hi, world";
    print_variable(
        &return_first_word(new_str_literal),
        String::from("1ST W2:\t"),
    );

    //     Slice of array of numerals.
    let numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    print_variable(&&numbers[..3], String::from("\n1ST 3:\t"));
}

///
/// Generic function to print a generic variable, var,
/// preceded by string, description. Please note the
/// implementation of std::fmt::Debug on T to enable
/// formatted printout.
///
/// Please note: since we are passing a references to the
/// variable var as input argument, the function does not
/// take ownsership of the variable var.
///
fn print_variable<T>(var: &T, description: String)
where
    T: std::fmt::Debug,
{
    println!("{} {:?}", description, var);
}

fn increment_and_print_u32(var: &mut u32, description: String) {
    *var += 1;
    println!("{} {:?}", description, var);
}

fn return_first_word(var: &str) -> &str {
    let word_bytes = var.as_bytes();
    for (i, &item) in word_bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &var[..i];
        }
    }

    return &var[..];
}
