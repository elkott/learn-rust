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
    let localhost1=IPAddressKind::V6(String::from("http://localhost:8080"));

    println!("\n\nIP is : {:#?}", localhost0);
    println!("Local Host 0 : {}", localhost0.to_string());
    println!("Local Host 1 : {}", localhost1.to_string());
}
