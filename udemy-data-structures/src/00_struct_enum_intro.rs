mod mod01 {

    /// STRUCT - Person
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u32,
        pub children: u32,
        pub fav_col: Colour,
    }

    impl Person {
        pub fn print(&self) -> String {
            format!(
                "Name: {}, Age {}, has {} children, and loves the colour {}.",
                self.name,
                self.age,
                self.children,
                self.fav_col.print_content()
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
        pub fn print_content(&self) -> String {
            match self {
                Colour::Blue => format!("{:#?}", self),
                Colour::Green => format!("{:#?}", self),
                Colour::Red(s) => format!("{:#?}", s),
            }
        }
    }
}
