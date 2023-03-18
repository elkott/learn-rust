mod mod01 {

    ///# Struct PERSON
    ///## Example of Using the Person Struct:
    ///
    ///```
    /// let p1 = mod01::Person {
    ///     name: String::from("John Smith"),
    ///     age: 46,
    ///     children: 2,
    ///     fav_col: mod01::Colour::Green
    ///     };
    ///
    /// let output = p1.print();
    /// assert_eq!(output, "Name: John Smith, Age 46, has 2 children, and loves the colour Green.");
    ///```
    ///
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
