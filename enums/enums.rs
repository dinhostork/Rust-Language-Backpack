enum Gender {
    Female,
    Male,
    Other,
}

struct Person {
    name: &'static str,
    gender: Gender,
}

fn main() {
    let mario = Person {
        name: "Mario Brother",
        gender: Gender::Male,
    };

    
    let gender = Gender::Male;
    match gender {
        Gender::Other => println!("Other"),
        Gender::Male => println!("Male"),
        Gender::Female => println!("Female"),
    }
}
