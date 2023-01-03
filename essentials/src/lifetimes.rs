// LIFETIMES IN STRUCTS
//      We need to specify lifetime if we need to construct
//      a struct using references.
struct User<'a> {
    user_name: &'a String,
    email: &'a String,
}

fn lifetimes_demo() {
    // SIMPLE LIFETIME DEMO
    let string1 = String::from("abcde");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);

    println!("\n\nLONGEST STRING: {}", result);

    // LIFETIMES IN STRUCTS
    let u_name = String::from("Diaa ElKott");
    let u_email = String::from("diaa.elkott@mail.com");

    let auser = User {
        user_name: &u_name,
        email: &u_email,
    };

    print_user_data(auser);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn print_user_data(user: User) {
    println!("\n\n============================\nUSER DATA\n============================");
    println!("Name:\t{}", user.user_name);
    println!("Email:\t{}", user.email);
    println!("============================");
}
