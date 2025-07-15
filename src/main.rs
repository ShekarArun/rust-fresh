#[allow(unused_variables)] // Only for test purposes

fn main() {
    println!("Welcome to the rustified world!");

    let a_bool: bool = true;
    println!("Here is a boolean value: {}", a_bool);

    let an_int8: i8 = 10; // -128 to 127
    println!("Here is an int value: {}", an_int8);
    println!("Min i8 is {}", std::i8::MIN);
    println!("Max i8 is {}", std::i8::MAX);

    let an_uint8: u8 = 255;
    println!("Here is an unsigned int value: {}", an_uint8);

    let a_float: f32 = 10.678; // Note: decimal mandatory for float values
    println!("Here is a float value: {}", a_float);

    let a_char: char = 'a'; // IMP: Single quotes around chars
    println!("Here is a char: {}", a_char);
}
