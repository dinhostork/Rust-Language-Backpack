fn main() {
    let tuple = ("key", "value");
    println!("{} {}", tuple.0, tuple.1); // key value

    let coordinates = ("x", "y", "z");
    println!("{} {} {}", coordinates.0, coordinates.1, coordinates.2); // x y z

    let a = ("1", "Teste", "c");
    let (a0, a1, a2) = a;
    println!("{} {} {}", a0, a1, a2);

    let group = [coordinates, a];
    for item in group.iter() {
        println!("{}", item.0);
    }
}
