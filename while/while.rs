fn main() {
    let mut counter = 0;

    while counter <= 16 {
        println!("{}", counter);
        counter+=1
    }

    // infinite loop, like while true
    loop {
        println!("Running forever!");
    }
}