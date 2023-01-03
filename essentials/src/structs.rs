fn structs_demo() {
    struct User {
        user_name: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1: User = User {
        email: String::from("person@gmail.com"),
        user_name: String::from("jsmith"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(user1.user_name.clone(), user1.email);

    println!(
        "\n\nUser1 Name: {}\nUser2 name: {}",
        user1.user_name, user2.user_name
    );

    fn build_user(user_name: String, email: String) -> User {
        User {
            user_name: user_name,
            email: email,
            sign_in_count: 1,
            active: true,
        }
    }

    // RECTANGLE STRUC
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    //      STRUCT METHOD IMPLEMENTATION
    impl Rectangle {
        fn area(&self) -> u32 {
            self.height * self.width
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // fn area(rectangle: &Rectangle) -> u32 {
    //     rectangle.width * rectangle.height
    // }

    let _my_rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let _my_square = Rectangle::square(30);

    //      FORMATTING STRUCT
    println!("\n\nRECTANGLE: {:#?}", _my_rectangle);
    println!(
        "\n\nAREA OF RECTANGLE {:?} = {}",
        _my_rectangle,
        _my_rectangle.area()
    );

    println!(
        "\n\nRECTANGLE {:?} HOLD TEST OF SQUARE {:?} is: {}",
        _my_rectangle,
        _my_square,
        _my_rectangle.can_hold(&_my_square)
    );
}
