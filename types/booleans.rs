fn main() {
    let x: bool = true;
    let y = false;
    if x {
        println!("X is true!");
    }
    if y {
        println!("Y is true!");
    }

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
}
