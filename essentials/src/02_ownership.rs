///
/// A function to demonstarte hte Rust ownership concepts.
///
fn run_ownership_demo() {
    // Variables on the stack.
    let mut var0:u32 = 10; // Immutable variable.
    let mut var1:u32 = var0; // Copy; variables on the stack may be copied.
    var1 += 10;

    increment_and_print_u32(&mut var0, String::from("\nVAR0+1: ")); //Note how a value is passed by mutable reference.
    print_variable(&var0, String::from("VAR0:\t"));
    print_variable(&var1, String::from("VAR1:\t"));

    let str_lit: &str = "Hello"; // String literal - created on the stack.
    let str_lit_cp = str_lit; // String literals can be copied.

    print_variable(&str_lit_cp, String::from("STR_LIT_CP:\t"));

    //      Array.
    let array = [10, 20, 30, 40, 50, 60, 70];
    let array_2 = array;

    print_variable(&array, String::from("\nARRAY:\t"));
    print_variable(&array_2, String::from("ARRAY2:\t"));

    // Variables on the heap.
    //      String.
    let s0 = String::from("Hello");

    let s1 = s0; // MOVE; data were moved to s1.
    let s2 = s1.clone() + &String::from(", World!"); // CLONE.

    print_variable(&s1, String::from("\nS1:\t"));
    print_variable(&s2, String::from("S2:\t"));

    // print_variable(s2, String::from("S2:\t"));

    //      Vector.
    let my_vec = vec![1u32, 2, 3];
    let my_vec_cp = my_vec; // MOVE!

    print_variable(&my_vec_cp, String::from("\nVEC COPY-0:\t"));
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

fn increment_and_print_u32(var: &mut u32, description: String)
{
    *var += 1;
    println!("{} {:?}", description, var);
}