include!("00_struct_enum_intro.rs");

fn main() {

    // Create a PERSON struct and print its contents 
    // using two different methods.
    let p1 = mod01::Person {
        name: String::from("Diaa ElKott"),
        age: 53,
        children: 3,
    };

    println!("\nClient Information - DEBUG:\n{:#?}", p1);
    println!("\nClient Information - SELF PRINT:\n\t{:#?}", p1.print());

    // Create a COLOUR enum and interrogate it.
    let c1 = mod01::Colour::Green;
    let c2 = mod01::Colour::Red(p1.print());

    println!("\nC1: {}", c1.shoutout());
    println!("\nC2: {}", c2.shoutout());
}
