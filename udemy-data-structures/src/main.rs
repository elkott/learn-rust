include!("00_struct_enum_intro.rs");
include!("01_result_enum.rs");
include!("02_iterator.rs");

fn main() {
    // 00. Demonstrate enums and structs.
    mod00::demo_structs_enums();

    // 01. Demonstate the Result enum.
    mod01::demo_result();

// 02. Demonstrate iterator mechanisms.
mod02::demo_iterator();

}
