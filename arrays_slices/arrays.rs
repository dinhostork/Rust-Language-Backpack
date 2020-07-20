fn main() {
    let arr = ["You", "programming", "rust"];
    println!("{} are {} in {}", arr[0], arr[1], arr[2]);
    println!("{}", arr.len());

    let arr2 = [0; 3];
    println!("{} {} {}", arr2[0], arr2[1], arr2[2]); //[0,0,0]

    // slice
    let ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let identic = &ten[..];
    let interval = &ten[3..7];
    println!("ten got {} elements", ten.len());
    println!("identic got {} elements", identic.len());
    println!("interval got {} elements", interval.len());

    //itaration
    for number in ten.iter() {
        println!("{}", number);
    }
}
