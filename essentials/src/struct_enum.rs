//  =========================================================================
//  File:       struct_enum.rs
//  Purpose:    Demonstrate the utilization of structs with enums as members.
//  =========================================================================
// use std::time::{};

fn demo_struct_enum() {
    // ENUM: Country
    enum Country {
        Canada,
        USA,
    }

    // ENUM: AgeGroup
    enum AgeGroup {
        Chilld,
        Adult,
        Senior,
    }

    // STRUCT: DOB
    struct DOB{
        year: u8,
        month:u8,
        day:u8
    }

    // STRUCT: Address
    struct Address {
        address_line_1: String,
        address_line_2: String,
        city: String,
        province: String,
        country: Country,
        postal_code: String,
    }

    // STRUCT: Person
    struct Person{
        first_name: String,
        middle_name: String,
        last_name: String,
        address: Address,
        dob:
    }
}
