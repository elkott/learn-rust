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

    //      Slice type.
    let mut hello_world = String::from("Hello, World!");
    let hello = &hello_world[0..5];
    let world = &hello_world[7..13];

    print_variable(&hello_world, String::from("\nFull:\t"));
    print_variable(&hello, String::from("SLIC1:\t"));
    print_variable(&world, String::from("SLIC2:\t"));
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
