mod compute {
    // private function
    fn is_zero(number: i32) -> bool {
        if number == 0 {
            return true;
        };
        false
    }

    pub fn sum(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

    pub fn sub(n1: i32, n2: i32) -> i32 {
        n1 - n2
    }

    pub fn div(n1: i32, n2: i32) -> i32 {
        if is_zero(n2) {
            return 0;
        };
        n1 / n2
    }

    pub fn mul(n1: i32, n2: i32) -> i32 {
        n1 * n2
    }
}

fn main() {
    use compute::sum as my_sum;

    let a: i32 = 10;
    let b: i32 = 6;

    println!("{} + {} = {}", a, b, my_sum(a, b));
    println!("{} - {} = {}", a, b, compute::sub(a, b));
    println!("{} / {} = {}", a, b, compute::div(a, b));
    println!("{} * {} = {}", a, b, compute::mul(a, b));
}
