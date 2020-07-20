use std::io;

fn main() {
    let greeting = "Hello,";

    /* mut operator creating mutable variables */ 
    let mut name = String::new();

    /* read_line() from stdin allow to read text from keyboard */
    io::stdin().read_line(&mut name);

    println!("{} {}", greeting, name);
}

/* this will compile with a 'result' warning. This can be ignored for now */