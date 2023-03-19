mod mod02 {

    pub fn demo_iterator_0() {
        // 0. Implementation of a generic "STEPPER" loop.
        pub struct Stepper {
            val: i64,
            stp: i64,
            max: i64,
        }

        // Implement a custom iterator for Stepper struct.
        impl Iterator for Stepper {
            type Item = i64;
            fn next(&mut self) -> Option<i64> {
                if self.val >= self.max {
                    return Option::None;
                }
                let res = self.val;
                self.val += self.stp;
                return Option::Some(res);
            }
        }

        impl Stepper {
            pub fn print(&self) -> String {
                format!(
                    "Start: {}, Step: {}, Max: {}.",
                    self.val, self.stp, self.max,
                )
            }
        }

        let mut stpr = Stepper {
            val: 0,
            stp: 1,
            max: 3,
        };

        //Demonstrate LOOP.
        println!("\n===========================");
        println!("demo LOOP for Stepper:\n{}", stpr.print());
        println!("===========================");
        loop {
            match stpr.next() {
                Option::Some(v) => println!("Val =\t{}", v),
                Option::None => break,
            }
        }

        let mut stpr = Stepper {
            val: 0,
            stp: 1,
            max: 3,
        };

        //Demonstrate WHILE.
        println!("\n===========================");
        println!("demo WHILE for Stepper:\n{}", stpr.print());
        println!("===========================");

        while let Some(n) = stpr.next() {
            println!("Val =\t{}", n);
        }

        let stpr = Stepper {
            val: 0,
            stp: 1,
            max: 3,
        };

        //Demonstrate FOR.
        println!("\n===========================");
        println!("demo FOR for Stepper:\n{}", stpr.print());
        println!("===========================");

        for s in stpr {
            println!("Val =\t{}", s);
        }

        //Demonstrate FACTORIAL.
        println!("\n===========================");
        println!("demo FACTORIAL(5)",);
        println!("===========================");

        println!("{}", factorial(5, 1));

        // =====================================
        // Demonstrate VEC ITERATOR.
        // =====================================
        println!("\n===========================");
        println!("demo STD LIB ITERATOR",);
        println!("===========================");
        let v1 = vec![2, 4, 6];
        let v1_iter = v1.iter();

        println!("V1 ELEMENTS\t=\t{:?}", v1);

        for val in v1_iter {
            println!("FACTORIAL({})\t=\t{}", val, factorial(*val, 1));
        }

        // WHY did we need to declare the iterator again?!!
        let v1_iter = v1.iter();
        let sum: i32 = v1_iter.sum(); // SUM() is a "CONSUMER METHOD".
        println!("SUM(VEC)\t=\t{}", sum);

        let v1_iter = v1.iter();
        let v2: Vec<_> = v1_iter.map(|x: &i32| x + 1).collect(); // map() is an "ADAPTER METHOD".
        println!("V2 ELEMENTS\t=\t{:?}", v2);
        assert_eq!(v2, vec![3, 5, 7]);

        fn factorial(n: i32, r: i32) -> i32 {
            if n <= 1 {
                return r;
            }
            return factorial(n - 1, n * r);
        }
    }

    pub fn demo_iterator_1() {
        #[derive(PartialEq, Debug)]
        pub struct Shoe {
            size: f32,
            model: String,
        }

        fn match_shoe_size(shoes: Vec<Shoe>, size: f32) -> Vec<Shoe> {
            shoes
                .into_iter()
                .filter(|shoe: &Shoe| shoe.size == size)
                .collect()
        }

        let mut shoe_collection: Vec<Shoe> = vec![Shoe {
            size: 10.0,
            model: "Chukka".to_string(),
        }];

        shoe_collection.push(Shoe {
            size: 11.0,
            model: "Chukka".to_string(),
        });

        shoe_collection.push(Shoe {
            size: 11.0,
            model: "Boot".to_string(),
        });

        shoe_collection.push(Shoe {
            size: 10.5,
            model: "Oxford".to_string(),
        });

        shoe_collection.push(Shoe {
            size: 11.5,
            model: "Sneaker".to_string(),
        });

        shoe_collection.push(Shoe {
            size: 11.0,
            model: "Hiker".to_string(),
        });

        //Demonstrate ITERATOR over Vec<Shoe>
        println!("\n==============================");
        println!("demo iterator over Shoe vector");
        println!("==============================");

        let size = 11.0;
        let my_shoes_selection = match_shoe_size(shoe_collection, size);
        println!("Size {} Shoes:{:#?}", size, my_shoes_selection);
    }
}
