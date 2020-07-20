fn print_hw() {
    println!("Hello Pointers")
}

fn do_the_things(function: fn()) {
    function()
}

fn main() {
    let pointer_to_function: fn() = print_hw;
    do_the_things(pointer_to_function);
}
