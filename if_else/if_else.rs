fn check(grade: f32) -> () {
    if grade >= 0.0 && grade < 4.9 {
        println!("Disapproved");
    } else if grade >= 4.9 && grade < 6.0 {
        println!("Exam");
    } else if grade >= 6.0 {
        println!("Approved");
    } else {
        println!("Invalid grade!!!!");
    }
}
fn main() {
    let response = if 10 + 5 == 15 {
        "10 + 5 is 15"
    } else {
        "10 + 5 isn't 15"
    };

    println!("{}", response);

    let grade_a = 0.0;
    let grade_b = 3.2;
    let grade_c = 5.1;
    let grade_d = 8.3;
    check(grade_a);
    check(grade_b);
    check(grade_c);
    check(grade_d);
}
