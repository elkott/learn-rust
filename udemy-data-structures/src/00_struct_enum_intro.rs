mod mod01 {

    /// STRUCT - Person
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u32,
        pub children: u32,
    }

    impl Person {
        pub fn print(&self) -> String {
            format!(
                "Name: {}, Age {}, has {} children.",
                self.name, self.age, self.children
            )
        }
    }

    ///  ENUM - Colour
    #[derive(Debug)]
    pub enum Colour {
        Red(String), // You may associate a String with one of the enum members.
        Green,
        Blue,
    }

    impl Colour {
        pub fn shoutout(&self) -> String {
            match self {
                Colour::Blue => format!("Colour is {:#?}", self),
                Colour::Green => format!("Colour is {:#?}", self),
                Colour::Red(s) => format!("Contents are {:#?}", s),
            }
        }
    }
}
