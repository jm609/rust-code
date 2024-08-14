#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

fn main() {
    let mut jane = Person {
        name: String::from("Jane"),
        age: 38
    };
    jane.age += 1;
    println!("{} {}", jane.name, jane.age);
    println!("{:?}", jane);
}