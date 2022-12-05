fn sum_two_nums(x: u32, y: u32) -> u32 {
    x + y // Will return the sum since there is no ';'
}

fn loop_increment(x: u32, inc: u32, end_condition: u32) -> u32 {
    let mut final_value = x;
    while final_value != end_condition {
        final_value += inc;
    }

    //// Alternatively, use the code below.
    // loop {
    //     final_value += inc;
    //     if !(final_value < end_condition) {
    //         break;
    //     }
    // }

    return final_value;
}