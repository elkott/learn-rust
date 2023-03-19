mod mod01 {

    // Generic Result enum.
    #[derive(Debug)]
    pub enum Res<T, E> {
        Thing(T),
        Error(E),
    }

    pub fn divide(a: i32, b: i32) -> Res<i32, String> {
        if b == 0 {
            return Res::Error(String::from("Cannot divide by zero!"));
        }
        Res::Thing(a / b)
    }

    // Use the RESULT struct, which is part of the stanndard library.
    pub fn divide_std(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            return Err(String::from("Cannot divide by zero!")); // Same as Result::Err
        }
        Ok(a / b) // Same as Result::Ok
    }

    pub fn demo_result() {
        let a = 5;
        let b = 0;

        match divide(a, b) {
            Res::Thing(v) => println!("\n{}/{} = {:?}", a, b, v),
            Res::Error(e) => println!("\n{}/{} = {:?}", a, b, e),
            // _ => {} // This line is unreachable; used to demonstrate the usage of "_".
        }

        let b = 5;
        match divide_std(a, b) {
            Result::Ok(v) => println!("{}/{} = {:?}", a, b, v),
            Result::Err(e) => println!("{}/{} = {:?}", a, b, e),
            // _ => {} // This line is unreachable; sed to demonstrate the usage of "_".
        }

        let b = 2;
        println!("{}/{} = {:?}", a, b, divide_std(a, b));
    }
}
