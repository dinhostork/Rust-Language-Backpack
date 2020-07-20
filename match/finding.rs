fn check(n: i32) -> () {

    match n {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

}

fn main() {
    let n_a = 0;
    let n_b = 3;
    let n_c = 5;
    let n_d = 16;
    check(n_a);
    check(n_b);
    check(n_c);
    check(n_d);
}
