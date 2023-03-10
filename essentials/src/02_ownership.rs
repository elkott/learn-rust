///
/// A function to demonstarte hte Rust ownership concepts.
///
fn run_ownership_demo() {
    // Variables on the stack.
    let var0 = 10; // Immutable variable.
    let mut var1 = var0; // Copy; variables on the stack may be copied.
    var1 += 10;

    print_variable(var0, String::from("\nVAR0:\t"));
    print_variable(var1, String::from("VAR1:\t"));

    // Variables on the heap.
    //      String.
    let s0 = String::from("Hello");

    let s1 = s0; // MOVE; data were moved to s1.
    let s2 = s1.clone() + &String::from(", World!"); // CLONE.

    print_variable(s1, String::from("\nS1:\t"));
    print_variable(s2, String::from("S2:\t"));

    //      Array.
    let array = [10, 20, 30, 40, 50, 60, 70];
    print_variable(array, String::from("\nARRAY:\t"));
}

///
/// Generic function to print a generic variable, var,
/// preceded by string, description. Please note the
/// implementation of std::fmt::Debug on T to enable
/// formatted printout.
///
fn print_variable<T>(var: T, description: String)
where
    T: std::fmt::Debug,
{
    println!("{} {:?}", description, var);
}
