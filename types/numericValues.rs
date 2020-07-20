fn main() {
    //let a: i32 = 4294967295; this is more than we can take!
    let x: u32 = 4294967295; //now we can take it!
    println!("{}", x);

    //values
    println!("i8 = {} a {}", i8::min_value(), i8::max_value());
    println!("i16= {} a {}", i16::min_value(), i16::max_value());
    println!("i32= {} a {}", i32::min_value(), i32::max_value());
    println!("i64= {} a {}", i64::min_value(), i64::max_value());
    println!("u8 = {} a {}", u8::min_value(), u8::max_value());
    println!("u16= {} a {}", u16::min_value(), u16::max_value());
    println!("u32= {} a {}", u32::min_value(), u32::max_value());
    println!("u64= {} a {}", u64::min_value(), u64::max_value());

    //methods
    let a: i8 = 1;
    println!("Ones: {}", a.count_ones());
    println!("Zeros: {}", a.count_zeros());
    println!(">>: {}", a.rotate_left(7));
    println!(">>: {}", a.rotate_right(8));
    println!(">>: {}", a.swap_bytes());

    //floats
    let y: f64 = 42949.67295;
    println!("{}", y);

    // float methods
    println!("Floor: {}", y.floor());
    println!("Ceil: {}", y.ceil());
    println!("Round: {}", y.round());
    println!("Truncate: {}", y.trunc());
    println!("Fractional: {}", y.fract());
    println!("Is Finite?: {}", y.is_finite());
    println!("Is Infinite?: {}", y.is_infinite());
    println!("Is NaN?: {}", y.is_nan());
}
