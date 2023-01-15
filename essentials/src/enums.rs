use std::clone;

fn enums_demo() {
    // ENUM DEFINITION
    #[derive(Debug)]
    enum IPAddressKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // ENUM IMPLEMENTATION METHOD
    //      USE MATCH EXPRESSION!
    impl IPAddressKind {
        fn to_string(&self) -> String {
            match self {
                IPAddressKind::V4(a, b, c, d) => String::from(
                    [a.to_string(), b.to_string(), c.to_string(), d.to_string()].join("."),
                ),
                IPAddressKind::V6(s) => String::from(s),
            }
        }
    }

    let localhost0 = IPAddressKind::V4(127, 0, 0, 1);
    let localhost1 = IPAddressKind::V6(String::from("http://localhost:8080"));

    println!("\n\nIP is : {:#?}", localhost0);
    println!("Local Host 0 : {}", localhost0.to_string());
    println!("Local Host 1 : {}", localhost1.to_string());

    // ======================================
    // A NEW ENUM!!
    // ======================================
    #[derive(Clone)]
    enum Pet {
        dog,
        cat,
        fish,
    }

    impl Pet {
        fn what_am_i(self) -> &'static str {
            match self {
                Pet::dog => "I am a dog!",
                Pet::cat => "I am a cat!",
                Pet::fish => "I am a fish!",
            }
        }

        fn what_am_i2(self) -> String {
            match self {
                Pet::dog => String::from("dog!"),
                Pet::cat => String::from("cat!"),
                Pet::fish => String::from("fish!"),
            }
        }
    }

    let mut dog = Pet::dog;
    let dog2 = dog.clone();

    println!("\n\nDOG1 TYPE:\t{}", dog.what_am_i());
    println!("DOG2 TYPE:\t{}", dog2.what_am_i2());

    dog = Pet::cat;
    println!("DOG1 NEW TYPE:\t{}", dog.what_am_i2());
}
