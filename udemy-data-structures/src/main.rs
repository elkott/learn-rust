include!("00_struct_enum_intro.rs");
include!("01_result_enum.rs");
include!("02_iterator.rs");
include!("99_date_demo.rs");

fn main() {
    // 00. Demonstrate enums and structs.
    mod00::demo_structs_enums_0();
    mod00::demo_structs_enums_1();

    // 01. Demonstate the Result enum.
    mod01::demo_result();

    // 02. Demonstrate iterator mechanisms.
    mod02::demo_iterator_0();
    mod02::demo_iterator_1();

    // 99: Demonstrate date calculations.
    mod99::date_demo_0();
}
