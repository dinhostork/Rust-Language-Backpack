fn main() {
    //declarations
    let a: char = '1';
    let b: char = '8';
    let c: char = '\u{2764}';

    println!("{} {} {}", a, b, c);

    println!("{} is a digit? {}", a, a.is_digit(10));
    println!("{} is a binary? {}", a, a.is_digit(2));
    println!("{} is a digit? {}", b, b.is_digit(10));
    println!("{} is a binary? {}", b, b.is_digit(2));
    println!("{} is a digit? {}", c, c.is_digit(10));
    println!("{} is a binary? {}", c, c.is_digit(2));

    //escape unicode
    let arrow: char = '→';
    let repr: String = arrow.escape_unicode().collect();
    println!("{}", repr);

    //checking if the character belongs to alphabet
    println!("{}", 'a'.is_alphabetic());
    println!("{}", ' '.is_alphabetic());
    println!("{}", '→'.is_alphabetic());

    //others
    println!("Uppercase -> {}", 'a'.is_uppercase());
    println!("Lowercase -> {}", 'a'.is_lowercase());
    println!("Whitespace -> {}", 'a'.is_whitespace());
    println!("Alphanumeric -> {}", 'a'.is_alphanumeric());
    println!("Numeric -> {}", 'a'.is_numeric());
}
