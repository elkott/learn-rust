mod mod00 {

    ///# Struct PERSON
    ///## Example of Using the Person Struct:
    ///
    ///```
    /// let p1 = mod00::Person {
    ///     name: String::from("John Smith"),
    ///     age: 46,
    ///     children: 2,
    ///     fav_col: mod00::Colour::Green
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

    pub fn demo_structs_enums() {
        //
        // Create a PERSON struct and print its contents
        // using two different methods.
        let p1 = Person {
            name: String::from("Diaa ElKott"),
            age: 53,
            children: 3,
            fav_col: Colour::Blue,
        };

        println!("\nClient Information - DEBUG:\n{:#?}", p1);
        println!("\nClient Information - SELF PRINT:\n\t{:#?}", p1.print());

        // Create a COLOUR enum and interrogate it.
        let c1 = Colour::Green;
        let c2 = Colour::Red(String::from("Reddish!"));

        println!("\nC1: {}", c1.print_content());
        println!("\nC2: {}", c2.print_content());
    }
}
